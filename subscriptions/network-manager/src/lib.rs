// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

pub mod active_conns;
pub mod available_wifi;
pub mod current_networks;
pub mod devices;
pub mod hw_address;
pub mod wireless_enabled;

use std::{collections::HashMap, fmt::Debug, sync::Arc, time::Duration};

use available_wifi::NetworkType;
pub use cosmic_dbus_networkmanager as dbus;
pub use dbus::settings::connection::Settings;

use cosmic_dbus_networkmanager::{
    active_connection::ActiveConnection,
    device::SpecificDevice,
    interface::{
        active_connection::ActiveConnectionProxy,
        enums::{self, ActiveConnectionState, DeviceType, NmConnectivityState},
    },
    nm::NetworkManager,
    settings::NetworkManagerSettings,
};
use futures::{
    FutureExt, SinkExt, StreamExt,
    channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded},
};
use hw_address::HwAddress;
use iced_futures::{Subscription, stream};
use secure_string::SecureString;
use tokio::process::Command;
use zbus::zvariant::{self, ObjectPath, Value};

use self::{
    available_wifi::{AccessPoint, handle_wireless_device},
    current_networks::{ActiveConnectionInfo, active_connections},
};

pub type SSID = Arc<str>;
pub type UUID = Arc<str>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("access point not found")]
    AccessPointNotFound,
    #[error("failed to list bluetooth devices with rfkill")]
    BluetoothRfkillList(std::io::Error),
    #[error("failed to activate connection")]
    ConnectionActivate,
    #[error("no wifi devices found")]
    NoWiFiDevices,
    #[error("zbus error")]
    Zbus(#[from] zbus::Error),
}

#[derive(Debug)]
pub enum State {
    Ready(zbus::Connection),
    Waiting(zbus::Connection, UnboundedReceiver<Request>),
    Finished,
}

/// Reloads state on available connection changes.
pub async fn watch_connections_changed(
    conn: zbus::Connection,
    mut output: futures::channel::mpsc::Sender<Event>,
) {
    let Ok(nm) = NetworkManager::new(&conn).await else {
        return;
    };

    let mut device_stream = nm.receive_devices_changed().await;

    loop {
        // Emits updates when available connections changes.
        let connections_changed = std::pin::pin!(async {
            let devices = nm.devices().await.unwrap_or_default();

            let mut connection_streams =
                futures::stream::FuturesUnordered::from_iter(devices.into_iter().map(
                    |device| async move { device.receive_available_connections_changed().await },
                ))
                .collect::<Vec<_>>()
                .await;

            let mut available_connections = futures::stream::FuturesUnordered::from_iter(
                connection_streams
                    .iter_mut()
                    .map(|stream| async { stream.next().await }),
            );

            loop {
                _ = futures::join!(
                    available_connections.next(),
                    tokio::time::sleep(Duration::from_secs(3))
                );

                // TODO: although it should consume the stream, the stream is never empty.
                // while available_connections.next().now_or_never().is_some() {}

                let state = NetworkManagerState::new(&conn).await.unwrap_or_default();

                _ = output
                    .send(Event::RequestResponse {
                        req: Request::Reload,
                        state,
                        success: true,
                    })
                    .await;
            }
        });

        // Reload the connection streams whenever devices change.
        futures::future::select(connections_changed, device_stream.next()).await;
    }
}

pub fn subscription<I: Copy + Debug + std::hash::Hash + 'static>(
    id: I,
    conn: zbus::Connection,
) -> iced_futures::Subscription<Event> {
    Subscription::run_with_id(
        id,
        stream::channel(50, |output| async move {
            watch(conn, output).await;
            futures::future::pending().await
        }),
    )
}

pub async fn watch(conn: zbus::Connection, mut output: futures::channel::mpsc::Sender<Event>) {
    let mut state = State::Ready(conn);

    loop {
        state = start_listening(state, &mut output).await;
    }
}

