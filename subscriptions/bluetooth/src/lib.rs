// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;
use std::sync::Arc;
use zbus::zvariant::OwnedObjectPath;

mod adapter;
pub mod agent;
mod device;
mod properties;
pub mod subscription;

pub use adapter::*;
pub use device::*;

pub(crate) const SERVICE: &str = "org.bluez";
pub(crate) const ADAPTER_INTERFACE: &str = "org.bluez.Adapter1";
pub(crate) const DEVICE_INTERFACE: &str = "org.bluez.Device1";
pub(crate) const BATTERY_INTERFACE: &str = "org.bluez.Battery1";

#[derive(Clone, Debug)]
pub enum Event {
    AddedAdapter(OwnedObjectPath, Adapter),
    AddedDevice(OwnedObjectPath, Device),
    Agent(Arc<agent::Message>),
    DBusError(zbus::Error),
    DBusServiceUnknown,
    DeviceFailed(OwnedObjectPath),
    DeviceRenameFailed(OwnedObjectPath),
    Ok,
    NameHasNoOwner,
    RemovedAdapter(OwnedObjectPath),
    RemovedDevice(OwnedObjectPath),
    SetAdapters(HashMap<OwnedObjectPath, Adapter>),
    SetDevices(HashMap<OwnedObjectPath, Device>),
    UpdatedAdapter(OwnedObjectPath, Vec<AdapterUpdate>),
    UpdatedDevice(OwnedObjectPath, Vec<DeviceUpdate>),
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Active {
    #[default]
    Disabled,
    Disabling,
    Enabling,
    Enabled,
}

impl From<bool> for Active {
    fn from(enabled: bool) -> Self {
        if enabled {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}
