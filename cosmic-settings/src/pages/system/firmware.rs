// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::Task;
use cosmic::widget::{settings, text};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use cosmic_settings_system::about::Info;
use slotmap::SlotMap;

#[derive(Clone, Debug)]
pub enum Message {
    Info(Box<Info>),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Firmware(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Firmware(message)
    }
}

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
    info: Info,
    on_enter_handle: Option<cosmic::iced::task::Handle>,
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(firmware())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("firmware", "firmware-manager-symbolic")
            .title(fl!("firmware"))
            .description(fl!("firmware", "desc"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        let (task, handle) = Task::future(async move {
            crate::pages::Message::Firmware(Message::Info(Box::new(Info::load())))
        })
        .abortable();

        self.on_enter_handle = Some(handle);
        task
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }

        Task::none()
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::app::Task<crate::Message> {
        match message {
            Message::Info(info) => {
                self.info = *info;
            }
        }

        Task::none()
    }
}

fn firmware() -> Section<crate::pages::Message> {
    Section::default().view::<Page>(move |_binder, page, section| {
        settings::section()
            .title(&section.title)
            .add(settings::flex_item(
                fl!("firmware", "bios-version"),
                text::body(&page.info.bios_version),
            ))
            .into()
    })
}

impl page::AutoBind<crate::pages::Message> for Page {}
