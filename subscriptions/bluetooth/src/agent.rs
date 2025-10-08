// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::Event;

use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use zbus::zvariant::ObjectPath;

const AGENT_PATH: &str = "/org/bluez/agent/cosmic_settings";

pub async fn unregister(connection: zbus::Connection) -> zbus::Result<()> {
    let agent_path = ObjectPath::from_static_str_unchecked(AGENT_PATH);
    let bluez = bluez_zbus::agent_manager1::AgentManager1Proxy::new(&connection).await?;
    bluez.unregister_agent(&agent_path).await
}

pub async fn watch(
    connection: zbus::Connection,
    mut tx: futures::channel::mpsc::Sender<Event>,
) -> zbus::Result<()> {
    let span = tracing::span!(tracing::Level::INFO, "bluetooth::agent::watch");
    let _span = span.enter();

    let (agent, mut receiver) = bluez_zbus::agent1::create();

    let agent_path = ObjectPath::from_static_str_unchecked(AGENT_PATH);

    tracing::debug!("connecting agent");

    connection.object_server().at(&agent_path, agent).await?;

    tracing::debug!("connecting to bluez agent manager");

    let bluez = bluez_zbus::agent_manager1::AgentManager1Proxy::new(&connection).await?;

    tracing::debug!("registering agent");

    bluez
        .register_agent(
            &agent_path,
            <&'static str>::from(bluez_zbus::agent1::Capability::DisplayYesNo),
        )
        .await?;

    if let Err(why) = bluez.request_default_agent(&agent_path).await {
        _ = bluez.unregister_agent(&agent_path).await;
        Err(why)?;
    }

    tracing::debug!("registered");

    while let Some(msg) = receiver.next().await {
        tracing::debug!(?msg, "agent message received");

        if tx.send(Event::Agent(Arc::new(msg))).await.is_err() {
            break;
        }
    }

    _ = bluez.unregister_agent(&agent_path).await;

    tracing::debug!("exiting");

    Ok(())
}
