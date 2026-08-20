// SPDX-FileCopyrightText: 2026 erik-balfe
// SPDX-License-Identifier: GPL-3.0-only
//
// Mobile data page (Slice A): toggle, dual-SIM slot recovery, status.

use std::sync::Arc;

use anyhow::Context;
 
use cosmic::widget::{self, icon};
use cosmic::widget::space::horizontal as horizontal_space;
use cosmic::{Apply, Element, Task};
use cosmic_settings_page::{self as page, Section, section};
use futures::{SinkExt, StreamExt};
use zbus::zvariant::{ObjectPath, OwnedObjectPath};

use super::backend as network_manager;
use super::backend::NetworkManagerState;
use super::backend::current_networks::ActiveConnectionInfo;
use super::backend::devices::DeviceState;
use super::nm_edit_connection;

pub type ConnectionId = Arc<str>;

#[derive(Clone, Debug)]
pub enum Message {
    Activate,
    Deactivate,
    CancelDialog,
    Error(String),
    NetworkManager(network_manager::Event),
    NetworkManagerConnect(nmrs::NetworkManager),
    Refresh,
    RemoveProfileRequest(ConnectionId),
    RemoveProfile(ConnectionId),
    SelectDevice(Arc<network_manager::devices::DeviceInfo>),
    Settings(ConnectionId),
    SetPrimarySimSlot(u32),
    SlotReady(ModemSnapshot),
    UpdateState(NetworkManagerState),
    UpdateDevices(Vec<network_manager::devices::DeviceInfo>),
    UpdateModem(ModemSnapshot),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Mobile(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Mobile(message)
    }
}

#[derive(Clone, Debug, Default)]
pub struct ModemSnapshot {
    pub present: bool,
    pub path: String,
    pub state_failed_reason: String,
    pub operator: String,
    pub access_tech: String,
    pub signal_percent: Option<u32>,
    pub imei: String,
    pub model: String,
    pub primary_sim_slot: u32,
    pub slots_with_sim: Vec<u32>,
    pub active_slot_has_sim: bool,
    pub active_sim_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum MobileDialog {
    RemoveProfile(ConnectionId),
}

#[derive(Debug, Default)]
pub struct Page {
    entity: page::Entity,
    nm_task: Option<tokio::sync::oneshot::Sender<()>>,
    nm_state: Option<NmState>,
    dialog: Option<MobileDialog>,
    active_device: Option<Arc<network_manager::devices::DeviceInfo>>,
    modem: ModemSnapshot,
    busy: bool,
    status_message: Option<String>,
    /// First NM device list received (may be empty — still counts).
    devices_loaded: bool,
    /// First ModemManager snapshot received.
    modem_loaded: bool,
}

#[derive(Debug)]
pub struct NmState {
    conn: nmrs::NetworkManager,
    sender: futures::channel::mpsc::UnboundedSender<network_manager::Request>,
    active_conns: Vec<ActiveConnectionInfo>,
    devices: Vec<Arc<network_manager::devices::DeviceInfo>>,
    airplane_mode: bool,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> cosmic_settings_page::Info {
        page::Info::new("mobile", "network-cellular-symbolic")
            .title(fl!("mobile-data"))
            .description(fl!("xdg-entry-mobile-comment"))
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(mobile_view())])
    }

    fn dialog(&'_ self) -> Option<Element<'_, crate::pages::Message>> {
        self.dialog.as_ref().map(|dialog| match dialog {
            MobileDialog::RemoveProfile(uuid) => {
                let primary_action = widget::button::destructive(fl!("remove"))
                    .on_press(Message::RemoveProfile(uuid.clone()));
                let secondary_action =
                    widget::button::standard(fl!("cancel")).on_press(Message::CancelDialog);

                widget::dialog()
                    .title(fl!("remove-connection-dialog"))
                    .icon(icon::from_name("dialog-information").size(64))
                    .body(fl!("remove-connection-dialog", "mobile-description"))
                    .primary_action(primary_action)
                    .secondary_action(secondary_action)
                    .apply(Element::from)
                    .map(crate::pages::Message::Mobile)
            }
        })
    }

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        if self.nm_task.is_none() {
            return cosmic::Task::future(async move {
                nmrs::NetworkManager::new()
                    .await
                    .context("failed to connect to NetworkManager")
                    .map_or_else(
                        |why| Message::Error(why.to_string()),
                        Message::NetworkManagerConnect,
                    )
                    .apply(crate::pages::Message::Mobile)
            });
        }

        Task::none()
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        // Keep modem snapshot + ready flags so re-open does not flash
        // "No modem" → real content. Drop live NM sender only.
        self.nm_state = None;
        self.busy = false;
        self.status_message = None;

        if let Some(cancel) = self.nm_task.take() {
            _ = cancel.send(());
        }

        Task::none()
    }
}

