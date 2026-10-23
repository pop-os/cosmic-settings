// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use crate::properties::Properties;
use crate::{Active, Event};
use bluez_zbus::device1::Device1Proxy;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use zbus::proxy::CacheProperties;
use zbus::zvariant::OwnedObjectPath;

const DEFAULT_DEVICE_ICON: &str = "bluetooth-symbolic";

/// How many times a request to a device is tried, unless said otherwise.
const ATTEMPTS: u64 = 4;

/// Errors after which trying to connect again would be pointless, or would ask the user again
/// about a pairing they just rejected.
const FINAL_ERRORS: &[&str] = &[
    "org.bluez.Error.AuthenticationCanceled",
    "org.bluez.Error.AuthenticationFailed",
    "org.bluez.Error.AuthenticationRejected",
    "org.bluez.Error.AuthenticationTimeout",
    "org.bluez.Error.DoesNotExist",
    "org.freedesktop.DBus.Error.UnknownObject",
];

// Copied from https://github.com/bluez/bluez/blob/39467578207889fd015775cbe81a3db9dd26abea/src/dbus-common.c#L53
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
        _ => DEFAULT_DEVICE_ICON,
    }
}

#[derive(Default, Debug, Clone)]
pub struct Device {
    pub alias: Option<String>,
    pub address: String,
    pub adapter: OwnedObjectPath,
    pub enabled: Active,
    pub paired: bool,
    pub icon: &'static str,
    pub battery: Option<String>,
}

impl Device {
    /// Reads a device from its `org.bluez.Device1` properties, and those of its
    /// `org.bluez.Battery1` interface if it has one.
    pub(crate) fn from_properties(
        device: &impl Properties,
        battery: Option<&impl Properties>,
    ) -> Option<Self> {
        let address = device
            .string("Address")
            .filter(|address| !address.is_empty())?;
        let adapter = device
            .object_path("Adapter")
            .filter(|adapter| !adapter.as_str().is_empty())?;
        // Without a name, BlueZ gives the address as alias.
        let alias = match device.string("Alias") {
            Some(alias) if !alias.is_empty() && alias != address.replace(':', "-") => Some(alias),
            _ => device.string("Name"),
        };

        Some(Self {
            alias,
            address,
            adapter,
            enabled: Active::from(device.boolean("Connected").unwrap_or(false)),
            paired: device.boolean("Paired").unwrap_or(false),
            icon: device_type_to_icon(device.string("Icon").as_deref().unwrap_or("unknown")),
            battery: battery
                .and_then(|battery| battery.byte("Percentage"))
                .map(|percentage| percentage.to_string()),
        })
    }

    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.enabled == Active::Enabled
    }
    /// Update the state of the device without overriding intermediary states.
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
    pub fn is_known_device_type(&self) -> bool {
        self.icon != DEFAULT_DEVICE_ICON
    }
    #[must_use]
    pub fn alias_or_addr(&self) -> &str {
        self.alias.as_ref().unwrap_or(&self.address)
    }
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
                        Some(DeviceUpdate::Enabled(Active::from(value)))
                    }
                    ("Paired", zbus::zvariant::Value::Bool(value)) => {
                        Some(DeviceUpdate::Paired(value))
                    }
                    ("Icon", zbus::zvariant::Value::Str(value)) => {
                        Some(DeviceUpdate::Icon(device_type_to_icon(&value)))
                    }
                    // Battery
                    ("Percentage", zbus::zvariant::Value::U8(percentage)) => {
                        Some(DeviceUpdate::Battery(Some(percentage.to_string())))
                    }
                    (message, value) => {
                        tracing::debug!(message, ?value, "device update");
                        None
                    }
                }
            })
            .collect()
    }
}

/// A proxy that reads the properties of the device when asked, without caching them.
async fn proxy(
    connection: &zbus::Connection,
    path: OwnedObjectPath,
) -> zbus::Result<Device1Proxy<'static>> {
    Device1Proxy::builder(connection)
        .path(path)?
        .cache_properties(CacheProperties::No)
        .build()
        .await
}

fn is_final(why: &zbus::Error) -> bool {
    matches!(why, zbus::Error::MethodError(name, ..) if FINAL_ERRORS.contains(&name.as_str()))
}