async fn start_listening(
    state: State,
    output: &mut futures::channel::mpsc::Sender<Event>,
) -> State {
    match state {
        State::Ready(conn) => {
            let (tx, rx) = unbounded();

            if output
                .send(Event::Init {
                    conn: conn.clone(),
                    sender: tx,
                    state: NetworkManagerState::new(&conn).await.unwrap_or_default(),
                })
                .await
                .is_ok()
            {
                State::Waiting(conn, rx)
            } else {
                State::Finished
            }
        }
        State::Waiting(conn, mut rx) => {
            let network_manager = match NetworkManager::new(&conn).await {
                Ok(n) => n,
                Err(_) => return State::Finished,
            };

            match rx.next().await {
                Some(Request::Deactivate(uuid)) => {
                    let mut success = false;
                    for c in network_manager
                        .active_connections()
                        .await
                        .unwrap_or_default()
                    {
                        if c.uuid().await.unwrap_or_default().as_str() == uuid.as_ref()
                            && network_manager.deactivate_connection(&c).await.is_ok()
                        {
                            success = true;
                            if let Ok(ActiveConnectionState::Deactivated) = c.state().await {
                                break;
                            } else {
                                let mut changed = c.receive_state_changed().await;
                                _ = tokio::time::timeout(Duration::from_secs(5), async move {
                                    loop {
                                        if let Some(next) = changed.next().await {
                                            if let Ok(ActiveConnectionState::Deactivated) =
                                                next.get().await.map(ActiveConnectionState::from)
                                            {
                                                break;
                                            }
                                        }
                                    }
                                })
                                .await;
                            }
                            break;
                        }
                    }

                    _ = request_response(&conn, Request::Deactivate(uuid.clone()), success)
                        .then(|event| output.send(event))
                        .await;
                }

                Some(Request::Disconnect(ssid)) => {
                    let mut success = false;
                    for c in network_manager
                        .active_connections()
                        .await
                        .unwrap_or_default()
                    {
                        if c.id().await.unwrap_or_default().as_str() == ssid.as_ref()
                            && network_manager.deactivate_connection(&c).await.is_ok()
                        {
                            success = true;
                            if let Ok(ActiveConnectionState::Deactivated) = c.state().await {
                                break;
                            } else {
                                let mut changed = c.receive_state_changed().await;
                                _ = tokio::time::timeout(Duration::from_secs(5), async move {
                                    loop {
                                        if let Some(next) = changed.next().await {
                                            if let Ok(ActiveConnectionState::Deactivated) =
                                                next.get().await.map(ActiveConnectionState::from)
                                            {
                                                break;
                                            }
                                        }
                                    }
                                })
                                .await;
                            }
                            break;
                        }
                    }

                    _ = request_response(&conn, Request::Disconnect(ssid.clone()), success)
                        .then(|event| output.send(event))
                        .await;
                }

                Some(Request::SetAirplaneMode(airplane_mode)) => {
                    // wifi
                    let mut success = network_manager
                        .set_wireless_enabled(!airplane_mode)
                        .await
                        .is_ok();
                    // bluetooth
                    success = success
                        && Command::new("rfkill")
                            .arg(if airplane_mode { "block" } else { "unblock" })
                            .arg("bluetooth")
                            .output()
                            .await
                            .is_ok();

                    let mut state = NetworkManagerState::new(&conn).await.unwrap_or_default();
                    state.airplane_mode = if success {
                        airplane_mode
                    } else {
                        !airplane_mode
                    };
                    if state.airplane_mode {
                        state.wifi_enabled = false;
                    }

                    _ = output
                        .send(Event::RequestResponse {
                            req: Request::SetAirplaneMode(airplane_mode),
                            success,
                            state,
                        })
                        .await;
                }

                Some(Request::SetWiFi(enabled)) => {
                    let success = network_manager.set_wireless_enabled(enabled).await.is_ok();

                    let mut state = NetworkManagerState::new(&conn).await.unwrap_or_default();

                    state.wifi_enabled = if success { enabled } else { !enabled };

                    if state.wifi_enabled {
                        tokio::time::sleep(Duration::from_secs(3)).await;
                    }

                    _ = request_response(&conn, Request::SetWiFi(enabled), success)
                        .then(|event| output.send(event))
                        .await;
                }

                Some(Request::Authenticate {
                    ssid,
                    identity,
                    password,
                    hw_address,
                }) => {
                    let nm_state = NetworkManagerState::new(&conn).await.unwrap_or_default();
                    let success = nm_state
                        .connect_wifi(
                            &conn,
                            &ssid,
                            identity.as_deref(),
                            Some(password.unsecure()),
                            hw_address,
                        )
                        .await
                        .is_ok();

                    _ = output
                        .send(Event::RequestResponse {
                            req: Request::Authenticate {
                                ssid: ssid.clone(),
                                identity: identity.clone(),
                                password: password.clone(),
                                hw_address,
                            },
                            success,
                            state: NetworkManagerState::new(&conn).await.unwrap_or_default(),
                        })
                        .await;
                }

                Some(Request::SelectAccessPoint(ssid, hw_address, network_type)) => {
                    if matches!(network_type, NetworkType::Open) {
                        attempt_wifi_connection(&conn, ssid, hw_address, network_type, output)
                            .await;
                    } else {
                        // For secured networks, check if we have saved credentials
                        if !has_saved_wifi_credentials(&conn, &ssid).await {
                            return State::Waiting(conn, rx);
                        }

                        // We have saved credentials, attempt connection
                        attempt_wifi_connection(&conn, ssid, hw_address, network_type, output)
                            .await;
                    }
                }

                Some(Request::Activate(device_path, connection_path)) => {
                    let mut success = true;

                    if let Err(why) = network_manager
                        .activate_connection_by_paths(&connection_path, &device_path)
                        .await
                    {
                        tracing::error!(
                            ?why,
                            "failed to activate connection on {device_path:?} to {connection_path}"
                        );
                        success = false;
                    };

                    _ = request_response(
                        &conn,
                        Request::Activate(device_path, connection_path),
                        success,
                    )
                    .then(|event| output.send(event))
                    .await;
                }

                Some(Request::Reload) => {
                    _ = output
                        .send(request_response(&conn, Request::Reload, true).await)
                        .await;
                }

                Some(Request::Remove(uuid)) => {
                    let s = match NetworkManagerSettings::new(&conn).await {
                        Ok(s) => s,
                        Err(why) => {
                            tracing::error!(?why, "error getting network manager settings");
                            _ = output
                                .send(Event::RequestResponse {
                                    req: Request::Forget(uuid.clone()),
                                    success: false,
                                    state: NetworkManagerState::new(&conn)
                                        .await
                                        .unwrap_or_default(),
                                })
                                .await;

                            return State::Waiting(conn, rx);
                        }
                    };

                    let known_conns = s.list_connections().await.unwrap_or_default();
                    let mut success = false;
                    for c in known_conns {
                        let settings = c.get_settings().await.ok().unwrap_or_default();

                        let c_uuid = settings
                            .get("connection")
                            .and_then(|conn| conn.get("uuid"))
                            .and_then(|uuid| uuid.downcast_ref::<String>().ok())
                            .unwrap_or_default();

                        if uuid.as_ref() == c_uuid.as_str() {
                            _ = c.delete().await;
                            success = true;
                        }
                    }

                    _ = request_response(&conn, Request::Remove(uuid.clone()), success)
                        .then(|event| output.send(event))
                        .await;
                }

                Some(Request::Forget(ssid)) => {
                    let s = match NetworkManagerSettings::new(&conn).await {
                        Ok(s) => s,
                        Err(why) => {
                            tracing::error!(?why, "error getting network manager settings");
                            _ = output
                                .send(Event::RequestResponse {
                                    req: Request::Forget(ssid.clone()),
                                    success: false,
                                    state: NetworkManagerState::new(&conn)
                                        .await
                                        .unwrap_or_default(),
                                })
                                .await;

                            return State::Waiting(conn, rx);
                        }
                    };

                    let known_conns = s.list_connections().await.unwrap_or_default();
                    let mut success = false;
                    for c in known_conns {
                        let settings = c.get_settings().await.ok().unwrap_or_default();
                        let s = Settings::new(settings);
                        if s.wifi
                            .clone()
                            .and_then(|w| w.ssid)
                            .and_then(|ssid| String::from_utf8(ssid).ok())
                            .is_some_and(|s| s == ssid.as_ref())
                        {
                            _ = c.delete().await;
                            success = true;
                            break;
                        }
                    }

                    _ = request_response(&conn, Request::Forget(ssid.clone()), success)
                        .then(|event| output.send(event))
                        .await;
                }

                None => {
                    return State::Finished;
                }
            };

            State::Waiting(conn, rx)
        }
        State::Finished => futures::future::pending().await,
    }
}

