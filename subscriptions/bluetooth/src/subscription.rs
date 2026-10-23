// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Follows the adapters and devices of BlueZ.

use crate::properties::Properties;
use crate::{
    ADAPTER_INTERFACE, Adapter, AdapterUpdate, BATTERY_INTERFACE, DEVICE_INTERFACE, Device,
    DeviceUpdate, Event, SERVICE,
};
use futures::channel::mpsc;
use futures::future::Either;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::pin::pin;
use std::time::Duration;
use tracing::Instrument;
use zbus::fdo;
use zbus::message::Type;
use zbus::names::UniqueName;
use zbus::zvariant::OwnedObjectPath;
use zbus::{MatchRule, Message, MessageStream};

const BUS: &str = "org.freedesktop.DBus";

/// How many signals of bluetoothd can wait to be read, before the connection stops reading
/// anything else.
const QUEUE: usize = 1024;

/// Follows the adapters and devices of BlueZ.
///
/// Sends all of them first, with [`Event::SetAdapters`] then [`Event::SetDevices`], then their
/// changes. When bluetoothd stops, it sends them empty, then all of them again once bluetoothd
/// is back. After an error, it starts over and sends all of them again. It returns when `tx`
/// closes, or when BlueZ isn't there at the start: after [`Event::DBusServiceUnknown`] when it
/// isn't installed, or [`Event::NameHasNoOwner`] when it isn't running and can't be started.
pub async fn watch(connection: zbus::Connection, mut tx: mpsc::Sender<Event>) {
    let (queue, mut queued) = mpsc::unbounded();

    // Signals have to be read as they come, while `tx` takes events at its own pace, and this
    // future may not be polled all the time: they are read on a task of their own.
    let _reader = AbortOnDrop(tokio::spawn(
        read(connection, queue).instrument(tracing::info_span!("bluetooth::subscription::watch")),
    ));

    while let Some(event) = queued.next().await {
        if tx.send(event).await.is_err() {
            return;
        }
    }
}

struct AbortOnDrop(tokio::task::JoinHandle<()>);

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        self.0.abort();
    }
}

