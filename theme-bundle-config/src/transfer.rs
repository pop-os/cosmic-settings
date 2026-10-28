//! A look written out as one zip, and read back in.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use appstream::xmltree::{Element, EmitterConfig, XMLNode};
use cosmic_config::{Config, ConfigSet};
use cosmic_theme::{DEFAULT_FONT_FAMILY, DEFAULT_ICON_THEME, FontConfig, Theme, ThemeBuilder};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::{
    Error, Mode, NAME_KEY, Part, Store, interface_key, is_slug, neutral_builder, neutral_theme,
    read, rename, rename_base, version_of,
};

const METAINFO_SUFFIX: &str = ".metainfo.xml";
pub const VERSION: u64 = 1;
pub const CONFIG_ROOT: &str = "config";
pub const DATA_ROOT: &str = "data";

const BUNDLE_ID: &str = "com.system76.CosmicThemeBundle.";
const META_FORMAT: &str = "x-cosmic-format";
const META_SLUG: &str = "x-cosmic-slug";
const META_MODE: &str = "x-cosmic-mode";
const META_PARTS: &str = "x-cosmic-parts";
const META_ASSETS: &str = "x-cosmic-assets";

const OPTIONAL_KEYS: [&str; 3] = ["icon_theme", "font", "mono_font"];

const MAX_DEPTH: usize = 64;
const MAX_PATH: usize = 1024;

/// A bundle written out as one file.
#[derive(Debug, Clone, PartialEq)]
pub struct Manifest {
    pub version: u64,
    /// The slug the trees inside are named with.
    pub slug: String,
    pub name: String,
    pub mode: Mode,
    /// The parts carried.
    pub parts: Vec<Part>,
    pub assets: Vec<Asset>,
}

/// A file the look needs, and what it is for.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Asset {
    pub kind: AssetKind,
    /// Where it sits in the archive.
    pub path: String,
    /// What the look calls it.
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AssetKind {
    Wallpaper,
    IconTheme,
    Font,
}

impl Manifest {
    fn to_xml(&self) -> Result<Vec<u8>, Error> {
        let mut component = Element::new("component");
        component
            .attributes
            .insert("type".to_owned(), "generic".to_owned());
        component.children.push(XMLNode::Element(text_element(
            "id",
            &format!("{BUNDLE_ID}{}", self.slug),
        )));
        component
            .children
            .push(XMLNode::Element(text_element("name", &self.name)));
        component.children.push(XMLNode::Element(text_element(
            "summary",
            "Cosmic theme bundle",
        )));
        let mut description = Element::new("description");
        description.children.push(XMLNode::Element(text_element(
            "p",
            "A COSMIC theme bundle carrying the complete look for one mode: the theme, its builder settings, and the background it is shown with.",
        )));
        component.children.push(XMLNode::Element(description));
        component.children.push(XMLNode::Element(text_element(
            "metadata_license",
            "CC0-1.0",
        )));

        let mut icon = text_element("icon", "preferences-other-symbolic");
        icon.attributes
            .insert("type".to_owned(), "stock".to_owned());
        component.children.push(XMLNode::Element(icon));

        let assets = ron::ser::to_string(&self.assets)
            .map_err(|err| Error::Archive(format!("assets: {err}")))?;
        let parts: Vec<&str> = self.parts.iter().map(|part| part.suffix()).collect();
        let mut metadata = Element::new("custom");
        for (key, value) in [
            (META_FORMAT, self.version.to_string()),
            (META_SLUG, self.slug.clone()),
            (META_MODE, self.mode.name().to_owned()),
            (META_PARTS, parts.join(",")),
            (META_ASSETS, assets),
        ] {
            metadata
                .children
                .push(XMLNode::Element(metadata_value(key, &value)));
        }
        component.children.push(XMLNode::Element(metadata));

        let mut bytes = Vec::new();
        component
            .write_with_config(&mut bytes, EmitterConfig::new().perform_indent(true))
            .map_err(|err| Error::Archive(format!("appstream: {err}")))?;
        Ok(bytes)
    }