impl Page {
    fn page_ready(&self) -> bool {
        self.devices_loaded && self.modem_loaded
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::NetworkManager(network_manager::Event::RequestResponse {
                req,
                state,
                success,
            }) => {
                if !success {
                    tracing::error!(request = ?req, "network-manager request failed");
                    self.status_message = Some(fl!("mobile-data", "activate-failed"));
                }
                self.busy = false;
                if let Some(NmState { ref conn, .. }) = self.nm_state {
                    let conn = conn.clone();
                    self.update_active_conns(state);
                    return cosmic::Task::batch(vec![
                        update_devices(conn.clone()),
                        refresh_modem(),
                    ]);
                }
            }

            Message::UpdateDevices(devices) => {
                self.update_devices(devices);
                self.devices_loaded = true;
            }

            Message::UpdateState(state) => {
                self.update_active_conns(state);
            }

            Message::UpdateModem(modem) => {
                self.modem = modem;
                self.modem_loaded = true;
                self.busy = false;
            }

            Message::SelectDevice(device) => {
                self.active_device = Some(device);
            }

            Message::NetworkManager(
                network_manager::Event::ActiveConns | network_manager::Event::Devices,
            ) => {
                if let Some(NmState { ref conn, .. }) = self.nm_state {
                    return cosmic::Task::batch(vec![
                        update_state(conn.clone()),
                        update_devices(conn.clone()),
                        refresh_modem(),
                    ]);
                }
            }

            Message::NetworkManager(network_manager::Event::Init {
                conn,
                sender,
                state,
            }) => {
                self.nm_state = Some(NmState {
                    conn: conn.clone(),
                    sender,
                    devices: Vec::new(),
                    airplane_mode: state.airplane_mode,
                    active_conns: state
                        .active_conns
                        .into_iter()
                        .filter(|info| matches!(info, ActiveConnectionInfo::Mobile { .. }))
                        .collect(),
                });

                return cosmic::Task::batch(vec![update_devices(conn), refresh_modem()]);
            }

            Message::NetworkManager(_event) => {}

            Message::Activate => {
                return self.activate_mobile();
            }

            Message::Deactivate => {
                if let Some(uuid) = self.active_mobile_uuid() {
                    self.busy = true;
                    if let Some(NmState { ref sender, .. }) = self.nm_state {
                        _ = sender.unbounded_send(network_manager::Request::Deactivate(uuid));
                    }
                }
            }

            Message::RemoveProfileRequest(uuid) => {
                self.dialog = Some(MobileDialog::RemoveProfile(uuid));
            }

            Message::RemoveProfile(uuid) => {
                self.dialog = None;
                if let Some(NmState { ref sender, .. }) = self.nm_state {
                    _ = sender.unbounded_send(network_manager::Request::Remove(uuid));
                }
                if let Some(NmState { ref conn, .. }) = self.nm_state {
                    return cosmic::Task::batch(vec![
                        update_state(conn.clone()),
                        update_devices(conn.clone()),
                    ]);
                }
            }

            Message::Settings(uuid) => {
                return cosmic::task::future(async move {
                    _ = nm_edit_connection(uuid.as_ref()).await;
                    Message::Refresh
                });
            }

            Message::SetPrimarySimSlot(slot) => {
                self.busy = true;
                self.status_message = Some(fl!("mobile-data", "switching-sim"));
                let modem_path = self.modem.path.clone();
                let conn = self.nm_state.as_ref().map(|s| s.conn.clone());
                return cosmic::task::future(async move {
                    match set_primary_sim_slot(&modem_path, slot).await {
                        Ok(()) => {
                            for _ in 0..30 {
                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                let snap = modem_snapshot().await.unwrap_or_default();
                                let nm_ready = if let Some(ref nm) = conn {
                                    modem_nm_ready(nm).await
                                } else {
                                    snap.active_slot_has_sim
                                };
                                if snap.active_slot_has_sim && nm_ready {
                                    return Message::SlotReady(snap);
                                }
                            }
                            match modem_snapshot().await {
                                Ok(s) if s.active_slot_has_sim => Message::SlotReady(s),
                                Ok(s) => Message::UpdateModem(s),
                                Err(why) => Message::Error(why),
                            }
                        }
                        Err(why) => Message::Error(why),
                    }
                });
            }

            Message::SlotReady(snap) => {
                self.modem = snap;
                self.busy = false;
                self.status_message = Some(fl!("mobile-data", "slot-ready"));
                return self.activate_mobile();
            }

            Message::Refresh => {
                if let Some(NmState { ref conn, .. }) = self.nm_state {
                    return cosmic::Task::batch(vec![
                        update_state(conn.clone()),
                        update_devices(conn.clone()),
                        refresh_modem(),
                    ]);
                }
            }

            Message::CancelDialog => {
                self.dialog = None;
            }

            Message::Error(why) => {
                tracing::error!(why);
                self.busy = false;
                self.status_message = Some(why);
                // Don't trap the page on an infinite loader if MM/NM fails once.
                self.devices_loaded = true;
                self.modem_loaded = true;
            }

            Message::NetworkManagerConnect(conn) => {
                return self.connect(conn);
            }
        }

