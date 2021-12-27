use super::{Section, SectionInfo};
use gtk4::prelude::*;

pub struct WifiSection;

impl SectionInfo for WifiSection {
	const NAME: &'static str = "WiFi";
	const ICON: &'static str = "network-wireless-symbolic";
}

impl Section for WifiSection {
	fn widget(boxed: &gtk4::Box) {
		let label = gtk4::Label::new("Hello world!".into());
		boxed.append(&label);
	}
}
