// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use cosmic_dbus_networkmanager::{
    device::wireless::WirelessDevice,
    interface::enums::{ApFlags, ApSecurityFlags, DeviceState},
};

use futures::StreamExt;
use itertools::Itertools;
use std::{collections::HashMap, sync::Arc};
use zbus::zvariant::ObjectPath;

use super::hw_address::HwAddress;

pub async fn handle_wireless_device(
    device: WirelessDevice<'_>,
    hw_address: Option<String>,
) -> zbus::Result<Vec<AccessPoint>> {
    device.request_scan(HashMap::new()).await?;

    let mut scan_changed = device.receive_last_scan_changed().await;

    if let Some(t) = scan_changed.next().await {
        if let Ok(-1) = t.get().await {
            tracing::error!("wireless device scan errored");
            return Ok(Default::default());
        }
    }

    let access_points = device.get_access_points().await?;

    let state: DeviceState = device
        .upcast()
        .await
        .and_then(|dev| dev.cached_state())
        .unwrap_or_default()
        .map(|s| s.into())
        .unwrap_or_else(|| DeviceState::Unknown);

    // Sort by strength and remove duplicates
    let mut aps = HashMap::<String, AccessPoint>::new();
    for ap in access_points {
        let (ssid_res, strength_res) = futures::join!(ap.ssid(), ap.strength());

        if let Some((ssid, strength)) = ssid_res.ok().zip(strength_res.ok()) {
            let ssid = String::from_utf8_lossy(&ssid.clone()).into_owned();
            if let Some(access_point) = aps.get(&ssid) {
                if access_point.strength > strength {
                    continue;
                }
            }

            let Ok(flags) = ap.rsn_flags().await else {
                continue;
            };
            let network_type = if flags.intersects(ApSecurityFlags::KEY_MGMT_802_1X) {
                NetworkType::EAP
            } else if flags.intersects(ApSecurityFlags::KEY_MGMTPSK) {
                NetworkType::PSK
            } else if flags.is_empty() {
                NetworkType::Open
            } else {
                continue;
            };

            aps.insert(
                ssid.clone(),
                AccessPoint {
                    ssid: Arc::from(ssid),
                    strength,
                    state,
                    working: false,
                    path: ap.inner().path().to_owned(),
                    secured: !ap.wpa_flags().await?.is_empty(),
                    wps_push: ap.flags().await?.contains(ApFlags::WPS_PBC),
                    network_type,
                    hw_address: hw_address
                        .as_ref()
                        .and_then(|str_addr| HwAddress::from_str(str_addr))
                        .unwrap_or_default(),
                },
            );
        }
    }

    let aps = aps
        .into_values()
        .sorted_by(|a, b| b.strength.cmp(&a.strength))
        .collect();

    Ok(aps)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessPoint {
    pub ssid: Arc<str>,
    pub strength: u8,
    pub state: DeviceState,
    pub working: bool,
    pub path: ObjectPath<'static>,
    pub hw_address: HwAddress,
    pub secured: bool,
    pub wps_push: bool,
    pub network_type: NetworkType,
}

// TODO do we want to support eap methods other than peap in the applet?
// Then we'd need a dropdown for the eap method,
// and tls requires a cert instead of a password
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkType {
    Open,
    PSK,
    EAP,
}
