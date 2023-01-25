// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::page::{self, section, Content, Section};
use apply::Apply;
use cosmic::{
    iced::{widget::horizontal_space, Length},
    widget::settings,
};
use slotmap::SlotMap;

#[derive(Default)]
pub struct Model {
    auto: bool,
    auto_timezone: bool,
    military_time: bool,
}

impl Model {
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

pub struct Page;

impl page::Page for Page {
    type Model = Model;

    const PERSISTENT_ID: &'static str = "time-date";

    fn page() -> page::Meta {
        page::Meta::default()
            .title(fl!("time-date"))
            .description(fl!("time-date", "desc"))
            .icon_name("preferences-system-time-symbolic")
    }

    fn content(sections: &mut SlotMap<section::Entity, Section>) -> Option<Content> {
        Some(vec![
            sections.insert(date()),
            sections.insert(timezone()),
            sections.insert(format()),
        ])
    }
}

fn date() -> Section {
    Section::new()
        .title(fl!("time-date"))
        .descriptions(vec![fl!("time-date", "auto"), fl!("time-date")])
        .view_fn(|app, section| {
            let model = app
                .pages
                .resource::<Model>()
                .expect("time & date model not found");

            settings::view_section(&section.title)
                .add(
                    settings::item::builder(&section.descriptions[0])
                        .toggler(model.auto, Message::Automatic),
                )
                .add(settings::item(
                    &section.descriptions[1],
                    horizontal_space(Length::Fill),
                ))
                .apply(cosmic::Element::from)
                .map(crate::Message::DateAndTime)
        })
}

fn format() -> Section {
    Section::new()
        .title(fl!("time-format"))
        .descriptions(vec![
            fl!("time-format", "twenty-four"),
            fl!("time-format", "first"),
        ])
        .view_fn(|app, section| {
            let model = app
                .pages
                .resource::<Model>()
                .expect("time & date model not found");

            settings::view_section(&section.title)
                // 24-hour toggle
                .add(
                    settings::item::builder(&section.descriptions[0])
                        .toggler(model.military_time, Message::MilitaryTime),
                )
                // First day of week
                .add(settings::item(
                    &section.descriptions[1],
                    horizontal_space(Length::Fill),
                ))
                .apply(cosmic::Element::from)
                .map(crate::Message::DateAndTime)
        })
}

fn timezone() -> Section {
    Section::new()
        .title(fl!("time-zone"))
        .descriptions(vec![
            fl!("time-zone", "auto"),
            fl!("time-zone", "auto-info"),
            fl!("time-zone"),
        ])
        .view_fn(|app, section| {
            let model = app
                .pages
                .resource::<Model>()
                .expect("time & date model not found");

            settings::view_section(&section.title)
                // Automatic timezone toggle
                .add(
                    settings::item::builder(&section.descriptions[0])
                        .description(&section.descriptions[1])
                        .toggler(model.auto_timezone, Message::AutomaticTimezone),
                )
                // Time zone select
                .add(
                    settings::item::builder(&section.descriptions[2])
                        .control(horizontal_space(Length::Fill)),
                )
                .apply(cosmic::Element::from)
                .map(crate::Message::DateAndTime)
        })
}
