// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only
use std::collections::HashMap;

use cosmic::cosmic_config::{ConfigGet, ConfigSet};
use cosmic::iced::Subscription;
use cosmic::iced::stream;
use cosmic::widget::{dropdown, settings};
use cosmic::{Apply, Element, Task};
use cosmic_comp_config::input::{DeviceState, InputConfig};
use cosmic_settings_page::{self as page, Section, section};
use futures::SinkExt;
use slotmap::{Key, SlotMap};
use udev::Enumerator;

pub fn default_drawing_tablet_button() -> cosmic::widget::segmented_button::SingleSelectModel {
    let mut model = cosmic::widget::segmented_button::SingleSelectModel::builder().build();
    model.activate_position(0);
    model
}

#[derive(Clone, Debug)]
pub enum Message {
    SetTabletDeviceState(String, bool),
    SetTabletOutput(String, Option<String>),
    Surface(cosmic::surface::Action<crate::app::Message>),
    RefreshTablets,
    RefreshDisplays,
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::DrawingTablet(message)
    }
}

pub struct Page {
    entity: page::Entity,
    connected_tablets: HashMap<String, Vec<udev::Device>>,
    connected_displays: Vec<String>,
    saved_input_devices: HashMap<String, InputConfig>,
    tablet_config: cosmic_config::Config,
}

pub fn get_connected_tablets() -> HashMap<String, Vec<udev::Device>> {
    let mut tablets = HashMap::new();
    let Ok(mut device_scanner) = Enumerator::new() else {
        return tablets;
    };

    if device_scanner.match_subsystem("input").is_err() {
        return tablets;
    }

    let Ok(devices) = device_scanner.scan_devices() else {
        return tablets;
    };

    for device in devices {
        if device
            .property_value("ID_INPUT_TABLET")
            .is_some_and(|v| v == "1")
        {
            let physical_device_id = get_physical_device_id(&device);

            let device_name = device.property_value("NAME");
            if !device_name.is_some() {
                continue;
            }

            tablets.entry(physical_device_id).or_default().push(device);
        }
    }

    tablets
}

pub fn get_physical_device_id(device: &udev::Device) -> String {
    if let Some(group) = device.property_value("LIBINPUT_DEVICE_GROUP") {
        return group.to_string_lossy().into_owned();
    }

    let mut curr = Some(device.clone());
    while let Some(dev) = curr {
        if dev.devtype().is_some_and(|t| t == "usb_device") {
            return dev.syspath().to_string_lossy().into_owned();
        }
        curr = dev.parent();
    }

    device.syspath().to_string_lossy().into_owned()
}

pub fn get_pen_interface(devices: &Vec<udev::Device>) -> Option<&udev::Device> {
    devices.into_iter().find(|device| {
        let is_tablet = device
            .property_value("ID_INPUT_TABLET")
            .is_some_and(|v| v == "1");

        let is_pad = device
            .property_value("ID_INPUT_TABLET_PAD")
            .is_some_and(|v| v == "1");
        let is_touchpad = device
            .property_value("ID_INPUT_TOUCHPAD")
            .is_some_and(|v| v == "1");

        is_tablet && !is_pad && !is_touchpad
    })
}

pub fn get_connected_displays() -> Vec<String> {
    let mut displays: Vec<String> = futures::executor::block_on(async {
        cosmic_randr_shell::list()
            .await
            .map(|l| l.outputs.values().map(|o| o.name.clone()).collect())
            .unwrap_or_default()
    });

    displays.insert(0, fl!("drawing-tablet", "not-set"));

    displays
}

