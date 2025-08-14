// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod nmcli;

use std::sync::Arc;

use anyhow::Context;
use cosmic::dialog::file_chooser::FileFilter;
use cosmic::{
    Apply, Element, Task,
    iced::{Alignment, Length},
    iced_core::text::Wrapping,
    widget::{self, icon},
};
use cosmic_settings_page::{self as page, Section, section};
use cosmic_settings_subscriptions::network_manager::{
    self, NetworkManagerState, UUID, current_networks::ActiveConnectionInfo,
};
use futures::{FutureExt, StreamExt};
use indexmap::IndexMap;
use secure_string::SecureString;

pub type ConnectionId = Arc<str>;
pub type InterfaceId = String;

#[derive(Clone, Debug)]
pub enum Message {
    /// Activate a connection
    Activate(ConnectionId),
    /// Add a network connection
    AddNetwork,
    /// Show a dialog requesting a name for the WireGuard device
    AddWireGuardDevice(String, String, String),
    /// Cancels an active dialog.
    CancelDialog,
    /// Connect to a VPN with the given username and password
    ConnectWithPassword,
    /// Deactivate a connection.
    Deactivate(ConnectionId),
    /// An error occurred.
    Error(ErrorKind, String),
    /// VPN connection error.
    VpnDialogError(VpnDialog),
    /// Update the list of known connections.
    KnownConnections(IndexMap<UUID, ConnectionSettings>),
    /// An update from the network manager daemon
    NetworkManager(network_manager::Event),
    /// Successfully connected to the system dbus.
    NetworkManagerConnect(zbus::Connection),
    /// Updates the password text input
    PasswordUpdate(SecureString),
    /// Refresh devices and their connection profiles
    Refresh,
    /// Create a dialog to ask for confirmation of removal.
    RemoveProfileRequest(ConnectionId),
    /// Remove a connection profile
    RemoveProfile(ConnectionId),
    /// Opens settings page for the access point.
    Settings(ConnectionId),
    /// Toggles visibility of password input.
    TogglePasswordVisibility,
    /// Update NetworkManagerState
    UpdateState(NetworkManagerState),
    /// Update the devices lists
    UpdateDevices(Vec<network_manager::devices::DeviceInfo>),
    /// Updates the username text input
    UsernameUpdate(String),
    /// Display more options for an access point
    ViewMore(Option<ConnectionId>),
    /// Create a new wireguard connection
    WireGuardConfig,
    /// Update the text input for the wireguard device name
    WireGuardDeviceInput(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Config,
    Connect,
    ConnectionEditor,
    ConnectionSettings,
    DbusConnection,
    UpdatingState,
    WireGuardConfigPath,
    WireGuardDevice,
    WithPassword(&'static str),
}

impl ErrorKind {
    pub fn localized(self) -> String {
        match self {
            ErrorKind::Config => fl!("vpn-error", "config"),
            ErrorKind::Connect => fl!("vpn-error", "connect"),
            ErrorKind::ConnectionEditor => fl!("vpn-error", "connection-editor"),
            ErrorKind::ConnectionSettings => fl!("vpn-error", "connection-settings"),
            ErrorKind::DbusConnection => fl!("dbus-connection-error"),
            ErrorKind::UpdatingState => fl!("vpn-error", "updating-state"),
            ErrorKind::WireGuardConfigPath => fl!("vpn-error", "wireguard-config-path"),
            ErrorKind::WireGuardDevice => fl!("vpn-error", "wireguard-device"),
            ErrorKind::WithPassword(field) => fl!("vpn-error", "with-password", field = field),
        }
    }
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Vpn(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Vpn(message)
    }
}

#[derive(Clone, Debug)]
pub enum ConnectionSettings {
    Vpn(VpnConnectionSettings),
    Wireguard { id: String },
}

#[derive(Clone, Debug, Default)]
pub struct VpnConnectionSettings {
    id: String,
    username: Option<String>,
    connection_type: Option<ConnectionType>,
    password_flag: Option<PasswordFlag>,
}

impl VpnConnectionSettings {
    fn password_flag(&self) -> Option<PasswordFlag> {
        self.connection_type
            .as_ref()
            .map_or(false, |ct| match ct {
                ConnectionType::Password => true,
            })
            .then_some(self.password_flag)
            .flatten()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ConnectionType {
    Password,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PasswordFlag {
    /// The system is responsible for providing and storing this secret.
    None = 0,
    /// A user-session secret agent is responsible for providing and storing
    /// this secret; when it is required, agents will be asked to provide it.
    AgentOwned = 1,
    /// This secret should not be saved but should be requested from the user
    /// each time it is required. This flag should be used for One-Time-Pad
    /// secrets, PIN codes from hardware tokens, or if the user simply does not
    /// want to save the secret.
    NotSaved = 2,
    /// in some situations it cannot be automatically determined that a secret is required or not. This flag hints that the secret is not required and should not be requested from the user.
    NotRequired = 4,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VpnDialog {
    Error(ErrorKind, String),
    Password {
        id: String,
        uuid: Arc<str>,
        username: String,
        password: SecureString,
        password_hidden: bool,
        error: Option<(ErrorKind, String)>,
    },
    RemoveProfile(ConnectionId),
    WireGuardName(String, String, String),
}

#[derive(Debug)]
pub struct NmState {
    conn: zbus::Connection,
    sender: futures::channel::mpsc::UnboundedSender<network_manager::Request>,
    active_conns: Vec<ActiveConnectionInfo>,
    devices: Vec<network_manager::devices::DeviceInfo>,
}

#[derive(Debug, Default)]
pub struct Page {
    entity: page::Entity,
    nm_task: Option<tokio::sync::oneshot::Sender<()>>,
    nm_state: Option<NmState>,
    dialog: Option<VpnDialog>,
    view_more_popup: Option<ConnectionId>,
    known_connections: IndexMap<UUID, ConnectionSettings>,
    /// Withhold device update if the view more popup is shown.
    withheld_devices: Option<Vec<network_manager::devices::DeviceInfo>>,
    /// Withhold active connections update if the view more popup is shown.
    withheld_active_conns: Option<Vec<ActiveConnectionInfo>>,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> cosmic_settings_page::Info {
        page::Info::new("vpn", "preferences-vpn-symbolic")
            .title(fl!("vpn"))
            .description(fl!("connections-and-profiles", variant = "vpn"))
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(devices_view())])
    }

    fn dialog(&self) -> Option<Element<crate::pages::Message>> {
        self.dialog.as_ref().map(|dialog| match dialog {
            VpnDialog::Error(error_kind, message) => {
                let reason = widget::text::body(message.as_str()).wrapping(Wrapping::Word);

                let primary_action =
                    widget::button::standard(fl!("ok")).on_press(Message::CancelDialog);

                widget::dialog()
                    .title(fl!("vpn-error"))
                    .icon(icon::from_name("dialog-error-symbolic").size(64))
                    .body(error_kind.localized())
                    .control(reason)
                    .primary_action(primary_action)
                    .apply(Element::from)
                    .map(crate::pages::Message::Vpn)
            }

            VpnDialog::Password {
                username,
                password,
                password_hidden,
                error,
                ..
            } => {
                let username = widget::text_input(fl!("username"), username.as_str())
                    .on_input(Message::UsernameUpdate);

                let password = widget::text_input::secure_input(
                    fl!("password"),
                    password.unsecure(),
                    Some(Message::TogglePasswordVisibility),
                    *password_hidden,
                )
                .on_input(|input| Message::PasswordUpdate(SecureString::from(input)))
                .on_submit(|_| Message::ConnectWithPassword);
                let (err_kind, error) = if let Some(err) = error.as_ref() {
                    (
                        Some(err.0),
                        Some(widget::text::body(err.1.as_str()).wrapping(Wrapping::Word)),
                    )
                } else {
                    (None, None)
                };

                let controls = widget::column::with_capacity(2)
                    .spacing(12)
                    .push(username)
                    .push(password)
                    .push_maybe(error)
                    .apply(Element::from);

                let primary_action = widget::button::suggested(fl!("connect"))
                    .on_press(Message::ConnectWithPassword);

                let secondary_action =
                    widget::button::standard(fl!("cancel")).on_press(Message::CancelDialog);

                widget::dialog()
                    .title(if let Some(error_kind) = err_kind {
                        error_kind.localized()
                    } else {
                        fl!("auth-dialog")
                    })
                    .icon(icon::from_name("network-vpn-symbolic").size(64))
                    .body(fl!("auth-dialog", "vpn-description"))
                    .control(controls)
                    .primary_action(primary_action)
                    .secondary_action(secondary_action)
                    .apply(Element::from)
                    .map(crate::pages::Message::Vpn)
            }

            VpnDialog::WireGuardName(device, ..) => {
                let input = widget::text_input("", device.as_str()).on_input(|input| {
                    Message::WireGuardDeviceInput(input.replace(|c: char| !c.is_alphanumeric(), ""))
                });

                let primary_action =
                    widget::button::suggested(fl!("connect")).on_press(Message::WireGuardConfig);

                let secondary_action =
                    widget::button::standard(fl!("cancel")).on_press(Message::CancelDialog);

                widget::dialog()
                    .title(fl!("wireguard-dialog"))
                    .icon(icon::from_name("network-vpn-symbolic").size(64))
                    .body(fl!("wireguard-dialog", "description"))
                    .control(input)
                    .primary_action(primary_action)
                    .secondary_action(secondary_action)
                    .apply(Element::from)
                    .map(crate::pages::Message::Vpn)
            }

            VpnDialog::RemoveProfile(uuid) => {
                let primary_action = widget::button::destructive(fl!("remove"))
                    .on_press(Message::RemoveProfile(uuid.clone()));

                let secondary_action =
                    widget::button::standard(fl!("cancel")).on_press(Message::CancelDialog);

                widget::dialog()
                    .title(fl!("remove-connection-dialog"))
                    .icon(icon::from_name("dialog-information").size(64))
                    .body(fl!("remove-connection-dialog", "vpn-description"))
                    .primary_action(primary_action)
                    .secondary_action(secondary_action)
                    .apply(Element::from)
                    .map(crate::pages::Message::Vpn)
            }
        })
    }

    fn header_view(&self) -> Option<Element<'_, crate::pages::Message>> {
        Some(
            widget::button::standard(fl!("add-network"))
                .trailing_icon(icon::from_name("window-pop-out-symbolic"))
                .on_press(Message::AddNetwork)
                .apply(widget::container)
                .width(Length::Fill)
                .align_x(Alignment::End)
                .apply(Element::from)
                .map(crate::pages::Message::Vpn),
        )
    }

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        if self.nm_task.is_none() {
            return cosmic::task::future(async move {
                zbus::Connection::system()
                    .await
                    .context("failed to create system dbus connection")
                    .map_or_else(
                        |why| Message::Error(ErrorKind::DbusConnection, why.to_string()),
                        Message::NetworkManagerConnect,
                    )
            });
        }

        Task::none()
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        self.view_more_popup = None;
        self.nm_state = None;
        self.withheld_active_conns = None;
        self.withheld_devices = None;
        self.dialog = None;

        if let Some(cancel) = self.nm_task.take() {
            _ = cancel.send(());
        }

        Task::none()
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        let span = tracing::span!(tracing::Level::INFO, "vpn::update");
        let _span = span.enter();

        match message {
            Message::NetworkManager(network_manager::Event::RequestResponse {
                req,
                state,
                success,
            }) => {
                if !success {
                    tracing::error!(request = ?req, "network-manager request failed");
                }

                if let Some(NmState { ref conn, .. }) = self.nm_state {
                    let conn = conn.clone();
                    self.update_active_conns(state);
                    return cosmic::Task::batch(vec![
                        connection_settings(conn.clone()),
                        update_devices(conn),
                    ]);
                }
            }

            Message::KnownConnections(connections) => {
                self.known_connections = connections;
            }

            Message::UpdateDevices(devices) => {
                self.update_devices(devices);
            }

            Message::UpdateState(state) => {
                self.update_active_conns(state);
            }

            Message::NetworkManager(
                network_manager::Event::ActiveConns | network_manager::Event::Devices,
            ) => {
                if let Some(NmState { ref conn, .. }) = self.nm_state {
                    return cosmic::Task::batch(vec![
                        update_state(conn.clone()),
                        update_devices(conn.clone()),
                        connection_settings(conn.clone()),
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
                    active_conns: state
                        .active_conns
                        .into_iter()
                        .filter(|info| matches!(info, ActiveConnectionInfo::Vpn { .. }))
                        .collect(),
                });

                return cosmic::Task::batch(vec![
                    connection_settings(conn.clone()),
                    update_devices(conn),
                ]);
            }

            Message::NetworkManager(_event) => (),

            Message::AddNetwork => return add_network(),

            Message::AddWireGuardDevice(device, filename, path) => {
                self.dialog = Some(VpnDialog::WireGuardName(device, filename, path));
            }

            Message::WireGuardDeviceInput(input) => {
                if let Some(VpnDialog::WireGuardName(ref mut device, ..)) = self.dialog {
                    *device = input
                }
            }

            Message::WireGuardConfig => {
                if let Some(VpnDialog::WireGuardName(device, filename, path)) = self.dialog.take() {
                    return cosmic::task::future(async move {
                        let new_path = path.replace(&filename, &device);
                        _ = std::fs::rename(&path, &new_path);
                        match super::nm_add_vpn_file("wireguard", new_path).await {
                            Ok(_) => Message::Refresh,
                            Err(why) => Message::Error(ErrorKind::Config, why.to_string()),
                        }
                    });
                }
            }

            Message::Activate(uuid) => {
                self.close_popup_and_apply_updates();

                if let Some(settings) = self.known_connections.get(&uuid) {
                    let settings = match settings {
                        ConnectionSettings::Vpn(settings) => settings,
                        ConnectionSettings::Wireguard { id } => {
                            let connection_name = id.clone();
                            return cosmic::task::future(async move {
                                if let Err(why) = nmcli::connect(&connection_name).await {
                                    return Message::Error(
                                        ErrorKind::Connect,
                                        format!("failed to connect to WireGuard VPN: {why}"),
                                    );
                                }

                                Message::Refresh
                            });
                        }
                    };

                    match settings.password_flag() {
                        Some(PasswordFlag::NotSaved | PasswordFlag::AgentOwned) => {
                            self.view_more_popup = None;
                            self.dialog = Some(VpnDialog::Password {
                                id: settings.id.clone(),
                                uuid: uuid.clone(),
                                username: settings.username.clone().unwrap_or_default(),
                                password: SecureString::from(""),
                                password_hidden: true,
                                error: None,
                            });
                        }

                        _ => {
                            let connection_name = settings.id.clone();
                            let username = settings.username.clone();
                            return cosmic::task::future(async move {
                                if let Err(why) = nmcli::connect(&connection_name).await {
                                    return Message::VpnDialogError(VpnDialog::Password {
                                        error: Some((
                                            ErrorKind::Connect,
                                            format!("failed to connect to VPN: {why}"),
                                        )),
                                        id: connection_name.clone(),
                                        uuid,
                                        username: username.clone().unwrap_or_default(),
                                        password: SecureString::from(""),
                                        password_hidden: true,
                                    });
                                }

                                Message::Refresh
                            });
                        }
                    }
                }
            }

            Message::Deactivate(uuid) => {
                self.close_popup_and_apply_updates();
                if let Some(NmState { ref sender, .. }) = self.nm_state {
                    _ = sender.unbounded_send(network_manager::Request::Deactivate(uuid));
                }
            }

            Message::RemoveProfileRequest(uuid) => {
                self.view_more_popup = None;
                self.dialog = Some(VpnDialog::RemoveProfile(uuid));
            }

            Message::RemoveProfile(uuid) => {
                self.dialog = None;
                self.close_popup_and_apply_updates();
                if let Some(NmState { ref sender, .. }) = self.nm_state {
                    _ = sender.unbounded_send(network_manager::Request::Remove(uuid));
                }
            }

            Message::ViewMore(uuid) => {
                self.view_more_popup = uuid;
                if self.view_more_popup.is_none() {
                    self.close_popup_and_apply_updates();
                }
            }
            Message::Settings(uuid) => {
                self.close_popup_and_apply_updates();

                return cosmic::task::future(async move {
                    super::nm_edit_connection(uuid.as_ref())
                        .then(|res| async move {
                            match res {
                                Ok(_) => Message::Refresh,
                                Err(why) => {
                                    Message::Error(ErrorKind::ConnectionEditor, why.to_string())
                                }
                            }
                        })
                        .await
                });
            }

            Message::Refresh => {
                if let Some(NmState { ref conn, .. }) = self.nm_state {
                    return cosmic::Task::batch(vec![
                        update_state(conn.clone()),
                        update_devices(conn.clone()),
                        connection_settings(conn.clone()),
                    ]);
                }
            }

            Message::PasswordUpdate(pass) => {
                if let Some(VpnDialog::Password {
                    ref mut password, ..
                }) = self.dialog
                {
                    *password = pass;
                }
            }

            Message::ConnectWithPassword => {
                let Some(dialog) = self.dialog.take() else {
                    return Task::none();
                };

                if let VpnDialog::Password {
                    id,
                    uuid,
                    username,
                    password,
                    ..
                } = dialog
                {
                    return self
                        .activate_with_password(id, uuid, username, password)
                        .map(crate::app::Message::from);
                }
            }

            Message::UsernameUpdate(user) => {
                if let Some(VpnDialog::Password {
                    ref mut username, ..
                }) = self.dialog
                {
                    *username = user;
                }
            }
            Message::CancelDialog => {
                self.dialog = None;
            }
            Message::TogglePasswordVisibility => {
                if let Some(VpnDialog::Password {
                    ref mut password_hidden,
                    ..
                }) = self.dialog
                {
                    *password_hidden = !*password_hidden;
                }
            }

            Message::Error(error_kind, why) => {
                tracing::error!(?error_kind, why);
                self.dialog = Some(VpnDialog::Error(error_kind, why))
            }

            Message::NetworkManagerConnect(conn) => {
                return self.connect(conn.clone());
            }

            Message::VpnDialogError(vpn_dialog) => {
                self.dialog = Some(vpn_dialog);
            }
        }

        Task::none()
    }

    fn activate_with_password(
        &mut self,
        connection_name: String,
        uuid: Arc<str>,
        username: String,
        password: SecureString,
    ) -> Task<Message> {
        cosmic::task::future(async move {
            if let Err(why) = nmcli::set_username(&connection_name, &username).await {
                return Message::VpnDialogError(VpnDialog::Password {
                    error: Some((ErrorKind::WithPassword("username"), why.to_string())),
                    id: connection_name.clone(),
                    uuid,
                    username,
                    password,
                    password_hidden: true,
                });
            }

            if let Err(why) = nmcli::set_password_flags_none(&connection_name).await {
                return Message::VpnDialogError(VpnDialog::Password {
                    error: Some((ErrorKind::WithPassword("password-flags"), why.to_string())),
                    id: connection_name.clone(),
                    uuid,
                    username,
                    password,
                    password_hidden: true,
                });
            }

            if let Err(why) = nmcli::add_fallback(&connection_name).await {
                return Message::VpnDialogError(VpnDialog::Password {
                    error: Some((ErrorKind::Config, why.to_string())),
                    id: connection_name.clone(),
                    uuid,
                    username,
                    password,
                    password_hidden: true,
                });
            }

            if let Err(why) = nmcli::set_password_flags_none(&connection_name).await {
                return Message::VpnDialogError(VpnDialog::Password {
                    error: Some((ErrorKind::WithPassword("password-flags"), why.to_string())),
                    id: connection_name.clone(),
                    uuid,
                    username,
                    password,
                    password_hidden: true,
                });
            }

            if let Err(why) = nmcli::set_password(&connection_name, password.unsecure()).await {
                return Message::VpnDialogError(VpnDialog::Password {
                    error: Some((ErrorKind::WithPassword("password"), why.to_string())),
                    id: connection_name.clone(),
                    uuid,
                    username,
                    password,
                    password_hidden: true,
                });
            }

            if let Err(why) = nmcli::connect(&connection_name).await {
                return Message::VpnDialogError(VpnDialog::Password {
                    error: Some((ErrorKind::Connect, why.to_string())),
                    id: connection_name.clone(),
                    uuid,
                    username,
                    password,
                    password_hidden: true,
                });
            }

            Message::Refresh
        })
    }

    fn connect(&mut self, conn: zbus::Connection) -> Task<crate::app::Message> {
        if self.nm_task.is_none() {
            let (canceller, task) = crate::utils::forward_event_loop(move |emitter| async move {
                let (tx, mut rx) = futures::channel::mpsc::channel(1);

                let watchers = std::pin::pin!(async move {
                    futures::join!(
                        network_manager::watch(conn.clone(), tx.clone()),
                        network_manager::active_conns::watch(conn.clone(), tx.clone()),
                        network_manager::devices::watch(conn, true, tx)
                    )
                });

                let forwarder = std::pin::pin!(async move {
                    while let Some(message) = rx.next().await {
                        _ = emitter
                            .emit(crate::pages::Message::Vpn(Message::NetworkManager(message)))
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

    /// Closes the view more popup and applies any withheld updates.
    fn close_popup_and_apply_updates(&mut self) {
        self.view_more_popup = None;
        if let Some(ref mut nm_state) = self.nm_state {
            if let Some(active_conns) = self.withheld_active_conns.take() {
                nm_state.active_conns = active_conns;
            }

            if let Some(devices) = self.withheld_devices.take() {
                nm_state.devices = devices;
            }
        }
    }

    /// Withholds updates if the view more popup is displayed.
    fn update_devices(&mut self, devices: Vec<network_manager::devices::DeviceInfo>) {
        if let Some(ref mut nm_state) = self.nm_state {
            if self.view_more_popup.is_some() {
                self.withheld_devices = Some(devices);
            } else {
                nm_state.devices = devices;
            }
        }
    }

    /// Withholds updates if the view more popup is displayed.
    fn update_active_conns(&mut self, state: NetworkManagerState) {
        if let Some(ref mut nm_state) = self.nm_state {
            let conns = state
                .active_conns
                .into_iter()
                .filter(|info| matches!(info, ActiveConnectionInfo::Vpn { .. }))
                .collect();

            if self.view_more_popup.is_some() {
                self.withheld_active_conns = Some(conns);
            } else {
                nm_state.active_conns = conns;
            }
        }
    }
}

fn devices_view() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        vpn_conns_txt = fl!("vpn", "connections");
        remove_txt = fl!("vpn", "remove");
        connect_txt = fl!("connect");
        connected_txt = fl!("connected");
        settings_txt = fl!("settings");
        disconnect_txt = fl!("disconnect");
    });

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let Some(NmState {
                ref active_conns, ..
            }) = page.nm_state
            else {
                return cosmic::widget::column().into();
            };

            let theme = cosmic::theme::active();
            let spacing = &theme.cosmic().spacing;

            let mut view = widget::column::with_capacity(4);

            let vpn_connections =
                widget::settings::section().title(&section.descriptions[vpn_conns_txt]);

            if page.known_connections.is_empty() {
                view = view.push(vpn_connections.add(widget::settings::item_row(vec![
                    widget::text::body(fl!("no-vpn")).into(),
                ])));
            } else {
                let known_networks = page.known_connections.iter().fold(
                    vpn_connections,
                    |networks, (uuid, connection)| {
                        let id = match connection {
                            ConnectionSettings::Vpn(connection) => connection.id.as_str(),
                            ConnectionSettings::Wireguard { id } => id.as_str(),
                        };

                        let is_connected = active_conns.iter().any(|conn| match conn {
                            ActiveConnectionInfo::Vpn { name, .. } => name.as_str() == id,

                            _ => false,
                        });

                        let (connect_txt, connect_msg) = if is_connected {
                            (&section.descriptions[connected_txt], None)
                        } else {
                            (
                                &section.descriptions[connect_txt],
                                Some(Message::Activate(uuid.clone())),
                            )
                        };

                        let identifier = widget::text::body(id).wrapping(Wrapping::Glyph);

                        let connect: Element<'_, Message> = if let Some(msg) = connect_msg {
                            widget::button::text(connect_txt).on_press(msg).into()
                        } else {
                            widget::text::body(connect_txt)
                                .align_y(Alignment::Center)
                                .into()
                        };

                        let view_more_button =
                            widget::button::icon(widget::icon::from_name("view-more-symbolic"));

                        let view_more: Option<Element<_>> = if page
                            .view_more_popup
                            .as_deref()
                            .map_or(false, |id| id == uuid.as_ref())
                        {
                            widget::popover(view_more_button.on_press(Message::ViewMore(None)))
                                .position(widget::popover::Position::Bottom)
                                .on_close(Message::ViewMore(None))
                                .popup({
                                    widget::column()
                                        .push_maybe(is_connected.then(|| {
                                            popup_button(
                                                Message::Deactivate(uuid.clone()),
                                                &section.descriptions[disconnect_txt],
                                            )
                                        }))
                                        .push(popup_button(
                                            Message::Settings(uuid.clone()),
                                            &section.descriptions[settings_txt],
                                        ))
                                        .push(popup_button(
                                            Message::RemoveProfileRequest(uuid.clone()),
                                            &section.descriptions[remove_txt],
                                        ))
                                        .width(Length::Fixed(200.0))
                                        .apply(widget::container)
                                        .padding(1)
                                        .class(cosmic::style::Container::Dropdown)
                                })
                                .apply(|e| Some(Element::from(e)))
                        } else {
                            view_more_button
                                .on_press(Message::ViewMore(Some(uuid.clone())))
                                .apply(|e| Some(Element::from(e)))
                        };

                        let controls = widget::row::with_capacity(2)
                            .push(connect)
                            .push_maybe(view_more)
                            .align_y(Alignment::Center)
                            .spacing(spacing.space_xxs);

                        let widget = widget::settings::item_row(vec![
                            identifier.into(),
                            widget::horizontal_space().into(),
                            controls.into(),
                        ]);

                        networks.add(widget)
                    },
                );

                view = view.push(known_networks);
            }

            view.spacing(spacing.space_l)
                .apply(Element::from)
                .map(crate::pages::Message::Vpn)
        })
}

fn popup_button(message: Message, text: &str) -> Element<'_, Message> {
    let theme = cosmic::theme::active();
    let theme = theme.cosmic();
    widget::text::body(text)
        .align_y(Alignment::Center)
        .apply(widget::button::custom)
        .padding([theme.space_xxxs(), theme.space_xs()])
        .width(Length::Fill)
        .class(cosmic::theme::Button::MenuItem)
        .on_press(message)
        .into()
}

fn update_state(conn: zbus::Connection) -> Task<crate::app::Message> {
    cosmic::task::future(async move {
        match NetworkManagerState::new(&conn).await {
            Ok(state) => Message::UpdateState(state),
            Err(why) => Message::Error(ErrorKind::UpdatingState, why.to_string()),
        }
    })
}

fn update_devices(conn: zbus::Connection) -> Task<crate::app::Message> {
    cosmic::task::future(async move {
        let filter =
            |device_type| matches!(device_type, network_manager::devices::DeviceType::WireGuard);

        match network_manager::devices::list(&conn, filter).await {
            Ok(devices) => Message::UpdateDevices(devices),
            Err(why) => Message::Error(ErrorKind::UpdatingState, why.to_string()),
        }
    })
}

fn add_network() -> Task<crate::app::Message> {
    let Some(dir) = dirs::download_dir().or_else(dirs::home_dir) else {
        return Task::none();
    };

    cosmic::dialog::file_chooser::open::Dialog::new()
        .directory(dir)
        .title(fl!("vpn", "select-file"))
        .filter(
            FileFilter::new("OpenVPN")
                .mimetype("application/x-openvpn-profile")
                .glob("*.ovpn"),
        )
        .filter(FileFilter::new("WireGuard").glob("*.conf*"))
        .open_file()
        .then(|result| async move {
            match result {
                Ok(response) => {
                    let response_str = response.url().as_str();
                    let result = if let Some(device) = response_str.strip_suffix(".conf") {
                        let Ok(path) = response.url().to_file_path() else {
                            return Message::Error(
                                ErrorKind::WireGuardConfigPath,
                                fl!("vpn-error", "wireguard-config-path-desc"),
                            );
                        };

                        let path = path.to_string_lossy().to_string();

                        let filename = device.rsplit_once("/").unwrap_or_default().1;

                        let mut device = filename
                            .replace(|c: char| !c.is_alphanumeric(), "")
                            .to_ascii_lowercase();

                        device.truncate(15);

                        return Message::AddWireGuardDevice(device, filename.to_owned(), path);
                    } else {
                        super::nm_add_vpn_file("openvpn", response.url().to_file_path().unwrap())
                            .await
                    };

                    match result {
                        Ok(_) => Message::Refresh,
                        Err(why) => Message::Error(ErrorKind::Config, why.to_string()),
                    }
                }
                Err(cosmic::dialog::file_chooser::Error::Cancelled) => {
                    return Message::CancelDialog;
                }
                Err(why) => {
                    return Message::Error(ErrorKind::Config, why.to_string());
                }
            }
        })
        .apply(cosmic::task::future)
}

fn connection_settings(conn: zbus::Connection) -> Task<crate::app::Message> {
    let settings = async move {
        let settings = network_manager::dbus::settings::NetworkManagerSettings::new(&conn).await?;

        _ = settings.load_connections(&[]).await;

        let settings = settings
            // Get a list of known connections.
            .list_connections()
            .await?
            // Prepare for wrapping in a concurrent stream.
            .into_iter()
            .map(|conn| async move { conn })
            // Create a concurrent stream for each connection.
            .apply(futures::stream::FuturesOrdered::from_iter)
            // Concurrently fetch settings for each connection, and filter for VPN.
            .filter_map(|conn| async move {
                let settings = conn.get_settings().await.ok()?;

                let connection = settings.get("connection")?;

                match connection
                    .get("type")?
                    .downcast_ref::<String>()
                    .ok()?
                    .as_str()
                {
                    "vpn" => (),

                    "wireguard" => {
                        let id = connection.get("id")?.downcast_ref::<String>().ok()?;
                        let uuid = connection.get("uuid")?.downcast_ref::<String>().ok()?;
                        return Some((Arc::from(uuid), ConnectionSettings::Wireguard { id }));
                    }

                    _ => return None,
                }

                let vpn = settings.get("vpn")?;
                let id = connection.get("id")?.downcast_ref::<String>().ok()?;
                let uuid = connection.get("uuid")?.downcast_ref::<String>().ok()?;

                let (username, connection_type, password_flag) = vpn
                    .get("data")
                    .and_then(|data| data.downcast_ref::<zbus::zvariant::Dict>().ok())
                    .map(|dict| {
                        let (mut connection_type, mut password_flag) = (None, None);

                        let username = dict
                            .get::<String, String>(&String::from("username"))
                            .ok()
                            .flatten()
                            .filter(|value| !value.is_empty());

                        if let Some("password") = dict
                            .get::<String, String>(&String::from("connection-type"))
                            .ok()
                            .flatten()
                            .as_deref()
                        {
                            connection_type = Some(ConnectionType::Password);

                            password_flag = dict
                                .get::<String, String>(&String::from("password-flags"))
                                .ok()
                                .flatten()
                                .and_then(|value| match value.as_str() {
                                    "0" => Some(PasswordFlag::None),
                                    "1" => Some(PasswordFlag::AgentOwned),
                                    "2" => Some(PasswordFlag::NotSaved),
                                    "4" => Some(PasswordFlag::NotRequired),
                                    _ => None,
                                });
                        }

                        (username, connection_type, password_flag)
                    })
                    .unwrap_or_default();

                Some((
                    Arc::from(uuid),
                    ConnectionSettings::Vpn(VpnConnectionSettings {
                        id,
                        connection_type,
                        password_flag,
                        username,
                    }),
                ))
            })
            // Reduce the settings list into
            .fold(IndexMap::new(), |mut set, (uuid, data)| async move {
                set.insert(uuid, data);
                set
            })
            .await;

        Ok::<_, zbus::Error>(settings)
    };

    cosmic::task::future(async move {
        settings.await.map_or_else(
            |why| Message::Error(ErrorKind::ConnectionSettings, why.to_string()),
            Message::KnownConnections,
        )
    })
}