    fn from_xml(bytes: &[u8]) -> Result<Self, Error> {
        let element =
            Element::parse(bytes).map_err(|err| Error::Archive(format!("appstream: {err}")))?;
        let component = appstream::Component::try_from(&element)
            .map_err(|err| Error::Archive(format!("appstream: {err}")))?;
        let custom = element.get_child("custom");
        let value = |key: &str| -> Option<String> {
            let custom = custom?;
            for child in &custom.children {
                let XMLNode::Element(value) = child else {
                    continue;
                };
                if value.attributes.get("key").map(String::as_str) != Some(key) {
                    continue;
                }
                return value.get_text().map(|text| text.to_string());
            }
            None
        };

        let version = value(META_FORMAT)
            .ok_or_else(|| Error::Archive("the manifest names no format version".to_owned()))?
            .parse()
            .map_err(|_| Error::Archive("the manifest has an invalid format version".to_owned()))?;
        let slug = value(META_SLUG)
            .or_else(|| component.id.0.strip_prefix(BUNDLE_ID).map(str::to_owned))
            .filter(|slug| !slug.is_empty())
            .ok_or_else(|| Error::Archive("the manifest names no bundle slug".to_owned()))?;
        let name = component
            .name
            .get_default()
            .cloned()
            .unwrap_or_else(|| slug.clone());
        let mode = match value(META_MODE).as_deref() {
            Some("dark") => Mode::Dark,
            Some("light") => Mode::Light,
            _ => {
                return Err(Error::Archive(
                    "the manifest names no bundle mode".to_owned(),
                ));
            }
        };

        let mut parts = Vec::new();
        let stored_parts = value(META_PARTS).unwrap_or_default();
        for part in stored_parts.split(',') {
            parts.push(match part {
                "Theme" => Part::Theme,
                "Builder" => Part::Builder,
                "Background" => Part::Background,
                _ => {
                    return Err(Error::Archive(format!(
                        "the manifest names an unknown part: {part}"
                    )));
                }
            });
        }
        if parts.is_empty() {
            return Err(Error::Archive("the manifest names no parts".to_owned()));
        }

        let assets = match value(META_ASSETS) {
            Some(text) => {
                ron::from_str(&text).map_err(|err| Error::Archive(format!("assets: {err}")))?
            }
            None => Vec::new(),
        };

        Ok(Self {
            version,
            slug,
            name,
            mode,
            parts,
            assets,
        })
    }
}

fn text_element(name: &str, text: &str) -> Element {
    let mut element = Element::new(name);
    element.children.push(XMLNode::Text(text.to_owned()));
    element
}

fn metadata_value(key: &str, value: &str) -> Element {
    let mut element = Element::new("value");
    element.attributes.insert("key".to_owned(), key.to_owned());
    element.children.push(XMLNode::Text(value.to_owned()));
    element
}

