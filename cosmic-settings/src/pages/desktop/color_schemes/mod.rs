pub mod config;
pub mod page;

pub use page::{Message, Page};

use crate::pages::desktop::color_schemes::config::ColorSchemeVariant;
use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix::raw::off_t;
use std::path::PathBuf;
use std::sync::OnceLock;

static COLOR_SCHEMES: OnceLock<Vec<ColorScheme>> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ColorScheme {
    name: String,
    light: ColorSchemeVariant,
    dark: ColorSchemeVariant,
}

impl ColorScheme {
    pub fn fetch_color_schemes() -> anyhow::Result<()> {
        let mut color_schemes = vec![];
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
                        let mut color_scheme: ColorScheme = ron::from_str(&theme_data)?;

                        color_scheme.light.path = path
                            .join(&color_scheme.light.path)
                            .to_str()
                            .unwrap_or_default()
                            .to_string();
                        color_scheme.dark.path = path
                            .join(&color_scheme.dark.path)
                            .to_str()
                            .unwrap_or_default()
                            .to_string();
                        color_schemes.push(color_scheme);
                    }
                }
            }
        }

        COLOR_SCHEMES.get_or_init(|| color_schemes);
        Ok(())
    }
}
