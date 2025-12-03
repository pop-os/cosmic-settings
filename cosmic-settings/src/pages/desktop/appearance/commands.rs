// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! CLI commands for theme import and export.

use cosmic::cosmic_config::CosmicConfigEntry;
use cosmic::cosmic_theme::{Theme, ThemeBuilder, ThemeMode};
use std::path::Path;

/// Import a theme from a RON file and apply it.
///
/// # Errors
///
/// Returns an error if the theme file cannot be read, parsed, or applied.
pub fn import_theme(path: &Path) -> color_eyre::Result<()> {
    if !path.exists() {
        return Err(color_eyre::eyre::eyre!(
            "Theme file not found: {}",
            path.display()
        ));
    }

    if path.extension().is_none_or(|ext| ext != "ron") {
        return Err(color_eyre::eyre::eyre!(
            "Theme file must have .ron extension: {}",
            path.display()
        ));
    }

    let content = std::fs::read_to_string(path).map_err(|e| {
        color_eyre::eyre::eyre!("Failed to read theme file '{}': {}", path.display(), e)
    })?;

    let builder: ThemeBuilder = ron::de::from_str(&content).map_err(|e| {
        color_eyre::eyre::eyre!("Failed to parse theme file '{}': {}", path.display(), e)
    })?;

    let is_dark = builder.palette.is_dark();
    let mode_str = if is_dark { "dark" } else { "light" };

    if let Ok(mode_config) = ThemeMode::config() {
        let mut mode = ThemeMode::get_entry(&mode_config).unwrap_or_default();
        if mode.is_dark != is_dark {
            let _ = mode.set_is_dark(&mode_config, is_dark);
        }
    }

    let (theme_config, builder_config) = if is_dark {
        (Theme::dark_config(), ThemeBuilder::dark_config())
    } else {
        (Theme::light_config(), ThemeBuilder::light_config())
    };

    if let Ok(config) = builder_config {
        builder.write_entry(&config).map_err(|e| {
            color_eyre::eyre::eyre!("Failed to write theme builder config: {:?}", e)
        })?;
    } else {
        return Err(color_eyre::eyre::eyre!(
            "Failed to get {} theme builder config",
            mode_str
        ));
    }

    let theme = builder.build();
    if let Ok(config) = theme_config {
        theme
            .write_entry(&config)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to write theme config: {:?}", e))?;
    } else {
        return Err(color_eyre::eyre::eyre!(
            "Failed to get {} theme config",
            mode_str
        ));
    }

    println!(
        "Successfully imported {} theme from: {}",
        mode_str,
        path.display()
    );

    Ok(())
}

/// Export the current theme to a RON file.
///
/// # Errors
///
/// Returns an error if the theme cannot be read or the file cannot be written.
pub fn export_theme(path: &Path) -> color_eyre::Result<()> {
    if path.extension().is_none_or(|ext| ext != "ron") {
        return Err(color_eyre::eyre::eyre!(
            "Theme file must have .ron extension: {}",
            path.display()
        ));
    }

    let mode_config = ThemeMode::config()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to get theme mode config: {:?}", e))?;

    let mode = ThemeMode::get_entry(&mode_config).unwrap_or_default();
    let is_dark = mode.is_dark;
    let mode_str = if is_dark { "dark" } else { "light" };

    let builder_config = if is_dark {
        ThemeBuilder::dark_config()
    } else {
        ThemeBuilder::light_config()
    }
    .map_err(|e| {
        color_eyre::eyre::eyre!("Failed to get {} theme builder config: {:?}", mode_str, e)
    })?;

    let builder = ThemeBuilder::get_entry(&builder_config).map_err(|e| {
        color_eyre::eyre::eyre!("Failed to get {} theme builder: {:?}", mode_str, e)
    })?;

    let ron_string = ron::ser::to_string_pretty(&builder, Default::default())
        .map_err(|e| color_eyre::eyre::eyre!("Failed to serialize theme to RON: {}", e))?;

    std::fs::write(path, ron_string).map_err(|e| {
        color_eyre::eyre::eyre!("Failed to write theme file '{}': {}", path.display(), e)
    })?;

    println!(
        "Successfully exported {} theme to: {}",
        mode_str,
        path.display()
    );

    Ok(())
}
