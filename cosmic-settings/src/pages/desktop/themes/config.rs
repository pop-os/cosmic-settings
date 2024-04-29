use cosmic::cosmic_config;
use cosmic::cosmic_config::Config;
use cosmic::cosmic_theme::ThemeBuilder;
use cosmic_config::cosmic_config_derive::CosmicConfigEntry;
use cosmic_config::{ConfigGet, CosmicConfigEntry};
use serde::{Deserialize, Serialize};
use std::fs;

pub const THEME_CONFIG_ID: &str = "com.system76.CosmicTheme.Theme";

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize, CosmicConfigEntry)]
#[version = 1]
pub struct Theme {
    pub name: String,
    pub path: String,
    pub wallpaper: String,
}

impl Theme {
    pub fn properties(&self) -> anyhow::Result<ThemeBuilder> {
        let file = fs::read_to_string(&self.path)?;
        let theme: ThemeBuilder = ron::from_str(&file)?;
        Ok(theme)
    }
}

impl Theme {
    pub const fn version() -> u64 {
        Self::VERSION
    }

    /// Get the config for the theme
    pub fn config() -> Result<Config, cosmic_config::Error> {
        Config::new(THEME_CONFIG_ID, Self::VERSION)
    }
}
