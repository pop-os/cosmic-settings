#![warn(missing_debug_implementations, rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::{fs, io};

use cosmic_config::{Config, ConfigGet, ConfigSet, CosmicConfigEntry};
use cosmic_theme::{
    DARK_THEME_BUILDER_ID, DARK_THEME_ID, LIGHT_THEME_BUILDER_ID, LIGHT_THEME_ID, Theme,
    ThemeBuilder, ThemeMode,
};

pub mod bundle;
pub mod transfer;

pub const MODE_ID: &str = "com.system76.CosmicTheme.Mode";

/// Prefix for all theme bundle config IDs.
pub const NAME: &str = "com.system76.CosmicThemeBundle";

/// Version of the live background config.
pub const BACKGROUND_VERSION: u64 = cosmic_bg_config::VERSION;

/// The theme mode a bundle applies to.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Mode {
    /// Dark mode.
    Dark,
    /// Light mode.
    Light,
}

impl Mode {
    /// Every mode.
    pub const ALL: [Self; 2] = [Self::Dark, Self::Light];

    /// The theme config ID for this mode.
    pub const fn theme_id(self) -> &'static str {
        match self {
            Self::Dark => DARK_THEME_ID,
            Self::Light => LIGHT_THEME_ID,
        }
    }

    /// The theme builder config ID for this mode.
    pub const fn builder_id(self) -> &'static str {
        match self {
            Self::Dark => DARK_THEME_BUILDER_ID,
            Self::Light => LIGHT_THEME_BUILDER_ID,
        }
    }

    /// The config key for this mode.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }

    /// Whether this mode is dark.
    pub const fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }

    /// The mode for a dark / light flag.
    pub const fn from_is_dark(is_dark: bool) -> Self {
        if is_dark { Self::Dark } else { Self::Light }
    }
}

/// A part of a theme bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Part {
    /// The theme.
    Theme,
    /// The theme builder.
    Builder,
    /// The background.
    Background,
}

impl Part {
    /// Every bundle part.
    pub const ALL: [Self; 3] = [Self::Theme, Self::Builder, Self::Background];

    /// The live config ID for this part.
    pub const fn live_id(self, mode: Mode) -> &'static str {
        match self {
            Self::Theme => mode.theme_id(),
            Self::Builder => mode.builder_id(),
            Self::Background => background_id(mode),
        }
    }

    /// The config version for this part.
    pub const fn version(self) -> u64 {
        match self {
            Self::Theme => <Theme as CosmicConfigEntry>::VERSION,
            Self::Builder => <ThemeBuilder as CosmicConfigEntry>::VERSION,
            Self::Background => BACKGROUND_VERSION,
        }
    }

    /// The config ID suffix for this part.
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Theme => "Theme",
            Self::Builder => "Builder",
            Self::Background => "Background",
        }
    }
}

/// The live background config ID for a mode.
pub const fn background_id(mode: Mode) -> &'static str {
    match mode {
        Mode::Dark => "com.system76.CosmicBackground.Dark",
        Mode::Light => "com.system76.CosmicBackground.Light",
    }
}

/// The config ID for `part` of the bundle with `slug`.
pub fn bundle_id(slug: &str, part: Part) -> String {
    format!("{NAME}.{slug}.{}", part.suffix())
}

/// The directory that holds the cosmic config entries.
pub fn config_dir() -> Option<PathBuf> {
    cosmic_config::get_config_dir().map(|dir| dir.join("cosmic"))
}

const TOOLKIT_ID: &str = "com.system76.CosmicTk";
const TOOLKIT_MIGRATION: &str = "theme-bundle-toolkit-migrated";
const THEME_NAMES_MIGRATION: &str = "theme-bundle-names-migrated";
const LEGACY_THEME_NAMES: [(&str, &str); 2] = [
    ("cosmic-dark", "COSMIC Dark"),
    ("cosmic-light", "COSMIC Light"),
];
const BASE: &str = "base";
const NAME_KEY: &str = "name";
const THEME_VERSION: u64 = <Theme as CosmicConfigEntry>::VERSION;
/// Default package version for exports that do not specify one.
pub const DEFAULT_PACKAGE_VERSION: &str = "1.0.0";
const PACKAGE_SUFFIX: &str = "Package";

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    cosmic_config::cosmic_config_derive::CosmicConfigEntry,
)]
#[version = 1]
pub struct Package {
    pub package_id: String,
    pub version: String,
}

impl Default for Package {
    fn default() -> Self {
        Self {
            package_id: String::new(),
            version: DEFAULT_PACKAGE_VERSION.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bundle {
    pub slug: String,
    pub name: String,
    pub mode: Mode,
    pub package: Option<Package>,
}

/// An error from managing bundles.
#[derive(Debug)]
pub enum Error {
    OnlyBundle(Mode),
    Config(cosmic_config::Error),
    Io(PathBuf, io::Error),
    NoConfigDir,
    SystemBundle(String),
    UnknownBundle(String),
    Slug(String),
    Zip(zip::result::ZipError),
    Archive(String),
    NoDataDir,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OnlyBundle(mode) => {
                write!(f, "the only {} theme bundle cannot be removed", mode.name())
            }
            Self::Config(err) => err.fmt(f),
            Self::Io(path, err) => write!(f, "{}: {err}", path.display()),
            Self::NoConfigDir => f.write_str("no cosmic config directory"),
            Self::SystemBundle(slug) => {
                write!(f, "theme bundle '{slug}' is part of the system install")
            }
            Self::UnknownBundle(slug) => write!(f, "unknown theme bundle '{slug}'"),
            Self::Slug(slug) => write!(f, "not a theme bundle slug: {slug:.64}"),
            Self::Zip(err) => write!(f, "zip: {err}"),
            Self::Archive(reason) => f.write_str(reason),
            Self::NoDataDir => f.write_str("no data directory"),
        }
    }
}

impl std::error::Error for Error {}

impl From<cosmic_config::Error> for Error {
    fn from(err: cosmic_config::Error) -> Self {
        Self::Config(err)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Self::Zip(err)
    }
}

#[derive(Debug, Clone)]
pub struct Store {
    cosmic_dir: PathBuf,
}

impl Store {
    pub fn new() -> Result<Self, Error> {
        let cosmic_dir = config_dir().ok_or(Error::NoConfigDir)?;
        Ok(Self { cosmic_dir })
    }

    fn config(&self, id: &str, version: u64) -> Result<Config, Error> {
        Ok(Config::new(id, version)?)
    }

    fn dir(&self, id: &str, version: u64) -> Result<PathBuf, Error> {
        Ok(self.cosmic_dir.join(Config::id_v(id, version)?))
    }

    pub fn live_config(&self, part: Part, mode: Mode) -> Result<Config, Error> {
        self.config(part.live_id(mode), part.version())
    }

    fn bundle_id(slug: &str, part: Part) -> String {
        bundle_id(slug, part)
    }

    fn package_config_id(slug: &str) -> String {
        format!("{NAME}.{slug}.{PACKAGE_SUFFIX}")
    }

    fn package_config(&self, slug: &str) -> Result<Config, Error> {
        self.config(
            &Self::package_config_id(slug),
            <Package as CosmicConfigEntry>::VERSION,
        )
    }

