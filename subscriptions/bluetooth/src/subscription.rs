// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use crate::{AdapterUpdate, Device, DeviceUpdate, Event};
use std::pin::Pin;

use bluez_zbus::BluetoothDevice;
use futures::{channel::mpsc, stream::FusedStream};
use iced_futures::futures::{SinkExt, StreamExt};
use zbus::{fdo, zvariant::OwnedObjectPath};

enum DevicePropertyWatcherTask {
    Add(OwnedObjectPath),
    Removed(OwnedObjectPath),
}

struct DevicePropertyWatcher {
    stream: futures::stream::SelectAll<SignalWatcher>,
    rx: mpsc::Receiver<DevicePropertyWatcherTask>,
}

struct SignalWatcher {
    stream: zbus::fdo::PropertiesChangedStream,
    path: OwnedObjectPath,
}

impl futures::Stream for SignalWatcher {
    type Item = zbus::fdo::PropertiesChanged;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        futures::Stream::poll_next(Pin::new(&mut self.stream), cx)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}

impl DevicePropertyWatcher {
    fn new() -> (Self, mpsc::Sender<DevicePropertyWatcherTask>) {
        let stream = futures::stream::select_all(vec![]);
        let (tx, rx) = mpsc::channel(10);

        (Self { stream, rx }, tx)
    }
    async fn insert(
        &mut self,
        connection: &zbus::Connection,
        path: OwnedObjectPath,
    ) -> zbus::Result<()> {
        if let Some(signal) = self.stream.iter_mut().find(|s| s.path.eq(&path)) {
            if signal.stream.is_terminated() {
                let property_proxy =
                    zbus::fdo::PropertiesProxy::new(connection, "org.bluez", path.clone()).await?;
                signal.stream = property_proxy.receive_properties_changed().await?;
            }
            return Ok(());
        }
        let property_proxy =
            zbus::fdo::PropertiesProxy::new(connection, "org.bluez", path.clone()).await?;
        let stream = property_proxy.receive_properties_changed().await?;
        self.stream.push(SignalWatcher { stream, path });
        Ok(())
    }
    fn remove(mut self, path: &OwnedObjectPath) -> Self {
        self.stream =
            futures::stream::select_all(self.stream.into_iter().filter(|p| !p.path.eq(path)));
        self
    }
}

