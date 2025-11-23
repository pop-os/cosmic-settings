// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Common utilities for storage management

use cosmic::iced::Color;
use std::process::Command;
use std::sync::OnceLock;

use super::CategoryType;

// Cache for command availability checks
static FLATPAK_AVAILABLE: OnceLock<bool> = OnceLock::new();
static BTRFS_AVAILABLE: OnceLock<bool> = OnceLock::new();

/// Check if a command is available in PATH
fn is_command_available(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Check if flatpak is available (cached)
pub fn is_flatpak_available() -> bool {
    *FLATPAK_AVAILABLE.get_or_init(|| is_command_available("flatpak"))
}

/// Check if btrfs is available (cached)
pub fn is_btrfs_available() -> bool {
    *BTRFS_AVAILABLE.get_or_init(|| is_command_available("btrfs"))
}

// Category color constants
pub const COLOR_SYSTEM: Color = Color::from_rgb(0.40, 0.62, 0.93); // Blue
pub const COLOR_HOME: Color = Color::from_rgb(0.95, 0.61, 0.07); // Orange
pub const COLOR_APPLICATIONS: Color = Color::from_rgb(0.45, 0.82, 0.46); // Green
pub const COLOR_OTHER: Color = Color::from_rgb(0.70, 0.70, 0.70); // Gray
pub const COLOR_AVAILABLE: Color = Color::from_rgb(0.2, 0.2, 0.2); // Dark gray

pub fn category_color(category: &CategoryType) -> Color {
    match category {
        CategoryType::System => COLOR_SYSTEM,
        CategoryType::Home => COLOR_HOME,
        CategoryType::Applications => COLOR_APPLICATIONS,
        CategoryType::Other => COLOR_OTHER,
    }
}

/// Create a loading spinner indicator with animation
pub fn loading_spinner<'a, Message: 'a + 'static>(
    animation_state: u8,
) -> cosmic::Element<'a, Message> {
    use cosmic::widget::text;
    // Animate through ., .., ...
    let dots = match animation_state % 3 {
        0 => ".",
        1 => "..",
        _ => "...",
    };
    text::body(dots).into()
}

/// Create a settings item that shows either a loading spinner or formatted size
pub fn loading_or_size_item<'a, Message: 'a + 'static + Clone>(
    label: &'a str,
    size: u64,
    loading: bool,
    animation_state: u8,
) -> cosmic::Element<'a, Message> {
    use cosmic::widget::settings;

    if loading {
        settings::flex_item(label, loading_spinner(animation_state)).into()
    } else {
        settings::flex_item(label, cosmic::widget::text::body(format_bytes(size))).into()
    }
}

/// Format bytes into human-readable string using IEC binary prefixes (e.g., "1.5 GiB")
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    // Use 0 decimal places for bytes, 1 for everything else
    if unit_index == 0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Parse a size string like "1.5 GB" or "1.5 GiB" into bytes
pub fn parse_size_string(size_str: &str) -> u64 {
    let parts: Vec<&str> = size_str.split_whitespace().collect();
    if parts.len() < 2 {
        return 0;
    }

    let Ok(number) = parts[0].replace(',', ".").parse::<f64>() else {
        return 0;
    };

    let multiplier = match parts[1] {
        "B" | "bytes" => 1.0,
        // Binary (IEC) units
        "KiB" | "kB" | "KB" => 1024.0,
        "MiB" | "MB" => 1024.0 * 1024.0,
        "GiB" | "GB" => 1024.0 * 1024.0 * 1024.0,
        "TiB" | "TB" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => 1.0,
    };

    (number * multiplier) as u64
}

/// Detect filesystem type for a given path
pub fn get_filesystem_type(path: &str) -> Option<String> {
    let output = Command::new("stat")
        .args(["-f", "-c", "%T", path])
        .output()
        .ok()?;

    String::from_utf8(output.stdout)
        .ok()
        .map(|s| s.trim().to_lowercase())
}

/// Calculate directory size with optimizations for different filesystems
pub fn get_directory_size(path: &str) -> u64 {
    use jwalk::WalkDir;

    // Use btrfs optimization if available and filesystem is btrfs
    if is_btrfs_available() {
        if let Some(fs_type) = get_filesystem_type(path) {
            if fs_type == "btrfs" {
                if let Ok(output) = Command::new("btrfs")
                    .args(["filesystem", "du", "-s", "--raw", path])
                    .output()
                {
                    if let Ok(stdout) = String::from_utf8(output.stdout) {
                        if let Some(line) = stdout.lines().nth(1) {
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            if !parts.is_empty() {
                                if let Ok(size) = parts[0].parse::<u64>() {
                                    return size;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Use jwalk for parallel directory traversal (4-8x faster than sequential)
    // Utilize available CPU cores (capped at 8 for reasonable memory usage)
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get().min(8))
        .unwrap_or(4);

    WalkDir::new(path)
        .skip_hidden(false) // Include hidden files
        .parallelism(jwalk::Parallelism::RayonNewPool(num_threads))
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            // Use file_type() - NO extra syscall (uses data from getdents)
            // This is 10-100x faster than calling metadata()
            entry.file_type().is_file()
        })
        .filter_map(|entry| {
            // Only call metadata() for files to get size
            entry.metadata().ok()
        })
        .map(|metadata| metadata.len())
        .sum()
}

/// Get RPM package total size (Fedora/RHEL)
pub fn get_rpm_package_size() -> u64 {
    // Use rpm directly with proper quoting
    let Ok(output) = Command::new("rpm")
        .args(["-qa", "--queryformat", "%{size}\n"])
        .output()
    else {
        return 0;
    };

    if let Ok(stdout) = String::from_utf8(output.stdout) {
        stdout
            .lines()
            .filter_map(|line| line.trim().parse::<u64>().ok())
            .sum()
    } else {
        0
    }
}

/// Get dpkg package total size (Debian/Ubuntu)
pub fn get_dpkg_package_size() -> u64 {
    // Use dpkg-query directly
    let Ok(output) = Command::new("dpkg-query")
        .args(["-W", "-f=${Installed-Size}\n"])
        .output()
    else {
        return 0;
    };

    if let Ok(stdout) = String::from_utf8(output.stdout) {
        // dpkg returns size in KB, convert to bytes
        stdout
            .lines()
            .filter_map(|line| line.trim().parse::<u64>().ok())
            .sum::<u64>()
            * 1024
    } else {
        0
    }
}

/// Cache structure for storing app order
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FlatpakCache {
    /// Maps app_id to its previous position (lower = higher priority/larger size)
    pub app_order: Vec<String>,
}

impl FlatpakCache {
    fn cache_path() -> Option<std::path::PathBuf> {
        dirs::cache_dir().map(|dir| dir.join("cosmic-settings").join("flatpak-order.ron"))
    }

    pub fn load() -> Self {
        let Some(path) = Self::cache_path() else {
            return Self::default();
        };

        if !path.exists() {
            return Self::default();
        }

        match std::fs::read_to_string(&path) {
            Ok(contents) => match ron::from_str(&contents) {
                Ok(cache) => cache,
                Err(_) => Self::default(),
            },
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) {
        let Some(path) = Self::cache_path() else {
            return;
        };

        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if let Ok(contents) = ron::to_string(self) {
            let _ = std::fs::write(&path, contents);
        }
    }

    pub fn get_position(&self, app_id: &str) -> Option<usize> {
        self.app_order.iter().position(|id| id == app_id)
    }
}