impl Store {
    pub fn export_live_archive(&self, mode: Mode, path: &Path) -> Result<Manifest, Error> {
        let theme = read::<Theme>(&self.live_config(Part::Theme, mode)?)?;
        let default_name = crate::store_name(&theme, mode);
        let name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .filter(|stem| !stem.is_empty())
            .map_or(default_name, str::to_owned);
        let slug = crate::slugify(&name);

        self.export_tree(
            |part| Self::bundle_id(&slug, part),
            |part| part.live_id(mode).to_owned(),
            &name,
            &slug,
            mode,
            &theme,
            path,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn export_tree(
        &self,
        id: impl Fn(Part) -> String,
        source_id: impl Fn(Part) -> String,
        name: &str,
        slug: &str,
        mode: Mode,
        theme: &Theme,
        path: &Path,
    ) -> Result<Manifest, Error> {
        if !is_slug(slug) {
            return Err(Error::Slug(slug.to_owned()));
        }
        let mut assets = Assets {
            slug: slug.to_owned(),
            ..Assets::default()
        };

        let mut parts = Vec::new();
        let mut configs = Vec::new();
        for part in Part::ALL {
            let id = id(part);
            let source = source_id(part);
            let versions = versions_in(&self.cosmic_dir.join(&source))?;

            let Some((version, dir)) = versions
                .into_iter()
                .find(|(version, _)| *version == part.version())
            else {
                continue;
            };
            parts.push(part);

            let exported_base: Option<Vec<u8>> = match part {
                Part::Theme => Some(serialize_base(&neutral_theme(theme))?),
                Part::Builder => {
                    let builder = read::<ThemeBuilder>(&self.config(&source, part.version())?)?;
                    Some(serialize_base(&neutral_builder(&builder))?)
                }
                Part::Background => None,
            };

            let keys = key_files(&dir)?;

            let outputs: Vec<String> = keys
                .iter()
                .find(|(key, _)| key == "backgrounds")
                .and_then(|(_, bytes)| ron::from_str(String::from_utf8_lossy(bytes).as_ref()).ok())
                .unwrap_or_default();

            let mut exported_all = None;
            let mut background_base = None;
            let mut output_backgrounds = Vec::new();

            for (key, bytes) in keys {
                if interface_key(&key) {
                    continue;
                }
                if part == Part::Theme && is_default_key(&key, &bytes) {
                    continue;
                }
                if part == Part::Background {
                    if key == "same-on-all" || key == "backgrounds" {
                        continue;
                    }
                    if key == "base" {
                        background_base = Some(bytes);
                        continue;
                    }
                    if let Some(output) = key.strip_prefix("output.") {
                        if let Ok(entry) = ron::from_str::<cosmic_bg_config::Entry>(
                            &String::from_utf8_lossy(&bytes),
                        ) {
                            output_backgrounds.push((output.to_owned(), entry));
                        }
                        continue;
                    }
                }
                if key == "base" {
                    continue;
                }
                let bytes = match part {
                    Part::Background => {
                        let rewritten = assets.background(&key, bytes)?;
                        if key == "all" {
                            exported_all = Some(rewritten.clone());
                        }
                        rewritten
                    }
                    _ => bytes,
                };
                configs.push((format!("{CONFIG_ROOT}/cosmic/{id}/v{version}/{key}"), bytes));
            }

            let mut background = exported_all;
            if part == Part::Background
                && background.is_none()
                && let Some(bytes) = background_base
            {
                let bytes = assets.background("base", bytes)?;
                configs.push((
                    format!("{CONFIG_ROOT}/cosmic/{id}/v{version}/all"),
                    bytes.clone(),
                ));
                background = Some(bytes);
            }

            if part == Part::Background
                && background.is_none()
                && let Some((output, entry)) = output_background(&outputs, &output_backgrounds)
            {
                let output = output.clone();
                let bytes = assets.background_entry("all", entry.clone())?;
                tracing::warn!(%output, "exporting an output background as the default");
                configs.push((
                    format!("{CONFIG_ROOT}/cosmic/{id}/v{version}/all"),
                    bytes.clone(),
                ));
                background = Some(bytes);
            }

            if part == Part::Background && background.is_some() {
                let bytes = ron::ser::to_string(&true)
                    .map(String::into_bytes)
                    .map_err(|err| Error::Archive(format!("same-on-all: {err}")))?;
                configs.push((
                    format!("{CONFIG_ROOT}/cosmic/{id}/v{version}/same-on-all"),
                    bytes,
                ));
            }

            let base = match part {
                Part::Background => background,
                _ => exported_base.clone(),
            };

            if let Some(base) = base {
                configs.push((format!("{CONFIG_ROOT}/cosmic/{id}/v{version}/base"), base));
            }
        }

        if !theme.icon_theme.is_empty() && theme.icon_theme != DEFAULT_ICON_THEME {
            assets.icon_theme(&theme.icon_theme);
        }
        if theme.font.family != DEFAULT_FONT_FAMILY
            || theme.mono_font.family != cosmic_theme::DEFAULT_MONOSPACE_FONT_FAMILY
        {
            let index = FontIndex::new();
            assets.font(&index, &theme.font, DEFAULT_FONT_FAMILY);
            assets.font(
                &index,
                &theme.mono_font,
                cosmic_theme::DEFAULT_MONOSPACE_FONT_FAMILY,
            );
        }

        let manifest = Manifest {
            version: VERSION,
            slug: slug.to_owned(),
            name: name.to_owned(),
            mode,
            parts,
            assets: assets.records.clone(),
        };
        write_archive(path, &manifest, &configs, &assets.files)
    }

    pub fn import_archive(&self, path: &Path, name: Option<&str>) -> Result<crate::Bundle, Error> {
        let file = File::open(path).map_err(|err| Error::Io(path.to_path_buf(), err))?;
        let mut archive = ZipArchive::new(file)?;
        let manifest = read_manifest(&mut archive)?;
        if manifest.version != VERSION {
            return Err(Error::Archive(format!(
                "the archive is format v{}, this build supports v{VERSION}",
                manifest.version
            )));
        }
        if !is_slug(&manifest.slug) {
            return Err(Error::Archive(format!(
                "the archive names a slug a bundle cannot have: {:.64}",
                manifest.slug
            )));
        }

        let data = xdg::BaseDirectories::new()
            .get_data_home()
            .ok_or(Error::NoDataDir)?;
        fs::create_dir_all(&data).map_err(|err| Error::Io(data.clone(), err))?;
        let temp = tempfile::Builder::new()
            .prefix(".cosmic-theme-import-")
            .tempdir_in(&data)
            .map_err(|err| Error::Io(data.clone(), err))?;
        extract(&mut archive, temp.path())?;

        let source_root = temp.path().join(CONFIG_ROOT);
        let source = source_root.join("cosmic");
        let theme = source.join(Self::bundle_id(&manifest.slug, Part::Theme));
        if source_version(&theme, Part::Theme.version())?.is_none() {
            return Err(Error::Archive("the archive carries no theme".to_owned()));
        }

        let avoid = self.reserved_names();
        let (name, slug) =
            self.unique_name_avoiding(name.unwrap_or(manifest.name.as_str()), &avoid)?;
        for part in Part::ALL {
            let id = Self::bundle_id(&manifest.slug, part);
            let Some(version) = source_version(&source.join(&id), part.version())? else {
                continue;
            };
            let from = Config::with_custom_path(&id, version, source_root.clone())?;
            let to = self.bundle_config(&slug, part)?;
            self.copy_part(&from, &to, part)?;
        }

        let wallpapers = temp
            .path()
            .join(DATA_ROOT)
            .join("cosmic")
            .join(&manifest.slug)
            .join("wallpapers");
        install_wallpapers(
            &wallpapers,
            &data.join("cosmic").join(&slug).join("wallpapers"),
        )?;
        rewrite_backgrounds(self, &slug, &manifest.slug, &data)?;

        for root in ["icons", "fonts"] {
            let dir = temp.path().join(DATA_ROOT).join(root);
            let Ok(entries) = fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let from = entry.path();
                if !from.is_dir() {
                    continue;
                }
                let to = data.join(root).join(entry.file_name());
                if to.exists() {
                    continue;
                }
                rename(&from, &to)?;
            }
        }

        self.write_imported_name(&slug, &name)?;

        match self.bundle(&slug) {
            Ok(bundle) => Ok(bundle),
            Err(err) => {
                for part in Part::ALL {
                    let _ = crate::remove_dir(&self.bundle_config_dir(&slug, part));
                }
                Err(err)
            }
        }
    }

    /// Name a copied-in or renamed bundle.
    pub(crate) fn write_imported_name(&self, slug: &str, name: &str) -> Result<(), Error> {
        let config = self.bundle_config(slug, Part::Theme)?;
        config.set(NAME_KEY, name.to_owned())?;
        rename_base(&config, name)?;
        Ok(())
    }
}

/// The files a bundle carries, and where they go.
#[derive(Debug, Default)]
struct Assets {
    slug: String,
    /// Source files and their archive paths.
    files: BTreeMap<PathBuf, String>,
    /// What the manifest lists.
    records: Vec<Asset>,
    /// The archive paths already given out.
    taken: BTreeSet<String>,
}

impl Assets {
    fn place(&mut self, source: &Path, wanted: String) -> String {
        if let Some(placed) = self.files.get(source) {
            return placed.clone();
        }
        let mut placed = wanted.clone();
        let mut nth = 2;
        while self.taken.contains(&placed) {
            placed = numbered(&wanted, nth);
            nth += 1;
        }
        if nth > 2 {
            tracing::warn!(
                source = %source.display(),
                wanted = %wanted,
                placed = %placed,
                "two assets want the same archive path",
            );
        }
        self.taken.insert(placed.clone());
        self.files.insert(source.to_path_buf(), placed.clone());
        placed
    }

