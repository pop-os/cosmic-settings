// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::widget::{settings, text};
use cosmic_settings_page::{self as page, section, Section};
use slotmap::SlotMap;

// crate::cache_dynamic_lazy! {
//     pub static SOUND_ALERTS_VOLUME: String = fl!("sound-alerts", "volume");
//     pub static SOUND_ALERTS_SOUND: String = fl!("sound-alerts", "sound");

//     pub static SOUND_APPLICATIONS_DESC: String = fl!("sound-applications", "desc");

//     pub static SOUND_INPUT_VOLUME: String = fl!("sound-input", "volume");
//     pub static SOUND_INPUT_DEVICE: String = fl!("sound-input", "device");
//     pub static SOUND_INPUT_LEVEL: String = fl!("sound-input", "level");

//     pub static SOUND_OUTPUT_VOLUME: String = fl!("sound-output", "volume");
//     pub static SOUND_OUTPUT_DEVICE: String = fl!("sound-output", "device");
//     pub static SOUND_OUTPUT_LEVEL: String = fl!("sound-output", "level");
//     pub static SOUND_OUTPUT_CONFIG: String = fl!("sound-output", "config");
//     pub static SOUND_OUTPUT_BALANCE: String = fl!("sound-output", "balance");
// }

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
            fl!("sound-alerts", "volume").into(),
            fl!("sound-alerts", "sound").into(),
        ])
        .view::<Page>(|_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(&*section.descriptions[0], text("TODO")))
                .add(settings::item(&*section.descriptions[1], text("TODO")))
                .into()
        })
}

fn applications() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("sound-applications"))
        .descriptions(vec![fl!("sound-applications", "desc").into()])
        .view::<Page>(|_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(&*section.descriptions[0], text("TODO")))
                .into()
        })
}

fn input() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("sound-input"))
        .descriptions(vec![
            fl!("sound-input", "volume").into(),
            fl!("sound-input", "device").into(),
            fl!("sound-input", "level").into(),
        ])
        .view::<Page>(|_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(&*section.descriptions[0], text("TODO")))
                .add(settings::item(&*section.descriptions[1], text("TODO")))
                .add(settings::item(&*section.descriptions[2], text("TODO")))
                .into()
        })
}

fn output() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("sound-output"))
        .descriptions(vec![
            fl!("sound-output", "volume").into(),
            fl!("sound-output", "device").into(),
            fl!("sound-output", "level").into(),
            fl!("sound-output", "config").into(),
            fl!("sound-output", "balance").into(),
        ])
        .view::<Page>(|_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(&*section.descriptions[0], text("TODO")))
                .add(settings::item(&*section.descriptions[1], text("TODO")))
                .add(settings::item(&*section.descriptions[2], text("TODO")))
                .add(settings::item(&*section.descriptions[3], text("TODO")))
                .into()
        })
}