impl Default for Page {
    fn default() -> Self {
        let tablet_config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let saved_input_devices: HashMap<String, InputConfig> = tablet_config
            .get("saved_input_devices")
            .unwrap_or_else(|_| HashMap::new());

        let connected_tablets = get_connected_tablets();
        let connected_displays = get_connected_displays();

        Page {
            entity: page::Entity::null(),
            connected_tablets,
            connected_displays,
            saved_input_devices,
            tablet_config,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        let devices_section = Section::default().view::<Page>(move |_binder, page, section| {
            if page.connected_tablets.is_empty() {
                settings::section()
                    .title(&section.title)
                    .add(
                        settings::item::builder(fl!("drawing-tablet", "no-devices-detected"))
                            .control(cosmic::widget::space()),
                    )
                    .apply(Element::from)
                    .map(crate::pages::Message::DrawingTablet)
            } else {
                let mut sections_column = Vec::new();
                for tablet in page.connected_tablets.values() {
                    if let Some(pen_interface) = get_pen_interface(tablet) {
                        if let Some(pen_name) = pen_interface.property_value("NAME") {
                            let tablet_device_name =
                                pen_name.to_string_lossy().trim_matches('"').to_owned();
                            sections_column.push(tablet_settings_ui(tablet_device_name, page));
                        }
                    }
                }
                settings::view_column(sections_column).apply(Element::from)
            }
        });

        Some(vec![sections.insert(devices_section)])
    }

    fn info(&self) -> page::Info {
        page::Info::new("drawing-tablet", "input-tablet-symbolic")
            .title(fl!("drawing-tablet"))
            .description(fl!("xdg-entry-drawing-tablet-comment"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        Task::none()
    }

    fn subscription(&self, _core: &cosmic::Core) -> Subscription<crate::pages::Message> {
        let tablet_subscription = Subscription::run(|| {
            stream::channel(
                1,
                move |mut emitter: futures::channel::mpsc::Sender<crate::pages::Message>| async move {
                    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

                    let tokio_handle = tokio::runtime::Handle::current();
                    std::thread::spawn(move || {
                        tokio_handle.block_on(async move {
                            let Ok(builder) = udev::MonitorBuilder::new() else {
                                return;
                            };

                            let Ok(builder) = builder.match_subsystem("input") else {
                                return;
                            };

                            let Ok(socket) = builder.listen() else {
                                return;
                            };

                            let Ok(mut async_fd) = tokio::io::unix::AsyncFd::new(socket) else {
                                return;
                            };

                            loop {
                                if let Ok(mut guard) = async_fd.writable().await {
                                    guard.clear_ready();

                                    let input_event_occurred = &mut false;
                                    async_fd.get_mut().iter().for_each(|event| {
                                        let event_type = event.event_type();
                                        if event_type == udev::EventType::Add
                                            || event_type == udev::EventType::Remove
                                        {
                                            *input_event_occurred = true;
                                        }
                                    });

                                    if *input_event_occurred {
                                        if tx.send(()).await.is_err() {
                                            break;
                                        }
                                    }
                                }
                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            }
                        });
                    });

                    while let Some(()) = rx.recv().await {
                        if emitter.send(Message::RefreshTablets.into()).await.is_err() {
                            break;
                        }
                    }
                },
            )
        });

        let display_subscription = Subscription::run(|| {
            stream::channel(
                1,
                move |mut emitter: futures::channel::mpsc::Sender<crate::pages::Message>| async move {
                    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

                    let tokio_handle = tokio::runtime::Handle::current();
                    std::thread::spawn(move || {
                        tokio_handle.block_on(async move {
                            let Ok(builder) = udev::MonitorBuilder::new() else {
                                return;
                            };

                            let Ok(builder) = builder.match_subsystem("drm") else {
                                return;
                            };

                            let Ok(socket) = builder.listen() else {
                                return;
                            };

                            let Ok(mut async_fd) = tokio::io::unix::AsyncFd::new(socket) else {
                                return;
                            };

                            loop {
                                if let Ok(mut guard) = async_fd.writable().await {
                                    guard.clear_ready();

                                    let drm_hotplug_occurred = &mut false;
                                    async_fd.get_mut().iter().for_each(|_| {
                                        *drm_hotplug_occurred = true;
                                    });

                                    if *drm_hotplug_occurred {
                                        if tx.send(()).await.is_err() {
                                            break;
                                        }
                                    }
                                }
                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            }
                        });
                    });

                    while let Some(()) = rx.recv().await {
                        if emitter.send(Message::RefreshDisplays.into()).await.is_err() {
                            break;
                        }
                    }
                },
            )
        });

        Subscription::batch(vec![tablet_subscription, display_subscription])
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::SetTabletDeviceState(device_name, enabled) => {
                let state = if enabled {
                    DeviceState::Enabled
                } else {
                    DeviceState::Disabled
                };
                let mut devices: HashMap<String, InputConfig> = self.saved_input_devices.clone();
                devices.entry(device_name).or_default().state = state;
                if let Err(err) = self.tablet_config.set("input_devices", &devices) {
                    tracing::error!(?err, "Failed to set input_devices config");
                }
                self.saved_input_devices = devices;
            }

            Message::SetTabletOutput(device_name, output) => {
                let mut devices: HashMap<String, InputConfig> = self.saved_input_devices.clone();
                devices.entry(device_name).or_default().map_to_output = output;
                if let Err(err) = self.tablet_config.set("input_devices", &devices) {
                    tracing::error!(?err, "Failed to set input_devices config");
                }
                self.saved_input_devices = devices;
            }

            Message::RefreshTablets => {
                self.connected_tablets = get_connected_tablets();
            }

            Message::RefreshDisplays => {
                self.connected_displays = get_connected_displays();
            }

            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }
        }

        Task::none()
    }
}

// The returned value represents all the settings UI for a single tablet device.
fn tablet_settings_ui(
    tablet_device_name: String,
    page: &Page,
) -> Element<'_, crate::pages::Message> {
    let mut section_ui = settings::section().title(tablet_device_name.clone());
    let tablet_config = page.saved_input_devices.get(&tablet_device_name);

    let enabled = tablet_config.map_or(false, |c| {
        c.state == cosmic_comp_config::input::DeviceState::Enabled
    });
    let tablet_name = tablet_device_name.clone();
    let device_state_toggler = settings::item::builder(fl!("drawing-tablet", "enabled"))
        .toggler(enabled, {
            move |checked| Message::SetTabletDeviceState(tablet_name.clone(), checked)
        });
    section_ui = section_ui.add(device_state_toggler);

    if enabled {
        let mut display_names = page.connected_displays.clone();
        if let Some(preexisting_output) = tablet_config.and_then(|c| c.map_to_output.as_ref()) {
            if !display_names.contains(preexisting_output) {
                display_names.push(preexisting_output.clone());
            }
        }

        let selected_dropdown_index = tablet_config
            .and_then(|c| c.map_to_output.as_ref())
            .and_then(|output_display_name| {
                display_names.iter().position(|d| d == output_display_name)
            })
            .unwrap_or(0);
        let tablet_name = tablet_device_name.clone();
        let dropdown_element = dropdown::popup_dropdown(
            display_names.clone(),
            Some(selected_dropdown_index),
            move |idx| {
                let output = if idx == 0 {
                    None
                } else {
                    Some(display_names[idx].clone())
                };
                Message::SetTabletOutput(tablet_name.clone(), output)
            },
            cosmic::iced::window::Id::RESERVED,
            Message::Surface,
            |a| crate::app::Message::PageMessage(crate::pages::Message::DrawingTablet(a)),
        );

        let dropdown_item = settings::item::builder(fl!("drawing-tablet", "map-to-display"))
            .control(dropdown_element);
        section_ui = section_ui.add(dropdown_item);
    }

    section_ui
        .apply(Element::from)
        .map(crate::pages::Message::DrawingTablet)
}
