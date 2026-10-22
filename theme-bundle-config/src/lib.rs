#![warn(missing_debug_implementations, rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

use cosmic_config::{Config, ConfigGet, ConfigSet, CosmicConfigEntry};
use cosmic_theme::{
    DARK_THEME_BUILDER_ID, DARK_THEME_ID, LIGHT_THEME_BUILDER_ID, LIGHT_THEME_ID, Theme,
    ThemeBuilder, ThemeMode,
};

pub mod transfer;

pub const NAME: &str = "com.system76.CosmicThemeBundle";
pub const MODE_ID: &str = "com.system76.CosmicTheme.Mode";
pub const BACKGROUND_VERSION: u64 = cosmic_bg_config::VERSION;

const TOOLKIT_ID: &str = "com.system76.CosmicTk";
const TOOLKIT_MIGRATION: &str = "theme-bundle-toolkit-migrated";
const BASE: &str = "base";
const NAME_KEY: &str = "name";
const THEME_VERSION: u64 = <Theme as CosmicConfigEntry>::VERSION;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Mode {
    Dark,
    Light,
}

impl Mode {
    pub const fn theme_id(self) -> &'static str {
        match self {
            Self::Dark => DARK_THEME_ID,
            Self::Light => LIGHT_THEME_ID,
        }
    }
    pub const fn builder_id(self) -> &'static str {
        match self {
            Self::Dark => DARK_THEME_BUILDER_ID,
            Self::Light => LIGHT_THEME_BUILDER_ID,
        }
    }
    pub const fn name(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }
    pub const fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }
    pub const fn from_is_dark(is_dark: bool) -> Self {
        if is_dark { Self::Dark } else { Self::Light }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Part {
    Theme,
    Builder,
    Background,
}

impl Part {
    pub const ALL: [Self; 3] = [Self::Theme, Self::Builder, Self::Background];

    /// XXX must be kept up to date
    pub const fn live_id(self, mode: Mode) -> &'static str {
        match self {
            Self::Theme => mode.theme_id(),
            Self::Builder => mode.builder_id(),
            Self::Background => cosmic_bg_config::Mode::from_is_dark(mode.is_dark()).config_name(),
        }
    }

    /// XXX must be kept up to date
    pub const fn version(self) -> u64 {
        match self {
            Self::Theme => THEME_VERSION,
            Self::Builder => <ThemeBuilder as CosmicConfigEntry>::VERSION,
            Self::Background => BACKGROUND_VERSION,
        }
    }

    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Theme => "Theme",
            Self::Builder => "Builder",
            Self::Background => "Background",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bundle {
    pub slug: String,
    pub name: String,
    pub mode: Mode,
}

/// An error from managing bundles.
#[derive(Debug)]
pub enum Error {
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
        let home = cosmic_config::get_config_dir().ok_or(Error::NoConfigDir)?;
        let cosmic_dir = home.join("cosmic");
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
        format!("{NAME}.{slug}.{}", part.suffix())
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

    pub fn system_bundle(&self, slug: &str) -> Result<Bundle, Error> {
        let config = self.system_config(slug, Part::Theme)?;
        let theme = theme_of(&config)?;
        Ok(Bundle {
            slug: slug.to_owned(),
            name: theme.name,
            mode: Mode::from_is_dark(theme.is_dark),
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
            return read::<Theme>(&self.bundle_config(slug, Part::Theme)?);
        }
        theme_of(&self.system_config(slug, Part::Theme)?)
    }

    fn system_dir(&self) -> Option<PathBuf> {
        let config = Config::system(DARK_THEME_ID, THEME_VERSION).ok()?;
        let path = config.system_path()?;
        path.parent()?.parent().map(Path::to_path_buf)
    }

    /// Copy a bundle installed in the system into the user's config.
    pub fn adopt(&self, slug: &str) -> Result<(), Error> {
        let dir = self
            .system_dir()
            .ok_or_else(|| Error::UnknownBundle(slug.to_owned()))?;
        for part in Part::ALL {
            let from = dir.join(Self::bundle_id(slug, part));
            let to = self.bundle_config_dir(slug, part);
            copy_versions(&from, &to)?;
        }
        self.materialize(slug)?;

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

    /// Write a packaged preset out as the keys of an adopted bundle.
    fn materialize(&self, slug: &str) -> Result<(), Error> {
        let source = |part| Config::system(&Self::bundle_id(slug, part), part.version());
        let theme_source = source(Part::Theme)?;
        if theme_source.get::<String>(NAME_KEY).is_ok() {
            return Ok(());
        }
        if let Ok(theme) = theme_source.get::<Theme>(BASE) {
            let config = self.bundle_config(slug, Part::Theme)?;
            write_look_keys(&config, &theme)?;
            config.set(BASE, neutral_theme(&theme))?;
        }
        if let Ok(builder) = source(Part::Builder)?.get::<ThemeBuilder>(BASE) {
            write_look_keys(&self.bundle_config(slug, Part::Builder)?, &builder)?;
        }
        Ok(())
    }

    pub fn bundle(&self, slug: &str) -> Result<Bundle, Error> {
        let dir = self.bundle_dir(slug, Part::Theme)?;
        if !has_keys(&dir) {
            return Err(Error::UnknownBundle(slug.to_owned()));
        }
        let theme = read::<Theme>(&self.bundle_config(slug, Part::Theme)?)?;
        Ok(Bundle {
            slug: slug.to_owned(),
            name: theme.name,
            mode: Mode::from_is_dark(theme.is_dark),
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
            self.preserve_live(bundle.mode)?;
        }
        self.swap_live(bundle.mode, Some(&bundle.slug))?;
        Ok(bundle)
    }

    /// Save a mode's standalone live look before another look replaces it.
    fn preserve_live(&self, mode: Mode) -> Result<(), Error> {
        let theme = read::<Theme>(&self.live_config(Part::Theme, mode)?)?;
        let name = store_name(&theme, mode);
        if self.worn_bundle(mode, &name)?.is_some() {
            return Ok(());
        }
        if self.live_matches_system(mode)? {
            return Ok(());
        }
        let avoid: Vec<String> = self
            .system_bundles()?
            .into_iter()
            .map(|bundle| bundle.name)
            .collect();
        let (name, slug) = self.unique_name_avoiding(&name, &avoid)?;
        self.store_live(&name, &slug, mode)?;
        Ok(())
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

    /// Whether a mode's live look is the one a bundle holds, key for key.
    fn live_matches(&self, mode: Mode, slug: &str, parts: &[Part]) -> Result<bool, Error> {
        for part in parts.iter().copied() {
            let live = self.live_config(part, mode)?;
            let stored = self.bundle_config(slug, part)?;
            let mut versions = chain_versions(&live);
            for version in chain_versions(&stored) {
                if !versions.contains(&version) {
                    versions.push(version);
                }
            }
            for version in versions {
                let live_dir = self.dir(part.live_id(mode), version)?;
                let stored_dir = self.dir(&Self::bundle_id(slug, part), version)?;
                let installed = Config::system(part.live_id(mode), version)
                    .ok()
                    .and_then(|config| config.system_path().map(Path::to_path_buf))
                    .unwrap_or_default();
                if reads(&live_dir, &installed)? != reads(&stored_dir, &installed)? {
                    return Ok(false);
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

    pub fn delete(&self, slug: &str) -> Result<(), Error> {
        if self.is_system(slug)? {
            return Err(Error::SystemBundle(slug.to_owned()));
        }
        match self.bundle(slug) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
        for part in Part::ALL {
            remove_dir(&self.bundle_config_dir(slug, part))?;
        }
        Ok(())
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

        write_look_keys(
            &self.live_config(Part::Theme, mode)?,
            &builder.clone().build(),
        )?;
        write_look_keys(&self.live_config(Part::Builder, mode)?, &builder)?;
        Ok(())
    }

    fn store_live(&self, name: &str, slug: &str, mode: Mode) -> Result<Bundle, Error> {
        let mut theme = read::<Theme>(&self.live_config(Part::Theme, mode)?)?;
        let builder = read::<ThemeBuilder>(&self.live_config(Part::Builder, mode)?)?;
        name.clone_into(&mut theme.name);
        for part in Part::ALL {
            let stored = self.bundle_dir(slug, part)?;
            let live = self.dir(part.live_id(mode), part.version())?;
            copy_dir(&live, &stored)?;
        }
        write_base(&self.bundle_config(slug, Part::Theme)?, name, &theme)?;
        write_builder_base(&self.bundle_config(slug, Part::Builder)?, &builder)?;
        if let Some(background) = default_background(&self.live_config(Part::Background, mode)?) {
            self.bundle_config(slug, Part::Background)?
                .set(BASE, &background)?;
        }
        for part in Part::ALL {
            self.write_installed_keys(slug, part, mode)?;
        }
        Ok(Bundle {
            slug: slug.to_owned(),
            name: name.to_owned(),
            mode,
        })
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

    fn swap_live(&self, mode: Mode, incoming: Option<&str>) -> Result<Bundle, Error> {
        let theme = read::<Theme>(&self.live_config(Part::Theme, mode)?)?;
        let name = store_name(&theme, mode);
        let worn = self.worn_bundle(mode, &name)?;
        if let Some(worn) = worn.as_ref() {
            for part in Part::ALL {
                self.write_live_into(&worn.slug, part, mode)?;
            }
        }
        let (_, scratch) = self.unique_name(".swap")?;
        for part in Part::ALL {
            self.apply_versions(part, mode, incoming, &scratch)?;
        }
        for part in Part::ALL {
            remove_dir(&self.bundle_config_dir(&scratch, part))?;
        }
        Ok(worn.unwrap_or(Bundle {
            slug: String::new(),
            name,
            mode,
        }))
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
            // The old keys stay where they are: something older may still read
            // them, and the marker is what says this has been done.
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

    fn state_dir(&self) -> PathBuf {
        xdg::BaseDirectories::new()
            .get_state_home()
            .unwrap_or_else(std::env::temp_dir)
            .join("cosmic")
    }

    fn apply_versions(
        &self,
        part: Part,
        mode: Mode,
        incoming: Option<&str>,
        scratch: &str,
    ) -> Result<(), Error> {
        let live = self.live_config(part, mode)?;
        let live_id = part.live_id(mode);
        let scratch_id = Self::bundle_id(scratch, part);
        let mut versions = chain_versions(&live);
        if let Some(incoming) = incoming {
            for version in chain_versions(&self.bundle_config(incoming, part)?) {
                if !versions.contains(&version) {
                    versions.push(version);
                }
            }
        }
        for version in versions {
            let scratch_path = self.dir(&scratch_id, version)?;
            remove_dir(&scratch_path)?;
            match incoming {
                Some(incoming) => {
                    let from = self.dir(&Self::bundle_id(incoming, part), version)?;
                    if part == Part::Background {
                        copy_dir(&from, &scratch_path)?;
                    } else {
                        let live_version = self.dir(live_id, version)?;
                        copy_keys(&live_version, &scratch_path, interface_key)?;
                        copy_keys(&from, &scratch_path, |key| !interface_key(key))?;
                    }
                }
                None => fs::create_dir_all(&scratch_path)
                    .map_err(|err| Error::Io(scratch_path.clone(), err))?,
            }
            let live_path = self.dir(live_id, version)?;
            fs::create_dir_all(&live_path).map_err(|err| Error::Io(live_path.clone(), err))?;
            exchange(&live_path, &scratch_path)?;
        }
        Ok(())
    }

    fn write_live_into(&self, slug: &str, part: Part, mode: Mode) -> Result<(), Error> {
        let live = self.live_config(part, mode)?;
        let live_id = part.live_id(mode);
        let bundle_id = Self::bundle_id(slug, part);
        let mut versions = chain_versions(&live);
        for version in chain_versions(&self.bundle_config(slug, part)?) {
            if !versions.contains(&version) {
                versions.push(version);
            }
        }
        for version in versions {
            let to = self.dir(&bundle_id, version)?;
            remove_dir(&to)?;
            copy_dir(&self.dir(live_id, version)?, &to)?;
        }
        Ok(())
    }

    /// A free name and slug for a stored look.
    fn unique_name(&self, name: &str) -> Result<(String, String), Error> {
        self.unique_name_avoiding(name, &[])
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
            let name_taken = stored.iter().any(|bundle| bundle.name == candidate)
                || avoid.iter().any(|taken| *taken == candidate);
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
            if holds_keys(&self.bundle_config_dir(slug, part))? {
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
pub(crate) fn neutral_theme(theme: &Theme) -> Theme {
    let mut base = theme.clone();
    let neutral = Theme::default();
    base.active_hint = neutral.active_hint;
    base.spacing = neutral.spacing;
    base.corner_radii = neutral.corner_radii;
    base.gaps = neutral.gaps;
    base.frosted = neutral.frosted;
    base.frosted_windows = neutral.frosted_windows;
    base.frosted_system_interface = neutral.frosted_system_interface;
    base.frosted_panel = neutral.frosted_panel;
    base.frosted_applets = neutral.frosted_applets;
    base.frosted_maximized_apps = neutral.frosted_maximized_apps;
    base.alpha_map = neutral.alpha_map;
    base
}

pub(crate) fn neutral_builder(builder: &ThemeBuilder) -> ThemeBuilder {
    let mut base = builder.clone();
    let neutral = ThemeBuilder::default();
    base.active_hint = neutral.active_hint;
    base.spacing = neutral.spacing;
    base.corner_radii = neutral.corner_radii;
    base.gaps = neutral.gaps;
    base.frosted = neutral.frosted;
    base.frosted_windows = neutral.frosted_windows;
    base.frosted_system_interface = neutral.frosted_system_interface;
    base.frosted_panel = neutral.frosted_panel;
    base.frosted_applets = neutral.frosted_applets;
    base.frosted_maximized_apps = neutral.frosted_maximized_apps;
    base.alpha_map = neutral.alpha_map;
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

fn copy_versions(from: &Path, to: &Path) -> Result<(), Error> {
    let entries = match fs::read_dir(from) {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(err) => return Err(Error::Io(from.to_path_buf(), err)),
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && version_of(&path).is_some() {
            copy_dir(&path, &to.join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// The keys a version directory stores, over the installed ones it reads through.
fn reads(dir: &Path, installed: &Path) -> Result<BTreeMap<String, Vec<u8>>, Error> {
    let mut keys = keys_of(installed)?;
    keys.extend(keys_of(dir)?);
    Ok(keys)
}

fn holds_keys(dir: &Path) -> Result<bool, Error> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(Error::Io(dir.to_path_buf(), err)),
    };
    for entry in entries {
        let entry = entry.map_err(|err| Error::Io(dir.to_path_buf(), err))?;
        let path = entry.path();
        if path.is_dir() && version_of(&path).is_some() && !keys_of(&path)?.is_empty() {
            return Ok(true);
        }
    }
    Ok(false)
}

/// The keys a version directory holds, minus the preset, name, and interface keys.
fn keys_of(dir: &Path) -> Result<BTreeMap<String, Vec<u8>>, Error> {
    let mut keys = BTreeMap::new();
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(keys),
        Err(err) => return Err(Error::Io(dir.to_path_buf(), err)),
    };
    for entry in entries {
        let entry = entry.map_err(|err| Error::Io(dir.to_path_buf(), err))?;
        let name = entry.file_name();
        let Some(key) = name.to_str() else {
            continue;
        };
        if key == BASE || key == NAME_KEY || INTERFACE_KEYS.contains(&key) || is_staging(key) {
            continue;
        }
        let path = entry.path();
        if !entry
            .file_type()
            .map_err(|err| Error::Io(path.clone(), err))?
            .is_file()
        {
            continue;
        }
        keys.insert(
            key.to_owned(),
            fs::read(&path).map_err(|err| Error::Io(path, err))?,
        );
    }
    Ok(keys)
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

/// The name a key is written under until it lands.
const STAGING: &str = ".atomicwrite";

fn is_staging(key: &str) -> bool {
    key.starts_with(STAGING)
}

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
            if let Ok(theme) = from.get::<Theme>(BASE) {
                write_look_keys(to, &theme)?;
                return Ok(true);
            }
        }
        Part::Builder => {
            if let Ok(builder) = from.get::<ThemeBuilder>(BASE) {
                write_look_keys(to, &builder)?;
                return Ok(true);
            }
        }
        Part::Background => {
            if let Ok(background) = from.get::<cosmic_bg_config::Entry>(BASE) {
                to.set(cosmic_bg_config::DEFAULT_BACKGROUND, background)?;
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// Write an entry's look keys, leaving the interface's keys as they are.
fn write_look_keys<T: CosmicConfigEntry>(config: &Config, value: &T) -> Result<(), Error> {
    let Some(dir) = config.user_path() else {
        return Err(Error::NoConfigDir);
    };
    let temp = tempfile::tempdir().map_err(|err| Error::Io(std::env::temp_dir(), err))?;
    let scratch = Config::with_custom_path("look", T::VERSION, temp.path().to_path_buf())?;
    value.write_entry(&scratch)?;
    let Some(from) = scratch.user_path() else {
        return Err(Error::NoConfigDir);
    };
    let entries = fs::read_dir(from).map_err(|err| Error::Io(from.to_path_buf(), err))?;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(key) = name.to_str() else {
            continue;
        };
        if interface_key(key) || is_staging(key) {
            continue;
        }
        let target = dir.join(&name);
        let staging = dir.join(format!("{STAGING}.look-{key}"));
        fs::copy(entry.path(), &staging).map_err(|err| Error::Io(staging.clone(), err))?;
        if let Err(err) = fs::rename(&staging, &target) {
            let _ = fs::remove_file(&staging);
            return Err(Error::Io(target, err));
        }
    }
    Ok(())
}

fn slug_of(id: &str) -> Option<String> {
    let rest = id.strip_prefix(NAME)?.strip_prefix('.')?;
    let (slug, suffix) = rest.rsplit_once('.')?;
    if slug.is_empty() || !matches!(suffix, "Theme" | "Builder" | "Background") {
        return None;
    }
    Some(slug.to_owned())
}

/// The longest slug in characters: 48 four-byte characters and the id's 42 bytes fit 255.
const MAX_SLUG: usize = 48;

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

/// The look a bundle's config holds, which a packaged preset names under `base`.
fn theme_of(config: &Config) -> Result<Theme, Error> {
    match config.get::<Theme>(BASE) {
        Ok(base) => Ok(base),
        Err(_) => read::<Theme>(config),
    }
}

const SCRATCH: &str = ".theme-bundle-swap";

fn chain_versions(config: &Config) -> Vec<u64> {
    std::iter::successors(Some(config), |config| config.previous())
        .filter_map(|config| config.user_path().filter(|path| path.is_dir()))
        .filter_map(version_of)
        .collect()
}

fn version_of(path: &Path) -> Option<u64> {
    path.file_name()?.to_str()?.strip_prefix('v')?.parse().ok()
}

/// Swap two directories in one step, or in three renames.
fn exchange(a: &Path, b: &Path) -> Result<(), Error> {
    #[cfg(target_os = "linux")]
    match rustix::fs::renameat_with(
        rustix::fs::CWD,
        a,
        rustix::fs::CWD,
        b,
        rustix::fs::RenameFlags::EXCHANGE,
    ) {
        Ok(()) => return Ok(()),
        Err(rustix::io::Errno::INVAL | rustix::io::Errno::NOSYS | rustix::io::Errno::OPNOTSUPP) => {
        }
        Err(err) => return Err(Error::Io(a.to_path_buf(), io::Error::from(err))),
    }
    let scratch = a.with_file_name(SCRATCH);
    remove_dir(&scratch)?;
    rename(a, &scratch)?;
    rename(b, a)?;
    rename(&scratch, b)
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

fn copy_keys(from: &Path, to: &Path, keep: impl Fn(&str) -> bool) -> Result<(), Error> {
    fs::create_dir_all(to).map_err(|err| Error::Io(to.to_path_buf(), err))?;
    let entries = match fs::read_dir(from) {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(err) => return Err(Error::Io(from.to_path_buf(), err)),
    };
    for entry in entries {
        let entry = entry.map_err(|err| Error::Io(from.to_path_buf(), err))?;
        let path = entry.path();
        let name = entry.file_name();
        let Some(key) = name.to_str() else {
            continue;
        };
        if !path.is_file() || is_staging(key) || !keep(key) {
            continue;
        }
        let target = to.join(&name);
        fs::copy(&path, &target).map_err(|err| Error::Io(target, err))?;
    }
    Ok(())
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), Error> {
    fs::create_dir_all(to).map_err(|err| Error::Io(to.to_path_buf(), err))?;
    let entries = if from.is_dir() {
        fs::read_dir(from).map_err(|err| Error::Io(from.to_path_buf(), err))?
    } else {
        return Ok(());
    };
    for entry in entries {
        let entry = entry.map_err(|err| Error::Io(from.to_path_buf(), err))?;
        let path = entry.path();
        let name = entry.file_name();
        let Some(key) = name.to_str() else {
            continue;
        };
        if !is_staging(key)
            && !interface_key(key)
            && entry
                .file_type()
                .map_err(|err| Error::Io(path.clone(), err))?
                .is_file()
        {
            let target = to.join(name);
            fs::copy(&path, &target).map_err(|err| Error::Io(target, err))?;
        }
    }
    Ok(())
}
