use super::{Section, SectionLayout, SettingsGroup};
use crate::widgets::SettingsEntry;
use gtk4::prelude::*;

pub struct KeyboardSection;

impl Section for KeyboardSection {
	const NAME: &'static str = "Keyboard";
	const ICON: &'static str = "preferences-desktop-keyboard-symbolic";

	fn layout() -> SectionLayout {
		SectionLayout::Multiple(vec![
			(
				"Input",
				vec![InputSourceSwitching::new(), TypeSpecialCharacters::new()],
			),
			("Keyboard Shortcuts", vec![]),
		])
	}
}

#[derive(Default)]
struct InputSourceSwitching;

impl SettingsGroup for InputSourceSwitching {
	fn title(&self) -> &'static str {
		"Input Source Switching"
	}

	fn keywords(&self) -> &[&'static str] {
		&["input", "source", "switch", "shortcut", "keyboard"]
	}

	fn layout(&self, target: &gtk4::Box) {
		let check = gtk4::CheckButton::builder()
			.valign(gtk4::Align::Center)
			.build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Use the same source for all windows");
			..set_child(&check);
			..align_child(gtk4::Align::Start);
		};
		target.append(&entry);
		let check = gtk4::CheckButton::builder()
			.valign(gtk4::Align::Center)
			.group(&check)
			.build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Switch input sources individually for each window");
			..set_child(&check);
			..align_child(gtk4::Align::Start);
		};
		target.append(&entry);
		let label = gtk4::Label::new(Some("Super+Space"));
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Keyboard Shortcut");
			..set_description("This can be changed in Shortcuts");
			..set_child(&label);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
struct TypeSpecialCharacters;

impl SettingsGroup for TypeSpecialCharacters {
	fn title(&self) -> &'static str {
		"Type Special Characters"
	}

	fn keywords(&self) -> &[&'static str] {
		&[
			"type",
			"special",
			"characters",
			"unicode",
			"utf",
			"alternate char",
			"compose",
		]
	}

	fn layout(&self, target: &gtk4::Box) {
		let label = gtk4::Label::new(Some("Layout Default"));
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Alternate Characters Key");
			..set_description("Hold down and type to enter different characters");
			..set_child(&label);
		};
		target.append(&entry);
		let label = gtk4::Label::new(Some("Layout Default"));
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Compose Key");
			..set_child(&label);
		};
		target.append(&entry);
	}
}
