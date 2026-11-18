// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use crate::properties::Properties;
use crate::{
    ADAPTER_INTERFACE, Adapter, AdapterUpdate, BATTERY_INTERFACE, DEVICE_INTERFACE, Device,
    DeviceUpdate, Event, SERVICE,
};
use futures::channel::mpsc;
use futures::future::Either;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::pin::pin;
use std::time::Duration;
use tracing::Instrument;
use zbus::fdo;
use zbus::message::Type;
use zbus::names::UniqueName;
use zbus::{MatchRule, Message, MessageStream, OwnedObjectPath};

const BUS: &str = "org.freedesktop.DBus";

const QUEUE: usize = 1024;

pub async fn watch(connection: zbus::Connection, mut tx: mpsc::Sender<Event>) {
    let (queue, mut queued) = mpsc::unbounded();

    let reader = pin!(
        read(connection, queue).instrument(tracing::info_span!("bluetooth::subscription::watch"))
    );
    let forward = pin!(async {
        while let Some(event) = queued.next().await {
            if tx.send(event).await.is_err() {
                return;
            }
        }
    });

    if let Either::Left(((), forward)) = futures::future::select(reader, forward).await {
        forward.await;
    }
}

async fn read(connection: zbus::Connection, queue: mpsc::UnboundedSender<Event>) {
    let mut delay = Duration::from_secs(1);
    let mut loaded = false;

    loop {
        let why = match follow(&connection, &queue, &mut loaded).await {
            Ok(()) => None,
            Err(why) if loaded && unavailable(&why).is_some() => None,
            Err(why) => Some(why),
        };

        let why = match why {
            None => {
                tracing::warn!("bluetoothd stopped");
                _ = queue.unbounded_send(Event::SetAdapters(HashMap::new()));
                _ = queue.unbounded_send(Event::SetDevices(HashMap::new()));
                match wait_for_bluetoothd(&connection).await {
                    Ok(()) => {
                        delay = Duration::from_secs(1);
                        continue;
                    }
                    Err(why) => why,
                }
            }
            Some(why) => why,
        };

        if queue.is_closed() {
            return;
        }

        tracing::error!("failed to watch bluetooth event: {why:?}.");
        _ = queue.unbounded_send(Event::DBusError(why.clone()));

        if let Some(event) = unavailable(&why) {
            match event {
                Event::DBusServiceUnknown => tracing::error!(
                    "The org.bluez dbus service is unknown. Is the bluez service installed and activatable?"
                ),
                _ => tracing::error!("The org.bluez dbus service is not enabled or active"),
            }
            _ = queue.unbounded_send(event);
            return;
        }

        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_secs(60));
    }
}

fn unavailable(why: &zbus::Error) -> Option<Event> {
    match fdo::Error::from(why.clone()) {
        fdo::Error::ServiceUnknown(_) => Some(Event::DBusServiceUnknown),
        fdo::Error::NameHasNoOwner(_) => Some(Event::NameHasNoOwner),
        _ => None,
    }
}

async fn wait_for_bluetoothd(connection: &zbus::Connection) -> zbus::Result<()> {
    let bus = fdo::DBusProxy::new(connection).await?;
    let mut owners = bus
        .receive_name_owner_changed_with_args(&[(0, SERVICE)])
        .await?;

    if bus.name_has_owner(SERVICE.try_into()?).await? {
        return Ok(());
    }

    while let Some(signal) = owners.next().await {
        if signal.args().is_ok_and(|args| args.new_owner().is_some()) {
            return Ok(());
        }
    }

    Err(closed())
}

enum Signal {
    Bluez(zbus::Result<Message>),
    Bus(zbus::Result<Message>),
}

async fn follow(
    connection: &zbus::Connection,
    queue: &mpsc::UnboundedSender<Event>,
    loaded: &mut bool,
) -> zbus::Result<()> {
    let bluez = MessageStream::for_match_rule(
        MatchRule::builder()
            .msg_type(Type::Signal)
            .sender(SERVICE)
            .build()?,
        connection,
        NonZeroUsize::new(QUEUE),
    )
    .await?;
    let owners = MessageStream::for_match_rule(
        MatchRule::builder()
            .msg_type(Type::Signal)
            .sender(BUS)
            .interface(BUS)
            .member("NameOwnerChanged")
            .arg(0, SERVICE)
            .build()?,
        connection,
        None,
    )
    .await?;
    let mut signals = pin!(futures::stream::select(
        bluez.map(Signal::Bluez),
        owners.map(Signal::Bus),
    ));

    let mut owner_changes = Vec::new();
    let (owner, objects) = {
        let mut read = pin!(managed_objects(connection));
        loop {
            match futures::future::select(read.as_mut(), signals.next()).await {
                Either::Left((objects, _)) => break objects?,
                Either::Right((Some(Signal::Bus(message)), _)) => owner_changes.push(message?),
                Either::Right((Some(Signal::Bluez(message)), _)) => _ = message?,
                Either::Right((None, _)) => return Err(closed()),
            }
        }
    };

    if owner_changes.iter().any(|message| left(message, &owner)) {
        return Ok(());
    }

    tracing::debug!("{} objects from bluetoothd {owner}", objects.len());
    _ = queue.unbounded_send(Event::SetAdapters(adapters(&objects)));
    _ = queue.unbounded_send(Event::SetDevices(devices(&objects)));
    *loaded = true;

    while let Some(signal) = signals.next().await {
        match signal {
            Signal::Bus(message) => {
                if left(&message?, &owner) {
                    return Ok(());
                }
            }

            Signal::Bluez(message) => {
                let message = message?;
                let header = message.header();
                if header
                    .sender()
                    .is_none_or(|sender| sender.as_str() != owner.as_str())
                {
                    continue;
                }

                match changes(&message) {
                    Ok(events) => {
                        for event in events {
                            _ = queue.unbounded_send(event);
                        }
                    }
                    Err(why) => tracing::warn!("Ignoring a signal of bluetoothd: {why}"),
                }
            }
        }
    }

    Err(closed())
}

