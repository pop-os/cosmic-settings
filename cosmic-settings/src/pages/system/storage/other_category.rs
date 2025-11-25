// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::Subscription;
use cosmic::widget::settings;
use cosmic::{Apply, Task};
use cosmic_settings_page::{self as page, Section, section};
use slab::Slab;
use slotmap::SlotMap;
use std::time::Duration;

use super::utils::loading_or_size_item;

#[derive(Clone, Debug)]
pub enum Message {
    LoadData(u64),
    SetData { size: u64, loading: bool },
    AnimationTick,
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::StorageOtherCategory(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::StorageOtherCategory(message)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    entity: page::Entity,
    other_size: u64,
    loading: bool,
    animation_state: u8,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(other_details())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("storage-other", "folder-symbolic").title(fl!("storage-category-other"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        // Data is managed by parent storage page, just display what we have
        Task::none()
    }

    fn subscription(&self, _core: &cosmic::Core) -> Subscription<crate::pages::Message> {
        if self.loading {
            cosmic::iced::time::every(Duration::from_millis(500))
                .map(|_| crate::pages::Message::StorageOtherCategory(Message::AnimationTick))
        } else {
            Subscription::none()
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::app::Task<crate::Message> {
        match message {
            Message::LoadData(size) => {
                self.other_size = size;
                self.loading = false;
            }
            Message::SetData { size, loading } => {
                self.other_size = size;
                self.loading = loading;
            }
            Message::AnimationTick => {
                self.animation_state = (self.animation_state + 1) % 3;
            }
        }

        Task::none()
    }
}

fn other_details() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let total_label = descriptions.insert(fl!("storage-app-total"));

    Section::default()
        .title(fl!("storage-category-other"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let desc = &section.descriptions;
            let loading = page.loading;
            let animation_state = page.animation_state;

            settings::section()
                .title(&section.title)
                .add(loading_or_size_item(
                    &desc[total_label],
                    page.other_size,
                    loading,
                    animation_state,
                ))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::StorageOtherCategory)
        })
}
