// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Mobile broadband profiles managed by NetworkManager.

use std::sync::Arc;

use anyhow::Context;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::space::horizontal as horizontal_space;
use cosmic::widget::{self, icon};
use cosmic::{Apply, Element, Task};
use cosmic_settings_page::{self as page, Section, section};
use slotmap::SlotMap;

use super::backend as network_manager;
use super::backend::devices::{DeviceInfo, DeviceState, DeviceType};

pub type ConnectionId = Arc<str>;

const DEFAULT_AUTOCONNECT_PRIORITY: i32 = 100;

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
    SetDefault(ConnectionId),
    SetRadio(bool),
    Settings(ConnectionId),
    Update {
        devices: Vec<DeviceInfo>,
        radio_enabled: bool,
    },
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
    network_manager: Option<nmrs::NetworkManager>,
    radio_enabled: bool,
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
        self.active_device = None;
        self.devices.clear();
        self.dialog = None;
        self.network_manager = None;
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
                radio_enabled,
            } => {
                self.radio_enabled = radio_enabled;
                self.devices = devices.into_iter().map(Arc::new).collect();
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
            Message::SetDefault(uuid) => {
                if let Some(network_manager) = self.network_manager.clone() {
                    return cosmic::task::future(async move {
                        set_default_profile(&network_manager, uuid.as_ref())
                            .await
                            .map(|_| Message::Refresh)
                            .unwrap_or_else(Message::Error)
                    });
                }
            }
            Message::Settings(uuid) => {
                return cosmic::task::future(async move {
                    let _ = super::nm_edit_connection(uuid.as_ref()).await;
                    Message::Refresh
                });
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
            .map(|connection| fl!("mobile", "status-connected", profile = connection.id.as_str()))
            .unwrap_or_else(|| fl!("mobile", "status-disconnected"));
        let connection = widget::settings::section()
            .title(fl!("mobile", "connection"))
            .add(
                widget::settings::item::builder(fl!("mobile", "data"))
                    .description(connection_status)
                    .toggler(self.radio_enabled, Message::SetRadio),
            )
            .add(widget::settings::item_row(vec![
                widget::text::body(fl!("mobile", "interface", interface = device.interface.as_str()))
                    .into(),
            ]));
        let mut profiles = widget::settings::section()
            .title(fl!("mobile", "profiles"));

        if device.known_connections.is_empty() {
            profiles = profiles.add(widget::settings::item_row(vec![
                widget::text::body(fl!("mobile", "no-profiles")).into(),
            ]));
        }

        let default_uuid = default_profile(&device.known_connections).map(|profile| profile.uuid.as_ref());
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
                    .on_press(Message::SetDefault(profile.uuid.clone()))
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

        widget::column::with_capacity(2)
            .push(connection)
            .push(profiles)
            .spacing(cosmic::theme::spacing().space_l)
            .into()
    }
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
) -> Result<(), String> {
    let profiles = network_manager
        .list_saved_connections()
        .await
        .map_err(|why| why.to_string())?;
    let mobile_profiles = profiles
        .iter()
        .filter(|profile| matches!(profile.connection_type.as_str(), "gsm" | "cdma"))
        .collect::<Vec<_>>();

    if !mobile_profiles.iter().any(|profile| profile.uuid == uuid) {
        return Err(format!("mobile profile {uuid} was not found"));
    }

    for profile in mobile_profiles {
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

fn devices_view() -> Section<crate::pages::Message> {
    Section::default().view::<Page>(move |_binder, page, _section| {
        let active_device = page
            .active_device
            .as_ref()
            .or_else(|| (page.devices.len() == 1).then(|| page.devices.first())?);

        match active_device {
            Some(device) => page
                .device_view(device)
                .map(crate::pages::Message::Mobile),
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
        let (devices, radio) = futures::join!(
            network_manager::devices::list(&network_manager, |device_type| {
                matches!(device_type, DeviceType::Modem)
            }),
            network_manager.wwan_state(),
        );

        match devices {
            Ok(devices) => Message::Update {
                devices,
                radio_enabled: radio.map(|state| state.enabled).unwrap_or(false),
            },
            Err(why) => Message::Error(why.to_string()),
        }
    })
}

fn slash_path() -> nmrs::raw::zvariant::OwnedObjectPath {
    nmrs::raw::zvariant::OwnedObjectPath::try_from("/").expect("slash is a valid object path")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile(id: &str, autoconnect: bool, autoconnect_priority: i32) -> network_manager::devices::KnownDeviceConnection {
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
}