    fn carry_tree(&mut self, dir: &Path, root: &str) {
        let mut sources = Vec::new();
        collect(dir, &mut sources);
        sources.sort();
        for (source, rel) in sources {
            self.place(&source, format!("{root}/{rel}"));
        }
    }

    fn background(&mut self, key: &str, bytes: Vec<u8>) -> Result<Vec<u8>, Error> {
        let Ok(entry) = ron::from_str::<cosmic_bg_config::Entry>(&String::from_utf8_lossy(&bytes))
        else {
            return Ok(bytes);
        };
        self.background_entry(key, entry)
    }

    fn background_entry(
        &mut self,
        key: &str,
        mut entry: cosmic_bg_config::Entry,
    ) -> Result<Vec<u8>, Error> {
        entry.output = "all".to_owned();
        if let cosmic_bg_config::Source::Path(path) = &entry.source {
            if let Some(place) = self.wallpaper(path) {
                entry.source = cosmic_bg_config::Source::Path(PathBuf::from(place));
            } else {
                tracing::warn!(
                    key,
                    wallpaper = %path.display(),
                    "a wallpaper could not be carried"
                );
            }
        }
        ron::ser::to_string_pretty(&entry, ron::ser::PrettyConfig::default())
            .map(String::into_bytes)
            .map_err(|err| Error::Archive(format!("background: {err}")))
    }

