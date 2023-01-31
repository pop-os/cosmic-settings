// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::{
    page::{self, desktop::wallpaper::settings, Content, Section},
    SettingsApp,
};
use cosmic::{
    iced::{
        widget::{horizontal_space, row},
        Length,
    },
    widget::{icon, list_column, settings, text},
};
use cosmic_settings_system::Info;
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

impl Model {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Info(info) => self.info = info,
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

    fn load(_page: page::Entity) -> crate::page::PageTask {
        Box::pin(async move { crate::Message::About(Message::Info(Info::load())) })
    }
}

fn device() -> Section {
    Section::new()
        .descriptions(vec![fl!("about-device"), fl!("about-device", "desc")])
        .view_fn(|app, section| {
            let desc = &section.descriptions;
            let model = model(app);

            let device_name = settings::item::builder(&desc[0])
                .description(&desc[1])
                .control(text(&model.info.device_name));

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
        .view_fn(|app, section| {
            let desc = &section.descriptions;
            let model = model(app);

            let mut sections = settings::view_section(&section.title)
                .add(settings::item(&desc[0], text(&model.info.hardware_model)))
                .add(settings::item(&desc[1], text(&model.info.memory)))
                .add(settings::item(&desc[2], text(&model.info.processor)));

            for card in &model.info.graphics {
                sections = sections.add(settings::item(&desc[3], text(card.as_str())));
            }

            // .add(settings::item(&desc[3], text(&model.info.graphics)))
            sections
                .add(settings::item(&desc[4], text(&model.info.disk_capacity)))
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
            let model = model(app);

            let desc = &section.descriptions;
            settings::view_section(&section.title)
                .add(settings::item(&desc[0], text(&model.info.operating_system)))
                .add(settings::item(&desc[1], text(&model.info.os_architecture)))
                .add(settings::item(
                    &desc[2],
                    text(&model.info.desktop_environment),
                ))
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

fn model(app: &SettingsApp) -> &Model {
    app.pages
        .resource::<Model>()
        .expect("missing system->about model")
}
