// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod appearance;
pub mod dock;
pub mod panel;
pub mod wallpaper;
pub mod window_management;
pub mod workspaces;

use cosmic::{config::CosmicTk, cosmic_config::CosmicConfigEntry};
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_page as page;
use tracing::error;

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct Page {
    pub cosmic_tk_config: Option<cosmic::cosmic_config::Config>,
    pub cosmic_tk: CosmicTk,

    pub comp_config: cosmic::cosmic_config::Config,

    show_active_hint: bool,
}

impl Default for Page {
    fn default() -> Self {
        let (cosmic_tk, cosmic_tk_config) = CosmicTk::config().map_or_else(
            |why| {
                tracing::error!(?why, "failed to read CosmicTk config");
                (CosmicTk::default(), None)
            },
            |config| match CosmicTk::get_entry(&config) {
                Ok(tk) => (tk, Some(config)),
                Err((_errors, tk)) => (tk, Some(config)),
            },
        );

        let comp_config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let show_active_hint = comp_config
            .get("active_hint")
            .inspect_err(|err| {
                if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                    error!(?err, "Failed to read config 'active_hint'")
                }
            })
            .unwrap_or(true);

        Self {
            cosmic_tk_config,
            cosmic_tk,
            comp_config,
            show_active_hint,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("desktop", "video-display-symbolic").title(fl!("desktop"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<wallpaper::Page>()
            .sub_page::<appearance::Page>()
            .sub_page::<panel::Page>()
            .sub_page::<dock::Page>()
            .sub_page::<window_management::Page>()
            .sub_page::<workspaces::Page>()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Message {
    ShowActiveWindowHint(bool),
    ShowMaximizeButton(bool),
    ShowMinimizeButton(bool),
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ShowMaximizeButton(value) => {
                if let Some(config) = self.cosmic_tk_config.as_mut() {
                    let _res = self.cosmic_tk.set_show_maximize(config, value);
                }
            }

            Message::ShowMinimizeButton(value) => {
                if let Some(config) = self.cosmic_tk_config.as_mut() {
                    let _res = self.cosmic_tk.set_show_minimize(config, value);
                }
            }
            Message::ShowActiveWindowHint(value) => {
                self.show_active_hint = value;
                if let Err(err) = self.comp_config.set("active_hint", value) {
                    error!(?err, "Failed to set config 'active_hint'");
                }
            }
        }
    }
}
