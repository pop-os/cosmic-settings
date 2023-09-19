// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::widget::{settings, text};
use cosmic_settings_page::{self as page, section, Section};
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(output()),
            sections.insert(input()),
            sections.insert(alerts()),
            sections.insert(applications()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("sound", "multimedia-volume-control-symbolic")
            .title(fl!("sound"))
            .description(fl!("sound", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn alerts() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("sound-alerts"))
        .descriptions(vec![
            fl!("sound-alerts", "volume"),
            fl!("sound-alerts", "sound"),
        ])
        .view::<Page>(|_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(&section.descriptions[0], text("TODO")))
                .add(settings::item(&section.descriptions[1], text("TODO")))
                .into()
        })
}

fn applications() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("sound-applications"))
        .descriptions(vec![fl!("sound-applications", "desc")])
        .view::<Page>(|_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(&section.descriptions[0], text("TODO")))
                .into()
        })
}

fn input() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("sound-input"))
        .descriptions(vec![
            fl!("sound-input", "volume"),
            fl!("sound-input", "device"),
            fl!("sound-input", "level"),
        ])
        .view::<Page>(|_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(&section.descriptions[0], text("TODO")))
                .add(settings::item(&section.descriptions[1], text("TODO")))
                .add(settings::item(&section.descriptions[2], text("TODO")))
                .into()
        })
}

fn output() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("sound-output"))
        .descriptions(vec![
            fl!("sound-output", "volume"),
            fl!("sound-output", "device"),
            fl!("sound-output", "level"),
            fl!("sound-output", "config"),
            fl!("sound-output", "balance"),
        ])
        .view::<Page>(|_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(&section.descriptions[0], text("TODO")))
                .add(settings::item(&section.descriptions[1], text("TODO")))
                .add(settings::item(&section.descriptions[2], text("TODO")))
                .add(settings::item(&section.descriptions[3], text("TODO")))
                .into()
        })
}