/// Lets the device connect by itself later, without asking.
async fn trust(device: &Device1Proxy<'_>) {
    if let Err(why) = device.set_trusted(true).await {
        tracing::error!("Failed to trust the device: {why}");
    }
}

/// Whether BlueZ has the device paired, asked now. False when that can't be known.
pub async fn is_paired(connection: zbus::Connection, device_path: OwnedObjectPath) -> bool {
    match proxy(&connection, device_path).await {
        Ok(device) => device.paired().await.unwrap_or(false),
        Err(_) => false,
    }
}

pub async fn disconnect_device(
    connection: zbus::Connection,
    device_path: OwnedObjectPath,
) -> Event {
    let device = match proxy(&connection, device_path.clone()).await {
        Err(why) => {
            tracing::error!("Unable to get the device: {why}");
            return Event::DeviceFailed(device_path);
        }
        Ok(device) => device,
    };

    for attempt in 1..=ATTEMPTS {
        let result = async {
            if !device.connected().await? {
                return Ok(());
            }

            device.disconnect().await
        }
        .await;

        match result {
            Ok(()) => return Event::Ok,
            Err(why) => {
                tracing::warn!("Unable to disconnect the device: {why}");
                if attempt < ATTEMPTS {
                    tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
                }
            }
        }
    }

    Event::DeviceFailed(device_path)
}

/// Connects the device, after pairing with it if needed, then trusts it.
///
/// Tries up to `attempts` times, but not again after a failed or rejected pairing.
pub async fn connect_device(
    connection: zbus::Connection,
    device_path: OwnedObjectPath,
    attempts: u64,
) -> Event {
    let device = match proxy(&connection, device_path.clone()).await {
        Err(why) => {
            tracing::error!("Unable to get the device: {why}");
            return Event::DeviceFailed(device_path);
        }
        Ok(device) => device,
    };

    for attempt in 1..=attempts.max(1) {
        let result = async {
            if device.connected().await? {
                return Ok(());
            }
            if !device.paired().await.unwrap_or(false) {
                device.pair().await?;
                trust(&device).await;
            }
            device.connect().await
        }
        .await;

        match result {
            Ok(()) => {
                trust(&device).await;
                return Event::Ok;
            }
            Err(why) => {
                tracing::warn!("Unable to connect to device: {why}");
                if is_final(&why) {
                    break;
                }
                if attempt < attempts {
                    tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
                }
            }
        }
    }

    Event::DeviceFailed(device_path)
}

pub async fn forget_device(connection: zbus::Connection, device_path: OwnedObjectPath) -> Event {
    let device = match proxy(&connection, device_path.clone()).await {
        Err(why) => {
            tracing::error!("Unable to get the device: {why}");
            return Event::DeviceFailed(device_path);
        }
        Ok(device) => device,
    };

    let adapter_path = match device.adapter().await {
        Err(why) => {
            tracing::error!("Unable to get the adapter: {why}");
            return Event::DeviceFailed(device_path);
        }
        Ok(adapter_path) => adapter_path,
    };

    let adapter = match bluez_zbus::get_adapter(&connection, adapter_path).await {
        Err(why) => {
            tracing::error!("Unable to get the adapter: {why}");
            return Event::DeviceFailed(device_path);
        }
        Ok(adapter) => adapter,
    };

    for attempt in 1..=ATTEMPTS {
        let result = async {
            if device.connected().await? {
                device.disconnect().await?;
            }

            adapter.remove_device(&device_path).await
        }
        .await;

        match result {
            Ok(()) => return Event::Ok,
            Err(why) => {
                tracing::warn!("Unable to forget the device: {why}");
                if attempt < ATTEMPTS {
                    tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
                }
            }
        }
    }

    Event::DeviceFailed(device_path)
}

pub async fn rename_device(
    connection: zbus::Connection,
    device_path: OwnedObjectPath,
    name: String,
) -> Event {
    let device = match proxy(&connection, device_path.clone()).await {
        Err(why) => {
            tracing::error!("Unable to get the device: {why}");
            return Event::DeviceFailed(device_path);
        }
        Ok(device) => device,
    };

    for attempt in 1..=ATTEMPTS {
        match device.set_alias(&name).await {
            Ok(()) => return Event::Ok,
            Err(why) => {
                tracing::warn!("Unable to rename device: {why}");
                if attempt < ATTEMPTS {
                    tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
                }
            }
        }
    }

    Event::DeviceRenameFailed(device_path)
}

