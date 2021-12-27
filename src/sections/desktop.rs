use super::{Section, SectionInfo};
use gtk4::prelude::*;

pub struct DesktopSection;

impl SectionInfo for DesktopSection {
	const NAME: &'static str = "Desktop";
	const ICON: &'static str = "user-desktop-symbolic";
}

impl Section for DesktopSection {
	fn widget(boxed: &gtk4::Box) {
		let label = gtk4::Label::new("Hello world!".into());
		boxed.append(&label);
	}
}
