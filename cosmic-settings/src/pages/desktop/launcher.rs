use std::path::Path;

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{icon, settings, text};
use cosmic::{surface, widget};
use cosmic_config::{Config, CosmicConfigEntry as _};
use cosmic_launcher_config::IconSource;
use cosmic_settings_page::section::Entity;
use cosmic_settings_page::{self as page, Section};
use slotmap::SlotMap;
use tracing::error;

#[derive(Clone, Debug)]
pub enum Message {
    TogglePlugin(String, bool),
    Surface(surface::Action<crate::app::Message>),
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Launcher(message)
    }
}

#[derive(Clone, Debug)]
pub struct Page {
    config: cosmic_launcher_config::Config,
    config_handler: Option<Config>,
}

impl Default for Page {
    fn default() -> Self {
        let (config, config_handler) = cosmic_launcher_config::Config::load();
        Self {
            config,
            config_handler,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(launcher_plugins())])
    }

    fn info(&self) -> cosmic_settings_page::Info {
        page::Info::new("launcher", "preferences-launcher-symbolic")
            .title(fl!("launcher"))
            .description(fl!("xdg-entry-launcher-comment"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl Page {
    fn save_launcher_config(&mut self) {
        if let Some(config) = self.config_handler.as_ref()
            && let Err(err) = self.config.write_entry(config)
        {
            error!(?err, "Failed to set config 'launcher'");
        }
    }

    pub fn update(&mut self, message: Message) -> cosmic::iced::Task<crate::app::Message> {
        match message {
            Message::TogglePlugin(name, enable) => {
                if let Some(plugin) = self
                    .config
                    .plugins
                    .iter_mut()
                    .find(|plugin| plugin.name == name)
                {
                    plugin.disabled = !enable;
                }
                self.save_launcher_config();
            }
            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }
        }
        cosmic::iced::Task::none()
    }
}

fn launcher_plugins() -> Section<crate::pages::Message> {
    let cosmic::cosmic_theme::Spacing {
        space_xxs,
        space_xs,
        ..
    } = cosmic::theme::spacing();

    Section::default()
        .title(fl!("launcher-plugins"))
        .view::<Page>(move |_binder, page: &Page, _section| {
            let view = widget::column::with_capacity(1).spacing(space_xxs);
            let mut section = settings::section();

            if page.config.plugins.is_empty() {
                section = section.add(text::body(fl!("launcher-plugins", "none")))
            } else {
                for plugin in &page.config.plugins {
                    let mut row = widget::row::with_capacity(3)
                        .spacing(space_xs)
                        .align_y(Alignment::Center);

                    row = row.push(
                        match &plugin.icon {
                            Some(IconSource::Name(name)) => {
                                if Path::new(name.as_ref()).exists() {
                                    icon::from_path(Path::new(name.as_ref()).into()).icon()
                                } else {
                                    icon::from_name(name.as_ref()).icon()
                                }
                            }
                            Some(IconSource::Mime(mime)) => {
                                icon::from_name(mime.replace('/', "-")).icon()
                            }
                            None => icon::from_name("application-default").icon(),
                        }
                        .size(32),
                    );

                    row = row.push(text::body(&plugin.name).width(Length::Fill));

                    row = row.push(widget::toggler(!plugin.disabled).on_toggle(|enabled| {
                        Message::TogglePlugin(plugin.name.clone(), enabled).into()
                    }));

                    section = section.add(row);
                }
            }

            view.push(section).into()
        })
}