    fn bundle_package(&self, slug: &str) -> Option<Package> {
        let dir = self.cosmic_dir.join(Self::package_config_id(slug));
        if !any_version_keys(&dir).ok()? {
            return None;
        }
        self.package_of(&self.package_config(slug).ok()?)
    }

    fn system_package(&self, slug: &str) -> Option<Package> {
        let dir = self.system_dir()?.join(Self::package_config_id(slug));
        if !any_version_keys(&dir).ok()? {
            return None;
        }
        let config = Config::system(
            &Self::package_config_id(slug),
            <Package as CosmicConfigEntry>::VERSION,
        )
        .ok()?;
        self.package_of(&config)
    }

    fn package_of(&self, config: &Config) -> Option<Package> {
        let package = match Package::get_entry(config) {
            Ok(package) => package,
            Err((errors, package)) => {
                for error in errors {
                    tracing::warn!(?error, "invalid package metadata");
                }
                package
            }
        };
        (!package.package_id.is_empty()).then_some(package)
    }

    fn write_package(&self, slug: &str, package: &Package) -> Result<(), Error> {
        let config = self.package_config(slug)?;
        package.write_entry(&config)?;
        Ok(())
    }

    pub(crate) fn remove_package(&self, slug: &str) -> Result<(), Error> {
        remove_dir(&self.cosmic_dir.join(Self::package_config_id(slug)))
    }

    pub fn bundle_config(&self, slug: &str, part: Part) -> Result<Config, Error> {
        self.config(&Self::bundle_id(slug, part), part.version())
    }

    fn bundle_dir(&self, slug: &str, part: Part) -> Result<PathBuf, Error> {
        self.dir(&Self::bundle_id(slug, part), part.version())
    }

    fn bundle_config_dir(&self, slug: &str, part: Part) -> PathBuf {
        self.cosmic_dir.join(Self::bundle_id(slug, part))
    }

    fn mode_config(&self) -> Result<Config, Error> {
        self.config(MODE_ID, <ThemeMode as CosmicConfigEntry>::VERSION)
    }

    pub fn current_mode(&self) -> Mode {
        self.mode_config().map_or(Mode::Dark, |config| {
            read::<ThemeMode>(&config).map_or(Mode::Dark, |mode| Mode::from_is_dark(mode.is_dark))
        })
    }

    pub fn bundles(&self) -> Result<Vec<Bundle>, Error> {
        let mut bundles: Vec<Bundle> = slugs_in(&self.cosmic_dir)?
            .into_iter()
            .filter_map(|slug| self.bundle(&slug).ok())
            .collect();
        bundles.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(bundles)
    }

    pub fn system_bundles(&self) -> Result<Vec<Bundle>, Error> {
        let Some(dir) = self.system_dir() else {
            return Ok(Vec::new());
        };
        let mut bundles: Vec<Bundle> = slugs_in(&dir)?
            .into_iter()
            .filter_map(|slug| self.system_bundle(&slug).ok())
            .collect();
        bundles.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(bundles)
    }

    fn bundles_for(&self, mode: Mode) -> Result<Vec<Bundle>, Error> {
        let mut bundles = self.bundles()?;
        bundles.retain(|bundle| bundle.mode == mode);
        for installed in self.system_bundles()? {
            if installed.mode != mode {
                continue;
            }
            let shadowed = bundles
                .iter()
                .any(|bundle| bundle.slug == installed.slug || bundle.name == installed.name);
            if !shadowed {
                bundles.push(installed);
            }
        }
        Ok(bundles)
    }

    pub fn system_bundle(&self, slug: &str) -> Result<Bundle, Error> {
        let config = self.system_config(slug, Part::Theme)?;
        let theme = theme_of(&config)?;
        Ok(Bundle {
            slug: slug.to_owned(),
            name: theme.name,
            mode: Mode::from_is_dark(theme.is_dark),
            package: self.system_package(slug),
        })
    }

    fn system_config(&self, slug: &str, part: Part) -> Result<Config, Error> {
        let Some(dir) = self.system_dir() else {
            return Err(Error::UnknownBundle(slug.to_owned()));
        };
        let path = dir
            .join(Self::bundle_id(slug, part))
            .join(format!("v{}", part.version()));
        if !has_keys(&path) {
            return Err(Error::UnknownBundle(slug.to_owned()));
        }
        Ok(Config::system(
            &Self::bundle_id(slug, part),
            part.version(),
        )?)
    }

    pub fn bundle_theme(&self, slug: &str) -> Result<Theme, Error> {
        if has_keys(&self.bundle_dir(slug, Part::Theme)?) {
            return theme_of(&self.bundle_config(slug, Part::Theme)?);
        }
        theme_of(&self.system_config(slug, Part::Theme)?)
    }

    fn system_dir(&self) -> Option<PathBuf> {
        xdg::BaseDirectories::with_prefix("cosmic")
            .get_data_dirs()
            .into_iter()
            .find(|dir| slugs_in(dir).is_ok_and(|slugs| !slugs.is_empty()))
    }

    /// Copy a bundle installed in the system into the user's config.
    pub fn adopt(&self, slug: &str) -> Result<(), Error> {
        for part in Part::ALL {
            let from = match self.system_config(slug, part) {
                Ok(config) => config,
                Err(Error::UnknownBundle(_)) => continue,
                Err(err) => return Err(err),
            };
            let to = self.bundle_config(slug, part)?;
            self.copy_part(&from, &to, part)?;
        }
        self.materialize(slug)?;
        if let Some(package) = self.system_package(slug) {
            self.write_package(slug, &package)?;
        }

        let name = self.bundle(slug)?.name;
        if self
            .bundles()?
            .iter()
            .any(|bundle| bundle.slug != slug && bundle.name == name)
        {
            let (name, _) = self.unique_name_avoiding(&name, &self.reserved_names())?;
            self.write_imported_name(slug, &name)?;
        }
        Ok(())
    }

    /// Copy the known keys of a config part into a bundle.
    fn copy_part(&self, from: &Config, to: &Config, part: Part) -> Result<(), Error> {
        match part {
            Part::Theme => {
                let theme = neutral_theme(&theme_of(from)?);
                theme.write_entry(to)?;
                to.set(BASE, &theme)?;
            }
            Part::Builder => {
                let builder = neutral_builder(&builder_of(from)?);
                builder.write_entry(to)?;
                to.set(BASE, &builder)?;
            }
            Part::Background => {
                let same_on_all = from
                    .get::<bool>(cosmic_bg_config::SAME_ON_ALL)
                    .unwrap_or(true);
                let outputs = from
                    .get::<Vec<String>>(cosmic_bg_config::BACKGROUNDS)
                    .unwrap_or_default();
                to.set(cosmic_bg_config::SAME_ON_ALL, same_on_all)?;
                if let Ok(entry) =
                    from.get::<cosmic_bg_config::Entry>(cosmic_bg_config::DEFAULT_BACKGROUND)
                {
                    to.set(cosmic_bg_config::DEFAULT_BACKGROUND, &entry)?;
                }
                for output in &outputs {
                    let key = format!("output.{output}");
                    if let Ok(entry) = from.get::<cosmic_bg_config::Entry>(&key) {
                        to.set(&key, &entry)?;
                    }
                }
                to.set(cosmic_bg_config::BACKGROUNDS, &outputs)?;
                if let Ok(base) = from.get::<cosmic_bg_config::Entry>(BASE) {
                    to.set(BASE, &base)?;
                }
            }
        }
        Ok(())
    }

