// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::{color, Alignment, Length};
use cosmic::iced_core::text::Wrapping;
use cosmic::widget::{self, settings, text};
use cosmic::Task;
use cosmic::{Apply, Element};
use cosmic_settings_page::{self as page, section, Section};
use futures::channel::oneshot;
use slab::Slab;
use slotmap::SlotMap;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use zbus::zvariant::OwnedObjectPath;

mod agent;
mod backend;
pub use backend::*;
mod subscription;

enum Dialog {
    // RequestAuthorization {
    //     device: OwnedObjectPath,
    //     response: oneshot::Sender<bool>,
    // },
    RequestConfirmation {
        device: String,
        passkey: u32,
        response: oneshot::Sender<bool>,
    },
    // RequestPasskey {
    //     device: OwnedObjectPath,
    //     response: oneshot::Sender<Option<u32>>,
    // },
    // RequestPinCode {
    //     device: OwnedObjectPath,
    //     response: oneshot::Sender<Option<String>>,
    // },
}

#[derive(Default)]
pub struct Page {
    active: Active,
    connection: Option<zbus::Connection>,
    dialog: Option<Dialog>,
    adapters: HashMap<OwnedObjectPath, Adapter>,
    selected_adapter: Option<OwnedObjectPath>,
    heading: String,
    devices: HashMap<OwnedObjectPath, Device>,
    popup_setting: bool,
    popup_device: Option<OwnedObjectPath>,
    show_device_without_alias: bool,
    subscription: Option<tokio::sync::oneshot::Sender<()>>,
}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("bluetooth", "bluetooth-symbolic")
            .title(fl!("bluetooth"))
            .description(fl!("bluetooth", "desc"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(status()),
            sections.insert(multiple_adapter()),
            sections.insert(connected_devices()),
            sections.insert(available_devices()),
        ])
    }

    fn on_enter(
        &mut self,
        sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> cosmic::Task<crate::pages::Message> {
        // TODO start stream for new device
        cosmic::command::future(async move {
            match zbus::Connection::system().await {
                Ok(connection) => Message::DBusConnect(connection, sender),
                Err(why) => Message::DBusError(why.to_string()),
            }
        })
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some(cancel) = self.subscription.take() {
            _ = cancel.send(());
        }

        if let Some(connection) = self.connection.take() {
            tokio::spawn(async move {
                _ = agent::unregister(connection).await;
            });
        }

        self.adapters.clear();
        self.selected_adapter = None;
        self.devices.clear();
        self.popup_device = None;
        self.popup_setting = false;
        self.show_device_without_alias = false;

        Task::none()
    }

    fn dialog(&self) -> Option<Element<'_, crate::pages::Message>> {
        match self.dialog.as_ref()? {
            Dialog::RequestConfirmation {
                device, passkey, ..
            } => {
                let description = widget::text::body(fl!(
                    "bluetooth-confirm-pin",
                    "description",
                    device = device
                ))
                .wrapping(Wrapping::Word);

                let pin = widget::text::title1(itoa::Buffer::new().format(*passkey).to_owned())
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .wrapping(Wrapping::None);

                let control = widget::column::with_capacity(2)
                    .push(description)
                    .push(pin)
                    .spacing(cosmic::theme::active().cosmic().spacing.space_xxs);

                let confirm_button =
                    widget::button::suggested(fl!("confirm")).on_press(Message::PinConfirm);

                let cancel_button =
                    widget::button::standard(fl!("cancel")).on_press(Message::PinCancel);

                let dialog = widget::dialog(fl!("bluetooth-confirm-pin"))
                    .control(control)
                    .primary_action(confirm_button)
                    .secondary_action(cancel_button)
                    .apply(Element::from)
                    .map(Into::into);

                Some(dialog)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    AddedAdapter(OwnedObjectPath, Adapter),
    AddedDevice(OwnedObjectPath, Device),
    Agent(Arc<bluez_zbus::agent1::Message>),
    ConnectDevice(OwnedObjectPath),
    DBusConnect(
        zbus::Connection,
        tokio::sync::mpsc::Sender<crate::pages::Message>,
    ),
    DBusError(String),
    DeviceFailed(OwnedObjectPath),
    DisconnectDevice(OwnedObjectPath),
    ForgetDevice(OwnedObjectPath),
    PinCancel,
    PinConfirm,
    PopupDevice(Option<OwnedObjectPath>),
    PopupSetting(bool),
    Nop,
    RemovedAdapter(OwnedObjectPath),
    RemovedDevice(OwnedObjectPath),
    SelectAdapter(Option<OwnedObjectPath>),
    SetActive(bool),
    SetAdapters(HashMap<OwnedObjectPath, Adapter>),
    SetDevices(HashMap<OwnedObjectPath, Device>),
    ShowDeviceWithoutAlias(bool),
    UpdatedAdapter(OwnedObjectPath, Vec<AdapterUpdate>),
    UpdatedDevice(OwnedObjectPath, Vec<DeviceUpdate>),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Bluetooth(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Bluetooth(message)
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::Message> {
        let span = tracing::span!(tracing::Level::INFO, "bluetooth::update");
        let _span = span.enter();

        match message {
            Message::Agent(message) => {
                let Some(message) = Arc::into_inner(message) else {
                    return Task::none();
                };

                match message {
                    bluez_zbus::agent1::Message::RequestAuthorization { response, .. } => {
                        _ = response.send(true);
                    }

                    bluez_zbus::agent1::Message::RequestConfirmation {
                        device,
                        passkey,
                        response,
                    } => {
                        let device = self.devices.get(&device).map_or_else(
                            || device.to_string(),
                            |device| device.alias_or_addr().to_owned(),
                        );

                        self.dialog = Some(Dialog::RequestConfirmation {
                            device,
                            passkey,
                            response,
                        });
                    }

                    bluez_zbus::agent1::Message::RequestPasskey { response, .. } => {
                        _ = response.send(None);
                    }

                    bluez_zbus::agent1::Message::RequestPinCode { response, .. } => {
                        _ = response.send(None);
                    }

                    bluez_zbus::agent1::Message::Cancel => {
                        self.dialog = None;
                    }

                    _ => (),
                }
            }

            Message::PinCancel => {
                if let Some(Dialog::RequestConfirmation { response, .. }) = self.dialog.take() {
                    _ = response.send(false);
                }
            }

            Message::PinConfirm => {
                if let Some(Dialog::RequestConfirmation { response, .. }) = self.dialog.take() {
                    _ = response.send(true);
                }
            }

            Message::SetActive(active) => {
                if let Some(connection) = self.connection.clone() {
                    if let Some((path, adapter)) = self.get_selected_adapter_mut() {
                        adapter.enabled = if active {
                            Active::Enabling
                        } else {
                            Active::Disabling
                        };
                        self.update_status();
                        return cosmic::command::future(change_adapter_status(
                            connection.clone(),
                            path,
                            active,
                        ));
                    }
                    let tasks: Vec<Task<Message>> = self
                        .adapters
                        .iter_mut()
                        .map(|(path, adapter)| {
                            adapter.enabled = if active {
                                Active::Enabling
                            } else {
                                Active::Disabling
                            };
                            cosmic::command::future(change_adapter_status(
                                connection.clone(),
                                path.clone(),
                                active,
                            ))
                        })
                        .collect();
                    self.update_status();
                    return cosmic::command::batch(tasks);
                }
                tracing::warn!("No DBus connection ready");
            }

            Message::DBusConnect(connection, sender) => {
                self.connection = Some(connection.clone());

                if self.subscription.is_none() {
                    let connection = connection.clone();
                    self.subscription = Some(crate::utils::forward_event_loop(
                        sender,
                        crate::pages::Message::Bluetooth,
                        move |tx| async move {
                            _ = futures::join!(
                                subscription::watch(connection.clone(), tx.clone()),
                                agent::watch(connection, tx),
                            );
                        },
                    ));
                }

                return cosmic::command::future(async move {
                    let result: zbus::Result<HashMap<OwnedObjectPath, Adapter>> = async {
                        futures::future::join_all(
                            bluez_zbus::get_adapters(&connection)
                                .await?
                                .into_iter()
                                .map(|(path, proxy)| async move {
                                    Ok((path.to_owned(), Adapter::from_device(&proxy).await?))
                                }),
                        )
                        .await
                        .into_iter()
                        .collect::<zbus::Result<HashMap<_, _>>>()
                    }
                    .await;
                    match result {
                        Ok(adapters) => Message::SetAdapters(adapters),
                        Err(why) => {
                            tracing::error!("dbus connection failed. {why}");
                            Message::DBusError(fl!(
                                "bluetooth",
                                "dbus-error",
                                why = why.to_string()
                            ))
                        }
                    }
                });
            }
            Message::SetDevices(devices) => {
                self.devices = devices;
            }
            Message::SetAdapters(adapters) => {
                self.adapters = adapters;
                self.update_status();

                if self.selected_adapter.is_none() && self.adapters.len() == 1 {
                    return cosmic::command::message(Message::SelectAdapter(
                        self.adapters.keys().next().cloned(),
                    ));
                }
            }
            Message::AddedDevice(path, device) => {
                tracing::debug!("Device {} added", device.address);
                self.devices.insert(path, device);
            }
            Message::UpdatedDevice(path, update) => {
                if let Some(existing) = self.devices.get_mut(&path) {
                    tracing::debug!("Device {} updated", existing.address);
                    existing.update(update);
                }
            }
            Message::RemovedDevice(path) => {
                tracing::debug!("Device {path} removed");
                self.devices.remove(&path);
            }
            Message::AddedAdapter(path, adapter) => {
                tracing::debug!("Adapter {} added", adapter.address);
                self.adapters.insert(path.clone(), adapter);
                if self.selected_adapter.is_none() {
                    return cosmic::command::message(Message::SelectAdapter(Some(path)));
                }
            }
            Message::UpdatedAdapter(path, update) => {
                if let Some(existing) = self.adapters.get_mut(&path) {
                    tracing::debug!("Adapter {} updated: {update:#?}", existing.address);
                    existing.update(update);
                }
                self.update_status();
                if let Some(connection) = self.connection.clone() {
                    match self.get_selected_adapter_mut() {
                        Some((path, existing))
                            if existing.enabled == Active::Enabled
                                && existing.scanning == Active::Disabled =>
                        {
                            existing.scanning = Active::Enabling;
                            return cosmic::command::future(start_discovery(connection, path));
                        }
                        _ => {}
                    }
                } else {
                    tracing::warn!("No DBus connection ready");
                }
            }
            Message::RemovedAdapter(path) => {
                tracing::debug!("Device {path} removed");
                self.adapters.remove(&path);
                if self.selected_adapter == Some(path) {
                    self.selected_adapter = None;
                }
            }
            Message::PopupDevice(popup) => {
                self.popup_device = popup;
            }
            Message::PopupSetting(popup) => {
                self.popup_setting = popup;
            }
            Message::ShowDeviceWithoutAlias(show_device_without_alias) => {
                self.show_device_without_alias = show_device_without_alias;
            }
            Message::SelectAdapter(adapter_maybe) => {
                tracing::debug!("Adapter selected: {adapter_maybe:?}");
                self.selected_adapter = adapter_maybe;
                self.update_status();
                if let Some(connection) = self.connection.as_ref() {
                    let connection = connection.clone();
                    if let Some((path, adapter)) = self.get_selected_adapter_mut() {
                        let mut fut: Vec<Task<Message>> = vec![cosmic::command::future(
                            get_devices(connection.clone(), path.clone()),
                        )];
                        if adapter.enabled == Active::Enabled
                            && adapter.scanning == Active::Disabled
                        {
                            fut.push(cosmic::command::future(start_discovery(
                                connection,
                                path.clone(),
                            )));
                        }

                        return cosmic::command::batch(fut);
                    }
                } else {
                    tracing::warn!("No DBus connection ready");
                }
            }
            Message::ForgetDevice(path) => {
                tracing::debug!("Forgetting to device {path}");
                self.popup_device = None;
                if self.connection.is_none() {
                    return cosmic::Task::none();
                }
                if let Some(connection) = self.connection.as_ref() {
                    let connection = connection.clone();
                    if let Some(device) = self.devices.get_mut(&path) {
                        device.enabled = Active::Disabling;
                        return cosmic::command::future(forget_device(connection, path.clone()));
                    }
                } else {
                    tracing::warn!("No DBus connection ready");
                }
            }
            Message::ConnectDevice(path) => {
                tracing::debug!("Connecting device {path}");
                if self.connection.is_none() {
                    return cosmic::Task::none();
                }
                if let Some(connection) = self.connection.as_ref() {
                    let connection = connection.clone();
                    if let Some(device) = self.devices.get_mut(&path) {
                        if matches!(device.enabled, Active::Enabled | Active::Enabling) {
                            return cosmic::Task::none();
                        }
                        device.enabled = Active::Enabling;
                        return cosmic::command::future(connect_device(connection, path));
                    }
                } else {
                    tracing::warn!("No DBus connection ready");
                }
            }
            Message::DisconnectDevice(path) => {
                tracing::debug!("Disconnecting device {path}");
                self.popup_device = None;
                if let Some(connection) = self.connection.as_ref() {
                    let connection = connection.clone();
                    if let Some(device) = self.devices.get_mut(&path) {
                        if matches!(device.enabled, Active::Disabled | Active::Disabling) {
                            return cosmic::Task::none();
                        }
                        device.enabled = Active::Disabling;
                        return cosmic::command::future(disconnect_device(connection, path));
                    }
                } else {
                    tracing::warn!("No DBus connection ready");
                }
            }
            Message::DeviceFailed(path) => {
                tracing::warn!("Failed operation on device {path}");
                if let Some(device) = self.devices.get_mut(&path) {
                    if matches!(device.enabled, Active::Disabled | Active::Disabling) {
                        return cosmic::Task::none();
                    }
                    device.enabled = match device.enabled {
                        Active::Disabling => Active::Enabled,
                        Active::Enabling => Active::Disabled,
                        e => e,
                    };
                }
            }
            Message::Nop => {}
            Message::DBusError(why) => {
                tracing::error!("dbus connection failed. {why}");
            }
        };
        cosmic::Task::none()
    }

    fn update_status(&mut self) {
        if let Some((_, adapter)) = self.get_selected_adapter() {
            self.heading = fl!(
                "bluetooth",
                "status",
                aliases = format!("“{}”", adapter.alias)
            );
        } else {
            self.heading = fl!(
                "bluetooth",
                "status",
                aliases = self
                    .adapters
                    .values()
                    .map(|adapter| format!("“{}”", adapter.alias))
                    .collect::<HashSet<String>>()
                    .into_iter()
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
        self.active = if let Some((_, adapter)) = self.get_selected_adapter() {
            adapter.enabled
        } else {
            self.adapters
                .values()
                .fold(Active::Disabled, |current, adapter| {
                    if current == Active::Enabled || adapter.enabled == Active::Enabled {
                        Active::Enabled
                    } else {
                        Active::Disabled
                    }
                })
        }
    }
    fn adapter_connected(&self, adapter_path: &OwnedObjectPath) -> bool {
        self.devices
            .iter()
            .any(|(path, device)| path.starts_with(adapter_path.as_str()) && device.is_connected())
    }
    fn get_selected_adapter(&self) -> Option<(&'_ OwnedObjectPath, &'_ Adapter)> {
        if let Some(iface) = &self.selected_adapter {
            self.adapters.get_key_value(iface)
        } else {
            None
        }
    }
    fn devices_for_adapter<'a>(
        &'a self,
        adapter_path: &'a OwnedObjectPath,
    ) -> impl Iterator<Item = (&'a OwnedObjectPath, &'a Device)> {
        self.devices.iter().filter_map(|(path, device)| {
            if device.adapter.eq(adapter_path) {
                Some((path, device))
            } else {
                None
            }
        })
    }
    fn get_selected_adapter_mut(&mut self) -> Option<(OwnedObjectPath, &'_ mut Adapter)> {
        if let Some(path) = &self.selected_adapter {
            self.adapters
                .get_mut(path)
                .map(|adapter| (path.to_owned(), adapter))
        } else {
            None
        }
    }
}

fn status() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let bluetooth_heading = descriptions.insert(fl!("bluetooth"));
    let bluetooth_opt_device_without_name =
        descriptions.insert(fl!("bluetooth", "show-device-without-name"));

    Section::default()
        .descriptions(descriptions)
        .show_while::<Page>(|page| !page.adapters.is_empty())
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let status = page
                .get_selected_adapter()
                .map_or(page.active, |(_, adapter)| adapter.enabled);
            widget::list_column()
                .add(settings::item::item_row(vec![
                    if matches!(status, Active::Enabling | Active::Enabled) {
                        widget::column::with_capacity(2)
                            .push(text::body(&descriptions[bluetooth_heading]))
                            .push(text::caption(&page.heading))
                            .into()
                    } else {
                        text::body(&descriptions[bluetooth_heading]).into()
                    },
                    widget::horizontal_space().width(Length::Fill).into(),
                    if page.popup_setting {
                        widget::popover(
                            widget::button::icon(widget::icon::from_name(
                                "preferences-system-symbolic",
                            ))
                            .on_press(Message::PopupSetting(false)),
                        )
                        .position(widget::popover::Position::Bottom)
                        .on_close(Message::PopupSetting(false))
                        .popup({
                            let theme = cosmic::theme::active();
                            let theme = theme.cosmic();
                            widget::container(
                                settings::item::builder(
                                    &descriptions[bluetooth_opt_device_without_name],
                                )
                                .toggler(
                                    page.show_device_without_alias,
                                    Message::ShowDeviceWithoutAlias,
                                ),
                            )
                            .width(Length::Fixed(300.0))
                            .height(Length::Shrink)
                            .padding([theme.space_xs(), theme.space_xxxs()])
                            .class(cosmic::theme::Container::Dialog)
                        })
                        .into()
                    } else {
                        widget::button::icon(widget::icon::from_name("preferences-system-symbolic"))
                            .on_press(Message::PopupSetting(true))
                            .into()
                    },
                    widget::toggler(status == Active::Enabled)
                        .on_toggle(Message::SetActive)
                        .into(),
                ]))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Bluetooth)
        })
}

