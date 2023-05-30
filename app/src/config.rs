// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::cosmic_config::{self, ConfigGet, ConfigSet};

const NAME: &str = "com.system76.CosmicSettings";

const ACTIVE_PAGE: &str = "active-page";

#[must_use]
#[derive(Debug)]
pub struct Config {
    pub cosmic_config: Option<cosmic_config::Config>,
    pub active_page: Box<str>,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Self::default();

        let context = match cosmic_config::Config::new(NAME, 1) {
            Ok(context) => context,
            Err(why) => {
                tracing::warn!(?why, "failed to get config");
                return Self::default();
            }
        };

        if let Ok(page) = context.get::<Box<str>>(ACTIVE_PAGE) {
            config.active_page = page;
        }

        config.cosmic_config = Some(context);

        config
    }

    pub fn set_active_page(&mut self, page: Box<str>) {
        if let Some(context) = self.cosmic_config.as_ref() {
            if let Err(why) = context.set::<Box<str>>(ACTIVE_PAGE, page.clone()) {
                tracing::error!(?why, "failed to store active page ID");
            }
        }

        self.active_page = page;
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cosmic_config: None,
            active_page: Box::from("desktop"),
        }
    }
}
