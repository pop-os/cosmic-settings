pub use cosmic_bg_config::{Color, Config, Entry, Gradient, ScalingMode, Source};

use image::RgbaImage;
use std::{
    borrow::Cow,
    collections::{hash_map::DefaultHasher, HashMap},
    fs::DirEntry,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::mpsc::{self, Receiver};

pub const DEFAULT_COLORS: &[Color] = &[
    Color::Single([0.580, 0.922, 0.922]),
    Color::Single([0.000, 0.286, 0.427]),
    Color::Single([1.000, 0.678, 0.000]),
    Color::Single([0.282, 0.725, 0.78]),
    Color::Single([0.333, 0.278, 0.259]),
    Color::Single([0.969, 0.878, 0.384]),
    Color::Single([0.063, 0.165, 0.298]),
    Color::Single([1.000, 0.843, 0.631]),
    Color::Single([0.976, 0.227, 0.514]),
    Color::Single([1.000, 0.612, 0.867]),
    Color::Single([0.812, 0.490, 1.000]),
    Color::Single([0.835, 0.549, 1.000]),
    Color::Single([0.243, 0.533, 1.000]),
    Color::Single([0.584, 0.769, 0.988]),
    Color::Gradient(Gradient {
        colors: Cow::Borrowed(&[[1.000, 0.678, 0.000], [0.282, 0.725, 0.78]]),
        radius: 270.0,
    }),
    Color::Gradient(Gradient {
        colors: Cow::Borrowed(&[[1.000, 0.843, 0.631], [0.58, 0.922, 0.922]]),
        radius: 270.0,
    }),
    Color::Gradient(Gradient {
        colors: Cow::Borrowed(&[[1.000, 0.612, 0.867], [0.976, 0.29, 0.514]]),
        radius: 270.0,
    }),
    Color::Gradient(Gradient {
        colors: Cow::Borrowed(&[[0.584, 0.769, 0.988], [0.063, 0.165, 0.298]]),
        radius: 270.0,
    }),
];

pub fn config() -> (Config, HashMap<String, String>) {
    let mut displays = HashMap::new();

    if let Ok(outputs) = crate::outputs::outputs() {
        for output in outputs {
            if let Some(name) = output.name {
                displays.insert(name, output.make);
            }
        }
    }

    let helper = Config::helper().expect("failed to get helper for cosmic bg config");

    let config = match Config::load(&helper) {
        Ok(conf) => conf,
        Err(why) => {
            tracing::warn!(?why, "Config file error, falling back to defaults");
            Config::default()
        }
    };

    (config, displays)
}

pub fn set(config: &mut Config, entry: Entry) {
    if let Ok(context) = Config::helper() {
        tracing::info!(
            output = entry.output.to_string(),
            source = ?entry.source,
            "setting wallpaper",
        );

        let _res = Config::set_same_on_all(&context, config.same_on_all);

        if let Err(why) = config.set_entry(&context, entry) {
            tracing::error!(?why, "failed to set background");
        }
    }
}

/// Path to directory where wallpaper thumbnails are stored.
#[must_use]
pub fn cache_dir() -> Option<PathBuf> {
    dirs::cache_dir().map(|path| {
        let cache = path.join("cosmic-settings/wallpapers");
        let _res = std::fs::create_dir_all(&cache);
        cache
    })
}

/// Loads wallpapers in parallel by spawning tasks with a rayon thread pool.
#[must_use]
pub fn load_each_from_path(path: PathBuf) -> Receiver<(PathBuf, RgbaImage)> {
    let cache_dir = Arc::new(cache_dir());

    let (tx, rx) = mpsc::channel(1);

    tokio::task::spawn_blocking(move || {
        let mut paths = vec![path];

        while let Some(path) = paths.pop() {
            if let Ok(dir) = path.read_dir() {
                for entry in dir.filter_map(Result::ok) {
                    let Ok(file_type) = entry.file_type() else {
                        continue
                    };

                    let path = entry.path();

                    if file_type.is_dir() {
                        paths.push(path);
                    } else if file_type.is_file() {
                        let tx = tx.clone();
                        let cache_dir = cache_dir.clone();
                        rayon::spawn_fifo(move || {
                            let thumbnail =
                                load_thumbnail(cache_dir.as_deref(), &path, &entry, 300, 169);
                            if let Some(image) = thumbnail {
                                let _res = tx.blocking_send((path, image));
                            }
                        });
                    }
                }
            }
        }
    });

    rx
}

/// Generates and caches the thumbnail of a wallpaper.
///
///
/// Caching reduces time required to load a wallpaper by 99%.
#[must_use]
pub fn load_thumbnail(
    cache_dir: Option<&Path>,
    path: &Path,
    entry: &DirEntry,
    width: u32,
    height: u32,
) -> Option<RgbaImage> {
    if let Some(cache_dir) = cache_dir {
        if let Ok(ctime) = entry.metadata().and_then(|meta| meta.created()) {
            // Search for thumbnail by a unique hash string.
            let mut hasher = DefaultHasher::new();
            path.hash(&mut hasher);
            ctime.hash(&mut hasher);
            let hash = hasher.finish();

            let thumbnail_path = cache_dir.join(format!("{hash:x}.png"));

            // Load image from thumbnail if it exists and can be opened.
            if thumbnail_path.exists() {
                if let Ok(image) = image::open(&thumbnail_path) {
                    return Some(image.into_rgba8());
                }
            }

            // Create new thumbnail and save it if not.
            return image::open(path).ok().map(|mut image| {
                image = image.thumbnail_exact(width, height);
                let _res = image.save(&thumbnail_path);
                image.into_rgba8()
            });
        }
    }

    // Generate thumbnail from wallpaper without saving it
    image::open(path).ok().map(|mut image| {
        image = image.thumbnail_exact(width, height);
        image.into_rgba8()
    })
}
