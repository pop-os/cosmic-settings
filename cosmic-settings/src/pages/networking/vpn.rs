// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;

use anyhow::Context;
use ashpd::desktop::file_chooser::FileFilter;
use cosmic::{
    iced::{alignment, Length},
    iced_core::text::Wrap,
    prelude::CollectionWidget,
    widget, Apply, Command, Element,
};
use cosmic_settings_page::{self as page, section, Section};
use cosmic_settings_subscriptions::network_manager::{
    self, current_networks::ActiveConnectionInfo, NetworkManagerState, UUID,
};
use futures::{FutureExt, StreamExt};
use indexmap::IndexMap;
use slab::Slab;
use zbus::zvariant::ObjectPath;

pub type ConnectionId = Arc<str>;
pub type InterfaceId = String;

#[derive(Clone, Debug)]
pub enum Message {
    /// Activate a connection
    Activate(ConnectionId),
    /// Add a network connection
    AddNetwork,
    /// Deactivate a connection.
    Deactivate(ConnectionId),
    /// An error occurred.
    Error(String),
    /// Update the list of known connections.
    KnownConnections(IndexMap<UUID, VpnConnectionSettings>),
    /// An update from the network manager daemon
    NetworkManager(network_manager::Event),
    /// Successfully connected to the system dbus.
    NetworkManagerConnect(
        (
            zbus::Connection,
            tokio::sync::mpsc::Sender<crate::pages::Message>,
        ),
    ),
    /// Refresh devices and their connection profiles
    Refresh,
    /// Remove a connection profile
    RemoveProfile(ConnectionId),
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
        crate::pages::Message::Vpn(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Vpn(message)
    }
}

