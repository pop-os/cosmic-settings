// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::{AutoBind, Binder, Content, Entity};

/// An inserted page which may have additional properties assigned to it.
pub struct Insert<'a, Message> {
    pub model: &'a mut Binder<Message>,
    pub id: Entity,
}

impl<Message: 'static> Insert<'_, Message> {
    #[must_use]
    #[inline]
    pub fn id(self) -> Entity {
        self.id
    }

    #[must_use]
    #[inline]
    pub fn content(self, content: Content) -> Self {
        self.model.content.insert(self.id, content);
        self
    }

    /// Adds a page and associates it with its parent page.
    #[allow(clippy::return_self_not_must_use)]
    #[allow(clippy::must_use_candidate)]
    pub fn sub_page<P: AutoBind<Message>>(self) -> Self {
        let sub_page = self.model.register::<P>().id();
        self.sub_page_inner(sub_page)
    }

    #[inline(never)]
    fn sub_page_inner(self, sub_page: Entity) -> Self {
        self.model.info[sub_page].parent = Some(self.id);

        self.model
            .sub_pages
            .entry(self.id)
            .expect("parent page missing")
            .and_modify(|v| v.push(sub_page))
            .or_insert_with(|| vec![sub_page]);

        self
    }

    #[allow(clippy::return_self_not_must_use)]
    #[allow(clippy::must_use_candidate)]
    pub fn sub_page_with_id<P: AutoBind<Message>>(&mut self) -> Entity {
        let sub_page = self.model.register::<P>().id();
        self.sub_page_with_id_inner(sub_page)
    }

    #[inline(never)]
    fn sub_page_with_id_inner(&mut self, sub_page: Entity) -> Entity {
        self.model.info[sub_page].parent = Some(self.id);

        self.model
            .sub_pages
            .entry(self.id)
            .expect("parent page missing")
            .and_modify(|v| v.push(sub_page))
            .or_insert_with(|| vec![sub_page]);

        sub_page
    }
}
