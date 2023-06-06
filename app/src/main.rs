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
pub mod pages;
pub mod theme;
pub mod widget;

pub mod subscription;

use cosmic::{
    iced::{wayland::actions::window::SctkWindowSettings, Application, Limits},
    iced_sctk::settings::InitialSurface,
};
use i18n_embed::DesktopLanguageRequester;

/// # Errors
///
/// Returns error if iced fails to run the application.
pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    if std::env::var("RUST_SPANTRACE").is_err() {
        std::env::set_var("RUST_SPANTRACE", "0");
    }

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .without_time()
        .with_line_number(true)
        .with_file(true)
        .with_target(false)
        .with_thread_names(true)
        .init();

    let localizer = crate::localize::localizer();
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Err(why) = localizer.select(&requested_languages) {
        tracing::error!(%why, "error while loading fluent localizations");
    }

    cosmic::settings::set_default_icon_theme("Pop");
    let mut settings = cosmic::settings();
    settings.initial_surface = InitialSurface::XdgWindow(SctkWindowSettings {
        size_limits: Limits::NONE.min_width(600.0).min_height(300.0),
        ..Default::default()
    });

    SettingsApp::run(settings)?;

    Ok(())
}
