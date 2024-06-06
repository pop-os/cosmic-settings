// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    cosmic_config::{self, ConfigGet, ConfigSet},
    iced::{widget::horizontal_space, Length},
    widget::{dropdown, settings},
    Apply,
};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slab::Slab;
use slotmap::SlotMap;
use tracing::error;

crate::cache_dynamic_lazy! {
    static WEEKDAYS: [String; 4] = [fl!("time-format", "friday"), fl!("time-format", "saturday"), fl!("time-format", "sunday"), fl!("time-format", "monday")];
}

pub struct Page {
    config: cosmic_config::Config,
    auto: bool,
    auto_timezone: bool,
    military_time: bool,
    first_day_of_week: usize,
    show_date_in_top_panel: bool,
}

impl Default for Page {
    fn default() -> Self {
        let config = cosmic_config::Config::new("com.system76.CosmicAppletTime", 1).unwrap();
        let military_time = config.get("military_time").unwrap_or_else(|err| {
            error!(?err, "Failed to read config 'military_time'");
            false
        });
        let first_day_of_week = config.get("first_day_of_week").unwrap_or_else(|err| {
            error!(?err, "Failed to read config 'first_day_of_week'");
            6
        });
        let show_date_in_top_panel = config.get("show_date_in_top_panel").unwrap_or_else(|err| {
            error!(?err, "Failed to read config 'show_date_in_top_panel'");
            true
        });
        Self {
            config,
            auto: false,
            auto_timezone: false,
            military_time,
            first_day_of_week,
            show_date_in_top_panel,
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
            Message::FirstDayOfWeek(weekday) => {
                self.first_day_of_week = weekday;
                if let Err(err) = self.config.set("first_day_of_week", weekday) {
                    error!(?err, "Failed to set config 'first_day_of_week'");
                }
            }
            Message::ShowDate(enable) => {
                self.show_date_in_top_panel = enable;
                if let Err(err) = self.config.set("show_date_in_top_panel", enable) {
                    error!(?err, "Failed to set config 'show_date_in_top_panel'");
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
    FirstDayOfWeek(usize),
    ShowDate(bool),
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn date() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let auto = descriptions.insert(fl!("time-date", "auto"));
    let title = descriptions.insert(fl!("time-date"));

    Section::default()
        .title(fl!("time-date"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            settings::view_section(&section.title)
                .add(
                    settings::item::builder(&*section.descriptions[auto])
                        .toggler(page.auto, Message::Automatic),
                )
                .add(settings::item(
                    &*section.descriptions[title],
                    horizontal_space(Length::Fill),
                ))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::DateAndTime)
        })
}

fn format() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let military = descriptions.insert(fl!("time-format", "twenty-four"));
    let first = descriptions.insert(fl!("time-format", "first"));
    let show_date = descriptions.insert(fl!("time-format", "show-date"));

    Section::default()
        .title(fl!("time-format"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            settings::view_section(&section.title)
                // 24-hour toggle
                .add(
                    settings::item::builder(&section.descriptions[military])
                        .toggler(page.military_time, Message::MilitaryTime),
                )
                // First day of week
                .add(
                    settings::item::builder(&section.descriptions[first]).flex_control(dropdown(
                        &*WEEKDAYS,
                        match page.first_day_of_week {
                            4 => Some(0), // friday
                            5 => Some(1), // saturday
                            0 => Some(3), // monday
                            _ => Some(2), // sunday
                        },
                        |v| {
                            match v {
                                0 => Message::FirstDayOfWeek(4), // friday
                                1 => Message::FirstDayOfWeek(5), // saturday
                                3 => Message::FirstDayOfWeek(0), // monday
                                _ => Message::FirstDayOfWeek(6), // sunday
                            }
                        },
                    )),
                )
                // Date on top panel toggle
                .add(
                    settings::item::builder(&section.descriptions[show_date])
                        .toggler(page.show_date_in_top_panel, Message::ShowDate),
                )
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::DateAndTime)
        })
}

fn timezone() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let auto = descriptions.insert(fl!("time-zone", "auto"));
    let auto_info = descriptions.insert(fl!("time-zone", "auto-info"));
    let time_zone = descriptions.insert(fl!("time-zone"));

    Section::default()
        .title(fl!("time-zone"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            settings::view_section(&section.title)
                // Automatic timezone toggle
                .add(
                    settings::item::builder(&*section.descriptions[auto])
                        .description(&*section.descriptions[auto_info])
                        .toggler(page.auto_timezone, Message::AutomaticTimezone),
                )
                // Time zone select
                .add(
                    settings::item::builder(&*section.descriptions[time_zone])
                        .control(horizontal_space(Length::Fill)),
                )
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::DateAndTime)
        })
}
