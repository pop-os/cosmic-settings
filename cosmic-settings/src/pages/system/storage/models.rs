// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Data models for storage management

use super::utils::{FlatpakCache, get_directory_size, is_flatpak_available, parse_size_string};

#[derive(Clone, Debug, Default)]
pub struct FlatpakApp {
    pub name: String,
    pub app_id: String,
    pub installed_size: u64,
    pub data_size: u64,
    pub icon: String,
    pub version: String,
    pub developer: String,
    pub loading: bool, // true while sizes are being calculated
}

impl FlatpakApp {
    pub fn total_size(&self) -> u64 {
        self.installed_size + self.data_size
    }
}

#[derive(Clone, Debug, Default)]
pub struct SystemCategory {
    pub system_files: u64,     // /usr, /lib, etc.
    pub package_cache: u64,    // /var/cache/dnf, /var/cache/apt
    pub system_logs: u64,      // /var/log
    pub system_cache: u64,     // /var/cache (excluding package cache)
    pub boot_files: u64,       // /boot
    pub flatpak_runtimes: u64, // flatpak runtimes (user + system)
}

impl SystemCategory {
    pub fn total_size(&self) -> u64 {
        self.system_files
            + self.package_cache
            + self.system_logs
            + self.system_cache
            + self.boot_files
            + self.flatpak_runtimes
    }

    // Individual field loaders for streaming updates
    pub fn load_system_files() -> u64 {
        super::utils::get_rpm_package_size()
            .max(super::utils::get_dpkg_package_size())
            .max(
                get_directory_size("/usr")
                    + get_directory_size("/lib")
                    + get_directory_size("/lib64")
                    + get_directory_size("/opt"),
            )
    }

    pub fn load_boot_files() -> u64 {
        get_directory_size("/boot")
    }

    pub fn load_system_logs() -> u64 {
        get_directory_size("/var/log")
    }

    pub fn load_package_cache() -> u64 {
        get_directory_size("/var/cache/dnf")
            + get_directory_size("/var/cache/libdnf5")
            + get_directory_size("/var/cache/PackageKit")
            + get_directory_size("/var/cache/apt")
    }

    pub fn load_flatpak_runtimes() -> u64 {
        Self::get_flatpak_runtime_size()
    }

    pub fn load_system_cache() -> (u64, u64) {
        let total_cache = get_directory_size("/var/cache");
        let package_cache = Self::load_package_cache();
        (total_cache, package_cache)
    }

    pub fn load() -> Self {
        use jwalk::rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        let total_cache = get_directory_size("/var/cache");

        // Define tasks to run in parallel
        let tasks = [
            "boot_files",
            "system_logs",
            "package_cache",
            "flatpak_runtimes",
            "system_files",
        ];

        let results: Vec<(_, u64)> = tasks
            .par_iter()
            .map(|&task| {
                let size = match task {
                    "boot_files" => get_directory_size("/boot"),
                    "system_logs" => get_directory_size("/var/log"),
                    "package_cache" => {
                        get_directory_size("/var/cache/dnf")
                            + get_directory_size("/var/cache/libdnf5")
                            + get_directory_size("/var/cache/PackageKit")
                            + get_directory_size("/var/cache/apt")
                    }
                    "flatpak_runtimes" => Self::get_flatpak_runtime_size(),
                    "system_files" => super::utils::get_rpm_package_size()
                        .max(super::utils::get_dpkg_package_size())
                        .max(
                            get_directory_size("/usr")
                                + get_directory_size("/lib")
                                + get_directory_size("/lib64")
                                + get_directory_size("/opt"),
                        ),
                    _ => 0,
                };
                (task, size)
            })
            .collect();

        let mut cat = SystemCategory::default();
        for (task, size) in results {
            match task {
                "boot_files" => cat.boot_files = size,
                "system_logs" => cat.system_logs = size,
                "package_cache" => cat.package_cache = size,
                "flatpak_runtimes" => cat.flatpak_runtimes = size,
                "system_files" => cat.system_files = size,
                _ => {}
            }
        }

        cat.system_cache = total_cache.saturating_sub(cat.package_cache);

        cat
    }

    fn get_flatpak_runtime_size() -> u64 {
        use std::process::Command;

        if !is_flatpak_available() {
            return 0;
        }

        let Ok(output) = Command::new("flatpak")
            .args(["list", "--runtime", "--columns=ref,size"])
            .output()
        else {
            return 0;
        };

        let Ok(stdout) = String::from_utf8(output.stdout) else {
            return 0;
        };

        stdout
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let size_parts = &parts[1..];
                    if size_parts.len() >= 2 {
                        Some(parse_size_string(&format!(
                            "{} {}",
                            size_parts[0], size_parts[1]
                        )))
                    } else if size_parts.len() == 1 {
                        Some(parse_size_string(size_parts[0]))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum()
    }
}

#[derive(Clone, Debug, Default)]
pub struct HomeCategory {
    pub documents: u64,
    pub downloads: u64,
    pub pictures: u64,
    pub videos: u64,
    pub music: u64,
    pub desktop: u64,
    pub other: u64,
}

impl HomeCategory {
    pub fn total_size(&self) -> u64 {
        self.documents
            + self.downloads
            + self.pictures
            + self.videos
            + self.music
            + self.desktop
            + self.other
    }

