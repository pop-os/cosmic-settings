pub mod config;
pub mod page;

pub use page::{Message, Page};

use crate::pages::desktop::themes::config::Theme;
use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix::raw::off_t;
use std::path::PathBuf;
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
        let xdg_data_home = std::env::var("XDG_DATA_HOME")
            .ok()
            .and_then(|value| {
                if value.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(value))
                }
            })
            .or_else(dirs::data_local_dir)
            .map(|dir| dir.join("themes"));

        let xdg_data_dirs = std::env::var("XDG_DATA_DIRS").ok();

        let xdg_data_dirs = xdg_data_dirs
            .as_deref()
            // Default from the XDG Base Directory Specification
            .or(Some("/usr/local/share/:/usr/share/"))
            .into_iter()
            .flat_map(|arg| std::env::split_paths(arg).map(|dir| dir.join("themes")));

        for theme_dir in xdg_data_dirs.chain(xdg_data_home) {
            let Ok(read_dir) = fs::read_dir(&theme_dir) else {
                continue;
            };

            for entry in read_dir.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    let theme_file = path.join("theme.ron");
                    if theme_file.exists() {
                        let theme_data = fs::read_to_string(theme_file)?;
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
            }
        }

        THEMES.get_or_init(|| themes);
        Ok(())
    }
}
