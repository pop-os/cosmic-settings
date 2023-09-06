use crate::app;
use cosmic::{
    cosmic_config::{self, ConfigGet, ConfigSet},
    iced::{self, wayland::actions::window::SctkWindowSettings, window},
    iced_sctk::commands,
    iced_widget::core::layout,
};
use cosmic_comp_config::{
    input::{AccelProfile, InputConfig},
    XkbConfig,
};
use cosmic_settings_page as page;
use itertools::Itertools;
use tracing::error;

pub mod keyboard;
mod mouse;
mod touchpad;

#[derive(Clone, Debug)]
pub enum Message {
    SetAcceleration(bool, bool),
    SetNaturalScroll(bool, bool),
    SetScrollFactor(f64, bool),
    SetDoubleClickSpeed(u32, bool),
    SetMouseSpeed(f64, bool),
    PrimaryButtonSelected(cosmic::widget::segmented_button::Entity, bool),
    // seperate close message, to make sure another isn't closed?
    ExpandInputSourcePopover(Option<String>),
    OpenSpecialCharacterDialog(keyboard::SpecialKey),
    CloseSpecialCharacterDialog,
    SpecialCharacterSelect(Option<&'static str>),
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

    // Keyboard
    expanded_source_popover: Option<String>,
    sources: Vec<keyboard::InputSource>,
    special_character_dialog: Option<keyboard::SpecialKey>,
    xkb: XkbConfig,
}

fn get_config<T: Default + serde::de::DeserializeOwned>(
    config: &cosmic_config::Config,
    key: &str,
) -> T {
    config.get(key).unwrap_or_else(|err| {
        error!(?err, "Failed to read config '{}'", key);
        T::default()
    })
}

impl Default for Page {
    fn default() -> Self {
        let config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let input_default: InputConfig = get_config(&config, "input-default");
        let input_touchpad: InputConfig = get_config(&config, "input-touchpad");
        let xkb = get_config(&config, "xkb-config");

        let mut primary_button = mouse::default_primary_button();
        let idx = if input_default.left_handed.unwrap_or(false) {
            1
        } else {
            0
        };
        primary_button.activate_position(idx);

        let mut touchpad_primary_button = mouse::default_primary_button();
        let idx = if input_touchpad.left_handed.unwrap_or(false) {
            1
        } else {
            0
        };
        touchpad_primary_button.activate_position(idx);

        Self {
            config,
            input_default,
            input_touchpad,

            // Mouse
            primary_button,

            // Touchpad
            touchpad_primary_button,

            // Keyboard
            expanded_source_popover: None,
            sources: keyboard::default_input_sources(),
            special_character_dialog: None,
            xkb,
        }
    }
}

impl Page {
    fn update_input<F: Fn(&mut InputConfig)>(&mut self, touchpad: bool, f: F) {
        let (name, input_config) = if touchpad {
            ("input-touchpad", &mut self.input_touchpad)
        } else {
            ("input-default", &mut self.input_default)
        };
        f(input_config);
        if let Err(err) = self.config.set(name, input_config) {
            error!(?err, "Failed to set config '{}'", name);
        }
    }

    pub fn update(&mut self, message: Message) -> iced::Command<app::Message> {
        match message {
            Message::SetAcceleration(value, touchpad) => {
                let profile = if value {
                    AccelProfile::Adaptive
                } else {
                    AccelProfile::Flat
                };
                self.update_input(touchpad, |x| {
                    x.acceleration.get_or_insert(Default::default()).profile = Some(profile);
                });
            }
            Message::SetNaturalScroll(value, touchpad) => self.update_input(touchpad, |x| {
                x.scroll_config
                    .get_or_insert(Default::default())
                    .natural_scroll = Some(value);
            }),
            Message::SetScrollFactor(value, touchpad) => self.update_input(touchpad, |x| {
                x.scroll_config
                    .get_or_insert(Default::default())
                    .scroll_factor = Some(value)
            }),
            Message::SetDoubleClickSpeed(_value, _touchpad) => {
                // TODO
            }
            Message::SetMouseSpeed(value, touchpad) => self.update_input(touchpad, |x| {
                x.acceleration.get_or_insert(Default::default()).speed = value
            }),
            Message::PrimaryButtonSelected(entity, touchpad) => {
                let select_model = if touchpad {
                    &mut self.touchpad_primary_button
                } else {
                    &mut self.primary_button
                };
                select_model.activate(entity);
                let left_entity = select_model.entity_at(1).unwrap();
                let left_handed = select_model.active() == left_entity;
                self.update_input(touchpad, |x| x.left_handed = Some(left_handed));
            }
            Message::ExpandInputSourcePopover(value) => {
                self.expanded_source_popover = value;
            }
            Message::OpenSpecialCharacterDialog(special_key) => {
                self.special_character_dialog = Some(special_key);
                let window_settings = SctkWindowSettings {
                    window_id: keyboard::SPECIAL_CHARACTER_DIALOGUE_ID,
                    app_id: Some("com.system76.CosmicSettings".to_string()),
                    title: Some(special_key.title()),
                    parent: Some(window::Id(0)),
                    autosize: false,
                    size_limits: layout::Limits::NONE
                        .min_width(300.0)
                        .max_width(800.0)
                        .min_height(200.0)
                        .max_height(1080.0),
                    size: (512, 420),
                    resizable: None,
                    client_decorations: true,
                    transparent: true,
                };
                return commands::window::get_window(window_settings);
            }
            Message::CloseSpecialCharacterDialog => {
                self.special_character_dialog = None;
                return commands::window::close_window(keyboard::SPECIAL_CHARACTER_DIALOGUE_ID);
            }
            Message::SpecialCharacterSelect(id) => {
                if let Some(special_key) = self.special_character_dialog {
                    let options = self.xkb.options.as_deref().unwrap_or("");
                    let prefix = special_key.prefix();
                    let new_options = options
                        .split(',')
                        .filter(|x| !x.starts_with(prefix))
                        .chain(id.into_iter())
                        .join(",");
                    self.xkb.options = Some(new_options).filter(|x| !x.is_empty());
                    if let Err(err) = self.config.set("xkb-config", &self.xkb) {
                        error!(?err, "Failed to set config 'xkb-config'");
                    }
                }
            }
        }
        iced::Command::none()
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        // XXX icon?
        page::Info::new("input", "input-keyboard-symbolic")
            .title(fl!("input"))
            .description(fl!("input", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<keyboard::Page>()
            .sub_page::<mouse::Page>()
            .sub_page::<touchpad::Page>()
    }
}
