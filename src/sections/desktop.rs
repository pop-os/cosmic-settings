// SPDX-License-Identifier: GPL-3.0-only

mod dock;

use super::{Section, SectionLayout, SettingsGroup};
use crate::{ui::SettingsGui, widgets::SettingsEntry};
use gtk4::{prelude::*, Align, CheckButton, Switch};
use std::rc::Rc;

pub struct DesktopSection;

impl Section for DesktopSection {
	const NAME: &'static str = "Desktop";
	const ICON: &'static str = "user-desktop-symbolic";

	fn layout() -> SectionLayout {
		SectionLayout::Multiple(vec![
			(
				"Desktop",
				vec![
					SuperKeyAction::boxed(),
					HotCorner::boxed(),
					TopBar::boxed(),
					WindowControls::boxed(),
				],
			),
			(
				"Dock",
				vec![
					dock::Dock::boxed(),
					dock::DockOptions::boxed(),
					dock::DockVisibility::boxed(),
					dock::DockSize::boxed(),
				],
			),
		])
	}
}

#[derive(Default)]
struct SuperKeyAction;

impl SettingsGroup for SuperKeyAction {
	fn title(&self) -> &'static str {
		"Super Key Action"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&[
			"super",
			"launcher",
			"window",
			"workspace",
			"overview",
			"app",
		]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let check = CheckButton::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Launcher");
			..set_description("Pressing the Super key opens the Launcher");
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
			..set_title("Workspaces");
			..set_description("Pressing the Super key opens the Window and Workspaces Overview");
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
			..set_title("Applications");
			..set_description("Pressing the Super key opens the Applications Overview");
			..set_child(&check);
			..align_child(Align::Start);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
struct HotCorner;

impl SettingsGroup for HotCorner {
	fn title(&self) -> &'static str {
		"Hot Corner"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["corner", "hot"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let switch = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Enable top-left hot corner for Workspaces");
			..set_child(&switch);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
struct TopBar;

impl SettingsGroup for TopBar {
	fn title(&self) -> &'static str {
		"Top Bar"
	}

	fn keywords(&self) -> &'static [&'static str] {
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

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let switch = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Show Workspaces Button");
			..set_child(&switch);
		};
		target.append(&entry);
		let switch = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Show Applications Button");
			..set_child(&switch);
		};
		target.append(&entry);
	}
}

#[derive(Default)]
struct WindowControls;

impl SettingsGroup for WindowControls {
	fn title(&self) -> &'static str {
		"Window Controls"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["minimize", "maximize", "window", "controls"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let switch = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Show Minimize Button");
			..set_child(&switch);
		};
		target.append(&entry);
		let switch = Switch::builder().valign(Align::Center).build();
		let entry = cascade! {
			SettingsEntry::new();
			..set_title("Show Maximize Button");
			..set_child(&switch);
		};
		target.append(&entry);
	}
}