        Task::none()
    }

    fn connect(&mut self, conn: nmrs::NetworkManager) -> Task<crate::app::Message> {
        if self.nm_task.is_none() {
            let (canceller, task) =
                crate::utils::forward_event_loop(move |mut sender| async move {
                    let (tx, mut rx) = futures::channel::mpsc::channel(1);

                    let watchers = std::pin::pin!(async move {
                        network_manager::watch(conn, tx).await;
                    });

                    let forwarder = std::pin::pin!(async move {
                        while let Some(message) = rx.next().await {
                            _ = sender
                                .send(crate::pages::Message::Mobile(Message::NetworkManager(
                                    message,
                                )))
                                .await;
                        }
                    });

                    futures::future::select(watchers, forwarder).await;
                });

            self.nm_task = Some(canceller);
            return task.map(crate::app::Message::from);
        }

        Task::none()
    }

    fn update_devices(&mut self, devices: Vec<network_manager::devices::DeviceInfo>) {
        if let Some(ref mut nm_state) = self.nm_state {
            let devices: Vec<_> = devices.into_iter().map(Arc::new).collect();
            nm_state.devices = devices;
            if self.active_device.is_none() {
                if let Some(first) = nm_state.devices.first() {
                    self.active_device = Some(Arc::clone(first));
                }
            } else if let Some(active) = self.active_device.as_ref() {
                self.active_device = nm_state
                    .devices
                    .iter()
                    .find(|d| d.path == active.path)
                    .map(Arc::clone);
            }
        }
    }

    fn update_active_conns(&mut self, state: NetworkManagerState) {
        if let Some(ref mut nm_state) = self.nm_state {
            nm_state.airplane_mode = state.airplane_mode;
            nm_state.active_conns = state
                .active_conns
                .into_iter()
                .filter(|info| matches!(info, ActiveConnectionInfo::Mobile { .. }))
                .collect();
        }
    }

    fn is_connected(&self) -> bool {
        self.nm_state
            .as_ref()
            .is_some_and(|s| !s.active_conns.is_empty())
            || self
                .active_device
                .as_ref()
                .and_then(|d| d.active_connection.as_ref())
                .is_some_and(|(_, st)| {
                    matches!(
                        st,
                        network_manager::devices::ActiveConnectionState::Activated
                            | network_manager::devices::ActiveConnectionState::Activating
                    )
                })
    }

    fn active_mobile_uuid(&self) -> Option<ConnectionId> {
        let device = self.active_device.as_ref()?;
        device
            .active_connection
            .as_ref()
            .map(|(c, _)| c.uuid.clone())
    }

    fn preferred_profile(&self) -> Option<&network_manager::devices::KnownDeviceConnection> {
        let device = self.active_device.as_ref()?;
        device
            .known_connections
            .iter()
            .find(|c| c.id == "mobile")
            .or_else(|| device.known_connections.first())
    }

    fn activate_mobile(&mut self) -> Task<crate::app::Message> {
        let Some(nm_state) = self.nm_state.as_ref() else {
            return Task::none();
        };
        let Some(device) = self.active_device.as_ref() else {
            self.status_message = Some(fl!("mobile-data", "no-modem"));
            return Task::none();
        };

        if nm_state.airplane_mode {
            self.status_message = Some(fl!("mobile-data", "airplane"));
            return Task::none();
        }

        if !self.modem.active_slot_has_sim {
            if let Some(slot) = self
                .modem
                .slots_with_sim
                .iter()
                .copied()
                .find(|s| *s != self.modem.primary_sim_slot)
            {
                self.status_message = Some(fl!("mobile-data", "sim-other-slot", slot = slot));
            } else {
                self.status_message = Some(fl!("mobile-data", "no-sim"));
            }
            return Task::none();
        }

        if matches!(
            device.state,
            DeviceState::Unavailable | DeviceState::Unmanaged
        ) {
            self.status_message = Some(fl!("mobile-data", "device-unavailable"));
            return Task::none();
        }

        let preferred = self.preferred_profile();
        let preferred_uuid = preferred.map(|p| p.uuid.clone());
        let preferred_id = preferred.map(|p| p.id.clone());

        if let Some(conn) = device.available_connections.iter().find(|c| {
            preferred_uuid
                .as_ref()
                .is_some_and(|u| c.uuid.as_ref() == u.as_ref())
                || preferred_id.as_ref().is_some_and(|id| c.id == *id)
                || c.id == "mobile"
        }) {
            self.busy = true;
            _ = nm_state
                .sender
                .unbounded_send(network_manager::Request::Activate(
                    device.path.clone(),
                    conn.path.clone(),
                ));
            return Task::none();
        }

        self.status_message = Some(fl!("mobile-data", "no-profile"));
        Task::none()
    }
}

