// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! CLI commands for theme import and export.

use cosmic::cosmic_theme::ThemeBuilder;
use std::path::Path;

use super::theme_manager::Manager;

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

    let mut manager = Manager::default();

    if manager.mode().is_dark != is_dark {
        if let Err(err) = manager.dark_mode(is_dark) {
            return Err(color_eyre::eyre::eyre!(
                "Failed to set {} mode: {:?}",
                mode_str,
                err
            ));
        }
    }

    manager
        .selected_customizer_mut()
        .set_builder(builder.clone())
        .set_theme(builder.build())
        .apply_builder()
        .apply_theme();

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

    let manager = Manager::default();
    let is_dark = manager.mode().is_dark;
    let mode_str = if is_dark { "dark" } else { "light" };

    let builder = manager.builder();

    let ron_string = ron::ser::to_string_pretty(builder, Default::default())
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
