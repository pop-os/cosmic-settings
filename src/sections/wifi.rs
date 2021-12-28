use super::{Section, SettingsGroup};
use crate::widgets::SettingsEntry;
use gtk4::prelude::*;

pub struct WifiSection;

impl Section for WifiSection {
	const NAME: &'static str = "WiFi";
	const ICON: &'static str = "network-wireless-symbolic";

	fn settings() -> Vec<Box<dyn SettingsGroup>> {
		vec![AirplaneMode::new()]
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
		let checkbox = gtk4::Switch::new();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Airplane Mode");
			..set_description("Disables Wi-Fi, Bluetooth, and mobile broadband");
			..set_child(&checkbox);
		};
		target.append(&entry);
	}
}
