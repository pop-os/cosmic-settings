// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use futures::join;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    time::Duration,
};
use zbus::zvariant::OwnedObjectPath;

use super::Message;

#[derive(Default, Debug, Clone)]
pub struct Device {
    alias: Option<String>,
    pub address: String,
    pub adapter: OwnedObjectPath,
    pub enabled: Active,
    pub paired: bool,
    pub icon: &'static str,
    pub battery: Option<String>,
}
#[derive(Debug, Clone)]
pub enum DeviceUpdate {
    Alias(Option<String>),
    Enabled(Active),
    Paired(bool),
    Icon(&'static str),
    Battery(Option<String>),
}

impl DeviceUpdate {
    pub fn from_update(update: HashMap<&'_ str, zbus::zvariant::Value<'_>>) -> Vec<Self> {
        update
            .into_iter()
            .filter_map(|(key, value)| {
                match (key, value) {
                    ("Alias", zbus::zvariant::Value::Str(value)) => {
                        Some(DeviceUpdate::Alias(Some(value.into())))
                    }
                    ("Connected", zbus::zvariant::Value::Bool(value)) => {
                        Some(DeviceUpdate::Enabled(if value {
                            Active::Enabled
                        } else {
                            Active::Disabled
                        }))
                    }
                    ("Paired", zbus::zvariant::Value::Bool(value)) => {
                        Some(DeviceUpdate::Paired(value))
                    }
                    ("Icon", zbus::zvariant::Value::Str(value)) => {
                        Some(DeviceUpdate::Icon(device_type_to_icon(&value)))
                    }
                    ("Percentage", zbus::zvariant::Value::U8(percentage)) => {
                        Some(DeviceUpdate::Battery(Some(fl!(
                            "bluetooth-paired",
                            "battery",
                            percentage = percentage.to_string()
                        ))))
                    }
                    // Battery
                    (message, value) => {
                        tracing::debug!(message, ?value, "device update");
                        None
                    }
                }
            })
            .collect()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Adapter {
    pub alias: String,
    pub address: String,
    pub scanning: Active,
    pub enabled: Active,
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

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Active {
    #[default]
    Disabled,
    Disabling,
    Enabling,
    Enabled,
}

impl Hash for Device {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Eq for Device {}

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

fn device_type_to_icon(device_type: &str) -> &'static str {
    match device_type {
        "computer" => "laptop-symbolic",
        "phone" => "smartphone-symbolic",
        "network-wireless" => "network-wireless-symbolic",
        "audio-headset" => "audio-headset-symbolic",
        "audio-headphones" => "audio-headphones-symbolic",
        "camera-video" => "camera-video-symbolic",
        "audio-card" => "audio-card-symbolic",
        "input-gaming" => "input-gaming-symbolic",
        "input-keyboard" => "input-keyboard-symbolic",
        "input-tablet" => "input-tablet-symbolic",
        "input-mouse" => "input-mouse-symbolic",
        "printer" => "printer-network-symbolic",
        "camera-photo" => "camera-photo-symbolic",
        _ => "bluetooth-symbolic",
    }
}

impl Device {
    pub async fn from_device(proxy: &bluez_zbus::BluetoothDevice<'_>) -> zbus::Result<Self> {
        let (address, adapter, alias) = join!(
            proxy.device.address(),
            proxy.device.adapter(),
            proxy.device.name()
        );
        let address = address?;
        if address.is_empty() {
            return Err(zbus::Error::Failure("Device has no MAC address".to_owned()));
        }
        let adapter = adapter?;
        if adapter.is_empty() {
            return Err(zbus::Error::Failure("Device has no adapter".to_owned()));
        }
        let alias = alias.ok();
        let device_type: String = proxy.icon().await;
        let paired = proxy.device.paired().await.unwrap_or(false);
        let enabled = if proxy.device.connected().await.unwrap_or(false) && paired {
            Active::Enabled
        } else {
            Active::Disabled
        };
        let battery = match &proxy.battery {
            Some(battery) => match battery.percentage().await {
                Ok(percentage) => Some(fl!(
                    "bluetooth-paired",
                    "battery",
                    percentage = percentage.to_string()
                )),
                Err(why) => {
                    eprintln!("couldn't fetch battery percentage: {why}");
                    None
                }
            },
            None => None,
        };

        // Copied from https://github.com/bluez/bluez/blob/39467578207889fd015775cbe81a3db9dd26abea/src/dbus-common.c#L53
        let icon = device_type_to_icon(device_type.as_str());

        Ok(Self {
            alias,
            address,
            adapter,
            enabled,
            paired,
            icon,
            battery,
        })
    }
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.enabled == Active::Enabled
    }
    /// Update the state of the device without overriding intermediary states.
    ///
    /// # Panics
    ///
    /// Panics if the device used for update doesn't have the same MAC address
    pub fn update(&mut self, updates: Vec<DeviceUpdate>) {
        for udpate in updates {
            match udpate {
                DeviceUpdate::Alias(alias) => self.alias = alias,
                DeviceUpdate::Enabled(enabled) => {
                    self.enabled = match (self.enabled, enabled) {
                        (Active::Enabling, Active::Enabled) => Active::Enabled,
                        (Active::Disabling, Active::Disabled) => Active::Disabled,
                        (Active::Enabled | Active::Disabled, status) => status,
                        (status, _) => status,
                    }
                }
                DeviceUpdate::Paired(paired) => {
                    self.enabled = Active::Disabling;
                    self.paired = paired;
                }
                DeviceUpdate::Icon(icon) => self.icon = icon,
                DeviceUpdate::Battery(battery) => self.battery = battery,
            }
        }
        if self.enabled == Active::Disabled {
            self.battery = None;
        }
    }
    #[must_use]
    pub fn has_alias(&self) -> bool {
        self.alias.is_some()
    }
    #[must_use]
    pub fn alias_or_addr(&self) -> &str {
        self.alias.as_ref().unwrap_or(&self.address)
    }
}

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

pub async fn start_discovery(
    connection: zbus::Connection,
    adapter_path: OwnedObjectPath,
) -> Message {
    let result: zbus::Result<()> = Ok(());

    let adapter = match bluez_zbus::get_adapter(&connection, adapter_path).await {
        Err(why) => {
            tracing::error!("Unable to get the adapter: {why}");
            return Message::DBusError(why.to_string());
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
            return Message::Nop;
        }
    }

    return if let Err(why) = result {
        Message::DBusError(why.to_string())
    } else {
        Message::Nop
    };
}

pub async fn stop_discovery(
    connection: zbus::Connection,
    adapter_path: OwnedObjectPath,
) -> Message {
    let result: zbus::Result<()> = Ok(());

    let adapter = match bluez_zbus::get_adapter(&connection, adapter_path).await {
        Err(why) => return Message::DBusError(format!("Unable to get the adapter: {why}")),
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
            tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
        } else {
            tracing::debug!("Discovery stopped");
            return Message::Nop;
        }
    }

    if let Err(why) = result {
        return Message::DBusError(why.to_string());
    }
    Message::Nop
}

pub async fn disconnect_device(
    connection: zbus::Connection,
    device_path: OwnedObjectPath,
) -> Message {
    let proxy = match bluez_zbus::get_device(&connection, device_path.clone()).await {
        Err(why) => {
            tracing::error!("Unable to get the device: {why}");
            return Message::DeviceFailed(device_path);
        }
        Ok(proxy) => proxy,
    };

    for attempt in 1..5 {
        let result = async {
            if !proxy.device.connected().await? {
                return Ok(());
            }

            proxy.device.disconnect().await
        }
        .await;

        if let Err(why) = result {
            tracing::warn!("Unable to disconnect to device: {why}");
            tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
        } else {
            return Message::Nop;
        }
    }

    Message::DeviceFailed(device_path)
}

pub async fn connect_device(connection: zbus::Connection, device_path: OwnedObjectPath) -> Message {
    let proxy = match bluez_zbus::get_device(&connection, device_path.clone()).await {
        Err(why) => {
            tracing::error!("Unable to get the device: {why}");
            return Message::DeviceFailed(device_path);
        }
        Ok(proxy) => proxy,
    };

    for attempt in 1..5 {
        let result = async {
            if proxy.device.connected().await? {
                Ok(())
            } else {
                proxy.device.connect().await
            }
        }
        .await;

        if let Err(why) = result {
            tracing::warn!("Unable to connect to device: {why}");
            tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
        } else {
            return Message::Nop;
        }
    }

    Message::DeviceFailed(device_path)
}

pub async fn forget_device(connection: zbus::Connection, device_path: OwnedObjectPath) -> Message {
    let mut result: zbus::Result<()> = Ok(());

    let proxy = match bluez_zbus::get_device(&connection, device_path.clone()).await {
        Err(why) => {
            tracing::error!("Unable to get the device: {why}");
            return Message::DeviceFailed(device_path);
        }
        Ok(proxy) => proxy,
    };

    let adapter_path = match proxy.device.adapter().await {
        Err(why) => {
            tracing::error!("Unable to get the adapter: {why}");
            return Message::DeviceFailed(device_path);
        }
        Ok(adapter_path) => adapter_path,
    };

    let adapter = match bluez_zbus::get_adapter(&connection, adapter_path).await {
        Err(why) => {
            tracing::error!("Unable to get the adapter: {why}");
            return Message::DeviceFailed(device_path);
        }
        Ok(adapter) => adapter,
    };

    for attempt in 1..5 {
        result = async {
            if proxy.device.connected().await? {
                proxy.device.disconnect().await?;
            }

            adapter.remove_device(&proxy.path()).await
        }
        .await;

        if let Err(why) = &result {
            tracing::warn!("Unable to connect to device: {why}");
            tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
        } else {
            return Message::Nop;
        }
    }

    return if result.is_err() {
        Message::DeviceFailed(device_path)
    } else {
        Message::Nop
    };
}

pub async fn change_adapter_status(
    connection: zbus::Connection,
    adapter_path: OwnedObjectPath,
    active: bool,
) -> Message {
    let mut result: zbus::Result<()> = Ok(());
    for attempt in 1..5 {
        result = async {
            let adapter = bluez_zbus::get_adapter(&connection, adapter_path.clone()).await?;
            if active {
                adapter.set_powered(true).await?;
                adapter.set_discoverable(true).await
            } else {
                if let Err(why) = adapter.set_discoverable(false).await {
                    tracing::warn!("Unable to change discoverability: {why}");
                }
                adapter.set_powered(false).await
            }
        }
        .await;
        if let Err(why) = &result {
            tracing::warn!("Unable to change the adapter state: {why}");
            tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
        } else {
            return Message::Nop;
        }
    }

    if let Err(why) = result {
        tracing::error!("Failed to change the adapter state!");
        return Message::DBusError(why.to_string()).into();
    }

    Message::Nop
}

pub async fn get_devices(connection: zbus::Connection, adapter_path: OwnedObjectPath) -> Message {
    // TODO error handling
    let result: zbus::Result<HashMap<OwnedObjectPath, Device>> = async {
        futures::future::join_all(
            bluez_zbus::get_devices(&connection, Some(&adapter_path))
                .await?
                .into_iter()
                .map(
                    |(path, device)| async move { Ok((path, Device::from_device(&device).await?)) },
                ),
        )
        .await
        .into_iter()
        .collect::<Result<HashMap<_, _>, _>>()
    }
    .await;
    match result {
        Ok(devices) => Message::SetDevices(devices),
        Err(why) => {
            tracing::error!("zbus connection failed. {why}");
            Message::DBusError(fl!("bluetooth", "dbus-error", why = why.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_device_with_intermediary_state() {
        let mut device = Device {
            alias: None,
            adapter: OwnedObjectPath::try_from("/dev/bluez/hci0").unwrap(),
            address: "AA:BB:CC:DD:EE:FF".to_owned(),
            enabled: Active::Disabled,
            paired: false,
            icon: "bluetooth-symbolic",
            battery: None,
        };
        device.update(vec![
            DeviceUpdate::Enabled(Active::Enabled),
            DeviceUpdate::Alias(Some("Foo".to_owned())),
        ]);
        assert_eq!(device.enabled, Active::Enabled);
        assert_eq!(device.alias, Some("Foo".to_owned()));

        device.enabled = Active::Disabling;
        device.update(vec![
            DeviceUpdate::Enabled(Active::Enabled),
            DeviceUpdate::Alias(Some("Foo".to_owned())),
        ]);
        assert_eq!(device.enabled, Active::Disabling);

        device.enabled = Active::Enabling;
        device.update(vec![
            DeviceUpdate::Enabled(Active::Enabled),
            DeviceUpdate::Alias(Some("Foo".to_owned())),
        ]);
        assert_eq!(device.enabled, Active::Enabled);
    }

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
