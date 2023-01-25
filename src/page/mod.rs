// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod desktop;
pub mod networking;
pub mod section;
pub mod time;
pub use section::Section;
pub mod sound;
pub mod system;

mod model;

pub use model::{Insert, Model};

use derive_setters::Setters;
use slotmap::SlotMap;

slotmap::new_key_type! {
    /// ID of a page
    pub struct Entity;
}

pub trait Page {
    type Model: Default + 'static;

    /// A unique identity that is the same between application runs.
    const PERSISTENT_ID: &'static str;

    fn page() -> Meta;

    #[must_use]
    fn content(_sections: &mut SlotMap<section::Entity, Section>) -> Option<Content> {
        None
    }

    /// Attaches sub-pages to the page.
    #[allow(clippy::must_use_candidate)]
    fn sub_pages(page: Insert) -> Insert {
        page
    }
}

#[derive(Default, Setters)]
#[must_use]
pub struct Meta {
    #[setters(into)]
    pub title: String,
    #[setters(into)]
    pub icon_name: &'static str,
    #[setters(into)]
    pub description: String,
    #[setters(strip_option)]
    pub parent: Option<Entity>,
}

pub type Content = Vec<section::Entity>;
