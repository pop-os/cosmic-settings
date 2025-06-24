// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use anyhow::Context;
use cosmic::{
    Apply, Element, Task,
    iced::{Alignment, Length},
    iced_core::text::Wrapping,
    iced_widget::focus_next,
    widget::{self, column, icon},
};
use cosmic_settings_page::{self as page, Section, section};
use cosmic_settings_subscriptions::network_manager::{
    self, NetworkManagerState,
    available_wifi::{AccessPoint, NetworkType},
    current_networks::ActiveConnectionInfo,
    hw_address::HwAddress,
};
use futures::StreamExt;
use secure_string::SecureString;

#[derive(Clone, Debug)]
pub enum Message {
    /// Add a network connection with nm-connection-editor
    AddNetwork,
    /// Cancels a dialog.
    CancelDialog,
    /// Connect to a WiFi network access point.
    Connect(network_manager::SSID),
    /// Connect with a password
    ConnectWithPassword,
    /// Settings for known connections.
    ConnectionSettings(BTreeMap<Box<str>, Box<str>>),
    /// Disconnect from an access point.
    Disconnect(network_manager::SSID),
    /// An error occurred.
    Error(String),
    /// Identity update from the dialog
    IdentityUpdate(String),
    /// Create a dialog to ask for confirmation on forgetting a connection.
    ForgetRequest(network_manager::SSID),
    /// Forget a known access point.
    Forget(network_manager::SSID),
    /// An update from the network manager daemon
    NetworkManager(network_manager::Event),
    /// Successfully connected to the system dbus.
    NetworkManagerConnect(zbus::Connection),
    /// Request an auth dialog
    PasswordRequest(network_manager::SSID),
    /// Update the password from the dialog
    PasswordUpdate(SecureString),
    /// Selects a device to display connections from
    SelectDevice(Arc<network_manager::devices::DeviceInfo>),
    /// Opens settings page for the access point.
    Settings(network_manager::SSID),
    /// Identity submitted from the dialog
    SubmitIdentity,
    /// Toggles visibility of the password input
    TogglePasswordVisibility,
    /// Update NetworkManagerState
    UpdateState(NetworkManagerState),
    /// Update the devices lists
    UpdateDevices(Vec<network_manager::devices::DeviceInfo>),
    /// Display more options for an access point
    ViewMore(Option<network_manager::SSID>),
    /// Toggle WiFi access
    WiFiEnable(bool),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::WiFi(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::WiFi(message)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WiFiDialog {
    Forget(network_manager::SSID),
    Password {
        ssid: network_manager::SSID,
        hw_address: HwAddress,
        identity: Option<String>,
        password: SecureString,
        password_hidden: bool,
    },
}

#[derive(Debug, Default)]
pub struct Page {
    entity: page::Entity,
    nm_task: Option<tokio::sync::oneshot::Sender<()>>,
    nm_state: Option<NmState>,
    /// When defined, displays connections for the specific device.
    active_device: Option<Arc<network_manager::devices::DeviceInfo>>,
    dialog: Option<WiFiDialog>,
    view_more_popup: Option<network_manager::SSID>,
    connecting: BTreeSet<network_manager::SSID>,
    ssid_to_uuid: BTreeMap<Box<str>, Box<str>>,
    /// Withhold device update if the view more popup is shown.
    withheld_devices: Option<Vec<network_manager::devices::DeviceInfo>>,
    /// Withhold state update if the view more popup is shown.
    withheld_state: Option<NetworkManagerState>,
}

#[derive(Debug)]
pub struct NmState {
    conn: zbus::Connection,
    sender: futures::channel::mpsc::UnboundedSender<network_manager::Request>,
    state: network_manager::NetworkManagerState,
    devices: Vec<network_manager::devices::DeviceInfo>,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> cosmic_settings_page::Info {
        page::Info::new("wifi", "preferences-wireless-symbolic")
            .title(fl!("wifi"))
            .description(fl!("connections-and-profiles", variant = "wifi"))
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(devices_view())])
    }

