// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Read-only radio, cell, and signal information from ModemManager.

use std::collections::HashMap;

use zbus::{Connection, fdo::ObjectManagerProxy, proxy, zvariant::OwnedValue};

const SIGNAL_INTERFACE: &str = "org.freedesktop.ModemManager1.Modem.Signal";
const THREE_GPP_INTERFACE: &str = "org.freedesktop.ModemManager1.Modem.Modem3gpp";
const LOCATION_INTERFACE: &str = "org.freedesktop.ModemManager1.Modem.Location";

/// ModemManager samples detailed radio values at this rate while this page is open.
pub(super) const SIGNAL_REFRESH_SECONDS: u32 = 5;

#[derive(Clone, Debug, Default)]
pub struct ModemDetails {
    pub(super) interface: String,
    pub(super) operator: Option<String>,
    pub(super) operator_code: Option<String>,
    pub(super) cell: Option<Cell>,
    pub(super) lte: SignalMetrics,
    pub(super) nr5g: SignalMetrics,
    pub(super) started_signal_polling: Option<u32>,
}

impl ModemDetails {
    pub(super) fn cell_description(&self) -> String {
        let Some(cell) = self.cell.as_ref() else {
            return fl!("mobile", "network-cell-unavailable");
        };

        let mut values = Vec::with_capacity(4);
        if let (Some(mcc), Some(mnc)) = (&cell.mcc, &cell.mnc) {
            values.push(format!("MCC {mcc} · MNC {mnc}"));
        }
        if let Some(tac) = cell.tac.as_deref() {
            values.push(format!("TAC {tac}"));
        }
        if let Some(lac) = cell.lac.as_deref().filter(|lac| *lac != "0") {
            values.push(format!("LAC {lac}"));
        }
        if let Some(cell_id) = cell.cell_id.as_deref() {
            values.push(format!("Cell ID {cell_id}"));
        }

        (!values.is_empty())
            .then(|| values.join(" · "))
            .unwrap_or_else(|| fl!("mobile", "network-cell-unavailable"))
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct Cell {
    mcc: Option<String>,
    mnc: Option<String>,
    lac: Option<String>,
    tac: Option<String>,
    cell_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct SignalMetrics {
    rssi: Option<f64>,
    rsrp: Option<f64>,
    rsrq: Option<f64>,
    sinr: Option<f64>,
}

impl SignalMetrics {
    pub(super) fn is_empty(&self) -> bool {
        self.rssi.is_none() && self.rsrp.is_none() && self.rsrq.is_none() && self.sinr.is_none()
    }

    pub(super) fn description(&self) -> String {
        [
            metric("RSSI", self.rssi, "dBm", rssi_quality),
            metric("RSRP", self.rsrp, "dBm", rsrp_quality),
            metric("RSRQ", self.rsrq, "dB", rsrq_quality),
            metric("SINR/SNR", self.sinr, "dB", sinr_quality),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" · ")
    }
}

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem",
    default_service = "org.freedesktop.ModemManager1"
)]
trait Modem {
    #[zbus(property, name = "PrimaryPort")]
    fn primary_port(&self) -> zbus::Result<String>;
}

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Signal",
    default_service = "org.freedesktop.ModemManager1"
)]
trait Signal {
    #[zbus(name = "Setup")]
    async fn setup(&self, rate: u32) -> zbus::Result<()>;

    #[zbus(property, name = "Lte")]
    fn lte(&self) -> zbus::Result<HashMap<String, OwnedValue>>;

    #[zbus(property, name = "Nr5g")]
    fn nr5g(&self) -> zbus::Result<HashMap<String, OwnedValue>>;

    #[zbus(property, name = "Rate")]
    fn rate(&self) -> zbus::Result<u32>;
}

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Modem3gpp",
    default_service = "org.freedesktop.ModemManager1"
)]
trait Modem3gpp {
    #[zbus(property, name = "OperatorCode")]
    fn operator_code(&self) -> zbus::Result<String>;

    #[zbus(property, name = "OperatorName")]
    fn operator_name(&self) -> zbus::Result<String>;
}

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Location",
    default_service = "org.freedesktop.ModemManager1"
)]
trait Location {
    #[zbus(name = "GetLocation")]
    async fn location(&self) -> zbus::Result<HashMap<u32, OwnedValue>>;
}

