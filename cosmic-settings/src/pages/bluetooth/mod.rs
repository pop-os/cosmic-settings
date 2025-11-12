// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::{Alignment, Length, color};
use cosmic::iced_core::text::Wrapping;
use cosmic::widget::{self, settings, text};
use cosmic::{Apply, Element, Task, theme};
use cosmic_settings_bluetooth_subscription::*;
use cosmic_settings_page::{self as page, Section, section};
use futures::StreamExt;
use futures::channel::oneshot;
use slab::Slab;
use slotmap::SlotMap;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
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
pub struct Model {
    active: Active,
    adapters: HashMap<OwnedObjectPath, Adapter>,
    selected_adapter: Option<OwnedObjectPath>,
    devices: HashMap<OwnedObjectPath, Device>,
    popup_device: Option<OwnedObjectPath>,
    popup_setting: bool,
}

impl Model {
    pub fn clear(&mut self) {
        self.adapters.clear();
        self.selected_adapter = None;
        self.devices.clear();
        self.popup_device = None;
        self.popup_setting = false;
    }

    /// Updates known adapters, and selects an adapter if no adapter was selected.
    pub fn set_adapters(
        &mut self,
        adapters: HashMap<OwnedObjectPath, Adapter>,
    ) -> Option<OwnedObjectPath> {
        self.adapters = adapters;
        self.update_status();

        if self.selected_adapter.is_none() && self.adapters.len() == 1 {
            return self.adapters.keys().next().cloned();
        }

        None
    }

    fn update_status(&mut self) {
        self.active = if let Some((_, adapter)) = self.get_selected_adapter() {
            adapter.enabled
        } else if self
            .adapters
            .values()
            .any(|adapter| matches!(adapter.enabled, Active::Enabled | Active::Enabling))
        {
            Active::Enabled
        } else {
            Active::Disabled
        }
    }

    // Updates an adapter, and initiates discovery if an existing adapter is enabled.
    fn updated_adapter(
        &mut self,
        path: OwnedObjectPath,
        update: Vec<AdapterUpdate>,
        connection: zbus::Connection,
    ) -> Option<impl Future<Output = Event> + Send + 'static> {
        if let Some(existing) = self.adapters.get_mut(&path) {
            tracing::debug!("Adapter {} updated: {update:#?}", existing.address);
            existing.update(update);
        }

        self.update_status();

        if let Some((path, existing)) = self.get_selected_adapter_mut()
            && existing.enabled == Active::Enabled
            && existing.scanning == Active::Disabled
        {
            existing.scanning = Active::Enabling;
            return Some(start_discovery(connection, path));
        }

        None
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

#[derive(Default)]
pub struct Page {
    model: Model,

    connection: Option<zbus::Connection>,
    dialog: Option<Dialog>,
    heading: String,

