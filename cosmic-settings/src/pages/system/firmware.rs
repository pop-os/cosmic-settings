// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(Section::default())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("firmware", "firmware-manager-symbolic")
            .title(fl!("firmware"))
            .description(fl!("firmware", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}