    fn wallpaper(&mut self, path: &Path) -> Option<String> {
        let root = format!("{DATA_ROOT}/cosmic/{}/wallpapers", self.slug);
        if path.is_file() {
            let name = path.file_name()?.to_str()?;
            let place = self.place(path, format!("{root}/{name}"));
            self.records.push(Asset {
                kind: AssetKind::Wallpaper,
                path: place.clone(),
                name: path.display().to_string(),
            });
            return Some(place);
        }
        if !path.is_dir() {
            return None;
        }
        self.carry_tree(path, &root);
        self.records.push(Asset {
            kind: AssetKind::Wallpaper,
            path: root.clone(),
            name: path.display().to_string(),
        });
        Some(root)
    }

    fn icon_theme(&mut self, name: &str) {
        let Some(dir) = icon_theme_dir(name) else {
            return;
        };
        let root = format!("{DATA_ROOT}/icons/{name}");
        self.carry_tree(&dir, &root);
        self.records.push(Asset {
            kind: AssetKind::IconTheme,
            path: root,
            name: name.to_owned(),
        });
    }

    fn font(&mut self, index: &FontIndex, config: &FontConfig, default: &str) {
        if config.family == default {
            return;
        }
        let files = index.family(&config.family);
        if files.is_empty() {
            return;
        }
        let root = format!("{DATA_ROOT}/fonts/{}", crate::slugify(&config.family));
        for file in files {
            let Some(name) = file.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            self.place(&file, format!("{root}/{name}"));
        }
        self.records.push(Asset {
            kind: AssetKind::Font,
            path: root,
            name: config.family.clone(),
        });
    }
}

/// The installed faces, by family.
#[derive(Debug)]
struct FontIndex(fontdb::Database);

impl FontIndex {
    fn new() -> Self {
        let mut database = fontdb::Database::new();
        database.load_system_fonts();
        Self(database)
    }

    /// Every file of a family.
    fn family(&self, family: &str) -> Vec<PathBuf> {
        let mut files = BTreeSet::new();
        for face in self.0.faces() {
            let named = face
                .families
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case(family));
            if !named {
                continue;
            }
            if let fontdb::Source::File(path) = &face.source
                && path.is_file()
            {
                files.insert(path.clone());
            }
        }
        files.into_iter().collect()
    }
}

fn is_default_key(key: &str, bytes: &[u8]) -> bool {
    if !OPTIONAL_KEYS.contains(&key) {
        return false;
    }
    let bytes = String::from_utf8_lossy(bytes);
    match key {
        "icon_theme" => {
            ron::from_str::<String>(&bytes).is_ok_and(|name| name == DEFAULT_ICON_THEME)
        }
        "font" => {
            ron::from_str::<FontConfig>(&bytes).is_ok_and(|font| font == FontConfig::default())
        }
        "mono_font" => {
            ron::from_str::<FontConfig>(&bytes).is_ok_and(|font| font == FontConfig::monospace())
        }
        _ => false,
    }
}

/// Pick the per-output background to carry as the bundle's default.
fn output_background<'a>(
    outputs: &[String],
    backgrounds: &'a [(String, cosmic_bg_config::Entry)],
) -> Option<&'a (String, cosmic_bg_config::Entry)> {
    outputs
        .iter()
        .find_map(|output| backgrounds.iter().find(|(name, _)| name == output))
        .or_else(|| backgrounds.first())
}