    /// Write a packaged preset out as the keys of an adopted bundle.
    fn materialize(&self, slug: &str) -> Result<(), Error> {
        let source = |part| Config::system(&Self::bundle_id(slug, part), part.version());
        let theme_source = source(Part::Theme)?;
        if theme_source.get::<String>(NAME_KEY).is_ok() {
            return Ok(());
        }
        if let Ok(theme) = theme_source.get::<Theme>(BASE) {
            let theme = neutral_theme(&theme);
            let config = self.bundle_config(slug, Part::Theme)?;
            theme.write_entry(&config)?;
            write_base(&config, &theme.name, &theme)?;
        }
        if let Ok(builder) = source(Part::Builder)?.get::<ThemeBuilder>(BASE) {
            let builder = neutral_builder(&builder);
            let config = self.bundle_config(slug, Part::Builder)?;
            builder.write_entry(&config)?;
            write_builder_base(&config, &builder)?;
        }
        Ok(())
    }

    pub fn bundle(&self, slug: &str) -> Result<Bundle, Error> {
        if !has_keys(&self.bundle_dir(slug, Part::Theme)?) {
            return Err(Error::UnknownBundle(slug.to_owned()));
        }
        let theme = theme_of(&self.bundle_config(slug, Part::Theme)?)?;
        Ok(Bundle {
            slug: slug.to_owned(),
            name: theme.name,
            mode: Mode::from_is_dark(theme.is_dark),
            package: self.bundle_package(slug),
        })
    }

    pub fn activate(&self, slug: &str) -> Result<Bundle, Error> {
        let bundle = match self.bundle(slug) {
            Ok(bundle) => bundle,
            Err(err @ Error::UnknownBundle(_)) => match self.system_bundle(slug) {
                Ok(_) => {
                    self.adopt(slug)?;
                    self.bundle(slug)?
                }
                Err(_) => return Err(err),
            },
            Err(err) => return Err(err),
        };
        for part in Part::ALL {
            if !self.bundle_dir(slug, part)?.is_dir() {
                return Err(Error::UnknownBundle(slug.to_owned()));
            }
        }
        let live_name = self
            .live_config(Part::Theme, bundle.mode)?
            .get::<String>(NAME_KEY)
            .ok();
        let matches = self.live_matches(bundle.mode, &bundle.slug, &Part::ALL)?;
        if live_name.as_deref() == Some(bundle.name.as_str()) && matches {
            return Ok(bundle);
        }
        if !matches {
            let _ = self.preserve_live(bundle.mode)?;
        }
        self.swap_live(bundle.mode, &bundle.slug)?;
        Ok(bundle)
    }

    /// Save the live look into the active bundle for `mode`.
    pub fn sync_active(&self, mode: Mode) -> Result<Option<Bundle>, Error> {
        let bundle = match self.find_active(mode)? {
            Some(bundle) => Some(bundle),
            None => self.preserve_live(mode)?,
        };
        let Some(bundle) = bundle else {
            return Ok(None);
        };
        let matches = self
            .live_matches(mode, &bundle.slug, &[Part::Theme, Part::Builder])
            .unwrap_or(false);
        if !matches {
            self.sync_bundle_look(&bundle, mode)?;
        }
        Ok(Some(bundle))
    }

    /// Save only the live background into the active bundle for `mode`.
    pub fn sync_active_background(&self, mode: Mode) -> Result<Option<Bundle>, Error> {
        let bundle = match self.find_active(mode)? {
            Some(bundle) => Some(bundle),
            None => self.preserve_live(mode)?,
        };
        let Some(bundle) = bundle else {
            return Ok(None);
        };
        let matches = self
            .live_matches(mode, &bundle.slug, &[Part::Background])
            .unwrap_or(false);
        if !matches {
            self.sync_bundle_background(&bundle, mode)?;
        }
        Ok(Some(bundle))
    }

    /// Apply a bundle's background to the live config for a mode.
    pub fn apply_background(&self, slug: &str, mode: Mode) -> Result<bool, Error> {
        let user = self.bundle_config(slug, Part::Background)?;
        let from = if user
            .get::<cosmic_bg_config::Entry>(cosmic_bg_config::DEFAULT_BACKGROUND)
            .is_ok()
        {
            user
        } else {
            match self.system_config(slug, Part::Background) {
                Ok(system) => system,
                Err(Error::UnknownBundle(_)) => return Ok(false),
                Err(err) => return Err(err),
            }
        };

        let base = from.get::<cosmic_bg_config::Entry>(BASE).ok();
        let same_on_all = from
            .get::<bool>(cosmic_bg_config::SAME_ON_ALL)
            .unwrap_or(true);
        let Some(default_background) = from
            .get::<cosmic_bg_config::Entry>(cosmic_bg_config::DEFAULT_BACKGROUND)
            .ok()
        else {
            return Ok(false);
        };
        let backgrounds = if same_on_all {
            Vec::new()
        } else {
            from.get::<Vec<String>>(cosmic_bg_config::BACKGROUNDS)
                .unwrap_or_default()
                .into_iter()
                .filter_map(|output| {
                    from.get::<cosmic_bg_config::Entry>(&format!("output.{output}"))
                        .ok()
                })
                .collect::<Vec<_>>()
        };

        let target = cosmic_bg_config::Context(self.live_config(Part::Background, mode)?);
        target.set_same_on_all(same_on_all)?;
        target
            .0
            .set(cosmic_bg_config::DEFAULT_BACKGROUND, default_background)?;
        for entry in &backgrounds {
            let key = format!("output.{}", entry.output);
            target.0.set(&key, entry)?;
        }
        target.0.set(
            cosmic_bg_config::BACKGROUNDS,
            backgrounds
                .iter()
                .map(|entry| entry.output.clone())
                .collect::<Vec<_>>(),
        )?;
        if let Some(base) = base {
            target.0.set(BASE, base)?;
        }

        Ok(true)
    }

    /// The background shown for a mode.
    pub fn display_background(&self, mode: Mode) -> Result<cosmic_bg_config::Context, Error> {
        Ok(cosmic_bg_config::Context(
            self.live_config(Part::Background, mode)?,
        ))
    }

    fn find_active(&self, mode: Mode) -> Result<Option<Bundle>, Error> {
        let theme = read::<Theme>(&self.live_config(Part::Theme, mode)?)?;
        let name = store_name(&theme, mode);
        if let Some(bundle) = self.worn_bundle(mode, &name)? {
            return Ok(Some(bundle));
        }
        if let Some(system) = self
            .system_bundles()?
            .into_iter()
            .find(|bundle| bundle.mode == mode && bundle.name == name)
        {
            self.adopt(&system.slug)?;
            return Ok(self.bundle(&system.slug).ok());
        }
        Ok(None)
    }

    fn sync_bundle_background(&self, bundle: &Bundle, mode: Mode) -> Result<(), Error> {
        let live = cosmic_bg_config::Config::load(&cosmic_bg_config::Context(
            self.live_config(Part::Background, mode)?,
        ))?;
        let stored = self.bundle_config(&bundle.slug, Part::Background)?;

        stored.set(cosmic_bg_config::SAME_ON_ALL, live.same_on_all)?;
        stored.set(
            cosmic_bg_config::DEFAULT_BACKGROUND,
            live.default_background.clone(),
        )?;
        for entry in &live.backgrounds {
            let key = format!("output.{}", entry.output);
            stored.set(&key, entry)?;
        }
        stored.set(
            cosmic_bg_config::BACKGROUNDS,
            live.backgrounds
                .iter()
                .map(|entry| entry.output.clone())
                .collect::<Vec<_>>(),
        )?;
        stored.set(BASE, &live.default_background)?;

        Ok(())
    }

