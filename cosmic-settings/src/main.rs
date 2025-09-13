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
    /// Accessibility settings page
    #[cfg(feature = "page-accessibility")]
    Accessibility,
    /// Accessibility Magnifier settings page
    #[cfg(feature = "page-accessibility")]
    AccessibilityMagnifier,
    /// About settings page
    #[cfg(feature = "page-about")]
    About,
    /// Appearance settings page
    Appearance,
    /// Applications settings page
    Applications,
    /// Bluetooth settings page
    #[cfg(feature = "page-bluetooth")]
    Bluetooth,
    /// Date & Time settings page
    #[cfg(feature = "page-date")]
    DateTime,
    /// Default application associations
    #[cfg(feature = "page-default-apps")]
    DefaultApps,
    /// Desktop settings page
    Desktop,
    /// Displays settings page
    #[cfg(feature = "page-display")]
    Displays,
    /// Dock settings page
    #[cfg(feature = "wayland")]
    Dock,
    /// Dock applets page
    #[cfg(feature = "wayland")]
    DockApplet,
    /// Firmware settings page
    Firmware,
    /// Input Devices settings page
    #[cfg(feature = "page-input")]
    Input,
    /// Keyboard settings page
    #[cfg(feature = "page-input")]
    Keyboard,
    /// Legacy Applications settings page
    #[cfg(feature = "page-legacy-applications")]
    LegacyApplications,
    /// Mouse settings page
    #[cfg(feature = "page-input")]
    Mouse,
    /// Network settings page
    #[cfg(feature = "page-networking")]
    Network,
    /// Panel settings page
    #[cfg(feature = "wayland")]
    Panel,
    /// Panel applets page
    #[cfg(feature = "wayland")]
    PanelApplet,
    /// Power settings page
    #[cfg(feature = "page-power")]
    Power,
    /// Region & Language settings page
    #[cfg(feature = "page-region")]
    RegionLanguage,
    /// Sound settings page
    #[cfg(feature = "page-sound")]
    Sound,
    /// Startup applications settings page
    StartupApps,
    /// System & Accounts settings page
    System,
    /// Time & Language settings page
    Time,
    /// Touchpad settings page
    #[cfg(feature = "page-input")]
    Touchpad,
    /// Users settings page
    #[cfg(feature = "page-users")]
    Users,
    /// VPN settings page
    #[cfg(feature = "page-networking")]
    Vpn,
    /// Wallpaper settings page
    Wallpaper,
    /// Window management settings page
    #[cfg(feature = "page-window-management")]
    WindowManagement,
    /// Wired settings page
    #[cfg(feature = "page-networking")]
    Wired,
    /// WiFi settings page
    #[cfg(feature = "page-networking")]
    Wireless,
    /// Workspaces settings page
    #[cfg(feature = "page-workspaces")]
    Workspaces,
}

impl FromStr for PageCommands {
    type Err = SpannedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ron::de::from_str(s)
    }
}

impl std::fmt::Display for PageCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ron::ser::to_string(self).unwrap())
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
        unsafe { std::env::set_var("RUST_SPANTRACE", "0") };
    }

    init_logger();
    init_localizer();

    #[cfg(feature = "gettext")]
    {
        let _ = gettextrs::setlocale(gettextrs::LocaleCategory::LcAll, "");
    }

    let args = Args::parse();

    let settings = cosmic::app::Settings::default()
        .size_limits(Limits::NONE.min_width(360.0).min_height(300.0));

    #[cfg(feature = "single-instance")]
    {
        cosmic::app::run_single_instance::<app::SettingsApp>(settings, args)?;
    }
    #[cfg(not(feature = "single-instance"))]
    {
        cosmic::app::run::<app::SettingsApp>(settings, args)?;
    }
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
