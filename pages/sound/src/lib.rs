// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod model;
use std::sync::Arc;

pub use cosmic_settings_audio_client as audio_client;
use futures::{SinkExt, StreamExt};

pub async fn subscribe<T>(
    mut emitter: futures::channel::mpsc::Sender<T>,
    apply_fn: fn(Message) -> T,
) {
    loop {
        let mut client = match audio_client::connect().await {
            Ok(client) => client,
            Err(why) => {
                if let audio_client::zlink::Error::Io(ref why) = why
                    && why.kind() == std::io::ErrorKind::NotFound
                {
                    tracing::error!("cosmic-settings-daemon varlink service not found.");
                } else {
                    tracing::error!(
                        ?why,
                        "failed to connect to cosmic-settings's varlink service"
                    );
                }

                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                continue;
            }
        };

        if let Ok(Ok(mut stream)) = client.recv_events().await {
            _ = emitter
                .send(apply_fn(Message::Client(Arc::new(client))))
                .await;
            while let Some(message) = stream.next().await {
                match message {
                    Ok(event) => {
                        _ = emitter.send(apply_fn(Message::Subscription(event))).await;
                    }
                    Err(why) => {
                        tracing::error!(?why, "event error");
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    /// Connection to `com.system76.CosmicSettings`.
    Client(Arc<audio_client::Client>),
    /// Messages from the varlink audio client,
    Subscription(audio_client::Event),
}
