// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod binder;
pub use binder::{AutoBind, Binder};

mod insert;
use cosmic::{Command, Element};
use downcast_rs::{impl_downcast, Downcast};
pub use insert::Insert;

pub mod section;
pub use section::Section;

use derive_setters::Setters;
use slotmap::SlotMap;
use std::borrow::Cow;

slotmap::new_key_type! {
    /// The unique ID of a page.
    pub struct Entity;
}

/// A collection of sections which a page may be comprised of.
pub type Content = Vec<section::Entity>;

pub trait Page<Message: 'static>: Downcast {
    /// Information about the page
    fn info(&self) -> Info;

    /// Initialize the sections used by this page.
    #[must_use]
    fn content(
        &self,
        _sections: &mut SlotMap<section::Entity, Section<Message>>,
    ) -> Option<Content> {
        None
    }

    /// Display a context drawer for the page.
    #[must_use]
    fn context_drawer(&self) -> Option<Element<'_, Message>> {
        None
    }

    /// Display an inner app dialog for the page.
    fn dialog(&self) -> Option<Element<'_, Message>> {
        None
    }

    /// Response from a file chooser dialog request.
    fn file_chooser(&mut self, _selected: Vec<url::Url>) -> Command<Message> {
        Command::none()
    }

    /// Alter the contents of the page's header view.
    fn header_view(&self) -> Option<Element<'_, Message>> {
        None
    }

    /// Reload page metadata via a Command.
    #[allow(unused)]
    fn on_enter(
        &mut self,
        page: crate::Entity,
        sender: tokio::sync::mpsc::Sender<Message>,
    ) -> Command<Message> {
        Command::none()
    }

    /// Emit a command when the page is left
    fn on_leave(&mut self) -> Command<Message> {
        Command::none()
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

#[macro_export]
macro_rules! update {
    ($binder:expr, $message:expr, $page:ty) => {{
        if let Some(page) = $binder.page_mut::<$page>() {
            page.update($message);
        }
    }};
}
