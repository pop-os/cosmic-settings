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

#[derive(Clone, Debug)]
pub enum Message {
    Info(Info),
}

#[derive(Clone, Debug, Default)]
pub struct Model {
    info: Info,
    support_page: page::Entity,
}

#[derive(Clone, Debug, Default)]
pub struct Info {
    hardware_model: String,
    memory: String,
    processor: String,
    graphics: String,
    disk_capacity: String,
    operating_system: String,
    os_architecture: String,
    desktop_environment: String,
    windowing_system: String,
}

impl Model {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Info(info) => self.info = dbg!(info),
        }
    }
}

pub struct Page;

impl page::Page for Page {
    type Model = Model;

    fn page() -> page::Meta {
        page::Meta::new("about", "help-about-symbolic")
            .title(fl!("about"))
            .description(fl!("about", "desc"))
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

    fn load(page: page::Entity) -> crate::Message {
        let info = Info {
            windowing_system: std::env::var("XDG_SESSION_TYPE")
                .ok()
                .unwrap_or_else(|| fl!("unknown")),
            ..Info::default()
        };

        crate::Message::About(Message::Info(info))
    }
}

fn device() -> Section {
    Section::new()
        .descriptions(vec![fl!("about-device"), fl!("about-device", "desc")])
        .view_fn(|_app, section| {
            let desc = &section.descriptions;

            let device_name = settings::item::builder(&desc[0])
                .description(&desc[1])
                .control(text("TODO"));

            list_column().add(device_name).into()
        })
}

fn distributor_logo() -> Section {
    Section::new().search_ignore().view_fn(|_app, _section| {
        row!(
            horizontal_space(Length::Fill),
            icon("distributor-logo", 78),
            horizontal_space(Length::Fill),
        )
        // Add extra padding to reach 40px from the first section.
        .padding([0, 16, 0, 16])
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
        .view_fn(|app, section| {
            let model = app
                .pages
                .resource::<Model>()
                .expect("missing system->about model");

            let desc = &section.descriptions;
            settings::view_section(&section.title)
                .add(settings::item(&desc[0], text("TODO")))
                .add(settings::item(&desc[1], text("TODO")))
                .add(settings::item(&desc[2], text("TODO")))
                .add(settings::item(&desc[3], text(&model.info.windowing_system)))
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
