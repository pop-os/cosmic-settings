// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::BTreeMap, path::PathBuf};

use super::Message;
use cosmic::{
    Element,
    iced::{Background, Length},
    widget::{button, icon, text},
};
use tokio::io::AsyncBufReadExt;

const ICON_PREV_N: usize = 6;
const ICON_PREV_ROW: usize = 3;
const ICON_TRY_SIZES: [u16; 3] = [32, 48, 64];
const ICON_THUMB_SIZE: u16 = 32;
const ICON_NAME_TRUNC: usize = 20;

pub type IconThemes = Vec<IconTheme>;
pub type IconHandles = Vec<[icon::Handle; ICON_PREV_N]>;

/// Button with a preview of the icon theme.
pub fn button(
    name: &str,
    handles: &[icon::Handle],
    id: usize,
    selected: bool,
    callback: impl Fn(usize) -> super::Message,
) -> Element<'static, Message> {
    let theme = cosmic::theme::active();
    let theme = theme.cosmic();
    let background = Background::Color(theme.palette.neutral_4.into());

    cosmic::widget::column()
        .push(
            cosmic::widget::button::custom_image_button(
                cosmic::widget::column::with_children([
                    cosmic::widget::row()
                        .extend(
                            handles
                                .iter()
                                .take(ICON_PREV_ROW)
                                .cloned()
                                // TODO: Maybe allow choosable sizes/zooming
                                .map(|handle| handle.icon().size(ICON_THUMB_SIZE).into()),
                        )
                        .spacing(theme.space_xxxs())
                        .into(),
                    cosmic::widget::row()
                        .extend(
                            handles
                                .iter()
                                .skip(ICON_PREV_ROW)
                                .cloned()
                                // TODO: Maybe allow choosable sizes/zooming
                                .map(|handle| handle.icon().size(ICON_THUMB_SIZE).into()),
                        )
                        .spacing(theme.space_xxxs())
                        .into(),
                ])
                .spacing(theme.space_xxxs()),
                None,
            )
            .on_press(callback(id))
            .selected(selected)
            .padding(theme.space_xs())
            // Image button's style mostly works, but it needs a background to fit the design
            .class(button::ButtonClass::Custom {
                active: Box::new(move |focused, theme| {
                    let mut appearance = <cosmic::theme::Theme as button::Catalog>::active(
                        theme,
                        focused,
                        selected,
                        &cosmic::theme::Button::Image,
                    );
                    appearance.background = Some(background);
                    appearance
                }),
                disabled: Box::new(move |theme| {
                    let mut appearance = <cosmic::theme::Theme as button::Catalog>::disabled(
                        theme,
                        &cosmic::theme::Button::Image,
                    );
                    appearance.background = Some(background);
                    appearance
                }),
                hovered: Box::new(move |focused, theme| {
                    let mut appearance = <cosmic::theme::Theme as button::Catalog>::hovered(
                        theme,
                        focused,
                        selected,
                        &cosmic::theme::Button::Image,
                    );
                    appearance.background = Some(background);
                    appearance
                }),
                pressed: Box::new(move |focused, theme| {
                    let mut appearance = <cosmic::theme::Theme as button::Catalog>::pressed(
                        theme,
                        focused,
                        selected,
                        &cosmic::theme::Button::Image,
                    );
                    appearance.background = Some(background);
                    appearance
                }),
            }),
        )
        .push(
            text::body(if name.len() > ICON_NAME_TRUNC {
                format!("{name:.ICON_NAME_TRUNC$}...")
            } else {
                name.into()
            })
            .width(Length::Fixed((ICON_THUMB_SIZE * 3) as _)),
        )
        .spacing(theme.space_xxs())
        .into()
}