    // Set to true when the org.bluez dbus service is unknown.
    bluez_service_unknown: bool,
    service_is_enabled: bool,
    service_is_active: bool,

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
                Err(why) => Message::DBusConnectFailed(why),
            }
        })
        .chain(cosmic::Task::done(Message::SelectAdapter(None).into()))
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        let mut task = Task::none();

        if let Some(cancel) = self.subscription.take() {
            _ = cancel.send(());
        }

        if let Some(connection) = self.connection.take() {
            let adapters = self.model.adapters.clone();

            // Execute within task to ensure it completes before the app exits.
            task = Task::future(async move {
                for (path, _) in adapters {
                    stop_discovery(connection.clone(), path.clone()).await;
                }
                _ = agent::unregister(connection).await;
            })
            .discard();
        }

        self.model.clear();

        task
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
                    .spacing(theme::spacing().space_xxs);

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
    DBusConnectFailed(zbus::Error),
    DisconnectDevice(OwnedObjectPath),
    ForgetDevice(OwnedObjectPath),
    PinCancel,
    PinConfirm,
    PopupDevice(Option<OwnedObjectPath>),
    PopupSetting(bool),
    SelectAdapter(Option<OwnedObjectPath>),
    ServiceActivate,
    ServiceEnable,
    SetActive(bool),
    UpdateStatus,
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
                    tracing::debug!("bluetooth dbus error {why:?}");
                    tracing::error!(
                        "bluetooth service error {}",
                        fl!("bluetooth", "dbus-error", why = why.to_string())
                    );
                }

                Event::NameHasNoOwner => {
                    self.connection = None;
                    self.service_is_active = false;
                    self.service_is_enabled = false;
                    if let Some(abort_handle) = self.subscription.take() {
                        _ = abort_handle.send(());
                    }
                }

                Event::Ok => {}

                Event::SetDevices(devices) => {
                    self.model.devices = devices;
                }

                Event::DeviceFailed(path) => {
                    tracing::warn!("Failed operation on device {path}");
                    if let Some(device) = self.model.devices.get_mut(&path) {
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
                    let select_adapter = self.model.set_adapters(adapters);

                    if let Some((_, adapter)) = self.model.get_selected_adapter() {
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
                                .model
                                .adapters
                                .values()
                                .map(|adapter| format!("“{}”", adapter.alias))
                                .collect::<HashSet<String>>()
                                .into_iter()
                                .collect::<Vec<String>>()
                                .join(", ")
                        );
                    }

                    if let Some(adapter) = select_adapter {
                        return cosmic::task::message(Message::SelectAdapter(Some(adapter)));
                    }
                }

                Event::UpdatedAdapter(path, update) => {
                    if let Some(existing) = self.model.adapters.get_mut(&path) {
                        tracing::debug!("Adapter {} updated: {update:#?}", existing.address);
                        existing.update(update.clone());
                    }

                    self.model.update_status();
                    if let Some(connection) = self.connection.clone() {
                        if let Some(discovery_future) =
                            self.model.updated_adapter(path, update, connection)
                        {
                            return cosmic::task::future(discovery_future);
                        }
                    } else {
                        tracing::warn!("No DBus connection ready");
                    }
                }

                Event::UpdatedDevice(path, update) => {
                    if let Some(existing) = self.model.devices.get_mut(&path) {
                        tracing::debug!("Device {} updated", existing.address);
                        existing.update(update);
                    }
                }

                Event::RemovedAdapter(path) => {
                    tracing::debug!("Device {path} removed");
                    self.model.adapters.remove(&path);
                    if self.model.selected_adapter == Some(path) {
                        self.model.selected_adapter = None;
                    }
                }

                Event::RemovedDevice(path) => {
                    tracing::debug!("Device {path} removed");
                    self.model.devices.remove(&path);
                }

                Event::AddedDevice(path, device) => {
                    tracing::debug!("Device {} added", device.address);
                    self.model.devices.insert(path, device);
                }

                Event::AddedAdapter(path, adapter) => {
                    tracing::debug!("Adapter {} added", adapter.address);
                    self.model.adapters.insert(path.clone(), adapter);
                    if self.model.selected_adapter.is_none() {
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
                            let device = self.model.devices.get(&device).map_or_else(
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
                    if let Some((path, adapter)) = self.model.get_selected_adapter_mut() {
                        adapter.enabled = if active {
                            Active::Enabling
                        } else {
                            Active::Disabling
                        };

                        return cosmic::task::future(change_adapter_status(
                            connection.clone(),
                            path,
                            active,
                        ))
                        .then(|event| {
                            if matches!(event, Event::Ok) {
                                Task::none()
                            } else {
                                Task::done(event.into())
                            }
                        })
                        .chain(Task::done(Message::UpdateStatus.into()));
                    }
                    let tasks: Vec<Task<Message>> = self
                        .model
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
                            .then(|event| {
                                if matches!(event, Event::Ok) {
                                    Task::none()
                                } else {
                                    Task::done(event.into())
                                }
                            })
                        })
                        .collect();

                    return cosmic::task::batch(tasks)
                        .chain(Task::done(Message::UpdateStatus.into()));
                }
                tracing::warn!("No DBus connection ready");
            }

            Message::UpdateStatus => {
                self.model.update_status();
            }

            Message::DBusConnect(connection) => {
                self.service_is_active = systemd::is_bluetooth_active();
                self.service_is_enabled = systemd::is_bluetooth_enabled();
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
                self.model.popup_device = popup;
            }

            Message::PopupSetting(popup) => {
                self.model.popup_setting = popup;
            }

            Message::SelectAdapter(adapter_maybe) => {
                tracing::debug!("Adapter selected: {adapter_maybe:?}");
                self.model.selected_adapter = adapter_maybe;
                self.model.update_status();
                let Some(connection) = self.connection.as_ref() else {
                    tracing::error!("No DBus connection ready");
                    return Task::none();
                };

                let connection = connection.clone();
                if let Some((path, adapter)) = self.model.get_selected_adapter_mut() {
                    let mut fut: Vec<Task<Message>> = vec![cosmic::task::future(get_devices(
                        connection.clone(),
                        path.clone(),
                    ))];
                    if adapter.enabled == Active::Enabled && adapter.scanning == Active::Disabled {
                        fut.push(cosmic::task::future(start_discovery(
                            connection,
                            path.clone(),
                        )));
                    }

                    return cosmic::task::batch(fut);
                }
            }

            Message::ForgetDevice(path) => {
                tracing::debug!("Forgetting to device {path}");
                self.model.popup_device = None;
                if self.connection.is_none() {
                    return cosmic::Task::none();
                }
                if let Some(connection) = self.connection.as_ref() {
                    let connection = connection.clone();
                    if let Some(device) = self.model.devices.get_mut(&path) {
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
                    if let Some(device) = self.model.devices.get_mut(&path) {
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
                self.model.popup_device = None;
                if let Some(connection) = self.connection.as_ref() {
                    let connection = connection.clone();
                    if let Some(device) = self.model.devices.get_mut(&path) {
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

            Message::ServiceActivate => {
                return cosmic::task::future(async {
                    systemd::activate_bluetooth().await;
                    tokio::time::sleep(Duration::from_secs(3)).await;

                    match zbus::Connection::system().await {
                        Ok(connection) => Message::DBusConnect(connection),
                        Err(why) => Message::DBusConnectFailed(why),
                    }
                });
            }

            Message::ServiceEnable => {
                return cosmic::task::future(async {
                    systemd::enable_bluetooth().await;
                    tokio::time::sleep(Duration::from_secs(3)).await;

                    match zbus::Connection::system().await {
                        Ok(connection) => Message::DBusConnect(connection),
                        Err(why) => Message::DBusConnectFailed(why),
                    }
                });
            }

            Message::DBusConnectFailed(why) => {
                tracing::error!("dbus connection failed. {why:?}");
                self.connection = None;
                self.service_is_active = false;
                self.service_is_enabled = false;
                if let Some(abort_handle) = self.subscription.take() {
                    _ = abort_handle.send(());
                }
            }
        };

        cosmic::Task::none()
    }
}

fn status() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let bluetooth = descriptions.insert(fl!("bluetooth"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            fn bluetooth_service_issue<'a>(
                description: String,
                label: String,
                message: Message,
            ) -> Element<'a, crate::pages::Message> {
                widget::settings::item::builder(description)
                    .control(widget::button::suggested(label).on_press(message.into()))
                    .apply(|control| Element::from(widget::settings::section().add(control)))
            }

            if page.bluez_service_unknown {
                let control = widget::text::body(fl!("bluetooth", "unknown"));

                return Element::from(widget::settings::section().add(control));
            } else if !page.service_is_enabled {
                return bluetooth_service_issue(
                    fl!("bluetooth", "disabled"),
                    fl!("enable"),
                    Message::ServiceEnable,
                );
            } else if !page.service_is_active {
                return bluetooth_service_issue(
                    fl!("bluetooth", "inactive"),
                    fl!("activate"),
                    Message::ServiceEnable,
                );
            }

            let status = page
                .model
                .get_selected_adapter()
                .map_or(page.model.active, |(_, adapter)| adapter.enabled);

            let mut bluetooth_toggle = settings::item::builder(&descriptions[bluetooth]);
            if matches!(status, Active::Enabling | Active::Enabled) {
                bluetooth_toggle = bluetooth_toggle.description(&page.heading);
            }

            widget::list_column()
                .add(
                    bluetooth_toggle.control(
                        widget::toggler(matches!(status, Active::Enabling | Active::Enabled))
                            .on_toggle(|active| Message::SetActive(active).into()),
                    ),
                )
                .apply(Element::from)
        })
}