    fn dialog(&self) -> Option<Element<crate::pages::Message>> {
        self.dialog.as_ref().map(|dialog| match dialog {
            WiFiDialog::Password {
                password,
                identity,
                password_hidden,
                ..
            } => {
                let password = widget::text_input::secure_input(
                    fl!("password"),
                    password.unsecure(),
                    Some(Message::TogglePasswordVisibility),
                    *password_hidden,
                )
                .on_input(|input| Message::PasswordUpdate(SecureString::from(input)))
                .on_submit(|_| Message::ConnectWithPassword);

                let primary_action = widget::button::suggested(fl!("connect"))
                    .on_press(Message::ConnectWithPassword);

                let secondary_action =
                    widget::button::standard(fl!("cancel")).on_press(Message::CancelDialog);

                let control: Element<_> = if let Some(identity) = identity {
                    column::column()
                        .spacing(8)
                        .push(
                            widget::text_input::text_input(fl!("identity"), identity)
                                .on_input(Message::IdentityUpdate)
                                .on_submit(|_| Message::SubmitIdentity),
                        )
                        .push(password)
                        .into()
                } else {
                    password.into()
                };

                widget::dialog()
                    .title(fl!("auth-dialog"))
                    .icon(icon::from_name("preferences-wireless-symbolic").size(64))
                    .body(fl!("auth-dialog", "wifi-description"))
                    .control(control)
                    .primary_action(primary_action)
                    .secondary_action(secondary_action)
                    .apply(Element::from)
                    .map(crate::pages::Message::WiFi)
            }

            WiFiDialog::Forget(ssid) => {
                let primary_action = widget::button::destructive(fl!("forget"))
                    .on_press(Message::Forget(ssid.clone()));

                let secondary_action =
                    widget::button::standard(fl!("cancel")).on_press(Message::CancelDialog);

                widget::dialog()
                    .title(fl!("forget-dialog"))
                    .icon(icon::from_name("dialog-information").size(64))
                    .body(fl!("forget-dialog", "description"))
                    .primary_action(primary_action)
                    .secondary_action(secondary_action)
                    .apply(Element::from)
                    .map(crate::pages::Message::WiFi)
            }
        })
    }