async fn request_response(conn: &zbus::Connection, req: Request, success: bool) -> Event {
    Event::RequestResponse {
        req,
        success,
        state: NetworkManagerState::new(conn).await.unwrap_or_default(),
    }
}

async fn has_saved_wifi_credentials(conn: &zbus::Connection, ssid: &str) -> bool {
    let Ok(nm_settings) = NetworkManagerSettings::new(conn).await else {
        return false;
    };

    let known_conns = nm_settings.list_connections().await.unwrap_or_default();

    for connection in known_conns {
        if let Ok(settings) = connection.get_settings().await {
            let settings = Settings::new(settings);
            if let Some(saved_ssid) = settings
                .wifi
                .and_then(|w| w.ssid)
                .and_then(|ssid| String::from_utf8(ssid).ok())
            {
                if saved_ssid == ssid {
                    return true;
                }
            }
        }
    }

    false
}

async fn attempt_wifi_connection(
    conn: &zbus::Connection,
    ssid: SSID,
    hw_address: HwAddress,
    network_type: NetworkType,
    output: &mut futures::channel::mpsc::Sender<Event>,
) {
    let state = NetworkManagerState::new(conn).await.unwrap_or_default();

    let success = if let Err(err) = state
        .connect_wifi(conn, ssid.as_ref(), None, None, hw_address)
        .await
    {
        tracing::error!("Failed to connect to access point: {:?}", err);
        false
    } else {
        true
    };

    _ = request_response(
        conn,
        Request::SelectAccessPoint(ssid, hw_address, network_type),
        success,
    )
    .then(|event| output.send(event))
    .await;
}

