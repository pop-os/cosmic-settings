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
        let mut info = Info {
            os_architecture: architecture(),
            kernel_version: kernel_version(),
            hardware_model: hardware_model(),
            operating_system: operating_system(),
            processor: processor_name(),
            ..Default::default()
        };

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

            // Intel GPU quirk: wgpu sometimes reports incomplete names (just "Intel" or "Mesa Intel").
            // Mesa OpenGL backend also reports Intel GPUs with "Mesa Intel(R)" prefix and device ID 0.
            // These are generic/duplicate entries when the driver can't properly identify the GPU.
            // Use lspci as fallback to get proper full name like "Intel Corporation HD Graphics 500"
            let is_intel_quirk = gpu_name == "Intel"
                || gpu_name == "Mesa Intel"
                || (gpu_name.starts_with("Mesa Intel") && adapter_info.device == 0);

            if is_intel_quirk {
                if adapter_info.device != 0 {
                    let lspci_gpus = get_lspci_gpu_names();
                    if let Some(lspci_name) = lspci_gpus.get(&adapter_info.device) {
                        gpu_name = lspci_name.clone();
                    } else {
                        // If lspci lookup fails, skip this generic name to avoid duplicates
                        continue;
                    }
                } else {
                    // Skip generic Intel names with no device ID (Mesa OpenGL duplicates)
                    continue;
                }
            }

            // AMD parsing to append codename to specific name
            let is_amd_gpu = gpu_name.starts_with("AMD");

            if is_amd_gpu {
                if adapter_info.device != 0 {
                    let lspci_gpus = get_lspci_gpu_names();
                    // If we can't find an lspci name for a GPU, just keep its original name
                    if let Some(lspci_name) = lspci_gpus.get(&adapter_info.device) {
                        // Codename starts after first pair of brackets
                        if let Some(bracket_end) = lspci_name.find(']') {
                            let mut code_name = lspci_name[bracket_end + 1..].to_string();
                            // After codename, there may be another pair of brackets.
                            if let Some(bracket_start) = code_name.find('[') {
                                code_name = code_name[..bracket_start].to_string();
                            }
                            if code_name.trim() != "Device" {
                                gpu_name = format!("{} ({})", gpu_name, code_name.trim());
                            }
                        }
                    }
                } else {
                    // Can't identify GPU
                    continue;
                }
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

        // NVIDIA Optimus quirk: On laptops with NVIDIA Optimus (switchable graphics),
        // the dedicated NVIDIA GPU may be powered off to save battery and won't be
        // enumerated by wgpu. Use lspci as a fallback to detect these inactive GPUs.
        // This ensures users can see all GPUs in their system, even when not in use.
        // Reference: https://wiki.archlinux.org/title/NVIDIA_Optimus
        let lspci_gpus = get_all_lspci_gpus();
        for (vendor, device, name) in lspci_gpus {
            let device_key = (vendor, device);
            if !seen_devices.contains(&device_key) {
                seen_devices.insert(device_key);
                info.graphics.push(name);
            }
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

    let sys_vendor = read_to_string(format!("{DMI_DIR}sys_vendor"))
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    if sys_vendor.is_empty() {
        // Fallback to sysinfo if DMI information is not available
        return sysinfo::Product::name().unwrap_or_default();
    }

    let mut model = sys_vendor.clone();

    if let Some(mut name) =
        read_to_string(format!("{DMI_DIR}board_name")).map(|s| s.trim().to_string())
        && !name.is_empty()
        && name != sys_vendor
    {
        // Ensure that the name does not contain the vendor
        name = name
            .strip_prefix(&sys_vendor)
            .map(|s| s.trim().to_string())
            .unwrap_or(name);

        model.push(' ');
        model.push_str(&name);

        if let Some(version) =
            read_to_string(format!("{DMI_DIR}board_version")).map(|s| s.trim().to_string())
            && !version.is_empty()
            && !VERSION_IGNORING_PRODUCTS.contains(&name.as_str())
        {
            model.push_str(&format!(" ({version})"));
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
                if let Some(mut info) = info.trim_start().strip_prefix(':') {
                    if let Some(cpu_with) = info.find(" w/ ") {
                        info = &info[..cpu_with];
                    }
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

/// Get all GPUs from lspci with their vendor ID, device ID, and name.
fn get_all_lspci_gpus() -> Vec<(u32, u32, String)> {
    let mut gpus = Vec::new();

    let output = match Command::new("lspci").arg("-nn").output() {
        Ok(output) => output,
        Err(_) => return gpus,
    };

    let stdout = match std::str::from_utf8(&output.stdout) {
        Ok(s) => s,
        Err(_) => return gpus,
    };

    for line in stdout.lines() {
        if !line.contains("VGA compatible") // Avoid "Non-VGA unclassified"
            && !line.contains("3D controller")
            && !line.contains("Display controller")
        {
            continue;
        }

        // Parse device ID from format: "00:02.0 VGA compatible controller [0300]: Intel Corporation HD Graphics 500 [8086:5a85] (rev 0b)"
        // We want to extract vendor:device IDs (8086:5a85) and the name (Intel Corporation HD Graphics 500)

        if let Some(ids_start) = line.rfind('[')
            && let Some(ids_end) = line.rfind(']')
        {
            let ids = &line[ids_start + 1..ids_end];

            if let Some(colon_pos) = ids.find(':') {
                let vendor_id_str = &ids[..colon_pos];
                let device_id_str = &ids[colon_pos + 1..];

                if let (Ok(vendor_id), Ok(device_id)) = (
                    u32::from_str_radix(vendor_id_str, 16),
                    u32::from_str_radix(device_id_str, 16),
                ) && let Some(name_start) = line.find(": ")
                {
                    let full_name = line[name_start + 2..ids_start].trim();

                    // Look for marketing name in brackets like "GP108M [GeForce MX150]"
                    // We prefer the marketing name over the chip code
                    let gpu_name = if let Some(bracket_start) = full_name.find('[') {
                        if let Some(bracket_end) = full_name.find(']') {
                            let vendor_part = full_name[..bracket_start].trim();
                            let marketing_name = full_name[bracket_start + 1..bracket_end].trim();

                            let vendor =
                                vendor_part.split_whitespace().next().unwrap_or(vendor_part);

                            format!("{} {}", vendor, marketing_name)
                        } else {
                            full_name.to_string()
                        }
                    } else {
                        full_name.to_string()
                    };

                    if !gpu_name.is_empty() {
                        gpus.push((vendor_id, device_id, gpu_name));
                    }
                }
            }
        }
    }

    gpus
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
        if !line.contains("VGA compatible") // Avoid "Non-VGA unclassified"
            && !line.contains("3D controller")
            && !line.contains("Display controller")
        {
            continue;
        }

        // Parse device ID from format: "00:02.0 VGA compatible controller [0300]: Intel Corporation HD Graphics 500 [8086:5a85] (rev 0b)"
        // Or with marketing name: "00:02.0 VGA compatible controller: Intel Corporation Alder Lake-P GT2 [Iris Xe Graphics] [8086:46a6] (rev 0c)"
        // We want to extract the device ID (5a85/46a6) and the name, preferring marketing name if available

        if let Some(ids_start) = line.rfind('[')
            && let Some(ids_end) = line.rfind(']')
        {
            let ids = &line[ids_start + 1..ids_end];

            // Parse vendor:device format like "8086:5a85"
            if let Some(colon_pos) = ids.find(':') {
                let device_id_str = &ids[colon_pos + 1..];
                if let Ok(device_id) = u32::from_str_radix(device_id_str, 16) {
                    // Extract the GPU name between ": " and the last "["
                    if let Some(name_start) = line.find(": ") {
                        let full_name = line[name_start + 2..ids_start].trim();
                        let gpu_name = full_name
                            .replace(" Corporation", "")
                            .replace(" [Intel Graphics]", "");
                        if !gpu_name.is_empty() {
                            gpu_map.insert(device_id, gpu_name);
                        }
                    }
                }
            }
        }
    }

    gpu_map
}