    fn header_view(&self) -> Option<cosmic::Element<'_, crate::pages::Message>> {
        Some(
            widget::button::standard(fl!("add-network"))
                .trailing_icon(icon::from_name("window-pop-out-symbolic"))
                .on_press(Message::AddNetwork)
                .apply(widget::container)
                .width(Length::Fill)
                .align_x(Alignment::End)
                .apply(Element::from)
                .map(crate::pages::Message::WiFi),
        )
    }

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        if self.nm_task.is_none() {
            return cosmic::Task::future(async move {
                zbus::Connection::system()
                    .await
                    .context("failed to create system dbus connection")
                    .map_or_else(
                        |why| Message::Error(why.to_string()),
                        Message::NetworkManagerConnect,
                    )
                    .apply(crate::pages::Message::WiFi)
            });
        }

        Task::none()
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        self.active_device = None;
        self.view_more_popup = None;
        self.nm_state = None;
        self.ssid_to_uuid.clear();
        self.connecting.clear();
        self.withheld_state = None;
        self.withheld_devices = None;

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

                match req {
                    network_manager::Request::Authenticate {
                        ssid,
                        identity,
                        hw_address,
                        ..
                    } => {
                        if success {
                            self.connecting.remove(ssid.as_str());
                        } else {
                            // Request to retry
                            self.dialog = Some(WiFiDialog::Password {
                                ssid: ssid.into(),
                                identity,
                                hw_address,
                                password: SecureString::from(""),
                                password_hidden: true,
                            });
                        }
                    }

                    network_manager::Request::SelectAccessPoint(
                        ssid,
                        _hw_address,
                        _network_type,
                    ) => {
                        self.connecting.remove(ssid.as_ref());
                    }

                    _ => (),
                }

                self.update_state(state);

                if let Some(NmState { ref conn, .. }) = self.nm_state {
                    return update_devices(conn.clone());
                }
            }

            Message::UpdateDevices(devices) => {
                self.update_devices(devices);
            }

            Message::UpdateState(state) => {
                self.update_state(state);

                if let Some(NmState { ref conn, .. }) = self.nm_state {
                    return connection_settings(conn.clone());
                }
            }

            Message::NetworkManager(
                network_manager::Event::ActiveConns
                | network_manager::Event::Devices
                | network_manager::Event::WiFiEnabled(_)
                | network_manager::Event::WirelessAccessPoints,
            ) => {
                if let Some(NmState { ref conn, .. }) = self.nm_state {
                    return cosmic::Task::batch(vec![
                        update_state(conn.clone()),
                        update_devices(conn.clone()),
                    ]);
                }
            }

            Message::ConnectionSettings(settings) => {
                self.ssid_to_uuid = settings;
            }

            Message::NetworkManager(network_manager::Event::Init {
                conn,
                sender,
                state,
            }) => {
                self.nm_state = Some(NmState {
                    conn: conn.clone(),
                    sender,
                    state,
                    devices: Vec::new(),
                });

                return update_devices(conn);
            }

            Message::AddNetwork => {
                tokio::task::spawn(super::nm_add_wifi());
            }

            Message::Connect(ssid) => {
                if let Some(nm) = self.nm_state.as_mut() {
                    let Some(ap) = nm
                        .state
                        .wireless_access_points
                        .iter()
                        .chain(nm.state.known_access_points.iter())
                        .find(|ap| ap.ssid == ssid)
                    else {
                        return Task::none();
                    };
                    self.connecting.insert(ssid.clone());
                    _ = nm
                        .sender
                        .unbounded_send(network_manager::Request::SelectAccessPoint(
                            ssid,
                            ap.hw_address,
                            ap.network_type,
                        ));
                }
            }

            Message::IdentityUpdate(new_identity) => {
                if let Some(WiFiDialog::Password {
                    ref mut identity, ..
                }) = self.dialog
                {
                    *identity = Some(new_identity);
                }
            }

            Message::PasswordRequest(ssid) => {
                if let Some(nm) = self.nm_state.as_mut() {
                    let Some(ap) = nm
                        .state
                        .wireless_access_points
                        .iter()
                        .chain(nm.state.known_access_points.iter())
                        .find(|ap| ap.ssid == ssid)
                    else {
                        return Task::none();
                    };
                    self.dialog = Some(WiFiDialog::Password {
                        ssid,
                        identity: matches!(ap.network_type, NetworkType::EAP).then(String::new),
                        hw_address: ap.hw_address,
                        password: SecureString::from(""),
                        password_hidden: true,
                    });
                }
            }

            Message::PasswordUpdate(pass) => {
                if let Some(WiFiDialog::Password {
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

                if let WiFiDialog::Password {
                    ssid,
                    identity,
                    password,
                    hw_address,
                    ..
                } = dialog
                {
                    if let Some(nm) = self.nm_state.as_mut() {
                        self.connecting.insert(ssid.clone());
                        _ = nm
                            .sender
                            .unbounded_send(network_manager::Request::Authenticate {
                                ssid: ssid.to_string(),
                                identity,
                                hw_address,
                                password,
                            });
                    }
                }
            }

            Message::TogglePasswordVisibility => {
                if let Some(WiFiDialog::Password {
                    ref mut password_hidden,
                    ..
                }) = self.dialog
                {
                    *password_hidden = !*password_hidden;
                }
            }

            Message::ViewMore(ssid) => {
                self.view_more_popup = ssid;
                if self.view_more_popup.is_none() {
                    self.close_popup_and_apply_updates();
                }
            }

            Message::Disconnect(ssid) => {
                self.close_popup_and_apply_updates();
                if let Some(nm) = self.nm_state.as_mut() {
                    _ = nm
                        .sender
                        .unbounded_send(network_manager::Request::Disconnect(ssid));
                }
            }

            Message::ForgetRequest(ssid) => {
                self.dialog = Some(WiFiDialog::Forget(ssid));
                self.view_more_popup = None;
            }

            Message::Forget(ssid) => {
                self.dialog = None;
                self.close_popup_and_apply_updates();
                if let Some(nm) = self.nm_state.as_mut() {
                    _ = nm
                        .sender
                        .unbounded_send(network_manager::Request::Forget(ssid));
                }
            }

            Message::Settings(ssid) => {
                self.close_popup_and_apply_updates();

                if let Some(uuid) = self.ssid_to_uuid.get(ssid.as_ref()).cloned() {
                    tokio::task::spawn(
                        async move { super::nm_edit_connection(uuid.as_ref()).await },
                    );
                }
            }

            Message::SubmitIdentity => {
                if self.dialog.is_some() {
                    return focus_next();
                }
            }

            Message::WiFiEnable(enable) => {
                if let Some(nm) = self.nm_state.as_mut() {
                    _ = nm
                        .sender
                        .unbounded_send(network_manager::Request::SetWiFi(enable));
                    _ = nm.sender.unbounded_send(network_manager::Request::Reload);
                }
            }

            Message::CancelDialog => {
                self.dialog = None;
            }

            Message::Error(why) => {
                tracing::error!(why);
            }

            Message::SelectDevice(device) => {
                // TODO: Per-device wifi connection handling.
                self.active_device = Some(device);
            }

            Message::NetworkManagerConnect(conn) => {
                return cosmic::task::batch(vec![
                    self.connect(conn.clone()),
                    connection_settings(conn),
                ]);
            }
        }

        Task::none()
    }

    fn connect(&mut self, conn: zbus::Connection) -> Task<crate::app::Message> {
        if self.nm_task.is_none() {
            let (canceller, task) = crate::utils::forward_event_loop(move |emitter| async move {
                let (tx, mut rx) = futures::channel::mpsc::channel(1);

                let watchers = std::pin::pin!(async move {
                    futures::join!(
                        network_manager::watch(conn.clone(), tx.clone()),
                        network_manager::active_conns::watch(conn.clone(), tx.clone()),
                        network_manager::wireless_enabled::watch(conn.clone(), tx.clone()),
                        network_manager::watch_connections_changed(conn, tx)
                    );
                });

                let forwarder = std::pin::pin!(async move {
                    while let Some(message) = rx.next().await {
                        _ = emitter
                            .emit(crate::pages::Message::WiFi(Message::NetworkManager(
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

    /// Closes the view more popup and applies any withheld updates.
    fn close_popup_and_apply_updates(&mut self) {
        self.view_more_popup = None;
        if let Some(ref mut nm_state) = self.nm_state {
            if let Some(state) = self.withheld_state.take() {
                nm_state.state = state;
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
    fn update_state(&mut self, state: NetworkManagerState) {
        if let Some(ref mut nm_state) = self.nm_state {
            if self.view_more_popup.is_some() {
                self.withheld_state = Some(state);
            } else {
                nm_state.state = state;
            }
        }
    }
}

fn devices_view() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        airplane_mode_txt = fl!("airplane-on");
        connect_txt = fl!("connect");
        connected_txt = fl!("connected");
        connecting_txt = fl!("connecting");
        disconnect_txt = fl!("disconnect");
        forget_txt = fl!("wifi", "forget");
        known_networks_txt = fl!("known-networks");
        no_networks_txt = fl!("no-networks");
        settings_txt = fl!("settings");
        visible_networks_txt = fl!("visible-networks");
        wifi_txt = fl!("wifi");
    });

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let Some(NmState { ref state, .. }) = page.nm_state else {
                return cosmic::widget::column().into();
            };

            let theme = cosmic::theme::active();
            let spacing = &theme.cosmic().spacing;

            let wifi_enable = widget::settings::item::builder(&section.descriptions[wifi_txt])
                .control(widget::toggler(state.wifi_enabled).on_toggle(Message::WiFiEnable));

            let mut view = widget::column::with_capacity(4)
                .push(widget::list_column().add(wifi_enable))
                .push_maybe(state.airplane_mode.then(|| {
                    widget::row::with_capacity(2)
                        .push(icon::from_name("airplane-mode-symbolic"))
                        .push(widget::text::body(&section.descriptions[airplane_mode_txt]))
                        .spacing(8)
                        .align_y(Alignment::Center)
                        .apply(widget::container)
                        .center_x(Length::Fill)
                }));

            if !state.airplane_mode
                && state.known_access_points.is_empty()
                && state.wireless_access_points.is_empty()
            {
                let no_networks_found =
                    widget::container(widget::text::body(&section.descriptions[no_networks_txt]))
                        .center_x(Length::Fill);

                view = view.push(no_networks_found);
            } else {
                let mut has_known = false;
                let mut has_visible = false;

                // Create separate sections for known and visible networks.
                let (known_networks, visible_networks) = state.wireless_access_points.iter().fold(
                    (
                        widget::settings::section()
                            .title(&section.descriptions[known_networks_txt]),
                        widget::settings::section()
                            .title(&section.descriptions[visible_networks_txt]),
                    ),
                    |(mut known_networks, mut visible_networks), network| {
                        let is_connected = is_connected(state, network);

                        let is_known = state
                            .known_access_points
                            .iter()
                            .map(|known| known.ssid.as_ref())
                            .chain(state.active_conns.iter().filter_map(|active| {
                                if let ActiveConnectionInfo::WiFi { name, .. } = active {
                                    Some(name.as_str())
                                } else {
                                    None
                                }
                            }))
                            .any(|known| known == network.ssid.as_ref());

                        let is_encrypted = network.network_type != NetworkType::Open;

                        let (connect_txt, connect_msg) = if is_connected {
                            (&section.descriptions[connected_txt], None)
                        } else if page.connecting.contains(&network.ssid) {
                            (&section.descriptions[connecting_txt], None)
                        } else {
                            (
                                &section.descriptions[connect_txt],
                                Some(if is_known || !is_encrypted {
                                    Message::Connect(network.ssid.clone())
                                } else {
                                    Message::PasswordRequest(network.ssid.clone())
                                }),
                            )
                        };

                        let identifier = widget::row::with_capacity(3)
                            .push(widget::icon::from_name(wifi_icon(network.strength)))
                            .push_maybe(
                                is_encrypted
                                    .then(|| widget::icon::from_name("connection-secure-symbolic")),
                            )
                            .push(
                                widget::text::body(network.ssid.as_ref()).wrapping(Wrapping::Glyph),
                            )
                            .spacing(spacing.space_xxs);

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
                            .map_or(false, |id| id == network.ssid.as_ref())
                        {
                            widget::popover(view_more_button.on_press(Message::ViewMore(None)))
                                .position(widget::popover::Position::Bottom)
                                .on_close(Message::ViewMore(None))
                                .popup({
                                    widget::column()
                                        .push_maybe(is_connected.then(|| {
                                            popup_button(
                                                Message::Disconnect(network.ssid.clone()),
                                                &section.descriptions[disconnect_txt],
                                            )
                                        }))
                                        .push(popup_button(
                                            Message::Settings(network.ssid.clone()),
                                            &section.descriptions[settings_txt],
                                        ))
                                        .push_maybe(is_known.then(|| {
                                            popup_button(
                                                Message::ForgetRequest(network.ssid.clone()),
                                                &section.descriptions[forget_txt],
                                            )
                                        }))
                                        .width(Length::Fixed(170.0))
                                        .apply(widget::container)
                                        .class(cosmic::style::Container::Dialog)
                                })
                                .apply(|e| Some(Element::from(e)))
                        } else if is_known {
                            view_more_button
                                .on_press(Message::ViewMore(Some(network.ssid.clone())))
                                .apply(|e| Some(Element::from(e)))
                        } else {
                            None
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

                        if is_known {
                            has_known = true;
                            known_networks = known_networks.add(widget);
                        } else {
                            has_visible = true;
                            visible_networks = visible_networks.add(widget);
                        }

                        (known_networks, visible_networks)
                    },
                );

                if has_known {
                    view = view.push(known_networks);
                }

                if has_visible {
                    view = view.push(visible_networks);
                }
            };

            view.spacing(spacing.space_l)
                .apply(Element::from)
                .map(crate::pages::Message::WiFi)
        })
}

fn is_connected(state: &NetworkManagerState, network: &AccessPoint) -> bool {
    state.active_conns.iter().any(|active| {
        if let ActiveConnectionInfo::WiFi { name, .. } = active {
            *name == network.ssid.as_ref()
        } else {
            false
        }
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
            // Concurrently fetch settings for each connection.
            .filter_map(|conn| async move {
                conn.get_settings()
                    .await
                    .map(network_manager::Settings::new)
                    .ok()
            })
            // Reduce the settings list into a SSID->UUID map.
            .fold(BTreeMap::new(), |mut set, settings| async move {
                if let Some(ref wifi) = settings.wifi {
                    if let Some(ssid) = wifi
                        .ssid
                        .clone()
                        .and_then(|ssid| String::from_utf8(ssid).ok())
                    {
                        if let Some(ref connection) = settings.connection {
                            if let Some(uuid) = connection.uuid.clone() {
                                set.insert(ssid.into(), uuid.into());
                                return set;
                            }
                        }
                    }
                }

                set
            })
            .await;

        Ok::<_, zbus::Error>(settings)
    };

    cosmic::task::future(async move {
        settings
            .await
            .context("failed to get connection settings")
            .map_or_else(
                |why| Message::Error(why.to_string()),
                Message::ConnectionSettings,
            )
            .apply(crate::pages::Message::WiFi)
    })
}

pub fn update_state(conn: zbus::Connection) -> Task<crate::app::Message> {
    cosmic::task::future(async move {
        match NetworkManagerState::new(&conn).await {
            Ok(state) => Message::UpdateState(state),
            Err(why) => Message::Error(why.to_string()),
        }
    })
}

pub fn update_devices(conn: zbus::Connection) -> Task<crate::app::Message> {
    cosmic::task::future(async move {
        let filter =
            |device_type| matches!(device_type, network_manager::devices::DeviceType::Wifi);
        match network_manager::devices::list(&conn, filter).await {
            Ok(devices) => Message::UpdateDevices(devices),
            Err(why) => Message::Error(why.to_string()),
        }
    })
}

fn wifi_icon(strength: u8) -> &'static str {
    if strength < 25 {
        "network-wireless-signal-weak-symbolic"
    } else if strength < 50 {
        "network-wireless-signal-ok-symbolic"
    } else if strength < 75 {
        "network-wireless-signal-good-symbolic"
    } else {
        "network-wireless-signal-excellent-symbolic"
    }
}