#[derive(Debug, Clone)]
pub enum Request {
    /// Activate a device's connection profile
    Activate(ObjectPath<'static>, ObjectPath<'static>),
    /// Deactivate a connection
    Deactivate(UUID),
    /// Disconnect from an access point.
    Disconnect(SSID),
    /// Forget a known access point.
    Forget(SSID),
    /// Create a connection to a new access point.
    Authenticate {
        ssid: String,
        identity: Option<String>,
        password: SecureString,
        hw_address: HwAddress,
    },
    /// Signal to reload the service.
    Reload,
    /// Remove a connection profile.
    Remove(UUID),
    /// Connect to a known access point.
    SelectAccessPoint(SSID, HwAddress, NetworkType),
    /// Toggle airplaine mode.
    SetAirplaneMode(bool),
    /// Toggle WiFi enablement.
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
        conn: zbus::Connection,
        sender: UnboundedSender<Request>,
        state: NetworkManagerState,
    },
    Devices,
    WiFiEnabled(bool),
    WirelessAccessPoints,
    ActiveConns,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkManagerState {
    pub wireless_access_points: Vec<AccessPoint>,
    pub active_conns: Vec<ActiveConnectionInfo>,
    pub known_access_points: Vec<AccessPoint>,
    pub wifi_enabled: bool,
    pub airplane_mode: bool,
    pub connectivity: NmConnectivityState,
}

impl Default for NetworkManagerState {
    fn default() -> Self {
        Self {
            wireless_access_points: Vec::new(),
            active_conns: Vec::new(),
            known_access_points: Vec::new(),
            wifi_enabled: false,
            airplane_mode: false,
            connectivity: NmConnectivityState::Unknown,
        }
    }
}

impl NetworkManagerState {
    pub async fn new(conn: &zbus::Connection) -> Result<Self, Error> {
        let network_manager = NetworkManager::new(conn).await?;
        let mut this = Self::default();

        this.refresh_wifi_state(conn, &network_manager).await?;

        Ok(this)
    }

