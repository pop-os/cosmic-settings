use cosmic::iced::Element;
use cosmic::widget::settings;
use cosmic::Apply;
use cosmic_config::CosmicConfigEntry;
use cosmic_osk_config::Config;
use cosmic_settings_page::section::{self, Section};
use cosmic_settings_page::{self as page, Entity};
use slotmap::SlotMap;

#[derive(Debug)]
pub struct Page {
    entity: Entity,

    config: Config,
    config_handler: cosmic_config::Config,
}

#[derive(Debug, Clone)]
pub enum Message {
    AlwaysShow(bool),
    Config(Config),
    GamepadShortcut(bool),
    ImeActivation(bool),
}

impl Default for Page {
    fn default() -> Self {
        let (config_handler, config) = match Config::handler() {
            Ok(config_handler) => {
                let config = Config::get_entry(&config_handler).unwrap_or_else(|(errs, config)| {
                    tracing::warn!("errors loading OSK config: {:?}", errs);
                    config
                });
                (config_handler, config)
            }
            Err(err) => {
                panic!("failed to create OSK config handler: {}", err);
            }
        };

        Page {
            entity: Entity::default(),
            config,
            config_handler,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: Entity) {
        self.entity = entity;
    }

    fn info(&self) -> page::Info {
        page::Info::new("accessibility_osk", "preferences-desktop-accessibility")
            .title(fl!("on-screen-keyboard"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, page::Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(osk())])
    }

    fn subscription(
        &self,
        _core: &cosmic::Core,
    ) -> cosmic::iced::Subscription<crate::pages::Message> {
        Config::subscription().map(|update| {
            if !update.errors.is_empty() {
                tracing::warn!("failed to parse OSK config update: {:?}", update.errors);
            }
            crate::pages::Message::AccessibilityOsk(Message::Config(update.config))
        })
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

pub fn osk() -> section::Section<crate::pages::Message> {
    crate::slab!(descriptions {
        on_screen_keyboard = fl!("on-screen-keyboard");
        always_show = fl!("on-screen-keyboard", "always-show");
        //TODO: icon_on_panel = fl!("on-screen-keyboard", "icon-on-panel");
        show_on_gamepad_shortcut = fl!("on-screen-keyboard", "show-on-gamepad-shortcut");
        show_on_text_input = fl!("on-screen-keyboard", "show-on-text-input");
    });

    Section::default()
        .title(&descriptions[on_screen_keyboard])
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .add(
                    settings::item::builder(&descriptions[always_show])
                        .toggler(page.config.always_shown, Message::AlwaysShow),
                )
                .add(
                    settings::item::builder(&descriptions[show_on_gamepad_shortcut])
                        .toggler(page.config.gamepad_shortcut, Message::GamepadShortcut),
                )
                .add(
                    settings::item::builder(&descriptions[show_on_text_input])
                        .toggler(page.config.ime_activation, Message::ImeActivation),
                )
                .apply(Element::from)
                .map(crate::pages::Message::AccessibilityOsk)
        })
}

impl Page {
    pub fn update(
        &mut self,
        _: page::Entity,
        message: Message,
    ) -> cosmic::iced::Task<crate::app::Message> {
        match message {
            Message::AlwaysShow(value) => {
                if let Err(err) = self.config.set_always_shown(&self.config_handler, value) {
                    tracing::error!("failed to set OSK config: {}", err);
                }
            }
            Message::Config(config) => {
                self.config = config;
            }
            Message::GamepadShortcut(value) => {
                if let Err(err) = self
                    .config
                    .set_gamepad_shortcut(&self.config_handler, value)
                {
                    tracing::error!("failed to set OSK config: {}", err);
                }
            }
            Message::ImeActivation(value) => {
                if let Err(err) = self.config.set_ime_activation(&self.config_handler, value) {
                    tracing::error!("failed to set OSK config: {}", err);
                }
            }
        }

        cosmic::iced::Task::none()
    }
}
