// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use pipewire::device::DeviceInfoRef;

/// Device information
#[must_use]
#[derive(Clone, Debug)]
pub struct Device {
    pub id: u32,
    pub name: String,
}

impl Device {
    /// Attains process info from a pipewire info node.
    #[must_use]
    pub fn from_device(info: &DeviceInfoRef) -> Option<Self> {
        let props = info.props()?;

        let device = Device {
            id: props.get("object.id")?.parse::<u32>().ok()?,
            name: props.get("device.description")?.to_owned(),
        };

        Some(device)
    }
}