    pub async fn refresh_wifi_state(
        &mut self,
        conn: &zbus::Connection,
        network_manager: &NetworkManager<'_>,
    ) -> Result<(), Error> {
        let (airplane_mode, wireless_enabled, settings_res) = futures::join!(
            Command::new("rfkill")
                .arg("list")
                .arg("bluetooth")
                .output()
                .then(|res| async move {
                    let Ok(output) = res else {
                        return false;
                    };

                    std::str::from_utf8(&output.stdout)
                        .ok()
                        .is_some_and(|stdout| stdout.contains("Soft blocked: yes"))
                }),
            network_manager
                .wireless_enabled()
                .then(|res| async move { res.unwrap_or_default() }),
            NetworkManagerSettings::new(conn)
        );

        self.wifi_enabled = wireless_enabled;
        self.airplane_mode = airplane_mode && !self.wifi_enabled;

        let settings = settings_res?;

        _ = settings.load_connections(&[]).await;

        let (known_conns, active_conns, devices, connectivity) = futures::join!(
            settings.list_connections(),
            network_manager.active_connections(),
            network_manager.devices(),
            network_manager.connectivity(),
        );

        let devices = devices.unwrap_or_default();
        let known_conns = known_conns.unwrap_or_default();

        let (active_conns, wireless_access_points) = futures::join!(
            // Retrieve active connections.
            async move {
                let mut active_conns = active_connections(active_conns.unwrap_or_default())
                    .await
                    .unwrap_or_default();

                active_conns.sort_by(|a, b| {
                    let helper = |conn: &ActiveConnectionInfo| match conn {
                        ActiveConnectionInfo::Vpn { name, .. } => format!("0{name}"),
                        ActiveConnectionInfo::Wired { name, .. } => format!("1{name}"),
                        ActiveConnectionInfo::WiFi { name, .. } => format!("2{name}"),
                    };
                    helper(a).cmp(&helper(b))
                });

                active_conns
            },
            // Retrieve all access points, and sort by strength.
            async move {
                let mut wireless_access_points = futures::stream::FuturesUnordered::from_iter(
                    devices.iter().map(|device| async move {
                        if let Ok(Some(SpecificDevice::Wireless(wireless_device))) =
                            device.downcast_to_device().await
                        {
                            handle_wireless_device(wireless_device, device.hw_address().await.ok())
                                .await
                                .unwrap_or_default()
                        } else {
                            Vec::new()
                        }
                    }),
                )
                .fold(
                    Vec::with_capacity(devices.len()),
                    |mut access_points, mut f| async move {
                        access_points.append(&mut f);
                        access_points
                    },
                )
                .await;

                wireless_access_points.sort_by(|a, b| b.strength.cmp(&a.strength));
                wireless_access_points
            }
        );

        // Concurrently get
        let known_ssid: Vec<Arc<str>> = futures::stream::FuturesOrdered::from_iter(
            known_conns.into_iter().map(|c| async move {
                let s = c.get_settings().await.ok()?;
                let s = Settings::new(s);
                let curr_ssid = s
                    .wifi
                    .clone()
                    .and_then(|w| w.ssid)
                    .and_then(|ssid| String::from_utf8(ssid).ok())?;

                Some(Arc::from(curr_ssid))
            }),
        )
        .filter_map(|c| async move { c })
        .collect()
        .await;

        self.known_access_points = wireless_access_points
            .iter()
            .filter(|a| {
                known_ssid.contains(&a.ssid)
                    && !active_conns.iter().any(|ac| ac.name() == a.ssid.as_ref())
            })
            .cloned()
            .collect();

        self.wireless_access_points = wireless_access_points;
        self.active_conns = active_conns;
        self.connectivity = connectivity?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.active_conns = Vec::new();
        self.known_access_points = Vec::new();
        self.wireless_access_points = Vec::new();
    }

