// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::too_many_lines)]

pub mod app;
use std::str::FromStr;

pub use app::{Message, SettingsApp};
pub mod config;

#[macro_use]
pub mod localize;
pub mod pages;
pub mod subscription;
pub mod theme;
pub mod utils;
pub mod widget;

use clap::{Parser, Subcommand};
use cosmic::{app::CosmicFlags, iced::Limits};
use i18n_embed::DesktopLanguageRequester;
use ron::error::SpannedError;
use serde::{Deserialize, Serialize};
use tracing_subscriber::prelude::*;

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    sub_command: Option<PageCommands>,
}

#[derive(Subcommand, Debug, Serialize, Deserialize, Clone)]
pub enum PageCommands {
    /// About settings page
    About,
    /// Appearance settings page
    Appearance,
    /// Bluetooth settings page
    Bluetooth,
    /// Date & Time settings page
    DateTime,
    /// Desktop settings page
    Desktop,
    /// Displays settings page
    Displays,
    /// Dock settings page
    Dock,
    /// Firmware settings page
    Firmware,
    /// Input Devices settings page
    Input,
    /// Keyboard settings page
    Keyboard,
    /// Mouse settings page
    Mouse,
    /// Network settings page
    Network,
    /// Panel settings page
    Panel,
    /// Power settings page
    Power,
    /// Region & Language settings page
    RegionLanguage,
    /// Sound settings page
    Sound,
    /// System & Accounts settings page
    System,
    /// Time & Language settings page
    Time,
    /// Touchpad settings page
    Touchpad,
    /// Users settings page
    Users,
    /// VPN settings page
    Vpn,
    /// Wallpaper settings page
    Wallpaper,
    /// Window management settings page
    WindowManagement,
    /// Wired settings page
    Wired,
    /// WiFi settings page
    Wireless,
    /// Workspaces settings page
    Workspaces,
}

impl FromStr for PageCommands {
    type Err = SpannedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ron::de::from_str(s)
    }
}

impl ToString for PageCommands {
    fn to_string(&self) -> String {
        ron::ser::to_string(self).unwrap()
    }
}

impl CosmicFlags for Args {
    type SubCommand = PageCommands;
    type Args = Vec<String>;

    fn action(&self) -> Option<&PageCommands> {
        self.sub_command.as_ref()
    }
}

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

    let args = Args::parse();

    let settings = cosmic::app::Settings::default()
        .size_limits(Limits::NONE.min_width(360.0).min_height(300.0));

    cosmic::app::run_single_instance::<app::SettingsApp>(settings, args)?;

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

#[macro_export]
macro_rules! cache_dynamic_lazy {
    ( $( $visible:vis static $variable:ident: $type:ty = $expression:expr; )+ ) => {
        $(
            #[static_init::dynamic(lazy)]
            $visible static $variable: $type = $expression;
        )+
    };
}