    // Individual field loaders for streaming updates
    pub fn load_documents() -> u64 {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => return 0,
        };
        get_directory_size(&home_dir.join("Documents").to_string_lossy().to_string())
    }

    pub fn load_downloads() -> u64 {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => return 0,
        };
        get_directory_size(&home_dir.join("Downloads").to_string_lossy().to_string())
    }

    pub fn load_pictures() -> u64 {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => return 0,
        };
        get_directory_size(&home_dir.join("Pictures").to_string_lossy().to_string())
    }

    pub fn load_videos() -> u64 {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => return 0,
        };
        get_directory_size(&home_dir.join("Videos").to_string_lossy().to_string())
    }

    pub fn load_music() -> u64 {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => return 0,
        };
        get_directory_size(&home_dir.join("Music").to_string_lossy().to_string())
    }

    pub fn load_desktop() -> u64 {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => return 0,
        };
        get_directory_size(&home_dir.join("Desktop").to_string_lossy().to_string())
    }

    // Load total_home and var_dir (used to calculate "other" without re-scanning directories)
    pub fn load_total_and_var() -> (u64, u64) {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => return (0, 0),
        };

        let total_home = get_directory_size(&home_dir.to_string_lossy().to_string());
        let var_dir = get_directory_size(&home_dir.join(".var").to_string_lossy().to_string());

        (total_home, var_dir)
    }

    pub fn load() -> Self {
        use jwalk::rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => return HomeCategory::default(),
        };

        let total_home = get_directory_size(&home_dir.to_string_lossy().to_string());
        let var_dir = get_directory_size(&home_dir.join(".var").to_string_lossy().to_string());

        // Define directories to scan in parallel
        let dirs_to_scan = [
            ("Documents", home_dir.join("Documents")),
            ("Downloads", home_dir.join("Downloads")),
            ("Pictures", home_dir.join("Pictures")),
            ("Videos", home_dir.join("Videos")),
            ("Music", home_dir.join("Music")),
            ("Desktop", home_dir.join("Desktop")),
        ];

        let sizes: Vec<(_, u64)> = dirs_to_scan
            .par_iter()
            .map(|(name, path)| {
                (
                    *name,
                    get_directory_size(&path.to_string_lossy().to_string()),
                )
            })
            .collect();

        let mut cat = HomeCategory::default();
        for (name, size) in sizes {
            match name {
                "Documents" => cat.documents = size,
                "Downloads" => cat.downloads = size,
                "Pictures" => cat.pictures = size,
                "Videos" => cat.videos = size,
                "Music" => cat.music = size,
                "Desktop" => cat.desktop = size,
                _ => {}
            }
        }

        cat.other = total_home.saturating_sub(
            cat.documents
                + cat.downloads
                + cat.pictures
                + cat.videos
                + cat.music
                + cat.desktop
                + var_dir,
        );

        cat
    }
}

#[derive(Clone, Debug, Default)]
pub struct StorageInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub system: u64,
    pub home: u64,
    pub applications: u64,
    pub other: u64,
    pub flatpak_apps: Vec<FlatpakApp>,
}

impl StorageInfo {
    pub fn load() -> Self {
        let mut info = StorageInfo::default();

        let disks = sysinfo::Disks::new_with_refreshed_list();

        for disk in disks.list() {
            // Focus on the root filesystem
            if disk.mount_point().to_str() == Some("/") {
                info.total = disk.total_space();
                info.available = disk.available_space();
                info.used = info.total.saturating_sub(info.available);
                break;
            }
        }

        // If we didn't find root, use the first disk
        if info.total == 0 {
            if let Some(disk) = disks.list().first() {
                info.total = disk.total_space();
                info.available = disk.available_space();
                info.used = info.total.saturating_sub(info.available);
            }
        }

        // Quick load Flatpak apps (without sizes for immediate display)
        info.flatpak_apps = Self::load_flatpak_apps_quick();

        // Don't calculate category sizes here - will be done in background
        // This allows the Flatpak app list to show immediately
        info.system = 0;
        info.home = 0;
        info.applications = 0;
        info.other = 0;

        info
    }

    pub fn load_category_details() -> (SystemCategory, HomeCategory) {
        let system_category = SystemCategory::load();
        let home_category = HomeCategory::load();

        (system_category, home_category)
    }