pub(super) async fn read_modem_details() -> Result<Vec<ModemDetails>, String> {
    let connection = Connection::system()
        .await
        .map_err(|why| format!("connect to ModemManager: {why}"))?;
    let manager = ObjectManagerProxy::builder(&connection)
        .destination(super::MODEM_MANAGER_SERVICE)
        .map_err(|why| format!("build ModemManager manager: {why}"))?
        .path(super::MODEM_MANAGER_PATH)
        .map_err(|why| format!("build ModemManager manager: {why}"))?
        .build()
        .await
        .map_err(|why| format!("connect to ModemManager: {why}"))?;
    let paths = manager
        .get_managed_objects()
        .await
        .map_err(|why| format!("enumerate ModemManager devices: {why}"))?
        .into_iter()
        .filter(|(_, interfaces)| interfaces.contains_key(super::MODEM_INTERFACE))
        .map(|(path, interfaces)| {
            (
                path.to_string(),
                interfaces.contains_key(SIGNAL_INTERFACE),
                interfaces.contains_key(THREE_GPP_INTERFACE),
                interfaces.contains_key(LOCATION_INTERFACE),
            )
        })
        .collect::<Vec<_>>();

    let mut details = Vec::new();
    for (path, has_signal, has_3gpp, has_location) in paths {
        match read_modem_details_at(&connection, &path, has_signal, has_3gpp, has_location).await {
            Ok(detail) => details.push(detail),
            Err(why) => tracing::debug!(why, path, "failed to read modem network details"),
        }
    }
    Ok(details)
}

pub(super) async fn restore_signal_polling(modems: &[(String, u32)]) {
    let Ok(connection) = Connection::system().await else {
        return;
    };
    let Ok(manager_builder) = ObjectManagerProxy::builder(&connection)
        .destination(super::MODEM_MANAGER_SERVICE)
        .and_then(|builder| builder.path(super::MODEM_MANAGER_PATH))
    else {
        return;
    };
    let Ok(manager) = manager_builder.build().await else {
        return;
    };
    let Ok(objects) = manager.get_managed_objects().await else {
        return;
    };

    for (path, object_interfaces) in objects {
        if !object_interfaces.contains_key(SIGNAL_INTERFACE) {
            continue;
        }
        let Ok(modem_builder) = ModemProxy::builder(&connection).path(&path) else {
            continue;
        };
        let Ok(modem) = modem_builder.build().await else {
            continue;
        };
        let Ok(interface) = modem.primary_port().await else {
            continue;
        };
        let Some((_, previous_rate)) = modems.iter().find(|(active, _)| active == &interface)
        else {
            continue;
        };
        let Ok(signal_builder) = SignalProxy::builder(&connection).path(&path) else {
            continue;
        };
        if let Ok(signal) = signal_builder.build().await {
            let _ = signal.setup(*previous_rate).await;
        }
    }
}

async fn read_modem_details_at(
    connection: &Connection,
    path: &str,
    has_signal: bool,
    has_3gpp: bool,
    has_location: bool,
) -> Result<ModemDetails, String> {
    let modem = ModemProxy::builder(connection)
        .path(path)
        .map_err(|why| format!("build modem proxy: {why}"))?
        .build()
        .await
        .map_err(|why| format!("read modem properties: {why}"))?;
    let interface = modem
        .primary_port()
        .await
        .map_err(|why| format!("read modem interface: {why}"))?;
    let mut details = ModemDetails {
        interface,
        ..Default::default()
    };

    if has_3gpp {
        let proxy = Modem3gppProxy::builder(connection)
            .path(path)
            .map_err(|why| format!("build 3GPP proxy: {why}"))?
            .build()
            .await
            .map_err(|why| format!("read 3GPP properties: {why}"))?;
        let (operator, operator_code) =
            futures::join!(proxy.operator_name(), proxy.operator_code());
        details.operator = operator.ok().filter(|operator| !operator.is_empty());
        details.operator_code = operator_code.ok().filter(|operator| !operator.is_empty());
    }

    if has_location {
        let proxy = LocationProxy::builder(connection)
            .path(path)
            .map_err(|why| format!("build location proxy: {why}"))?
            .build()
            .await
            .map_err(|why| format!("read location properties: {why}"))?;
        details.cell = proxy
            .location()
            .await
            .ok()
            .and_then(|locations| locations.get(&1).and_then(string_value))
            .and_then(|location| parse_3gpp_location(&location));
    }

    if has_signal {
        let proxy = SignalProxy::builder(connection)
            .path(path)
            .map_err(|why| format!("build signal proxy: {why}"))?
            .build()
            .await
            .map_err(|why| format!("read signal properties: {why}"))?;
        let current_rate = proxy.rate().await.unwrap_or_default();
        if current_rate < SIGNAL_REFRESH_SECONDS
            && proxy.setup(SIGNAL_REFRESH_SECONDS).await.is_ok()
        {
            details.started_signal_polling = Some(current_rate);
        }
        let (lte, nr5g) = futures::join!(proxy.lte(), proxy.nr5g());
        details.lte = SignalMetrics::from_properties(lte.unwrap_or_default());
        details.nr5g = SignalMetrics::from_properties(nr5g.unwrap_or_default());
    }

    Ok(details)
}