#[derive(Clone, Debug, Default)]
struct VpnConnectionSettings {
    path: ObjectPath<'static>,
    id: String,
    connection_type: String,
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
    nm_task: Option<tokio::sync::oneshot::Sender<()>>,
    nm_state: Option<NmState>,
    view_more_popup: Option<ConnectionId>,
    known_connections: IndexMap<UUID, VpnConnectionSettings>,
    /// Withhold device update if the view more popup is shown.
    withheld_devices: Option<Vec<network_manager::devices::DeviceInfo>>,
    /// Withhold active connections update if the view more popup is shown.
    withheld_active_conns: Option<Vec<ActiveConnectionInfo>>,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> cosmic_settings_page::Info {
        page::Info::new("vpn", "preferences-network-and-wireless-symbolic")
            .title(fl!("vpn"))
            .description(fl!("connections-and-profiles", variant = "vpn"))
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(devices_view())])
    }

    fn header_view(&self) -> Option<cosmic::Element<'_, crate::pages::Message>> {
        Some(
            widget::button::standard(fl!("add-network"))
                .on_press(Message::AddNetwork)
                .apply(widget::container)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Right)
                .apply(Element::from)
                .map(crate::pages::Message::Vpn),
        )
    }

    fn on_enter(
        &mut self,
        _page: cosmic_settings_page::Entity,
        sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> cosmic::Command<crate::pages::Message> {
        if self.nm_task.is_none() {
            return cosmic::command::future(async move {
                zbus::Connection::system()
                    .await
                    .context("failed to create system dbus connection")
                    .map_or_else(
                        |why| Message::Error(why.to_string()),
                        |conn| Message::NetworkManagerConnect((conn, sender.clone())),
                    )
            });
        }

        Command::none()
    }

    fn on_leave(&mut self) -> Command<crate::pages::Message> {
        self.view_more_popup = None;
        self.nm_state = None;
        self.withheld_active_conns = None;
        self.withheld_devices = None;

        if let Some(cancel) = self.nm_task.take() {
            _ = cancel.send(());
        }

        Command::none()
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
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
                    return cosmic::command::batch(vec![
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
                    return cosmic::command::batch(vec![
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

                return cosmic::command::batch(vec![
                    connection_settings(conn.clone()),
                    update_devices(conn),
                ]);
            }

            Message::NetworkManager(_event) => (),

            Message::AddNetwork => return add_network(),

            Message::Activate(uuid) => {
                self.close_popup_and_apply_updates();

                if let Some(NmState { ref sender, .. }) = self.nm_state {
                    if let Some(settings) = self.known_connections.get(&uuid) {
                        _ = sender.unbounded_send(network_manager::Request::Activate(
                            ObjectPath::from_static_str_unchecked("/"),
                            settings.path.clone(),
                        ));
                    }
                }
            }

            Message::Deactivate(uuid) => {
                self.close_popup_and_apply_updates();
                if let Some(NmState { ref sender, .. }) = self.nm_state {
                    _ = sender.unbounded_send(network_manager::Request::Deactivate(uuid));
                }
            }

            Message::RemoveProfile(uuid) => {
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

                return cosmic::command::future(async move {
                    super::nm_edit_connection(uuid.as_ref())
                        .then(|res| async move {
                            match res.context("failed to open connection editor") {
                                Ok(_) => Message::Refresh,
                                Err(why) => Message::Error(why.to_string()),
                            }
                        })
                        .await
                });
            }

            Message::Refresh => {
                if let Some(NmState { ref conn, .. }) = self.nm_state {
                    return cosmic::command::batch(vec![
                        update_state(conn.clone()),
                        update_devices(conn.clone()),
                        connection_settings(conn.clone()),
                    ]);
                }
            }

            Message::Error(why) => {
                tracing::error!(why, "error in VPN settings page");
            }

            Message::NetworkManagerConnect((conn, output)) => {
                self.connect(conn.clone(), output);
            }
        }

        Command::none()
    }

    fn connect(
        &mut self,
        conn: zbus::Connection,
        sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) {
        if self.nm_task.is_none() {
            self.nm_task = Some(crate::utils::forward_event_loop(
                sender,
                |event| crate::pages::Message::Vpn(Message::NetworkManager(event)),
                move |tx| async move {
                    futures::join!(
                        network_manager::watch(conn.clone(), tx.clone()),
                        network_manager::active_conns::watch(conn.clone(), tx.clone()),
                        network_manager::devices::watch(conn, true, tx)
                    );
                },
            ));
        }
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

            if page.known_connections.is_empty() {
            } else {
                let known_networks = page.known_connections.iter().fold(
                    widget::settings::view_section(&section.descriptions[vpn_conns_txt]),
                    |networks, (uuid, connection)| {
                        let is_connected = active_conns.iter().any(|conn| match conn {
                            ActiveConnectionInfo::Vpn { name, .. } => {
                                name.as_str() == connection.id.as_str()
                            }

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

                        let identifier =
                            widget::text::body(connection.id.as_str()).wrap(Wrap::Glyph);

                        let connect = widget::button::text(connect_txt).on_press_maybe(connect_msg);

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
                                            Message::RemoveProfile(uuid.clone()),
                                            &section.descriptions[remove_txt],
                                        ))
                                        .width(Length::Fixed(200.0))
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
                            .spacing(spacing.space_xxs);

                        let widget = widget::settings::item_row(vec![
                            identifier.into(),
                            widget::horizontal_space(Length::Fill).into(),
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

fn popup_button<'a>(message: Message, text: &'a str) -> Element<'a, Message> {
    widget::text::body(text)
        .vertical_alignment(alignment::Vertical::Center)
        .apply(widget::button)
        .padding([4, 16])
        .width(Length::Fill)
        .style(cosmic::theme::Button::MenuItem)
        .on_press(message)
        .into()
}

fn update_state(conn: zbus::Connection) -> Command<crate::app::Message> {
    cosmic::command::future(async move {
        match NetworkManagerState::new(&conn).await {
            Ok(state) => Message::UpdateState(state),
            Err(why) => Message::Error(why.to_string()),
        }
    })
}

fn update_devices(conn: zbus::Connection) -> Command<crate::app::Message> {
    cosmic::command::future(async move {
        let filter =
            |device_type| matches!(device_type, network_manager::devices::DeviceType::WireGuard);

        match network_manager::devices::list(&conn, filter).await {
            Ok(devices) => Message::UpdateDevices(devices),
            Err(why) => Message::Error(why.to_string()),
        }
    })
}

fn add_network() -> Command<crate::app::Message> {
    let Some(dir) = dirs::download_dir().or_else(dirs::home_dir) else {
        return Command::none();
    };

    cosmic::dialog::file_chooser::open::Dialog::new()
        .directory(dir)
        .title(fl!("vpn", "select-file"))
        .filter(FileFilter::new("OpenVPN").mimetype("application/x-openvpn-profile"))
        .open_file()
        .then(|result| async move {
            match result {
                Ok(response) => {
                    _ = super::nm_add_vpn_file("openvpn", response.url().path()).await;
                    Message::Refresh
                }
                Err(why) => {
                    return Message::Error(why.to_string());
                }
            }
        })
        .apply(cosmic::command::future)
}

fn connection_settings(conn: zbus::Connection) -> Command<crate::app::Message> {
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

                let (connection, vpn) = settings.get("connection").zip(settings.get("vpn"))?;

                if connection.get("type")?.downcast_ref::<String>().ok()? != "vpn" {
                    return None;
                }

                let id = connection.get("id")?.downcast_ref::<String>().ok()?;
                let uuid = connection.get("uuid")?.downcast_ref::<String>().ok()?;

                let connection_type = vpn
                    .get("data")
                    .and_then(|data| data.downcast_ref::<zbus::zvariant::Dict>().ok())
                    .and_then(|dict| {
                        dict.get::<String, String>(&String::from("connection-type"))
                            .ok()
                    })
                    .flatten()
                    .unwrap_or_default();

                let path = conn.inner().path().to_owned();

                Some((
                    Arc::from(uuid),
                    VpnConnectionSettings {
                        path,
                        id,
                        connection_type,
                    },
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

    cosmic::command::future(async move {
        settings
            .await
            .context("failed to get connection settings")
            .map_or_else(
                |why| Message::Error(why.to_string()),
                Message::KnownConnections,
            )
    })
}
