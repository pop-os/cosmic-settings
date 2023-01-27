// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::page::{self, section, Content, Section};
use slotmap::SlotMap;

pub struct Page;

impl page::Page for Page {
    type Model = ();

    fn page() -> page::Meta {
        page::Meta::new("time-region", "preferences-desktop-locale-symbolic")
            .title(fl!("time-region"))
            .description(fl!("time-region", "desc"))
    }

    fn content(sections: &mut SlotMap<section::Entity, Section>) -> Option<Content> {
        Some(vec![sections.insert(Section::new())])
    }
}