    fn sync_bundle(&self, bundle: &Bundle, mode: Mode) -> Result<(), Error> {
        self.sync_bundle_look(bundle, mode)?;
        self.sync_bundle_background(bundle, mode)?;
        Ok(())
    }

    fn sync_bundle_look(&self, bundle: &Bundle, mode: Mode) -> Result<(), Error> {
        let mut theme = read::<Theme>(&self.live_config(Part::Theme, mode)?)?;
        theme.name.clone_from(&bundle.name);
        let config = self.bundle_config(&bundle.slug, Part::Theme)?;
        if theme_has_per_key(&config) {
            let old = theme_of(&config)?;
            write_theme_changes(&config, &old, &theme)?;
        } else {
            theme.write_entry(&config)?;
        }

        let builder = read::<ThemeBuilder>(&self.live_config(Part::Builder, mode)?)?;
        let config = self.bundle_config(&bundle.slug, Part::Builder)?;
        if builder_has_per_key(&config) {
            let old = builder_of(&config)?;
            write_builder_changes(&config, &old, &builder)?;
        } else {
            builder.write_entry(&config)?;
        }
        Ok(())
    }

    /// Save a mode's standalone live look before another look replaces it.
    fn preserve_live(&self, mode: Mode) -> Result<Option<Bundle>, Error> {
        let theme = read::<Theme>(&self.live_config(Part::Theme, mode)?)?;
        let name = store_name(&theme, mode);
        if let Some(bundle) = self.worn_bundle(mode, &name)? {
            return Ok(Some(bundle));
        }
        if self.live_matches_system(mode)? {
            return Ok(None);
        }
        if let Some(system) = self
            .system_bundles()?
            .into_iter()
            .find(|bundle| bundle.mode == mode && bundle.name == name)
        {
            self.adopt(&system.slug)?;
            let bundle = self.bundle(&system.slug)?;
            self.sync_bundle(&bundle, mode)?;
            return Ok(Some(bundle));
        }
        let avoid: Vec<String> = self
            .system_bundles()?
            .into_iter()
            .map(|bundle| bundle.name)
            .collect();
        let (name, slug) = self.unique_name_avoiding(&name, &avoid)?;
        self.live_config(Part::Theme, mode)?
            .set(NAME_KEY, name.clone())?;
        let bundle = self.store_live(&name, &slug, mode)?;
        Ok(Some(bundle))
    }

    /// Whether a mode's live look is the installed preset for its name.
    fn live_matches_system(&self, mode: Mode) -> Result<bool, Error> {
        let Some(system) = self.system_for_live(mode)? else {
            return Ok(false);
        };
        let live_theme = read::<Theme>(&self.live_config(Part::Theme, mode)?)?;
        let Ok(system_theme) = self
            .system_config(&system.slug, Part::Theme)?
            .get::<Theme>(BASE)
        else {
            return Ok(false);
        };
        if neutral_theme(&live_theme) != neutral_theme(&system_theme) {
            return Ok(false);
        }
        let live_builder = read::<ThemeBuilder>(&self.live_config(Part::Builder, mode)?)?;
        let Ok(system_builder) = self
            .system_config(&system.slug, Part::Builder)?
            .get::<ThemeBuilder>(BASE)
        else {
            return Ok(false);
        };
        if neutral_builder(&live_builder) != neutral_builder(&system_builder) {
            return Ok(false);
        }
        let Ok(system_background) = self.system_config(&system.slug, Part::Background) else {
            return Ok(false);
        };
        let system_background = system_background
            .get::<cosmic_bg_config::Entry>(BASE)
            .or_else(|_| system_background.get(cosmic_bg_config::DEFAULT_BACKGROUND))
            .ok();
        Ok(default_background(&self.live_config(Part::Background, mode)?) == system_background)
    }

    /// Whether a mode's live look is the one a bundle holds, ignoring interface settings.
    fn live_matches(&self, mode: Mode, slug: &str, parts: &[Part]) -> Result<bool, Error> {
        for part in parts.iter().copied() {
            match part {
                Part::Theme => {
                    let live = read::<Theme>(&self.live_config(part, mode)?)?;
                    let stored = theme_of(&self.bundle_config(slug, part)?)?;
                    if neutral_theme(&live) != neutral_theme(&stored) {
                        return Ok(false);
                    }
                }
                Part::Builder => {
                    let live = read::<ThemeBuilder>(&self.live_config(part, mode)?)?;
                    let stored = builder_of(&self.bundle_config(slug, part)?)?;
                    if neutral_builder(&live) != neutral_builder(&stored) {
                        return Ok(false);
                    }
                }
                Part::Background => {
                    let live = self.live_config(part, mode)?;
                    let stored = self.bundle_config(slug, part)?;
                    let live_background = live
                        .get_local::<cosmic_bg_config::Entry>(cosmic_bg_config::DEFAULT_BACKGROUND)
                        .ok();
                    let stored_background = stored
                        .get::<cosmic_bg_config::Entry>(BASE)
                        .or_else(|_| {
                            stored.get_local::<cosmic_bg_config::Entry>(
                                cosmic_bg_config::DEFAULT_BACKGROUND,
                            )
                        })
                        .ok();
                    if live_background != stored_background {
                        return Ok(false);
                    }
                    let live_same = live
                        .get_local::<bool>(cosmic_bg_config::SAME_ON_ALL)
                        .unwrap_or(true);
                    let stored_same = stored
                        .get_local::<bool>(cosmic_bg_config::SAME_ON_ALL)
                        .unwrap_or(true);
                    if live_same != stored_same {
                        return Ok(false);
                    }
                    if !live_same {
                        let live = cosmic_bg_config::Config::load(&cosmic_bg_config::Context(
                            live.clone(),
                        ))?;
                        let stored = cosmic_bg_config::Config::load(&cosmic_bg_config::Context(
                            stored.clone(),
                        ))?;
                        let entries = |config: &cosmic_bg_config::Config| {
                            config
                                .backgrounds
                                .iter()
                                .cloned()
                                .map(|entry| (entry.output.clone(), entry))
                                .collect::<BTreeMap<_, _>>()
                        };
                        if entries(&live) != entries(&stored) {
                            return Ok(false);
                        }
                    }
                }
            }
        }
        Ok(true)
    }

    /// Whether a slug names a look the system install defines, or the user's copy of it.
    fn is_system(&self, slug: &str) -> Result<bool, Error> {
        if self.system_bundle(slug).is_ok() {
            return Ok(true);
        }
        let Ok(bundle) = self.bundle(slug) else {
            return Ok(false);
        };
        Ok(self.system_bundles()?.iter().any(|installed| {
            installed.slug == bundle.slug
                || (installed.mode == bundle.mode && installed.name == bundle.name)
        }))
    }

