// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced_wgpu::wgpu;
use std::{collections::HashMap, collections::HashSet, ffi::OsStr, process::Command};

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

        // Track seen GPUs by (vendor, device) and by name to handle different scenarios:
        // - Same GPU via different backends (Vulkan/OpenGL) -> deduplicate by device ID or name
        // - Multiple identical GPUs -> show all (different device IDs or backend enumeration order)
        // - Backends with invalid device IDs (0x0000) -> deduplicate by name
        let mut seen_devices = HashSet::new();
        let mut seen_names = HashSet::new();

        for adapter in adapters {
            let adapter_info = adapter.get_info();

            if adapter_info.device_type == wgpu::DeviceType::Cpu {
                continue;
            }

            let mut gpu_name = if let Some(pos) = adapter_info.name.find('(') {
                adapter_info.name[..pos].trim().to_string()
            } else {
                adapter_info.name
            };

            // Intel GPU quirk: wgpu sometimes reports incomplete names (just "Intel").
            // Use lspci as fallback to get proper full name like "Intel Corporation HD Graphics 500"
            if gpu_name == "Intel" && adapter_info.device != 0 {
                let lspci_gpus = get_lspci_gpu_names();
                if let Some(lspci_name) = lspci_gpus.get(&adapter_info.device) {
                    gpu_name = lspci_name.clone();
                }
                // If lspci lookup fails, keep "Intel"
            }

            let device_key = (adapter_info.vendor, adapter_info.device);
            if adapter_info.device != 0 && seen_devices.contains(&device_key) {
                continue;
            }

            if adapter_info.device == 0 && seen_names.contains(&gpu_name) {
                continue;
            }

            if adapter_info.device != 0 {
                seen_devices.insert(device_key);
            }
            seen_names.insert(gpu_name.clone());
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

/// Get GPU names from lspci, mapping device ID to full GPU name.
/// This is used as a quirk for Intel GPUs which often report incomplete names via their OpenGL and Vulkan drivers.
/// https://www.intel.com/content/www/us/en/support/articles/000005520/graphics.html
fn get_lspci_gpu_names() -> HashMap<u32, String> {
    let mut gpu_map = HashMap::new();

    let output = match Command::new("lspci").arg("-nn").output() {
        Ok(output) => output,
        Err(_) => return gpu_map,
    };

    let stdout = match std::str::from_utf8(&output.stdout) {
        Ok(s) => s,
        Err(_) => return gpu_map,
    };

    for line in stdout.lines() {
        // Look for any line containing VGA, 3D controller, or Display controller
        if !line.contains("VGA")
            && !line.contains("3D controller")
            && !line.contains("Display controller")
        {
            continue;
        }

        // Parse device ID from format: "00:02.0 VGA compatible controller [0300]: Intel Corporation HD Graphics 500 [8086:5a85] (rev 0b)"
        // We want to extract the device ID (5a85) and the name (Intel Corporation HD Graphics 500)

        if let Some(ids_start) = line.rfind('[') {
            if let Some(ids_end) = line.rfind(']') {
                let ids = &line[ids_start + 1..ids_end];

                // Parse vendor:device format like "8086:5a85"
                if let Some(colon_pos) = ids.find(':') {
                    let device_id_str = &ids[colon_pos + 1..];
                    if let Ok(device_id) = u32::from_str_radix(device_id_str, 16) {
                        // Extract the GPU name between ": " and the last "["
                        if let Some(name_start) = line.find(": ") {
                            let name_part = &line[name_start + 2..ids_start].trim();
                            // Remove any trailing parentheses with revision info
                            let gpu_name = if let Some(paren_pos) = name_part.rfind(" [") {
                                name_part[..paren_pos].trim()
                            } else {
                                name_part
                            };

                            if !gpu_name.is_empty() {
                                gpu_map.insert(device_id, gpu_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    gpu_map
}