/// Write the manifest, the config trees, and the files the look needs.
fn write_archive(
    path: &Path,
    manifest: &Manifest,
    configs: &[(String, Vec<u8>)],
    files: &BTreeMap<PathBuf, String>,
) -> Result<Manifest, Error> {
    let file = File::create(path).map_err(|err| Error::Io(path.to_path_buf(), err))?;
    let mut zip = ZipWriter::new(file);
    let text = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);
    let binary = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o644);

    let written = (|| {
        zip.start_file(
            format!("{BUNDLE_ID}{}{METAINFO_SUFFIX}", manifest.slug),
            text,
        )?;
        zip.write_all(&manifest.to_xml()?)
            .map_err(|err| Error::Io(path.to_path_buf(), err))?;
        for (name, bytes) in configs {
            zip.start_file(name.as_str(), text)?;
            zip.write_all(bytes)
                .map_err(|err| Error::Io(path.to_path_buf(), err))?;
        }
        for (source, name) in files {
            let mut from = File::open(source).map_err(|err| Error::Io(source.clone(), err))?;
            zip.start_file(name.as_str(), binary)?;
            io::copy(&mut from, &mut zip).map_err(|err| Error::Io(source.clone(), err))?;
        }
        zip.finish()?;
        Ok::<(), Error>(())
    })();
    if let Err(err) = written {
        let _ = fs::remove_file(path);
        return Err(err);
    }
    Ok(manifest.clone())
}

/// The name an archive says its look carries.
pub fn archive_name(path: &Path) -> Result<String, Error> {
    let file = File::open(path).map_err(|err| Error::Io(path.to_path_buf(), err))?;
    let mut archive = ZipArchive::new(file)?;
    Ok(read_manifest(&mut archive)?.name)
}

fn read_manifest(archive: &mut ZipArchive<File>) -> Result<Manifest, Error> {
    let name = archive
        .file_names()
        .find(|name| !name.contains('/') && name.ends_with(METAINFO_SUFFIX))
        .map(str::to_owned)
        .ok_or_else(|| {
            Error::Archive(format!(
                "the archive holds no root *{METAINFO_SUFFIX} metadata file"
            ))
        })?;
    let mut entry = archive.by_name(&name)?;
    let mut bytes = Vec::new();
    entry
        .by_ref()
        .take(1_000_000)
        .read_to_end(&mut bytes)
        .map_err(|err| Error::Io(PathBuf::from(name), err))?;
    Manifest::from_xml(&bytes)
}

fn extract(archive: &mut ZipArchive<File>, into: &Path) -> Result<(), Error> {
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        let Some(name) = entry.enclosed_name() else {
            return Err(Error::Archive(format!(
                "the archive holds a path that leaves it: {}",
                entry.name()
            )));
        };
        if entry.is_symlink() {
            return Err(Error::Archive(format!(
                "the archive holds a symlink, which is not accepted: {}",
                entry.name()
            )));
        }
        let depth = name.components().count();
        let length = name.as_os_str().len();
        if depth > MAX_DEPTH || length > MAX_PATH {
            return Err(Error::Archive(format!(
                "the archive holds a path that is too deep or too long: {depth} deep, {length} bytes"
            )));
        }
        let target = into.join(&name);
        if entry.is_dir() {
            fs::create_dir_all(&target).map_err(|err| Error::Io(target, err))?;
            continue;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|err| Error::Io(parent.to_path_buf(), err))?;
        }
        let mut out = File::create(&target).map_err(|err| Error::Io(target, err))?;
        io::copy(&mut entry, &mut out).map_err(|err| Error::Io(into.to_path_buf(), err))?;
    }
    Ok(())
}

/// Point a copied-in bundle's backgrounds at its installed wallpapers.
fn rewrite_backgrounds(
    store: &Store,
    slug: &str,
    imported: &str,
    data: &Path,
) -> Result<(), Error> {
    let id = crate::bundle_id(slug, Part::Background);
    for (version, dir) in versions_in(&store.bundle_config_dir(slug, Part::Background))? {
        let config = Config::new(&id, version)?;
        for (key, bytes) in key_files(&dir)? {
            let Ok(mut entry) =
                ron::from_str::<cosmic_bg_config::Entry>(&String::from_utf8_lossy(&bytes))
            else {
                continue;
            };
            let cosmic_bg_config::Source::Path(path) = &entry.source else {
                continue;
            };
            let Some(installed) = installed_path(path, imported, slug, data) else {
                continue;
            };
            entry.source = cosmic_bg_config::Source::Path(installed);
            config.set(&key, &entry)?;
        }
    }
    Ok(())
}

