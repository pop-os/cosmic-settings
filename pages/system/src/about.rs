// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use bumpalo::Bump;
use std::{collections::HashSet, ffi::OsStr, io::Read};

use concat_in_place::strcat;
use const_format::concatcp;

const DMI_DIR: &str = "/sys/devices/virtual/dmi/id/";
const BOARD_NAME: &str = concatcp!(DMI_DIR, "board_name");
const BOARD_VERSION: &str = concatcp!(DMI_DIR, "board_version");
const SYS_VENDOR: &str = concatcp!(DMI_DIR, "sys_vendor");

const VERSION_IGNORING_PRODUCTS: &[&str] = &["Dev One"];

#[must_use]
#[derive(Clone, Debug, Default)]
pub struct Info {
    pub desktop_environment: String,
    pub device_name: String,
    pub disk_capacity: String,
    pub graphics: Vec<String>,
    pub hardware_model: String,
    pub memory: String,
    pub operating_system: String,
    pub os_architecture: String,
    pub processor: String,
    pub windowing_system: String,
}

impl Info {
    pub fn load() -> Info {
        let mut info = Info::default();
        let mut bump = Bump::with_capacity(8 * 1024);

        architecture(&bump, &mut info.os_architecture);
        bump.reset();

        hardware_model(&bump, &mut info.hardware_model);
        bump.reset();

        operating_system(&bump, &mut info.operating_system);
        bump.reset();

        processor_name(&bump, &mut info.processor);
        bump.reset();

        let mut sys = sysinfo::System::new();

        let disks = sysinfo::Disks::new_with_refreshed_list();

        sys.refresh_memory();

        let mut total_capacity = 0;
        let mut disk_set = HashSet::new();
        for disk in disks.list() {
            if disk_set.contains(disk.name()) {
                continue;
            }
            disk_set.insert(disk.name());
            total_capacity += disk.total_space();
        }

        info.disk_capacity = format_size(total_capacity);

        if let Some(name) = sysinfo::System::host_name() {
            info.device_name = name;
        }

        info.memory = format_size(sys.total_memory());

        if let Ok(mut session) = std::env::var("XDG_SESSION_TYPE") {
            if let Some(first) = session.get_mut(0..1) {
                first.make_ascii_uppercase();
            }

            info.windowing_system = session;
        }

        // prefer XDG_SESSION_DESKTOP because the value is singular: https://www.freedesktop.org/software/systemd/man/latest/pam_systemd.html
        if let Ok(mut session) = std::env::var("XDG_SESSION_DESKTOP")
            // otherwise, XDG_CURRENT_DESKTOP (could be plural): https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html
            .or_else(|_| std::env::var("XDG_CURRENT_DESKTOP"))
            // fallback to legacy environment variable
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
        {
            if let Some(first) = session.get_mut(0..1) {
                first.make_ascii_uppercase();
            }

            info.desktop_environment = session;
        }

        if let Ok(output) = std::process::Command::new("lspci").output() {
            if let Ok(stdout) = std::str::from_utf8(&output.stdout) {
                for line in stdout.lines() {
                    if let Some(pos) = memchr::memmem::find(line.as_bytes(), b"VGA") {
                        let line = &line[pos + 3..];
                        if let Some(pos) = memchr::memmem::find(line.as_bytes(), b": ") {
                            info.graphics.push(String::from(&line[pos + 2..]));
                        }
                    }
                }
            }
        }

        info
    }
}

pub fn architecture(bump: &Bump, arch: &mut String) {
    let buffer = &mut bumpalo::collections::Vec::new_in(bump);
    if let Some(value) = read_to_string("/proc/sys/kernel/arch", buffer) {
        arch.push_str(value.trim());
    }
}

pub fn hardware_model(bump: &Bump, hardware_model: &mut String) {
    let buffer = &mut bumpalo::collections::Vec::new_in(bump);
    if let Some(mut sys_vendor) = read_to_string(SYS_VENDOR, buffer) {
        sys_vendor = sys_vendor.trim();

        hardware_model.push_str(sys_vendor);

        let buffer = &mut bumpalo::collections::Vec::new_in(bump);
        if let Some(mut name) = read_to_string(BOARD_NAME, buffer) {
            name = name.trim();

            if !name.is_empty() && name != sys_vendor {
                // Ensure that the name does not contain the vendor.
                name = match name.strip_prefix(sys_vendor) {
                    Some(stripped) => stripped.trim(),
                    None => name,
                };

                let _str = strcat!(&mut *hardware_model, " " name);
            }

            let buffer = &mut bumpalo::collections::Vec::new_in(bump);
            if let Some(mut version) = read_to_string(BOARD_VERSION, buffer) {
                version = version.trim();

                if !version.is_empty() && !VERSION_IGNORING_PRODUCTS.contains(&name) {
                    let _str = strcat!(hardware_model, " (" version.trim() ")");
                }
            }
        }
    } else {
        // simple fallback to sysinfo if DMI information is not available
        hardware_model.push_str(&sysinfo::Product::name().unwrap_or("".to_string()));
    }
}

pub fn operating_system(bump: &Bump, operating_system: &mut String) {
    let mut buffer = bumpalo::collections::Vec::new_in(bump);
    let Some(os_release) = read_to_string("/etc/os-release", &mut buffer) else {
        return;
    };

    for line in os_release.lines() {
        if let Some(mut value) = line.strip_prefix("PRETTY_NAME=") {
            if let Some(v) = value.strip_prefix('"') {
                value = v;
            }

            if let Some(v) = value.strip_suffix('"') {
                value = v;
            }

            operating_system.push_str(value.trim());
            break;
        }
    }
}

pub fn processor_name(bump: &Bump, name: &mut String) {
    if let Some(cpuinfo) = read_to_string(
        "/proc/cpuinfo",
        &mut bumpalo::collections::Vec::new_in(bump),
    ) {
        for line in cpuinfo.lines() {
            if let Some(info) = line.strip_prefix("model name") {
                if let Some(info) = info.trim_start().strip_prefix(':') {
                    name.push_str(info.trim());
                }

                break;
            }
        }
    }

    // fallback to sysinfo if /proc/cpuinfo is not present
    let s = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing().with_cpu(sysinfo::CpuRefreshKind::everything()),
    );
    name.push_str(s.cpus().into_iter().nth(0).unwrap().brand());
}

pub fn read_to_string<'a, P: AsRef<OsStr>>(
    path: P,
    buffer: &'a mut bumpalo::collections::Vec<u8>,
) -> Option<&'a str> {
    let mut file = std::fs::File::open(path.as_ref()).ok()?;

    if let Ok(metadata) = file.metadata() {
        if let Ok(len) = usize::try_from(metadata.len()) {
            buffer.reserve_exact(len);
        }
    }

    let mut buf = [0; 16 * 1024];

    loop {
        match file.read(&mut buf) {
            Ok(0) => break,
            Ok(read) => buffer.extend_from_slice(&buf[..read]),
            Err(_) => return None,
        }
    }

    std::str::from_utf8(buffer.as_slice()).ok()
}

fn format_size(size: u64) -> String {
    format!(
        "{:.2}",
        byte_unit::Byte::from_u64(size).get_appropriate_unit(byte_unit::UnitType::Binary)
    )
}
