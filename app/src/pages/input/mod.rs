use crate::app;
use cosmic::{
    iced::{self, wayland::actions::window::SctkWindowSettings, window},
    iced_sctk::commands,
    iced_widget::core::layout,
};
use cosmic_settings_page as page;

pub mod keyboard;
mod mouse;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct XkbConfig {
    pub rules: String,
    pub model: String,
    pub layout: String,
    pub variant: String,
    pub options: Option<String>,
}

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

#[derive(derivative::Derivative)]
#[derivative(Default)]
pub struct Page {
    // cosmic_config::Config

    // Mouse
    #[derivative(Default(value = "mouse::default_primary_button()"))]
    primary_button: cosmic::widget::segmented_button::SingleSelectModel,
    acceleration: bool,
    natural_scroll: bool,
    double_click_speed: u32,
    scroll_speed: u32,
    mouse_speed: u32,

    // Keyboard
    expanded_source_popover: Option<String>,
    #[derivative(Default(value = "keyboard::default_input_sources()"))]
    sources: Vec<keyboard::InputSource>,
    special_character_dialog: Option<keyboard::SpecialKey>,
    xkb_options: Vec<String>,
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
                    let prefix = special_key.prefix();
                    if let Some(idx) = self.xkb_options.iter().position(|x| x.starts_with(prefix)) {
                        self.xkb_options.remove(idx);
                    }
                    if let Some(id) = id {
                        self.xkb_options.push(id.to_string());
                    }
                    // TODO set in cosmic-config
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
