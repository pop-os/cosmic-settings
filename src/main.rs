#[macro_use]
extern crate cascade;

mod sections;
mod widgets;

use crate::sections::Section;
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

fn setup_section<S: Section>(nav: &gtk4::ListBox, stack: &gtk4::Stack) {
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

	// Alright, now we setup the actual settings panel
	let panel = gtk4::Box::builder()
		.orientation(gtk4::Orientation::Vertical)
		.spacing(24)
		.hexpand(true)
		.build();

	let entries = S::settings();
	for group in entries {
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

	stack.add_titled(&panel, Some(S::NAME), S::NAME);
}

fn build_ui(application: &gtk4::Application) {
	let provider = CssProvider::new();
	provider.load_from_data(include_bytes!(concat!(env!("OUT_DIR"), "/style.css")));
	// We give the CssProvided to the default screen so the CSS rules we added
	// can be applied to our window.
	StyleContext::add_provider_for_display(
		&Display::default().expect("Could not connect to a display."),
		&provider,
		STYLE_PROVIDER_PRIORITY_APPLICATION,
	);

	let window = gtk4::ApplicationWindow::new(application);

	window.set_title(Some("Settings"));
	window.set_default_size(842, 632);

	let base_box = gtk4::Box::builder()
		.orientation(gtk4::Orientation::Horizontal)
		.margin_top(16)
		.margin_bottom(0)
		.margin_start(16)
		.margin_end(32)
		.build();

	let header = gtk4::HeaderBar::builder().css_name("title").build();
	let nav_button_box = gtk4::Box::builder()
		.orientation(gtk4::Orientation::Horizontal)
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
		.css_name("nav-button")
		.child(&nav_button_box)
		.margin_top(10)
		.build();
	header.pack_start(&nav_button);
	window.set_titlebar(Some(&header));

	let settings_stack = gtk4::Stack::new();

	let nav = gtk4::ListBox::builder()
		.margin_top(20)
		.margin_bottom(20)
		.margin_start(12)
		.margin_end(12)
		.css_name("nav")
		.build();
	setup_section::<sections::WifiSection>(&nav, &settings_stack);
	setup_section::<sections::DesktopSection>(&nav, &settings_stack);
	nav.connect_row_activated(glib::clone!(@weak settings_stack => move |_, row| {
		let row = row
			.downcast_ref::<widgets::ListBoxSelectionRow>()
			.expect("invalid object");
		println!("{}", row.row_id());
		settings_stack.set_visible_child_name(&row.row_id());
	}));
	base_box.append(&nav);
	base_box.append(&settings_stack);

	window.set_child(Some(&base_box));

	window.show();
}