fn popup_button(message: Option<Message>, text: &str) -> Element<'_, Message> {
    let theme = cosmic::theme::active();
    let theme = theme.cosmic();
    widget::text::body(text)
        .align_y(Alignment::Center)
        .apply(widget::button::custom)
        .padding([theme.space_xxxs(), theme.space_xs()])
        .width(Length::Fill)
        .class(cosmic::theme::Button::MenuItem)
        .on_press_maybe(message)
        .into()
}

fn connected_devices() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        device_connected = fl!("bluetooth", "connected");
        device_connecting = fl!("bluetooth", "connecting");
        device_disconnecting = fl!("bluetooth", "disconnecting");
        device_connect = fl!("bluetooth", "connect");
        device_disconnect = fl!("bluetooth", "disconnect");
        device_forget = fl!("bluetooth", "forget");
    });

    Section::default()
        .title(fl!("bluetooth-paired"))
        .descriptions(descriptions)
        .show_while::<Page>(|page| {
            page.selected_adapter.as_ref().map(|adapter| {
                page.devices_for_adapter(adapter)
                    .any(|(_, device)| device.paired)
            }) == Some(true)
                && page.active != Active::Disabled
        })
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let section = settings::section().title(&section.title);

            page.devices_for_adapter(page.selected_adapter.as_ref().unwrap())
                .filter_map(|(path, device)| {
                    if !device.paired {
                        return None;
                    }
                    if !page.show_device_without_alias && !device.has_alias() {
                        return None;
                    }

                    let device_menu: Element<_> = if page
                        .popup_device
                        .as_deref()
                        .map_or(false, |p| path.as_str() == p.as_str())
                    {
                        widget::popover(
                            widget::button::icon(widget::icon::from_name("view-more-symbolic"))
                                .on_press(Message::PopupDevice(None)),
                        )
                        .position(widget::popover::Position::Bottom)
                        .on_close(Message::PopupDevice(None))
                        .popup({
                            let theme = cosmic::theme::active();
                            let theme = theme.cosmic();
                            widget::container(
                                widget::column()
                                    .push_maybe(device.is_connected().then(|| {
                                        popup_button(
                                            Some(Message::DisconnectDevice(path.clone())),
                                            &descriptions[device_disconnect],
                                        )
                                    }))
                                    .push(popup_button(
                                        Some(Message::ForgetDevice(path.clone())),
                                        &descriptions[device_forget],
                                    )),
                            )
                            .width(Length::Fixed(200.0))
                            .padding(theme.space_xxxs())
                            .class(cosmic::theme::Container::Dialog)
                        })
                        .into()
                    } else {
                        widget::button::icon(widget::icon::from_name("view-more-symbolic"))
                            .on_press(Message::PopupDevice(Some(path.clone())))
                            .into()
                    };

                    Some(settings::item_row(vec![
                        widget::icon::from_name(device.icon).size(16).into(),
                        if let Some(battery) = &device.battery {
                            widget::column::with_capacity(2)
                                .push(text::body(device.alias_or_addr()))
                                .push(text::caption(battery))
                                .into()
                        } else {
                            widget::text(device.alias_or_addr())
                                .wrapping(Wrapping::Word)
                                .into()
                        },
                        widget::horizontal_space().width(Length::Fill).into(),
                        match device.enabled {
                            Active::Enabled => widget::text(&descriptions[device_connected]).into(),
                            Active::Enabling => widget::text(&descriptions[device_connecting])
                                .class(cosmic::theme::Text::Color(color!(128, 128, 128)))
                                .into(),
                            Active::Disabling => widget::text(&descriptions[device_disconnecting])
                                .class(cosmic::theme::Text::Color(color!(128, 128, 128)))
                                .into(),
                            Active::Disabled => widget::button::text(&descriptions[device_connect])
                                .on_press(Message::ConnectDevice(path.clone()))
                                .class(widget::button::ButtonClass::Text)
                                .into(),
                        },
                        device_menu,
                    ]))
                })
                .fold(section, settings::Section::add)
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Bluetooth)
        })
}

