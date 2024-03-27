// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::section::{self, Section};
use crate::{Content, Info, Page};
use cosmic::iced_runtime::command::Command;
use cosmic::Element;
use regex::Regex;
use slotmap::{SecondaryMap, SlotMap, SparseSecondaryMap};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

/// All settings pages are registered and managed by the [`Binder`].
pub struct Binder<Message> {
    pub info: SlotMap<crate::Entity, Info>,
    pub page: SecondaryMap<crate::Entity, Box<dyn Page<Message>>>,
    pub typed_page_ids: HashMap<TypeId, crate::Entity>,
    pub resource: HashMap<TypeId, Box<dyn Any>>,
    pub storage: HashMap<TypeId, SecondaryMap<crate::Entity, Box<dyn Any>>>,
    pub sub_pages: SparseSecondaryMap<crate::Entity, Vec<crate::Entity>>,
    pub sections: SlotMap<section::Entity, Section<Message>>,
    pub content: SparseSecondaryMap<crate::Entity, Content>,
}

impl<Message> Default for Binder<Message> {
    fn default() -> Self {
        Self {
            content: SparseSecondaryMap::new(),
            info: SlotMap::with_key(),
            page: SecondaryMap::new(),
            typed_page_ids: HashMap::new(),
            resource: HashMap::new(),
            sections: SlotMap::with_key(),
            storage: HashMap::new(),
            sub_pages: SparseSecondaryMap::new(),
        }
    }
}

impl<Message: 'static> Binder<Message> {
    /// Check if a page exists in the model.
    #[must_use]
    pub fn contains_item(&self, id: crate::Entity) -> bool {
        self.info.contains_key(id)
    }

    /// Returns the content of a page, if it has any.
    #[must_use]
    pub fn content(&self, page: crate::Entity) -> Option<&[section::Entity]> {
        self.content.get(page).map(Vec::as_slice)
    }

    /// Get an immutable reference to data associated with a page.
    #[must_use]
    pub fn data<Data: 'static>(&self, id: crate::Entity) -> Option<&Data> {
        self.storage
            .get(&TypeId::of::<Data>())
            .and_then(|storage| storage.get(id))
            .and_then(|data| data.downcast_ref())
    }

    /// Get a mutable reference to data associated with a page.
    pub fn data_mut<Data: 'static>(&mut self, id: crate::Entity) -> Option<&mut Data> {
        self.storage
            .get_mut(&TypeId::of::<Data>())
            .and_then(|storage| storage.get_mut(id))
            .and_then(|data| data.downcast_mut())
    }

    /// Associates data with the item.
    pub fn data_set<Data: 'static>(&mut self, id: crate::Entity, data: Data) {
        if self.contains_item(id) {
            self.storage
                .entry(TypeId::of::<Data>())
                .or_insert_with(SecondaryMap::new)
                .insert(id, Box::new(data));
        }
    }

    /// Removes a specific data type from the item.
    pub fn data_remove<Data: 'static>(&mut self, id: crate::Entity) {
        self.storage
            .get_mut(&TypeId::of::<Data>())
            .and_then(|storage| storage.remove(id));
    }

    #[must_use]
    pub fn find_page_by_id(&self, id: &str) -> Option<(crate::Entity, &Info)> {
        self.info.iter().find(|(_id, info)| info.id == id)
    }

    /// Registers a new page in the settings panel.
    pub fn register<P: AutoBind<Message>>(&mut self) -> crate::Insert<Message> {
        let page = P::default();

        let id = self.register_page(page);

        self.typed_page_ids.insert(TypeId::of::<P>(), id);

        P::sub_pages(crate::Insert { id, model: self })
    }

    pub fn register_page<P: Page<Message>>(&mut self, page: P) -> crate::Entity {
        let id = self.info.insert(page.info());

        if let Some(content) = page.content(&mut self.sections) {
            self.content.insert(id, content);
        }

        self.page.insert(id, Box::new(page));

        id
    }

    #[must_use]
    pub fn model(&self, id: crate::Entity) -> Option<&dyn Page<Message>> {
        self.page.get(id).map(AsRef::as_ref)
    }

    #[must_use]
    pub fn model_mut(&mut self, id: crate::Entity) -> Option<&mut dyn Page<Message>> {
        self.page.get_mut(id).map(AsMut::as_mut)
    }

    /// Get entity ID of page by its type ID.
    pub fn page_id<P: Page<Message>>(&self) -> Option<crate::Entity> {
        self.typed_page_ids.get(&TypeId::of::<P>()).copied()
    }

    /// Obtain a reference to a page by its type ID.
    #[must_use]
    pub fn page<P: Page<Message>>(&self) -> Option<&P> {
        let page = self.page.get(self.page_id::<P>()?)?;
        page.downcast_ref::<P>()
    }

    /// Create a context drawer for the given page.
    #[must_use]
    pub fn context_drawer(&self, id: crate::Entity) -> Option<Element<'_, Message>> {
        let page = self.page.get(id)?;
        page.context_drawer()
    }

    /// Create a dialog for the given page.
    #[must_use]
    pub fn dialog(&self, id: crate::Entity) -> Option<Element<'_, Message>> {
        let page = self.page.get(id)?;
        page.dialog()
    }

    /// Obtain a reference to a page by its type ID.
    #[must_use]
    pub fn page_mut<P: Page<Message>>(&mut self) -> Option<&mut P> {
        let page = self.page.get_mut(self.page_id::<P>()?)?;
        page.downcast_mut::<P>()
    }

    /// Returns a command when a page is left
    pub fn on_leave(&mut self, id: crate::Entity) -> Option<Command<Message>> {
        if let Some(page) = self.page.get_mut(id) {
            return Some(page.on_leave());
        }
        None
    }

    /// Calls a page's load function to refresh its data.
    pub fn page_reload(&mut self, id: crate::Entity) -> Command<Message> {
        if let Some(page) = self.page.get_mut(id) {
            return page.reload(id);
        }

        Command::none()
    }

    #[must_use]
    pub fn resource<Resource: 'static>(&self) -> Option<&Resource> {
        self.resource
            .get(&TypeId::of::<Resource>())
            .and_then(|resource| resource.downcast_ref())
    }

    #[must_use]
    pub fn resource_mut<Resource: 'static>(&mut self) -> Option<&mut Resource> {
        self.resource
            .get_mut(&TypeId::of::<Resource>())
            .and_then(|resource| resource.downcast_mut())
    }

    #[allow(unused_must_use)]
    pub fn resource_register<Resource: Default + 'static>(&mut self) {
        self.resource
            .entry(TypeId::of::<Resource>())
            .or_insert_with(|| Box::<Resource>::default());
    }

    /// Finds content of panels that match the search.
    pub fn search<'a>(
        &'a self,
        rule: &'a Regex,
    ) -> impl Iterator<Item = (crate::Entity, section::Entity)> + 'a {
        generator::Gn::new_scoped_local(|mut s| {
            for (page, sections) in self.content.iter() {
                for id in sections {
                    if self.sections[*id].search_matches(rule) {
                        s.yield_((page, *id));
                    }
                }
            }

            generator::done!();
        })
    }

    /// Returns the sub-pages of a page, if it has any.
    pub fn sub_pages(&self, page: crate::Entity) -> Option<&[crate::Entity]> {
        self.sub_pages.get(page).map(AsRef::as_ref)
    }
}

pub trait AutoBind<Message: 'static>: Page<Message> + Default + 'static {
    /// Attaches sub-pages to the page.
    #[allow(clippy::must_use_candidate)]
    fn sub_pages(page: crate::Insert<Message>) -> crate::Insert<Message> {
        page
    }
}