/// Queues the events of [`watch`].
async fn read(connection: zbus::Connection, queue: mpsc::UnboundedSender<Event>) {
    let mut delay = Duration::from_secs(1);
    // Whether BlueZ answered once.
    let mut loaded = false;

    loop {
        let why = match follow(&connection, &queue, &mut loaded).await {
            Ok(()) => None,
            // It went away while being read.
            Err(why) if loaded && unavailable(&why).is_some() => None,
            Err(why) => Some(why),
        };

        let why = match why {
            None => {
                tracing::warn!("bluetoothd stopped");
                // Nothing is left of it. Once back, it has everything again.
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

/// The event for BlueZ not being installed or not running, if the error says so.
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

    // Already back, maybe.
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

/// Queues all the adapters and devices, then their changes, until bluetoothd stops (`Ok`) or
/// something fails. Sets `loaded` once it queued them.
async fn follow(
    connection: &zbus::Connection,
    queue: &mpsc::UnboundedSender<Event>,
    loaded: &mut bool,
) -> zbus::Result<()> {
    // Listen before reading the objects, so that no change goes missing in between.
    let bluez = MessageStream::for_match_rule(
        MatchRule::builder()
            .msg_type(Type::Signal)
            .sender(SERVICE)?
            .build(),
        connection,
        Some(QUEUE),
    )
    .await?;
    let owners = MessageStream::for_match_rule(
        MatchRule::builder()
            .msg_type(Type::Signal)
            .sender(BUS)?
            .interface(BUS)?
            .member("NameOwnerChanged")?
            .arg(0, SERVICE)?
            .build(),
        connection,
        None,
    )
    .await?;
    let mut signals = pin!(futures::stream::select(
        bluez.map(Signal::Bluez),
        owners.map(Signal::Bus),
    ));

    // The connection reads the reply after the signals sent before it, so they have to be read
    // meanwhile. Their changes are already in the reply.
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
    _ = queue.unbounded_send(Event::SetDevices(devices(&objects, None)));
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
                // The bus picks the signals of the owner of the name, but the connection can't
                // tell them apart from other signals.
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

/// Whether the message says that `owner` no longer owns the name of BlueZ.
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

/// The changes to adapters and devices in a signal of bluetoothd.
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
            // Properties of other interfaces can have the same names, but not the same
            // meaning, like `Connected` of `org.bluez.MediaControl1`.
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

/// All the objects of BlueZ, and the unique name of the bluetoothd that sent them.
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

/// The devices, of all the adapters or of one.
pub(crate) fn devices(
    objects: &fdo::ManagedObjects,
    adapter: Option<&OwnedObjectPath>,
) -> HashMap<OwnedObjectPath, Device> {
    let prefix = adapter.map(|adapter| format!("{adapter}/"));

    objects
        .iter()
        .filter_map(|(path, interfaces)| {
            if prefix
                .as_ref()
                .is_some_and(|prefix| !path.as_str().starts_with(prefix.as_str()))
            {
                return None;
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Active;
    use zbus::zvariant::{ObjectPath, OwnedValue, Value};

    const OWNER: &str = ":1.42";
    const HEADPHONES: &str = "/org/bluez/hci0/dev_AA_BB_CC_DD_EE_FF";

    fn properties_changed(
        sender: &str,
        path: &str,
        interface: &str,
        changed: &[(&str, Value<'_>)],
    ) -> Message {
        let changed: HashMap<&str, &Value<'_>> =
            changed.iter().map(|(name, value)| (*name, value)).collect();
        Message::signal(path, "org.freedesktop.DBus.Properties", "PropertiesChanged")
            .unwrap()
            .sender(sender)
            .unwrap()
            .build(&(interface, changed, Vec::<&str>::new()))
            .unwrap()
    }

    fn interfaces_added(path: &str, interfaces: &[(&str, &[(&str, Value<'_>)])]) -> Message {
        let interfaces: HashMap<&str, HashMap<&str, &Value<'_>>> = interfaces
            .iter()
            .map(|(interface, properties)| {
                let properties = properties
                    .iter()
                    .map(|(name, value)| (*name, value))
                    .collect();
                (*interface, properties)
            })
            .collect();
        Message::signal("/", "org.freedesktop.DBus.ObjectManager", "InterfacesAdded")
            .unwrap()
            .sender(OWNER)
            .unwrap()
            .build(&(ObjectPath::try_from(path).unwrap(), interfaces))
            .unwrap()
    }

    fn interfaces_removed(path: &str, interfaces: &[&str]) -> Message {
        Message::signal(
            "/",
            "org.freedesktop.DBus.ObjectManager",
            "InterfacesRemoved",
        )
        .unwrap()
        .sender(OWNER)
        .unwrap()
        .build(&(ObjectPath::try_from(path).unwrap(), interfaces))
        .unwrap()
    }

    fn name_owner_changed(sender: &str, name: &str, old: &str, new: &str) -> Message {
        Message::signal("/org/freedesktop/DBus", BUS, "NameOwnerChanged")
            .unwrap()
            .sender(sender)
            .unwrap()
            .build(&(name, old, new))
            .unwrap()
    }

    fn headphones() -> Vec<(&'static str, Value<'static>)> {
        vec![
            ("Address", Value::from("AA:BB:CC:DD:EE:FF")),
            (
                "Adapter",
                Value::from(ObjectPath::from_static_str_unchecked("/org/bluez/hci0")),
            ),
            ("Alias", Value::from("Headphones")),
            ("Icon", Value::from("audio-headphones")),
            ("Paired", Value::from(true)),
            ("Connected", Value::from(true)),
        ]
    }

    fn path(path: &str) -> OwnedObjectPath {
        OwnedObjectPath::try_from(path).unwrap()
    }

    #[test]
    fn test_device_changes() {
        let events = changes(&properties_changed(
            OWNER,
            HEADPHONES,
            DEVICE_INTERFACE,
            &[
                ("Connected", Value::from(false)),
                ("RSSI", Value::from(-60i16)),
            ],
        ))
        .unwrap();
        assert!(matches!(
            &events[..],
            [Event::UpdatedDevice(p, updates)] if *p == path(HEADPHONES)
                && matches!(&updates[..], [DeviceUpdate::Enabled(Active::Disabled)])
        ));

        // Changes that don't matter here send nothing.
        let events = changes(&properties_changed(
            OWNER,
            HEADPHONES,
            DEVICE_INTERFACE,
            &[("RSSI", Value::from(-60i16))],
        ))
        .unwrap();
        assert!(events.is_empty());
    }

    #[test]
    fn test_media_control_is_not_the_device() {
        let events = changes(&properties_changed(
            OWNER,
            HEADPHONES,
            "org.bluez.MediaControl1",
            &[("Connected", Value::from(false))],
        ))
        .unwrap();
        assert!(events.is_empty(), "{events:?}");

        let events = changes(&interfaces_added(
            HEADPHONES,
            &[(
                "org.bluez.MediaControl1",
                &[("Connected", Value::from(true))],
            )],
        ))
        .unwrap();
        assert!(events.is_empty(), "{events:?}");
    }

    #[test]
    fn test_battery_changes() {
        let events = changes(&properties_changed(
            OWNER,
            HEADPHONES,
            BATTERY_INTERFACE,
            &[("Percentage", Value::from(50u8))],
        ))
        .unwrap();
        assert!(matches!(
            &events[..],
            [Event::UpdatedDevice(_, updates)]
                if matches!(&updates[..], [DeviceUpdate::Battery(Some(b))] if b == "50")
        ));

        let events = changes(&interfaces_added(
            HEADPHONES,
            &[(BATTERY_INTERFACE, &[("Percentage", Value::from(80u8))])],
        ))
        .unwrap();
        assert!(matches!(
            &events[..],
            [Event::UpdatedDevice(p, updates)] if *p == path(HEADPHONES)
                && matches!(&updates[..], [DeviceUpdate::Battery(Some(b))] if b == "80")
        ));

        let events = changes(&interfaces_removed(HEADPHONES, &[BATTERY_INTERFACE])).unwrap();
        assert!(matches!(
            &events[..],
            [Event::UpdatedDevice(_, updates)]
                if matches!(&updates[..], [DeviceUpdate::Battery(None)])
        ));
    }

    #[test]
    fn test_added_and_removed() {
        let device = headphones();
        let events = changes(&interfaces_added(
            HEADPHONES,
            &[
                (DEVICE_INTERFACE, &device),
                (BATTERY_INTERFACE, &[("Percentage", Value::from(90u8))]),
            ],
        ))
        .unwrap();
        assert!(matches!(
            &events[..],
            [Event::AddedDevice(p, device)] if *p == path(HEADPHONES)
                && device.alias.as_deref() == Some("Headphones")
                && device.is_connected()
                && device.battery.as_deref() == Some("90")
        ));

        let events = changes(&interfaces_added(
            "/org/bluez/hci1",
            &[(
                ADAPTER_INTERFACE,
                &[
                    ("Address", Value::from("00:11:22:33:44:55")),
                    ("Alias", Value::from("dongle")),
                    ("Powered", Value::from(true)),
                ],
            )],
        ))
        .unwrap();
        assert!(matches!(
            &events[..],
            [Event::AddedAdapter(p, adapter)] if *p == path("/org/bluez/hci1")
                && adapter.alias == "dongle"
                && adapter.enabled == Active::Enabled
        ));

        let events = changes(&interfaces_removed(
            HEADPHONES,
            &[
                DEVICE_INTERFACE,
                BATTERY_INTERFACE,
                "org.freedesktop.DBus.Properties",
            ],
        ))
        .unwrap();
        assert!(matches!(&events[..], [Event::RemovedDevice(p)] if *p == path(HEADPHONES)));

        let events = changes(&interfaces_removed("/org/bluez/hci1", &[ADAPTER_INTERFACE])).unwrap();
        assert!(matches!(&events[..], [Event::RemovedAdapter(p)] if *p == path("/org/bluez/hci1")));
    }

    #[test]
    fn test_adapter_changes() {
        let events = changes(&properties_changed(
            OWNER,
            "/org/bluez/hci0",
            ADAPTER_INTERFACE,
            &[
                ("Powered", Value::from(false)),
                ("Class", Value::from(0u32)),
            ],
        ))
        .unwrap();
        assert!(matches!(
            &events[..],
            [Event::UpdatedAdapter(p, updates)] if *p == path("/org/bluez/hci0")
                && matches!(&updates[..], [AdapterUpdate::Enabled(Active::Disabled)])
        ));
    }

    #[test]
    fn test_owner_left() {
        let owner = UniqueName::try_from(OWNER).unwrap();
        assert!(left(&name_owner_changed(BUS, SERVICE, OWNER, ""), &owner));
        assert!(!left(&name_owner_changed(BUS, SERVICE, "", OWNER), &owner));
        assert!(!left(&name_owner_changed(BUS, SERVICE, ":1.7", ""), &owner));
        assert!(!left(
            &name_owner_changed(BUS, "org.example", OWNER, ""),
            &owner
        ));
        // Only the bus says who owns a name.
        assert!(!left(
            &name_owner_changed(":1.9", SERVICE, OWNER, ""),
            &owner
        ));
    }

    #[test]
    fn test_objects() {
        let properties = |properties: &[(&str, Value<'_>)]| -> HashMap<String, OwnedValue> {
            properties
                .iter()
                .map(|(name, value)| ((*name).to_owned(), value.try_to_owned().unwrap()))
                .collect()
        };
        let interface = |name: &str| zbus::names::OwnedInterfaceName::try_from(name).unwrap();

        let mut objects = fdo::ManagedObjects::new();
        objects.insert(
            path("/org/bluez"),
            HashMap::from([(interface("org.bluez.AgentManager1"), HashMap::new())]),
        );
        objects.insert(
            path("/org/bluez/hci0"),
            HashMap::from([(
                interface(ADAPTER_INTERFACE),
                properties(&[
                    ("Address", Value::from("00:11:22:33:44:55")),
                    ("Powered", Value::from(true)),
                ]),
            )]),
        );
        objects.insert(
            path(HEADPHONES),
            HashMap::from([
                (interface(DEVICE_INTERFACE), properties(&headphones())),
                (
                    interface(BATTERY_INTERFACE),
                    properties(&[("Percentage", Value::from(10u8))]),
                ),
            ]),
        );
        // Not a usable device: one bad device doesn't keep the others away.
        objects.insert(
            path("/org/bluez/hci0/dev_00"),
            HashMap::from([(interface(DEVICE_INTERFACE), properties(&[]))]),
        );
        objects.insert(
            path("/org/bluez/hci1/dev_11_22_33_44_55_66"),
            HashMap::from([(
                interface(DEVICE_INTERFACE),
                properties(&[
                    ("Address", Value::from("11:22:33:44:55:66")),
                    (
                        "Adapter",
                        Value::from(ObjectPath::from_static_str_unchecked("/org/bluez/hci1")),
                    ),
                ]),
            )]),
        );

        let adapters = adapters(&objects);
        assert_eq!(adapters.len(), 1);
        assert_eq!(adapters[&path("/org/bluez/hci0")].enabled, Active::Enabled);

        let all = devices(&objects, None);
        assert_eq!(all.len(), 2);
        assert_eq!(all[&path(HEADPHONES)].battery.as_deref(), Some("10"));

        let hci0 = devices(&objects, Some(&path("/org/bluez/hci0")));
        assert_eq!(hci0.len(), 1);
        assert!(hci0.contains_key(&path(HEADPHONES)));
    }
}
