// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use crate::{Active, Event};
use std::{
    collections::HashMap,
    convert::Infallible,
    hash::{Hash, Hasher},
    path::PathBuf,
    time::Duration,
};
use zbus::zvariant::OwnedObjectPath;

#[derive(Default, Debug, Clone)]
pub struct Adapter {
    pub alias: String,
    pub address: String,
    pub scanning: Active,
    pub enabled: Active,
}

impl Hash for Adapter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

impl PartialEq for Adapter {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Eq for Adapter {}

impl Adapter {
    pub async fn from_device(
        proxy: &bluez_zbus::adapter1::Adapter1Proxy<'_>,
    ) -> zbus::Result<Self> {
        let (address, alias, scanning, enabled) = futures::try_join!(
            proxy.address(),
            proxy.alias(),
            async {
                Ok(
                    if proxy.discoverable().await? && proxy.discovering().await? {
                        Active::Enabled
                    } else {
                        Active::Disabled
                    },
                )
            },
            async {
                Ok(if proxy.powered().await? {
                    Active::Enabled
                } else {
                    Active::Disabled
                })
            }
        )?;

        Ok(Self {
            alias,
            address,
            scanning,
            enabled,
        })
    }

    pub fn update(&mut self, updates: Vec<AdapterUpdate>) {
        for update in updates {
            match update {
                AdapterUpdate::Alias(alias) => self.alias = alias,
                AdapterUpdate::Address(address) => self.address = address,
                AdapterUpdate::Enabled(enabled) => {
                    self.enabled = match (self.enabled, enabled) {
                        (Active::Enabling, Active::Enabled) => Active::Enabled,
                        (Active::Disabling, Active::Disabled) => Active::Disabled,
                        (Active::Enabled | Active::Disabled, status) => status,
                        (status, _) => status,
                    }
                }
                AdapterUpdate::Scanning(scanning) => {
                    self.scanning = match (self.scanning, scanning) {
                        (Active::Enabling, Active::Enabled) => Active::Enabled,
                        (Active::Disabling, Active::Disabled) => Active::Disabled,
                        (Active::Enabled | Active::Disabled, status) => status,
                        (status, _) => status,
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum AdapterUpdate {
    Alias(String),
    Address(String),
    Scanning(Active),
    Enabled(Active),
}

impl AdapterUpdate {
    #[must_use]
    pub fn from_update(update: HashMap<&'_ str, zbus::zvariant::Value<'_>>) -> Vec<Self> {
        update
            .into_iter()
            .filter_map(|(key, value)| {
                match (key, value) {
                    ("Alias", zbus::zvariant::Value::Str(value)) => Some(Self::Alias(value.into())),
                    ("Discovering" | "Discoverable", zbus::zvariant::Value::Bool(value)) => {
                        Some(Self::Scanning(if value {
                            Active::Enabled
                        } else {
                            Active::Disabled
                        }))
                    }
                    ("Powered", zbus::zvariant::Value::Bool(value)) => {
                        Some(Self::Enabled(if value {
                            Active::Enabled
                        } else {
                            Active::Disabled
                        }))
                    }
                    ("Address", zbus::zvariant::Value::Str(value)) => {
                        Some(Self::Address(value.into()))
                    }
                    // Battery
                    (message, value) => {
                        tracing::error!(message, ?value, "adapter update");
                        None
                    }
                }
            })
            .collect()
    }
}

pub async fn start_discovery(connection: zbus::Connection, adapter_path: OwnedObjectPath) -> Event {
    let result: zbus::Result<()> = Ok(());

    let adapter = match bluez_zbus::get_adapter(&connection, adapter_path).await {
        Err(why) => {
            tracing::error!("Unable to get the adapter: {why}");
            return Event::DBusError(why);
        }
        Ok(adapter) => adapter,
    };

    for attempt in 1..5 {
        let result = async {
            tracing::debug!("Starting discovery");
            // We don't seem to be able to use join here as it seem to lead to some kind of race condition and not start scanning occasionally
            adapter.set_pairable(true).await?;
            adapter.set_discoverable(true).await?;
            if adapter.discovering().await? {
                return Ok(());
            }
            adapter.start_discovery().await
        }
        .await;

        if let Err(why) = result {
            tracing::warn!("Unable to start bluetooth scanning: {why}");
            tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
        } else {
            tracing::debug!("Discovery started");
            return Event::Ok;
        }
    }

    if let Err(why) = result {
        Event::DBusError(why)
    } else {
        Event::Ok
    }
}

pub async fn stop_discovery(connection: zbus::Connection, adapter_path: OwnedObjectPath) -> Event {
    let result: zbus::Result<()> = Ok(());

    let adapter = match bluez_zbus::get_adapter(&connection, adapter_path).await {
        Err(why) => return Event::DBusError(why),
        Ok(adapter) => adapter,
    };

    for attempt in 1..5 {
        let result = async {
            tracing::debug!("Stopping discovery");

            // We don't seem to be able to use join here as it seem to lead to some kind of race condition and not stop scanning occasionally
            adapter.set_pairable(false).await?;
            adapter.set_discoverable(false).await?;
            if adapter.discovering().await? {
                adapter.stop_discovery().await
            } else {
                Ok(())
            }
        }
        .await;

        if let Err(why) = result {
            tracing::warn!("Unable to stop bluetooth scanning: {why}");
            if why.to_string().contains("No discovery started") {
                return Event::DBusError(why);
            }

            tracing::warn!("Unable to stop bluetooth scanning: {why}");
            tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
        } else {
            tracing::debug!("Discovery stopped");
            return Event::Ok;
        }
    }

    if let Err(why) = result {
        return Event::DBusError(why);
    }
    Event::Ok
}

// In some distros, rfkill is only in sbin, which isn't normally in PATH
// TODO: Directly access `/dev/rfkill`
pub fn rfkill_path_var() -> std::ffi::OsString {
    let mut path = std::env::var_os("PATH").unwrap_or_default();
    path.push(":/usr/sbin");
    path
}

pub async fn change_adapter_status(
    connection: zbus::Connection,
    adapter_path: OwnedObjectPath,
    active: bool,
) -> Event {
    // rfkill will be persisted after reboot
    let adapter = bluez_zbus::get_adapter(&connection, adapter_path.clone())
        .await
        .map_err(|err| Ok::<_, Infallible>(Event::DBusError(err)))
        .unwrap();

    let path = PathBuf::from(adapter_path.to_string());
    let name = path.file_name().unwrap();

    let mut cmd = tokio::process::Command::new("rfkill");
    cmd.env("PATH", rfkill_path_var())
        .arg("--noheadings")
        .arg("--output")
        .arg("ID,DEVICE");

    let rfkill_list = cmd.output().await.ok();

    if let Some(id) = rfkill_list.and_then(|o| {
        let lines = String::from_utf8(o.stdout).ok()?;
        lines.split('\n').into_iter().find_map(|row| {
            let (id, cname) = row.trim().split_once(' ')?;
            (name == cname).then_some(id.to_string())
        })
    }) {
        if let Err(err) = tokio::process::Command::new("rfkill")
            .env("PATH", rfkill_path_var())
            .arg(if active { "unblock" } else { "block" })
            .arg(id)
            .output()
            .await
        {
            tracing::error!("Failed to set bluetooth state using rfkill. {err:?}");
        }
        if !active {
            return Event::Ok;
        }
    } else {
        tracing::error!("Failed to find rfkill ID for bluetooth adapter {name:?}");
    }

    let mut result: zbus::Result<()> = Ok(());
    for attempt in 1..5 {
        result = async {
            adapter.set_powered(true).await?;
            adapter.set_discoverable(true).await
        }
        .await;
        if let Err(why) = &result {
            tracing::warn!("Unable to change the adapter state: {why}");
            tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
        } else {
            return Event::Ok;
        }
    }

    if let Err(why) = result {
        tracing::error!("Failed to change the adapter state!");
        return Event::DBusError(why);
    }

    Event::Ok
}

pub async fn get_adapters(connection: zbus::Connection) -> Event {
    let result: zbus::Result<HashMap<OwnedObjectPath, Adapter>> = async {
        futures::future::join_all(
            bluez_zbus::get_adapters(&connection)
                .await?
                .into_iter()
                .map(|(path, proxy)| async move {
                    Ok((path.to_owned(), Adapter::from_device(&proxy).await?))
                }),
        )
        .await
        .into_iter()
        .collect::<zbus::Result<HashMap<_, _>>>()
    }
    .await;
    match result {
        Ok(adapters) => Event::SetAdapters(adapters),
        Err(why) => {
            tracing::error!("dbus connection failed. {why}");
            Event::DBusError(why)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_device_with_intermediary_state() {
        let mut adapter = Adapter {
            alias: "foo".to_owned(),
            address: "AA:BB:CC:DD:EE:FF".to_owned(),
            scanning: Active::Disabled,
            enabled: Active::Disabled,
        };
        adapter.update(vec![
            AdapterUpdate::Enabled(Active::Enabled),
            AdapterUpdate::Alias("xxx".to_owned()),
        ]);
        assert_eq!(adapter.enabled, Active::Enabled);
        assert_eq!(&adapter.alias, "xxx");

        adapter.enabled = Active::Disabling;
        adapter.update(vec![
            AdapterUpdate::Enabled(Active::Enabled),
            AdapterUpdate::Alias("xxx".to_owned()),
        ]);
        assert_eq!(adapter.enabled, Active::Disabling);

        adapter.scanning = Active::Enabling;
        adapter.update(vec![
            AdapterUpdate::Scanning(Active::Disabled),
            AdapterUpdate::Alias("xxx".to_owned()),
        ]);
        assert_eq!(adapter.scanning, Active::Enabling);

        adapter.update(vec![
            AdapterUpdate::Scanning(Active::Enabled),
            AdapterUpdate::Alias("xxx".to_owned()),
        ]);
        assert_eq!(adapter.scanning, Active::Enabled);
        assert_eq!(&adapter.alias, "xxx");
    }
}