fn mobile_view() -> Section<crate::pages::Message> {
    Section::default().view::<Page>(move |_binder, page, _section| {
        let spacing = cosmic::theme::spacing();

        // Gate content until first NM + MM snapshots — no "no modem" flash then jump.
        if !page.page_ready() {
            return widget::column::with_capacity(1)
                .push(widget::settings::section().add(widget::settings::item_row(vec![
                    widget::text::body(fl!("mobile-data", "loading")).into(),
                ])))
                .spacing(spacing.space_m)
                .apply(Element::from)
                .map(crate::pages::Message::Mobile);
        }

        let mut column = widget::column::with_capacity(8).spacing(spacing.space_m);

        if let Some(msg) = page.status_message.as_ref() {
            column = column.push(widget::text::body(msg.clone()));
        }

        // MM dual-SIM / no-SIM banners are independent of whether NM currently
        // lists a gsm device (Unavailable while empty eSIM is primary).
        if page.nm_state.as_ref().is_some_and(|s| s.airplane_mode) {
            column = column.push(banner(fl!("mobile-data", "airplane")));
        } else if !page.modem.present
            && page.nm_state.as_ref().is_none_or(|s| s.devices.is_empty())
        {
            column = column.push(banner(fl!("mobile-data", "no-modem")));
        } else if page.modem.present && !page.modem.active_slot_has_sim {
            if let Some(slot) = page
                .modem
                .slots_with_sim
                .iter()
                .copied()
                .find(|s| *s != page.modem.primary_sim_slot)
            {
                let hint = if page.modem.state_failed_reason == "esim-without-profiles" {
                    fl!("mobile-data", "esim-empty-use-physical", slot = slot)
                } else {
                    fl!("mobile-data", "sim-other-slot", slot = slot)
                };
                column = column.push(
                    widget::settings::section().add(widget::settings::item_row(vec![
                        widget::text::body(hint).into(),
                        horizontal_space().into(),
                        widget::button::suggested(fl!("mobile-data", "use-slot", slot = slot))
                            .on_press_maybe(
                                (!page.busy).then_some(Message::SetPrimarySimSlot(slot)),
                            )
                            .into(),
                    ])),
                );
            } else {
                column = column.push(banner(fl!("mobile-data", "no-sim")));
            }
        }

        let connected = page.is_connected();
        let can_toggle = page.nm_state.as_ref().is_some_and(|s| !s.devices.is_empty())
            && page.modem.active_slot_has_sim
            && !page.nm_state.as_ref().is_some_and(|s| s.airplane_mode)
            && !page.busy;

        let toggle = {
            let t = widget::toggler(connected);
            if can_toggle {
                t.on_toggle(|enable: bool| {
                    if enable {
                        Message::Activate
                    } else {
                        Message::Deactivate
                    }
                })
            } else {
                t
            }
        };

        let status_label = if page.busy {
            fl!("mobile-data", "working")
        } else if connected {
            fl!("connected")
        } else {
            fl!("network-device-state", "disconnected")
        };

        column = column.push(widget::settings::section().add(widget::settings::item_row(vec![
            widget::column::with_capacity(2)
                .push(widget::text::body(fl!("mobile-data")))
                .push(widget::text::caption(status_label))
                .into(),
            horizontal_space().into(),
            toggle.into(),
        ])));

        // Modem details
        if page.modem.present {
            let mut details = widget::settings::section().title(fl!("mobile-data", "details"));
            if !page.modem.operator.is_empty() {
                details = details.add(widget::settings::item(
                    fl!("mobile-data", "operator"),
                    widget::text::body(page.modem.operator.clone()),
                ));
            }
            if !page.modem.access_tech.is_empty() {
                details = details.add(widget::settings::item(
                    fl!("mobile-data", "technology"),
                    widget::text::body(page.modem.access_tech.clone()),
                ));
            }
            if let Some(sig) = page.modem.signal_percent {
                details = details.add(widget::settings::item(
                    fl!("mobile-data", "signal"),
                    widget::text::body(format!("{sig}%")),
                ));
            }
            if !page.modem.active_sim_label.is_empty() {
                details = details.add(widget::settings::item(
                    fl!("mobile-data", "sim"),
                    widget::text::body(page.modem.active_sim_label.clone()),
                ));
            }
            if !page.modem.imei.is_empty() {
                details = details.add(widget::settings::item(
                    fl!("mobile-data", "imei"),
                    widget::text::body(page.modem.imei.clone()),
                ));
            }
            if !page.modem.model.is_empty() {
                details = details.add(widget::settings::item(
                    fl!("mobile-data", "model"),
                    widget::text::body(page.modem.model.clone()),
                ));
            }
            column = column.push(details);
        }

        // Profile actions
        if let Some(profile) = page.preferred_profile() {
            let uuid = profile.uuid.clone();
            column = column.push(
                widget::settings::section()
                    .title(fl!("mobile-data", "profile"))
                    .add(widget::settings::item(
                        profile.id.clone(),
                        widget::row::with_capacity(2)
                            .spacing(spacing.space_s)
                            .push(
                                widget::button::standard(fl!("settings"))
                                    .on_press(Message::Settings(uuid.clone())),
                            )
                            .push(
                                widget::button::destructive(fl!("remove"))
                                    .on_press(Message::RemoveProfileRequest(uuid)),
                            ),
                    )),
            );
        }

        column
            .apply(Element::from)
            .map(crate::pages::Message::Mobile)
    })
}

