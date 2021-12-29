#[macro_use]
extern crate cascade;

mod sections;
mod widgets;

use crate::sections::{Section, SectionLayout, SettingsGroup};
use gtk4::{
	gdk::Display,
	glib::{self, prelude::*},
	prelude::*,
	CssProvider, Orientation, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

fn main() {
	let application =
		gtk4::Application::new(Some("com.system76.cosmic.settings"), Default::default());
	application.connect_activate(build_ui);
	application.run();
}

fn setup_single_section(panel: &gtk4::Box, groups: Vec<Box<dyn SettingsGroup>>) {
	for group in groups {
		let title = group.title();
		let group_box = gtk4::Box::builder()
			.orientation(Orientation::Vertical)
			.spacing(8)
			.build();
		let group_title = gtk4::Label::builder()
			.label(title)
			.css_classes(vec!["settings-group-title".into()])
			.halign(gtk4::Align::Start)
			.build();
		let group_box_inner = gtk4::Box::builder()
			.orientation(gtk4::Orientation::Vertical)
			.spacing(16)
			.css_classes(vec!["settings-group".into()])
			.build();
		group_box.append(&group_title);
		group_box.append(&group_box_inner);
		group.layout(&group_box_inner);
		panel.append(&group_box);
	}
}

fn setup_multi_section(
	name: &str,
	main_stack: &gtk4::Stack,
	subsection_stack: &gtk4::Stack,
	sections: Vec<(&'static str, Vec<Box<dyn SettingsGroup>>)>,
) {
	let nav = gtk4::ListBox::builder()
		.margin_top(20)
		.margin_bottom(20)
		.margin_start(12)
		.margin_end(12)
		.css_classes(vec!["nav-subsection".into()])
		.build();
	for (name, groups) in sections {
		// Set up the subsection in the nav panel
		let label = gtk4::Label::builder()
			.label(name)
			.margin_top(8)
			.margin_bottom(8)
			.margin_start(8)
			.margin_end(8)
			.build();
		let row = cascade! {
			widgets::ListBoxSelectionRow::new(name.into());
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
			.orientation(gtk4::Orientation::Vertical)
			.spacing(24)
			.hexpand(true)
			.build();
		setup_single_section(&panel, groups);
		main_stack.add_named(&panel, Some(name));
	}
	nav.connect_row_activated(glib::clone!(@weak main_stack, => move |_, row| {
		let row = row
			.downcast_ref::<widgets::ListBoxSelectionRow>()
			.expect("invalid object");
		main_stack.set_visible_child_name(&row.row_id());
	}));
	subsection_stack.add_named(&nav, Some(name));
}

fn setup_section<S: Section>(
	nav: &gtk4::ListBox,
	main_stack: &gtk4::Stack,
	subsection_stack: &gtk4::Stack,
) {
	// Set up the nav entry
	let icon = gtk4::Image::from_icon_name(Some(S::ICON));
	let label = gtk4::Label::new(Some(S::NAME));
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
		widgets::ListBoxSelectionRow::new(S::NAME.into());
		..add_css_class("nav-element");
		..set_margin_top(8);
		..set_margin_bottom(8);
		..set_margin_start(8);
		..set_margin_end(8);
		..set_child(Some(&entry_box));
	};
	nav.append(&row);

	let entries = S::layout();
	match entries {
		SectionLayout::Single(groups) => {
			// Alright, now we setup the actual settings panel
			let panel = gtk4::Box::builder()
				.orientation(gtk4::Orientation::Vertical)
				.spacing(24)
				.hexpand(true)
				.build();
			setup_single_section(&panel, groups);
			main_stack.add_titled(&panel, Some(S::NAME), S::NAME);
		}
		SectionLayout::Multiple(subsections) => {
			setup_multi_section(S::NAME, &main_stack, subsection_stack, subsections);
			row.set_subsection(true);
		}
	}
}

fn build_ui(application: &gtk4::Application) {
	let provider = cascade! {
		CssProvider::new();
		..load_from_data(include_bytes!(concat!(env!("OUT_DIR"), "/style.css")));
	};
	// We give the CssProvided to the default screen so the CSS rules we added
	// can be applied to our window.
	StyleContext::add_provider_for_display(
		&Display::default().expect("Could not connect to a display."),
		&provider,
		STYLE_PROVIDER_PRIORITY_APPLICATION,
	);

	let window = gtk4::ApplicationWindow::builder()
		.application(application)
		.title("Settings")
		.default_width(842)
		.default_height(632)
		.build();

	let base_box = gtk4::Box::builder()
		.orientation(gtk4::Orientation::Horizontal)
		.margin_top(16)
		.margin_bottom(0)
		.margin_start(16)
		.margin_end(32)
		.build();

	let settings_stack = gtk4::Stack::new();

	let header = gtk4::HeaderBar::builder().css_name("title").build();

	let search_entry = gtk4::SearchEntry::builder()
		.valign(gtk4::Align::Center)
		.hexpand(true)
		.build();
	let search_bar = gtk4::SearchBar::builder()
		.valign(gtk4::Align::Center)
		.key_capture_widget(&window)
		.child(&search_entry)
		.build();
	let search_button = gtk4::ToggleButton::builder()
		.icon_name("system-search-symbolic")
		.valign(gtk4::Align::Center)
		.css_classes(vec!["search-button".into()])
		.build();
	search_bar.connect_entry(&search_entry);
	search_entry.connect_search_started(
		glib::clone!(@weak settings_stack, @weak search_button => move |_| {
			search_button.set_active(true);
			settings_stack.set_visible_child_name("_search");
		}),
	);
	search_entry.connect_stop_search(glib::clone!(@weak search_button => move |_| {
		search_button.set_active(false);
	}));

	let nav_revealer = gtk4::Revealer::builder()
		.transition_type(gtk4::RevealerTransitionType::SlideRight)
		.build();

	let nav_button_box = gtk4::Box::builder()
		.orientation(gtk4::Orientation::Horizontal)
		.valign(gtk4::Align::Center)
		.spacing(8)
		.margin_start(10)
		.margin_end(10)
		.margin_top(10)
		.margin_bottom(10)
		.build();
	let nav_button_label = gtk4::Label::new(Some("Navigation"));
	nav_button_box.append(&nav_button_label);
	let nav_button_sep = gtk4::Separator::new(Orientation::Vertical);
	nav_button_box.append(&nav_button_sep);
	let nav_button_icon = gtk4::Image::from_icon_name(Some("go-next-symbolic"));
	nav_button_box.append(&nav_button_icon);
	let nav_button = gtk4::Button::builder()
		.child(&nav_button_box)
		.margin_top(10)
		.css_classes(vec!["nav-button".into()])
		.build();
	nav_button.connect_clicked(
		glib::clone!(@weak nav_revealer, @weak nav_button_icon => move |_| {
			let active = nav_revealer.reveals_child();
			if active {
				nav_button_icon.set_icon_name(Some("go-next-symbolic"));
				nav_revealer.set_reveal_child(false);
			} else {
				nav_button_icon.set_icon_name(Some("go-previous-symbolic"));
				nav_revealer.set_reveal_child(true);
			}
		}),
	);
	header.pack_start(&nav_button);
	header.pack_start(&search_button);
	header.pack_start(&search_bar);
	window.set_titlebar(Some(&header));

	let nav_box = gtk4::Box::builder()
		.orientation(gtk4::Orientation::Horizontal)
		.margin_top(20)
		.margin_bottom(20)
		.margin_start(12)
		.margin_end(12)
		.css_classes(vec!["nav".into()])
		.build();
	let nav_stack = gtk4::Stack::builder()
		.css_classes(vec!["nav-subsection".into()])
		.build();
	let nav_stack_revealer = gtk4::Revealer::builder()
		.child(&nav_stack)
		.transition_type(gtk4::RevealerTransitionType::SlideRight)
		.build();
	let nav = gtk4::ListBox::builder().build();
	nav_box.append(&nav);
	nav_box.append(&nav_stack_revealer);
	nav_revealer.set_child(Some(&nav_box));
	setup_section::<sections::WifiSection>(&nav, &settings_stack, &nav_stack);
	setup_section::<sections::DesktopSection>(&nav, &settings_stack, &nav_stack);
	nav.connect_row_activated(
		glib::clone!(@weak settings_stack, @weak nav_stack, @weak nav_stack_revealer => move |_, row| {
			let row = row
				.downcast_ref::<widgets::ListBoxSelectionRow>()
				.expect("invalid object");
			if row.subsection() {
				nav_stack_revealer.set_reveal_child(true);
				nav_stack.set_visible_child_name(&row.row_id());
			} else {
				nav_stack_revealer.set_reveal_child(false);
				settings_stack.set_visible_child_name(&row.row_id());
			}
		}),
	);
	base_box.append(&nav_revealer);
	base_box.append(&settings_stack);

	window.set_child(Some(&base_box));

	window.show();
}
