// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Mobile broadband profiles managed by NetworkManager.

pub mod details;

use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use cosmic::iced::{Alignment, Length, Subscription};
use cosmic::widget::space::horizontal as horizontal_space;
use cosmic::widget::{self, icon};
use cosmic::{Apply, Element, Task, surface};
use cosmic_settings_page::{self as page, Section, section};
use slotmap::SlotMap;
use zbus::{Connection, fdo::ObjectManagerProxy, proxy};

use super::backend as network_manager;
use super::backend::devices::{DeviceInfo, DeviceState, DeviceType};

pub type ConnectionId = Arc<str>;

const DEFAULT_AUTOCONNECT_PRIORITY: i32 = 100;
const MODEM_MANAGER_SERVICE: &str = "org.freedesktop.ModemManager1";
const MODEM_MANAGER_PATH: &str = "/org/freedesktop/ModemManager1";
const MODEM_INTERFACE: &str = "org.freedesktop.ModemManager1.Modem";

// `MMModemMode` bit flags from ModemManager's public D-Bus API.
const MODE_3G: u32 = 1 << 2;
const MODE_4G: u32 = 1 << 3;
const MODE_5G: u32 = 1 << 4;
const MODE_3G_4G: u32 = MODE_3G | MODE_4G;
const MODE_3G_5G: u32 = MODE_3G | MODE_5G;
const MODE_4G_5G: u32 = MODE_4G | MODE_5G;
const MODE_3G_4G_5G: u32 = MODE_3G | MODE_4G | MODE_5G;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NetworkMode {
    allowed: u32,
    preferred: u32,
}

impl NetworkMode {
    const AUTOMATIC: Self = Self {
        allowed: MODE_3G_4G_5G,
        preferred: MODE_5G,
    };
    const FOUR_G_FIVE_G: Self = Self {
        allowed: MODE_4G_5G,
        preferred: MODE_5G,
    };
    const FIVE_G_ONLY: Self = Self {
        allowed: MODE_5G,
        preferred: 0,
    };
    const THREE_G_FOUR_G: Self = Self {
        allowed: MODE_3G_4G,
        preferred: MODE_4G,
    };
    const FOUR_G_ONLY: Self = Self {
        allowed: MODE_4G,
        preferred: 0,
    };
    const THREE_G_ONLY: Self = Self {
        allowed: MODE_3G,
        preferred: 0,
    };
    fn label(self) -> String {
        match self {
            Self::AUTOMATIC => fl!("mobile", "network-automatic"),
            Self::FOUR_G_FIVE_G => fl!("mobile", "network-4g-5g"),
            Self::FIVE_G_ONLY => fl!("mobile", "network-5g-only"),
            Self::THREE_G_FOUR_G => fl!("mobile", "network-3g-4g"),
            Self::FOUR_G_ONLY => fl!("mobile", "network-4g-only"),
            Self::THREE_G_ONLY => fl!("mobile", "network-3g-only"),
            _ => fl!(
                "mobile",
                "network-custom",
                allowed = self.allowed_label(),
                preferred = self.preferred_label()
            ),
        }
    }

    fn allowed_label(self) -> &'static str {
        match self.allowed {
            MODE_3G => "3G",
            MODE_4G => "4G",
            MODE_5G => "5G",
            MODE_3G_4G => "3G / 4G",
            MODE_3G_5G => "3G / 5G",
            MODE_4G_5G => "4G / 5G",
            MODE_3G_4G_5G => "3G / 4G / 5G",
            _ => "Mobile",
        }
    }

    fn preferred_label(self) -> &'static str {
        match self.preferred {
            MODE_3G => "3G",
            MODE_4G => "4G",
            MODE_5G => "5G",
            _ => "Automatic",
        }
    }

    fn menu_order(self) -> u8 {
        match self {
            Self::AUTOMATIC => 0,
            Self::FOUR_G_FIVE_G => 1,
            Self::FIVE_G_ONLY => 2,
            Self::THREE_G_FOUR_G => 3,
            Self::FOUR_G_ONLY => 4,
            Self::THREE_G_ONLY => 5,
            _ => 6,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModemModes {
    interface: String,
    current: NetworkMode,
    supported: Vec<NetworkMode>,
}

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem",
    default_service = "org.freedesktop.ModemManager1"
)]
trait Modem {
    #[zbus(property, name = "PrimaryPort")]
    fn primary_port(&self) -> zbus::Result<String>;

