use cosmic::Command;
use cosmic::{Apply, widget};
use cosmic::widget::{column, text};
use cosmic::{cosmic_theme::ThemeBuilder, widget::row, Element};
use cosmic_settings_page::{AutoBind, Info, Page as SettingsPage, Section};
use std::fs;

use crate::app;
use cosmic_settings_page::section::Entity;
use serde::{Deserialize, Serialize};

pub struct Page {}

impl Default for Page {
    fn default() -> Self {
        Self {}
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        match message {
            Message::SetTheme(theme) => Command::batch(vec![
                cosmic::command::message(
                    app::Message::PageMessage(crate::pages::Message::Appearance(
                        crate::pages::desktop::appearance::Message::ImportSuccess(Box::new(theme)),
                    ))
                )
            ])
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    SetTheme(ThemeBuilder),
}

impl AutoBind<crate::pages::Message> for Page {}

impl SettingsPage<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut slotmap::SlotMap<Entity, Section<crate::pages::Message>>,
    ) -> Option<cosmic_settings_page::Content> {
        Some(vec![sections.insert(reset_button())])
    }

    fn info(&self) -> Info {
        Info::new("themes", "utilities-tweak-tool-symbolic")
            .title("Themes")
            .description("Change the system theme.")
    }
}

#[allow(clippy::too_many_lines)]
pub fn reset_button() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![fl!("reset-to-default").into()])
        .view::<Page>(|_binder, page, section| {
            let Ok(themes) = get_themes() else {
                return row::with_children(vec![text("Error fetching themes").into()]).into();
            };

            let themes: Vec<SettingsTheme> = themes.list.iter().map(|t| t.clone().into()).collect();

            let txt: Vec<Element<crate::pages::Message>> = themes
                .iter()
                .map(|t| {
                    column::with_children(vec![
                        text(t.name.clone()).into(),
                        widget::button(text(t.light_name.clone()))
                            .on_press(Message::SetTheme(t.light_theme.clone().into()))
                            .into(),
                        widget::button(text(t.dark_name.clone()))
                            .on_press(Message::SetTheme(t.dark_theme.clone().into()))
                            .into(),
                    ]).apply(Element::from).map(crate::pages::Message::Themes)
                })
                .collect();
            let column = column::with_children(txt);
            column.into()
        })
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

struct SettingsTheme {
    name: String,
    light_name: String,
    dark_name: String,
    light_theme: ThemeBuilder,
    dark_theme: ThemeBuilder,
    light_wallpaper: String,
    dark_wallpaper: String,
}

impl From<GlobalTheme> for SettingsTheme {
    fn from(theme: GlobalTheme) -> Self {
        let light_file = std::fs::read_to_string(&theme.light.path).unwrap();
        let light: ThemeBuilder = ron::from_str(&light_file).unwrap();
        let dark_file = std::fs::read_to_string(&theme.dark.path).unwrap();
        let dark: ThemeBuilder = ron::from_str(&dark_file).unwrap();
        Self {
            name: theme.name,
            light_name: theme.light.name,
            dark_name: theme.dark.name,
            light_theme: light,
            dark_theme: dark,
            light_wallpaper: theme.light.wallpaper,
            dark_wallpaper: theme.dark.wallpaper,
        }
    }
}