impl SignalMetrics {
    fn from_properties(properties: HashMap<String, OwnedValue>) -> Self {
        Self {
            rssi: properties.get("rssi").and_then(f64_value),
            rsrp: properties.get("rsrp").and_then(f64_value),
            rsrq: properties.get("rsrq").and_then(f64_value),
            sinr: properties.get("snr").and_then(f64_value),
        }
    }
}

fn string_value(value: &OwnedValue) -> Option<String> {
    String::try_from(value.try_clone().ok()?).ok()
}

fn f64_value(value: &OwnedValue) -> Option<f64> {
    f64::try_from(value.try_clone().ok()?).ok()
}

fn parse_3gpp_location(location: &str) -> Option<Cell> {
    let mut values = location.split(',');
    let mcc = values.next()?.to_owned();
    let mnc = values.next()?.to_owned();
    let lac = values.next()?.to_owned();
    let cell_id = values.next()?.to_owned();
    let tac = values.next()?.to_owned();
    (values.next().is_none()).then_some(Cell {
        mcc: (!mcc.is_empty()).then_some(mcc),
        mnc: (!mnc.is_empty()).then_some(mnc),
        lac: (!lac.is_empty()).then_some(lac),
        tac: (!tac.is_empty()).then_some(tac),
        cell_id: (!cell_id.is_empty()).then_some(cell_id),
    })
}

fn metric(
    name: &str,
    value: Option<f64>,
    unit: &str,
    quality: fn(f64) -> String,
) -> Option<String> {
    value.map(|value| format!("{name} {value:.1} {unit} ({})", quality(value)))
}

fn rssi_quality(value: f64) -> String {
    if value >= -65.0 {
        fl!("mobile", "signal-excellent")
    } else if value >= -75.0 {
        fl!("mobile", "signal-good")
    } else if value >= -85.0 {
        fl!("mobile", "signal-fair")
    } else if value >= -95.0 {
        fl!("mobile", "signal-poor")
    } else {
        fl!("mobile", "signal-very-poor")
    }
}

fn rsrp_quality(value: f64) -> String {
    if value >= -80.0 {
        fl!("mobile", "signal-excellent")
    } else if value >= -90.0 {
        fl!("mobile", "signal-good")
    } else if value >= -100.0 {
        fl!("mobile", "signal-fair")
    } else if value >= -110.0 {
        fl!("mobile", "signal-poor")
    } else {
        fl!("mobile", "signal-very-poor")
    }
}

fn rsrq_quality(value: f64) -> String {
    if value >= -10.0 {
        fl!("mobile", "signal-excellent")
    } else if value >= -15.0 {
        fl!("mobile", "signal-good")
    } else if value >= -20.0 {
        fl!("mobile", "signal-fair")
    } else if value >= -25.0 {
        fl!("mobile", "signal-poor")
    } else {
        fl!("mobile", "signal-very-poor")
    }
}

fn sinr_quality(value: f64) -> String {
    if value >= 20.0 {
        fl!("mobile", "signal-excellent")
    } else if value >= 13.0 {
        fl!("mobile", "signal-good")
    } else if value >= 0.0 {
        fl!("mobile", "signal-fair")
    } else if value >= -5.0 {
        fl!("mobile", "signal-poor")
    } else {
        fl!("mobile", "signal-very-poor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_3gpp_location_keeps_the_serving_cell_identity() {
        let cell = parse_3gpp_location("222,88,0,22C3406,310F").expect("valid location");

        assert_eq!(cell.mcc.as_deref(), Some("222"));
        assert_eq!(cell.mnc.as_deref(), Some("88"));
        assert_eq!(cell.cell_id.as_deref(), Some("22C3406"));
        assert_eq!(cell.tac.as_deref(), Some("310F"));
    }

    #[test]
    fn rsrp_quality_uses_clear_boundaries() {
        assert_eq!(rsrp_quality(-80.0), fl!("mobile", "signal-excellent"));
        assert_eq!(rsrp_quality(-90.0), fl!("mobile", "signal-good"));
        assert_eq!(rsrp_quality(-100.0), fl!("mobile", "signal-fair"));
        assert_eq!(rsrp_quality(-110.0), fl!("mobile", "signal-poor"));
        assert_eq!(rsrp_quality(-111.0), fl!("mobile", "signal-very-poor"));
    }
}
