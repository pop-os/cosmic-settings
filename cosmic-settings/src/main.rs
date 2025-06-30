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

use clap_lex::RawArgs;
use cosmic::{app::CosmicFlags, iced::Limits};
use i18n_embed::DesktopLanguageRequester;
use serde::{Deserialize, Serialize};
use tracing_subscriber::prelude::*;


pub struct Args {
    sub_command: Option<PageCommands>,
}

impl Args {
    pub fn parse() -> Self {
        let raw_args = RawArgs::from_args();
        let mut cursor = raw_args.cursor();

        // Ignore App name
        let _ = raw_args.next_os(&mut cursor);

        let sub_command = raw_args
            .next_os(&mut cursor)
            .and_then(|os_str| os_str.to_str())
            .and_then(|s| <PageCommands as FromStr>::from_str(s).ok());

        Args { sub_command }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageCommands {
    Accessibility,
    AccessibilityMagnifier,
    About,
    Appearance,
    Applications,
    Bluetooth,
    DateTime,
    DefaultApps,
    Desktop,
    Displays,
    Dock,
    Firmware,
    Input,
    Keyboard,
    LegacyApplications,
    Mouse,
    Network,
    Panel,
    Power,
    RegionLanguage,
    Sound,
    System,
    Time,
    Touchpad,
    Users,
    Vpn,
    Wallpaper,
    WindowManagement,
    Wired,
    Wireless,
    Workspaces,
}

impl PageCommands {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Accessibility" => Some(Self::Accessibility),
            "AccessibilityMagnifier" => Some(Self::AccessibilityMagnifier),
            "About" => Some(Self::About),
            "Appearance" => Some(Self::Appearance),
            "Applications" => Some(Self::Applications),
            "Bluetooth" => Some(Self::Bluetooth),
            "DateTime" => Some(Self::DateTime),
            "DefaultApps" => Some(Self::DefaultApps),
            "Desktop" => Some(Self::Desktop),
            "Displays" => Some(Self::Displays),
            "Dock" => Some(Self::Dock),
            "Firmware" => Some(Self::Firmware),
            "Input" => Some(Self::Input),
            "Keyboard" => Some(Self::Keyboard),
            "LegacyApplications" => Some(Self::LegacyApplications),
            "Mouse" => Some(Self::Mouse),
            "Network" => Some(Self::Network),
            "Panel" => Some(Self::Panel),
            "Power" => Some(Self::Power),
            "RegionLanguage" => Some(Self::RegionLanguage),
            "Sound" => Some(Self::Sound),
            "System" => Some(Self::System),
            "Time" => Some(Self::Time),
            "Touchpad" => Some(Self::Touchpad),
            "Users" => Some(Self::Users),
            "Vpn" => Some(Self::Vpn),
            "Wallpaper" => Some(Self::Wallpaper),
            "WindowManagement" => Some(Self::WindowManagement),
            "Wired" => Some(Self::Wired),
            "Wireless" => Some(Self::Wireless),
            "Workspaces" => Some(Self::Workspaces),
            _ => None,
        }
    }
}

impl FromStr for PageCommands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PageCommands::from_str(s).ok_or(())
    }
}

impl std::fmt::Display for PageCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
