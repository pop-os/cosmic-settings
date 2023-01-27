// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use slotmap::SlotMap;

use crate::page::{self, section, Content, Section};

pub struct Page;

impl page::Page for Page {
    type Model = super::Model;

    fn page() -> page::Meta {
        page::Meta::new("dock", "preferences-pop-desktop-dock-symbolic")
            .title(fl!("dock"))
            .description(fl!("dock", "desc"))
    }

    fn content(sections: &mut SlotMap<section::Entity, Section>) -> Option<Content> {
        Some(vec![sections.insert(Section::new())])
    }
}
