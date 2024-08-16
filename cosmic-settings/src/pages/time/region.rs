// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(Section::default())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("time-region", "preferences-region-and-language-symbolic")
            .title(fl!("time-region"))
            .description(fl!("time-region", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}
