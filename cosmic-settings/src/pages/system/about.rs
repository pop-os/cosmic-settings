// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic_settings_page::{self as page, section, Section};

use cosmic::widget::{list_column, settings, text};
use cosmic::{command, Command};
use cosmic_settings_system::about::Info;
use slotmap::SlotMap;

#[derive(Clone, Debug)]
pub enum Message {
    Info(Box<Info>),
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    info: Info,
    // support_page: page::Entity,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(device()),
            sections.insert(hardware()),
            sections.insert(os()),
            sections.insert(related()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("about", "help-about-symbolic")
            .title(fl!("about"))
            .description(fl!("about", "desc"))
    }

    fn reload(&mut self, _page: page::Entity) -> Command<crate::pages::Message> {
        command::future(async move {
            crate::pages::Message::About(Message::Info(Box::new(Info::load())))
        })
    }
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Info(info) => self.info = *info,
        }
    }
}

fn device() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![
            fl!("about-device").into(),
            fl!("about-device", "desc").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            let desc = &section.descriptions;
            let device_name = settings::item::builder(&*desc[0])
                .description(&*desc[1])
                .control(text(&page.info.device_name));

            list_column().add(device_name).into()
        })
}

fn hardware() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("about-hardware"))
        .descriptions(vec![
            fl!("about-hardware", "model").into(),
            fl!("about-hardware", "memory").into(),
            fl!("about-hardware", "processor").into(),
            fl!("about-hardware", "graphics").into(),
            fl!("about-hardware", "disk-capacity").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            let desc = &section.descriptions;

            let mut sections = settings::view_section(&section.title)
                .add(settings::item(&*desc[0], text(&page.info.hardware_model)))
                .add(settings::item(&*desc[1], text(&page.info.memory)))
                .add(settings::item(&*desc[2], text(&page.info.processor)));

            for card in &page.info.graphics {
                sections = sections.add(settings::item(&*desc[3], text(card.as_str())));
            }

            sections
                .add(settings::item(&*desc[4], text(&page.info.disk_capacity)))
                .into()
        })
}

fn os() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("about-os"))
        .descriptions(vec![
            fl!("about-os", "os").into(),
            fl!("about-os", "os-architecture").into(),
            fl!("about-os", "desktop-environment").into(),
            fl!("about-os", "windowing-system").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            let desc = &section.descriptions;
            settings::view_section(&section.title)
                .add(settings::item(&*desc[0], text(&page.info.operating_system)))
                .add(settings::item(&*desc[1], text(&page.info.os_architecture)))
                .add(settings::item(
                    &*desc[2],
                    text(&page.info.desktop_environment),
                ))
                .add(settings::item(&*desc[3], text(&page.info.windowing_system)))
                .into()
        })
}

fn related() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("about-related"))
        .descriptions(vec![fl!("about-related", "support").into()])
        .view::<Page>(|_binder, _page, section| {
            settings::view_section(&section.title)
                .add(settings::item(&*section.descriptions[0], text("TODO")))
                .into()
        })
}

// fn page(app: &crate::SettingsApp) -> &Page {
//     app.pages
//         .resource::<Page>()
//         .expect("missing system->about page")
// }
