// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

// XXX error handling?

use futures::{FutureExt, StreamExt};
use iced_futures::Subscription;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub fn subscription(connection: zbus::Connection) -> iced_futures::Subscription<Event> {
    Subscription::run_with_id(
        "settings-daemon",
        async move {
            let settings_daemon = match CosmicSettingsDaemonProxy::new(&connection).await {
                Ok(value) => value,
                Err(err) => {
                    log::error!("Error connecting to settings daemon: {}", err);
                    futures::future::pending().await
                }
            };

            let (tx, rx) = unbounded_channel();

            let max_brightness_stream = settings_daemon
                .receive_max_display_brightness_changed()
                .await;
            let brightness_stream = settings_daemon.receive_display_brightness_changed().await;

            let initial = futures::stream::iter([Event::Sender(tx)]);

            initial.chain(futures::stream_select!(
                Box::pin(UnboundedReceiverStream::new(rx).filter_map(move |request| {
                    let settings_daemon = settings_daemon.clone();
                    async move {
                        match request {
                            Request::SetDisplayBrightness(brightness) => {
                                let _ = settings_daemon.set_display_brightness(brightness).await;
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
        .flatten_stream(),
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
}
