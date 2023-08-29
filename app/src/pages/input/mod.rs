use crate::app;
use cosmic::{
    cosmic_config::{self, ConfigGet, ConfigSet},
    iced::{self, wayland::actions::window::SctkWindowSettings, window},
    iced_sctk::commands,
    iced_widget::core::layout,
};
use cosmic_comp_config::XkbConfig;
use cosmic_settings_page as page;
use itertools::Itertools;
use tracing::error;

pub mod keyboard;
mod mouse;

#[derive(Clone, Debug)]
pub enum Message {
    SetAcceleration(bool),
    SetNaturalScroll(bool),
    SetScrollSpeed(u32),
    SetDoubleClickSpeed(u32),
    SetMouseSpeed(u32),
    PrimaryButtonSelected(cosmic::widget::segmented_button::Entity),
    // seperate close message, to make sure another isn't closed?
    ExpandInputSourcePopover(Option<String>),
    OpenKeyboardShortcuts,
    OpenSpecialCharacterDialog(keyboard::SpecialKey),
    CloseSpecialCharacterDialog,
    SpecialCharacterSelect(Option<&'static str>),
}

pub struct Page {
    config: cosmic_config::Config,

    // Mouse
    primary_button: cosmic::widget::segmented_button::SingleSelectModel,
    acceleration: bool,
    natural_scroll: bool,
    double_click_speed: u32,
    scroll_speed: u32,
    mouse_speed: u32,

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
        let xkb = get_config(&config, "xkb-config");

        Self {
            config,

            // Mouse
            primary_button: mouse::default_primary_button(),
            acceleration: false,
            natural_scroll: false,
            double_click_speed: 0,
            scroll_speed: 0,
            mouse_speed: 0,

            // Keyboard
            expanded_source_popover: None,
            sources: keyboard::default_input_sources(),
            special_character_dialog: None,
            xkb,
        }
    }
}

impl Page {
    // TODO
    pub fn update(&mut self, message: Message) -> iced::Command<app::Message> {
        match message {
            Message::SetAcceleration(value) => {
                self.acceleration = value;
            }
            Message::SetNaturalScroll(value) => {
                self.natural_scroll = value;
            }
            Message::SetScrollSpeed(value) => {
                self.scroll_speed = value;
            }
            Message::SetDoubleClickSpeed(value) => {
                self.double_click_speed = value;
            }
            Message::SetMouseSpeed(value) => {
                self.mouse_speed = value;
            }
            Message::PrimaryButtonSelected(entity) => {
                self.primary_button.activate(entity);
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
            Message::OpenKeyboardShortcuts => {}
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
        page.sub_page::<keyboard::Page>().sub_page::<mouse::Page>()
    }
}
