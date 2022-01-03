// SPDX-License-Identifier: GPL-3.0-only

#[macro_use]
extern crate cascade;

mod sections;
mod ui;
mod widgets;

use crate::ui::section;
use gtk4::{
	gdk::Display, gio::ApplicationFlags, prelude::*, CssProvider, StyleContext,
	STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use std::rc::Rc;

fn main() {
	let application = gtk4::Application::new(
		Some("com.system76.cosmic.settings"),
		ApplicationFlags::default(),
	);
	application.connect_activate(build_ui);
	application.run();
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

	let ui = Rc::new(ui::SettingsGui::new(&window));
	section::setup::<sections::WifiSection>(ui.clone());
	section::setup::<sections::DesktopSection>(ui.clone());
	section::setup::<sections::KeyboardSection>(ui);

	window.show();
}
