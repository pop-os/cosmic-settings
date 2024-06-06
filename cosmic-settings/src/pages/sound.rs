// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::widget::{settings, text};
use cosmic_settings_page::{self as page, section, Section};
use slab::Slab;
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
    let mut descriptions = Slab::new();
    let volume = descriptions.insert(fl!("sound-alerts", "volume"));
    let sound = descriptions.insert(fl!("sound-alerts", "sound"));

    Section::default()
        .title(fl!("sound-alerts"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(&section.descriptions[volume], text("TODO")))
                .add(settings::item(&section.descriptions[sound], text("TODO")))
                .into()
        })
}

fn applications() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let applications = descriptions.insert(fl!("sound-applications", "desc"));

    Section::default()
        .title(fl!("sound-applications"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(
                    &*section.descriptions[applications],
                    text("TODO"),
                ))
                .into()
        })
}

fn input() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let volume = descriptions.insert(fl!("sound-input", "volume"));
    let device = descriptions.insert(fl!("sound-input", "device"));
    let level = descriptions.insert(fl!("sound-input", "level"));

    Section::default()
        .title(fl!("sound-input"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(&*section.descriptions[volume], text("TODO")))
                .add(settings::item(&*section.descriptions[device], text("TODO")))
                .add(settings::item(&*section.descriptions[level], text("TODO")))
                .into()
        })
}

fn output() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let volume = descriptions.insert(fl!("sound-output", "volume"));
    let device = descriptions.insert(fl!("sound-output", "device"));
    let level = descriptions.insert(fl!("sound-output", "level"));
    let config = descriptions.insert(fl!("sound-output", "config"));
    // let balance = descriptions.insert(fl!("sound-output", "balance"));

    Section::default()
        .title(fl!("sound-output"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(&*section.descriptions[volume], text("TODO")))
                .add(settings::item(&*section.descriptions[device], text("TODO")))
                .add(settings::item(&*section.descriptions[level], text("TODO")))
                .add(settings::item(&*section.descriptions[config], text("TODO")))
                .into()
        })
}
