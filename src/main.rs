mod sections;

use gtk4::{CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, StyleContext, gdk::Display, prelude::*};
use crate::sections::{SectionInfo};

fn main() {
	let application =
		gtk4::Application::new(Some("com.system76.cosmic.settings"), Default::default());
	application.connect_activate(build_ui);
	application.run();
}

fn setup_section<S: SectionInfo>(nav: &gtk4::ListBox) {
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
	let button = gtk4::Button::builder().child(&entry_box).build();
	let row = gtk4::ListBoxRow::builder()
		.margin_bottom(8)
		.margin_top(8)
		.margin_end(8)
		.margin_start(8)
		.child(&button)
		.css_name("rounded")
		.build();
	nav.append(&row);
}

fn build_ui(application: &gtk4::Application) {
	let provider = CssProvider::new();
	provider.load_from_data(include_bytes!("style.css"));
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
	let nav = gtk4::ListBox::builder()
		.margin_top(20)
		.margin_bottom(20)
		.margin_start(12)
		.margin_end(12)
		.css_name("nav")
		.build();
	setup_section::<sections::WifiSection>(&nav);
	setup_section::<sections::DesktopSection>(&nav);
	base_box.append(&nav);

	window.set_child(Some(&base_box));

	window.show();
}
