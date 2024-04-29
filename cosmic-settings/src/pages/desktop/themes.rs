use cosmic::{Command, cosmic_config};
use cosmic::{Apply, widget};
use cosmic::widget::{button, column, flex_row, text};
use cosmic::{cosmic_theme::ThemeBuilder, widget::row, Element};
use cosmic_settings_page::{AutoBind, Info, Page as SettingsPage, Section};
use std::fs;
use color_eyre::owo_colors::OwoColorize;
use cosmic::cosmic_config::cosmic_config_derive::CosmicConfigEntry;
use cosmic::iced::{Alignment, Color};

use crate::app;
use cosmic_settings_page::section::Entity;
use serde::{Deserialize, Serialize};
use crate::pages::desktop::appearance::color_button;

pub struct Page {
    current_theme: String
}

impl Default for Page {
    fn default() -> Self {
        Self {
            current_theme: String::new(),
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        match message {
            Message::SetTheme(is_dark, theme) => {
                let path = if is_dark {
                    theme.dark.path.clone()
                } else {
                    theme.light.path.clone()
                };
                let theme: SettingsTheme = theme.into();
                let theme = if is_dark {
                    theme.dark
                } else {
                    theme.light
                };
                self.current_theme = path;
                Command::batch(vec![
                    cosmic::command::message(
                        app::Message::PageMessage(crate::pages::Message::Appearance(
                            crate::pages::desktop::appearance::Message::DarkMode(is_dark),
                        ))
                    ),
                    cosmic::command::message(
                        app::Message::PageMessage(crate::pages::Message::Appearance(
                            crate::pages::desktop::appearance::Message::ImportSuccess(Box::new(theme)),
                        ))
                    )
                ])
            }
        }
    }

    pub fn theme_list(&self) -> Section<crate::pages::Message> {
        let current_theme = self.current_theme.clone();
        Section::default()
            .descriptions(vec![fl!("reset-to-default").into()])
            .view::<Page>(move |_binder, page, section| {
                let Ok(themes) = get_themes() else {
                    return row::with_children(vec![text("Error fetching themes").into()]).into();
                };

                let themes: Vec<Element<crate::pages::Message>> = themes.list
                    .iter()
                    .map(|theme| {
                        let global_theme = theme.clone();
                        let settings_theme: SettingsTheme = theme.clone().into();

                        let light_name = settings_theme.light_name.clone();
                        let dark_name = settings_theme.dark_name.clone();
                        let light_theme = settings_theme.light.clone();
                        let dark_theme = settings_theme.dark.clone();

                        widget::container(
                            column::with_children(vec![
                                text(settings_theme.name.clone()).into(),
                                flex_row(vec![
                                    Self::theme_button(global_theme.clone(), false, current_theme.clone()),
                                    Self::theme_button(global_theme, true, current_theme.clone())
                                ]).into()
                            ])
                                .spacing(cosmic::theme::active().cosmic().space_xs())
                                .padding(cosmic::theme::active().cosmic().space_xs())
                                .align_items(Alignment::Center)
                                .apply(Element::from)
                                .map(crate::pages::Message::Themes)
                        )
                            .style(cosmic::theme::Container::Card)
                            .into()
                    })
                    .collect();
                flex_row(themes)
                    .row_spacing(cosmic::theme::active().cosmic().space_xxs())
                    .column_spacing(cosmic::theme::active().cosmic().space_xxs())
                    .into()
            })
    }

    pub fn theme_button<'a>(global_theme: GlobalTheme, is_dark: bool, current_theme: String) -> Element<'a, Message> {
        let icon_size = 40;
        let name = if is_dark { global_theme.dark.name.clone() } else { global_theme.light.name.clone() };
        let theme: SettingsTheme = global_theme.clone().into();
        let theme = if is_dark {
            theme.dark
        } else {
            theme.light
        };
        if global_theme.dark.path == current_theme {
            println!("TES")
        }

        let selected = if is_dark {
            global_theme.dark.path == current_theme
        } else {
            global_theme.light.path == current_theme
        };
        widget::button(
            column::with_children(vec![
                text(name).into(),
                row::with_children(vec![
                    color_button(None, Color::from(theme.bg_color.unwrap_or_default()), false, icon_size, icon_size).into(),
                    color_button(None, Color::from(theme.primary_container_bg.unwrap_or_default()), false, icon_size, icon_size).into()
                ]).spacing(cosmic::theme::active().cosmic().space_xxxs()).into(),
                row::with_children(vec![
                    color_button(None, Color::from(theme.accent.unwrap_or_default()), false, icon_size, icon_size).into(),
                    color_button(None, Color::from(theme.text_tint.unwrap_or_default()), false, icon_size, icon_size).into()
                ]).spacing(cosmic::theme::active().cosmic().space_xxxs()).into(),
            ])
                .spacing(cosmic::theme::active().cosmic().space_xxxs())
                .padding(cosmic::theme::active().cosmic().space_xs())
                .align_items(Alignment::Center)
        )
            .selected(selected)
            .style(button::Style::Image)
            .on_press(Message::SetTheme(is_dark, global_theme.clone().into()))
            .into()
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    SetTheme(bool, GlobalTheme),
}

impl AutoBind<crate::pages::Message> for Page {}

impl SettingsPage<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut slotmap::SlotMap<Entity, Section<crate::pages::Message>>,
    ) -> Option<cosmic_settings_page::Content> {
        Some(vec![sections.insert(self.theme_list())])
    }

    fn info(&self) -> Info {
        Info::new("themes", "utilities-tweak-tool-symbolic")
            .title("Themes")
            .description("Change the system theme.")
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalTheme {
    pub name: String,
    pub light: Theme,
    pub dark: Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub path: String,
    pub wallpaper: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Themes {
    pub list: Vec<GlobalTheme>,
}

impl Themes {
    pub fn add(&mut self, theme: GlobalTheme) {
        self.list.push(theme);
    }
}

pub fn get_themes() -> anyhow::Result<Themes> {
    let mut themes = Themes::default();

    for entry in fs::read_dir("resources/themes")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let theme_data = fs::read_to_string(path.join("theme.ron"))?;
            let mut theme: GlobalTheme = ron::from_str(&theme_data)?;

            theme.light.path = path.join(&theme.light.path).to_str().unwrap().to_string();
            theme.dark.path = path.join(&theme.dark.path).to_str().unwrap().to_string();
            theme.light.wallpaper = path
                .join(&theme.light.wallpaper)
                .to_str()
                .unwrap()
                .to_string();
            theme.dark.wallpaper = path
                .join(&theme.dark.wallpaper)
                .to_str()
                .unwrap()
                .to_string();
            themes.add(theme);
        }
    }

    Ok(themes)
}

#[derive(Debug, Clone)]
struct SettingsTheme {
    name: String,
    light: ThemeBuilder,
    dark: ThemeBuilder,
    light_name: String,
    dark_name: String,
    light_wallpaper: String,
    dark_wallpaper: String,
}

impl From<GlobalTheme> for SettingsTheme {
    fn from(theme: GlobalTheme) -> Self {
        let light_file = fs::read_to_string(&theme.light.path).unwrap();
        let light: ThemeBuilder = ron::from_str(&light_file).unwrap();
        let dark_file = fs::read_to_string(&theme.dark.path).unwrap();
        let dark: ThemeBuilder = ron::from_str(&dark_file).unwrap();
        Self {
            name: theme.name,
            light_name: theme.light.name,
            dark_name: theme.dark.name,
            light,
            dark,
            light_wallpaper: theme.light.wallpaper,
            dark_wallpaper: theme.dark.wallpaper,
        }
    }
}
