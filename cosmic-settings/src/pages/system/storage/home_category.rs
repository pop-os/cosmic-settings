// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::Subscription;
use cosmic::widget::settings;
use cosmic::{Apply, Task};
use cosmic_settings_page::{self as page, Section, section};
use slab::Slab;
use slotmap::SlotMap;
use std::time::Duration;

use super::{HomeCategory, utils::loading_or_size_item};

#[derive(Clone, Debug)]
pub enum Message {
    LoadData(HomeCategory),
    SetData { data: HomeCategory, loading: bool },
    FieldUpdate(crate::pages::system::storage::HomeFieldUpdate),
    AnimationTick,
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::StorageHomeCategory(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::StorageHomeCategory(message)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    entity: page::Entity,
    home_category: HomeCategory,
    loading: bool,
    animation_state: u8,
    // Track which individual fields are still loading (false = loading, true = loaded)
    fields_loaded: FieldsLoaded,
}

#[derive(Clone, Debug)]
struct FieldsLoaded {
    documents: bool,
    downloads: bool,
    pictures: bool,
    videos: bool,
    music: bool,
    desktop: bool,
    other: bool,
}

impl Default for FieldsLoaded {
    fn default() -> Self {
        Self {
            documents: false,
            downloads: false,
            pictures: false,
            videos: false,
            music: false,
            desktop: false,
            other: false,
        }
    }
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
        Some(vec![sections.insert(home_details())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("storage-home", "user-home-symbolic").title(fl!("storage-category-home"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        // Data is managed by parent storage page, just display what we have
        Task::none()
    }

    fn subscription(&self, _core: &cosmic::Core) -> Subscription<crate::pages::Message> {
        if self.loading {
            cosmic::iced::time::every(Duration::from_millis(500))
                .map(|_| crate::pages::Message::StorageHomeCategory(Message::AnimationTick))
        } else {
            Subscription::none()
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::app::Task<crate::Message> {
        match message {
            Message::LoadData(data) => {
                self.home_category = data;
                self.loading = false;
                // Mark all fields as loaded
                self.fields_loaded = FieldsLoaded {
                    documents: true,
                    downloads: true,
                    pictures: true,
                    videos: true,
                    music: true,
                    desktop: true,
                    other: true,
                };
            }
            Message::SetData { data, loading } => {
                self.home_category = data;
                self.loading = loading;
                // If not loading anymore, mark all as loaded
                if !loading {
                    self.fields_loaded = FieldsLoaded {
                        documents: true,
                        downloads: true,
                        pictures: true,
                        videos: true,
                        music: true,
                        desktop: true,
                        other: true,
                    };
                }
            }
            Message::FieldUpdate(field_update) => {
                use crate::pages::system::storage::HomeFieldUpdate;
                // Update the specific field and mark it as loaded
                match field_update {
                    HomeFieldUpdate::Documents(size) => {
                        self.home_category.documents = size;
                        self.fields_loaded.documents = true;
                    }
                    HomeFieldUpdate::Downloads(size) => {
                        self.home_category.downloads = size;
                        self.fields_loaded.downloads = true;
                    }
                    HomeFieldUpdate::Pictures(size) => {
                        self.home_category.pictures = size;
                        self.fields_loaded.pictures = true;
                    }
                    HomeFieldUpdate::Videos(size) => {
                        self.home_category.videos = size;
                        self.fields_loaded.videos = true;
                    }
                    HomeFieldUpdate::Music(size) => {
                        self.home_category.music = size;
                        self.fields_loaded.music = true;
                    }
                    HomeFieldUpdate::Desktop(size) => {
                        self.home_category.desktop = size;
                        self.fields_loaded.desktop = true;
                    }
                    HomeFieldUpdate::Other(size) => {
                        self.home_category.other = size;
                        self.fields_loaded.other = true;
                    }
                }

                // Check if all fields are loaded
                if self.fields_loaded.documents
                    && self.fields_loaded.downloads
                    && self.fields_loaded.pictures
                    && self.fields_loaded.videos
                    && self.fields_loaded.music
                    && self.fields_loaded.desktop
                    && self.fields_loaded.other
                {
                    self.loading = false;
                }
            }
            Message::AnimationTick => {
                self.animation_state = (self.animation_state + 1) % 3;
            }
        }

        Task::none()
    }
}

fn home_details() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let documents_label = descriptions.insert(fl!("storage-home-documents"));
    let downloads_label = descriptions.insert(fl!("storage-home-downloads"));
    let pictures_label = descriptions.insert(fl!("storage-home-pictures"));
    let videos_label = descriptions.insert(fl!("storage-home-videos"));
    let music_label = descriptions.insert(fl!("storage-home-music"));
    let desktop_label = descriptions.insert(fl!("storage-home-desktop"));
    let other_label = descriptions.insert(fl!("storage-home-other"));
    let total_label = descriptions.insert(fl!("storage-app-total"));

    Section::default()
        .title(fl!("storage-category-home"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let desc = &section.descriptions;
            let home = &page.home_category;
            let fields_loaded = &page.fields_loaded;
            let animation_state = page.animation_state;

            settings::section()
                .title(&section.title)
                .add(loading_or_size_item(
                    &desc[documents_label],
                    home.documents,
                    !fields_loaded.documents,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[downloads_label],
                    home.downloads,
                    !fields_loaded.downloads,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[pictures_label],
                    home.pictures,
                    !fields_loaded.pictures,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[videos_label],
                    home.videos,
                    !fields_loaded.videos,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[music_label],
                    home.music,
                    !fields_loaded.music,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[desktop_label],
                    home.desktop,
                    !fields_loaded.desktop,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[other_label],
                    home.other,
                    !fields_loaded.other,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[total_label],
                    home.total_size(),
                    page.loading,
                    animation_state,
                ))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::StorageHomeCategory)
        })
}
