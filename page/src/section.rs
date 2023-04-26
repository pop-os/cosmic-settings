// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use derive_setters::Setters;
use regex::Regex;

use crate::{Binder, Page};

slotmap::new_key_type! {
    /// The unique ID of a section of page.
    pub struct Entity;
}

pub type ViewFn<Message> = Box<
    dyn for<'a> Fn(
        &'a Binder<Message>,
        &'a dyn Page<Message>,
        &'a Section<Message>,
    ) -> cosmic::Element<'a, Message>,
>;

/// A searchable sub-component of a page.
///
/// Searches can group multiple sections together.
#[derive(Setters)]
#[must_use]
pub struct Section<Message> {
    #[setters(into)]
    pub title: String,
    pub descriptions: Vec<String>,
    #[setters(skip)]
    pub view_fn: ViewFn<Message>,
    #[setters(bool)]
    pub search_ignore: bool,
}

impl<Message: 'static> Default for Section<Message> {
    fn default() -> Self {
        Self {
            title: String::new(),
            descriptions: Vec::new(),
            view_fn: Box::new(unimplemented),
            search_ignore: false,
        }
    }
}

impl<Message: 'static> Section<Message> {
    #[must_use]
    pub fn search_matches(&self, rule: &Regex) -> bool {
        if self.search_ignore {
            return false;
        }

        if rule.is_match(self.title.as_str()) {
            return true;
        }

        for description in &self.descriptions {
            if rule.is_match(description.as_str()) {
                return true;
            }
        }

        false
    }

    /// # Panics
    ///
    /// Will panic if the `Model` type does not match the page type.
    pub fn view<Model: Page<Message>>(
        mut self,
        func: impl for<'a> Fn(
                &'a Binder<Message>,
                &'a Model,
                &'a Section<Message>,
            ) -> cosmic::Element<'a, Message>
            + 'static,
    ) -> Self {
        self.view_fn = Box::new(move |binder, model: &dyn Page<Message>, section| {
            let model = model.downcast_ref::<Model>().unwrap_or_else(|| {
                panic!(
                    "page model type mismatch: expected {}",
                    std::any::type_name::<Model>()
                )
            });

            func(binder, model, section)
        });
        self
    }
}

#[must_use]
pub fn unimplemented<'a, Message: 'static>(
    _binder: &'a Binder<Message>,
    _page: &'a dyn Page<Message>,
    _section: &'a Section<Message>,
) -> cosmic::Element<'a, Message> {
    cosmic::widget::settings::view_column(vec![cosmic::widget::settings::view_section("").into()])
        .into()
}
