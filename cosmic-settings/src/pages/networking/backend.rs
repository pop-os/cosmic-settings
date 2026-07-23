// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::net::Ipv4Addr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded};
use futures::{FutureExt, SinkExt, StreamExt};
use nmrs::agent::{SecretAgent, SecretAgentFlags, SecretRequest, SecretResponder, SecretSetting};
use nmrs::raw::zbus;
use nmrs::raw::zvariant::{OwnedObjectPath, OwnedValue, Str, Value};
use nmrs::{
    ActiveConnection, ConnectType, ConnectionOptions, EapOptions, NetworkEvent, NetworkManager,
    SavedConnection, SettingsSummary, WifiKeyMgmt, WifiSecurity,
};
use secure_string::SecureString;
use tokio::sync::oneshot;

pub type SSID = Arc<str>;
pub type UUID = Arc<str>;

const NM_DEST: &str = "org.freedesktop.NetworkManager";
const NM_PATH: &str = "/org/freedesktop/NetworkManager";
const NM_IFACE: &str = "org.freedesktop.NetworkManager";
const SECRET_ID: &str = "com.system76.CosmicSettings.NetworkManager";

#[derive(Debug)]
pub enum Error {
    Message(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for Error {}

impl From<nmrs::ConnectionError> for Error {
    fn from(value: nmrs::ConnectionError) -> Self {
        Self::Message(value.to_string())
    }
}

impl From<zbus::Error> for Error {
    fn from(value: zbus::Error) -> Self {
        Self::Message(value.to_string())
    }
}

pub mod hw_address {
    #[derive(Clone, PartialEq, Eq, Default, Debug, PartialOrd, Ord)]
    pub struct HwAddress {
        octets: Vec<u8>,
    }

    impl HwAddress {
        pub fn from_str(arg: &str) -> Option<Self> {
            let segments: Vec<&str> = arg.split(':').collect();
            if segments.len() != 6 && segments.len() != 8 {
                return None;
            }

            let mut octets = Vec::with_capacity(segments.len());
            for segment in segments {
                if segment.len() != 2 {
                    return None;
                }
                octets.push(u8::from_str_radix(segment, 16).ok()?);
            }

            Some(Self { octets })
        }
    }

    impl std::fmt::Display for HwAddress {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let hex_parts: Vec<String> = self
                .octets
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect();
            write!(f, "{}", hex_parts.join(":"))
        }
    }
}

pub mod available_wifi {
    use std::sync::Arc;

    use nmrs::raw::zvariant::OwnedObjectPath;

    use super::devices::DeviceState;
    use super::hw_address::HwAddress;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AccessPoint {
        pub ssid: Arc<str>,
        pub strength: u8,
        pub state: DeviceState,
        pub working: bool,
        pub path: OwnedObjectPath,
        pub hw_address: HwAddress,
        pub secured: bool,
        pub wps_push: bool,
        pub network_type: NetworkType,
        pub interface: Option<String>,
        pub bssid: Option<String>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum NetworkType {
        Open,
        PskOrSae,
        Sae,
        EAP,
    }
}

pub mod current_networks {
    use std::net::Ipv4Addr;

    use super::devices::ActiveConnectionState;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ActiveConnectionInfo {
        Wired {
            name: String,
            hw_address: String,
            speed: u32,
            ip_addresses: Vec<Ipv4Addr>,
        },
        WiFi {
            name: String,
            ip_addresses: Vec<Ipv4Addr>,
            hw_address: String,
            state: ActiveConnectionState,
            strength: u8,
        },
        Vpn {
            name: String,
            ip_addresses: Vec<Ipv4Addr>,
        },
    }

    impl ActiveConnectionInfo {
        pub fn name(&self) -> String {
            match self {
                Self::Wired { name, .. } | Self::WiFi { name, .. } | Self::Vpn { name, .. } => {
                    name.clone()
                }
            }
        }
    }
}

pub mod devices {
    use std::sync::Arc;

    use nmrs::raw::zvariant::OwnedObjectPath;
    use nmrs::{NetworkManager, SavedConnection, SettingsSummary};

    use super::{Error, active_connection_by_interface, owned_path, saved_matches_device};

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub enum DeviceType {
        Ethernet,
        Wifi,
        WifiP2P,
        Loopback,
        Bluetooth,
        Vlan,
        WireGuard,
        Other(u32),
    }