    /// Delete a bundle, promoting another bundle for its mode when the active
    /// one is removed so the mode keeps a saved look for the daemon to apply.
    /// A mode's last bundle is kept.
    pub fn delete(&self, slug: &str) -> Result<(), Error> {
        if self.is_system(slug)? {
            return Err(Error::SystemBundle(slug.to_owned()));
        }
        let bundle = self.bundle(slug)?;
        if self.bundles_for(bundle.mode)?.len() <= 1 {
            return Err(Error::OnlyBundle(bundle.mode));
        }
        let live = read::<Theme>(&self.live_config(Part::Theme, bundle.mode)?)?;
        let was_active = store_name(&live, bundle.mode) == bundle.name;
        for part in Part::ALL {
            remove_dir(&self.bundle_config_dir(slug, part))?;
        }
        self.remove_package(slug)?;
        crate::transfer::remove_installed_assets(self, slug)?;
        if was_active {
            self.promote(&bundle)?;
        }
        Ok(())
    }

    /// Show another bundle for the mode of a bundle that was just removed.
    fn promote(&self, removed: &Bundle) -> Result<(), Error> {
        let Some(slug) = self.replacement(removed)? else {
            return Ok(());
        };
        self.write_live(removed.mode, &slug)
    }

    /// The bundle to show after `removed` is gone: the next one in the order the
    /// bundle sections list, wrapping around.
    fn replacement(&self, removed: &Bundle) -> Result<Option<String>, Error> {
        let mut candidates = self.bundles_for(removed.mode)?;
        candidates.retain(|bundle| bundle.slug != removed.slug);
        candidates.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.slug.cmp(&b.slug)));
        let removed_key = (&removed.name, &removed.slug);
        let replacement = candidates
            .iter()
            .find(|bundle| (&bundle.name, &bundle.slug) > removed_key)
            .or_else(|| candidates.first());
        let Some(replacement) = replacement else {
            return Ok(None);
        };
        let slug = replacement.slug.clone();
        if self.bundle(&slug).is_err() {
            self.adopt(&slug)?;
        }
        Ok(Some(slug))
    }

    pub fn reset(&self, slug: &str) -> Result<(), Error> {
        for part in Part::ALL {
            let config = self.bundle_config(slug, part)?;
            restore_part(&config, &config, part)?;
        }
        Ok(())
    }

    pub fn reset_live(&self, mode: Mode) -> Result<bool, Error> {
        let installed = self.system_for_live(mode)?;
        let mut restored = false;
        for part in Part::ALL {
            let live = self.live_config(part, mode)?;
            let source = installed
                .as_ref()
                .and_then(|bundle| self.system_config(&bundle.slug, part).ok());
            let part_restored = match source {
                Some(source) => restore_part(&source, &live, part)?,
                None => restore_part(&live, &live, part)?,
            };
            restored |= part_restored;
        }
        Ok(restored)
    }

    /// The system preset whose name and mode a mode's live look has.
    fn system_for_live(&self, mode: Mode) -> Result<Option<Bundle>, Error> {
        let config = self.live_config(Part::Theme, mode)?;
        let name = config
            .get::<String>(NAME_KEY)
            .ok()
            .filter(|name| !name.is_empty())
            .or_else(|| theme_of(&config).ok().map(|theme| theme.name));
        let Some(name) = name else {
            return Ok(None);
        };
        Ok(self
            .system_bundles()?
            .into_iter()
            .find(|bundle| bundle.mode == mode && bundle.name == name))
    }

    pub fn reset_live_to_installed(&self, mode: Mode) -> Result<(), Error> {
        let builder = if mode.is_dark() {
            ThemeBuilder::dark()
        } else {
            ThemeBuilder::light()
        };

        let config = self.live_config(Part::Theme, mode)?;
        let mut theme = builder.clone().build();
        keep_theme_interface(&mut theme, &read::<Theme>(&config)?);
        theme.write_entry(&config)?;
        config.set(BASE, neutral_theme(&theme))?;

        let config = self.live_config(Part::Builder, mode)?;
        let mut builder = builder;
        keep_builder_interface(&mut builder, &read::<ThemeBuilder>(&config)?);
        builder.write_entry(&config)?;
        Ok(())
    }

    fn store_live(&self, name: &str, slug: &str, mode: Mode) -> Result<Bundle, Error> {
        let bundle = Bundle {
            slug: slug.to_owned(),
            name: name.to_owned(),
            mode,
            package: None,
        };
        self.sync_bundle(&bundle, mode)?;
        for part in Part::ALL {
            self.write_installed_keys(slug, part, mode)?;
        }
        Ok(bundle)
    }

    fn write_installed_keys(&self, slug: &str, part: Part, mode: Mode) -> Result<(), Error> {
        let live_id = part.live_id(mode);
        let bundle_id = Self::bundle_id(slug, part);

        let mut written = Vec::new();
        for version in chain_versions(&self.bundle_config(slug, part)?) {
            let dir = self.dir(&bundle_id, version)?;
            if let Ok(entries) = fs::read_dir(&dir) {
                written.extend(entries.flatten().map(|entry| entry.file_name()));
            }
        }

        for version in chain_versions(&self.live_config(part, mode)?) {
            let Ok(installed) = Config::system(live_id, version) else {
                continue;
            };
            let Some(from) = installed.system_path().map(Path::to_path_buf) else {
                continue;
            };
            let entries = match fs::read_dir(&from) {
                Ok(entries) => entries,
                Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
                Err(err) => return Err(Error::Io(from, err)),
            };

            let to = self.dir(&bundle_id, version)?;
            fs::create_dir_all(&to).map_err(|err| Error::Io(to.clone(), err))?;
            for entry in entries.flatten() {
                let name = entry.file_name();
                if !entry.path().is_file()
                    || written.contains(&name)
                    || name.to_str().is_some_and(interface_key)
                {
                    continue;
                }
                let target = to.join(&name);
                fs::copy(entry.path(), &target).map_err(|err| Error::Io(target, err))?;
            }
        }

        Ok(())
    }

    fn swap_live(&self, mode: Mode, incoming: &str) -> Result<(), Error> {
        const LOOK: [Part; 2] = [Part::Theme, Part::Builder];
        let theme = read::<Theme>(&self.live_config(Part::Theme, mode)?)?;
        let name = store_name(&theme, mode);
        if let Some(worn) = self.worn_bundle(mode, &name)?
            && worn.slug != incoming
        {
            self.sync_bundle_background(&worn, mode)?;
            if !self.live_matches(mode, &worn.slug, &LOOK).unwrap_or(false) {
                self.sync_bundle_look(&worn, mode)?;
            }
        }
        self.write_live(mode, incoming)
    }

    /// Write a bundle's parts into a mode's live config, keeping its interface keys.
    fn write_live(&self, mode: Mode, slug: &str) -> Result<(), Error> {
        let theme_config = self.bundle_config(slug, Part::Theme)?;
        let builder_config = self.bundle_config(slug, Part::Builder)?;

        if theme_has_per_key(&theme_config) && builder_has_per_key(&builder_config) {
            let live = read::<Theme>(&self.live_config(Part::Theme, mode)?)?;
            let incoming = theme_of(&theme_config)?;
            write_theme_changes(&self.live_config(Part::Theme, mode)?, &live, &incoming)?;

            let live = read::<ThemeBuilder>(&self.live_config(Part::Builder, mode)?)?;
            let incoming = builder_of(&builder_config)?;
            write_builder_changes(&self.live_config(Part::Builder, mode)?, &live, &incoming)?;
        } else {
            let live = read::<Theme>(&self.live_config(Part::Theme, mode)?)?;
            let mut theme = theme_of(&theme_config)?;
            keep_theme_interface(&mut theme, &live);
            theme.write_entry(&self.live_config(Part::Theme, mode)?)?;

            let live = read::<ThemeBuilder>(&self.live_config(Part::Builder, mode)?)?;
            let mut builder = builder_of(&builder_config)?;
            keep_builder_interface(&mut builder, &live);
            builder.write_entry(&self.live_config(Part::Builder, mode)?)?;
        }

        self.apply_background(slug, mode)?;
        Ok(())
    }

    fn worn_bundle(&self, mode: Mode, name: &str) -> Result<Option<Bundle>, Error> {
        Ok(self
            .bundles()?
            .into_iter()
            .find(|bundle| bundle.mode == mode && bundle.name == name))
    }

    pub fn migrate_toolkit(&self) -> Result<(), Error> {
        const PAIRS: [(&str, &str); 3] = [
            ("icon_theme", "icon_theme"),
            ("interface_font", "font"),
            ("monospace_font", "mono_font"),
        ];

        let marker = self.state_dir().join(TOOLKIT_MIGRATION);
        if marker.exists() {
            return Ok(());
        }

        let toolkit = self.cosmic_dir.join(TOOLKIT_ID);
        let Ok(entries) = fs::read_dir(&toolkit) else {
            return Ok(());
        };
        let mut versions: Vec<PathBuf> = entries
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.is_dir() && version_of(path).is_some())
            .collect();
        versions.sort();
        versions.reverse();

        let mut complete = true;
        for (old_key, new_key) in PAIRS {
            let Some(from) = versions
                .iter()
                .map(|dir| dir.join(old_key))
                .find(|path| path.is_file())
            else {
                continue;
            };
            for mode in [Mode::Dark, Mode::Light] {
                let config = self.config(mode.theme_id(), THEME_VERSION)?;
                let target = self.dir(mode.theme_id(), THEME_VERSION)?;
                let to = target.join(new_key);
                if !to.exists() {
                    fs::create_dir_all(&target).map_err(|err| Error::Io(target.clone(), err))?;
                    fs::copy(&from, &to).map_err(|err| Error::Io(to, err))?;
                }
                let read_back = match new_key {
                    "icon_theme" => config.get::<String>(new_key).is_ok(),
                    _ => config.get::<cosmic_theme::FontConfig>(new_key).is_ok(),
                };
                complete &= read_back;
            }
        }
        if complete {
            if let Some(parent) = marker.parent() {
                fs::create_dir_all(parent).map_err(|err| Error::Io(parent.to_path_buf(), err))?;
            }
            fs::write(&marker, b"").map_err(|err| Error::Io(marker, err))?;
        } else {
            tracing::warn!("a toolkit setting did not read back, so the migration will run again");
        }
        Ok(())
    }

    /// Rename stored copies of the built-in themes to their current names.
    pub fn migrate_names(&self) -> Result<(), Error> {
        let marker = self.state_dir().join(THEME_NAMES_MIGRATION);
        if marker.exists() {
            return Ok(());
        }

        for (old, new) in LEGACY_THEME_NAMES {
            for bundle in self.bundles()? {
                if bundle.name == old {
                    self.rename_stored(&bundle.slug, new)?;
                }
            }
            for mode in [Mode::Dark, Mode::Light] {
                self.rename_live(mode, old, new)?;
            }
        }

        if let Some(parent) = marker.parent() {
            fs::create_dir_all(parent).map_err(|err| Error::Io(parent.to_path_buf(), err))?;
        }
        fs::write(&marker, b"").map_err(|err| Error::Io(marker, err))?;
        Ok(())
    }

    /// Rename a stored bundle's theme and palette.
    fn rename_stored(&self, slug: &str, name: &str) -> Result<(), Error> {
        let config = self.bundle_config(slug, Part::Theme)?;
        if let Ok(mut theme) = theme_of(&config) {
            theme.name = name.to_owned();
            theme.palette.name = name.to_owned();
            config.set(NAME_KEY, name.to_owned())?;
            config.set("palette", theme.palette.clone())?;
            if config.get::<Theme>(BASE).is_ok() {
                config.set(BASE, theme)?;
            }
        }

        let config = self.bundle_config(slug, Part::Builder)?;
        if any_version_keys(&self.bundle_config_dir(slug, Part::Builder))?
            && let Ok(mut builder) = builder_of(&config)
        {
            builder.palette.as_mut().name = name.to_owned();
            config.set("palette", builder.palette.clone())?;
            if config.get::<ThemeBuilder>(BASE).is_ok() {
                config.set(BASE, builder)?;
            }
        }
        Ok(())
    }

    /// Rename a live mode's theme and palette when it still has the old name.
    fn rename_live(&self, mode: Mode, old: &str, new: &str) -> Result<(), Error> {
        let config = self.live_config(Part::Theme, mode)?;
        let Ok(mut theme) = read::<Theme>(&config) else {
            return Ok(());
        };
        if theme.name != old {
            return Ok(());
        }

        theme.name = new.to_owned();
        theme.palette.name = new.to_owned();
        config.set(NAME_KEY, new.to_owned())?;
        config.set("palette", theme.palette.clone())?;
        if config.get::<Theme>(BASE).is_ok() {
            config.set(BASE, theme)?;
        }

        let config = self.live_config(Part::Builder, mode)?;
        if let Ok(mut builder) = read::<ThemeBuilder>(&config)
            && builder.palette.name() == old
        {
            builder.palette.as_mut().name = new.to_owned();
            config.set("palette", builder.palette.clone())?;
            if config.get::<ThemeBuilder>(BASE).is_ok() {
                config.set(BASE, builder)?;
            }
        }
        Ok(())
    }

    pub(crate) fn state_dir(&self) -> PathBuf {
        if let Some(dir) = xdg::BaseDirectories::new().get_state_home() {
            return dir.join("cosmic");
        }
        static FALLBACK: OnceLock<tempfile::TempDir> = OnceLock::new();
        FALLBACK
            .get_or_init(|| {
                tempfile::Builder::new()
                    .prefix("cosmic-settings-state-")
                    .tempdir()
                    .expect("failed to create a temporary state directory")
            })
            .path()
            .join("cosmic")
    }

    /// A free name and slug, avoiding the names an import must not reuse.
    fn unique_name_avoiding(
        &self,
        name: &str,
        avoid: &[String],
    ) -> Result<(String, String), Error> {
        let stored = self.bundles()?;
        let mut nth = 1;
        loop {
            let candidate = if nth == 1 {
                name.to_owned()
            } else {
                format!("{name} {nth}")
            };
            let name_taken =
                stored.iter().any(|bundle| bundle.name == candidate) || avoid.contains(&candidate);
            if !name_taken {
                let base = slugify(&candidate);
                let mut slug = base.clone();
                let mut slug_nth = 1;
                while self.slug_taken(&slug)? {
                    slug_nth += 1;
                    slug = numbered_slug(&base, slug_nth);
                }
                return Ok((candidate, slug));
            }
            nth += 1;
        }
    }

    fn slug_taken(&self, slug: &str) -> Result<bool, Error> {
        for part in Part::ALL {
            if any_version_keys(&self.bundle_config_dir(slug, part))? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Names a new look must not reuse: the live looks and the presets.
    fn reserved_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for mode in [Mode::Dark, Mode::Light] {
            if let Ok(config) = self.live_config(Part::Theme, mode)
                && let Ok(theme) = read::<Theme>(&config)
                && !theme.name.is_empty()
            {
                names.push(theme.name);
            }
        }
        if let Ok(system) = self.system_bundles() {
            names.extend(system.into_iter().map(|bundle| bundle.name));
        }
        names
    }

    /// Give every stored look a distinct name, keeping the one on screen.
    pub fn normalize_names(&self) -> Result<(), Error> {
        let mut groups: BTreeMap<String, Vec<Bundle>> = BTreeMap::new();
        for bundle in self.bundles()? {
            groups.entry(bundle.name.clone()).or_default().push(bundle);
        }

        let avoid = self.reserved_names();
        for (name, bundles) in groups {
            if bundles.len() < 2 {
                continue;
            }
            let keeper = bundles
                .iter()
                .position(|bundle| {
                    self.live_matches(bundle.mode, &bundle.slug, &Part::ALL)
                        .unwrap_or(false)
                })
                .unwrap_or(0);
            let keeper = &bundles[keeper];
            for bundle in bundles.iter().filter(|bundle| bundle.slug != keeper.slug) {
                let (new_name, _) = self.unique_name_avoiding(&name, &avoid)?;
                self.write_imported_name(&bundle.slug, &new_name)?;
            }
        }
        Ok(())
    }
}

