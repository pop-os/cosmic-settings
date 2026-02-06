// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic_dbus_a11y::*;
use futures::{self, SinkExt, StreamExt};
use iced_futures::{Subscription, stream};
use std::fmt::Debug;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use zbus::Connection;

#[derive(Debug, Clone)]
pub enum Response {
    Error(String),
    IsEnabled(bool),
    ScreenReader(bool),
    Init(bool, UnboundedSender<Request>),
}

pub enum Request {
    /// Enable the org.a11y.Bus
    Enable(bool),
    /// Enable the screen reader feature of org.a11y.Bus
    ScreenReader(bool),
}

#[derive(Debug)]
pub struct State {
    conn: Connection,
    retry: u8,
    enabled: bool,
    rx: UnboundedReceiver<Request>,
}

pub fn subscription() -> Subscription<Response> {
    struct MyId;

    Subscription::run_with_id(
        std::any::TypeId::of::<MyId>(),
        stream::channel(1, move |mut output| async move {
            if let Some(state) = State::new(&mut output).await {
                state.listen(&mut output).await;
            }

            futures::future::pending::<()>().await;
        }),
    )
}

impl State {
    pub async fn new(output: &mut futures::channel::mpsc::Sender<Response>) -> Option<Self> {
        let conn = match Connection::session().await.map_err(|e| e.to_string()) {
            Ok(conn) => conn,
            Err(e) => {
                _ = output.send(Response::Error(e)).await;
                return None;
            }
        };
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let mut enabled = false;
        if let Ok(proxy) = StatusProxy::new(&conn).await {
            if let Ok(status) = proxy.screen_reader_enabled().await {
                enabled = status;
            }
        }
        _ = output.send(Response::Init(enabled, tx)).await;
        Some(State {
            conn,
            retry: 20,
            enabled,
            rx,
        })
    }

    pub async fn listen(mut self, output: &mut futures::channel::mpsc::Sender<Response>) {
        loop {
            let Ok(proxy) = StatusProxy::new(&self.conn).await else {
                if self.retry == 0 {
                    tracing::error!("Accessibility Status is unavailable.");
                    return;
                } else {
                    _ = tokio::time::sleep(tokio::time::Duration::from_secs(
                        2_u64.pow(self.retry as u32),
                    ))
                    .await;
                    self.retry -= 1;
                    continue;
                }
            };

            self.retry = 20;

            let Ok(properties_proxy) =
                zbus::fdo::PropertiesProxy::new(&self.conn, "org.a11y.Bus", "/org/a11y/bus").await
            else {
                tracing::error!("org.a11y.Bus properties proxy failed");
                return;
            };

            let Ok(mut properties_changed_stream) =
                properties_proxy.receive_properties_changed().await
            else {
                tracing::error!("org.a11y.Bus receive properties changed failed");
                return;
            };

            if let Ok(status) = proxy.screen_reader_enabled().await {
                if self.enabled != status {
                    _ = output.send(Response::ScreenReader(self.enabled)).await;
                }
                self.enabled = status;
            }

            let requests_fut = Box::pin(async {
                while let Some(request) = self.rx.recv().await {
                    match request {
                        Request::ScreenReader(is_enabled) => {
                            _ = proxy.set_is_enabled(is_enabled).await;
                            _ = proxy.set_screen_reader_enabled(is_enabled).await;
                        }

                        Request::Enable(is_enabled) => {
                            _ = proxy.set_is_enabled(is_enabled).await;
                        }
                    }
                }
            });

            let properties_fut = Box::pin(async {
                while let Some(signal) = properties_changed_stream.next().await {
                    if let Ok(args) = signal.args() {
                        for (name, value) in args.changed_properties().iter() {
                            match *name {
                                "IsEnabled" => {
                                    if let Ok(status) = value.downcast_ref::<bool>() {
                                        _ = output.send(Response::IsEnabled(status)).await;
                                    }
                                }

                                "ScreenReaderEnabled" => {
                                    if let Ok(status) = value.downcast_ref::<bool>() {
                                        _ = output.send(Response::ScreenReader(status)).await;
                                    }
                                }

                                _ => (),
                            }
                        }
                    }
                }
            });

            futures::future::select(properties_fut, requests_fut).await;
        }
    }
}
