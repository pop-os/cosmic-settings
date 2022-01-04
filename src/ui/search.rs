// SPDX-License-Identifier: GPL-3.0-only

use super::SettingsGui;
use crate::{
	sections::SettingsGroupStore,
	widgets::{ListBoxKeywordedRow, SearchBar},
};
use gtk4::{
	glib::{self, clone},
	prelude::*,
	ListBox,
};
use std::rc::Rc;

/// A struct containing references to search-related elements
#[derive(Clone)]
pub struct SearchGui {
	/// The search bar of the application
	pub bar: SearchBar,
	/// The ListBox containing all the search results
	pub all_results: ListBox,
}

impl SearchGui {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn setup(&self, ui: Rc<SettingsGui>, sections_store: SettingsGroupStore) {
		let all_settings_groups = sections_store.borrow();
		for group in all_settings_groups.iter() {
			let keyworded_row = ListBoxKeywordedRow::new();
			keyworded_row.set_group(ui.clone(), &**group);
			self.all_results.append(&keyworded_row);
		}
		self.all_results
			.set_filter_func(clone!(@strong self.bar as bar => move |row| {
				let text = bar.text();
				let row = row
					.downcast_ref::<ListBoxKeywordedRow>()
					.expect("invalid object");
				row.matches(text.as_str())
			}));
		self.bar.connect_changed(
			clone!(@weak self.bar as bar, @weak self.all_results as all_results, @weak ui.content as content => move |_| {
				all_results.invalidate_filter();
				let text = bar.text();
				if !text.as_str().is_empty() {
					content.set_visible_child_name("_search");
				}
			}),
		);
	}

	fn create_all_results() -> ListBox {
		ListBox::new()
	}

	fn create_bar() -> SearchBar {
		cascade! {
			SearchBar::new();
			..set_margin_top(10);
		}
	}
}

impl Default for SearchGui {
	fn default() -> Self {
		let bar = Self::create_bar();
		let all_results = Self::create_all_results();
		Self { bar, all_results }
	}
}