fn popup_button(message: Option<Message>, text: &str) -> Element<'_, Message> {
    let spacing = theme::spacing();
    widget::text::body(text)
        .align_y(Alignment::Center)
        .apply(widget::button::custom)
        .padding([spacing.space_xxxs, spacing.space_xs])
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
            page.model.selected_adapter.as_ref().map(|adapter| {
                page.model
                    .devices_for_adapter(adapter)
                    .any(|(_, device)| device.paired)
            }) == Some(true)
                && page.model.active != Active::Disabled
        })
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let section = settings::section().title(&section.title);

            page.model
                .devices_for_adapter(page.model.selected_adapter.as_ref().unwrap())
                .filter_map(|(path, device)| {
                    if !device.paired {
                        return None;
                    }

                    let device_menu: Element<_> = if page
                        .model
                        .popup_device
                        .as_deref()
                        .is_some_and(|p| path.as_str() == p.as_str())
                    {
                        widget::popover(
                            widget::button::icon(widget::icon::from_name("view-more-symbolic"))
                                .on_press(Message::PopupDevice(None)),
                        )
                        .position(widget::popover::Position::Bottom)
                        .on_close(Message::PopupDevice(None))
                        .popup(
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
                                ))
                                .width(Length::Fixed(200.0))
                                .apply(widget::container)
                                .padding(theme::spacing().space_xxs)
                                .class(theme::Container::Dropdown),
                        )
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
            page.model.selected_adapter.as_ref().map(|adapter| {
                page.model
                    .devices_for_adapter(adapter)
                    .any(|(_, device)| !device.paired)
            }) == Some(true)
                && page.model.active != Active::Disabled
        })
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let section = settings::section().title(&section.title);

            page.model
                .devices_for_adapter(page.model.selected_adapter.as_ref().unwrap())
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
        .show_while::<Page>(|page| {
            page.model.adapters.len() > 1 && page.model.selected_adapter.is_none()
        })
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let section = settings::section().title(&section.title);

            page.model
                .adapters
                .iter()
                .map(|(path, adapter)| {
                    let mut items = vec![
                        widget::icon::from_name("bluetooth-symbolic")
                            .size(20)
                            .into(),
                        widget::horizontal_space()
                            .width(theme::spacing().space_xxs)
                            .into(),
                        text(&adapter.alias).wrapping(Wrapping::Word).into(),
                        widget::horizontal_space().into(),
                        widget::icon::from_name("go-next-symbolic").into(),
                    ];
                    if page.model.adapter_connected(path) {
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

mod systemd {
    use futures::FutureExt;

    pub fn activate_bluetooth() -> impl Future<Output = ()> + Send {
        tokio::process::Command::new("pkexec")
            .args(["systemctl", "start", "bluetooth"])
            .status()
            .map(|_| ())
    }

    pub fn enable_bluetooth() -> impl Future<Output = ()> + Send {
        tokio::process::Command::new("pkexec")
            .args(["systemctl", "enable", "--now", "bluetooth"])
            .status()
            .map(|_| ())
    }

    pub fn is_bluetooth_enabled() -> bool {
        std::process::Command::new("systemctl")
            .args(["is-enabled", "bluetooth"])
            .status()
            .map(|status| status.success())
            .unwrap_or(true)
    }

    pub fn is_bluetooth_active() -> bool {
        std::process::Command::new("systemctl")
            .args(["is-active", "bluetooth"])
            .status()
            .map(|status| status.success())
            .unwrap_or(true)
    }
}
