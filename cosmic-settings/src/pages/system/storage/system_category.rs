// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::Subscription;
use cosmic::widget::settings;
use cosmic::{Apply, Task};
use cosmic_settings_page::{self as page, Section, section};
use slab::Slab;
use slotmap::SlotMap;
use std::time::Duration;

use super::{SystemCategory, utils::loading_or_size_item};

#[derive(Clone, Debug)]
pub enum Message {
    LoadData(SystemCategory),
    SetData { data: SystemCategory, loading: bool },
    FieldUpdate(crate::pages::system::storage::SystemFieldUpdate),
    AnimationTick,
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::StorageSystemCategory(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::StorageSystemCategory(message)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    entity: page::Entity,
    system_category: SystemCategory,
    loading: bool,
    animation_state: u8,
    fields_loaded: FieldsLoaded,
}

#[derive(Clone, Debug)]
struct FieldsLoaded {
    system_files: bool,
    package_cache: bool,
    system_logs: bool,
    system_cache: bool,
    boot_files: bool,
    flatpak_runtimes: bool,
}

impl Default for FieldsLoaded {
    fn default() -> Self {
        Self {
            system_files: false,
            package_cache: false,
            system_logs: false,
            system_cache: false,
            boot_files: false,
            flatpak_runtimes: false,
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
        Some(vec![sections.insert(system_details())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("storage-system", "folder-symbolic").title(fl!("storage-category-system"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        // Data is managed by parent storage page, just display what we have
        Task::none()
    }

    fn subscription(&self, _core: &cosmic::Core) -> Subscription<crate::pages::Message> {
        if self.loading {
            cosmic::iced::time::every(Duration::from_millis(500))
                .map(|_| crate::pages::Message::StorageSystemCategory(Message::AnimationTick))
        } else {
            Subscription::none()
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::app::Task<crate::Message> {
        match message {
            Message::LoadData(data) => {
                self.system_category = data;
                self.loading = false;
                self.fields_loaded = FieldsLoaded {
                    system_files: true,
                    package_cache: true,
                    system_logs: true,
                    system_cache: true,
                    boot_files: true,
                    flatpak_runtimes: true,
                };
            }
            Message::SetData { data, loading } => {
                let old_data = self.system_category.clone();
                self.system_category = data;
                self.loading = loading;

                if self.system_category.system_files > 0 && old_data.system_files == 0 {
                    self.fields_loaded.system_files = true;
                }
                if self.system_category.package_cache > 0 && old_data.package_cache == 0 {
                    self.fields_loaded.package_cache = true;
                }
                if self.system_category.system_logs > 0 && old_data.system_logs == 0 {
                    self.fields_loaded.system_logs = true;
                }
                if self.system_category.system_cache > 0 && old_data.system_cache == 0 {
                    self.fields_loaded.system_cache = true;
                }
                if self.system_category.boot_files > 0 && old_data.boot_files == 0 {
                    self.fields_loaded.boot_files = true;
                }
                if self.system_category.flatpak_runtimes > 0 && old_data.flatpak_runtimes == 0 {
                    self.fields_loaded.flatpak_runtimes = true;
                }

                if !loading {
                    self.fields_loaded = FieldsLoaded {
                        system_files: true,
                        package_cache: true,
                        system_logs: true,
                        system_cache: true,
                        boot_files: true,
                        flatpak_runtimes: true,
                    };
                }
            }
            Message::FieldUpdate(field_update) => {
                use crate::pages::system::storage::SystemFieldUpdate;
                match field_update {
                    SystemFieldUpdate::SystemFiles(size) => {
                        self.system_category.system_files = size;
                        self.fields_loaded.system_files = true;
                    }
                    SystemFieldUpdate::PackageCache(size) => {
                        self.system_category.package_cache = size;
                        self.fields_loaded.package_cache = true;
                    }
                    SystemFieldUpdate::SystemLogs(size) => {
                        self.system_category.system_logs = size;
                        self.fields_loaded.system_logs = true;
                    }
                    SystemFieldUpdate::SystemCache(size) => {
                        self.system_category.system_cache = size;
                        self.fields_loaded.system_cache = true;
                    }
                    SystemFieldUpdate::BootFiles(size) => {
                        self.system_category.boot_files = size;
                        self.fields_loaded.boot_files = true;
                    }
                    SystemFieldUpdate::FlatpakRuntimes(size) => {
                        self.system_category.flatpak_runtimes = size;
                        self.fields_loaded.flatpak_runtimes = true;
                    }
                }

                if self.fields_loaded.system_files
                    && self.fields_loaded.package_cache
                    && self.fields_loaded.system_logs
                    && self.fields_loaded.system_cache
                    && self.fields_loaded.boot_files
                    && self.fields_loaded.flatpak_runtimes
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

fn system_details() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let system_files_label = descriptions.insert(fl!("storage-system-files"));
    let package_cache_label = descriptions.insert(fl!("storage-system-package-cache"));
    let system_logs_label = descriptions.insert(fl!("storage-system-logs"));
    let system_cache_label = descriptions.insert(fl!("storage-system-cache"));
    let boot_files_label = descriptions.insert(fl!("storage-system-boot"));
    let flatpak_runtimes_label = descriptions.insert(fl!("storage-system-flatpak-runtimes"));
    let total_label = descriptions.insert(fl!("storage-app-total"));

    Section::default()
        .title(fl!("storage-category-system"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let desc = &section.descriptions;
            let sys = &page.system_category;
            let fields_loaded = &page.fields_loaded;
            let animation_state = page.animation_state;

            settings::section()
                .title(&section.title)
                .add(loading_or_size_item(
                    &desc[system_files_label],
                    sys.system_files,
                    !fields_loaded.system_files,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[package_cache_label],
                    sys.package_cache,
                    !fields_loaded.package_cache,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[system_logs_label],
                    sys.system_logs,
                    !fields_loaded.system_logs,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[system_cache_label],
                    sys.system_cache,
                    !fields_loaded.system_cache,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[boot_files_label],
                    sys.boot_files,
                    !fields_loaded.boot_files,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[flatpak_runtimes_label],
                    sys.flatpak_runtimes,
                    !fields_loaded.flatpak_runtimes,
                    animation_state,
                ))
                .add(loading_or_size_item(
                    &desc[total_label],
                    sys.total_size(),
                    page.loading,
                    animation_state,
                ))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::StorageSystemCategory)
        })
}
