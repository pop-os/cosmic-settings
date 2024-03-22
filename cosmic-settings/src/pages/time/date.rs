// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    cosmic_config::{self, ConfigGet, ConfigSet},
    iced::{widget::horizontal_space, Length},
    widget::settings,
    Apply,
};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
// use icu::calendar::{DateTime, Gregorian};

use slotmap::SlotMap;
use tracing::error;

pub struct Page {
    config: cosmic_config::Config,
    auto: bool,
    auto_timezone: bool,
    military_time: bool,
    // info: Option<cosmic_settings_time::Info>,
}

impl Default for Page {
    fn default() -> Self {
        let config = cosmic_config::Config::new("com.system76.CosmicAppletTime", 1).unwrap();
        let military_time = config.get("military_time").unwrap_or_else(|err| {
            error!(?err, "Failed to read config 'military_time'");
            false
        });
        Self {
            config,
            auto: false,
            auto_timezone: false,
            military_time,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(date()),
            sections.insert(timezone()),
            sections.insert(format()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("time-date", "preferences-system-time-symbolic")
            .title(fl!("time-date"))
            .description(fl!("time-date", "desc"))
    }
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Automatic(enable) => self.auto = enable,
            Message::AutomaticTimezone(enable) => self.auto_timezone = enable,
            Message::MilitaryTime(enable) => {
                self.military_time = enable;
                if let Err(err) = self.config.set("military_time", enable) {
                    error!(?err, "Failed to set config 'military_time'");
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Automatic(bool),
    AutomaticTimezone(bool),
    MilitaryTime(bool),
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn date() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("time-date"))
        .descriptions(vec![
            fl!("time-date", "auto").into(),
            fl!("time-date").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            settings::view_section(&section.title)
                .add(
                    settings::item::builder(&*section.descriptions[0])
                        .toggler(page.auto, Message::Automatic),
                )
                .add(settings::item(
                    &*section.descriptions[1],
                    horizontal_space(Length::Fill),
                ))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::DateAndTime)
        })
}

fn format() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("time-format"))
        .descriptions(vec![
            fl!("time-format", "twenty-four").into(),
            fl!("time-format", "first").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            settings::view_section(&section.title)
                // 24-hour toggle
                .add(
                    settings::item::builder(&*section.descriptions[0])
                        .toggler(page.military_time, Message::MilitaryTime),
                )
                // First day of week
                .add(settings::item(
                    &*section.descriptions[1],
                    horizontal_space(Length::Fill),
                ))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::DateAndTime)
        })
}

fn timezone() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("time-zone"))
        .descriptions(vec![
            fl!("time-zone", "auto").into(),
            fl!("time-zone", "auto-info").into(),
            fl!("time-zone").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            settings::view_section(&section.title)
                // Automatic timezone toggle
                .add(
                    settings::item::builder(&*section.descriptions[0])
                        .description(&*section.descriptions[1])
                        .toggler(page.auto_timezone, Message::AutomaticTimezone),
                )
                // Time zone select
                .add(
                    settings::item::builder(&*section.descriptions[2])
                        .control(horizontal_space(Length::Fill)),
                )
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::DateAndTime)
        })
}
