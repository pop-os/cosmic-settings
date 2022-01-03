// SPDX-License-Identifier: GPL-3.0-only

use crate::{
	sections::{Section, SectionLayout, SettingsGroup},
	ui::SettingsGui,
	widgets::ListBoxSelectionRow,
};
use gtk4::{
	glib::{self, clone},
	prelude::*,
	Align, Image, Label, ListBox, Orientation,
};
use std::rc::Rc;

pub fn setup<S: Section>(ui: Rc<SettingsGui>) {
	// Set up the nav entry
	let icon = Image::from_icon_name(Some(S::ICON));
	let label = Label::new(Some(S::NAME));
	let entry_box = gtk4::Box::builder()
		.orientation(gtk4::Orientation::Horizontal)
		.spacing(8)
		.margin_start(10)
		.margin_top(10)
		.margin_end(10)
		.margin_bottom(10)
		.build();
	entry_box.append(&icon);
	entry_box.append(&label);
	let row = cascade! {
		ListBoxSelectionRow::new(S::NAME.into());
		..add_css_class("nav-element");
		..set_margin_top(8);
		..set_margin_bottom(8);
		..set_margin_start(8);
		..set_margin_end(8);
		..set_child(Some(&entry_box));
	};
	ui.nav.list.append(&row);

	let entries = S::layout();
	match entries {
		SectionLayout::Single(groups) => {
			// Alright, now we setup the actual settings panel
			let panel = gtk4::Box::builder()
				.orientation(Orientation::Vertical)
				.spacing(24)
				.hexpand(true)
				.build();
			setup_single(&panel, ui.clone(), groups);
			ui.content.add_titled(&panel, Some(S::NAME), S::NAME);
		}
		SectionLayout::Multiple(subsections) => {
			setup_multi(S::NAME, ui, subsections);
			row.set_subsection(true);
		}
	}
}

fn setup_single(panel: &gtk4::Box, ui: Rc<SettingsGui>, groups: Vec<Box<dyn SettingsGroup>>) {
	for group in groups {
		let title = group.title();
		let group_box = gtk4::Box::builder()
			.orientation(Orientation::Vertical)
			.spacing(8)
			.build();
		let group_title = Label::builder()
			.label(title)
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
		group.layout(&group_box_inner, ui.clone());
		panel.append(&group_box);
	}
}

fn setup_multi(
	name: &str,
	ui: Rc<SettingsGui>,
	sections: Vec<(&'static str, Vec<Box<dyn SettingsGroup>>)>,
) {
	let nav = ListBox::builder()
		.margin_top(20)
		.margin_bottom(20)
		.margin_start(12)
		.margin_end(12)
		.css_classes(vec!["nav-subsection".into()])
		.build();
	for (name, groups) in sections {
		// Set up the subsection in the nav panel
		let label = Label::builder()
			.label(name)
			.margin_top(8)
			.margin_bottom(8)
			.margin_start(8)
			.margin_end(8)
			.build();
		let row = cascade! {
			ListBoxSelectionRow::new(name.into());
			..add_css_class("nav-element");
			..set_margin_top(8);
			..set_margin_bottom(8);
			..set_margin_start(8);
			..set_margin_end(8);
			..set_child(Some(&label));
		};
		nav.append(&row);
		// Set up the actual groups
		let panel = gtk4::Box::builder()
			.orientation(Orientation::Vertical)
			.spacing(24)
			.hexpand(true)
			.build();
		setup_single(&panel, ui.clone(), groups);
		ui.content.add_named(&panel, Some(name));
	}
	let main_stack = &ui.content;
	nav.connect_row_activated(clone!(@weak main_stack, => move |_, row| {
		let row = row
			.downcast_ref::<ListBoxSelectionRow>()
			.expect("invalid object");
		main_stack.set_visible_child_name(&row.row_id());
	}));
	ui.nav.stack.add_named(&nav, Some(name));
}
