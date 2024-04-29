pub mod config;
pub mod page;

pub use page::{Message, Page};

use crate::pages::desktop::themes::config::Theme;
use cosmic::cosmic_theme::ThemeBuilder;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GlobalTheme {
    pub name: String,
    pub light: Theme,
    pub dark: Theme,
}

impl GlobalTheme {
    pub fn theme_name(&self) -> &str {
        &self.name
    }

    pub fn light_name(&self) -> &str {
        &self.light.name
    }

    pub fn dark_name(&self) -> &str {
        &self.dark.name
    }

    pub fn get_themes() -> anyhow::Result<Vec<GlobalTheme>> {
        let mut themes = vec![];

        for entry in fs::read_dir("resources/themes")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let theme_data = fs::read_to_string(path.join("theme.ron"))?;
                let mut theme: GlobalTheme = ron::from_str(&theme_data)?;

                theme.light.path = path
                    .join(&theme.light.path)
                    .to_str()
                    .unwrap_or_default()
                    .to_string();
                theme.dark.path = path
                    .join(&theme.dark.path)
                    .to_str()
                    .unwrap_or_default()
                    .to_string();
                theme.light.wallpaper = path
                    .join(&theme.light.wallpaper)
                    .to_str()
                    .unwrap_or_default()
                    .to_string();
                theme.dark.wallpaper = path
                    .join(&theme.dark.wallpaper)
                    .to_str()
                    .unwrap_or_default()
                    .to_string();
                themes.push(theme);
            }
        }

        Ok(themes)
    }
}
