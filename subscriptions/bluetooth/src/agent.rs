// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::Event;

use std::sync::Arc;

use cosmic_bluez_zbus::agent_manager1::AgentManager1Proxy;
use futures::future::Either;
use futures::{SinkExt, StreamExt};
use zbus::ObjectPath;

const AGENT_PATH: &str = "/org/bluez/agent/cosmic_settings";

pub async fn unregister(connection: zbus::Connection) -> zbus::Result<()> {
    let agent_path = ObjectPath::from_static_str_unchecked(AGENT_PATH);
    let bluez = cosmic_bluez_zbus::agent_manager1::AgentManager1Proxy::new(&connection).await?;
    bluez.unregister_agent(&agent_path).await
}

pub async fn watch(
    connection: zbus::Connection,
    mut tx: futures::channel::mpsc::Sender<Event>,
) -> zbus::Result<()> {
    let span = tracing::span!(tracing::Level::INFO, "bluetooth::agent::watch");
    let _span = span.enter();

    let (agent, mut receiver) = cosmic_bluez_zbus::agent1::create();

    let agent_path = ObjectPath::from_static_str_unchecked(AGENT_PATH);

    tracing::debug!("connecting agent");

    connection.object_server().at(&agent_path, agent).await?;

    tracing::debug!("connecting to bluez agent manager");

    let bluez = cosmic_bluez_zbus::agent_manager1::AgentManager1Proxy::new(&connection).await?;
    let mut owners = zbus::fdo::DBusProxy::new(&connection)
        .await?
        .receive_name_owner_changed_with_args(&[(0, "org.bluez")])
        .await?;

    tracing::debug!("registering agent");

    register(&bluez, &agent_path).await?;

    tracing::debug!("registered");

    loop {
        match futures::future::select(receiver.next(), owners.next()).await {
            Either::Left((Some(msg), _)) => {
                tracing::debug!(?msg, "agent message received");

                if tx.send(Event::Agent(Arc::new(msg))).await.is_err() {
                    break;
                }
            }

            Either::Right((Some(signal), _)) => {
                if signal.args().is_ok_and(|args| args.new_owner().is_some()) {
                    tracing::debug!("registering agent again");
                    if let Err(why) = register(&bluez, &agent_path).await {
                        tracing::error!("Failed to register the agent again: {why}");
                    }
                }
            }

            Either::Left((None, _)) | Either::Right((None, _)) => break,
        }
    }

    _ = bluez.unregister_agent(&agent_path).await;

    tracing::debug!("exiting");

    Ok(())
}

async fn register(bluez: &AgentManager1Proxy<'_>, agent_path: &ObjectPath<'_>) -> zbus::Result<()> {
    bluez
        .register_agent(
            agent_path,
            <&'static str>::from(cosmic_bluez_zbus::agent1::Capability::KeyboardDisplay),
        )
        .await?;

    if let Err(why) = bluez.request_default_agent(agent_path).await {
        _ = bluez.unregister_agent(agent_path).await;
        Err(why)?;
    }

    Ok(())
}
