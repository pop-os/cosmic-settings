pub use cosmic_bg_config::{Color, Config, Entry, Gradient, ScalingMode, Source};

use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use std::{
    borrow::Cow,
    collections::{hash_map::DefaultHasher, BTreeSet, HashMap},
    fs::DirEntry,
    hash::{Hash, Hasher},
    io::Read,
    path::{Path, PathBuf},
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
        tracing::debug!(
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
pub fn load_each_from_path(path: PathBuf) -> Receiver<(PathBuf, RgbaImage, RgbaImage)> {
    let cache_dir = cache_dir();

    let (tx, rx) = mpsc::channel(1);

    tokio::task::spawn(async move {
        let mut buffer = Vec::new();
        let mut paths = vec![path];
        let mut wallpapers = BTreeSet::new();

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
                        wallpapers.insert(path);
                    }
                }
            }
        }

        for path in wallpapers {
            if let Some(value) = load_image_with_thumbnail(&mut buffer, path).await {
                let _res = tx.send(value).await;
            }
        }
    });

    rx
}

pub async fn load_image_with_thumbnail(
    buffer: &mut Vec<u8>,
    path: PathBuf,
) -> Option<(
    PathBuf,
    ImageBuffer<Rgba<u8>, Vec<u8>>,
    ImageBuffer<Rgba<u8>, Vec<u8>>,
)> {
    let cache_dir = cache_dir();
    let image_operation = load_thumbnail(buffer, cache_dir.as_deref(), &path);

    let (tx, rx) = tokio::sync::oneshot::channel();

    if let Some(image_operation) = image_operation {
        let tokio_handle = tokio::runtime::Handle::current();

        rayon::spawn_fifo(move || {
            let display_thumbnail = match image_operation {
                ImageOperation::Cached(thumbnail) => thumbnail.to_rgba8(),

                ImageOperation::GenerateThumbnail { path, image } => {
                    let image = image.thumbnail(300, 169).to_rgba8();

                    if let Some(path) = path {
                        // Save thumbnail to disk without blocking.
                        tokio_handle.spawn_blocking({
                            let image = image.clone();
                            move || {
                                if let Err(why) = image.save(&path) {
                                    tracing::error!(?path, ?why, "failed to save image thumbnail");

                                    let _res = std::fs::remove_file(&path);
                                }
                            }
                        });
                    }

                    image
                }
            };

            let mut selection_thumbnail = image::imageops::resize(
                &display_thumbnail,
                158,
                105,
                image::imageops::FilterType::Lanczos3,
            );

            round(&mut selection_thumbnail, [8, 8, 8, 8]);

            let _res = tx.send(Some((path, display_thumbnail, selection_thumbnail)));
        });
    } else {
        tx.send(None);
    }

    rx.await.unwrap_or(None)
}

enum ImageOperation {
    GenerateThumbnail {
        path: Option<PathBuf>,
        image: DynamicImage,
    },
    Cached(DynamicImage),
}

/// Loads wallpaper thumbnails, or defines what needs to be done to create them.
///
///
/// Caching reduces time required to load a wallpaper by 99%.
#[must_use]
fn load_thumbnail(
    input_buffer: &mut Vec<u8>,
    cache_dir: Option<&Path>,
    path: &Path,
) -> Option<ImageOperation> {
    if let Some(cache_dir) = cache_dir {
        if let Ok(ctime) = path.metadata().and_then(|meta| meta.created()) {
            // Search for thumbnail by a unique hash string.
            let mut hasher = DefaultHasher::new();
            path.hash(&mut hasher);
            ctime.hash(&mut hasher);
            let hash = hasher.finish();

            let thumbnail_path = cache_dir.join(format!("{hash:x}.png"));

            if thumbnail_path.exists() {
                if let Some(image) = open_image(input_buffer, &thumbnail_path) {
                    return Some(ImageOperation::Cached(image));
                }

                let _res = std::fs::remove_file(&thumbnail_path);
            }

            return Some(ImageOperation::GenerateThumbnail {
                path: Some(thumbnail_path),
                image: open_image(input_buffer, path)?,
            });
        }
    }

    if let Some(image) = open_image(input_buffer, path) {
        return Some(ImageOperation::GenerateThumbnail { path: None, image });
    }

    None
}

