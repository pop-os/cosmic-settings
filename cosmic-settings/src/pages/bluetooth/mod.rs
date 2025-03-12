// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::{color, Alignment, Length};
use cosmic::iced_core::text::Wrapping;
use cosmic::widget::{self, settings, text};
use cosmic::{theme, Apply, Element, Task};
use cosmic_settings_page::{self as page, section, Section};
use cosmic_settings_subscriptions::bluetooth::*;
use futures::channel::oneshot;
use futures::StreamExt;
use slab::Slab;
use slotmap::SlotMap;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use zbus::zvariant::OwnedObjectPath;

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
    // Set to true when the org.bluez dbus service is unknown.
    bluez_service_unknown: bool,
    popup_setting: bool,
    popup_device: Option<OwnedObjectPath>,
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

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        // TODO start stream for new device
        cosmic::task::future(async move {
            match zbus::Connection::system().await {
                Ok(connection) => Message::DBusConnect(connection),
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
                    .spacing(theme::active().cosmic().space_xxs());

                let confirm_button =
                    widget::button::suggested(fl!("confirm")).on_press(Message::PinConfirm);

                let cancel_button =
                    widget::button::standard(fl!("cancel")).on_press(Message::PinCancel);

                let dialog = widget::dialog()
                    .title(fl!("bluetooth-confirm-pin"))
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
    BluetoothEvent(Event),
    ConnectDevice(OwnedObjectPath),
    DBusConnect(zbus::Connection),
    DBusError(String),
    DisconnectDevice(OwnedObjectPath),
    ForgetDevice(OwnedObjectPath),
    PinCancel,
    PinConfirm,
    PopupDevice(Option<OwnedObjectPath>),
    PopupSetting(bool),
    Nop,
    SelectAdapter(Option<OwnedObjectPath>),
    SetActive(bool),
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

impl From<Event> for crate::app::Message {
    fn from(event: Event) -> Self {
        crate::pages::Message::Bluetooth(Message::BluetoothEvent(event)).into()
    }
}

impl From<Event> for crate::pages::Message {
    fn from(event: Event) -> Self {
        crate::pages::Message::Bluetooth(Message::BluetoothEvent(event))
    }
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Message::BluetoothEvent(event)
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::Message> {
        let span = tracing::span!(tracing::Level::INFO, "bluetooth::update");
        let _span = span.enter();

        match message {
            Message::BluetoothEvent(event) => match event {
                Event::DBusError(why) => {
                    tracing::error!(
                        "dbus connection failed. {}",
                        fl!("bluetooth", "dbus-error", why = why.to_string())
                    );
                }
                Event::Ok => {}
                Event::SetDevices(devices) => {
                    self.devices = devices;
                }
                Event::DeviceFailed(path) => {
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
                Event::SetAdapters(adapters) => {
                    self.adapters = adapters;
                    self.update_status();

                    if self.selected_adapter.is_none() && self.adapters.len() == 1 {
                        return cosmic::task::message(Message::SelectAdapter(
                            self.adapters.keys().next().cloned(),
                        ));
                    }
                }
                Event::UpdatedAdapter(path, update) => {
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
                                return cosmic::task::future(start_discovery(connection, path));
                            }
                            _ => {}
                        }
                    } else {
                        tracing::warn!("No DBus connection ready");
                    }
                }
                Event::UpdatedDevice(path, update) => {
                    if let Some(existing) = self.devices.get_mut(&path) {
                        tracing::debug!("Device {} updated", existing.address);
                        existing.update(update);
                    }
                }
                Event::RemovedAdapter(path) => {
                    tracing::debug!("Device {path} removed");
                    self.adapters.remove(&path);
                    if self.selected_adapter == Some(path) {
                        self.selected_adapter = None;
                    }
                }
                Event::RemovedDevice(path) => {
                    tracing::debug!("Device {path} removed");
                    self.devices.remove(&path);
                }
                Event::AddedDevice(path, device) => {
                    tracing::debug!("Device {} added", device.address);
                    self.devices.insert(path, device);
                }
                Event::AddedAdapter(path, adapter) => {
                    tracing::debug!("Adapter {} added", adapter.address);
                    self.adapters.insert(path.clone(), adapter);
                    if self.selected_adapter.is_none() {
                        return cosmic::task::message(Message::SelectAdapter(Some(path)));
                    }
                }
                Event::DBusServiceUnknown => {
                    self.bluez_service_unknown = true;
                }
                Event::Agent(message) => {
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
            },
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
                        return cosmic::task::future(change_adapter_status(
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
                            cosmic::task::future(change_adapter_status(
                                connection.clone(),
                                path.clone(),
                                active,
                            ))
                        })
                        .collect();
                    self.update_status();
                    return cosmic::task::batch(tasks);
                }
                tracing::warn!("No DBus connection ready");
            }

            Message::DBusConnect(connection) => {
                self.connection = Some(connection.clone());

                let get_adapters_fut = get_adapters(connection.clone());

                if self.subscription.is_none() {
                    let connection = connection.clone();

                    let (cancellation, task) = crate::utils::forward_event_loop(move |emitter| {
                        let connection = connection.clone();

                        async move {
                            let (tx, mut rx) = futures::channel::mpsc::channel(1);

                            let watchers = std::pin::pin!(async move {
                                _ = futures::future::join(
                                    subscription::watch(connection.clone(), tx.clone()),
                                    agent::watch(connection, tx),
                                )
                                .await;
                            });

                            let forwarder = std::pin::pin!(async move {
                                while let Some(message) = rx.next().await {
                                    _ = emitter
                                        .emit(crate::pages::Message::Bluetooth(
                                            Message::BluetoothEvent(message),
                                        ))
                                        .await;
                                }
                            });

                            futures::future::select(watchers, forwarder).await;
                        }
                    });

                    self.subscription = Some(cancellation);
                    return cosmic::task::batch(vec![cosmic::task::future(get_adapters_fut), task]);
                } else {
                    return cosmic::task::future(get_adapters_fut);
                }
            }
            Message::PopupDevice(popup) => {
                self.popup_device = popup;
            }
            Message::PopupSetting(popup) => {
                self.popup_setting = popup;
            }
            Message::SelectAdapter(adapter_maybe) => {
                tracing::debug!("Adapter selected: {adapter_maybe:?}");
                self.selected_adapter = adapter_maybe;
                self.update_status();
                if let Some(connection) = self.connection.as_ref() {
                    let connection = connection.clone();
                    if let Some((path, adapter)) = self.get_selected_adapter_mut() {
                        let mut fut: Vec<Task<Message>> = vec![cosmic::task::future(get_devices(
                            connection.clone(),
                            path.clone(),
                        ))];
                        if adapter.enabled == Active::Enabled
                            && adapter.scanning == Active::Disabled
                        {
                            fut.push(cosmic::task::future(start_discovery(
                                connection,
                                path.clone(),
                            )));
                        }

                        return cosmic::task::batch(fut);
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
                        return cosmic::task::future(forget_device(connection, path.clone()));
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
                        return cosmic::task::future(connect_device(connection, path));
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
                        return cosmic::task::future(disconnect_device(connection, path));
                    }
                } else {
                    tracing::warn!("No DBus connection ready");
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

    let bluetooth = descriptions.insert(fl!("bluetooth"));

    Section::default()
        .descriptions(descriptions)
        .show_while::<Page>(|page| !page.adapters.is_empty())
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            if page.bluez_service_unknown {
                return widget::text::body(
                    "The org.bluez DBus service could not be activated. Is bluez installed?",
                )
                .apply(Element::from);
            }

            let status = page
                .get_selected_adapter()
                .map_or(page.active, |(_, adapter)| adapter.enabled);

            let mut bluetooth_toggle = settings::item::builder(&descriptions[bluetooth]);
            if matches!(status, Active::Enabling | Active::Enabled) {
                bluetooth_toggle = bluetooth_toggle.description(&page.heading);
            }

            widget::list_column()
                .add(bluetooth_toggle.control(
                    widget::toggler(status == Active::Enabled).on_toggle(Message::SetActive),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::Bluetooth)
        })
}

fn popup_button(message: Option<Message>, text: &str) -> Element<'_, Message> {
    let theme = theme::active();
    let theme = theme.cosmic();
    widget::text::body(text)
        .align_y(Alignment::Center)
        .apply(widget::button::custom)
        .padding([theme.space_xxxs(), theme.space_xs()])
        .width(Length::Fill)
        .class(theme::Button::MenuItem)
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
                            .padding(theme::active().cosmic().space_xxxs())
                            .class(theme::Container::Dialog)
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
                                .push(text::caption(fl!(
                                    "bluetooth-paired",
                                    "battery",
                                    percentage = battery
                                )))
                                .into()
                        } else {
                            widget::text(device.alias_or_addr())
                                .wrapping(Wrapping::Word)
                                .into()
                        },
                        widget::horizontal_space().into(),
                        match device.enabled {
                            Active::Enabled => widget::text(&descriptions[device_connected]).into(),
                            Active::Enabling => widget::text(&descriptions[device_connecting])
                                .class(theme::Text::Color(color!(128, 128, 128)))
                                .into(),
                            Active::Disabling => widget::text(&descriptions[device_disconnecting])
                                .class(theme::Text::Color(color!(128, 128, 128)))
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
                .apply(Element::from)
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
                page.devices_for_adapter(adapter)
                    .any(|(_, device)| !device.paired)
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

                    if !device.is_known_device_type() && !device.has_alias() {
                        return None::<Element<'_, Message>>;
                    }

                    let mut items = vec![
                        widget::icon::from_name(device.icon).size(16).into(),
                        text(device.alias_or_addr()).wrapping(Wrapping::Word).into(),
                        widget::horizontal_space().into(),
                    ];

                    if device.enabled == Active::Enabling {
                        items.push(
                            text(&descriptions[device_connecting])
                                .class(theme::Text::Color(color!(128, 128, 128)))
                                .into(),
                        );
                    }
                    Some(
                        widget::mouse_area(settings::item_row(items))
                            .on_press(Message::ConnectDevice(path.clone()))
                            .into(),
                    )
                })
                .fold(section, settings::Section::add)
                .apply(Element::from)
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

            page.adapters
                .iter()
                .map(|(path, adapter)| {
                    let mut items = vec![
                        widget::icon::from_name("bluetooth-symbolic")
                            .size(20)
                            .into(),
                        widget::horizontal_space()
                            .width(theme::active().cosmic().space_xxs())
                            .into(),
                        text(&adapter.alias).wrapping(Wrapping::Word).into(),
                        widget::horizontal_space().into(),
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
                .apply(Element::from)
                .map(crate::pages::Message::Bluetooth)
        })
}

impl page::AutoBind<crate::pages::Message> for Page {}