fn banner(text: String) -> Element<'static, Message> {
    widget::settings::section()
        .add(widget::text::body(text))
        .into()
}

fn update_state(conn: nmrs::NetworkManager) -> Task<crate::app::Message> {
    cosmic::task::future(async move {
        match NetworkManagerState::new(&conn).await {
            Ok(state) => Message::UpdateState(state),
            Err(why) => Message::Error(why.to_string()),
        }
    })
}

fn update_devices(conn: nmrs::NetworkManager) -> Task<crate::app::Message> {
    cosmic::task::future(async move {
        let filter =
            |device_type| matches!(device_type, network_manager::devices::DeviceType::Modem);

        match network_manager::devices::list(&conn, filter).await {
            Ok(devices) => Message::UpdateDevices(devices),
            Err(why) => Message::Error(why.to_string()),
        }
    })
}

fn refresh_modem() -> Task<crate::app::Message> {
    cosmic::task::future(async move {
        match modem_snapshot().await {
            Ok(s) => Message::UpdateModem(s),
            Err(why) => Message::Error(why),
        }
    })
}

async fn modem_nm_ready(nm: &nmrs::NetworkManager) -> bool {
    let filter =
        |device_type| matches!(device_type, network_manager::devices::DeviceType::Modem);
    match network_manager::devices::list(nm, filter).await {
        Ok(devices) => devices.iter().any(|d| {
            !matches!(
                d.state,
                DeviceState::Unavailable | DeviceState::Unmanaged | DeviceState::Unknown
            )
        }),
        Err(_) => false,
    }
}

