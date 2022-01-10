// SPDX-License-Identifier: GPL-3.0-only

use crate::{sections::SettingsGroup, ui::SettingsGui, widgets::SettingsEntry};
use gtk4::{prelude::*, Align, CheckButton, SpinButton};
use std::rc::Rc;

#[derive(Default)]
pub struct Workspaces;

impl SettingsGroup for Workspaces {
	fn title(&self) -> &'static str {
		"Workspaces"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["workspace", "dynamic", "fixed"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let check = CheckButton::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Dynamic Workspaces");
			..set_description("Automatically removes empty workspaces");
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
			..set_title("Fixed Number of Workspaces");
			..set_description("Specify a number of Workspaces");
			..set_child(&check);
			..align_child(Align::Start);
		};
		target.append(&entry);
		let spin = SpinButton::with_range(1., 10., 1.);
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Number of Workspaces");
			..set_child(&spin);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
pub struct MultiMonitorBehavior;

impl SettingsGroup for MultiMonitorBehavior {
	fn title(&self) -> &'static str {
		"Multi-monitor Behavior"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["monitor", "screen", "display"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let check = CheckButton::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Workspaces Span Displays");
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
			..set_title("Workspaces on Primary Display Only");
			..set_child(&check);
			..align_child(Align::Start);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
pub struct PlacementWorkspacePicker;

impl SettingsGroup for PlacementWorkspacePicker {
	fn title(&self) -> &'static str {
		"Placement of the Workspace Picker"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["place", "side", "workspace", "pick", "picker"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let check = CheckButton::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Along the left side");
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
			..set_title("Along the right side");
			..set_child(&check);
			..align_child(Align::Start);
		};
		target.append(&entry);
	}
}
