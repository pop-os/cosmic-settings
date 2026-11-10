// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod system_updates;

use cosmic::Task;
use cosmic_settings_page as page;

#[derive(Clone, Debug)]
pub enum Message {
    SystemUpdates(system_updates::Message),
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Updates(message)
    }
}

impl From<Message> for crate::Message {
    fn from(message: Message) -> Self {
        crate::Message::PageMessage(crate::pages::Message::from(message))
    }
}

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
    system_updates: system_updates::Model,
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::SystemUpdates(message) => self.system_updates.update(message),
        }
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> cosmic_settings_page::Info {
        page::Info::new("os-updates", "software-update-available-symbolic")
            .title(fl!("system-and-app-updates"))
    }

    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<
            page::section::Entity,
            page::Section<crate::pages::Message>,
        >,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(system_updates::section())])
    }

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        self.system_updates.update_labels();
        cosmic::Task::batch([
            cosmic::Task::future(async { system_updates::load_clients().await }),
            cosmic::Task::future(async { system_updates::load_configs().await }),
        ])
    }
}
