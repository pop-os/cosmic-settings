// SPDX-License-Identifier: GPL-3.0-only

use crate::{sections::SettingsGroup, ui::SettingsGui, widgets::SettingsEntry};
use gtk4::{prelude::*, Align, CheckButton};
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
	}
}
