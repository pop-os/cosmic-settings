// SPDX-License-Identifier: GPL-3.0-only

use crate::{sections::SettingsGroup, ui::SettingsGui};
use gtk4::{
	glib::{self, Object},
	prelude::*,
	subclass::prelude::*,
	Align, Label, Orientation,
};
use std::{cell::RefCell, rc::Rc};

glib::wrapper! {
	pub struct ListBoxKeywordedRow(ObjectSubclass<ListBoxKeywordedRowImp>)
		@extends gtk4::ListBoxRow, gtk4::Widget,
		@implements gtk4::Accessible, gtk4::Actionable;
}

impl ListBoxKeywordedRow {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn set_group(&self, ui: Rc<SettingsGui>, group: &dyn SettingsGroup) {
		let group_object = {
			let group_box = gtk4::Box::builder()
				.orientation(Orientation::Vertical)
				.spacing(8)
				.build();
			let group_title = Label::builder()
				.label(group.title())
				.css_classes(vec!["settings-group-title".into()])
				.halign(Align::Start)
				.build();
			let group_box_inner = gtk4::Box::builder()
				.orientation(gtk4::Orientation::Vertical)
				.spacing(16)
				.css_classes(vec!["settings-group".into()])
				.build();
			group_box.append(&group_title);
			group_box.append(&group_box_inner);
			group.layout(&group_box_inner, ui);
			group_box
		};
		self.set_child(Some(&group_object));
		*self.inner().keywords.borrow_mut() = group.keywords();
	}

	pub fn matches(&self, haystack: &str) -> bool {
		let haystack = haystack.trim().to_lowercase();
		let inner = self.inner();
		inner
			.keywords
			.borrow()
			.iter()
			.any(|keyword| haystack.contains(keyword))
	}

	fn inner(&self) -> &ListBoxKeywordedRowImp {
		ListBoxKeywordedRowImp::from_instance(self)
	}
}

impl Default for ListBoxKeywordedRow {
	fn default() -> Self {
		Object::new(&[]).expect("Failed to create `ListBoxKeywordedRow`.")
	}
}

#[derive(Debug, Default)]
pub struct ListBoxKeywordedRowImp {
	keywords: RefCell<&'static [&'static str]>,
}

#[glib::object_subclass]
impl ObjectSubclass for ListBoxKeywordedRowImp {
	const NAME: &'static str = "ListBoxKeywordedRow";
	type Type = ListBoxKeywordedRow;
	type ParentType = gtk4::ListBoxRow;
	type Interfaces = ();
	type Instance = glib::subclass::basic::InstanceStruct<Self>;
	type Class = glib::subclass::basic::ClassStruct<Self>;

	fn class_init(klass: &mut Self::Class) {
		// The layout manager determines how child widgets are laid out.
		klass.set_layout_manager_type::<gtk4::BinLayout>();
	}
}

impl ObjectImpl for ListBoxKeywordedRowImp {
	fn constructed(&self, obj: &Self::Type) {
		self.parent_constructed(obj);
	}
}

impl WidgetImpl for ListBoxKeywordedRowImp {}

impl gtk4::subclass::list_box_row::ListBoxRowImpl for ListBoxKeywordedRowImp {}