fn default_background(config: &Config) -> Option<cosmic_bg_config::Entry> {
    config.get(cosmic_bg_config::DEFAULT_BACKGROUND).ok()
}

/// The interface's neutral values, which a preset never carries.
fn keep_theme_interface(theme: &mut Theme, live: &Theme) {
    theme.active_hint = live.active_hint;
    theme.spacing = live.spacing;
    theme.corner_radii = live.corner_radii;
    theme.gaps = live.gaps;
    theme.frosted = live.frosted;
    theme.frosted_windows = live.frosted_windows;
    theme.frosted_system_interface = live.frosted_system_interface;
    theme.frosted_panel = live.frosted_panel;
    theme.frosted_applets = live.frosted_applets;
    theme.frosted_maximized_apps = live.frosted_maximized_apps;
    theme.alpha_map = live.alpha_map;
}

fn keep_builder_interface(builder: &mut ThemeBuilder, live: &ThemeBuilder) {
    builder.active_hint = live.active_hint;
    builder.spacing = live.spacing;
    builder.corner_radii = live.corner_radii;
    builder.gaps = live.gaps;
    builder.frosted = live.frosted;
    builder.frosted_windows = live.frosted_windows;
    builder.frosted_system_interface = live.frosted_system_interface;
    builder.frosted_panel = live.frosted_panel;
    builder.frosted_applets = live.frosted_applets;
    builder.frosted_maximized_apps = live.frosted_maximized_apps;
    builder.alpha_map = live.alpha_map;
}

