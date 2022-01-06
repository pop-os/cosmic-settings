// SPDX-License-Identifier: GPL-3.0-only

use super::{Section, SectionLayout, SettingsGroup};
use crate::{ui::SettingsGui, widgets::SettingsEntry};
use bytesize::ByteSize;
use gtk4::{prelude::*, Label, Orientation};
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

impl DeviceSpecs {
	fn gpus() -> Vec<String> {
		use std::process::Command;

		let mut out = vec![];
		if let Ok(output) = Command::new("glxinfo").output() {
			let output = String::from_utf8_lossy(&output.stdout);
			for line in output.lines() {
				if let Some(renderer_string) = line.strip_prefix("OpenGL renderer string: ") {
					let mut gpu = renderer_string.trim().to_string();
					if let Some(index) = gpu.find(';') {
						gpu.truncate(index);
					}
					out.push(gpu);
				}
			}
		}
		out
	}
}

impl SettingsGroup for DeviceSpecs {
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
			let gpus = Self::gpus();
			let graphics_box = gtk4::Box::new(Orientation::Vertical, 8);
			if !gpus.is_empty() {
				for gpu in gpus {
					let label = Label::builder()
						.label(&gpu)
						.css_classes(vec!["settings-entry-text".into()])
						.build();
					graphics_box.append(&label);
				}
				let graphics = cascade! {
					SettingsEntry::new();
					..set_title("Graphics");
					..set_child(&graphics_box);
				};
				target.append(&graphics);
			}
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
