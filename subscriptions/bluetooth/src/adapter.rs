// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use crate::{Active, Event};
use bluez_zbus::adapter1::Adapter1Proxy;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::Duration;
use zbus::proxy::CacheProperties;
use zbus::zvariant::OwnedObjectPath;

const ATTEMPTS: u64 = 5;

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

async fn proxy(
    connection: &zbus::Connection,
    path: OwnedObjectPath,
) -> zbus::Result<Adapter1Proxy<'static>> {
    Adapter1Proxy::builder(connection)
        .path(path)?
        .cache_properties(CacheProperties::No)
        .build()
        .await
}

pub async fn start_discovery(connection: zbus::Connection, adapter_path: OwnedObjectPath) -> Event {
    let adapter = match proxy(&connection, adapter_path).await {
        Ok(adapter) => adapter,
        Err(why) => {
            tracing::error!("Unable to get the adapter: {why}");
            return Event::DBusError(why);
        }
    };

    let mut attempt = 1;
    loop {
        tracing::debug!("Starting discovery");
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

        match result {
            Ok(()) => {
                tracing::debug!("Discovery started");
                return Event::Ok;
            }
            Err(why) if attempt == ATTEMPTS => {
                tracing::warn!("Unable to start bluetooth scanning: {why}");
                return Event::DBusError(why);
            }
            Err(why) => {
                tracing::warn!("Unable to start bluetooth scanning: {why}");
                tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
                attempt += 1;
            }
        }
    }
}

pub async fn stop_discovery(connection: zbus::Connection, adapter_path: OwnedObjectPath) -> Event {
    let adapter = match proxy(&connection, adapter_path).await {
        Ok(adapter) => adapter,
        Err(why) => return Event::DBusError(why),
    };

    let mut attempt = 1;
    loop {
        tracing::debug!("Stopping discovery");
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

        match result {
            Ok(()) => {
                tracing::debug!("Discovery stopped");
                return Event::Ok;
            }
            Err(why) if attempt == ATTEMPTS || why.to_string().contains("No discovery started") => {
                tracing::warn!("Unable to stop bluetooth scanning: {why}");
                return Event::DBusError(why);
            }
            Err(why) => {
                tracing::warn!("Unable to stop bluetooth scanning: {why}");
                tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
                attempt += 1;
            }
        }
    }
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
    let adapter = match proxy(&connection, adapter_path.clone()).await {
        Ok(adapter) => adapter,
        Err(why) => {
            tracing::error!("Unable to get the adapter: {why}");
            return Event::DBusError(why);
        }
    };

    if rfkill(&adapter_path, active).await && !active {
        return Event::Ok;
    }

    let mut attempt = 1;
    loop {
        let result: zbus::Result<()> = async {
            adapter.set_powered(active).await?;
            if active {
                adapter.set_discoverable(true).await?;
            }
            Ok(())
        }
        .await;
        match result {
            Ok(()) => return Event::Ok,
            Err(why) if attempt == ATTEMPTS => {
                tracing::error!("Failed to change the adapter state: {why}");
                return Event::DBusError(why);
            }
            Err(why) => {
                tracing::warn!("Unable to change the adapter state: {why}");
                tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
                attempt += 1;
            }
        }
    }
}

async fn rfkill(adapter_path: &OwnedObjectPath, active: bool) -> bool {
    let Some(name) = Path::new(adapter_path.as_str()).file_name() else {
        return false;
    };

    let list = tokio::process::Command::new("rfkill")
        .env("PATH", rfkill_path_var())
        .args(["--noheadings", "--output", "ID,DEVICE"])
        .output()
        .await;

    let id = list.ok().and_then(|output| {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .find_map(|row| {
                let mut columns = row.split_whitespace();
                let id = columns.next()?;
                (columns.next()? == name).then(|| id.to_owned())
            })
    });

    let Some(id) = id else {
        tracing::warn!("Failed to find rfkill ID for bluetooth adapter {name:?}");
        return false;
    };

    let result = tokio::process::Command::new("rfkill")
        .env("PATH", rfkill_path_var())
        .arg(if active { "unblock" } else { "block" })
        .arg(id)
        .output()
        .await;

    match result {
        Ok(output) if output.status.success() => true,
        Ok(output) => {
            tracing::error!(
                "Failed to set bluetooth state using rfkill: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
            false
        }
        Err(why) => {
            tracing::error!("Failed to set bluetooth state using rfkill. {why:?}");
            false
        }
    }
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
