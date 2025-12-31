// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::BTreeSet, sync::Arc};

use anyhow::Context;
use cosmic::{
    Apply, Element, Task,
    iced::{Alignment, Length},
    iced_core::text::Wrapping,
    widget::{self, icon},
};
use cosmic_dbus_networkmanager::interface::enums::DeviceState;
use cosmic_settings_network_manager_subscription::{
    self as network_manager, NetworkManagerState, current_networks::ActiveConnectionInfo,
};
use cosmic_settings_page::{self as page, Section, section};
use futures::StreamExt;

pub type ConnectionId = Arc<str>;

#[derive(Clone, Debug)]
pub enum Message {
    /// Activate a connection
    Activate(ConnectionId),
    /// Add a network connection with nm-connection-editor
    AddNetwork,
    /// Cancels an active dialog.
    CancelDialog,
    /// Deactivate a connection.
    Deactivate(ConnectionId),
    /// An error occurred.
    Error(String),
    /// An update from the network manager daemon
    NetworkManager(network_manager::Event),
    /// Successfully connected to the system dbus.
    NetworkManagerConnect(zbus::Connection),
    /// Refresh devices and their connection profiles
    Refresh,
    /// Create a dialog to ask for confirmation of removal.
    RemoveProfileRequest(ConnectionId),
    /// Remove a connection profile
    RemoveProfile(ConnectionId),
    /// Selects a device to display connections from
    SelectDevice(Arc<network_manager::devices::DeviceInfo>),
    /// Opens settings page for the access point.
    Settings(ConnectionId),
    /// Update NetworkManagerState
    UpdateState(NetworkManagerState),
    /// Update the devices lists
    UpdateDevices(Vec<network_manager::devices::DeviceInfo>),
    /// Display more options for an access point
    ViewMore(Option<ConnectionId>),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Wired(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Wired(message)
    }
}

pub type InterfaceId = String;

#[derive(Clone, Debug, Eq, PartialEq)]
enum WiredDialog {
    RemoveProfile(ConnectionId),
}

#[derive(Debug, Default)]
pub struct Page {
    entity: page::Entity,
    nm_task: Option<tokio::sync::oneshot::Sender<()>>,
    nm_state: Option<NmState>,
    dialog: Option<WiredDialog>,
    /// When defined, displays connections for the specific device.
    active_device: Option<Arc<network_manager::devices::DeviceInfo>>,
    /// Tracks which connections are in the act of connecting.
    connecting: BTreeSet<ConnectionId>,
    /// Displays a popup when set.
    view_more_popup: Option<ConnectionId>,
    /// Withhold device update if the view more popup is shown.
    withheld_devices: Option<Vec<Arc<network_manager::devices::DeviceInfo>>>,
    /// Withhold active connections update if the view more popup is shown.
    withheld_active_conns: Option<Vec<ActiveConnectionInfo>>,
}