    async fn connect_wifi(
        &self,
        conn: &zbus::Connection,
        ssid: &str,
        identity: Option<&str>,
        password: Option<&str>,
        hw_address: HwAddress,
    ) -> Result<(), Error> {
        let nm = NetworkManager::new(conn).await?;

        for c in nm.active_connections().await.unwrap_or_default() {
            if self
                .wireless_access_points
                .iter()
                .any(|w| Ok(Some(w.ssid.as_ref())) == c.cached_id().as_ref().map(|v| v.as_deref()))
            {
                _ = nm.deactivate_connection(&c).await;
                break;
            }
        }

        let Some(ap) = self
            .wireless_access_points
            .iter()
            .find(|ap| ap.ssid.as_ref() == ssid && ap.hw_address == hw_address)
        else {
            return Err(Error::AccessPointNotFound);
        };

        let mut conn_settings: HashMap<&str, HashMap<&str, zvariant::Value>> = HashMap::from([
            (
                "802-11-wireless",
                HashMap::from([("ssid", Value::Array(ssid.as_bytes().into()))]),
            ),
            (
                "connection",
                HashMap::from([
                    ("id", Value::Str(ssid.into())),
                    ("type", Value::Str("802-11-wireless".into())),
                ]),
            ),
        ]);
        if let Some(identity) = identity {
            conn_settings.insert(
                "802-1x",
                HashMap::from([
                    ("identity", Value::Str(identity.into())),
                    // most common default
                    ("eap", Value::Array(vec!["peap"].into())),
                    // most common default
                    ("phase2-auth", Value::Str("mschapv2".into())),
                    ("password", Value::Str(password.unwrap_or("").into())),
                ]),
            );
            let wireless = conn_settings.get_mut("802-11-wireless").unwrap();
            wireless.insert("security", Value::Str("802-11-wireless-security".into()));
            wireless.insert("mode", Value::Str("infrastructure".into()));
            conn_settings.insert(
                "802-11-wireless-security",
                HashMap::from([("key-mgmt", Value::Str("wpa-eap".into()))]),
            );
        } else if let Some(pass) = password {
            conn_settings.insert(
                "802-11-wireless-security",
                HashMap::from([
                    ("psk", Value::Str(pass.into())),
                    ("key-mgmt", Value::Str("wpa-psk".into())),
                ]),
            );
        }

        let devices = nm.devices().await?;
        for device in devices {
            if !matches!(
                device.device_type().await.unwrap_or(DeviceType::Other),
                DeviceType::Wifi
            ) {
                continue;
            }

            let s = NetworkManagerSettings::new(conn).await?;
            let known_conns = s.list_connections().await.unwrap_or_default();
            let mut known_conn = None;
            for c in known_conns {
                let settings = c.get_settings().await.ok().unwrap_or_default();

                let s = Settings::new(settings);
                if let Some(cur_ssid) = s
                    .wifi
                    .clone()
                    .and_then(|w| w.ssid)
                    .and_then(|ssid| String::from_utf8(ssid).ok())
                {
                    if cur_ssid == ssid {
                        known_conn = Some(c);
                        break;
                    }
                }
            }

            let active_conn = if let Some(known_conn) = known_conn.as_ref() {
                // update settings if needed
                if password.is_some() {
                    known_conn.update(conn_settings).await?;
                }

                nm.activate_connection(known_conn, &device).await?
            } else {
                let (_, active_conn) = nm
                    .add_and_activate_connection(conn_settings, device.inner().path(), &ap.path)
                    .await?;
                let dummy = ActiveConnectionProxy::new(conn, active_conn).await?;
                let active = ActiveConnectionProxy::builder(conn)
                    .destination(dummy.inner().destination().to_owned())
                    .unwrap()
                    .interface(dummy.inner().interface().to_owned())
                    .unwrap()
                    .path(dummy.inner().path().to_owned())
                    .unwrap()
                    .build()
                    .await
                    .unwrap();
                ActiveConnection::from(active)
            };
            let mut changes = active_conn.receive_state_changed().await;
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let mut count = 5;
            loop {
                let state = active_conn.state().await;
                if let Ok(enums::ActiveConnectionState::Activated) = state {
                    return Ok(());
                } else if let Ok(enums::ActiveConnectionState::Deactivated) = state {
                    return Err(Error::ConnectionActivate);
                }
                if let Ok(Some(s)) =
                    tokio::time::timeout(Duration::from_secs(20), changes.next()).await
                {
                    let state = s.get().await.unwrap_or_default().into();
                    if matches!(state, enums::ActiveConnectionState::Activated) {
                        return Ok(());
                    }
                };

                count -= 1;
                if count <= 0 {
                    return Err(Error::ConnectionActivate);
                }
            }
        }

        Err(Error::NoWiFiDevices)
    }
}