pub(crate) fn neutral_theme(theme: &Theme) -> Theme {
    let mut base = theme.clone();
    keep_theme_interface(&mut base, &Theme::default());
    base
}

pub(crate) fn neutral_builder(builder: &ThemeBuilder) -> ThemeBuilder {
    let mut base = builder.clone();
    keep_builder_interface(&mut base, &ThemeBuilder::default());
    base
}

/// Name a config and store the appearance it is a preset of.
fn write_base(config: &Config, name: &str, theme: &Theme) -> Result<(), Error> {
    config.set(NAME_KEY, name.to_owned())?;
    config.set(BASE, neutral_theme(theme))?;
    Ok(())
}

pub(crate) fn write_builder_base(config: &Config, builder: &ThemeBuilder) -> Result<(), Error> {
    config.set(BASE, neutral_builder(builder))?;
    Ok(())
}

fn slugs_in(cosmic_dir: &Path) -> Result<Vec<String>, Error> {
    let entries = match fs::read_dir(cosmic_dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(Error::Io(cosmic_dir.to_path_buf(), err)),
    };
    Ok(entries
        .flatten()
        .filter_map(|entry| entry.file_name().to_str().map(str::to_owned))
        .filter(|id| id.ends_with(".Theme"))
        .filter_map(|id| slug_of(&id))
        .collect())
}

fn store_name(theme: &Theme, mode: Mode) -> String {
    if theme.name.is_empty() {
        mode.name().to_owned()
    } else {
        theme.name.clone()
    }
}

fn has_keys(dir: &Path) -> bool {
    fs::read_dir(dir).is_ok_and(|mut keys| keys.next().is_some())
}

/// Whether any version directory of a config holds any keys.
fn any_version_keys(dir: &Path) -> Result<bool, Error> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(Error::Io(dir.to_path_buf(), err)),
    };
    for entry in entries {
        let entry = entry.map_err(|err| Error::Io(dir.to_path_buf(), err))?;
        let path = entry.path();
        if !path.is_dir() || version_of(&path).is_none() {
            continue;
        }
        match fs::read_dir(&path) {
            Ok(mut keys) => {
                if keys.next().is_some() {
                    return Ok(true);
                }
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
            Err(err) => return Err(Error::Io(path, err)),
        }
    }
    Ok(false)
}

/// The keys that belong to the interface rather than to any look.
const INTERFACE_KEYS: [&str; 11] = [
    "active_hint",
    "spacing",
    "corner_radii",
    "gaps",
    "frosted",
    "frosted_windows",
    "frosted_system_interface",
    "frosted_panel",
    "frosted_applets",
    "frosted_maximized_apps",
    "alpha_map",
];

fn rename_base(config: &Config, name: &str) -> Result<(), Error> {
    if let Ok(mut base) = config.get::<Theme>(BASE) {
        name.clone_into(&mut base.name);
        config.set(BASE, neutral_theme(&base))?;
    }
    Ok(())
}

/// Restore one part's look keys from a stored copy.
fn restore_part(from: &Config, to: &Config, part: Part) -> Result<bool, Error> {
    match part {
        Part::Theme => {
            let Ok(mut theme) = theme_of(from) else {
                return Ok(false);
            };
            keep_theme_interface(&mut theme, &read::<Theme>(to)?);
            theme.write_entry(to)?;
            Ok(true)
        }
        Part::Builder => {
            let Ok(mut builder) = builder_of(from) else {
                return Ok(false);
            };
            keep_builder_interface(&mut builder, &read::<ThemeBuilder>(to)?);
            builder.write_entry(to)?;
            Ok(true)
        }
        Part::Background => {
            let Ok(background) = from.get::<cosmic_bg_config::Entry>(BASE) else {
                return Ok(false);
            };
            to.set(cosmic_bg_config::DEFAULT_BACKGROUND, background)?;
            Ok(true)
        }
    }
}

fn slug_of(id: &str) -> Option<String> {
    let rest = id.strip_prefix(NAME)?.strip_prefix('.')?;
    let (slug, suffix) = rest.rsplit_once('.')?;
    if slug.is_empty() || !matches!(suffix, "Theme" | "Builder" | "Background") {
        return None;
    }
    Some(slug.to_owned())
}

/// The longest slug in characters.
const MAX_SLUG: usize = 40;

