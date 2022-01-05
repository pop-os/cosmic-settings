// SPDX-License-Identifier: GPL-3.0-only

use crate::{sections::SettingsGroup, ui::SettingsGui, widgets::SettingsEntry};
use gtk4::{prelude::*, Align, CheckButton, Switch};
use std::rc::Rc;

#[derive(Default)]
pub struct Dock;

impl SettingsGroup for Dock {
	fn title(&self) -> &'static str {
		"Dock"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["dock"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let switch = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Enable Dock");
			..set_child(&switch);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
pub struct DockOptions;

impl SettingsGroup for DockOptions {
	fn title(&self) -> &'static str {
		"Dock Options"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&[
			"dock",
			"edge",
			"launcher",
			"workspace",
			"application",
			"drive",
			"mount",
			"click",
			"icon",
		]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let switch = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Extend dock to the edge of the screen");
			..set_child(&switch);
		};
		target.append(&entry);
		let switch = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Show Launcher Icon in Dock");
			..set_child(&switch);
		};
		target.append(&entry);
		let switch = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Show Workspaces Icon in Dock");
			..set_child(&switch);
		};
		target.append(&entry);
		let switch = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Show Applications Icon in Dock");
			..set_child(&switch);
		};
		target.append(&entry);
		let switch = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Show Mounted Drives");
			..set_child(&switch);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
pub struct DockVisibility;

impl SettingsGroup for DockVisibility {
	fn title(&self) -> &'static str {
		"Dock Visibility"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["dock", "visible", "visibility", "hide"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let check = CheckButton::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Always Visible");
			..set_child(&check);
			..align_child(Align::Start);
		};
		target.append(&entry);
		let check = CheckButton::builder()
			.valign(Align::Center)
			.group(&check)
			.build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Always hide");
			..set_description("Dock always hides unless actively being revealed by the mouse");
			..set_child(&check);
			..align_child(Align::Start);
		};
		target.append(&entry);
		let check = CheckButton::builder()
			.valign(Align::Center)
			.group(&check)
			.build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Intelligently hide");
			..set_description("Dock hides when any window overlaps the dock area");
			..set_child(&check);
			..align_child(Align::Start);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
pub struct DockSize;

impl SettingsGroup for DockSize {
	fn title(&self) -> &'static str {
		"Dock Visibility"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["dock", "size"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let check = CheckButton::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Small (36px)");
			..set_child(&check);
			..align_child(Align::Start);
		};
		target.append(&entry);
		let check = CheckButton::builder()
			.valign(Align::Center)
			.group(&check)
			.build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Medium (48px)");
			..set_child(&check);
			..align_child(Align::Start);
		};
		target.append(&entry);
		let check = CheckButton::builder()
			.valign(Align::Center)
			.group(&check)
			.build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Large (60px)");
			..set_child(&check);
			..align_child(Align::Start);
		};
		target.append(&entry);
	}
}
