use super::{Section, SettingsGroup};
use gtk4::prelude::*;

pub struct DesktopSection;

impl Section for DesktopSection {
	const NAME: &'static str = "Desktop";
	const ICON: &'static str = "user-desktop-symbolic";

	fn settings() -> Vec<Box<dyn SettingsGroup>> {
		vec![]
	}
}
