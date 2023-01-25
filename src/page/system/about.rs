// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::page::{self, Content, Section};
use cosmic::{
    iced::{
        widget::{horizontal_space, row},
        Length,
    },
    widget::{icon, list_column, settings, text},
};
use slotmap::SlotMap;

pub struct Page;

impl page::Page for Page {
    type Model = super::Model;

    const PERSISTENT_ID: &'static str = "about";

    fn page() -> page::Meta {
        page::Meta::default()
            .title(fl!("about"))
            .description(fl!("about", "desc"))
            .icon_name("help-about-symbolic")
    }

    fn content(sections: &mut SlotMap<page::section::Entity, Section>) -> Option<Content> {
        Some(vec![
            sections.insert(distributor_logo()),
            sections.insert(device()),
            sections.insert(hardware()),
            sections.insert(os()),
            sections.insert(related()),
        ])
    }
}

fn device() -> Section {
    Section::new()
        .descriptions(vec![fl!("about-device")])
        .view_fn(|_app, section| {
            list_column()
                .add(settings::item(&section.descriptions[0], text("TODO")))
                .into()
        })
}

fn distributor_logo() -> Section {
    Section::new().search_ignore().view_fn(|_app, _section| {
        row!(
            horizontal_space(Length::Fill),
            icon("distributor-logo", 78),
            horizontal_space(Length::Fill),
        )
        .into()
    })
}

fn hardware() -> Section {
    Section::new()
        .title(fl!("about-hardware"))
        .descriptions(vec![
            fl!("about-hardware", "model"),
            fl!("about-hardware", "memory"),
            fl!("about-hardware", "processor"),
            fl!("about-hardware", "graphics"),
            fl!("about-hardware", "disk-capacity"),
        ])
        .view_fn(|_app, section| {
            let desc = &section.descriptions;
            settings::view_section(&section.title)
                .add(settings::item(&desc[0], text("TODO")))
                .add(settings::item(&desc[1], text("TODO")))
                .add(settings::item(&desc[2], text("TODO")))
                .add(settings::item(&desc[3], text("TODO")))
                .add(settings::item(&desc[4], text("TODO")))
                .into()
        })
}

fn os() -> Section {
    Section::new()
        .title(fl!("about-os"))
        .descriptions(vec![
            fl!("about-os", "os"),
            fl!("about-os", "os-architecture"),
            fl!("about-os", "desktop-environment"),
            fl!("about-os", "windowing-system"),
        ])
        .view_fn(|_app, section| {
            let desc = &section.descriptions;
            settings::view_section(&section.title)
                .add(settings::item(&desc[0], text("TODO")))
                .add(settings::item(&desc[1], text("TODO")))
                .add(settings::item(&desc[2], text("TODO")))
                .add(settings::item(&desc[3], text("TODO")))
                .into()
        })
}

fn related() -> Section {
    Section::new()
        .title(fl!("about-related"))
        .descriptions(vec![fl!("about-related", "support")])
        .view_fn(|_app, section| {
            settings::view_section(&section.title)
                .add(settings::item(&section.descriptions[0], text("TODO")))
                .into()
        })
}
