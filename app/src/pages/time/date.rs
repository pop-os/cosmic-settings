// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use apply::Apply;
use cosmic::{
    iced::{widget::horizontal_space, Length},
    widget::settings,
};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
// use icu::calendar::{DateTime, Gregorian};

use slotmap::SlotMap;

#[derive(Default)]
pub struct Page {
    auto: bool,
    auto_timezone: bool,
    military_time: bool,
    // info: Option<cosmic_settings_time::Info>,
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

    fn load(&self, _page: page::Entity) -> Option<page::Task<crate::pages::Message>> {
        None
    }
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Automatic(enable) => self.auto = enable,
            Message::AutomaticTimezone(enable) => self.auto_timezone = enable,
            Message::MilitaryTime(enable) => self.military_time = enable,
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
        .descriptions(vec![fl!("time-date", "auto"), fl!("time-date")])
        .view::<Page>(|_binder, page, section| {
            settings::view_section(&section.title)
                .add(
                    settings::item::builder(&section.descriptions[0])
                        .toggler(page.auto, Message::Automatic),
                )
                .add(settings::item(
                    &section.descriptions[1],
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
            fl!("time-format", "twenty-four"),
            fl!("time-format", "first"),
        ])
        .view::<Page>(|_binder, page, section| {
            settings::view_section(&section.title)
                // 24-hour toggle
                .add(
                    settings::item::builder(&section.descriptions[0])
                        .toggler(page.military_time, Message::MilitaryTime),
                )
                // First day of week
                .add(settings::item(
                    &section.descriptions[1],
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
            fl!("time-zone", "auto"),
            fl!("time-zone", "auto-info"),
            fl!("time-zone"),
        ])
        .view::<Page>(|_binder, page, section| {
            settings::view_section(&section.title)
                // Automatic timezone toggle
                .add(
                    settings::item::builder(&section.descriptions[0])
                        .description(&section.descriptions[1])
                        .toggler(page.auto_timezone, Message::AutomaticTimezone),
                )
                // Time zone select
                .add(
                    settings::item::builder(&section.descriptions[2])
                        .control(horizontal_space(Length::Fill)),
                )
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::DateAndTime)
        })
}
