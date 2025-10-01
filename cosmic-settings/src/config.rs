// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    cosmic_config::{self, ConfigGet, ConfigSet},
    cosmic_theme::palette::Srgba,
};

const NAME: &str = "com.system76.CosmicSettings";

const ACTIVE_PAGE: &str = "active_page";
const ACCENT_PALETTE_DARK: &str = "accent_palette_dark";
const ACCENT_PALETTE_LIGHT: &str = "accent_palette_light";

#[must_use]
#[derive(Debug, Clone)]
pub struct Config {
    config: cosmic_config::Config,
    state: cosmic_config::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let config = match cosmic_config::Config::new(NAME, 1) {
            Ok(config) => config,
            Err(why) => {
                panic!("failed to get {NAME} config: {:?}", why);
            }
        };

        let state = match cosmic_config::Config::new_state(NAME, 1) {
            Ok(state) => state,
            Err(why) => {
                panic!("failed to get {NAME} state: {:?}", why);
            }
        };

        Self { config, state }
    }

    pub fn accent_palette_dark(&self) -> Result<Vec<Srgba>, cosmic_config::Error> {
        self.config.get::<Vec<Srgba>>(ACCENT_PALETTE_DARK)
    }

    pub fn accent_palette_light(&self) -> Result<Vec<Srgba>, cosmic_config::Error> {
        self.config.get::<Vec<Srgba>>(ACCENT_PALETTE_LIGHT)
    }

    pub fn active_page(&self) -> Box<str> {
        self.state
            .get::<Box<str>>(ACTIVE_PAGE)
            .unwrap_or_else(|_| Box::from("desktop"))
    }

    pub fn set_active_page(&self, page: Box<str>) {
        if let Err(why) = self.state.set::<Box<str>>(ACTIVE_PAGE, page.clone()) {
            tracing::error!(?why, "failed to store active page ID");
        }
    }
}