    impl From<nmrs::DeviceType> for DeviceType {
        fn from(value: nmrs::DeviceType) -> Self {
            match value {
                nmrs::DeviceType::Ethernet => Self::Ethernet,
                nmrs::DeviceType::Wifi => Self::Wifi,
                nmrs::DeviceType::WifiP2P => Self::WifiP2P,
                nmrs::DeviceType::Loopback => Self::Loopback,
                nmrs::DeviceType::Bluetooth => Self::Bluetooth,
                nmrs::DeviceType::Vlan => Self::Vlan,
                nmrs::DeviceType::Other(v) => Self::Other(v),
                _ => Self::Other(0),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub enum DeviceState {
        Unknown,
        Unmanaged,
        Unavailable,
        Disconnected,
        Prepare,
        Config,
        NeedAuth,
        IpConfig,
        IpCheck,
        Secondaries,
        Activated,
        Deactivating,
        Failed,
        Other(u32),
    }

    impl From<nmrs::DeviceState> for DeviceState {
        fn from(value: nmrs::DeviceState) -> Self {
            match value {
                nmrs::DeviceState::Unmanaged => Self::Unmanaged,
                nmrs::DeviceState::Unavailable => Self::Unavailable,
                nmrs::DeviceState::Disconnected => Self::Disconnected,
                nmrs::DeviceState::Prepare => Self::Prepare,
                nmrs::DeviceState::Config => Self::Config,
                nmrs::DeviceState::NeedAuth => Self::NeedAuth,
                nmrs::DeviceState::IpConfig => Self::IpConfig,
                nmrs::DeviceState::IpCheck => Self::IpCheck,
                nmrs::DeviceState::Secondaries => Self::Secondaries,
                nmrs::DeviceState::Activated => Self::Activated,
                nmrs::DeviceState::Deactivating => Self::Deactivating,
                nmrs::DeviceState::Failed => Self::Failed,
                nmrs::DeviceState::Other(0) => Self::Unknown,
                nmrs::DeviceState::Other(v) => Self::Other(v),
                _ => Self::Unknown,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub enum ActiveConnectionState {
        Unknown,
        Activating,
        Activated,
        Deactivating,
        Deactivated,
        Other(u32),
    }

    impl From<nmrs::ActiveConnectionState> for ActiveConnectionState {
        fn from(value: nmrs::ActiveConnectionState) -> Self {
            match value {
                nmrs::ActiveConnectionState::Activating => Self::Activating,
                nmrs::ActiveConnectionState::Activated => Self::Activated,
                nmrs::ActiveConnectionState::Deactivating => Self::Deactivating,
                nmrs::ActiveConnectionState::Deactivated => Self::Deactivated,
                nmrs::ActiveConnectionState::Unknown => Self::Unknown,
                nmrs::ActiveConnectionState::Other(v) => Self::Other(v),
                _ => Self::Unknown,
            }
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct DeviceInfo {
        pub path: OwnedObjectPath,
        pub device_type: DeviceType,
        pub interface: String,
        pub state: DeviceState,
        pub active_connection: Option<(DeviceConnection, ActiveConnectionState)>,
        pub available_connections: Vec<DeviceConnection>,
        pub known_connections: Vec<KnownDeviceConnection>,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct DeviceConnection {
        pub path: OwnedObjectPath,
        pub id: String,
        pub uuid: Arc<str>,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct KnownDeviceConnection {
        pub id: String,
        pub uuid: Arc<str>,
    }

    pub async fn list(
        nm: &NetworkManager,
        device_type_filter: fn(DeviceType) -> bool,
    ) -> Result<Vec<DeviceInfo>, Error> {
        let (devices_res, saved_res, active_res) = futures::join!(
            nm.list_devices(),
            nm.list_saved_connections(),
            nm.list_active_connections()
        );

        let saved = saved_res?;
        let active = active_res.unwrap_or_default();
        let mut out = Vec::new();

        for device in devices_res? {
            let device_type = DeviceType::from(device.device_type);
            if !device_type_filter(device_type) {
                continue;
            }

            let Ok(path) = owned_path(&device.path) else {
                continue;
            };

            let matching_saved: Vec<&SavedConnection> = saved
                .iter()
                .filter(|connection| {
                    saved_matches_device(connection, device_type, &device.interface)
                })
                .collect();

            let available_connections = matching_saved
                .iter()
                .map(|connection| DeviceConnection {
                    path: connection.path.clone(),
                    id: connection.id.clone(),
                    uuid: Arc::from(connection.uuid.as_str()),
                })
                .collect();

            let known_connections = matching_saved
                .iter()
                .map(|connection| KnownDeviceConnection {
                    id: connection.id.clone(),
                    uuid: Arc::from(connection.uuid.as_str()),
                })
                .collect();

            let active_connection =
                active_connection_by_interface(&active, &device.interface).map(|connection| {
                    (
                        DeviceConnection {
                            path: connection.path.clone(),
                            id: connection.id.clone(),
                            uuid: Arc::from(connection.uuid.as_str()),
                        },
                        connection.state,
                    )
                });

            out.push(DeviceInfo {
                path,
                device_type,
                interface: device.interface,
                state: DeviceState::from(device.state),
                active_connection,
                available_connections,
                known_connections,
            });
        }

        Ok(out)
    }

    pub fn is_wifi_summary(summary: &SettingsSummary) -> bool {
        matches!(summary, SettingsSummary::Wifi { .. })
    }
}

#[derive(Debug, Clone)]
pub struct ActiveConnectionRecord {
    path: OwnedObjectPath,
    id: String,
    uuid: String,
    state: devices::ActiveConnectionState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkManagerState {
    pub wireless_access_points: Vec<available_wifi::AccessPoint>,
    pub active_conns: Vec<current_networks::ActiveConnectionInfo>,
    pub known_access_points: Vec<available_wifi::AccessPoint>,
    pub wifi_enabled: bool,
    pub airplane_mode: bool,
}

impl Default for NetworkManagerState {
    fn default() -> Self {
        Self {
            wireless_access_points: Vec::new(),
            active_conns: Vec::new(),
            known_access_points: Vec::new(),
            wifi_enabled: false,
            airplane_mode: false,
        }
    }
}

impl NetworkManagerState {
    pub async fn new(nm: &NetworkManager) -> Result<Self, Error> {
        let (wifi_state, airplane_mode, active_conns, access_points, saved_connections) = futures::join!(
            nm.wifi_state(),
            nm.airplane_mode_state(),
            nm.list_active_connections(),
            nm.list_access_points(None),
            nm.list_saved_connections()
        );

        let active_conns = active_conns.unwrap_or_default();
        let saved_connections = saved_connections.unwrap_or_default();

        let mut wireless_access_points = access_points
            .unwrap_or_default()
            .into_iter()
            .filter(|ap| !ap.ssid.is_empty() && ap.ssid != "<hidden>")
            .map(access_point_from_nmrs)
            .fold(
                BTreeMap::<(String, SSID), available_wifi::AccessPoint>::new(),
                |mut acc, ap| {
                    let key = (ap.interface.clone().unwrap_or_default(), ap.ssid.clone());
                    let replace = acc
                        .get(&key)
                        .is_none_or(|existing| existing.strength < ap.strength);
                    if replace {
                        acc.insert(key, ap);
                    }
                    acc
                },
            )
            .into_values()
            .collect::<Vec<_>>();
        wireless_access_points.sort_by(|a, b| b.strength.cmp(&a.strength));

        let active_infos = active_connection_infos(active_conns);
        let saved_wifi = saved_wifi_profiles(&saved_connections);

        let mut known_access_points = Vec::new();
        for saved in saved_wifi {
            let Some((ssid, network_type)) = saved_wifi_identity(saved) else {
                continue;
            };
            if active_infos.iter().any(|active| active.name() == ssid) {
                continue;
            }
            if let Some(visible) = wireless_access_points
                .iter()
                .find(|ap| ap.ssid.as_ref() == ssid)
                .cloned()
            {
                known_access_points.push(visible);
                continue;
            }

            known_access_points.push(available_wifi::AccessPoint {
                ssid: Arc::from(ssid),
                strength: 0,
                state: devices::DeviceState::Unknown,
                working: false,
                path: slash_path(),
                hw_address: hw_address::HwAddress::default(),
                secured: !matches!(network_type, available_wifi::NetworkType::Open),
                wps_push: false,
                network_type,
                interface: saved.interface_name.clone(),
                bssid: None,
            });
        }

        known_access_points.sort_by(|a, b| a.ssid.cmp(&b.ssid));
        known_access_points.dedup_by(|a, b| a.ssid == b.ssid);

        Ok(Self {
            wireless_access_points,
            active_conns: active_infos,
            known_access_points,
            wifi_enabled: wifi_state.map(|state| state.enabled).unwrap_or(false),
            airplane_mode: airplane_mode
                .map(|state| state.is_airplane_mode())
                .unwrap_or(false),
        })
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.active_conns = Vec::new();
        self.known_access_points = Vec::new();
        self.wireless_access_points = Vec::new();
    }
}

#[derive(Debug, Clone)]
pub enum Request {
    Activate(OwnedObjectPath, OwnedObjectPath),
    ActivateVpn(UUID),
    Deactivate(UUID),
    Disconnect(SSID),
    Forget(SSID),
    Authenticate {
        ssid: String,
        identity: Option<String>,
        password: SecureString,
        network_type: available_wifi::NetworkType,
        secret_tx: Option<tokio::sync::mpsc::Sender<nm_secret_agent::Request>>,
        interface: Option<String>,
    },
    GetWiFiCredentials(
        SSID,
        UUID,
        available_wifi::NetworkType,
        Option<tokio::sync::mpsc::Sender<nm_secret_agent::Request>>,
    ),
    Reload,
    Remove(UUID),
    SelectAccessPoint(
        SSID,
        available_wifi::NetworkType,
        Option<tokio::sync::mpsc::Sender<nm_secret_agent::Request>>,
        Option<String>,
    ),
    SetAirplaneMode(bool),
    SetWiFi(bool),
}

#[derive(Debug, Clone)]
pub enum Event {
    RequestResponse {
        req: Request,
        state: NetworkManagerState,
        success: bool,
    },
    Init {
        conn: NetworkManager,
        sender: UnboundedSender<Request>,
        state: NetworkManagerState,
    },
    Devices,
    WiFiEnabled(bool),
    WirelessAccessPoints,
    ActiveConns,
    WiFiCredentials {
        ssid: SSID,
        password: Option<SecureString>,
        security_type: available_wifi::NetworkType,
    },
}

pub async fn watch(nm: NetworkManager, mut output: futures::channel::mpsc::Sender<Event>) {
    let (request_tx, mut request_rx): (UnboundedSender<Request>, UnboundedReceiver<Request>) =
        unbounded();

    let state = NetworkManagerState::new(&nm).await.unwrap_or_default();
    if output
        .send(Event::Init {
            conn: nm.clone(),
            sender: request_tx,
            state,
        })
        .await
        .is_err()
    {
        return;
    }

    let Ok(events) = nm.network_events().await else {
        while let Some(req) = request_rx.next().await {
            let event = request_response(&nm, req, false).await;
            _ = output.send(event).await;
        }
        return;
    };

    let mut events = events.fuse();
    loop {
        futures::select! {
            req = request_rx.next().fuse() => {
                let Some(req) = req else {
                    break;
                };
                let event = handle_request(&nm, req).await;
                if output.send(event).await.is_err() {
                    break;
                }
            }
            event = events.next() => {
                let Some(event) = event else {
                    break;
                };
                let message = match event {
                    Ok(NetworkEvent::AccessPointsChanged) => Some(Event::WirelessAccessPoints),
                    Ok(NetworkEvent::DeviceChanged { .. } | NetworkEvent::SettingsChanged(_)) => Some(Event::Devices),
                    Ok(NetworkEvent::ActiveConnectionsChanged) => Some(Event::ActiveConns),
                    Ok(NetworkEvent::WirelessEnabledChanged) => Some(Event::WiFiEnabled(
                        nm.wifi_state().await.map(|state| state.enabled).unwrap_or(false),
                    )),
                    Ok(NetworkEvent::ConnectivityChanged | NetworkEvent::NetworkManagerRestarted) => Some(Event::ActiveConns),
                    Ok(_) => None,
                    Err(err) => {
                        tracing::error!(%err, "network event stream error");
                        None
                    }
                };

                if let Some(message) = message
                    && output.send(message).await.is_err()
                {
                    break;
                }
            }
        }
    }
}

async fn handle_request(nm: &NetworkManager, req: Request) -> Event {
    let success = match &req {
        Request::Activate(device_path, connection_path) => activate_connection(
            nm,
            connection_path.clone(),
            device_path.clone(),
            slash_path(),
        )
        .await
        .is_ok(),
        Request::ActivateVpn(uuid) => nm.connect_vpn_by_uuid(uuid).await.is_ok(),
        Request::Deactivate(uuid) => deactivate_by_uuid(nm, uuid).await.is_ok(),
        Request::Disconnect(ssid) => disconnect_wifi_by_ssid(nm, ssid).await.is_ok(),
        Request::Forget(ssid) => nm.forget(ssid).await.is_ok(),
        Request::Authenticate {
            ssid,
            identity,
            password,
            network_type,
            secret_tx,
            interface,
        } => {
            let result = authenticate_wifi(
                nm,
                ssid,
                identity.as_deref(),
                password,
                *network_type,
                interface.as_deref(),
            )
            .await;
            if let Err(err) = &result {
                tracing::error!(
                    %err,
                    %ssid,
                    ?network_type,
                    ?interface,
                    "wifi authentication failed"
                );
            }
            let success = result.is_ok();

            if success
                && let Some(secret_tx) = secret_tx
                && let Ok(Some(uuid)) = nm.get_saved_connection_uuid(ssid).await
            {
                let (applied_tx, applied_rx) = oneshot::channel();
                let (setting_name, key) = if identity.is_some() {
                    ("802-1x", "password")
                } else {
                    ("802-11-wireless-security", "psk")
                };
                _ = secret_tx
                    .send(nm_secret_agent::Request::SetSecrets {
                        setting_name: setting_name.to_string(),
                        uuid,
                        secrets: HashMap::from([(key.to_string(), password.clone())]),
                        applied_tx,
                    })
                    .await;
                _ = tokio::time::timeout(Duration::from_secs(1), applied_rx).await;
            }

            success
        }
        Request::GetWiFiCredentials(ssid, uuid, security_type, secret_tx) => {
            let password = if matches!(security_type, available_wifi::NetworkType::Open) {
                None
            } else if let Some(secret_tx) = secret_tx {
                let (resp_tx, resp_rx) = oneshot::channel();
                let (setting_name, key) =
                    if matches!(security_type, available_wifi::NetworkType::EAP) {
                        ("802-1x", "password")
                    } else {
                        ("802-11-wireless-security", "psk")
                    };
                _ = secret_tx
                    .send(nm_secret_agent::Request::GetSecrets {
                        setting_name: setting_name.to_string(),
                        uuid: uuid.to_string(),
                        resp_tx,
                    })
                    .await;
                tokio::time::timeout(Duration::from_secs(10), resp_rx)
                    .await
                    .ok()
                    .and_then(Result::ok)
                    .and_then(|mut secrets| secrets.remove(key))
            } else {
                None
            };

            return Event::WiFiCredentials {
                ssid: ssid.clone(),
                password,
                security_type: *security_type,
            };
        }
        Request::Reload => true,
        Request::Remove(uuid) => nm.delete_saved_connection(uuid).await.is_ok(),
        Request::SelectAccessPoint(ssid, network_type, _secret_tx, interface) => {
            let result = select_access_point(nm, ssid, *network_type, interface.as_deref()).await;
            if let Err(err) = &result {
                tracing::error!(
                    %err,
                    %ssid,
                    ?network_type,
                    ?interface,
                    "wifi access point selection failed"
                );
            }
            result.is_ok()
        }
        Request::SetAirplaneMode(enabled) => nm.set_airplane_mode(*enabled).await.is_ok(),
        Request::SetWiFi(enabled) => nm.set_wireless_enabled(*enabled).await.is_ok(),
    };

    request_response(nm, req, success).await
}

async fn request_response(nm: &NetworkManager, req: Request, success: bool) -> Event {
    Event::RequestResponse {
        req,
        success,
        state: NetworkManagerState::new(nm).await.unwrap_or_default(),
    }
}

async fn select_access_point(
    nm: &NetworkManager,
    ssid: &str,
    network_type: available_wifi::NetworkType,
    interface: Option<&str>,
) -> Result<(), Error> {
    match network_type {
        available_wifi::NetworkType::Open => {
            nm.connect(ssid, interface, WifiSecurity::Open).await?;
        }
        available_wifi::NetworkType::PskOrSae => {
            nm.connect(ssid, interface, WifiSecurity::WpaPsk { psk: String::new() })
                .await?;
        }
        available_wifi::NetworkType::Sae | available_wifi::NetworkType::EAP => {
            let Some(saved) = find_saved_wifi(nm, ssid, interface).await? else {
                return Err(Error::Message("no saved Wi-Fi connection".to_string()));
            };
            let device = saved
                .interface_name
                .as_deref()
                .or(interface)
                .map(|interface| nm.get_device_by_interface(interface))
                .ok_or_else(|| {
                    Error::Message("no Wi-Fi interface for saved connection".to_string())
                })?
                .await?;
            let ap = find_access_point_path(nm, ssid, interface)
                .await
                .unwrap_or_else(slash_path);
            activate_connection(nm, saved.path, device, ap).await?;
        }
    }

    Ok(())
}

async fn authenticate_wifi(
    nm: &NetworkManager,
    ssid: &str,
    identity: Option<&str>,
    password: &SecureString,
    network_type: available_wifi::NetworkType,
    interface: Option<&str>,
) -> Result<(), Error> {
    match network_type {
        available_wifi::NetworkType::Open => {
            nm.connect(ssid, interface, WifiSecurity::Open).await?
        }
        available_wifi::NetworkType::PskOrSae => {
            nm.connect(
                ssid,
                interface,
                WifiSecurity::WpaPsk {
                    psk: password.unsecure().to_string(),
                },
            )
            .await?;
        }
        available_wifi::NetworkType::Sae => {
            connect_sae(nm, ssid, password.unsecure(), interface).await?;
        }
        available_wifi::NetworkType::EAP => {
            nm.connect(
                ssid,
                interface,
                WifiSecurity::WpaEap {
                    opts: EapOptions::new(identity.unwrap_or_default(), password.unsecure()),
                },
            )
            .await?;
        }
    }

    Ok(())
}

async fn connect_sae(
    nm: &NetworkManager,
    ssid: &str,
    password: &str,
    interface: Option<&str>,
) -> Result<(), Error> {
    let mut settings = nmrs::builders::WifiConnectionBuilder::new(ssid)
        .wpa_psk(password)
        .autoconnect(true)
        .ipv4_auto()
        .ipv6_auto()
        .build();

    if let Some(security) = settings.get_mut("802-11-wireless-security") {
        security.insert("key-mgmt", Value::from("sae"));
        security.remove("auth-alg");
    }

    let ap_path = find_access_point_path(nm, ssid, interface)
        .await
        .ok_or_else(|| Error::Message(format!("access point {ssid} not found")))?;
    nm.add_and_activate_connection(settings, interface, Some(ap_path.as_str()))
        .await?;

    Ok(())
}

async fn disconnect_wifi_by_ssid(nm: &NetworkManager, ssid: &str) -> Result<(), Error> {
    for active in nm.list_active_connections().await? {
        if let ActiveConnection::Wifi(wifi) = active
            && wifi.ssid == ssid
        {
            return deactivate_by_uuid(nm, &wifi.uuid).await;
        }
    }

    Ok(())
}

async fn deactivate_by_uuid(nm: &NetworkManager, uuid: &str) -> Result<(), Error> {
    if nm.disconnect_vpn_by_uuid(uuid).await.is_ok() {
        return Ok(());
    }

    let active_path = find_active_connection_path(nm, uuid).await?;
    let proxy = zbus::Proxy::new(nm.dbus_connection(), NM_DEST, NM_PATH, NM_IFACE).await?;
    proxy
        .call_method("DeactivateConnection", &(active_path,))
        .await?;

    Ok(())
}

async fn activate_connection(
    nm: &NetworkManager,
    connection_path: OwnedObjectPath,
    device_path: OwnedObjectPath,
    specific_path: OwnedObjectPath,
) -> Result<(), Error> {
    let proxy = zbus::Proxy::new(nm.dbus_connection(), NM_DEST, NM_PATH, NM_IFACE).await?;
    proxy
        .call_method(
            "ActivateConnection",
            &(connection_path, device_path, specific_path),
        )
        .await?;
    Ok(())
}

async fn find_active_connection_path(
    nm: &NetworkManager,
    uuid: &str,
) -> Result<OwnedObjectPath, Error> {
    let proxy = zbus::Proxy::new(nm.dbus_connection(), NM_DEST, NM_PATH, NM_IFACE).await?;
    let paths: Vec<OwnedObjectPath> = proxy.get_property("ActiveConnections").await?;

    for path in paths {
        let active = zbus::Proxy::new(
            nm.dbus_connection(),
            NM_DEST,
            path.clone(),
            "org.freedesktop.NetworkManager.Connection.Active",
        )
        .await?;
        let active_uuid: String = active.get_property("Uuid").await?;
        if active_uuid == uuid {
            return Ok(path);
        }
    }

    Err(Error::Message(format!(
        "active connection {uuid} not found"
    )))
}

async fn find_access_point_path(
    nm: &NetworkManager,
    ssid: &str,
    interface: Option<&str>,
) -> Option<OwnedObjectPath> {
    nm.list_access_points(interface)
        .await
        .ok()?
        .into_iter()
        .filter(|ap| ap.ssid == ssid)
        .max_by_key(|ap| ap.strength)
        .map(|ap| ap.path)
}

async fn find_saved_wifi(
    nm: &NetworkManager,
    ssid: &str,
    interface: Option<&str>,
) -> Result<Option<SavedConnection>, Error> {
    Ok(nm
        .list_saved_connections()
        .await?
        .into_iter()
        .find(|connection| {
            if let Some(interface) = interface
                && connection
                    .interface_name
                    .as_deref()
                    .is_some_and(|i| i != interface)
            {
                return false;
            }
            saved_wifi_identity(connection).is_some_and(|(saved_ssid, _)| saved_ssid == ssid)
        }))
}

pub async fn wifi_connection_settings(
    nm: NetworkManager,
) -> Result<BTreeMap<Box<str>, Box<str>>, Error> {
    Ok(nm
        .list_saved_connections()
        .await?
        .into_iter()
        .filter_map(|connection| {
            saved_wifi_identity(&connection)
                .map(|(ssid, _)| (ssid.into_boxed_str(), connection.uuid.into_boxed_str()))
        })
        .collect())
}

pub async fn import_openvpn(path: impl AsRef<Path>) -> Result<(), Error> {
    use nmrs::builders::{OpenVpnBuilder, build_openvpn_connection};

    let nm = NetworkManager::new().await?;
    let config = OpenVpnBuilder::from_ovpn_file(path)?.build()?;
    let settings = build_openvpn_connection(&config, &ConnectionOptions::new(false))?;
    nm.add_connection(settings).await?;
    Ok(())
}

pub async fn import_wireguard(path: impl AsRef<Path>, name: &str) -> Result<(), Error> {
    use nmrs::builders::WireGuardBuilder;

    let nm = NetworkManager::new().await?;
    let config = parse_wireguard_config(path.as_ref(), name)?;
    let mut builder = WireGuardBuilder::new(&config.name)
        .private_key(config.private_key)
        .address(config.address)
        .add_peers(config.peers)
        .autoconnect(false);

    if !config.dns.is_empty() {
        builder = builder.dns(config.dns);
    }
    if let Some(mtu) = config.mtu {
        builder = builder.mtu(mtu);
    }

    nm.add_connection(builder.build()?).await?;
    Ok(())
}

struct ParsedWireGuard {
    name: String,
    private_key: String,
    address: String,
    dns: Vec<String>,
    mtu: Option<u32>,
    peers: Vec<nmrs::WireGuardPeer>,
}

fn parse_wireguard_config(path: &Path, name: &str) -> Result<ParsedWireGuard, Error> {
    let contents = std::fs::read_to_string(path)
        .map_err(|err| Error::Message(format!("failed to read {}: {err}", path.display())))?;
    let mut section = "";
    let mut private_key = None;
    let mut address = None;
    let mut dns = Vec::new();
    let mut mtu = None;
    let mut peers = Vec::new();
    let mut peer_public_key: Option<String> = None;
    let mut peer_endpoint: Option<String> = None;
    let mut peer_allowed_ips: Vec<String> = Vec::new();
    let mut peer_preshared_key: Option<String> = None;
    let mut peer_keepalive: Option<u32> = None;

    let flush_peer = |peers: &mut Vec<nmrs::WireGuardPeer>,
                      public_key: &mut Option<String>,
                      endpoint: &mut Option<String>,
                      allowed_ips: &mut Vec<String>,
                      preshared_key: &mut Option<String>,
                      keepalive: &mut Option<u32>|
     -> Result<(), Error> {
        if public_key.is_none()
            && endpoint.is_none()
            && allowed_ips.is_empty()
            && preshared_key.is_none()
            && keepalive.is_none()
        {
            return Ok(());
        }

        let public_key = public_key
            .take()
            .ok_or_else(|| Error::Message("WireGuard peer is missing PublicKey".to_string()))?;
        let endpoint = endpoint
            .take()
            .ok_or_else(|| Error::Message("WireGuard peer is missing Endpoint".to_string()))?;
        if allowed_ips.is_empty() {
            return Err(Error::Message(
                "WireGuard peer is missing AllowedIPs".to_string(),
            ));
        }

        let mut peer = nmrs::WireGuardPeer::new(public_key, endpoint, std::mem::take(allowed_ips));
        if let Some(psk) = preshared_key.take() {
            peer = peer.with_preshared_key(psk);
        }
        if let Some(interval) = keepalive.take() {
            peer = peer.with_persistent_keepalive(interval);
        }
        peers.push(peer);
        Ok(())
    };

    for raw_line in contents.lines() {
        let line = raw_line
            .split_once('#')
            .map_or(raw_line, |(line, _)| line)
            .trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            if section.eq_ignore_ascii_case("Peer") {
                flush_peer(
                    &mut peers,
                    &mut peer_public_key,
                    &mut peer_endpoint,
                    &mut peer_allowed_ips,
                    &mut peer_preshared_key,
                    &mut peer_keepalive,
                )?;
            }
            section = &line[1..line.len() - 1];
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();

        if section.eq_ignore_ascii_case("Interface") {
            match key.to_ascii_lowercase().as_str() {
                "privatekey" => private_key = Some(value.to_string()),
                "address" => {
                    address = value
                        .split(',')
                        .map(str::trim)
                        .find(|value| value.contains('.'))
                        .or_else(|| value.split(',').map(str::trim).next())
                        .map(ToOwned::to_owned);
                }
                "dns" => dns.extend(
                    value
                        .split(',')
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .map(ToOwned::to_owned),
                ),
                "mtu" => mtu = value.parse().ok(),
                _ => {}
            }
        } else if section.eq_ignore_ascii_case("Peer") {
            match key.to_ascii_lowercase().as_str() {
                "publickey" => peer_public_key = Some(value.to_string()),
                "endpoint" => peer_endpoint = Some(value.to_string()),
                "allowedips" => {
                    peer_allowed_ips.extend(
                        value
                            .split(',')
                            .map(str::trim)
                            .filter(|value| !value.is_empty())
                            .map(ToOwned::to_owned),
                    );
                }
                "presharedkey" => peer_preshared_key = Some(value.to_string()),
                "persistentkeepalive" => peer_keepalive = value.parse().ok(),
                _ => {}
            }
        }
    }

    if section.eq_ignore_ascii_case("Peer") {
        flush_peer(
            &mut peers,
            &mut peer_public_key,
            &mut peer_endpoint,
            &mut peer_allowed_ips,
            &mut peer_preshared_key,
            &mut peer_keepalive,
        )?;
    }

    Ok(ParsedWireGuard {
        name: name.to_string(),
        private_key: private_key
            .ok_or_else(|| Error::Message("WireGuard config is missing PrivateKey".to_string()))?,
        address: address
            .ok_or_else(|| Error::Message("WireGuard config is missing Address".to_string()))?,
        dns,
        mtu,
        peers,
    })
}

fn active_connection_infos(
    active: Vec<ActiveConnection>,
) -> Vec<current_networks::ActiveConnectionInfo> {
    let mut infos = active
        .into_iter()
        .filter_map(|connection| match connection {
            ActiveConnection::Wired(wired) => Some(current_networks::ActiveConnectionInfo::Wired {
                name: wired.id,
                hw_address: wired.hw_address.unwrap_or_default(),
                speed: wired.speed_mbps.unwrap_or_default(),
                ip_addresses: parse_ipv4_list([wired.ip4_address]),
            }),
            ActiveConnection::Wifi(wifi) => Some(current_networks::ActiveConnectionInfo::WiFi {
                name: wifi.ssid,
                ip_addresses: parse_ipv4_list([wifi.ip4_address]),
                hw_address: wifi.bssid.unwrap_or_default(),
                state: devices::ActiveConnectionState::from(wifi.state),
                strength: wifi.strength.unwrap_or_default(),
            }),
            ActiveConnection::Vpn(vpn) => Some(current_networks::ActiveConnectionInfo::Vpn {
                name: vpn.id,
                ip_addresses: parse_ipv4_list([vpn.ip4_address]),
            }),
            ActiveConnection::Other(_) => None,
            _ => None,
        })
        .collect::<Vec<_>>();

    infos.sort_by(|a, b| {
        let helper = |conn: &current_networks::ActiveConnectionInfo| match conn {
            current_networks::ActiveConnectionInfo::Vpn { name, .. } => format!("0{name}"),
            current_networks::ActiveConnectionInfo::Wired { name, .. } => format!("1{name}"),
            current_networks::ActiveConnectionInfo::WiFi { name, .. } => format!("2{name}"),
        };
        helper(a).cmp(&helper(b))
    });

    infos
}

fn access_point_from_nmrs(ap: nmrs::AccessPoint) -> available_wifi::AccessPoint {
    let network_type = match ap.security.preferred_connect_type() {
        ConnectType::Open | ConnectType::Owe => available_wifi::NetworkType::Open,
        ConnectType::Eap => available_wifi::NetworkType::EAP,
        ConnectType::Psk => available_wifi::NetworkType::PskOrSae,
        ConnectType::Sae => available_wifi::NetworkType::Sae,
        _ => available_wifi::NetworkType::PskOrSae,
    };

    available_wifi::AccessPoint {
        ssid: Arc::from(ap.ssid.as_str()),
        strength: ap.strength,
        state: devices::DeviceState::from(ap.device_state),
        working: false,
        path: ap.path,
        hw_address: hw_address::HwAddress::from_str(&ap.bssid).unwrap_or_default(),
        secured: !ap.security.is_open(),
        wps_push: ap.security.wps,
        network_type,
        interface: Some(ap.interface),
        bssid: Some(ap.bssid),
    }
}

fn saved_wifi_profiles(saved: &[SavedConnection]) -> impl Iterator<Item = &SavedConnection> {
    saved.iter().filter(|connection| {
        connection.connection_type == "802-11-wireless"
            && matches!(connection.summary, SettingsSummary::Wifi { .. })
    })
}

fn saved_wifi_identity(saved: &SavedConnection) -> Option<(String, available_wifi::NetworkType)> {
    let SettingsSummary::Wifi { ssid, security, .. } = &saved.summary else {
        return None;
    };
    if ssid.is_empty() {
        return None;
    }

    let network_type = match security.as_ref().map(|s| s.key_mgmt) {
        None | Some(WifiKeyMgmt::None | WifiKeyMgmt::Owe | WifiKeyMgmt::OweTransitionMode) => {
            available_wifi::NetworkType::Open
        }
        Some(WifiKeyMgmt::WpaEap) => available_wifi::NetworkType::EAP,
        Some(WifiKeyMgmt::Wep | WifiKeyMgmt::WpaPsk) => available_wifi::NetworkType::PskOrSae,
        Some(WifiKeyMgmt::Sae) => available_wifi::NetworkType::Sae,
        Some(_) => available_wifi::NetworkType::PskOrSae,
    };

    Some((ssid.clone(), network_type))
}

fn saved_matches_device(
    connection: &SavedConnection,
    device_type: devices::DeviceType,
    interface: &str,
) -> bool {
    let type_matches = match device_type {
        devices::DeviceType::Ethernet => connection.connection_type == "802-3-ethernet",
        devices::DeviceType::Wifi => connection.connection_type == "802-11-wireless",
        devices::DeviceType::WireGuard => connection.connection_type == "wireguard",
        _ => false,
    };

    type_matches
        && connection
            .interface_name
            .as_deref()
            .is_none_or(|name| name == interface)
}

fn active_connection_by_interface(
    active: &[ActiveConnection],
    interface: &str,
) -> Option<ActiveConnectionRecord> {
    for connection in active {
        match connection {
            ActiveConnection::Wired(wired) if wired.interface.as_deref() == Some(interface) => {
                return Some(ActiveConnectionRecord {
                    path: slash_path(),
                    id: wired.id.clone(),
                    uuid: wired.uuid.clone(),
                    state: devices::ActiveConnectionState::from(wired.state),
                });
            }
            ActiveConnection::Wifi(wifi) if wifi.interface.as_deref() == Some(interface) => {
                return Some(ActiveConnectionRecord {
                    path: slash_path(),
                    id: wifi.id.clone(),
                    uuid: wifi.uuid.clone(),
                    state: devices::ActiveConnectionState::from(wifi.state),
                });
            }
            ActiveConnection::Vpn(vpn) if vpn.interface.as_deref() == Some(interface) => {
                return Some(ActiveConnectionRecord {
                    path: slash_path(),
                    id: vpn.id.clone(),
                    uuid: vpn.uuid.clone(),
                    state: devices::ActiveConnectionState::from(vpn.state),
                });
            }
            _ => {}
        }
    }

    None
}

fn parse_ipv4_list(addresses: impl IntoIterator<Item = Option<String>>) -> Vec<Ipv4Addr> {
    addresses
        .into_iter()
        .flatten()
        .filter_map(|address| {
            address
                .split_once('/')
                .map_or(address.as_str(), |(address, _)| address)
                .parse()
                .ok()
        })
        .collect()
}

fn slash_path() -> OwnedObjectPath {
    OwnedObjectPath::try_from("/").expect("slash is a valid object path")
}

fn owned_path(path: &str) -> Result<OwnedObjectPath, Error> {
    OwnedObjectPath::try_from(path)
        .map_err(|err| Error::Message(format!("invalid object path {path}: {err}")))
}

pub mod nm_secret_agent {
    use super::*;

    pub type SecretSender =
        Arc<tokio::sync::Mutex<Option<tokio::sync::oneshot::Sender<SecureString>>>>;

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub enum PasswordFlag {
        None = 0,
        AgentOwned = 1,
        NotSaved = 2,
        NotRequired = 4,
    }

    #[derive(Clone, Debug)]
    pub struct Error(pub String);

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.0)
        }
    }

    impl std::error::Error for Error {}

    #[derive(Debug, Clone)]
    pub enum Event {
        RequestSecret {
            uuid: String,
            name: String,
            description: Option<String>,
            previous: SecureString,
            tx: SecretSender,
        },
        CancelGetSecrets {
            uuid: String,
            name: String,
        },
        Failed(Error),
    }

    #[derive(Debug)]
    pub enum Request {
        SetSecrets {
            setting_name: String,
            uuid: String,
            secrets: HashMap<String, SecureString>,
            applied_tx: oneshot::Sender<()>,
        },
        GetSecrets {
            setting_name: String,
            uuid: String,
            resp_tx: oneshot::Sender<HashMap<String, SecureString>>,
        },
    }

    pub fn secret_agent_stream(
        identifier: impl AsRef<str>,
        rx: tokio::sync::mpsc::Receiver<Request>,
    ) -> impl futures::Stream<Item = Event> {
        let identifier = identifier.as_ref().to_string();
        cosmic::iced::stream::channel(
            4,
            move |mut msg_tx: futures::channel::mpsc::Sender<Event>| async move {
                if let Err(error) = secret_agent_stream_impl(&identifier, msg_tx.clone(), rx).await
                {
                    _ = msg_tx.send(Event::Failed(error)).await;
                }
            },
        )
    }

    async fn secret_agent_stream_impl(
        identifier: &str,
        mut msg_tx: futures::channel::mpsc::Sender<Event>,
        mut rx: tokio::sync::mpsc::Receiver<Request>,
    ) -> Result<(), Error> {
        unlock_collection().await?;

        let (_handle, mut requests) = SecretAgent::builder()
            .with_identifier(identifier)
            .register()
            .await
            .map_err(|err| Error(err.to_string()))?;

        loop {
            futures::select! {
                request = rx.recv().fuse() => {
                    let Some(request) = request else {
                        break;
                    };
                    handle_control_request(request).await?;
                }
                request = requests.next().fuse() => {
                    let Some(request) = request else {
                        break;
                    };
                    handle_secret_request(&mut msg_tx, request).await?;
                }
            }
        }

        Ok(())
    }

    async fn handle_control_request(request: Request) -> Result<(), Error> {
        match request {
            Request::SetSecrets {
                setting_name,
                uuid,
                secrets,
                applied_tx,
            } => {
                if secrets.is_empty() {
                    delete_secrets(&uuid).await?;
                } else {
                    store_secrets(&uuid, &setting_name, &secrets).await?;
                }
                _ = applied_tx.send(());
            }
            Request::GetSecrets {
                setting_name,
                uuid,
                resp_tx,
            } => {
                let secrets = get_secrets(&uuid, &setting_name).await?;
                _ = resp_tx.send(secrets);
            }
        }

        Ok(())
    }

    async fn handle_secret_request(
        msg_tx: &mut futures::channel::mpsc::Sender<Event>,
        request: SecretRequest,
    ) -> Result<(), Error> {
        let setting_name = setting_name(&request.setting);
        if !request.flags.contains(SecretAgentFlags::REQUEST_NEW) {
            let stored = get_secrets(&request.connection_uuid, setting_name).await?;
            if !stored.is_empty() {
                return respond_with_stored(request, stored).await;
            }
        }

        if !request
            .flags
            .intersects(SecretAgentFlags::ALLOW_INTERACTION | SecretAgentFlags::REQUEST_NEW)
        {
            request
                .responder
                .no_secrets()
                .await
                .map_err(|err| Error(err.to_string()))?;
            return Ok(());
        }

        let (tx, rx) = oneshot::channel();
        let tx = Arc::new(tokio::sync::Mutex::new(Some(tx)));
        let description = match &request.setting {
            SecretSetting::WifiPsk { ssid } => Some(ssid.clone()),
            SecretSetting::WifiEap { identity, method } => Some(
                format!(
                    "{} {}",
                    identity.clone().unwrap_or_default(),
                    method.clone().unwrap_or_default()
                )
                .trim()
                .to_string(),
            ),
            SecretSetting::Vpn {
                service_type,
                user_name,
            } => Some(user_name.clone().unwrap_or_else(|| service_type.clone())),
            SecretSetting::Other(setting) => Some(setting.clone()),
            _ => None,
        }
        .filter(|value| !value.is_empty());

        let event_name = match &request.setting {
            SecretSetting::WifiPsk { .. } | SecretSetting::WifiEap { .. } => {
                request.connection_uuid.clone()
            }
            _ => request.connection_id.clone(),
        };

        let response_kind = ResponseKind::from(&request);
        let uuid = request.connection_uuid.clone();
        msg_tx
            .send(Event::RequestSecret {
                uuid: uuid.clone(),
                name: event_name,
                description,
                previous: SecureString::from(""),
                tx,
            })
            .await
            .map_err(|err| Error(err.to_string()))?;

        match rx.await {
            Ok(secret) => {
                let mut stored = HashMap::new();
                let key = response_kind.key();
                stored.insert(key.to_string(), secret.clone());
                store_secrets(&uuid, setting_name, &stored).await?;
                response_kind.respond(request.responder, secret).await?;
            }
            Err(_) => {
                request
                    .responder
                    .cancel()
                    .await
                    .map_err(|err| Error(err.to_string()))?;
            }
        }

        Ok(())
    }

    enum ResponseKind {
        WifiPsk,
        WifiEap { identity: Option<String> },
        Vpn { key: String },
        Raw { setting_name: String, key: String },
    }

    impl ResponseKind {
        fn from(request: &SecretRequest) -> Self {
            match &request.setting {
                SecretSetting::WifiPsk { .. } => Self::WifiPsk,
                SecretSetting::WifiEap { identity, .. } => Self::WifiEap {
                    identity: identity.clone(),
                },
                SecretSetting::Vpn { .. } => Self::Vpn {
                    key: request
                        .hints
                        .iter()
                        .find(|hint| !hint.contains(':'))
                        .cloned()
                        .unwrap_or_else(|| "password".to_string()),
                },
                SecretSetting::Other(setting_name) => Self::Raw {
                    setting_name: setting_name.clone(),
                    key: request
                        .hints
                        .first()
                        .cloned()
                        .unwrap_or_else(|| "password".to_string()),
                },
                _ => Self::Raw {
                    setting_name: setting_name(&request.setting).to_string(),
                    key: "password".to_string(),
                },
            }
        }

        fn key(&self) -> &str {
            match self {
                Self::WifiPsk => "psk",
                Self::WifiEap { .. } => "password",
                Self::Vpn { key } | Self::Raw { key, .. } => key.as_str(),
            }
        }

        async fn respond(
            self,
            responder: SecretResponder,
            secret: SecureString,
        ) -> Result<(), Error> {
            match self {
                Self::WifiPsk => responder
                    .wifi_psk(secret.unsecure().to_string())
                    .await
                    .map_err(|err| Error(err.to_string())),
                Self::WifiEap { identity } => responder
                    .wifi_eap(identity, secret.unsecure().to_string())
                    .await
                    .map_err(|err| Error(err.to_string())),
                Self::Vpn { key } => responder
                    .vpn_secrets(HashMap::from([(key, secret.unsecure().to_string())]))
                    .await
                    .map_err(|err| Error(err.to_string())),
                Self::Raw { setting_name, key } => {
                    let mut inner = HashMap::new();
                    inner.insert(
                        key,
                        OwnedValue::from(Str::from(secret.unsecure().to_string())),
                    );
                    responder
                        .raw(setting_name, inner)
                        .await
                        .map_err(|err| Error(err.to_string()))
                }
            }
        }
    }

    async fn respond_with_stored(
        request: SecretRequest,
        stored: HashMap<String, SecureString>,
    ) -> Result<(), Error> {
        match &request.setting {
            SecretSetting::WifiPsk { .. } => {
                if let Some(psk) = stored.get("psk") {
                    request
                        .responder
                        .wifi_psk(psk.unsecure().to_string())
                        .await
                        .map_err(|err| Error(err.to_string()))?;
                } else {
                    request
                        .responder
                        .no_secrets()
                        .await
                        .map_err(|err| Error(err.to_string()))?;
                }
            }
            SecretSetting::WifiEap { identity, .. } => {
                if let Some(password) = stored.get("password") {
                    request
                        .responder
                        .wifi_eap(identity.clone(), password.unsecure().to_string())
                        .await
                        .map_err(|err| Error(err.to_string()))?;
                } else {
                    request
                        .responder
                        .no_secrets()
                        .await
                        .map_err(|err| Error(err.to_string()))?;
                }
            }
            SecretSetting::Vpn { .. } => {
                let secrets = stored
                    .into_iter()
                    .map(|(key, value)| (key, value.unsecure().to_string()))
                    .collect();
                request
                    .responder
                    .vpn_secrets(secrets)
                    .await
                    .map_err(|err| Error(err.to_string()))?;
            }
            _ => {
                let setting_name = setting_name(&request.setting).to_string();
                let inner = stored
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            key,
                            OwnedValue::from(Str::from(value.unsecure().to_string())),
                        )
                    })
                    .collect();
                request
                    .responder
                    .raw(setting_name, inner)
                    .await
                    .map_err(|err| Error(err.to_string()))?;
            }
        }

        Ok(())
    }

    fn setting_name(setting: &SecretSetting) -> &'static str {
        match setting {
            SecretSetting::WifiPsk { .. } => "802-11-wireless-security",
            SecretSetting::WifiEap { .. } => "802-1x",
            SecretSetting::Vpn { .. } => "vpn",
            SecretSetting::Gsm => "gsm",
            SecretSetting::Cdma => "cdma",
            SecretSetting::Pppoe => "pppoe",
            SecretSetting::Other(_) => "connection",
            _ => "connection",
        }
    }

    async fn unlock_collection() -> Result<(), Error> {
        let ss = secret_service::SecretService::connect(secret_service::EncryptionType::Dh)
            .await
            .map_err(|err| Error(err.to_string()))?;
        let collection = ss
            .get_default_collection()
            .await
            .map_err(|err| Error(err.to_string()))?;
        if collection
            .is_locked()
            .await
            .map_err(|err| Error(err.to_string()))?
        {
            collection
                .unlock()
                .await
                .map_err(|err| Error(err.to_string()))?;
        }
        Ok(())
    }

    async fn store_secrets(
        uuid: &str,
        setting_name: &str,
        secrets: &HashMap<String, SecureString>,
    ) -> Result<(), Error> {
        let ss = secret_service::SecretService::connect(secret_service::EncryptionType::Dh)
            .await
            .map_err(|err| Error(err.to_string()))?;
        let collection = ss
            .get_default_collection()
            .await
            .map_err(|err| Error(err.to_string()))?;

        for (name, secret) in secrets {
            let mut attributes = HashMap::new();
            attributes.insert("application", SECRET_ID);
            attributes.insert("uuid", uuid);
            attributes.insert("setting_name", setting_name);
            attributes.insert("name", name.as_str());
            collection
                .create_item(
                    "NetworkManager Secret",
                    attributes,
                    secret.unsecure().as_bytes(),
                    true,
                    "text/plain",
                )
                .await
                .map_err(|err| Error(err.to_string()))?;
        }

        Ok(())
    }

    async fn get_secrets(
        uuid: &str,
        setting_name: &str,
    ) -> Result<HashMap<String, SecureString>, Error> {
        let ss = secret_service::SecretService::connect(secret_service::EncryptionType::Dh)
            .await
            .map_err(|err| Error(err.to_string()))?;
        let collection = ss
            .get_default_collection()
            .await
            .map_err(|err| Error(err.to_string()))?;
        let mut attributes = HashMap::new();
        attributes.insert("application", SECRET_ID);
        attributes.insert("uuid", uuid);
        attributes.insert("setting_name", setting_name);

        let search_items = collection
            .search_items(attributes)
            .await
            .map_err(|err| Error(err.to_string()))?;
        let mut secrets = HashMap::new();
        for item in &search_items {
            let name = item
                .get_attributes()
                .await
                .map_err(|err| Error(err.to_string()))?
                .get("name")
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            let secret = item
                .get_secret()
                .await
                .map_err(|err| Error(err.to_string()))?;
            let secret = String::from_utf8(secret).map_err(|err| Error(err.to_string()))?;
            secrets.insert(name, SecureString::from(secret));
        }
        Ok(secrets)
    }

    async fn delete_secrets(uuid: &str) -> Result<(), Error> {
        let ss = secret_service::SecretService::connect(secret_service::EncryptionType::Dh)
            .await
            .map_err(|err| Error(err.to_string()))?;
        let collection = ss
            .get_default_collection()
            .await
            .map_err(|err| Error(err.to_string()))?;
        let mut attributes = HashMap::new();
        attributes.insert("application", SECRET_ID);
        attributes.insert("uuid", uuid);

        let search_items = collection
            .search_items(attributes)
            .await
            .map_err(|err| Error(err.to_string()))?;
        for item in &search_items {
            item.delete().await.map_err(|err| Error(err.to_string()))?;
        }
        Ok(())
    }
}
