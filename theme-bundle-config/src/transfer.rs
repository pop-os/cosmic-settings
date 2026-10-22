//! A look written out as one zip, and read back in.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_theme::{DEFAULT_FONT_FAMILY, DEFAULT_ICON_THEME, FontConfig, Theme, ThemeBuilder};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::{
    BASE, Error, Mode, NAME_KEY, Part, Store, copy_versions, interface_key, is_slug,
    neutral_builder, neutral_theme, read, rename, rename_base, version_of, write_base,
    write_builder_base,
};

pub const MANIFEST: &str = "manifest.ron";
pub const VERSION: u64 = 1;
pub const CONFIG_ROOT: &str = "config";
pub const DATA_ROOT: &str = "data";

const OPTIONAL_KEYS: [&str; 3] = ["icon_theme", "font", "mono_font"];

const MAX_DEPTH: usize = 64;
const MAX_PATH: usize = 1024;

/// A bundle written out as one file.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    fn encode(&self) -> Result<Vec<u8>, Error> {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .map(String::into_bytes)
            .map_err(|err| Error::Archive(format!("manifest: {err}")))
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error> {
        ron::de::from_bytes(bytes).map_err(|err| Error::Archive(format!("manifest: {err}")))
    }
}

impl Store {
    pub fn export_live_archive(&self, mode: Mode, path: &Path) -> Result<Manifest, Error> {
        let theme = read::<Theme>(&self.live_config(Part::Theme, mode)?)?;
        let name = crate::store_name(&theme, mode);
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
            if versions.is_empty() {
                continue;
            }
            parts.push(part);

            let exported_base: Option<Vec<u8>> = match part {
                Part::Theme => Some(serialize_base(&neutral_theme(theme))?),
                Part::Builder => {
                    let builder = read::<ThemeBuilder>(&self.config(&source, part.version())?)?;
                    Some(serialize_base(&neutral_builder(&builder))?)
                }
                Part::Background => None,
            };

            let Some((version, dir)) = versions.into_iter().max_by_key(|(version, _)| *version)
            else {
                continue;
            };
            let primary = version == part.version();
            let keys = key_files(&dir)?;

            let outputs: Vec<String> = keys
                .iter()
                .find(|(key, _)| key == "backgrounds")
                .and_then(|(_, bytes)| ron::from_str(String::from_utf8_lossy(bytes).as_ref()).ok())
                .unwrap_or_default();
            let same_on_all = keys
                .iter()
                .find(|(key, _)| key == "same-on-all")
                .and_then(|(_, bytes)| {
                    ron::from_str::<bool>(String::from_utf8_lossy(bytes).as_ref()).ok()
                })
                .unwrap_or(true);

            let mut exported_all = None;
            let mut background_base = None;

            for (key, bytes) in keys {
                if interface_key(&key) {
                    continue;
                }
                if part == Part::Theme && is_default_key(&key, &bytes) {
                    continue;
                }
                if part == Part::Background
                    && let Some(output) = key.strip_prefix("output.")
                    && (same_on_all || !outputs.iter().any(|listed| listed == output))
                {
                    continue;
                }
                if key == "base" {
                    if primary && part == Part::Background {
                        background_base = Some(bytes);
                    }
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

            if !primary {
                continue;
            }

            let base = match part {
                Part::Background => match exported_all {
                    Some(bytes) => Some(bytes),
                    None => match background_base {
                        Some(bytes) => Some(assets.background("base", bytes)?),
                        None => None,
                    },
                },
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

        let avoid = self.reserved_names();
        let (name, slug) =
            self.unique_name_avoiding(name.unwrap_or(manifest.name.as_str()), &avoid)?;

        let source = temp.path().join(CONFIG_ROOT).join("cosmic");
        for part in Part::ALL {
            let from = source.join(Self::bundle_id(&manifest.slug, part));
            if !from.is_dir() {
                continue;
            }
            copy_versions(&from, &self.bundle_config_dir(&slug, part))?;
        }

        // The wallpapers the look points at move under the name it has here.
        let wallpapers = temp
            .path()
            .join(DATA_ROOT)
            .join("cosmic")
            .join(&manifest.slug)
            .join("wallpapers");
        if wallpapers.is_dir() {
            rename(
                &wallpapers,
                &data.join("cosmic").join(&slug).join("wallpapers"),
            )?;
        }
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

        let bundle = self.bundle(&slug)?;
        Ok(bundle)
    }

    /// Name a copied-in or renamed bundle.
    pub(crate) fn write_imported_name(&self, slug: &str, name: &str) -> Result<(), Error> {
        let config = self.bundle_config(slug, Part::Theme)?;
        if config.get::<Theme>(BASE).is_ok() {
            config.set(NAME_KEY, name.to_owned())?;
            rename_base(&config, name)?;
        } else {
            let mut theme = read::<Theme>(&config)?;
            let builder = read::<ThemeBuilder>(&self.bundle_config(slug, Part::Builder)?)?;
            name.clone_into(&mut theme.name);
            write_base(&config, name, &theme)?;
            write_builder_base(&self.bundle_config(slug, Part::Builder)?, &builder)?;
        }
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
        let Ok(mut entry) =
            ron::from_str::<cosmic_bg_config::Entry>(&String::from_utf8_lossy(&bytes))
        else {
            return Ok(bytes);
        };
        let cosmic_bg_config::Source::Path(path) = &entry.source else {
            return Ok(bytes);
        };
        let Some(place) = self.wallpaper(path) else {
            tracing::warn!(key, wallpaper = %path.display(), "a wallpaper could not be carried");
            return Ok(bytes);
        };
        entry.source = cosmic_bg_config::Source::Path(PathBuf::from(place));
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
        zip.start_file(MANIFEST, text)?;
        zip.write_all(&manifest.encode()?)
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
    let mut entry = archive
        .by_name(MANIFEST)
        .map_err(|_| Error::Archive(format!("the archive holds no {MANIFEST}")))?;
    let mut bytes = Vec::new();
    entry
        .by_ref()
        .take(100_000)
        .read_to_end(&mut bytes)
        .map_err(|err| Error::Io(PathBuf::from(MANIFEST), err))?;
    Manifest::decode(&bytes)
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
    for (_, dir) in versions_in(&store.bundle_config_dir(slug, Part::Background))? {
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
            let path = dir.join(&key);
            let text = ron::ser::to_string_pretty(&entry, ron::ser::PrettyConfig::default())
                .map_err(|err| Error::Archive(format!("background: {err}")))?;
            fs::write(&path, text).map_err(|err| Error::Io(path, err))?;
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
        if key.starts_with(".atomicwrite") || !path.is_file() {
            continue;
        }
        keys.push((key, fs::read(&path).map_err(|err| Error::Io(path, err))?));
    }
    keys.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(keys)
}
