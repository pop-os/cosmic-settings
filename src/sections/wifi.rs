// SPDX-License-Identifier: GPL-3.0-only

use super::{Section, SectionLayout, SettingsGroup};
use crate::{ui::SettingsGui, widgets::SettingsEntry};
use gtk4::{glib::clone, prelude::*, Align, Button, Image, Label, Orientation, Switch};
use std::{cell::RefCell, rc::Rc};

pub struct WifiSection;

impl Section for WifiSection {
	const NAME: &'static str = "WiFi";
	const ICON: &'static str = "network-wireless-symbolic";

	fn layout() -> SectionLayout {
		SectionLayout::Single(vec![
			AirplaneMode::boxed(),
			Wifi::boxed(),
			VisibleNetworks::boxed(),
			AdditionalNetworkSettings::boxed(),
		])
	}
}

#[derive(Default)]
struct AirplaneMode;

impl SettingsGroup for AirplaneMode {
	fn title(&self) -> &'static str {
		"Airplane Mode"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["airplane", "disable", "turn off"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let checkbox = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Airplane Mode");
			..set_description("Disables Wi-Fi, Bluetooth, and mobile broadband");
			..set_child(&checkbox);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
struct Wifi;

impl SettingsGroup for Wifi {
	fn title(&self) -> &'static str {
		"Wi-Fi"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["wifi", "wi-fi", "wireless", "disable", "turn off"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let checkbox = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Wi-Fi");
			..set_description("Disables all Wi-Fi functions");
			..set_child(&checkbox);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
struct VisibleNetworks;

impl VisibleNetworks {
	fn get_wifi_networks() -> Vec<String> {
		vec![
			"Example Wi-Fi #1".into(),
			"Example Wi-Fi #2".into(),
			"Example Wi-Fi #3".into(),
			"Example Wi-Fi #4".into(),
			"Example Wi-Fi #5".into(),
		]
	}
}

impl SettingsGroup for VisibleNetworks {
	fn title(&self) -> &'static str {
		"Visible Networks"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["wifi", "wi-fi", "connect", "ssid"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		for network in Self::get_wifi_networks() {
			let inner_box = gtk4::Box::new(Orientation::Horizontal, 16);
			let icon = Image::from_icon_name(Some("network-wireless-symbolic"));
			let label = Label::new(Some(&network));
			inner_box.append(&icon);
			inner_box.append(&label);
			let connect_button = Button::builder()
				.child(&inner_box)
				.css_classes(vec!["settings-button".into()])
				.hexpand(true)
				.build();
			let settings_button = Button::builder()
				.icon_name("emblem-system-symbolic")
				.css_classes(vec!["settings-button".into()])
				.build();
			let outer_box = gtk4::Box::new(Orientation::Horizontal, 0);
			outer_box.append(&connect_button);
			outer_box.append(&settings_button);
			target.append(&outer_box);
		}
	}
}

#[derive(Default)]
struct AdditionalNetworkSettings;

impl AdditionalNetworkSettings {
	pub fn create_hidden_network_popup() -> gtk4::Box {
		let base = gtk4::Box::builder()
			.orientation(Orientation::Vertical)
			.build();
		let label = Label::new(Some("Hello World!"));
		base.append(&label);
		base
	}
}

impl SettingsGroup for AdditionalNetworkSettings {
	fn title(&self) -> &'static str {
		"Additional Network Settings"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&[
			"wifi", "wi-fi", "wireless", "hotspot", "hidden", "network", "tether", "hot-spot",
			"hot spot",
		]
	}

	fn layout(&self, target: &gtk4::Box, ui: Rc<SettingsGui>) {
		let button = Button::builder()
			.label("Wi-Fi Hotspot")
			.css_classes(vec!["settings-button".into()])
			.build();
		target.append(&button);
		let button = Button::builder()
			.label("Connect to Hidden Networks")
			.css_classes(vec!["settings-button".into()])
			.build();
		button.connect_clicked(clone!(@strong ui => move |_| {
			ui.popup.pop_up("hidden-net");
		}));
		target.append(&button);
		ui.popup
			.add_overlay("hidden-net", Self::create_hidden_network_popup);
	}
}