    pub fn load_flatpak_apps_quick() -> Vec<FlatpakApp> {
        use std::process::Command;

        let mut apps = Vec::new();

        if !is_flatpak_available() {
            return apps;
        }

        let Ok(output) = Command::new("flatpak")
            .args(["list", "--app", "--columns=application,name"])
            .output()
        else {
            return apps;
        };

        let Ok(stdout) = String::from_utf8(output.stdout) else {
            return apps;
        };

        let desktop_entries = Self::get_desktop_entries();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                let app_id = parts[0].to_string();
                let name = parts[1].to_string();

                let icon = desktop_entries
                    .iter()
                    .find(|(id, _, _)| id == &app_id)
                    .map(|(_, _, icon)| icon.clone())
                    .unwrap_or_else(|| "application-default".to_string());

                apps.push(FlatpakApp {
                    name,
                    app_id,
                    installed_size: 0,
                    data_size: 0,
                    icon,
                    version: String::new(),
                    developer: String::new(),
                    loading: true, // Sizes not yet calculated
                });
            }
        }

        let cache = FlatpakCache::load();
        apps.sort_by(|a, b| {
            let pos_a = cache.get_position(&a.app_id).unwrap_or(usize::MAX);
            let pos_b = cache.get_position(&b.app_id).unwrap_or(usize::MAX);
            pos_a.cmp(&pos_b)
        });

        apps
    }

    pub fn load_flatpak_apps_with_sizes(apps: Vec<FlatpakApp>) -> Vec<FlatpakApp> {
        use jwalk::rayon::iter::{IntoParallelIterator, ParallelIterator};

        let desktop_entries = Self::get_desktop_entries();

        let mut apps_with_sizes: Vec<FlatpakApp> = apps
            .into_par_iter()
            .map(|mut app| {
                let (installed_size, version, developer) = Self::get_flatpak_app_info(&app.app_id);
                app.installed_size = installed_size;
                app.version = version;
                app.developer = developer;

                app.data_size = Self::get_flatpak_data_size(&app.app_id);

                if app.icon.is_empty() || app.icon == "application-default" {
                    app.icon = desktop_entries
                        .iter()
                        .find(|(id, _, _)| id == &app.app_id)
                        .map(|(_, _, icon)| icon.clone())
                        .unwrap_or_else(|| "application-default".to_string());
                }

                app.loading = false;
                app
            })
            .collect();

        apps_with_sizes.sort_by(|a, b| b.total_size().cmp(&a.total_size()));

        let cache = FlatpakCache {
            app_order: apps_with_sizes
                .iter()
                .map(|app| app.app_id.clone())
                .collect(),
        };
        cache.save();

        apps_with_sizes
    }

    fn get_desktop_entries() -> Vec<(String, String, String)> {
        use freedesktop_desktop_entry::{Iter, default_paths};

        let mut entries = Vec::new();

        let locales = std::env::var("LANG")
            .ok()
            .and_then(|lang| lang.split('.').next().map(String::from))
            .into_iter()
            .collect::<Vec<_>>();

        for entry in Iter::new(default_paths()).entries(Some(&locales)) {
            let app_id = entry.appid.to_string();
            let name = entry.name(&locales).unwrap_or_default().to_string();
            let icon = entry.icon().unwrap_or("application-default").to_string();

            entries.push((app_id, name, icon));
        }

        entries
    }

    fn get_flatpak_app_info(app_id: &str) -> (u64, String, String) {
        use std::process::Command;

        let Ok(output) = Command::new("flatpak").args(["info", app_id]).output() else {
            return (0, String::new(), String::new());
        };

        let Ok(stdout) = String::from_utf8(output.stdout) else {
            return (0, String::new(), String::new());
        };

        let mut installed_size = 0u64;
        let mut version = String::new();
        let mut developer = String::new();

        for line in stdout.lines() {
            let line = line.trim();

            if line.starts_with("Installed:") {
                // Parse the size from the line
                // Format: "Installed: 123.4 MB" or "   Installed: 441,6 MB"
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    let size_str = parts[1].trim();
                    installed_size = parse_size_string(size_str);
                }
            } else if line.starts_with("Version:") {
                if let Some(v) = line.split(':').nth(1) {
                    version = v.trim().to_string();
                }
            } else if line.starts_with("Subject:") {
                if let Some(subject) = line.split(':').nth(1) {
                    let subject = subject.trim();
                    if let Some(by_pos) = subject.find(" by ") {
                        developer = subject[by_pos + 4..].trim().to_string();
                    }
                }
            } else if line.starts_with("Origin:") && developer.is_empty() {
                if let Some(origin) = line.split(':').nth(1) {
                    let origin = origin.trim();
                    if origin != "flathub" && !origin.is_empty() {
                        developer = origin.to_string();
                    }
                }
            }
        }

        if developer.is_empty() {
            developer = app_id
                .split('.')
                .nth(1)
                .map(|s| {
                    s.chars()
                        .next()
                        .map(|c| c.to_uppercase().to_string() + &s[1..])
                        .unwrap_or_default()
                })
                .unwrap_or_default();
        }

        (installed_size, version, developer)
    }

    fn get_flatpak_data_size(app_id: &str) -> u64 {
        // Flatpak apps store their data in ~/.var/app/APP_ID
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => return 0,
        };

        let data_path = home_dir.join(".var/app").join(app_id);

        if !data_path.exists() {
            return 0;
        }

        get_directory_size(data_path.to_str().unwrap_or(""))
    }
}