/// Where an archive path lands once installed.
fn installed_path(archive: &Path, imported: &str, slug: &str, data: &Path) -> Option<PathBuf> {
    let rest = archive.strip_prefix(DATA_ROOT).ok()?;
    let old = Path::new("cosmic").join(imported);
    let rest = match rest.strip_prefix(&old) {
        Ok(rel) if rel.as_os_str().is_empty() => Path::new("cosmic").join(slug),
        Ok(rel) => Path::new("cosmic").join(slug).join(rel),
        Err(_) => rest.to_path_buf(),
    };
    Some(data.join(rest))
}

fn serialize_base<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default())
        .map(String::into_bytes)
        .map_err(|err| Error::Archive(format!("base: {err}")))
}

/// The archive path a name is given if another file already holds it.
fn numbered(wanted: &str, nth: u32) -> String {
    let path = Path::new(wanted);
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return format!("{wanted}-{nth}");
    };
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(name);
    let numbered = match path.extension().and_then(|extension| extension.to_str()) {
        Some(extension) if !extension.is_empty() => format!("{stem}-{nth}.{extension}"),
        _ => format!("{stem}-{nth}"),
    };
    path.with_file_name(&numbered)
        .to_str()
        .map_or_else(|| format!("{wanted}-{nth}"), str::to_owned)
}

fn collect(dir: &Path, out: &mut Vec<(PathBuf, String)>) {
    for entry in walkdir::WalkDir::new(dir).follow_links(true) {
        let Ok(entry) = entry else {
            continue;
        };
        if !entry.file_type().is_file() {
            continue;
        }
        let Ok(rel) = entry.path().strip_prefix(dir) else {
            continue;
        };
        let Some(rel) = rel.to_str() else {
            continue;
        };
        out.push((entry.path().to_path_buf(), rel.to_owned()));
    }
}

fn icon_theme_dir(name: &str) -> Option<PathBuf> {
    data_dirs()
        .into_iter()
        .map(|dir| dir.join("icons").join(name))
        .find(|dir| dir.join("index.theme").is_file())
}

fn data_dirs() -> Vec<PathBuf> {
    let base = xdg::BaseDirectories::new();
    base.get_data_home()
        .into_iter()
        .chain(base.get_data_dirs())
        .collect()
}

fn source_version(dir: &Path, current: u64) -> Result<Option<u64>, Error> {
    let versions = versions_in(dir)?;
    if versions.iter().any(|(version, _)| *version == current) {
        return Ok(Some(current));
    }
    Ok(versions.into_iter().map(|(version, _)| version).max())
}

fn versions_in(dir: &Path) -> Result<Vec<(u64, PathBuf)>, Error> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(Error::Io(dir.to_path_buf(), err)),
    };
    Ok(entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .filter_map(|path| version_of(&path).map(|version| (version, path)))
        .collect())
}

/// Move wallpapers into place, merging with files already there.
fn install_wallpapers(from: &Path, to: &Path) -> Result<(), Error> {
    if !from.is_dir() {
        return Ok(());
    }
    if !to.exists() {
        return rename(from, to);
    }
    copy_tree(from, to)
}

fn copy_tree(from: &Path, to: &Path) -> Result<(), Error> {
    fs::create_dir_all(to).map_err(|err| Error::Io(to.to_path_buf(), err))?;
    let entries = fs::read_dir(from).map_err(|err| Error::Io(from.to_path_buf(), err))?;
    for entry in entries {
        let entry = entry.map_err(|err| Error::Io(from.to_path_buf(), err))?;
        let path = entry.path();
        let target = to.join(entry.file_name());
        if path.is_dir() {
            copy_tree(&path, &target)?;
        } else {
            fs::copy(&path, &target).map_err(|err| Error::Io(target, err))?;
        }
    }
    Ok(())
}

fn key_files(dir: &Path) -> Result<Vec<(String, Vec<u8>)>, Error> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(Error::Io(dir.to_path_buf(), err)),
    };
    let mut keys = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(key) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        if !path.is_file() {
            continue;
        }
        keys.push((key, fs::read(&path).map_err(|err| Error::Io(path, err))?));
    }
    keys.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(keys)
}