    #[zbus(property, name = "SupportedModes")]
    fn supported_modes(&self) -> zbus::Result<Vec<(u32, u32)>>;

    #[zbus(property, name = "CurrentModes")]
    fn current_modes(&self) -> zbus::Result<(u32, u32)>;

    #[zbus(name = "SetCurrentModes")]
    async fn set_current_modes(&self, modes: (u32, u32)) -> zbus::Result<()>;
}

#[derive(Clone, Debug)]
pub enum Message {
    AddProfile,
    Activate {
        connection: network_manager::devices::DeviceConnection,
        device: Arc<DeviceInfo>,
    },
    CancelDialog,
    Deactivate(ConnectionId),
    Error(String),
    NetworkManagerConnect(nmrs::NetworkManager),
    Refresh,
    RemoveProfile(ConnectionId),
    RemoveProfileRequest(ConnectionId),
    SelectDevice(Arc<DeviceInfo>),
    SetDefault {
        uuid: ConnectionId,
        device_profiles: Vec<ConnectionId>,
    },
    SetNetworkMode {
        interface: String,
        mode: NetworkMode,
    },
    NetworkModeUpdated {
        mode: NetworkMode,
        result: Result<(), String>,
    },
    SetRadio(bool),
    Settings(ConnectionId),
    Surface(surface::Action),
    Update {
        devices: Vec<DeviceInfo>,
        modem_details: Vec<details::ModemDetails>,
        modem_modes: Vec<ModemModes>,
        radio_enabled: bool,
    },
    SignalPollingStopped,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum MobileDialog {
    RemoveProfile(ConnectionId),
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

#[derive(Debug, Default)]
pub struct Page {
    entity: page::Entity,
    active_device: Option<Arc<DeviceInfo>>,
    devices: Vec<Arc<DeviceInfo>>,
    dialog: Option<MobileDialog>,
    network_mode_change_in_progress: bool,
    network_mode_change_failed: bool,
    modem_details: Vec<details::ModemDetails>,
    modem_modes: Vec<ModemModes>,
    network_manager: Option<nmrs::NetworkManager>,
    radio_enabled: bool,
    rejected_network_modes: Vec<NetworkMode>,
    signal_polling_rates: Vec<(String, u32)>,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> cosmic_settings_page::Info {
        page::Info::new("mobile", "network-cellular-symbolic")
            .title(fl!("mobile"))
            .description(fl!("xdg-entry-mobile-comment"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(devices_view())])
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

    fn header_view(&self) -> Option<Element<'_, crate::pages::Message>> {
        Some(
            widget::button::standard(fl!("add-network", "profile"))
                .trailing_icon(icon::from_name("list-add-symbolic"))
                .on_press(Message::AddProfile)
                .apply(widget::container)
                .width(Length::Fill)
                .align_x(Alignment::End)
                .apply(Element::from)
                .map(crate::pages::Message::Mobile),
        )
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        if self.network_manager.is_none() {
            return cosmic::task::future(async move {
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
        let signal_polling_rates = std::mem::take(&mut self.signal_polling_rates);
        self.active_device = None;
        self.devices.clear();
        self.dialog = None;
        self.modem_details.clear();
        self.network_manager = None;
        if signal_polling_rates.is_empty() {
            Task::none()
        } else {
            cosmic::task::future(async move {
                details::restore_signal_polling(&signal_polling_rates).await;
                crate::pages::Message::Mobile(Message::SignalPollingStopped)
            })
        }
    }

    fn subscription(&self, _core: &cosmic::Core) -> Subscription<crate::pages::Message> {
        cosmic::iced::time::every(Duration::from_secs(details::SIGNAL_REFRESH_SECONDS.into()))
            .map(|_| crate::pages::Message::Mobile(Message::Refresh))
    }

    fn title(&self) -> Option<&str> {
        self.active_device
            .as_ref()
            .map(|device| device.interface.as_str())
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::NetworkManagerConnect(network_manager) => {
                self.network_manager = Some(network_manager.clone());
                return refresh(network_manager);
            }
            Message::CancelDialog => self.dialog = None,
            Message::Refresh => {
                if let Some(network_manager) = self.network_manager.clone() {
                    return refresh(network_manager);
                }
            }
            Message::Update {
                devices,
                modem_details,
                modem_modes,
                radio_enabled,
            } => {
                self.radio_enabled = radio_enabled;
                self.devices = devices.into_iter().map(Arc::new).collect();
                for (interface, previous_rate) in modem_details.iter().filter_map(|details| {
                    details
                        .started_signal_polling
                        .map(|rate| (details.interface.as_str(), rate))
                }) {
                    if !self
                        .signal_polling_rates
                        .iter()
                        .any(|(active, _)| active == interface)
                    {
                        self.signal_polling_rates
                            .push((interface.to_owned(), previous_rate));
                    }
                }
                self.modem_details = modem_details;
                self.modem_modes = modem_modes;
                self.update_active_device();
            }
            Message::SelectDevice(device) => self.active_device = Some(device),
            Message::SetRadio(enabled) => {
                if let Some(network_manager) = self.network_manager.clone() {
                    return cosmic::task::future(async move {
                        network_manager
                            .set_wwan_enabled(enabled)
                            .await
                            .map(|_| Message::Refresh)
                            .unwrap_or_else(|why| Message::Error(why.to_string()))
                    });
                }
            }
            Message::Activate { connection, device } => {
                if let Some(network_manager) = self.network_manager.clone() {
                    return cosmic::task::future(async move {
                        network_manager::activate_connection(
                            &network_manager,
                            connection.path,
                            device.path.clone(),
                            slash_path(),
                        )
                        .await
                        .map(|_| Message::Refresh)
                        .unwrap_or_else(|why| Message::Error(why.to_string()))
                    });
                }
            }
            Message::Deactivate(uuid) => {
                if let Some(network_manager) = self.network_manager.clone() {
                    return cosmic::task::future(async move {
                        network_manager::deactivate_by_uuid(&network_manager, uuid.as_ref())
                            .await
                            .map(|_| Message::Refresh)
                            .unwrap_or_else(|why| Message::Error(why.to_string()))
                    });
                }
            }
            Message::AddProfile => {
                return cosmic::task::future(async move {
                    let _ = super::nm_add_mobile().await;
                    Message::Refresh
                });
            }
            Message::RemoveProfileRequest(uuid) => {
                self.dialog = Some(MobileDialog::RemoveProfile(uuid));
            }
            Message::RemoveProfile(uuid) => {
                self.dialog = None;
                if let Some(network_manager) = self.network_manager.clone() {
                    return cosmic::task::future(async move {
                        network_manager
                            .delete_saved_connection(uuid.as_ref())
                            .await
                            .map(|_| Message::Refresh)
                            .unwrap_or_else(|why| Message::Error(why.to_string()))
                    });
                }
            }
            Message::SetDefault {
                uuid,
                device_profiles,
            } => {
                if let Some(network_manager) = self.network_manager.clone() {
                    return cosmic::task::future(async move {
                        set_default_profile(&network_manager, uuid.as_ref(), &device_profiles)
                            .await
                            .map(|_| Message::Refresh)
                            .unwrap_or_else(Message::Error)
                    });
                }
            }
            Message::SetNetworkMode { interface, mode } => {
                if self.network_mode_change_in_progress {
                    return Task::none();
                }
                self.network_mode_change_in_progress = true;
                self.network_mode_change_failed = false;
                return cosmic::task::future(async move {
                    Message::NetworkModeUpdated {
                        mode,
                        result: set_network_mode(&interface, mode).await,
                    }
                });
            }
            Message::NetworkModeUpdated { mode, result } => {
                self.network_mode_change_in_progress = false;
                self.network_mode_change_failed = result.is_err();
                if let Err(why) = result {
                    if !self.rejected_network_modes.contains(&mode) {
                        self.rejected_network_modes.push(mode);
                    }
                    tracing::warn!(why, "failed to apply preferred network type");
                }
                if let Some(network_manager) = self.network_manager.clone() {
                    return refresh(network_manager);
                }
            }
            Message::SignalPollingStopped => {}
            Message::Settings(uuid) => {
                return cosmic::task::future(async move {
                    let _ = super::nm_edit_connection(uuid.as_ref()).await;
                    Message::Refresh
                });
            }
            Message::Surface(action) => {
                return cosmic::task::message(crate::app::Message::Surface(action));
            }
            Message::Error(why) => tracing::error!(why),
        }

        Task::none()
    }

    fn update_active_device(&mut self) {
        if let Some(active) = self.active_device.as_ref() {
            self.active_device = self
                .devices
                .iter()
                .find(|device| device.path == active.path)
                .map(Arc::clone);
        }
    }

    fn device_view<'a>(&'a self, device: &'a Arc<DeviceInfo>) -> Element<'a, Message> {
        let active_uuid = device
            .active_connection
            .as_ref()
            .map(|(connection, _)| connection.uuid.as_ref());
        let connection_status = active_uuid
            .and_then(|uuid| {
                device
                    .known_connections
                    .iter()
                    .find(|connection| connection.uuid.as_ref() == uuid)
            })
            .map(|connection| {
                fl!(
                    "mobile",
                    "status-connected",
                    profile = connection.id.as_str()
                )
            })
            .unwrap_or_else(|| fl!("mobile", "status-disconnected"));
        let connection = widget::settings::section()
            .title(fl!("mobile", "connection"))
            .add(
                widget::settings::item::builder(fl!("mobile", "data"))
                    .description(connection_status)
                    .toggler(self.radio_enabled, Message::SetRadio),
            )
            .add(widget::settings::item_row(vec![
                widget::text::body(fl!(
                    "mobile",
                    "interface",
                    interface = device.interface.as_str()
                ))
                .into(),
            ]));
        let network_mode = self.network_mode_view(device);
        let network_details = self.network_details_view(device);
        let mut profiles = widget::settings::section().title(fl!("mobile", "profiles"));

        if device.known_connections.is_empty() {
            profiles = profiles.add(widget::settings::item_row(vec![
                widget::text::body(fl!("mobile", "no-profiles")).into(),
            ]));
        }

        let default_uuid =
            default_profile(&device.known_connections).map(|profile| profile.uuid.as_ref());
        let device_profiles = device
            .known_connections
            .iter()
            .map(|profile| profile.uuid.clone())
            .collect::<Vec<_>>();
        let mut profiles_sorted = device.known_connections.iter().collect::<Vec<_>>();
        profiles_sorted.sort_by(|left, right| {
            let left_default = Some(left.uuid.as_ref()) == default_uuid;
            let right_default = Some(right.uuid.as_ref()) == default_uuid;
            right_default
                .cmp(&left_default)
                .then_with(|| left.id.cmp(&right.id))
        });

        for profile in profiles_sorted {
            let active = active_uuid.is_some_and(|uuid| uuid == profile.uuid.as_ref())
                && device.state == DeviceState::Activated;
            let is_default = Some(profile.uuid.as_ref()) == default_uuid;
            let action = if active {
                widget::button::text(fl!("disconnect"))
                    .on_press(Message::Deactivate(profile.uuid.clone()))
            } else if self.radio_enabled && device.state != DeviceState::Unavailable {
                let Some(connection) = device
                    .available_connections
                    .iter()
                    .find(|connection| connection.uuid == profile.uuid)
                    .cloned()
                else {
                    continue;
                };

                widget::button::text(fl!("connect")).on_press(Message::Activate {
                    connection,
                    device: Arc::clone(device),
                })
            } else {
                widget::button::text(fl!("connect"))
            };

            let default_action: Element<'_, Message> = if is_default {
                widget::text::body(fl!("mobile", "default"))
                    .align_y(Alignment::Center)
                    .into()
            } else {
                widget::button::text(fl!("mobile", "set-default"))
                    .on_press(Message::SetDefault {
                        uuid: profile.uuid.clone(),
                        device_profiles: device_profiles.clone(),
                    })
                    .into()
            };
            let controls = widget::row::with_capacity(4)
                .push(action)
                .push(default_action)
                .push(
                    widget::button::icon(icon::from_name("emblem-system-symbolic"))
                        .on_press(Message::Settings(profile.uuid.clone())),
                )
                .push(
                    widget::button::icon(icon::from_name("edit-delete-symbolic"))
                        .on_press(Message::RemoveProfileRequest(profile.uuid.clone())),
                )
                .align_y(Alignment::Center)
                .spacing(cosmic::theme::spacing().space_xxs);

            profiles = profiles.add(widget::settings::item_row(vec![
                widget::text::body(&profile.id).into(),
                horizontal_space().into(),
                controls.into(),
            ]));
        }

        widget::column::with_capacity(4)
            .push(connection)
            .push(network_mode)
            .push(network_details)
            .push(profiles)
            .spacing(cosmic::theme::spacing().space_l)
            .into()
    }

    fn network_mode_view<'a>(&'a self, device: &'a Arc<DeviceInfo>) -> Element<'a, Message> {
        let Some(modem_modes) = self
            .modem_modes
            .iter()
            .find(|modes| modes.interface == device.interface)
        else {
            return widget::settings::section()
                .title(fl!("mobile", "network-type"))
                .add(widget::settings::item_row(vec![
                    widget::text::body(fl!("mobile", "network-type-unavailable")).into(),
                ]))
                .into();
        };

        let modes = available_network_modes(
            &modem_modes.supported,
            modem_modes.current,
            &self.rejected_network_modes,
        );

        let labels = modes.iter().map(|mode| mode.label()).collect::<Vec<_>>();
        let selected = modes.iter().position(|mode| *mode == modem_modes.current);
        let interface = device.interface.clone();
        let modes_for_action = modes.clone();
        let item = widget::settings::item::builder(fl!("mobile", "network-preferences"))
            .description(if self.network_mode_change_failed {
                fl!("mobile", "network-type-failed")
            } else {
                modem_modes.current.label()
            });
        let item = if self.network_mode_change_in_progress {
            item.control(widget::text::body(fl!("mobile", "network-type-applying")))
        } else {
            item.control(widget::dropdown::popup_dropdown(
                labels,
                selected,
                move |index| Message::SetNetworkMode {
                    interface: interface.clone(),
                    mode: modes_for_action[index],
                },
                cosmic::iced::window::Id::RESERVED,
                Message::Surface,
                |message| crate::app::Message::PageMessage(crate::pages::Message::Mobile(message)),
            ))
        };

        widget::settings::section()
            .title(fl!("mobile", "network-type"))
            .add(item)
            .into()
    }

    fn network_details_view<'a>(&'a self, device: &'a Arc<DeviceInfo>) -> Element<'a, Message> {
        let Some(details) = self
            .modem_details
            .iter()
            .find(|details| details.interface == device.interface)
        else {
            return widget::settings::section()
                .title(fl!("mobile", "network-details"))
                .add(detail_item(
                    fl!("mobile", "network-details-unavailable"),
                    String::new(),
                ))
                .into();
        };

        let operator = match (&details.operator, &details.operator_code) {
            (Some(name), Some(code)) => format!("{name} ({code})"),
            (Some(name), None) => name.clone(),
            (None, Some(code)) => code.clone(),
            (None, None) => fl!("mobile", "network-details-unavailable"),
        };
        let mut section = widget::settings::section()
            .title(fl!("mobile", "network-details"))
            .add(detail_item(fl!("mobile", "network-operator"), operator))
            .add(detail_item(
                fl!("mobile", "network-cell"),
                details.cell_description(),
            ));

        if !details.lte.is_empty() {
            section = section.add(detail_item(
                fl!("mobile", "network-lte-signal"),
                details.lte.description(),
            ));
        }
        if !details.nr5g.is_empty() {
            section = section.add(detail_item(
                fl!("mobile", "network-5g-signal"),
                details.nr5g.description(),
            ));
        }

        section
            .add(detail_item(
                fl!("mobile", "network-aggregation"),
                fl!("mobile", "network-aggregation-unavailable"),
            ))
            .add(detail_item(
                fl!("mobile", "network-signal-guide"),
                fl!("mobile", "network-signal-guide-description"),
            ))
            .into()
    }
}

fn detail_item<'a>(title: String, description: String) -> Element<'a, Message> {
    let content = widget::column::with_capacity(2)
        .push(widget::text::body(title))
        .push_maybe((!description.is_empty()).then(|| widget::text::caption(description)))
        .spacing(cosmic::theme::spacing().space_xxxs);

    widget::settings::item_row(vec![content.into()]).into()
}

fn default_profile(
    profiles: &[network_manager::devices::KnownDeviceConnection],
) -> Option<&network_manager::devices::KnownDeviceConnection> {
    profiles
        .iter()
        .filter(|profile| profile.autoconnect)
        .max_by_key(|profile| profile.autoconnect_priority)
}

async fn set_default_profile(
    network_manager: &nmrs::NetworkManager,
    uuid: &str,
    device_profiles: &[ConnectionId],
) -> Result<(), String> {
    let profiles = network_manager
        .list_saved_connections()
        .await
        .map_err(|why| why.to_string())?;
    let device_profiles = profiles
        .iter()
        .filter(|profile| is_profile_for_device(profile.uuid.as_str(), device_profiles))
        .collect::<Vec<_>>();

    if !device_profiles.iter().any(|profile| profile.uuid == uuid) {
        return Err(format!(
            "mobile profile {uuid} is not associated with the selected modem"
        ));
    }

    for profile in device_profiles {
        let selected = profile.uuid == uuid;
        let mut patch = nmrs::SettingsPatch::default();
        patch.autoconnect = Some(selected);
        patch.autoconnect_priority = Some(if selected {
            DEFAULT_AUTOCONNECT_PRIORITY
        } else {
            0
        });
        network_manager
            .update_saved_connection(&profile.uuid, patch)
            .await
            .map_err(|why| why.to_string())?;
    }

    Ok(())
}

fn is_profile_for_device(uuid: &str, device_profiles: &[ConnectionId]) -> bool {
    device_profiles
        .iter()
        .any(|device_profile| device_profile.as_ref() == uuid)
}

fn available_network_modes(
    supported: &[NetworkMode],
    current: NetworkMode,
    rejected: &[NetworkMode],
) -> Vec<NetworkMode> {
    let mut modes = supported
        .iter()
        .copied()
        .filter(|mode| *mode == current || !rejected.contains(mode))
        .collect::<Vec<_>>();
    if !modes.contains(&current) {
        modes.insert(0, current);
    }
    modes.sort_by_key(|mode| mode.menu_order());
    modes
}

fn devices_view() -> Section<crate::pages::Message> {
    Section::default().view::<Page>(move |_binder, page, _section| {
        let active_device = page
            .active_device
            .as_ref()
            .or_else(|| (page.devices.len() == 1).then(|| page.devices.first())?);

        match active_device {
            Some(device) => page.device_view(device).map(crate::pages::Message::Mobile),
            None => widget::settings::section()
                .add(widget::settings::item_row(vec![
                    widget::text::body(fl!("mobile", "no-device")).into(),
                ]))
                .apply(Element::from)
                .map(crate::pages::Message::Mobile),
        }
    })
}

fn refresh(network_manager: nmrs::NetworkManager) -> Task<crate::app::Message> {
    cosmic::task::future(async move {
        let (devices, radio, modem_details, modem_modes) = futures::join!(
            network_manager::devices::list(&network_manager, |device_type| {
                matches!(device_type, DeviceType::Modem)
            }),
            network_manager.wwan_state(),
            details::read_modem_details(),
            read_modem_modes(),
        );

        match devices {
            Ok(devices) => Message::Update {
                devices,
                modem_details: modem_details.unwrap_or_else(|why| {
                    tracing::warn!(why, "failed to read mobile network details");
                    Vec::new()
                }),
                modem_modes: modem_modes.unwrap_or_else(|why| {
                    tracing::warn!(why, "failed to read mobile network modes");
                    Vec::new()
                }),
                radio_enabled: radio.map(|state| state.enabled).unwrap_or(false),
            },
            Err(why) => Message::Error(why.to_string()),
        }
    })
}

async fn read_modem_modes() -> Result<Vec<ModemModes>, String> {
    let connection = Connection::system()
        .await
        .map_err(|why| format!("connect to ModemManager: {why}"))?;
    let manager = ObjectManagerProxy::builder(&connection)
        .destination(MODEM_MANAGER_SERVICE)
        .map_err(|why| format!("build ModemManager manager: {why}"))?
        .path(MODEM_MANAGER_PATH)
        .map_err(|why| format!("build ModemManager manager: {why}"))?
        .build()
        .await
        .map_err(|why| format!("connect to ModemManager: {why}"))?;
    let paths = manager
        .get_managed_objects()
        .await
        .map_err(|why| format!("enumerate ModemManager devices: {why}"))?
        .into_iter()
        .filter(|(_, interfaces)| interfaces.contains_key(MODEM_INTERFACE))
        .map(|(path, _)| path.to_string())
        .collect::<Vec<_>>();

    let mut modes = Vec::new();
    for path in paths {
        if let Ok(mode) = read_modem_mode(&connection, &path).await {
            modes.push(mode);
        }
    }
    Ok(modes)
}

async fn read_modem_mode(connection: &Connection, path: &str) -> Result<ModemModes, String> {
    let proxy = ModemProxy::builder(connection)
        .path(path)
        .map_err(|why| format!("build modem proxy: {why}"))?
        .build()
        .await
        .map_err(|why| format!("read modem properties: {why}"))?;
    let (interface, supported, current) = futures::join!(
        proxy.primary_port(),
        proxy.supported_modes(),
        proxy.current_modes(),
    );
    let (allowed, preferred) = current.map_err(|why| format!("read modem modes: {why}"))?;

    Ok(ModemModes {
        interface: interface.map_err(|why| format!("read modem interface: {why}"))?,
        current: NetworkMode { allowed, preferred },
        supported: supported
            .map_err(|why| format!("read supported modem modes: {why}"))?
            .into_iter()
            .map(|(allowed, preferred)| NetworkMode { allowed, preferred })
            .collect(),
    })
}

async fn set_network_mode(interface: &str, mode: NetworkMode) -> Result<(), String> {
    let connection = Connection::system()
        .await
        .map_err(|why| format!("connect to ModemManager: {why}"))?;
    let manager = ObjectManagerProxy::builder(&connection)
        .destination(MODEM_MANAGER_SERVICE)
        .map_err(|why| format!("build ModemManager manager: {why}"))?
        .path(MODEM_MANAGER_PATH)
        .map_err(|why| format!("build ModemManager manager: {why}"))?
        .build()
        .await
        .map_err(|why| format!("connect to ModemManager: {why}"))?;
    let paths = manager
        .get_managed_objects()
        .await
        .map_err(|why| format!("enumerate ModemManager devices: {why}"))?
        .into_iter()
        .filter(|(_, interfaces)| interfaces.contains_key(MODEM_INTERFACE))
        .map(|(path, _)| path.to_string())
        .collect::<Vec<_>>();

    for path in paths {
        let proxy = ModemProxy::builder(&connection)
            .path(path.as_str())
            .map_err(|why| format!("build modem proxy: {why}"))?
            .build()
            .await
            .map_err(|why| format!("read modem properties: {why}"))?;
        if proxy
            .primary_port()
            .await
            .map_err(|why| format!("read modem interface: {why}"))?
            == interface
        {
            return proxy
                .set_current_modes((mode.allowed, mode.preferred))
                .await
                .map_err(|why| format!("set preferred network type: {why}"));
        }
    }

    Err(format!("no modem found for network interface {interface}"))
}

fn slash_path() -> nmrs::raw::zvariant::OwnedObjectPath {
    nmrs::raw::zvariant::OwnedObjectPath::try_from("/").expect("slash is a valid object path")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile(
        id: &str,
        autoconnect: bool,
        autoconnect_priority: i32,
    ) -> network_manager::devices::KnownDeviceConnection {
        network_manager::devices::KnownDeviceConnection {
            id: id.to_owned(),
            uuid: Arc::from(id),
            autoconnect,
            autoconnect_priority,
        }
    }

    #[test]
    fn default_profile_uses_the_highest_autoconnect_priority() {
        let profiles = [
            profile("backup", true, 0),
            profile("manual", false, 100),
            profile("preferred", true, DEFAULT_AUTOCONNECT_PRIORITY),
        ];

        assert_eq!(
            default_profile(&profiles).map(|profile| profile.id.as_str()),
            Some("preferred")
        );
    }

    #[test]
    fn default_profile_scope_excludes_profiles_from_other_modems() {
        let selected_modem = [Arc::from("profile-on-selected-modem")];

        assert!(is_profile_for_device(
            "profile-on-selected-modem",
            &selected_modem
        ));
        assert!(!is_profile_for_device(
            "profile-on-other-modem",
            &selected_modem
        ));
    }

    #[test]
    fn network_mode_choices_include_every_supported_combination() {
        let additional_supported_mode = NetworkMode {
            allowed: MODE_3G_5G,
            preferred: MODE_5G,
        };
        let supported = [
            NetworkMode::AUTOMATIC,
            NetworkMode::FOUR_G_FIVE_G,
            NetworkMode::FIVE_G_ONLY,
            NetworkMode::THREE_G_FOUR_G,
            NetworkMode::FOUR_G_ONLY,
            NetworkMode::THREE_G_ONLY,
            additional_supported_mode,
        ];

        assert_eq!(
            available_network_modes(&supported, NetworkMode::AUTOMATIC, &[]),
            supported
        );
    }

    #[test]
    fn network_mode_choices_keep_a_supported_nonstandard_current_mode_visible() {
        let current = NetworkMode {
            allowed: MODE_3G_4G_5G,
            preferred: MODE_4G,
        };
        let supported = [current, NetworkMode::AUTOMATIC, NetworkMode::FOUR_G_ONLY];

        assert_eq!(
            available_network_modes(&supported, current, &[]),
            [NetworkMode::AUTOMATIC, NetworkMode::FOUR_G_ONLY, current]
        );
    }

    #[test]
    fn network_mode_choices_hide_a_rejected_mode_but_keep_the_current_one() {
        let supported = [NetworkMode::AUTOMATIC, NetworkMode::FOUR_G_FIVE_G];

        assert_eq!(
            available_network_modes(
                &supported,
                NetworkMode::FOUR_G_FIVE_G,
                &[NetworkMode::AUTOMATIC],
            ),
            [NetworkMode::FOUR_G_FIVE_G]
        );
    }
}
