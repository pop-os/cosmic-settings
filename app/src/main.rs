// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_lossless)]

pub mod app;
pub use app::{Message, SettingsApp};

pub mod config;

#[macro_use]
pub mod localize;

pub mod widget;

pub mod pages;

use cosmic::iced::Application;
use i18n_embed::DesktopLanguageRequester;

/// # Errors
///
/// Returns error if iced fails to run the application.
pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    if std::env::var("RUST_SPANTRACE").is_err() {
        std::env::set_var("RUST_SPANTRACE", "0");
    }

    let localizer = crate::localize::localizer();
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Err(error) = localizer.select(&requested_languages) {
        eprintln!("error while loading fluent localizations: {error}");
    }

    cosmic::settings::set_default_icon_theme("Pop");
    let mut settings = cosmic::settings();
    settings.window.min_size = Some((600, 300));
    SettingsApp::run(settings)?;

    Ok(())
}
