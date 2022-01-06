// SPDX-License-Identifier: GPL-3.0-only

use super::{Section, SectionLayout, SettingsGroup};
use crate::{ui::SettingsGui, widgets::SettingsEntry};
use bytesize::ByteSize;
use gtk4::prelude::*;
use std::rc::Rc;
use sysinfo::{DiskExt, ProcessorExt, System, SystemExt};

thread_local!(static SYSTEM_INFO: System = System::new_all());

pub struct AboutSection;

impl Section for AboutSection {
	const NAME: &'static str = "About";
	const ICON: &'static str = "dialog-information-symbolic";

	fn layout() -> SectionLayout {
		SectionLayout::Single(vec![Device::boxed(), DeviceSpecs::boxed(), OsInfo::boxed()])
	}
}

#[derive(Default)]
struct Device;
impl SettingsGroup for Device {
	fn title(&self) -> &'static str {
		""
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["device", "hostname", "name", "computer"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		SYSTEM_INFO.with(|info| {
			if let Some(hostname) = info.host_name() {
				let row = cascade! {
					SettingsEntry::new();
					..set_title("Device Name");
					..set_child_label(hostname);
				};
				target.append(&row);
			}
		});
	}
}

#[derive(Default)]
struct DeviceSpecs;

impl SettingsGroup for DeviceSpecs {
	fn title(&self) -> &'static str {
		""
	}

	fn keywords(&self) -> &'static [&'static str] {
		&[
			"device",
			"specs",
			"hardware",
			"cpu",
			"memory",
			"ram",
			"storage",
			"gpu",
			"graphics",
			"processor",
			"disk",
			"space",
		]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		SYSTEM_INFO.with(|info| {
			let memory = ByteSize::kb(info.total_memory());
			let row = cascade! {
				SettingsEntry::new();
				..set_title("Memory");
				..set_child_label(memory.to_string_as(true));
			};
			target.append(&row);
			let cpu = info.global_processor_info();
			let row = cascade! {
				SettingsEntry::new();
				..set_title("Processor");
				..set_child_label(cpu.brand());
			};
			target.append(&row);
			let disk = &info.disks()[0];
			let row = cascade! {
				SettingsEntry::new();
				..set_title("Disk Capacity");
				..set_child_label(ByteSize::b(disk.total_space()).to_string_as(true));
			};
			target.append(&row);
		});
	}
}

#[derive(Default)]
struct OsInfo;

impl SettingsGroup for OsInfo {
	fn title(&self) -> &'static str {
		""
	}

	fn keywords(&self) -> &'static [&'static str] {
		&[
			"x11",
			"xorg",
			"wayland",
			"os",
			"operating system",
			"window",
			"windowing system",
			"64 bit",
			"32 bit",
			"64-bit",
			"32-bit",
			"x86",
			"x64",
			"x86_64",
		]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		SYSTEM_INFO.with(|info| {
			if let (Some(os_version), Some(os_name)) = (info.os_version(), info.name()) {
				let row = cascade! {
					SettingsEntry::new();
					..set_title("OS Name");
					..set_child_label(format!("{} {}", os_name, os_version));
				};
				target.append(&row);
			}

			let os_type = if cfg!(target_arch = "aarch64") {
				"64-bit (ARM)"
			} else if cfg!(target_arch = "x86_64") {
				"64-bit (x86)"
			} else if cfg!(target_arch = "x86") {
				"32-bit (x86)"
			} else {
				"Unknown"
			};
			let row = cascade! {
				SettingsEntry::new();
				..set_title("OS Type");
				..set_child_label(os_type);
			};
			target.append(&row);

			let window_system = match std::env::var("XDG_SESSION_TYPE").as_deref() {
				Ok("wayland") => "Wayland",
				Ok("x11") => "X11",
				_ => "Unknown",
			};
			let row = cascade! {
				SettingsEntry::new();
				..set_title("Windowing System");
				..set_child_label(window_system);
			};
			target.append(&row);
		});
	}
}