async fn set_primary_sim_slot(modem_path: &str, slot: u32) -> Result<(), String> {
    if modem_path.is_empty() {
        return Err(fl!("mobile-data", "no-modem"));
    }

    let conn = zbus::Connection::system()
        .await
        .map_err(|e| e.to_string())?;
    let path = ObjectPath::try_from(modem_path.to_string()).map_err(|e| e.to_string())?;
    let proxy = zbus::Proxy::new(
        &conn,
        "org.freedesktop.ModemManager1",
        path,
        "org.freedesktop.ModemManager1.Modem",
    )
    .await
    .map_err(|e| e.to_string())?;

    match proxy.call_method("SetPrimarySimSlot", &(slot,)).await {
        Ok(_) => Ok(()),
        Err(e) => {
            let raw = e.to_string();
            let lower = raw.to_ascii_lowercase();
            if lower.contains("access denied")
                || lower.contains("not authorized")
                || lower.contains("denied")
            {
                Err(fl!("mobile-data", "slot-access-denied"))
            } else {
                Err(raw)
            }
        }
    }
}

async fn modem_snapshot() -> Result<ModemSnapshot, String> {
    use zbus::fdo::ObjectManagerProxy;
    use zbus::Proxy;

    let conn = zbus::Connection::system()
        .await
        .map_err(|e| e.to_string())?;
    let om = ObjectManagerProxy::builder(&conn)
        .destination("org.freedesktop.ModemManager1")
        .map_err(|e| e.to_string())?
        .path("/org/freedesktop/ModemManager1")
        .map_err(|e| e.to_string())?
        .build()
        .await
        .map_err(|e| e.to_string())?;

    let managed = om.get_managed_objects().await.map_err(|e| e.to_string())?;
    let mut snap = ModemSnapshot::default();

    // Quectel/MBIM often only exports the Modem node on ObjectManager — SIM
    // objects are reachable by path but not listed. Always probe SimSlots paths.
    async fn sim_usable_label(conn: &zbus::Connection, path_str: &str) -> (bool, String) {
        let Ok(path) = ObjectPath::try_from(path_str.to_string()) else {
            return (false, String::new());
        };
        let Ok(proxy) = Proxy::new(
            conn,
            "org.freedesktop.ModemManager1",
            path,
            "org.freedesktop.ModemManager1.Sim",
        )
        .await
        else {
            return (false, String::new());
        };

        let sim_type = proxy
            .get_property::<u32>("SimType")
            .await
            .unwrap_or(0);
        let eid = proxy
            .get_property::<String>("Eid")
            .await
            .unwrap_or_default();
        let imsi = proxy
            .get_property::<String>("Imsi")
            .await
            .unwrap_or_default();
        let iccid = proxy
            .get_property::<String>("SimIdentifier")
            .await
            .unwrap_or_default();
        let op = proxy
            .get_property::<String>("OperatorName")
            .await
            .unwrap_or_default();

        // MMSimType: 1=physical, 2=esim
        let is_esim = sim_type == 2 || !eid.is_empty();
        let has_identity = !imsi.is_empty() || !iccid.is_empty();
        let usable = if is_esim { has_identity } else { true };
        let kind = if is_esim { "eSIM" } else { "physical" };
        let label = if !op.is_empty() {
            format!("{op} · {kind}")
        } else if !imsi.is_empty() {
            format!("{imsi} · {kind}")
        } else {
            kind.to_string()
        };
        (usable, label)
    }

    for (path, ifaces) in &managed {
        let Some(modem) = ifaces.get("org.freedesktop.ModemManager1.Modem") else {
            continue;
        };
        snap.present = true;
        snap.path = path.to_string();

        if let Some(v) = modem.get("Model") {
            snap.model = owned_to_string(v);
        }
        if let Some(v) = modem.get("EquipmentIdentifier") {
            snap.imei = owned_to_string(v);
        }
        if let Some(v) = modem.get("PrimarySimSlot") {
            snap.primary_sim_slot = owned_to_u32(v);
        }
        if let Some(v) = modem.get("StateFailedReason") {
            let code = owned_to_u32(v);
            snap.state_failed_reason = match code {
                0 => String::new(),
                2 => "sim-missing".into(),
                5 => "esim-without-profiles".into(),
                n => n.to_string(),
            };
        }
        if let Some(v) = modem.get("SignalQuality") {
            if let Ok((pct, _recent)) = <(u32, bool)>::try_from(v.clone()) {
                snap.signal_percent = Some(pct);
            }
        }
        if let Some(v) = modem.get("AccessTechnologies") {
            snap.access_tech = access_tech_label(owned_to_u32(v));
        }

        let mut slot_paths: Vec<(u32, String)> = Vec::new();
        if let Some(v) = modem.get("SimSlots") {
            if let Ok(slots) = <Vec<OwnedObjectPath>>::try_from(v.clone()) {
                for (idx, slot_path) in slots.iter().enumerate() {
                    let slot_n = (idx as u32) + 1;
                    let path_str = slot_path.as_str().to_string();
                    if !path_str.is_empty() && path_str != "/" {
                        slot_paths.push((slot_n, path_str));
                    }
                }
            }
        }

        for (slot_n, path_str) in &slot_paths {
            let (usable, label) = sim_usable_label(&conn, path_str).await;
            if usable {
                snap.slots_with_sim.push(*slot_n);
            }
            if *slot_n == snap.primary_sim_slot {
                snap.active_sim_label = label;
                snap.active_slot_has_sim = usable;
            }
        }

        if let Some(v) = modem.get("Sim") {
            let sim_path = owned_to_string(v);
            if !sim_path.is_empty() && sim_path != "/" {
                let (usable, label) = sim_usable_label(&conn, &sim_path).await;
                if !label.is_empty() {
                    snap.active_sim_label = label;
                }
                if usable {
                    snap.active_slot_has_sim = true;
                }
            }
        }

        // Failed reasons that mean the *active* slot cannot do data.
        if snap.state_failed_reason == "esim-without-profiles"
            || snap.state_failed_reason == "sim-missing"
        {
            snap.active_slot_has_sim = false;
        }

        if let Some(gpp) = ifaces.get("org.freedesktop.ModemManager1.Modem.Modem3gpp") {
            if let Some(v) = gpp.get("OperatorName") {
                let op = owned_to_string(v);
                if !op.is_empty() {
                    snap.operator = op;
                }
            }
        }
        if snap.operator.is_empty() && !snap.active_sim_label.is_empty() {
            snap.operator = snap
                .active_sim_label
                .split(" · ")
                .next()
                .unwrap_or("")
                .to_string();
        }
        break;
    }

    Ok(snap)
}

fn owned_to_string(v: &zbus::zvariant::OwnedValue) -> String {
    if let Ok(s) = <&str>::try_from(&**v) {
        return s.to_string();
    }
    if let Ok(s) = String::try_from(v.clone()) {
        return s;
    }
    if let Ok(u) = u32::try_from(v.clone()) {
        return u.to_string();
    }
    if let Ok(p) = <ObjectPath<'_>>::try_from(&**v) {
        return p.as_str().to_string();
    }
    if let Ok(p) = OwnedObjectPath::try_from(v.clone()) {
        return p.as_str().to_string();
    }
    String::new()
}

fn owned_to_u32(v: &zbus::zvariant::OwnedValue) -> u32 {
    u32::try_from(v.clone()).unwrap_or(0)
}

fn access_tech_label(bits: u32) -> String {
    const LTE: u32 = 1 << 14;
    const HSPA: u32 = 1 << 9;
    const UMTS: u32 = 1 << 5;
    if bits & LTE != 0 {
        "LTE".into()
    } else if bits & HSPA != 0 {
        "HSPA".into()
    } else if bits & UMTS != 0 {
        "3G".into()
    } else if bits == 0 {
        String::new()
    } else {
        format!("0x{bits:x}")
    }
}