pub async fn get_devices(connection: zbus::Connection, adapter_path: OwnedObjectPath) -> Event {
    match crate::subscription::managed_objects(&connection).await {
        Ok((_, objects)) => {
            Event::SetDevices(crate::subscription::devices(&objects, Some(&adapter_path)))
        }
        Err(why) => {
            tracing::error!("zbus connection failed. {why}");
            Event::DBusError(why)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zbus::zvariant::{ObjectPath, OwnedValue, Value};

    fn properties(properties: &[(&str, Value<'_>)]) -> HashMap<String, OwnedValue> {
        properties
            .iter()
            .map(|(name, value)| ((*name).to_owned(), value.try_to_owned().unwrap()))
            .collect()
    }

    fn adapter() -> Value<'static> {
        Value::from(ObjectPath::from_static_str_unchecked("/org/bluez/hci0"))
    }

    #[test]
    fn test_device_from_properties() {
        let device = Device::from_properties(
            &properties(&[
                ("Address", Value::from("AA:BB:CC:DD:EE:FF")),
                ("Adapter", adapter()),
                ("Alias", Value::from("Headphones")),
                ("Icon", Value::from("audio-headphones")),
                ("Paired", Value::from(true)),
                ("Connected", Value::from(true)),
            ]),
            Some(&properties(&[("Percentage", Value::from(42u8))])),
        )
        .unwrap();
        assert_eq!(device.address, "AA:BB:CC:DD:EE:FF");
        assert_eq!(device.adapter.as_str(), "/org/bluez/hci0");
        assert_eq!(device.alias.as_deref(), Some("Headphones"));
        assert_eq!(device.icon, "audio-headphones-symbolic");
        assert!(device.paired);
        assert_eq!(device.enabled, Active::Enabled);
        assert_eq!(device.battery.as_deref(), Some("42"));
    }

    #[test]
    fn test_device_without_name() {
        let device = Device::from_properties(
            &properties(&[
                ("Address", Value::from("AA:BB:CC:DD:EE:FF")),
                ("Adapter", adapter()),
                ("Alias", Value::from("AA-BB-CC-DD-EE-FF")),
            ]),
            None::<&HashMap<String, OwnedValue>>,
        )
        .unwrap();
        assert_eq!(device.alias, None);
        assert_eq!(device.alias_or_addr(), "AA:BB:CC:DD:EE:FF");
        assert_eq!(device.icon, DEFAULT_DEVICE_ICON);
        assert!(!device.paired);
        assert_eq!(device.enabled, Active::Disabled);
        assert_eq!(device.battery, None);
    }

    #[test]
    fn test_device_without_address_or_adapter() {
        let no_battery = None::<&HashMap<String, OwnedValue>>;
        assert!(
            Device::from_properties(&properties(&[("Adapter", adapter())]), no_battery).is_none()
        );
        assert!(
            Device::from_properties(
                &properties(&[("Address", Value::from("AA:BB:CC:DD:EE:FF"))]),
                no_battery
            )
            .is_none()
        );
    }

    #[test]
    fn test_final_errors() {
        let error = |name: &str| {
            let call = zbus::Message::method_call("/org/bluez/hci0/dev_AA", "Pair")
                .unwrap()
                .build(&())
                .unwrap();
            let reply = zbus::Message::error(&call.header(), name)
                .unwrap()
                .build(&("",))
                .unwrap();
            zbus::Error::from(reply)
        };
        assert!(is_final(&error("org.bluez.Error.AuthenticationRejected")));
        assert!(is_final(&error("org.bluez.Error.AuthenticationFailed")));
        assert!(is_final(&error("org.bluez.Error.AuthenticationCanceled")));
        assert!(!is_final(&error("org.bluez.Error.Failed")));
        assert!(!is_final(&error("org.bluez.Error.InProgress")));
        assert!(!is_final(&zbus::Error::Failure("timeout".to_owned())));
    }

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
}
