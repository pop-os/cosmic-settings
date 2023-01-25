// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::page::{self, Content, Section};
use cosmic::{iced, widget::settings};
use slotmap::SlotMap;

use super::section;

#[derive(Default)]
pub struct Sound {}

pub struct Page;

impl page::Page for Page {
    type Model = Sound;

    const PERSISTENT_ID: &'static str = "sound";

    fn page() -> page::Meta {
        page::Meta::default()
            .title(fl!("sound"))
            .description(fl!("sound", "desc"))
            .icon_name("multimedia-volume-control-symbolic")
    }

    fn content(sections: &mut SlotMap<section::Entity, Section>) -> Option<Content> {
        Some(vec![
            sections.insert(output()),
            sections.insert(input()),
            sections.insert(alerts()),
            sections.insert(applications()),
        ])
    }
}

fn alerts() -> Section {
    Section::new()
        .title(fl!("sound-alerts"))
        .descriptions(vec![
            fl!("sound-alerts", "volume"),
            fl!("sound-alerts", "sound"),
        ])
        .view_fn(|app, section| {
            let _sound = app
                .pages
                .resource::<Sound>()
                .expect("sound model is missing");

            settings::view_section(&section.title)
                .add(settings::item(
                    &section.descriptions[0],
                    iced::widget::text("TODO"),
                ))
                .add(settings::item(
                    &section.descriptions[1],
                    iced::widget::text("TODO"),
                ))
                .into()
        })
}

fn applications() -> Section {
    Section::new()
        .title(fl!("sound-applications"))
        .descriptions(vec![fl!("sound-applications", "desc")])
        .view_fn(|app, section| {
            let _sound = app
                .pages
                .resource::<Sound>()
                .expect("sound model is missing");

            settings::view_section(&section.title)
                .add(settings::item(
                    &section.descriptions[0],
                    iced::widget::text("TODO"),
                ))
                .into()
        })
}

fn input() -> Section {
    Section::new()
        .title(fl!("sound-input"))
        .descriptions(vec![
            fl!("sound-input", "volume"),
            fl!("sound-input", "device"),
            fl!("sound-input", "level"),
        ])
        .view_fn(|app, section| {
            let _sound = app
                .pages
                .resource::<Sound>()
                .expect("sound model is missing");

            settings::view_section(&section.title)
                .add(settings::item(
                    &section.descriptions[0],
                    iced::widget::text("TODO"),
                ))
                .add(settings::item(
                    &section.descriptions[1],
                    iced::widget::text("TODO"),
                ))
                .add(settings::item(
                    &section.descriptions[2],
                    iced::widget::text("TODO"),
                ))
                .into()
        })
}

fn output() -> Section {
    Section::new()
        .title(fl!("sound-output"))
        .descriptions(vec![
            fl!("sound-output", "volume"),
            fl!("sound-output", "device"),
            fl!("sound-output", "level"),
            fl!("sound-output", "config"),
            fl!("sound-output", "balance"),
        ])
        .view_fn(|app, section| {
            let _sound = app
                .pages
                .resource::<Sound>()
                .expect("sound model is missing");

            settings::view_section(&section.title)
                .add(settings::item(
                    &section.descriptions[0],
                    iced::widget::text("TODO"),
                ))
                .add(settings::item(
                    &section.descriptions[1],
                    iced::widget::text("TODO"),
                ))
                .add(settings::item(
                    &section.descriptions[2],
                    iced::widget::text("TODO"),
                ))
                .add(settings::item(
                    &section.descriptions[3],
                    iced::widget::text("TODO"),
                ))
                .into()
        })
}
