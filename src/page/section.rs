// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use derive_setters::Setters;
use regex::Regex;

use crate::SettingsApp;

slotmap::new_key_type! {
    /// ID of a page section
    pub struct Entity;
}

/// A searchable sub-component of a page. Searches can group multiple sections together.
#[derive(Setters)]
#[must_use]
pub struct Section {
    #[setters(into)]
    pub title: String,
    pub descriptions: Vec<String>,
    pub view_fn: for<'a> fn(&'a SettingsApp, &'a Section) -> cosmic::Element<'a, crate::Message>,
    #[setters(bool)]
    pub search_ignore: bool,
}

impl Section {
    pub const fn new() -> Self {
        Self {
            title: String::new(),
            descriptions: Vec::new(),
            view_fn: Self::unimplemented,
            search_ignore: false,
        }
    }

    #[must_use]
    pub fn matches_search(&self, rule: &Regex) -> bool {
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

    #[must_use]
    pub fn unimplemented<'a>(
        _app: &'a SettingsApp,
        _section: &'a Section,
    ) -> cosmic::Element<'a, crate::Message> {
        cosmic::widget::settings::view_column(vec![cosmic::widget::settings::view_section("")
            .add(crate::widget::unimplemented_page())
            .into()])
        .into()
    }
}