fn open_image(input_buffer: &mut Vec<u8>, path: &Path) -> Option<DynamicImage> {
    let capacity = match path.metadata() {
        Ok(metadata) => metadata.len() as usize,
        Err(why) => {
            tracing::error!(?path, ?why, "error loading image metadata");
            return None;
        }
    };

    input_buffer.clear();
    input_buffer.reserve_exact(capacity);

    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(why) => {
            tracing::error!(?path, ?why, "error opening image");
            return None;
        }
    };

    if let Err(why) = file.read_to_end(input_buffer) {
        tracing::error!(?path, ?why, "error reading image");
    }

    let input_cursor = std::io::Cursor::new(input_buffer);
    let mut image_decoder = image::io::Reader::new(input_cursor);

    image_decoder = if let Ok(decoder) = image_decoder.with_guessed_format() {
        decoder
    } else {
        tracing::error!(?path, "unsupported image format");
        return None;
    };

    match image_decoder.decode() {
        Ok(image) => Some(image),
        Err(why) => {
            tracing::error!(?path, ?why, "image decode failed");
            None
        }
    }
}

// https://users.rust-lang.org/t/how-to-trim-image-to-circle-image-without-jaggy/70374/2
fn round(img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, radius: [u32; 4]) {
    let (width, height) = img.dimensions();
    assert!(radius[0] + radius[1] <= width);
    assert!(radius[3] + radius[2] <= width);
    assert!(radius[0] + radius[3] <= height);
    assert!(radius[1] + radius[2] <= height);

    // top left
    border_radius(img, radius[0], |x, y| (x - 1, y - 1));
    // top right
    border_radius(img, radius[1], |x, y| (width - x, y - 1));
    // bottom right
    border_radius(img, radius[2], |x, y| (width - x, height - y));
    // bottom left
    border_radius(img, radius[3], |x, y| (x - 1, height - y));
}

fn border_radius(
    img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    r: u32,
    coordinates: impl Fn(u32, u32) -> (u32, u32),
) {
    if r == 0 {
        return;
    }
    let r0 = r;

    // 16x antialiasing: 16x16 grid creates 256 possible shades, great for u8!
    let r = 16 * r;

    let mut x = 0;
    let mut y = r - 1;
    let mut p: i32 = 2 - r as i32;

    // ...

    let mut alpha: u16 = 0;
    let mut skip_draw = true;

    let draw = |img: &mut RgbaImage, alpha, x, y| {
        debug_assert!((1..=256).contains(&alpha));
        let pixel_alpha = &mut img[coordinates(r0 - x, r0 - y)].0[3];
        *pixel_alpha = ((alpha * *pixel_alpha as u16 + 128) / 256) as u8;
    };

    'l: loop {
        // (comments for bottom_right case:)
        // remove contents below current position
        {
            let i = x / 16;
            for j in y / 16 + 1..r0 {
                img[coordinates(r0 - i, r0 - j)].0[3] = 0;
            }
        }
        // remove contents right of current position mirrored
        {
            let j = x / 16;
            for i in y / 16 + 1..r0 {
                img[coordinates(r0 - i, r0 - j)].0[3] = 0;
            }
        }

        // draw when moving to next pixel in x-direction
        if !skip_draw {
            draw(img, alpha, x / 16 - 1, y / 16);
            draw(img, alpha, y / 16, x / 16 - 1);
            alpha = 0;
        }

        for _ in 0..16 {
            skip_draw = false;

            if x >= y {
                break 'l;
            }

            alpha += y as u16 % 16 + 1;
            if p < 0 {
                x += 1;
                p += (2 * x + 2) as i32;
            } else {
                // draw when moving to next pixel in y-direction
                if y % 16 == 0 {
                    draw(img, alpha, x / 16, y / 16);
                    draw(img, alpha, y / 16, x / 16);
                    skip_draw = true;
                    alpha = (x + 1) as u16 % 16 * 16;
                }

                x += 1;
                p -= (2 * (y - x) + 2) as i32;
                y -= 1;
            }
        }
    }

    // one corner pixel left
    if x / 16 == y / 16 {
        // column under current position possibly not yet accounted
        if x == y {
            alpha += y as u16 % 16 + 1;
        }
        let s = y as u16 % 16 + 1;
        let alpha = 2 * alpha - s * s;
        draw(img, alpha, x / 16, y / 16);
    }

    // remove remaining square of content in the corner
    let range = y / 16 + 1..r0;
    for i in range.clone() {
        for j in range.clone() {
            img[coordinates(r0 - i, r0 - j)].0[3] = 0;
        }
    }
}
