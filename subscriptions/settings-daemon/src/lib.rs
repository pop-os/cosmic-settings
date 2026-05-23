// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use std::hash::Hash;

use futures::{FutureExt, StreamExt};
use iced_futures::Subscription;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub(crate) struct Wrapper {
    id: &'static str,
    conn: zbus::Connection,
}

impl Hash for Wrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

const DAEMON_NAME: &str = "com.system76.CosmicSettingsDaemon";

async fn wait_for_daemon(conn: &zbus::Connection) {
    let Ok(dbus) = zbus::fdo::DBusProxy::new(conn).await else {
        return;
    };
    let name: zbus::names::BusName = DAEMON_NAME.try_into().unwrap();
    if dbus.name_has_owner(name.clone()).await.unwrap_or(false) {
        return;
    }
    log::info!("Waiting for {DAEMON_NAME} to appear on D-Bus...");
    if let Ok(mut stream) = dbus.receive_name_owner_changed().await {
        while let Some(signal) = stream.next().await {
            if let Ok(args) = signal.args()
                && args.name == name
                && args.new_owner.as_ref().is_some_and(|n| !n.is_empty())
            {
                break;
            }
        }
    }
}

pub fn subscription(connection: zbus::Connection) -> iced_futures::Subscription<Event> {
    Subscription::run_with(
        Wrapper {
            id: "settings-daemon",
            conn: connection,
        },
        |Wrapper {
             id: _id,
             conn: connection,
         }| {
            let connection = connection.clone();
            async move {
                wait_for_daemon(&connection).await;

                let settings_daemon = match CosmicSettingsDaemonProxy::new(&connection).await {
                    Ok(value) => value,
                    Err(err) => {
                        log::error!("Error connecting to settings daemon: {}", err);
                        return futures::future::pending().await;
                    }
                };

                let (tx, rx) = unbounded_channel();

                let max_brightness_stream = settings_daemon
                    .receive_max_display_brightness_changed()
                    .await;
                let brightness_stream = settings_daemon.receive_display_brightness_changed().await;

                let mut initial = vec![Event::Sender(tx)];
                if let Ok(max) = settings_daemon.max_display_brightness().await {
                    initial.push(Event::MaxDisplayBrightness(max));
                }
                if let Ok(brightness) = settings_daemon.display_brightness().await {
                    initial.push(Event::DisplayBrightness(brightness));
                }

                futures::stream::iter(initial).chain(futures::stream_select!(
                    Box::pin(UnboundedReceiverStream::new(rx).filter_map(move |request| {
                        let settings_daemon = settings_daemon.clone();
                        async move {
                            match request {
                                Request::SetDisplayBrightness(brightness) => {
                                    let _ =
                                        settings_daemon.set_display_brightness(brightness).await;
                                    None
                                }
                                Request::GetDisplayBrightness => {
                                    let b = settings_daemon.display_brightness().await;
                                    Some(Event::DisplayBrightness(b.ok()?))
                                }
                                Request::GetMaxDisplayBrightness => {
                                    let m = settings_daemon.max_display_brightness().await;
                                    Some(Event::MaxDisplayBrightness(m.ok()?))
                                }
                            }
                        }
                    })),
                    Box::pin(max_brightness_stream.filter_map(|evt| async move {
                        Some(Event::MaxDisplayBrightness(evt.get().await.ok()?))
                    })),
                    Box::pin(brightness_stream.filter_map(|evt| async move {
                        Some(Event::DisplayBrightness(evt.get().await.ok()?))
                    }))
                ))
            }
            .flatten_stream()
        },
    )
}

#[derive(Clone, Debug)]
pub enum Event {
    Sender(UnboundedSender<Request>),
    MaxDisplayBrightness(i32),
    DisplayBrightness(i32),
}

#[zbus::proxy(
    default_service = "com.system76.CosmicSettingsDaemon",
    interface = "com.system76.CosmicSettingsDaemon",
    default_path = "/com/system76/CosmicSettingsDaemon"
)]
trait CosmicSettingsDaemon {
    #[zbus(property)]
    fn display_brightness(&self) -> zbus::Result<i32>;
    #[zbus(property)]
    fn set_display_brightness(&self, value: i32) -> zbus::Result<()>;
    #[zbus(property)]
    fn max_display_brightness(&self) -> zbus::Result<i32>;
    #[zbus(property)]
    fn keyboard_brightness(&self) -> zbus::Result<i32>;
}

#[derive(Debug, Clone)]
pub enum Request {
    SetDisplayBrightness(i32),
    GetDisplayBrightness,
    GetMaxDisplayBrightness,
}
