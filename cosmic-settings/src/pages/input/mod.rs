// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::app;
use cosmic::Task;
use cosmic::cosmic_config::{self, ConfigGet, ConfigSet};
use cosmic_comp_config::CursorHideConfig;
use cosmic_comp_config::input::{
    AccelConfig, AccelProfile, ClickMethod, InputConfig, ScrollConfig, ScrollMethod, TapButtonMap,
    TapConfig,
};
use cosmic_settings_page as page;
use tracing::error;

pub mod keyboard;
pub mod mouse;
pub mod touchpad;

/// Seeds for a timeout the first time its toggle is switched on, since the
/// compositor stores "off" as `None` and so remembers no previous value.
const DEFAULT_IDLE_SECONDS: u32 = 5;
const DEFAULT_FULLSCREEN_SECONDS: u32 = 3;

/// Cursor-hiding state. Mirrors `CursorHideConfig`, but keeps each timeout's
/// seconds while its toggle is off so switching back on restores the previous
/// value instead of resetting to the seed.
#[derive(Clone, Copy)]
struct CursorHide {
    while_typing: bool,
    after_touch: bool,
    idle_enabled: bool,
    idle_seconds: u32,
    fullscreen_enabled: bool,
    fullscreen_seconds: u32,
}

impl CursorHide {
    fn from_config(config: CursorHideConfig) -> Self {
        Self {
            while_typing: config.while_typing,
            after_touch: config.after_touch,
            idle_enabled: config.idle_timeout.is_some(),
            idle_seconds: config.idle_timeout.unwrap_or(DEFAULT_IDLE_SECONDS),
            fullscreen_enabled: config.fullscreen_idle_timeout.is_some(),
            fullscreen_seconds: config
                .fullscreen_idle_timeout
                .unwrap_or(DEFAULT_FULLSCREEN_SECONDS),
        }
    }

    fn to_config(self) -> CursorHideConfig {
        CursorHideConfig {
            idle_timeout: self.idle_enabled.then_some(self.idle_seconds),
            fullscreen_idle_timeout: self.fullscreen_enabled.then_some(self.fullscreen_seconds),
            while_typing: self.while_typing,
            after_touch: self.after_touch,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    // seperate close message, to make sure another isn't closed?
    DisableWhileTyping(bool, bool),
    PrimaryButtonSelected(cosmic::widget::segmented_button::Entity, bool),
    SetAcceleration(bool, bool),
    SetCursorHideAfterTouch(bool),
    SetCursorHideFullscreenEnabled(bool),
    SetCursorHideFullscreenSeconds(u32),
    SetCursorHideIdleEnabled(bool),
    SetCursorHideIdleSeconds(u32),
    SetCursorHideWhileTyping(bool),
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
    cursor_hide: CursorHide,
    /// Whether to offer the touch trigger at all; there is nothing to configure
    /// on a machine with no touchscreen.
    has_touchscreen: bool,

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

        let cursor_hide = CursorHide::from_config(get_config(&config, "cursor_hide"));

        Self {
            config,
            input_default,
            input_touchpad,

            // Mouse
            primary_button,
            cursor_hide,
            has_touchscreen: system_has_touchscreen(),

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

    fn persist_cursor_hide(&self) {
        if let Err(err) = self.config.set("cursor_hide", self.cursor_hide.to_config()) {
            error!(?err, "Failed to set config 'cursor_hide'");
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

            Message::SetCursorHideWhileTyping(enabled) => {
                self.cursor_hide.while_typing = enabled;
                self.persist_cursor_hide();
            }

            Message::SetCursorHideAfterTouch(enabled) => {
                self.cursor_hide.after_touch = enabled;
                self.persist_cursor_hide();
            }

            Message::SetCursorHideIdleEnabled(enabled) => {
                self.cursor_hide.idle_enabled = enabled;
                self.persist_cursor_hide();
            }

            Message::SetCursorHideIdleSeconds(seconds) => {
                self.cursor_hide.idle_seconds = seconds;
                if self.cursor_hide.idle_enabled {
                    self.persist_cursor_hide();
                }
            }

            Message::SetCursorHideFullscreenEnabled(enabled) => {
                self.cursor_hide.fullscreen_enabled = enabled;
                self.persist_cursor_hide();
            }

            Message::SetCursorHideFullscreenSeconds(seconds) => {
                self.cursor_hide.fullscreen_seconds = seconds;
                if self.cursor_hide.fullscreen_enabled {
                    self.persist_cursor_hide();
                }
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
            .description(fl!("xdg-entry-input-comment"))
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

/// Uses `udev` to check if a touchscreen device exists on the system.
fn system_has_touchscreen() -> bool {
    let Ok(mut enumerator) = udev::Enumerator::new() else {
        return false;
    };

    let _res = enumerator.match_subsystem("input");

    let Ok(mut devices) = enumerator.scan_devices() else {
        return false;
    };

    devices.any(|device| {
        device
            .property_value("ID_INPUT_TOUCHSCREEN")
            .is_some_and(|value| value == "1")
    })
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

#[cfg(test)]
mod tests {
    use super::{CursorHide, DEFAULT_FULLSCREEN_SECONDS, DEFAULT_IDLE_SECONDS};
    use cosmic_comp_config::CursorHideConfig;

    #[test]
    fn a_disabled_timeout_is_still_seeded() {
        let state = CursorHide::from_config(CursorHideConfig {
            idle_timeout: None,
            fullscreen_idle_timeout: None,
            while_typing: false,
            after_touch: false,
        });

        assert_eq!(state.to_config().idle_timeout, None);
        assert_eq!(state.idle_seconds, DEFAULT_IDLE_SECONDS);
        assert_eq!(state.fullscreen_seconds, DEFAULT_FULLSCREEN_SECONDS);
    }

    #[test]
    fn seconds_survive_a_toggle_off_and_on() {
        let mut state = CursorHide::from_config(CursorHideConfig {
            idle_timeout: Some(30),
            fullscreen_idle_timeout: Some(12),
            while_typing: true,
            after_touch: true,
        });

        state.idle_enabled = false;
        state.fullscreen_enabled = false;
        let off = state.to_config();
        assert_eq!(off.idle_timeout, None);
        assert_eq!(off.fullscreen_idle_timeout, None);
        // The toggles are independent of the two booleans.
        assert!(off.while_typing);
        assert!(off.after_touch);

        state.idle_enabled = true;
        state.fullscreen_enabled = true;
        let on = state.to_config();
        assert_eq!(on.idle_timeout, Some(30));
        assert_eq!(on.fullscreen_idle_timeout, Some(12));
    }

    #[test]
    fn the_two_timeouts_do_not_share_state() {
        let mut state = CursorHide::from_config(CursorHideConfig::default());
        state.idle_enabled = true;
        state.idle_seconds = 42;

        let config = state.to_config();
        assert_eq!(config.idle_timeout, Some(42));
        // Enabling one timeout must not enable or alter the other.
        assert_eq!(
            config.fullscreen_idle_timeout,
            CursorHideConfig::default().fullscreen_idle_timeout
        );
    }
}
