use super::{Section, SectionLayout, SettingsGroup};
use crate::widgets::SettingsEntry;
use gtk4::prelude::*;

pub struct DesktopSection;

impl Section for DesktopSection {
	const NAME: &'static str = "Desktop";
	const ICON: &'static str = "user-desktop-symbolic";

	fn layout() -> SectionLayout {
		SectionLayout::Multiple(vec![("Desktop", vec![TopBar::new()])])
	}
}

#[derive(Default)]
struct TopBar;

impl SettingsGroup for TopBar {
	fn title(&self) -> &'static str {
		"Top Bar"
	}

	fn keywords(&self) -> &[&'static str] {
		&[
			"top bar",
			"menu bar",
			"workspace",
			"application",
			"app",
			"date",
			"time",
			"clock",
		]
	}

	fn layout(&self, target: &gtk4::Box) {
		let switch = gtk4::Switch::builder().valign(gtk4::Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Show Workspaces Button");
			..set_child(&switch);
		};
		target.append(&entry);
		let switch = gtk4::Switch::builder().valign(gtk4::Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Show Applications Button");
			..set_child(&switch);
		};
		target.append(&entry);
	}
}