fn available_devices() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let device_connecting = descriptions.insert(fl!("bluetooth", "connecting"));

    Section::default()
        .title(fl!("bluetooth-available"))
        .descriptions(descriptions)
        .show_while::<Page>(|page| {
            page.selected_adapter.as_ref().map(|adapter| {
                page.devices_for_adapter(adapter).any(|(_, device)| {
                    !device.paired && (page.show_device_without_alias || device.has_alias())
                })
            }) == Some(true)
                && page.active != Active::Disabled
        })
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let section = settings::section().title(&section.title);

            page.devices_for_adapter(page.selected_adapter.as_ref().unwrap())
                .filter_map(|(path, device)| {
                    if device.paired {
                        return None::<Element<'_, Message>>;
                    }
                    if !page.show_device_without_alias && !device.has_alias() {
                        return None::<Element<'_, Message>>;
                    }

                    let mut items = vec![
                        widget::icon::from_name(device.icon).size(16).into(),
                        text(device.alias_or_addr()).wrapping(Wrapping::Word).into(),
                        widget::horizontal_space().width(Length::Fill).into(),
                    ];

                    if device.enabled == Active::Enabling {
                        items.push(
                            text(&descriptions[device_connecting])
                                .class(cosmic::theme::Text::Color(color!(128, 128, 128)))
                                .into(),
                        );
                    }
                    let theme = cosmic::theme::active();
                    let theme = theme.cosmic();
                    Some(
                        widget::mouse_area(
                            settings::item_row(items).padding([theme.space_xxs(), theme.space_m()]),
                        )
                        .on_press(Message::ConnectDevice(path.clone()))
                        .into(),
                    )
                })
                .fold(section, settings::Section::add)
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Bluetooth)
        })
}