#[derive(Debug)]
pub struct NmState {
    conn: zbus::Connection,
    sender: futures::channel::mpsc::UnboundedSender<network_manager::Request>,
    active_conns: Vec<ActiveConnectionInfo>,
    devices: Vec<Arc<network_manager::devices::DeviceInfo>>,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> cosmic_settings_page::Info {
        page::Info::new("wired", "preferences-wired-symbolic")
            .title(fl!("wired"))
            .description(fl!("connections-and-profiles", variant = "wired"))
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(devices_view())])
    }

    fn dialog(&'_ self) -> Option<Element<'_, crate::pages::Message>> {
        self.dialog.as_ref().map(|dialog| match dialog {
            WiredDialog::RemoveProfile(uuid) => {
                let primary_action = widget::button::destructive(fl!("remove"))
                    .on_press(Message::RemoveProfile(uuid.clone()));

                let secondary_action =
                    widget::button::standard(fl!("cancel")).on_press(Message::CancelDialog);

                widget::dialog()
                    .title(fl!("remove-connection-dialog"))
                    .icon(icon::from_name("dialog-information").size(64))
                    .body(fl!("remove-connection-dialog", "wired-description"))
                    .primary_action(primary_action)
                    .secondary_action(secondary_action)
                    .apply(Element::from)
                    .map(crate::pages::Message::Wired)
            }
        })
    }

    fn header_view(&self) -> Option<cosmic::Element<'_, crate::pages::Message>> {
        Some(
            widget::button::standard(fl!("add-network", "profile"))
                .trailing_icon(icon::from_name("window-pop-out-symbolic"))
                .on_press(Message::AddNetwork)
                .apply(widget::container)
                .width(Length::Fill)
                .align_x(Alignment::End)
                .apply(Element::from)
                .map(crate::pages::Message::Wired),
        )
    }

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        if self.nm_task.is_none() {
            return cosmic::task::future(async move {
                zbus::Connection::system()
                    .await
                    .context("failed to create system dbus connection")
                    .map_or_else(
                        |why| Message::Error(why.to_string()),
                        Message::NetworkManagerConnect,
                    )
                    .apply(crate::pages::Message::Wired)
            });
        }

        Task::none()
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        self.active_device = None;
        self.view_more_popup = None;
        self.nm_state = None;
        self.withheld_active_conns = None;
        self.withheld_devices = None;
        self.connecting.clear();

        if let Some(cancel) = self.nm_task.take() {
            _ = cancel.send(());
        }

        Task::none()
    }

    fn title(&self) -> Option<&str> {
        self.active_device
            .as_ref()
            .map(|device| device.interface.as_str())
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
                    return update_devices(conn);
                }
            }

            Message::UpdateDevices(devices) => {
                self.update_devices(devices);
            }

            Message::UpdateState(state) => {
                self.update_active_conns(state);
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
                        .filter(|info| matches!(info, ActiveConnectionInfo::Wired { .. }))
                        .collect(),
                });

                return update_devices(conn);
            }

            Message::NetworkManager(_event) => (),

            Message::AddNetwork => {
                return cosmic::task::future(async move {
                    _ = super::nm_add_wired().await;
                    // TODO: Update when iced is rebased to use then method.
                    Message::Refresh
                });
            }

            Message::Activate(uuid) => {
                self.close_popup_and_apply_updates();

                if let Some(NmState {
                    ref devices,
                    ref sender,
                    ..
                }) = self.nm_state
                {
                    for device in devices {
                        let device_conn = device
                            .available_connections
                            .iter()
                            .find(|conn| conn.uuid.as_ref() == uuid.as_ref());

                        if let Some(device_conn) = device_conn {
                            let device_path = device.path.clone();
                            let conn_path = device_conn.path.clone();

                            _ = sender.unbounded_send(network_manager::Request::Activate(
                                device_path,
                                conn_path,
                            ));

                            break;
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
                self.dialog = Some(WiredDialog::RemoveProfile(uuid));
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
                    _ = super::nm_edit_connection(uuid.as_ref()).await;
                    // TODO: Update when iced is rebased to use then method.
                    Message::Refresh
                });
            }

            Message::Refresh => {
                if let Some(NmState { ref conn, .. }) = self.nm_state {
                    return cosmic::Task::batch(vec![
                        update_state(conn.clone()),
                        update_devices(conn.clone()),
                    ]);
                }
            }

            Message::CancelDialog => {
                self.dialog = None;
            }

            Message::Error(why) => {
                tracing::error!(why);
            }

            Message::NetworkManagerConnect(conn) => {
                return self.connect(conn);
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
                        network_manager::devices::watch(conn, true, tx)
                    )
                });

                let forwarder = std::pin::pin!(async move {
                    while let Some(message) = rx.next().await {
                        _ = emitter
                            .emit(crate::pages::Message::Wired(Message::NetworkManager(
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
            if let Some(active_conns) = self.withheld_active_conns.take() {
                nm_state.active_conns = active_conns;
            }

            if let Some(devices) = self.withheld_devices.take() {
                nm_state.devices = devices;
            }
        }

        self.update_active_device();
    }

    fn update_active_device(&mut self) {
        if let Some((nm_state, active)) = self.nm_state.as_ref().zip(self.active_device.as_ref()) {
            self.active_device = nm_state
                .devices
                .iter()
                .find(|device| device.path == active.path)
                .map(Arc::clone);
        }
    }

    /// Withholds updates if the view more popup is displayed.
    fn update_devices(&mut self, devices: Vec<network_manager::devices::DeviceInfo>) {
        if let Some(ref mut nm_state) = self.nm_state {
            let devices = devices.into_iter().map(Arc::new).collect();
            if self.view_more_popup.is_some() {
                self.withheld_devices = Some(devices);
            } else {
                nm_state.devices = devices;
            }
        }

        self.update_active_device();
    }

    /// Withholds updates if the view more popup is displayed.
    fn update_active_conns(&mut self, state: NetworkManagerState) {
        if let Some(ref mut nm_state) = self.nm_state {
            let conns = state
                .active_conns
                .into_iter()
                .filter(|info| matches!(info, ActiveConnectionInfo::Wired { .. }))
                .collect();

            if self.view_more_popup.is_some() {
                self.withheld_active_conns = Some(conns);
            } else {
                nm_state.active_conns = conns;
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn device_view<'a>(
        &'a self,
        spacing: cosmic::cosmic_theme::Spacing,
        nm_state: &'a NmState,
        connect_txt: &'a str,
        connected_txt: &'a str,
        disconnect_txt: &'a str,
        remove_txt: &'a str,
        settings_txt: &'a str,
        wired_conns_txt: &'a str,
        unplugged_txt: &'a str,
        device: &'a network_manager::devices::DeviceInfo,
    ) -> Element<'a, Message> {
        let has_multiple_connection_profiles = device.known_connections.len() > 1;
        let header_txt = wired_conns_txt.to_string();

        device
            .known_connections
            .iter()
            .fold(
                widget::settings::section().title(header_txt),
                |networks, connection| {
                    let is_connected = nm_state.active_conns.iter().any(|conn| match conn {
                        ActiveConnectionInfo::Wired { name, .. } => {
                            name.as_str() == connection.id.as_str()
                        }

                        _ => false,
                    });

                    let (connect_txt, connect_msg) = if is_connected {
                        (connected_txt, None)
                    } else if device.state == DeviceState::Unavailable {
                        (unplugged_txt, None)
                    } else {
                        (
                            connect_txt,
                            Some(Message::Activate(connection.uuid.clone())),
                        )
                    };

                    let identifier = widget::text::body(&connection.id).wrapping(Wrapping::Glyph);

                    let connect: Element<'_, Message> = if let Some(msg) = connect_msg {
                        widget::button::text(connect_txt).on_press(msg).into()
                    } else {
                        widget::text::body(connect_txt)
                            .align_y(Alignment::Center)
                            .into()
                    };

                    let view_more_button =
                        widget::button::icon(widget::icon::from_name("view-more-symbolic"));

                    let view_more: Option<Element<_>> = if self
                        .view_more_popup
                        .as_deref()
                        .is_some_and(|id| id == connection.uuid.as_ref())
                    {
                        widget::popover(view_more_button.on_press(Message::ViewMore(None)))
                            .position(widget::popover::Position::Bottom)
                            .on_close(Message::ViewMore(None))
                            .popup(
                                widget::column()
                                    .push_maybe(is_connected.then(|| {
                                        popup_button(
                                            Message::Deactivate(connection.uuid.clone()),
                                            disconnect_txt,
                                        )
                                    }))
                                    .push(popup_button(
                                        Message::Settings(connection.uuid.clone()),
                                        settings_txt,
                                    ))
                                    .push_maybe(has_multiple_connection_profiles.then(|| {
                                        popup_button(
                                            Message::RemoveProfileRequest(connection.uuid.clone()),
                                            remove_txt,
                                        )
                                    }))
                                    .width(Length::Fixed(200.0))
                                    .apply(widget::container)
                                    .padding(cosmic::theme::spacing().space_xxs)
                                    .class(cosmic::theme::Container::Dropdown),
                            )
                            .apply(|e| Some(Element::from(e)))
                    } else {
                        view_more_button
                            .on_press(Message::ViewMore(Some(connection.uuid.clone())))
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
            )
            .into()
    }
}

fn devices_view() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        wired_conns_txt = fl!("wired", "connections");
        remove_txt = fl!("wired", "remove");
        connect_txt = fl!("connect");
        connected_txt = fl!("connected");
        settings_txt = fl!("settings");
        disconnect_txt = fl!("disconnect");
        unplugged_txt = fl!("network-device-state", "unplugged");
    });

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let Some(ref nm_state) = page.nm_state else {
                return cosmic::widget::column().into();
            };

            let spacing = cosmic::theme::spacing();

            let mut view = widget::column::with_capacity(4);

            // Displays device connections if a device is selected, or only device exists.
            let active_device = page
                .active_device
                .as_ref()
                .or_else(|| (nm_state.devices.len() == 1).then(|| nm_state.devices.first())?);

            view = match active_device {
                Some(device) => view.push(page.device_view(
                    spacing,
                    nm_state,
                    &section.descriptions[connect_txt],
                    &section.descriptions[connected_txt],
                    &section.descriptions[disconnect_txt],
                    &section.descriptions[remove_txt],
                    &section.descriptions[settings_txt],
                    &section.descriptions[wired_conns_txt],
                    &section.descriptions[unplugged_txt],
                    device,
                )),

                None => view,
            };

            view.spacing(spacing.space_l)
                .apply(Element::from)
                .map(crate::pages::Message::Wired)
        })
}

fn popup_button(message: Message, text: &str) -> Element<'_, Message> {
    let spacing = cosmic::theme::spacing();
    widget::text::body(text)
        .align_y(Alignment::Center)
        .apply(widget::button::custom)
        .padding([spacing.space_xxxs, spacing.space_xs])
        .width(Length::Fill)
        .class(cosmic::theme::Button::MenuItem)
        .on_press(message)
        .into()
}

fn update_state(conn: zbus::Connection) -> Task<crate::app::Message> {
    cosmic::task::future(async move {
        match NetworkManagerState::new(&conn).await {
            Ok(state) => Message::UpdateState(state),
            Err(why) => Message::Error(why.to_string()),
        }
    })
}

fn update_devices(conn: zbus::Connection) -> Task<crate::app::Message> {
    cosmic::task::future(async move {
        let filter =
            |device_type| matches!(device_type, network_manager::devices::DeviceType::Ethernet);

        match network_manager::devices::list(&conn, filter).await {
            Ok(devices) => Message::UpdateDevices(devices),
            Err(why) => Message::Error(why.to_string()),
        }
    })
}
