// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod binder;
pub use binder::{AutoBind, Binder};

mod insert;
use downcast_rs::{impl_downcast, Downcast};
pub use insert::Insert;

pub mod section;
pub use section::Section;

use derive_setters::Setters;
use slotmap::SlotMap;
use std::{borrow::Cow, future::Future, pin::Pin};

slotmap::new_key_type! {
    /// The unique ID of a page.
    pub struct Entity;
}

/// A collection of sections which a page may be comprised of.
pub type Content = Vec<section::Entity>;

/// A request by a page to run a command in the background.
pub type Task<Message> = Pin<Box<dyn Future<Output = Message> + Send>>;

pub trait Page<Message: 'static>: Downcast {
    /// Information about the page
    fn info(&self) -> Info;

    #[must_use]
    fn content(
        &self,
        _sections: &mut SlotMap<section::Entity, Section<Message>>,
    ) -> Option<Content> {
        None
    }

    #[must_use]
    #[allow(unused)]
    fn load(&self, page: crate::Entity) -> Option<crate::Task<Message>> {
        None
    }
}

impl_downcast!(Page<Message>);

/// Information about a page; including its title, icon, and description.
#[derive(Setters)]
#[must_use]
pub struct Info {
    /// An identifier that is the same between application runs.
    #[setters(skip)]
    pub id: Cow<'static, str>,

    /// The icon associated with the page.
    #[setters(skip)]
    pub icon_name: Cow<'static, str>,

    /// The title of the page.
    #[setters(into)]
    pub title: String,

    /// A description of the page.
    #[setters(into)]
    pub description: String,

    /// The parent of the page.
    #[setters(strip_option)]
    pub parent: Option<Entity>,
}

impl Info {
    pub fn new(id: impl Into<Cow<'static, str>>, icon_name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            title: String::new(),
            icon_name: icon_name.into(),
            id: id.into(),
            description: String::new(),
            parent: None,
        }
    }
}
