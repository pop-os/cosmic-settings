pub mod config;
pub mod page;

pub use page::{Message, Page};

use crate::pages::desktop::themes::config::Theme;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::OnceLock;

static THEMES: OnceLock<Vec<GlobalTheme>> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GlobalTheme {
    name: String,
    light: Theme,
    dark: Theme,
}

impl GlobalTheme {
    pub fn init_themes() -> anyhow::Result<()> {
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

        THEMES.get_or_init(|| themes);
        Ok(())
    }
}