/// Watching new/removed devices, connected state changed
pub async fn watch(connection: zbus::Connection, mut tx: futures::channel::mpsc::Sender<Event>) {
    let span = tracing::span!(tracing::Level::INFO, "bluetooth::subscription::watch");
    let _span = span.enter();

    loop {
        let result = async {
            let managed_object_proxy =
                zbus::fdo::ObjectManagerProxy::new(&connection, "org.bluez", "/")
                    .await?;

            let mut receive_interfaces_added = managed_object_proxy
                .receive_interfaces_added()
                .await?;
            let mut receive_interfaces_removed = managed_object_proxy
                .receive_interfaces_removed()
                .await?;

            let (mut property_watcher, mut property_watcher_task) = DevicePropertyWatcher::new();

            for (path, interfaces) in managed_object_proxy.get_managed_objects().await? {
                if interfaces.contains_key("org.bluez.Device1")
                || interfaces.contains_key("org.bluez.Adapter1")
                || interfaces.contains_key("org.bluez.Battery1")
                {
                    property_watcher.insert(&connection, path).await?;
                }
            }

            while !property_watcher.rx.is_terminated() {
                futures::select! {
                    task = property_watcher.rx.next() => match task {
                        Some(DevicePropertyWatcherTask::Add(path)) => {
                            property_watcher.insert(&connection, path).await?;
                        }
                        Some(DevicePropertyWatcherTask::Removed(path)) => {
                            property_watcher = property_watcher.remove(&path);
                        }
                        None => {
                            tracing::error!("Bluetooth property watcher has shutdown unexpectedly");
                        }
                    },
                    signal = property_watcher.stream.next() => match signal {
                        Some(signal) => {
                            let args = signal.args()?;
                            let header = signal.message().header();
                            match header.path() {
                                Some(path) if path.contains("/dev_") =>
                                    tx
                                        .send(Event::UpdatedDevice(path.to_owned().into(), DeviceUpdate::from_update(args.changed_properties)))
                                        .await
                                        .map_err(|e| zbus::Error::Failure(e.to_string()))?,
                                Some(path) => tx
                                        .send(Event::UpdatedAdapter(path.to_owned().into(), AdapterUpdate::from_update(args.changed_properties)))
                                        .await
                                        .map_err(|e| zbus::Error::Failure(e.to_string()))?,
                                None => continue
                            }
                        }
                        None => {
                            tracing::error!("Bluetooth object watcher has shutdown unexpectedly");
                        }
                    },
                    signal = receive_interfaces_added.next() => match signal {
                        Some(signal) => {
                            let args = signal.args()?;
                            match BluetoothDevice::new(&connection, args.object_path.clone()).await {
                                Ok(device) => {
                                    match Device::from_device(&device).await {
                                        Ok(device) => {
                                            property_watcher_task
                                                .send(DevicePropertyWatcherTask::Add(args.object_path.to_owned().into())).await.map_err(|e| zbus::Error::Failure(e.to_string()))?;

                                            tx
                                                .send(Event::AddedDevice(args.object_path.to_owned().into(), device))
                                                .await
                                                .map_err(|e| zbus::Error::Failure(e.to_string()))?;

                                        }
                                        Err(why) => {
                                            tracing::warn!("Cannot deserialise device: {why}");
                                        }
                                    }
                                }
                                Err(zbus::Error::InterfaceNotFound) => continue,
                                Err(e) => return Err(e),
                            }
                        }
                        None => {
                            tracing::error!("Bluetooth object watcher has shutdown unexpectedly");
                        }
                    },
                    signal = receive_interfaces_removed.next() => match signal {
                        Some(signal) => {
                            let args = signal.args()?;
                            if args.interfaces.iter().any(|i| i == "org.bluez.Device1") {
                                property_watcher_task.send(DevicePropertyWatcherTask::Removed(
                                    args.object_path.to_owned().into(),
                                )).await.map_err(|e| zbus::Error::Failure(e.to_string()))?;
                                tx
                                    .send(Event::RemovedDevice(args.object_path.to_owned().into()))
                                    .await
                                    .map_err(|e| zbus::Error::Failure(e.to_string()))?;

                            } else if args.interfaces.iter().any(|i| i == "org.bluez.Battery1") {
                                tx
                                    .send(Event::UpdatedDevice(args.object_path.to_owned().into(), vec![DeviceUpdate::Battery(None)]))
                                    .await
                                    .map_err(|e| zbus::Error::Failure(e.to_string()))?;
                            } else if args.interfaces.iter().any(|i| i == "org.bluez.Adapter1") {
                                tx
                                    .send(Event::RemovedAdapter(args.object_path.to_owned().into()))
                                    .await
                                    .map_err(|e| zbus::Error::Failure(e.to_string()))?;
                            }
                        },
                        None => {
                            tracing::error!("Bluetooth object watcher has shutdown unexpectedly");
                        }
                    },
                }
            }
            tracing::warn!("bluetooth event loop gracefully terminated");
            Ok(())
        }.await;

        if let Err(why) = result {
            _ = tx.send(Event::DBusError(why.clone())).await;

            tracing::error!("failed to watch bluetooth event: {why:?}.");

            // Exit if the dbus service is not found.
            if let zbus::Error::FDO(fdo_error) = why {
                match *fdo_error {
                    fdo::Error::ServiceUnknown(_) => {
                        tracing::error!(
                            "The org.bluez dbus service is unknown. Is the bluez service installed and activatable?"
                        );
                        _ = tx.send(Event::DBusServiceUnknown).await;
                        return;
                    }

                    fdo::Error::NameHasNoOwner(_) => {
                        tracing::error!("The org.bluez dbus service is not enabled or active");
                        _ = tx.send(Event::NameHasNoOwner).await;
                        return;
                    }

                    _ => (),
                }
            }
        }
    }
}
