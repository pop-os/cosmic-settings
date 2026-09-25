// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only
use std::collections::HashMap;

use cosmic::cosmic_config::{ConfigGet, ConfigSet};
use cosmic::widget::{dropdown, settings};
use cosmic::{Apply, Element, Task};
use cosmic_comp_config::input::{DeviceState, InputConfig};
use cosmic_settings_page::{self as page, Section, section};
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
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::DrawingTablet(message)
    }
}

pub struct Page {
    entity: page::Entity,
    connected_tablets: Vec<String>,
    connected_displays: Vec<String>,
    input_devices: HashMap<String, InputConfig>,
    tablet_config: cosmic_config::Config,
}

pub fn get_connected_tablets() -> Vec<String> {
    let mut tablets = Vec::new();
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
            if let Some(name) = device
                .property_value("NAME")
                .or_else(|| device.property_value("ID_NAME"))
                .or_else(|| device.property_value("ID_MODEL"))
            {
                let name = name.to_string_lossy().trim_matches('"').to_string();
                if !tablets.contains(&name) {
                    tablets.push(name);
                }
            }
        }
    }

    tablets
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
        let config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let saved_input_devices: HashMap<String, InputConfig> = config
            .get("input_devices")
            .unwrap_or_else(|_| HashMap::new());

        let connected_tablets = get_connected_tablets();
        let connected_displays = get_connected_displays();

        Page {
            entity: page::Entity::null(),
            connected_tablets,
            connected_displays,
            input_devices: saved_input_devices,
            tablet_config: config,
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
        // Show a "no devices" message on the screen if none are found.
        if self.connected_tablets.is_empty() {
            let no_devices_section =
                Section::default().view::<Page>(move |_binder, _page, section| {
                    settings::section()
                        .title(&section.title)
                        .add(
                            settings::item::builder(fl!("drawing-tablet", "no-devices-detected"))
                                .control(cosmic::widget::space()),
                        )
                        .apply(Element::from)
                        .map(crate::pages::Message::DrawingTablet)
                });

            Some(vec![sections.insert(no_devices_section)])
        } else {
            // If devices are found, make a new Section for each one with tablet_settings_ui() and then add all the settings groups to the view.
            let mut tablets = Vec::new();
            for tablet in &self.connected_tablets {
                tablets.push(sections.insert(tablet_settings_ui(tablet.clone())));
            }

            Some(tablets)
        }
    }

    fn info(&self) -> page::Info {
        page::Info::new("drawing-tablet", "input-tablet-symbolic")
            .title(fl!("drawing-tablet"))
            .description(fl!("xdg-entry-drawing-tablet-comment"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        Task::none()
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
                let mut devices: HashMap<String, InputConfig> = self.input_devices.clone();
                devices.entry(device_name).or_default().state = state;
                if let Err(err) = self.tablet_config.set("input_devices", &devices) {
                    tracing::error!(?err, "Failed to set input_devices config");
                }
                self.input_devices = devices;
            }

            Message::SetTabletOutput(device_name, output) => {
                let mut devices: HashMap<String, InputConfig> = self.input_devices.clone();
                devices.entry(device_name).or_default().map_to_output = output;
                if let Err(err) = self.tablet_config.set("input_devices", &devices) {
                    tracing::error!(?err, "Failed to set input_devices config");
                }
                self.input_devices = devices;
            }

            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }
        }

        Task::none()
    }
}

// The returned value is a Section representing the settings for a single tablet device.
fn tablet_settings_ui(tablet_device_name: String) -> Section<crate::pages::Message> {
    Section::default()
        .title(tablet_device_name.clone())
        .view::<Page>(move |_binder, page, section| {
            let mut section_ui = settings::section().title(&section.title);
            let tablet_config = page.input_devices.get(&tablet_device_name);

            let enabled = tablet_config.map_or(false, |c| {
                c.state == cosmic_comp_config::input::DeviceState::Enabled
            });
            let tablet_name = tablet_device_name.clone();
            let device_state_toggler = settings::item::builder(fl!("drawing-tablet", "enabled"))
                .toggler(enabled, {
                    move |checked| Message::SetTabletDeviceState(tablet_name.clone(), checked)
                });
            section_ui = section_ui.add(device_state_toggler);

            // If enabled, show dropdown of displays that the tablet input can be mapped to.
            if enabled {
                let selected_dropdown_index = tablet_config
                    .and_then(|c| c.map_to_output.as_ref())
                    .and_then(|output_display_name| {
                        page.connected_displays
                            .iter()
                            .position(|d| d == output_display_name)
                    })
                    .unwrap_or(0);

                let displays_clone = page.connected_displays.clone();
                let device_name = tablet_device_name.clone();
                let dropdown_element = dropdown::popup_dropdown(
                    &page.connected_displays,
                    Some(selected_dropdown_index),
                    move |idx| {
                        let output = if idx == 0 {
                            None
                        } else {
                            Some(displays_clone[idx].clone())
                        };
                        Message::SetTabletOutput(device_name.clone(), output)
                    },
                    cosmic::iced::window::Id::RESERVED,
                    Message::Surface,
                    |a| crate::app::Message::PageMessage(crate::pages::Message::DrawingTablet(a)),
                );

                let dropdown_item =
                    settings::item::builder(fl!("drawing-tablet", "map-to-display"))
                        .control(dropdown_element);
                section_ui = section_ui.add(dropdown_item);
            }

            section_ui
                .apply(Element::from)
                .map(crate::pages::Message::DrawingTablet)
        })
}
