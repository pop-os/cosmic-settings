use super::{Section, SectionLayout, SettingsGroup};
use crate::widgets::SettingsEntry;
use gtk4::prelude::*;

pub struct WifiSection;

impl Section for WifiSection {
	const NAME: &'static str = "WiFi";
	const ICON: &'static str = "network-wireless-symbolic";

	fn layout() -> SectionLayout {
		SectionLayout::Single(vec![AirplaneMode::new(), Wifi::new()])
	}
}

#[derive(Default)]
struct AirplaneMode;

impl SettingsGroup for AirplaneMode {
	fn title(&self) -> &'static str {
		"Airplane Mode"
	}

	fn keywords(&self) -> &[&'static str] {
		&["airplane", "disable", "turn off"]
	}

	fn layout(&self, target: &gtk4::Box) {
		let checkbox = gtk4::Switch::builder().valign(gtk4::Align::Center).build();
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

	fn keywords(&self) -> &[&'static str] {
		&["wifi", "wi-fi", "wireless", "disable", "turn off"]
	}

	fn layout(&self, target: &gtk4::Box) {
		let checkbox = gtk4::Switch::builder().valign(gtk4::Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Wi-Fi");
			..set_description("Disables all Wi-Fi functions");
			..set_child(&checkbox);
		};
		target.append(&entry);
	}
}
