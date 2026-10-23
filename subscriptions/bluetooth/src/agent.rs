// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! The agent that BlueZ asks when pairing with a device, or when a device wants to use a
//! service of the computer.

use crate::Event;

use std::sync::Arc;

use bluez_zbus::agent_manager1::AgentManager1Proxy;
use futures::channel::{mpsc, oneshot};
use futures::{SinkExt, StreamExt};
use tracing::Instrument;
use zbus::zvariant::{ObjectPath, OwnedObjectPath};

const AGENT_PATH: &str = "/org/bluez/agent/cosmic_settings";

/// A request of BlueZ to the agent.
///
/// A request with a `response` waits for it. Dropping the `response` rejects the request.
#[derive(Debug)]
pub enum Message {
    /// A device wants to use a service of the computer.
    AuthorizeService {
        device: OwnedObjectPath,
        uuid: String,
        response: oneshot::Sender<bool>,
    },
    /// BlueZ no longer needs the answer to the last request, or the code on display.
    Cancel,
    DisplayPasskey {
        device: OwnedObjectPath,
        passkey: u32,
        entered: u16,
    },
    DisplayPinCode {
        device: OwnedObjectPath,
        pincode: String,
    },
    /// BlueZ no longer uses the agent.
    Release,
    /// A device wants to pair without a code.
    RequestAuthorization {
        device: OwnedObjectPath,
        response: oneshot::Sender<bool>,
    },
    RequestConfirmation {
        device: OwnedObjectPath,
        passkey: u32,
        response: oneshot::Sender<bool>,
    },
    RequestPasskey {
        device: OwnedObjectPath,
        response: oneshot::Sender<Option<u32>>,
    },
    RequestPinCode {
        device: OwnedObjectPath,
        response: oneshot::Sender<Option<String>>,
    },
}

#[derive(Debug, zbus::DBusError)]
#[zbus(prefix = "org.bluez.Error")]
enum Error {
    #[zbus(error)]
    ZBus(zbus::Error),
    Rejected,
    Canceled,
}

struct Agent {
    messages: mpsc::Sender<Message>,
}

impl Agent {
    async fn tell(&self, message: Message) {
        _ = self.messages.clone().send(message).await;
    }

    async fn ask<T>(
        &self,
        message: impl FnOnce(oneshot::Sender<T>) -> Message,
    ) -> Result<T, Error> {
        let (response, answer) = oneshot::channel();
        if self.messages.clone().send(message(response)).await.is_err() {
            return Err(Error::Canceled);
        }
        answer.await.map_err(|_| Error::Canceled)
    }
}

fn accepted(answer: bool) -> Result<(), Error> {
    if answer { Ok(()) } else { Err(Error::Rejected) }
}

/// The methods take `&self`, so that BlueZ can cancel a request while it waits for the user.
#[zbus::interface(name = "org.bluez.Agent1")]
impl Agent {
    async fn release(&self) {
        tracing::debug!("release");
        self.tell(Message::Release).await;
    }

    async fn request_pin_code(&self, device: OwnedObjectPath) -> Result<String, Error> {
        tracing::debug!(?device, "request_pin_code");
        self.ask(|response| Message::RequestPinCode { device, response })
            .await?
            .ok_or(Error::Rejected)
    }

    async fn display_pin_code(&self, device: OwnedObjectPath, pincode: String) {
        tracing::debug!(?device, "display_pin_code");
        self.tell(Message::DisplayPinCode { device, pincode }).await;
    }

    async fn request_passkey(&self, device: OwnedObjectPath) -> Result<u32, Error> {
        tracing::debug!(?device, "request_passkey");
        self.ask(|response| Message::RequestPasskey { device, response })
            .await?
            .ok_or(Error::Rejected)
    }

    async fn display_passkey(&self, device: OwnedObjectPath, passkey: u32, entered: u16) {
        tracing::debug!(?device, entered, "display_passkey");
        self.tell(Message::DisplayPasskey {
            device,
            passkey,
            entered,
        })
        .await;
    }

    async fn request_confirmation(
        &self,
        device: OwnedObjectPath,
        passkey: u32,
    ) -> Result<(), Error> {
        tracing::debug!(?device, "request_confirmation");
        accepted(
            self.ask(|response| Message::RequestConfirmation {
                device,
                passkey,
                response,
            })
            .await?,
        )
    }

    async fn request_authorization(&self, device: OwnedObjectPath) -> Result<(), Error> {
        tracing::debug!(?device, "request_authorization");
        accepted(
            self.ask(|response| Message::RequestAuthorization { device, response })
                .await?,
        )
    }

    async fn authorize_service(&self, device: OwnedObjectPath, uuid: String) -> Result<(), Error> {
        tracing::debug!(?device, uuid, "authorize_service");
        accepted(
            self.ask(|response| Message::AuthorizeService {
                device,
                uuid,
                response,
            })
            .await?,
        )
    }

    async fn cancel(&self) {
        tracing::debug!("cancel");
        self.tell(Message::Cancel).await;
    }
}

pub async fn unregister(connection: zbus::Connection) -> zbus::Result<()> {
    let agent_path = ObjectPath::from_static_str_unchecked(AGENT_PATH);
    let bluez = bluez_zbus::agent_manager1::AgentManager1Proxy::new(&connection).await?;
    bluez.unregister_agent(&agent_path).await
}

async fn register(
    bluez: &AgentManager1Proxy<'_>,
    agent_path: &ObjectPath<'_>,
    default: bool,
) -> zbus::Result<()> {
    bluez
        .register_agent(
            agent_path,
            <&'static str>::from(bluez_zbus::agent1::Capability::KeyboardDisplay),
        )
        .await?;

    if default && let Err(why) = bluez.request_default_agent(agent_path).await {
        _ = bluez.unregister_agent(agent_path).await;
        return Err(why);
    }

    Ok(())
}

/// Registers the agent, and sends the requests of BlueZ as [`Event::Agent`] until `tx` closes.
///
/// BlueZ asks the agent about the pairings that this connection starts. The default agent also
/// gets the requests that no application started, such as a device that pairs with the
/// computer by itself, or wants to use one of its services.
pub async fn watch(
    connection: zbus::Connection,
    mut tx: mpsc::Sender<Event>,
    default: bool,
) -> zbus::Result<()> {
    async move {
        let agent_path = ObjectPath::from_static_str_unchecked(AGENT_PATH);
        let (messages, mut receiver) = mpsc::channel(8);

        tracing::debug!("connecting agent");

        // Replace the agent of an earlier watch on this connection, which nobody listens to.
        let object_server = connection.object_server();
        _ = object_server.remove::<Agent, _>(&agent_path).await;
        object_server.at(&agent_path, Agent { messages }).await?;

        tracing::debug!("connecting to bluez agent manager");

        let bluez = AgentManager1Proxy::new(&connection).await?;
        _ = bluez.unregister_agent(&agent_path).await;

        tracing::debug!("registering agent");

        if let Err(why) = register(&bluez, &agent_path, default).await {
            _ = object_server.remove::<Agent, _>(&agent_path).await;
            return Err(why);
        }

        tracing::debug!("registered");

        while let Some(msg) = receiver.next().await {
            tracing::debug!(?msg, "agent message received");

            if tx.send(Event::Agent(Arc::new(msg))).await.is_err() {
                break;
            }
        }

        _ = bluez.unregister_agent(&agent_path).await;
        _ = object_server.remove::<Agent, _>(&agent_path).await;

        tracing::debug!("exiting");

        Ok(())
    }
    .instrument(tracing::info_span!("bluetooth::agent::watch"))
    .await
}
