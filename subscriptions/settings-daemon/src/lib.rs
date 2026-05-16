// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

// XXX error handling?

use std::{hash::Hash, time::Duration};

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
                let count = 5;
                let mut settings_daemon = None;
                for _ in 0..5 {
                    if count == 4 {
                        return futures::future::pending().await;
                    }
                    match CosmicSettingsDaemonProxy::new(&connection).await {
                        Ok(value) => {
                            // interface methods can be called
                            // FIXME why does this fail sometimes??
                            if let Err(err) = value.display_brightness().await {
                                log::error!("Error connecting to settings daemon: {}", err);

                                tokio::time::sleep(Duration::from_secs(1)).await;
                                continue;
                            }
                            if let Err(err) = value.max_display_brightness().await {
                                log::error!("Error connecting to settings daemon: {}", err);
                                tokio::time::sleep(Duration::from_secs(1)).await;

                                continue;
                            }
                            settings_daemon = Some(value);

                            break;
                        }
                        Err(err) => {
                            log::error!("Error connecting to settings daemon: {}", err);

                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    };
                }
                let settings_daemon = settings_daemon.unwrap();
                let (tx, rx) = unbounded_channel();

                let max_brightness_stream = settings_daemon
                    .receive_max_display_brightness_changed()
                    .await;

                let brightness_stream = settings_daemon.receive_display_brightness_changed().await;
                let mut init = vec![Event::Sender(tx)];

                let initial = futures::stream::iter(init);

                initial.chain(futures::stream_select!(
                    Box::pin(UnboundedReceiverStream::new(rx).filter_map(move |request| {
                        let settings_daemon = settings_daemon.clone();
                        async move {
                            match request {
                                Request::SetDisplayBrightness(brightness) => {
                                    let _ =
                                        settings_daemon.set_display_brightness(brightness).await;
                                }
                                Request::GetDisplayBrightness => {
                                    let b = settings_daemon.display_brightness().await;
                                    return Some(Event::DisplayBrightness(b.ok()?));
                                }
                                Request::GetMaxDisplayBrightness => {
                                    let m = settings_daemon.max_display_brightness().await;
                                    return Some(Event::MaxDisplayBrightness(m.ok()?));
                                }
                            }
                            None::<Event>
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
