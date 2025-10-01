// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::app;
use cosmic::{
    Task,
    cosmic_config::{self, ConfigGet, ConfigSet},
};
use cosmic_comp_config::input::{
    AccelConfig, AccelProfile, ClickMethod, InputConfig, ScrollConfig, ScrollMethod, TapButtonMap,
    TapConfig,
};
use cosmic_settings_page as page;
use tracing::error;

pub mod keyboard;
pub mod mouse;
pub mod touchpad;

#[derive(Clone, Debug)]
pub enum Message {
    // seperate close message, to make sure another isn't closed?
    DisableWhileTyping(bool, bool),
    PrimaryButtonSelected(cosmic::widget::segmented_button::Entity, bool),
    SetAcceleration(bool, bool),
    SetMouseSpeed(f64, bool),
    SetNaturalScroll(bool, bool),
    SetSecondaryClickBehavior(Option<ClickMethod>, bool),
    SetScrollFactor(f64, bool),
    SetScrollMethod(Option<ScrollMethod>, bool),
    TapToClick(bool),
}

pub struct Page {
    config: cosmic_config::Config,
    input_default: InputConfig,
    #[allow(dead_code)]
    input_touchpad: InputConfig,

    // Mouse
    primary_button: cosmic::widget::segmented_button::SingleSelectModel,

    // Touchpad
    touchpad_primary_button: cosmic::widget::segmented_button::SingleSelectModel,
}

fn get_config<T: Default + serde::de::DeserializeOwned>(
    config: &cosmic_config::Config,
    key: &str,
) -> T {
    config.get(key).unwrap_or_else(|why| {
        if why.is_err() {
            error!(?why, "Failed to read config '{}'", key);
        }

        T::default()
    })
}

impl Default for Page {
    fn default() -> Self {
        let config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let input_default: InputConfig = get_config(&config, "input_default");
        let input_touchpad: InputConfig = get_config(&config, "input_touchpad");

        let mut primary_button = mouse::default_primary_button();
        let idx = input_default.left_handed.unwrap_or(false) as u16;
        primary_button.activate_position(idx);

        let mut touchpad_primary_button = mouse::default_primary_button();
        let idx = input_touchpad.left_handed.unwrap_or(false) as u16;
        touchpad_primary_button.activate_position(idx);

        Self {
            config,
            input_default,
            input_touchpad,

            // Mouse
            primary_button,

            // Touchpad
            touchpad_primary_button,
        }
    }
}

impl Page {
    fn update_input<F: Fn(&mut InputConfig)>(&mut self, touchpad: bool, f: F) {
        let (name, input_config) = if touchpad {
            ("input_touchpad", &mut self.input_touchpad)
        } else {
            ("input_default", &mut self.input_default)
        };
        f(input_config);
        if let Err(err) = self.config.set(name, input_config) {
            error!(?err, "Failed to set config '{}'", name);
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) -> Task<app::Message> {
        match message {
            Message::SetAcceleration(value, touchpad) => {
                let profile = if value {
                    AccelProfile::Adaptive
                } else {
                    AccelProfile::Flat
                };

                self.update_input(touchpad, |x| {
                    x.acceleration.get_or_insert(AccelConfig::default()).profile = Some(profile);
                });
            }

            Message::SetMouseSpeed(value, touchpad) => self.update_input(touchpad, |x| {
                x.acceleration.get_or_insert(AccelConfig::default()).speed = value;
            }),

            Message::DisableWhileTyping(disabled, touchpad) => {
                self.update_input(touchpad, |conf| {
                    conf.disable_while_typing = Some(disabled);
                });
            }

            Message::SetNaturalScroll(enabled, touchpad) => self.update_input(touchpad, |x| {
                x.scroll_config
                    .get_or_insert(ScrollConfig::default())
                    .natural_scroll = Some(enabled);
            }),

            Message::SetSecondaryClickBehavior(click_method, touchpad) => {
                self.update_input(touchpad, |x| {
                    x.click_method = click_method;
                });
            }

            Message::SetScrollFactor(value, touchpad) => self.update_input(touchpad, |x| {
                x.scroll_config
                    .get_or_insert(ScrollConfig::default())
                    .scroll_factor = Some(value);
            }),

            Message::SetScrollMethod(method, touchpad) => {
                self.update_input(touchpad, |conf| {
                    conf.scroll_config
                        .get_or_insert(ScrollConfig::default())
                        .method = method;
                });
            }

            Message::PrimaryButtonSelected(entity, touchpad) => {
                let select_model = if touchpad {
                    &mut self.touchpad_primary_button
                } else {
                    &mut self.primary_button
                };
                select_model.activate(entity);

                let Some(left_entity) = select_model.entity_at(1) else {
                    return Task::none();
                };

                let left_handed = select_model.active() == left_entity;
                self.update_input(touchpad, |x| x.left_handed = Some(left_handed));
            }

            Message::TapToClick(enabled) => {
                self.update_input(true, |conf| {
                    conf.tap_config
                        .get_or_insert(TapConfig {
                            enabled: true,
                            button_map: Some(TapButtonMap::LeftRightMiddle),
                            drag: true,
                            drag_lock: false,
                        })
                        .enabled = enabled;
                });
            }
        }

        Task::none()
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        // XXX icon?
        page::Info::new("input-devices", "preferences-input-devices-symbolic")
            .title(fl!("input-devices"))
            .description(fl!("input-devices", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        let insert = page.sub_page::<keyboard::Page>().sub_page::<mouse::Page>();

        if system_has_touchpad() {
            insert.sub_page::<touchpad::Page>()
        } else {
            insert
        }
    }
}

/// Uses `udev` to check if a touchpad device exists on the system.
fn system_has_touchpad() -> bool {
    let Ok(mut enumerator) = udev::Enumerator::new() else {
        return false;
    };

    let _res = enumerator.match_subsystem("input");

    let Ok(mut devices) = enumerator.scan_devices() else {
        return false;
    };

    devices.any(|device| {
        device
            .property_value("ID_INPUT_TOUCHPAD")
            .is_some_and(|value| value == "1")
    })
}
