// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::cosmic_config::{self, ConfigGet, ConfigSet};
use cosmic_settings_wallpaper as wallpaper;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

const NAME: &str = "com.system76.CosmicSettings.Wallpaper";
const VERSION: u64 = 1;
const CURRENT_FOLDER: &str = "current-folder";
const CUSTOM_COLORS: &str = "custom-colors";
const CUSTOM_IMAGES: &str = "custom-images";
const RECENT_FOLDERS: &str = "recent-folders";

#[derive(Debug, Default)]
pub struct Config {
    context: Option<cosmic_config::Config>,
    pub(super) current_folder: Option<PathBuf>,
    custom_colors: Vec<wallpaper::Color>,
    custom_images: Vec<PathBuf>,
    recent_folders: VecDeque<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Self::default();

        let context = match cosmic_config::Config::new(NAME, VERSION) {
            Ok(context) => context,
            Err(why) => {
                tracing::warn!(?why, "failed to get config");
                return Self::default();
            }
        };

        // Get the active background folder from cosmic-config.
        if let Ok(path) = context.get::<Option<PathBuf>>(CURRENT_FOLDER) {
            // Set current folder if it exists.
            config.current_folder = path.filter(|path| path.exists());
        }

        // Get custom colors stored in cosmic-config.
        if let Ok(colors) = context.get::<Vec<wallpaper::Color>>(CUSTOM_COLORS) {
            config.custom_colors = colors;
        }

        // Get custom background images stored in cosmic-config.
        if let Ok(images) = context.get::<Vec<PathBuf>>(CUSTOM_IMAGES) {
            // Update config if images are missing.
            let mut update_config = false;

            for image in images {
                if image.exists() {
                    config.custom_images.push(image);
                } else {
                    update_config = true;
                }
            }

            if update_config {
                let _res = config.update_custom_images();
            }
        }

        // Get recently-added background folders from cosmic-config.
        if let Ok(folders) = context.get::<VecDeque<PathBuf>>(RECENT_FOLDERS) {
            // Update config if folders are missing
            let mut update_config = false;

            for folder in folders {
                if folder.exists() {
                    config.recent_folders.push_back(folder);
                } else {
                    update_config = true;
                }
            }

            if update_config {
                let _res = config.update_recent_folders();
            }
        }

        config.context = Some(context);

        config
    }

    #[must_use]
    pub fn current_folder(&self) -> &Path {
        self.current_folder
            .as_deref()
            .unwrap_or(Self::default_folder())
    }

    #[must_use]
    pub fn default_folder() -> &'static Path {
        Path::new("/usr/share/backgrounds/")
    }

    /// Sets the current background folder
    ///
    /// # Errors
    ///
    /// Returns an error if the on-disk configuration could not be updated.
    pub fn set_current_folder(
        &mut self,
        folder: Option<PathBuf>,
    ) -> Result<(), cosmic_config::Error> {
        let result = self.update(CURRENT_FOLDER, &folder);
        self.current_folder = folder;
        result
    }

    #[must_use]
    pub fn custom_colors(&self) -> &[wallpaper::Color] {
        &self.custom_colors
    }

    /// Adds a custom color
    ///
    /// # Errors
    ///
    /// Returns an error if the on-disk configuration could not be updated.
    pub fn add_custom_color(
        &mut self,
        color: wallpaper::Color,
    ) -> Result<(), cosmic_config::Error> {
        if !self.custom_colors.contains(&color) {
            self.custom_colors.push(color);
            return self.update_custom_colors();
        }

        Ok(())
    }

    /// Removes custom background colors.
    ///
    /// # Errors
    ///
    /// Returns an error if the on-disk configuration could not be updated.
    pub fn remove_custom_color(
        &mut self,
        color: &wallpaper::Color,
    ) -> Result<(), cosmic_config::Error> {
        if let Some(position) = self.custom_colors.iter().position(|c| c == color) {
            self.custom_colors.remove(position);
            return self.update_custom_colors();
        }

        Ok(())
    }

    #[must_use]
    pub fn custom_images(&self) -> &[PathBuf] {
        &self.custom_images
    }

    /// Adds a custom background image
    ///
    /// # Errors
    ///
    /// Returns an error if the on-disk configuration could not be updated.
    pub fn add_custom_image(&mut self, image: PathBuf) -> Result<(), cosmic_config::Error> {
        if !self.custom_images.contains(&image) {
            self.custom_images.push(image);
            return self.update_custom_images();
        }

        Ok(())
    }

    /// Removes custom background images.
    ///
    /// # Errors
    ///
    /// Returns an error if the on-disk configuration could not be updated.
    pub fn remove_custom_image(&mut self, image: &Path) -> Result<(), cosmic_config::Error> {
        if let Some(position) = self.custom_images.iter().position(|p| p == image) {
            self.custom_images.remove(position);
            return self.update_custom_images();
        }

        Ok(())
    }

    #[must_use]
    pub fn recent_folders(&self) -> &VecDeque<PathBuf> {
        &self.recent_folders
    }

    /// Adds a folder to the recent folders list
    ///
    /// # Errors
    ///
    /// Returns an error if the on-disk configuration could not be updated.
    pub fn add_recent_folder(&mut self, folder: PathBuf) -> Result<(), cosmic_config::Error> {
        while self.recent_folders.len() > 4 {
            self.recent_folders.pop_front();
        }

        if !self.recent_folders.contains(&folder) {
            self.recent_folders.push_back(folder);
            return self.update_recent_folders();
        }

        Ok(())
    }

    fn update<V: serde::Serialize>(
        &self,
        key: &str,
        value: &V,
    ) -> Result<(), cosmic_config::Error> {
        if let Some(context) = self.context.as_ref() {
            context.set(key, value)?;
        }

        Ok(())
    }

    fn update_custom_colors(&self) -> Result<(), cosmic_config::Error> {
        self.update(CUSTOM_COLORS, &self.custom_colors)
    }

    fn update_custom_images(&self) -> Result<(), cosmic_config::Error> {
        self.update(CUSTOM_IMAGES, &self.custom_images)
    }

    fn update_recent_folders(&self) -> Result<(), cosmic_config::Error> {
        self.update(RECENT_FOLDERS, &self.recent_folders)
    }
}