fn multiple_adapter() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let device_connected = descriptions.insert(fl!("bluetooth", "connected"));

    Section::default()
        .title(fl!("bluetooth-adapters"))
        .descriptions(descriptions)
        .show_while::<Page>(|page| page.adapters.len() > 1 && page.selected_adapter.is_none())
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let section = settings::section().title(&section.title);
            let theme = cosmic::theme::active();
            let theme = theme.cosmic();

            page.adapters
                .iter()
                .map(|(path, adapter)| {
                    let mut items = vec![
                        widget::icon::from_name("bluetooth-symbolic")
                            .size(20)
                            .into(),
                        widget::horizontal_space().width(theme.space_xxs()).into(),
                        text(&adapter.alias).wrapping(Wrapping::Word).into(),
                        widget::horizontal_space().width(Length::Fill).into(),
                        widget::icon::from_name("go-next-symbolic").into(),
                    ];
                    if page.adapter_connected(path) {
                        items.insert(
                            4,
                            text(&descriptions[device_connected])
                                .wrapping(Wrapping::Word)
                                .into(),
                        );
                    }
                    widget::mouse_area(settings::item_row(items))
                        .on_press(Message::SelectAdapter(Some(path.clone())))
                })
                .fold(section, settings::Section::add)
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Bluetooth)
        })
}

impl page::AutoBind<crate::pages::Message> for Page {}