/// Find all icon themes available on the system.
pub async fn fetch() -> Message {
    let mut icon_themes = BTreeMap::new();
    let mut theme_paths: BTreeMap<String, PathBuf> = BTreeMap::new();

    let mut buffer = String::new();

    let xdg_data_home = std::env::var("XDG_DATA_HOME")
        .ok()
        .and_then(|value| {
            if value.is_empty() {
                None
            } else {
                Some(PathBuf::from(value))
            }
        })
        .or_else(dirs::home_dir)
        .map(|dir| dir.join(".local/share/icons"));

    let xdg_data_dirs = std::env::var("XDG_DATA_DIRS").ok();

    let xdg_data_dirs = xdg_data_dirs
        .as_deref()
        // Default from the XDG Base Directory Specification
        .or(Some("/usr/local/share/:/usr/share/"))
        .into_iter()
        .flat_map(|arg| std::env::split_paths(arg).map(|dir| dir.join("icons")));

    for icon_dir in xdg_data_dirs.chain(xdg_data_home) {
        let Ok(read_dir) = std::fs::read_dir(&icon_dir) else {
            continue;
        };

        'icon_dir: for entry in read_dir.filter_map(Result::ok) {
            let Ok(path) = entry.path().canonicalize() else {
                continue;
            };

            let Some(id) = entry.file_name().to_str().map(String::from) else {
                continue;
            };

            let manifest = path.join("index.theme");

            if !manifest.exists() {
                continue;
            }

            let Ok(file) = tokio::fs::File::open(&manifest).await else {
                continue;
            };

            buffer.clear();
            let mut name = None;
            let mut valid_dirs = Vec::new();

            let mut line_reader = tokio::io::BufReader::new(file);
            while let Ok(read) = line_reader.read_line(&mut buffer).await {
                if read == 0 {
                    break;
                }

                if let Some(is_hidden) = buffer.strip_prefix("Hidden=") {
                    if is_hidden.trim() == "true" {
                        continue 'icon_dir;
                    }
                } else if name.is_none()
                    && let Some(value) = buffer.strip_prefix("Name=")
                {
                    name = Some(value.trim().to_owned());
                }

                if valid_dirs.is_empty()
                    && let Some(value) = buffer.strip_prefix("Inherits=")
                {
                    valid_dirs.extend(value.trim().split(',').map(|fallback| {
                        if let Some(path) = theme_paths.get(fallback) {
                            path.iter()
                                .next_back()
                                .and_then(|os| os.to_str().map(ToOwned::to_owned))
                                .unwrap_or_else(|| fallback.to_owned())
                        } else {
                            fallback.to_owned()
                        }
                    }));
                }

                buffer.clear();
            }

            if let Some(name) = name {
                // Name of the directory theme was found in (e.g. Pop for Pop)
                valid_dirs.push(
                    path.iter()
                        .next_back()
                        .and_then(|os| os.to_str().map(ToOwned::to_owned))
                        .unwrap_or_else(|| name.clone()),
                );
                theme_paths.entry(name.clone()).or_insert(path);

                let theme = id.clone();
                /* This section is performance critical  */
                // `icon::from_name` may perform blocking I/O
                if let Ok(handles) =
                    tokio::task::spawn_blocking(|| preview_handles(theme, valid_dirs)).await
                {
                    icon_themes.insert(IconTheme { id, name }, handles);
                }
                /* END  */
            }
        }
    }

    Message::DrawerIcon(super::drawer::IconMessage::IconLoaded(
        icon_themes.into_iter().unzip(),
    ))
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IconTheme {
    // COSMIC uses the file name of the folder containing the theme
    pub id: String,
    // GTK uses the name of the theme as specified in its index file
    pub name: String,
}

/// Generate [icon::Handle]s to use for icon theme previews.
fn preview_handles(theme: String, inherits: Vec<String>) -> [icon::Handle; ICON_PREV_N] {
    // Cache current default and set icon theme as a temporary default
    let default = cosmic::icon_theme::default();
    cosmic::icon_theme::set_default(theme);

    // Evaluate handles with the temporary theme
    let handles = [
        icon_handle("folder", "folder-symbolic", &inherits),
        icon_handle("user-home", "user-home-symbolic", &inherits),
        icon_handle("text-x-generic", "text-x-generic-symbolic", &inherits),
        icon_handle("image-x-generic", "images-x-generic-symbolic", &inherits),
        icon_handle("audio-x-generic", "audio-x-generic-symbolic", &inherits),
        icon_handle("video-x-generic", "video-x-generic-symbolic", &inherits),
    ];

    // Reset default icon theme.
    cosmic::icon_theme::set_default(default);
    handles
}

/// Evaluate an icon handle for a specific theme.
///
/// `alternate` is a fallback icon name such as a symbolic variant.
///
/// `valid_dirs` should be a slice of directories from which we consider an icon to be valid. Valid
/// directories would usually be inherited themes as well as the actual theme's location.
fn icon_handle(icon_name: &str, alternate: &str, valid_dirs: &[String]) -> icon::Handle {
    ICON_TRY_SIZES
        .iter()
        .zip(std::iter::repeat_n(icon_name, ICON_TRY_SIZES.len()))
        // Try fallback icon name after the default
        .chain(
            ICON_TRY_SIZES
                .iter()
                .zip(std::iter::repeat(alternate))
                .take(ICON_TRY_SIZES.len()),
        )
        .find_map(|(&size, name)| {
            icon::from_name(name)
                // Set the size on the handle to evaluate the correct icon
                .size(size)
                // Get the path to the icon for the currently set theme.
                // Without the exact path, the handles will all resolve to icons from the same theme in
                // [`icon_theme_button`] rather than the icons for each different theme
                .path()
                // `libcosmic` should always return a path if the default theme is installed
                // The returned path has to be verified as an icon from the set theme or an
                // inherited theme
                .and_then(|path| {
                    let mut theme_dir = &*path;
                    while let Some(parent) = theme_dir.parent() {
                        if parent.ends_with("icons") {
                            break;
                        }
                        theme_dir = parent;
                    }

                    if let Some(dir_name) = theme_dir
                        .iter()
                        .next_back()
                        .and_then(std::ffi::OsStr::to_str)
                    {
                        valid_dirs
                            .iter()
                            .any(|valid| dir_name == valid)
                            .then(|| icon::from_path(path))
                    } else {
                        None
                    }
                })
        })
        // Fallback icon handle
        .unwrap_or_else(|| icon::from_name(icon_name).size(ICON_THUMB_SIZE).handle())
}
