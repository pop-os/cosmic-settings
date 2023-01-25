// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use slotmap::SlotMap;

use crate::page::{self, section, Content, Section};

pub struct Page;

impl page::Page for Page {
    type Model = super::Model;

    const PERSISTENT_ID: &'static str = "dock";

    fn page() -> page::Meta {
        page::Meta::default()
            .title(fl!("dock"))
            .description(fl!("dock", "desc"))
            .icon_name("preferences-pop-desktop-dock-symbolic")
    }

    fn content(sections: &mut SlotMap<section::Entity, Section>) -> Option<Content> {
        Some(vec![sections.insert(Section::new())])
    }
}
