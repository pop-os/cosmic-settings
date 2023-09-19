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
    iced::{wayland::actions::window::SctkWindowSettings, Limits},
    iced_sctk::settings::InitialSurface,
};
use i18n_embed::DesktopLanguageRequester;
use tracing_subscriber::prelude::*;

/// # Errors
///
/// Returns error if iced fails to run the application.
pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    if std::env::var("RUST_SPANTRACE").is_err() {
        std::env::set_var("RUST_SPANTRACE", "0");
    }

    init_logger();
    init_localizer();

    let settings = cosmic::app::Settings::default()
        .size_limits(Limits::NONE.min_width(400.0).min_height(300.0));

    cosmic::app::run::<app::SettingsApp>(settings, ())?;

    Ok(())
}

fn init_localizer() {
    let localizer = crate::localize::localizer();
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Err(why) = localizer.select(&requested_languages) {
        tracing::error!(%why, "error while loading fluent localizations");
    }
}

fn init_logger() {
    let log_level = std::env::var("RUST_LOG")
        .ok()
        .and_then(|level| level.parse::<tracing::Level>().ok())
        .unwrap_or(tracing::Level::INFO);

    let log_format = tracing_subscriber::fmt::format()
        .pretty()
        .without_time()
        .with_line_number(true)
        .with_file(true)
        .with_target(false)
        .with_thread_names(true);

    let log_filter = tracing_subscriber::fmt::Layer::default()
        .with_writer(std::io::stderr)
        .event_format(log_format)
        .with_filter(tracing_subscriber::filter::filter_fn(move |metadata| {
            let target = metadata.target();
            metadata.level() == &tracing::Level::ERROR
                || ((target.starts_with("cosmic_settings") || target.starts_with("cosmic_bg"))
                    && metadata.level() <= &log_level)
        }));

    tracing_subscriber::registry().with(log_filter).init();
}