/// Turn a display name into a config-safe slug.
#[must_use]
fn slugify(name: &str) -> String {
    let mut slug = String::with_capacity(name.len().min(MAX_SLUG * 4));
    let mut dashed = false;
    for ch in name.chars() {
        if ch.is_alphanumeric() {
            slug.extend(ch.to_lowercase().filter(|lower| lower.is_alphanumeric()));
            dashed = false;
        } else if !dashed && !slug.is_empty() {
            slug.push('-');
            dashed = true;
        }
    }
    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        return "theme".to_owned();
    }
    let truncated: String = slug.chars().take(MAX_SLUG).collect();
    truncated.trim_end_matches('-').to_owned()
}

/// Whether a string is already a slug.
#[must_use]
fn is_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug.chars().count() <= MAX_SLUG
        && slug.chars().all(|ch| ch.is_alphanumeric() || ch == '-')
        && slug.split('-').all(|word| !word.is_empty())
}

/// A slug for a name already taken.
fn numbered_slug(slug: &str, nth: usize) -> String {
    let suffix = format!("-{nth}");
    let keep = MAX_SLUG.saturating_sub(suffix.chars().count());
    let base: String = slug.chars().take(keep).collect();
    format!("{}{suffix}", base.trim_end_matches('-'))
}

fn read<T: CosmicConfigEntry>(config: &Config) -> Result<T, Error> {
    match T::get_entry(config) {
        Ok(value) => Ok(value),
        Err((errors, value)) => {
            if let Some(err) = errors.into_iter().find(cosmic_config::Error::is_err) {
                return Err(Error::Config(err));
            }
            Ok(value)
        }
    }
}

fn theme_has_per_key(config: &Config) -> bool {
    config.get_local::<String>(NAME_KEY).is_ok()
        && config.get_local::<bool>("is_dark").is_ok()
        && config
            .get_local::<cosmic_theme::CosmicPaletteInner>("palette")
            .is_ok()
}

fn builder_has_per_key(config: &Config) -> bool {
    config
        .get_local::<cosmic_theme::CosmicPalette>("palette")
        .is_ok()
}

/// Set only the non-interface theme keys that differ between `old` and `new`.
fn write_theme_changes(config: &Config, old: &Theme, new: &Theme) -> Result<(), Error> {
    let tx = config.transaction();
    macro_rules! changed {
        ($key:literal, $old:expr, $new:expr) => {
            if $old != $new {
                tx.set($key, $new)?;
            }
        };
    }

    changed!("name", &old.name, &new.name);
    changed!("background", old.background(false), new.background(false));
    changed!(
        "transparent_background",
        old.background(true),
        new.background(true)
    );
    changed!("primary", old.primary(false), new.primary(false));
    changed!("transparent_primary", old.primary(true), new.primary(true));
    changed!("secondary", old.secondary(false), new.secondary(false));
    changed!(
        "transparent_secondary",
        old.secondary(true),
        new.secondary(true)
    );
    changed!("button", &old.button, &new.button);
    changed!("accent", &old.accent, &new.accent);
    changed!("success", &old.success, &new.success);
    changed!("destructive", &old.destructive, &new.destructive);
    changed!("warning", &old.warning, &new.warning);
    changed!("accent_button", &old.accent_button, &new.accent_button);
    changed!("success_button", &old.success_button, &new.success_button);
    changed!(
        "destructive_button",
        &old.destructive_button,
        &new.destructive_button
    );
    changed!("warning_button", &old.warning_button, &new.warning_button);
    changed!("icon_button", &old.icon_button, &new.icon_button);
    changed!("link_button", &old.link_button, &new.link_button);
    changed!("list_button", &old.list_button, &new.list_button);
    changed!("text_button", &old.text_button, &new.text_button);
    changed!("palette", &old.palette, &new.palette);
    changed!("is_dark", old.is_dark, new.is_dark);
    changed!(
        "is_high_contrast",
        old.is_high_contrast,
        new.is_high_contrast
    );
    changed!("window_hint", &old.window_hint, &new.window_hint);
    changed!("shade", &old.shade, &new.shade);
    changed!("accent_text", &old.accent_text, &new.accent_text);
    changed!("control_tint", &old.control_tint, &new.control_tint);
    changed!("text_tint", &old.text_tint, &new.text_tint);
    changed!("icon_theme", &old.icon_theme, &new.icon_theme);
    changed!("font", &old.font, &new.font);
    changed!("mono_font", &old.mono_font, &new.mono_font);

    tx.commit()?;
    Ok(())
}

/// Set only the non-interface builder keys that differ between `old` and `new`.
fn write_builder_changes(
    config: &Config,
    old: &ThemeBuilder,
    new: &ThemeBuilder,
) -> Result<(), Error> {
    let tx = config.transaction();
    macro_rules! changed {
        ($key:literal, $old:expr, $new:expr) => {
            if $old != $new {
                tx.set($key, $new)?;
            }
        };
    }

    changed!("palette", &old.palette, &new.palette);
    changed!("neutral_tint", &old.neutral_tint, &new.neutral_tint);
    changed!("bg_color", &old.bg_color, &new.bg_color);
    changed!(
        "primary_container_bg",
        &old.primary_container_bg,
        &new.primary_container_bg
    );
    changed!(
        "secondary_container_bg",
        &old.secondary_container_bg,
        &new.secondary_container_bg
    );
    changed!("text_tint", &old.text_tint, &new.text_tint);
    changed!("accent", &old.accent, &new.accent);
    changed!("success", &old.success, &new.success);
    changed!("warning", &old.warning, &new.warning);
    changed!("destructive", &old.destructive, &new.destructive);
    changed!("window_hint", &old.window_hint, &new.window_hint);
    changed!("icon_theme", &old.icon_theme, &new.icon_theme);
    changed!("font", &old.font, &new.font);
    changed!("mono_font", &old.mono_font, &new.mono_font);

    tx.commit()?;
    Ok(())
}

/// The look a bundle's config holds.
fn theme_of(config: &Config) -> Result<Theme, Error> {
    if theme_has_per_key(config)
        && let Ok(theme) = read::<Theme>(config)
    {
        return Ok(theme);
    }
    match config.get::<Theme>(BASE) {
        Ok(base) => Ok(base),
        Err(_) => read::<Theme>(config),
    }
}

fn builder_of(config: &Config) -> Result<ThemeBuilder, Error> {
    if builder_has_per_key(config)
        && let Ok(builder) = read::<ThemeBuilder>(config)
    {
        return Ok(builder);
    }
    match config.get::<ThemeBuilder>(BASE) {
        Ok(base) => Ok(base),
        Err(_) => read::<ThemeBuilder>(config),
    }
}

fn chain_versions(config: &Config) -> Vec<u64> {
    std::iter::successors(Some(config), |config| config.previous())
        .filter_map(|config| config.user_path().filter(|path| path.is_dir()))
        .filter_map(version_of)
        .collect()
}

fn version_of(path: &Path) -> Option<u64> {
    path.file_name()?.to_str()?.strip_prefix('v')?.parse().ok()
}

/// Rename a directory into place, making the destination's own directory first.
fn rename(from: &Path, to: &Path) -> Result<(), Error> {
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent).map_err(|err| Error::Io(parent.to_path_buf(), err))?;
    }
    fs::rename(from, to).map_err(|err| Error::Io(from.to_path_buf(), err))
}

fn remove_dir(path: &Path) -> Result<(), Error> {
    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(Error::Io(path.to_path_buf(), err)),
    }
}

pub(crate) fn interface_key(key: &str) -> bool {
    INTERFACE_KEYS.contains(&key)
}
