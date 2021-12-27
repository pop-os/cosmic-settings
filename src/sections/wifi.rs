use super::{Section, SettingsGroup};
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
		let am_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
		let label = gtk4::Label::new(Some("Airplane Mode"));
		let description =
			gtk4::Label::new(Some("Disables Wi-Fi, Bluetooth, and mobile broadband "));
		let label_desc_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
		label_desc_box.append(&label);
		label_desc_box.append(&description);
		let separator = gtk4::Separator::new(gtk4::Orientation::Vertical);
		am_box.append(&label_desc_box);
		am_box.append(&separator);
		let toggle = gtk4::ToggleButton::new();
		am_box.append(&toggle);
		target.append(&am_box);
	}
}
