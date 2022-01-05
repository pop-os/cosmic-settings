// SPDX-License-Identifier: GPL-3.0-only

use super::SettingsGui;
use crate::{
	sections::SettingsGroupStore,
	widgets::{ListBoxKeywordedRow, SearchBar},
};
use fuzzy_matcher::skim::SkimMatcherV2;
use gtk4::{
	glib::{self, clone},
	prelude::*,
	ListBox,
};
use std::{cell::RefCell, rc::Rc};

/// A struct containing references to search-related elements
#[derive(Clone)]
pub struct SearchGui {
	/// The search bar of the application
	pub bar: SearchBar,
	/// The ListBox containing all the search results
	pub all_results: ListBox,
	/// The name of the content child that was present *before* searching
	pub before_search_child: RefCell<Option<String>>,
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
		let matcher = Rc::new(SkimMatcherV2::default());
		self.all_results.set_filter_func(
			clone!(@strong self.bar as bar, @strong matcher => move |row| {
				let text = bar.text();
				let row = row
					.downcast_ref::<ListBoxKeywordedRow>()
					.expect("invalid object");
				row.matches(matcher.clone(), text.as_str())
			}),
		);
		self.bar.connect_changed(clone!(@weak ui => move |_| {
			ui.search.all_results.invalidate_filter();
			let text = ui.search.bar.text();
			if !text.as_str().trim().is_empty() {
				if ui.search.before_search_child.borrow().is_none() {
					*ui.search.before_search_child.borrow_mut() = ui.content.visible_child_name().map(|name| name.as_str().to_string());
				}
				ui.content.set_visible_child_name("_search");
			} else if let Some(name) = ui.search.before_search_child.borrow_mut().take() {
				ui.content.set_visible_child_name(name.as_str());
			}
		}));
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
		let before_search_child = RefCell::new(None);
		Self {
			bar,
			all_results,
			before_search_child,
		}
	}
}
