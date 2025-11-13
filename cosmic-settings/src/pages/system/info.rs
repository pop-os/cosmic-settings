// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced_wgpu::wgpu;
use std::{collections::HashSet, ffi::OsStr};

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
    pub kernel_version: String,
    pub processor: String,
    pub windowing_system: String,
}

impl Info {
    pub fn load() -> Info {
        let mut info = Info::default();

        info.os_architecture = architecture();
        info.kernel_version = kernel_version();
        info.hardware_model = hardware_model();
        info.operating_system = operating_system();
        info.processor = processor_name();

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

        // prefer XDG_SESSION_DESKTOP because the value is singular
        if let Ok(mut session) = std::env::var("XDG_SESSION_DESKTOP")
            .or_else(|_| std::env::var("XDG_CURRENT_DESKTOP"))
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
        {
            if let Some(first) = session.get_mut(0..1) {
                first.make_ascii_uppercase();
            }
            info.desktop_environment = session;
        }

        // Use wgpu to enumerate GPUs. Works cross-platform and doesn't require external tools
        let instance = wgpu::Instance::default();
        let adapters = instance.enumerate_adapters(wgpu::Backends::all());

        // Track seen GPUs by vendor ID to avoid duplicates from different backends
        let mut seen_vendors = HashSet::new();

        for adapter in adapters {
            let adapter_info = adapter.get_info();

            if adapter_info.device_type == wgpu::DeviceType::Cpu {
                continue;
            }

            // Deduplicate by vendor ID. Same GPU can be exposed via multiple backends
            if seen_vendors.contains(&adapter_info.vendor) {
                continue;
            }

            seen_vendors.insert(adapter_info.vendor);

            // Strip driver info in parentheses for cleaner display
            let gpu_name = if let Some(pos) = adapter_info.name.find('(') {
                adapter_info.name[..pos].trim().to_string()
            } else {
                adapter_info.name
            };

            info.graphics.push(gpu_name);
        }

        info
    }
}

fn architecture() -> String {
    read_to_string("/proc/sys/kernel/arch")
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn kernel_version() -> String {
    read_to_string("/proc/version")
        .and_then(|content| content.split_whitespace().nth(2).map(|s| s.to_string()))
        .unwrap_or_default()
}

fn hardware_model() -> String {
    const DMI_DIR: &str = "/sys/devices/virtual/dmi/id/";
    const VERSION_IGNORING_PRODUCTS: &[&str] = &["Dev One"];

    let sys_vendor = read_to_string(&format!("{DMI_DIR}sys_vendor"))
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    if sys_vendor.is_empty() {
        // Fallback to sysinfo if DMI information is not available
        return sysinfo::Product::name().unwrap_or_default();
    }

    let mut model = sys_vendor.clone();

    if let Some(mut name) =
        read_to_string(&format!("{DMI_DIR}board_name")).map(|s| s.trim().to_string())
    {
        if !name.is_empty() && name != sys_vendor {
            // Ensure that the name does not contain the vendor
            name = name
                .strip_prefix(&sys_vendor)
                .map(|s| s.trim().to_string())
                .unwrap_or(name);

            model.push(' ');
            model.push_str(&name);

            if let Some(version) =
                read_to_string(&format!("{DMI_DIR}board_version")).map(|s| s.trim().to_string())
            {
                if !version.is_empty() && !VERSION_IGNORING_PRODUCTS.contains(&name.as_str()) {
                    model.push_str(&format!(" ({version})"));
                }
            }
        }
    }

    model
}

fn operating_system() -> String {
    let content = read_to_string("/etc/os-release").unwrap_or_default();

    for line in content.lines() {
        if let Some(mut value) = line.strip_prefix("PRETTY_NAME=") {
            value = value.strip_prefix('"').unwrap_or(value);
            value = value.strip_suffix('"').unwrap_or(value);
            return value.trim().to_string();
        }
    }

    String::new()
}

fn processor_name() -> String {
    if let Some(cpuinfo) = read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if let Some(info) = line.strip_prefix("model name") {
                if let Some(info) = info.trim_start().strip_prefix(':') {
                    return info.trim().to_string();
                }
                break;
            }
        }
    }

    // Fallback to sysinfo if /proc/cpuinfo is not present
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing().with_cpu(sysinfo::CpuRefreshKind::everything()),
    );
    sys.cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_default()
}

fn read_to_string<P: AsRef<OsStr>>(path: P) -> Option<String> {
    std::fs::read_to_string(path.as_ref()).ok()
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    const FACTOR: f64 = 1024.0;

    if bytes == 0 {
        return String::from("0 B");
    }

    let bytes_f64 = bytes as f64;
    let exp = (bytes_f64.ln() / FACTOR.ln()).floor() as usize;
    let exp = exp.min(UNITS.len() - 1);
    let value = bytes_f64 / FACTOR.powi(exp as i32);

    format!("{:.2} {}", value, UNITS[exp])
}
