// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::{AutoBind, Binder, Content, Entity};

/// An inserted page which may have additional properties assigned to it.
pub struct Insert<'a, Message> {
    pub model: &'a mut Binder<Message>,
    pub id: Entity,
}

impl<'a, Message: 'static> Insert<'a, Message> {
    #[must_use]
    pub fn id(self) -> Entity {
        self.id
    }

    #[must_use]
    pub fn content(self, content: Content) -> Self {
        self.model.content.insert(self.id, content);
        self
    }

    /// Adds a page and associates it with its parent page.
    #[allow(clippy::return_self_not_must_use)]
    #[allow(clippy::must_use_candidate)]
    pub fn sub_page<P: AutoBind<Message>>(self) -> Self {
        let sub_page = self.model.register::<P>().id();

        self.model.info[sub_page].parent = Some(self.id);

        self.model
            .sub_pages
            .entry(self.id)
            .expect("parent page missing")
            .and_modify(|v| v.push(sub_page))
            .or_insert_with(|| vec![sub_page]);

        self
    }
}