fn closed() -> zbus::Error {
    zbus::Error::Failure("The D-Bus connection closed".to_owned())
}

fn left(message: &Message, owner: &UniqueName<'_>) -> bool {
    if message
        .header()
        .sender()
        .is_none_or(|sender| sender.as_str() != BUS)
    {
        return false;
    }
    let Some(signal) = fdo::NameOwnerChanged::from_message(message.clone()) else {
        return false;
    };
    let Ok(args) = signal.args() else {
        return false;
    };
    let old_owner: &Option<UniqueName<'_>> = &args.old_owner;
    args.name.as_str() == SERVICE
        && old_owner
            .as_ref()
            .is_some_and(|old_owner| old_owner.as_str() == owner.as_str())
}

fn changes(message: &Message) -> zbus::Result<Vec<Event>> {
    if let Some(signal) = fdo::PropertiesChanged::from_message(message.clone()) {
        let Some(path) = message.header().path().map(|path| path.to_owned().into()) else {
            return Ok(Vec::new());
        };
        let args = signal.args()?;

        let event = match args.interface_name.as_str() {
            ADAPTER_INTERFACE => {
                let updates = AdapterUpdate::from_update(args.changed_properties);
                (!updates.is_empty()).then(|| Event::UpdatedAdapter(path, updates))
            }
            DEVICE_INTERFACE | BATTERY_INTERFACE => {
                let updates = DeviceUpdate::from_update(args.changed_properties);
                (!updates.is_empty()).then(|| Event::UpdatedDevice(path, updates))
            }
            _ => None,
        };

        return Ok(event.into_iter().collect());
    }

    if let Some(signal) = fdo::InterfacesAdded::from_message(message.clone()) {
        let args = signal.args()?;
        let path = OwnedObjectPath::from(args.object_path.clone());
        let interfaces = &args.interfaces_and_properties;
        let mut events = Vec::new();

        if let Some(adapter) = interfaces.get(ADAPTER_INTERFACE) {
            match Adapter::from_properties(adapter) {
                Some(adapter) => events.push(Event::AddedAdapter(path.clone(), adapter)),
                None => tracing::warn!("Ignoring the adapter {path}, which has no address"),
            }
        }

        if let Some(device) = interfaces.get(DEVICE_INTERFACE) {
            match Device::from_properties(device, interfaces.get(BATTERY_INTERFACE)) {
                Some(device) => events.push(Event::AddedDevice(path, device)),
                None => {
                    tracing::warn!("Ignoring the device {path}, which has no address or adapter")
                }
            }
        } else if let Some(percentage) = interfaces
            .get(BATTERY_INTERFACE)
            .and_then(|battery| battery.byte("Percentage"))
        {
            events.push(Event::UpdatedDevice(
                path,
                vec![DeviceUpdate::Battery(Some(percentage.to_string()))],
            ));
        }

        return Ok(events);
    }

    if let Some(signal) = fdo::InterfacesRemoved::from_message(message.clone()) {
        let args = signal.args()?;
        let path = OwnedObjectPath::from(args.object_path.clone());
        let removed = |name: &str| args.interfaces.iter().any(|interface| interface == name);
        let mut events = Vec::new();

        if removed(DEVICE_INTERFACE) {
            events.push(Event::RemovedDevice(path.clone()));
        } else if removed(BATTERY_INTERFACE) {
            events.push(Event::UpdatedDevice(
                path.clone(),
                vec![DeviceUpdate::Battery(None)],
            ));
        }

        if removed(ADAPTER_INTERFACE) {
            events.push(Event::RemovedAdapter(path));
        }

        return Ok(events);
    }

    Ok(Vec::new())
}

pub(crate) async fn managed_objects(
    connection: &zbus::Connection,
) -> zbus::Result<(UniqueName<'static>, fdo::ManagedObjects)> {
    let reply = connection
        .call_method(
            Some(SERVICE),
            "/",
            Some("org.freedesktop.DBus.ObjectManager"),
            "GetManagedObjects",
            &(),
        )
        .await?;
    let owner = reply
        .header()
        .sender()
        .ok_or(zbus::Error::MissingField)?
        .to_owned();
    let objects = reply.body().deserialize()?;
    Ok((owner, objects))
}

pub(crate) fn adapters(objects: &fdo::ManagedObjects) -> HashMap<OwnedObjectPath, Adapter> {
    objects
        .iter()
        .filter_map(|(path, interfaces)| {
            let adapter = interfaces.get(ADAPTER_INTERFACE)?;
            let Some(adapter) = Adapter::from_properties(adapter) else {
                tracing::warn!("Ignoring the adapter {path}, which has no address");
                return None;
            };
            Some((path.clone(), adapter))
        })
        .collect()
}

pub(crate) fn devices(objects: &fdo::ManagedObjects) -> HashMap<OwnedObjectPath, Device> {
    objects
        .iter()
        .filter_map(|(path, interfaces)| {
            let device = interfaces.get(DEVICE_INTERFACE)?;
            let Some(device) = Device::from_properties(device, interfaces.get(BATTERY_INTERFACE))
            else {
                tracing::warn!("Ignoring the device {path}, which has no address or adapter");
                return None;
            };
            Some((path.clone(), device))
        })
        .collect()
}
