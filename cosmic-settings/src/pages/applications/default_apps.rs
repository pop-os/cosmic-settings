// Copyright 2024 System76 <info@system76.com>
// Copyright 2024 bbb651 <bar.ye651@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    sync::Arc,
};

use cosmic::{
    Apply, Element, Task, surface,
    widget::{self, dropdown, icon, settings},
};
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_config::shortcuts::SystemActions;
use cosmic_settings_config::shortcuts::action::System;
use cosmic_settings_page::{self as page, Section, section};
use freedesktop_desktop_entry::{
    DesktopEntry, Iter as DesktopEntryIter, default_paths, get_languages_from_env,
};
use mime_apps::App;
use slotmap::SlotMap;

const DROPDOWN_WEB_BROWSER: usize = 0;
const DROPDOWN_FILE_MANAGER: usize = 1;
const DROPDOWN_MAIL: usize = 2;
const DROPDOWN_MUSIC: usize = 3;
const DROPDOWN_VIDEO: usize = 4;
const DROPDOWN_PHOTO: usize = 5;
const DROPDOWN_CALENDAR: usize = 6;
const DROPDOWN_TERMINAL: usize = 7;
const DROPDOWN_TEXT_EDITOR: usize = 8;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Category {
    Audio,
    Calendar,
    FileManager,
    Image,
    Mail,
    Mime(&'static str),
    Terminal,
    Video,
    WebBrowser,
    TextEditor,
}

#[derive(Clone, Debug)]
pub enum Message {
    SetDefault(Category, usize),
    Update(CachedMimeApps),
    Surface(surface::Action),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::DefaultApps(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::DefaultApps(message)
    }
}

#[derive(Clone, Debug)]
pub struct CachedMimeApps {
    pub list: mime_apps::List,
    pub local_list: mime_apps::List,
    pub apps: Vec<AppMeta>,
    pub known_mimes: BTreeSet<mime::Mime>,
    pub config_path: Box<Path>,
}

#[derive(Clone, Debug)]
pub struct AppMeta {
    selected: Option<usize>,
    app_ids: Vec<String>,
    apps: Vec<String>,
    icons: Vec<icon::Handle>,
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    on_enter_handle: Option<cosmic::iced::task::Handle>,
    mime_apps: Option<CachedMimeApps>,
    shortcuts_config: Option<cosmic_config::Config>,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<cosmic_settings_page::Content> {
        Some(vec![sections.insert(apps())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("default-apps", "preferences-default-applications-symbolic")
            .title(fl!("default-apps"))
            .description(fl!("default-apps", "desc"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }

        if self.shortcuts_config.is_none() {
            self.shortcuts_config = cosmic_settings_config::shortcuts::context().ok();
        }

        let (task, on_enter_handle) = Task::future(async move {
            let mut list = mime_apps::List::default();
            list.load_from_paths(&mime_apps::list_paths());

            let mut local_list = mime_apps::List::default();

            if let Some(path) = mime_apps::local_list_path() {
                if let Ok(buffer) = std::fs::read_to_string(&path) {
                    local_list.load_from(&buffer);
                }
            }

            let assocs = mime_apps::associations::by_app();

            let apps = vec![
                load_defaults(&assocs, &["x-scheme-handler/http"]).await,
                load_defaults(&assocs, &["inode/directory"]).await,
                load_defaults(&assocs, &["x-scheme-handler/mailto"]).await,
                load_defaults(&assocs, &["audio/mp3", "application/ogg", "video/mp4"]).await,
                load_defaults(&assocs, &["video/mp4"]).await,
                load_defaults(&assocs, &["image/png"]).await,
                load_defaults(&assocs, &["text/calendar"]).await,
                load_terminal_apps(&assocs).await,
                load_defaults(&assocs, &["text/plain"]).await,
            ];

            Message::Update(CachedMimeApps {
                apps,
                list,
                local_list,
                known_mimes: mime_apps::mime_info::mime_types(),
                config_path: dirs::config_dir()
                    .expect("config dir not found")
                    .join("mimeapps.list")
                    .into(),
            })
            .into()
        })
        .abortable();

        self.on_enter_handle = Some(on_enter_handle);

        task
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }

        self.mime_apps = None;

        Task::none()
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::Message> {
        match message {
            Message::SetDefault(category, id) => {
                let Some(mime_apps) = self.mime_apps.as_mut() else {
                    return Task::none();
                };

                let mime_types: Vec<&str>;
                let (category_id, mime_types): (usize, &[&str]) = match category {
                    Category::Audio => (DROPDOWN_MUSIC, {
                        mime_types = mime_apps
                            .known_mimes
                            .iter()
                            .map(|m| m.essence_str())
                            .filter(|m| m.starts_with("audio"))
                            .chain(
                                [
                                    "application/ogg",
                                    "application/x-cue",
                                    "application/x-ogg",
                                    "audio/mp3",
                                    "x-content/audio-cdda",
                                ]
                                .into_iter(),
                            )
                            .collect();
                        &mime_types
                    }),
                    Category::Calendar => (DROPDOWN_CALENDAR, &["text/calendar"]),
                    Category::FileManager => (DROPDOWN_FILE_MANAGER, &["inode/directory"]),
                    Category::Image => (DROPDOWN_PHOTO, {
                        mime_types = mime_apps
                            .known_mimes
                            .iter()
                            .map(|m| m.essence_str())
                            .filter(|m| m.starts_with("image"))
                            .collect();
                        &mime_types
                    }),
                    Category::Mail => (DROPDOWN_MAIL, &["x-scheme-handler/mailto"]),
                    Category::Terminal => (
                        DROPDOWN_TERMINAL,
                        &[
                            "x-scheme-handler/terminal",
                            "application/x-terminal-emulator",
                        ],
                    ),
                    Category::Video => (DROPDOWN_VIDEO, {
                        mime_types = mime_apps
                            .known_mimes
                            .iter()
                            .map(|m| m.essence_str())
                            .filter(|m| m.starts_with("video"))
                            .collect();
                        &mime_types
                    }),
                    Category::WebBrowser => (
                        DROPDOWN_WEB_BROWSER,
                        &[
                            "text/html",
                            "application/xhtml+xml",
                            "x-scheme-handler/chrome",
                            "x-scheme-handler/http",
                            "x-scheme-handler/https",
                        ],
                    ),
                    Category::TextEditor => (DROPDOWN_TEXT_EDITOR, &["text/plain"]),
                    Category::Mime(_mime_type) => return Task::none(),
                };

                let meta = &mut mime_apps.apps[category_id];

                if meta.selected != Some(id) {
                    meta.selected = Some(id);
                    let appid = &meta.app_ids[id];

                    if category == Category::Terminal && self.shortcuts_config.is_some() {
                        if let Some(config) = self.shortcuts_config.as_ref() {
                            assign_default_terminal(config, appid);
                        }
                    }

                    for mime in mime_types {
                        if let Ok(mime) = mime.parse() {
                            mime_apps
                                .local_list
                                .set_default_app(mime, [appid, ".desktop"].concat());
                        };
                    }

                    let mut buffer = mime_apps.local_list.to_string();
                    buffer.push('\n');

                    _ = std::fs::write(&mime_apps.config_path, buffer);
                    _ = std::process::Command::new("update-desktop-database").status();
                }
            }
            Message::Update(mime_apps) => {
                self.mime_apps = Some(mime_apps);
            }
            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }
        }

        Task::none()
    }
}

fn apps() -> Section<crate::pages::Message> {
    Section::default().view::<Page>(move |_binder, page, section| {
        let Some(mime_apps) = page.mime_apps.as_ref() else {
            return widget::row().into();
        };

        settings::section()
            .title(&section.title)
            .add({
                let meta = &mime_apps.apps[DROPDOWN_WEB_BROWSER];
                settings::flex_item(
                    fl!("default-apps", "web-browser"),
                    dropdown::popup_dropdown(
                        &meta.apps,
                        meta.selected,
                        |id| Message::SetDefault(Category::WebBrowser, id),
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        |a| crate::app::Message::PageMessage(crate::pages::Message::DefaultApps(a)),
                    )
                    .icons(Cow::Borrowed(&meta.icons)),
                )
                .min_item_width(300.0)
            })
            .add({
                let meta = &mime_apps.apps[DROPDOWN_FILE_MANAGER];
                settings::flex_item(
                    fl!("default-apps", "file-manager"),
                    dropdown::popup_dropdown(
                        &meta.apps,
                        meta.selected,
                        |id| Message::SetDefault(Category::FileManager, id),
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        |a| crate::app::Message::PageMessage(crate::pages::Message::DefaultApps(a)),
                    )
                    .icons(Cow::Borrowed(&meta.icons)),
                )
            })
            .add({
                let meta = &mime_apps.apps[DROPDOWN_MAIL];
                settings::flex_item(
                    fl!("default-apps", "mail-client"),
                    dropdown::popup_dropdown(
                        &meta.apps,
                        meta.selected,
                        |id| Message::SetDefault(Category::Mail, id),
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        |a| crate::app::Message::PageMessage(crate::pages::Message::DefaultApps(a)),
                    )
                    .icons(Cow::Borrowed(&meta.icons)),
                )
            })
            .add({
                let meta = &mime_apps.apps[DROPDOWN_MUSIC];
                settings::flex_item(
                    fl!("default-apps", "music"),
                    dropdown::popup_dropdown(
                        &meta.apps,
                        meta.selected,
                        |id| Message::SetDefault(Category::Audio, id),
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        |a| crate::app::Message::PageMessage(crate::pages::Message::DefaultApps(a)),
                    )
                    .icons(Cow::Borrowed(&meta.icons)),
                )
            })
            .add({
                let meta = &mime_apps.apps[DROPDOWN_VIDEO];
                settings::flex_item(
                    fl!("default-apps", "video"),
                    dropdown::popup_dropdown(
                        &meta.apps,
                        meta.selected,
                        |id| Message::SetDefault(Category::Video, id),
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        |a| crate::app::Message::PageMessage(crate::pages::Message::DefaultApps(a)),
                    )
                    .icons(Cow::Borrowed(&meta.icons)),
                )
            })
            .add({
                let meta = &mime_apps.apps[DROPDOWN_PHOTO];
                settings::flex_item(
                    fl!("default-apps", "photos"),
                    dropdown::popup_dropdown(
                        &meta.apps,
                        meta.selected,
                        |id| Message::SetDefault(Category::Image, id),
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        |a| crate::app::Message::PageMessage(crate::pages::Message::DefaultApps(a)),
                    )
                    .icons(Cow::Borrowed(&meta.icons)),
                )
            })
            .add({
                let meta = &mime_apps.apps[DROPDOWN_CALENDAR];
                settings::flex_item(
                    fl!("default-apps", "calendar"),
                    dropdown::popup_dropdown(
                        &meta.apps,
                        meta.selected,
                        |id| Message::SetDefault(Category::Calendar, id),
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        |a| crate::app::Message::PageMessage(crate::pages::Message::DefaultApps(a)),
                    )
                    .icons(Cow::Borrowed(&meta.icons)),
                )
            })
            .add({
                let meta = &mime_apps.apps[DROPDOWN_TERMINAL];
                settings::flex_item(
                    fl!("default-apps", "terminal"),
                    dropdown::popup_dropdown(
                        &meta.apps,
                        meta.selected,
                        |id| Message::SetDefault(Category::Terminal, id),
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        |a| crate::app::Message::PageMessage(crate::pages::Message::DefaultApps(a)),
                    )
                    .icons(Cow::Borrowed(&meta.icons)),
                )
            })
            .add({
                let meta = &mime_apps.apps[DROPDOWN_TEXT_EDITOR];
                settings::flex_item(
                    fl!("default-apps", "text-editor"),
                    dropdown::popup_dropdown(
                        &meta.apps,
                        meta.selected,
                        |id| Message::SetDefault(Category::TextEditor, id),
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        |a| crate::app::Message::PageMessage(crate::pages::Message::DefaultApps(a)),
                    )
                    .icons(Cow::Borrowed(&meta.icons)),
                )
            })
            .apply(Element::from)
            .map(crate::pages::Message::DefaultApps)
    })
}

fn assign_default_terminal(config: &cosmic_config::Config, appid: &str) {
    let mut actions = config
        .get_local::<SystemActions>("system_actions")
        .unwrap_or_default();

    let default_paths = default_paths();
    let mut resolved_path = None;

    // loop through all FDE paths to try and find a valid .desktop file
    for path in default_paths {
        if let Ok(mut full_path) = path.canonicalize() {
            full_path = full_path.join([appid, ".desktop"].concat());
            if full_path.exists() && full_path.is_file() {
                resolved_path = Some(full_path);
                break;
            }
        }
    }

    // if we find a valid .desktop file, we can grab its exec
    if let Some(resolved_path) = resolved_path {
        let desktop_entry = DesktopEntry::from_path(resolved_path, Some(&get_languages_from_env()));

        if let Ok(desktop_entry) = desktop_entry {
            if let Some(exec) = desktop_entry.exec() {
                actions.insert(System::Terminal, String::from(exec));

                if let Err(why) = config.set("system_actions", actions) {
                    tracing::error!(?why, "Unable to set system_actions shortcuts config");
                }
            }
        }
    }
}

async fn load_defaults(assocs: &BTreeMap<Arc<str>, Arc<App>>, for_mimes: &[&str]) -> AppMeta {
    let mut unsorted = Vec::new();
    let mut current_app = None;

    for for_mime in for_mimes {
        let Ok(mime) = for_mime.parse() else {
            return AppMeta {
                selected: None,
                app_ids: Vec::new(),
                apps: Vec::new(),
                icons: Vec::new(),
            };
        };

        let current_app_entry = xdg_mime_query_default(for_mime).await;
        let current_appid = current_app_entry
            .as_ref()
            .and_then(|entry| entry.strip_suffix(".desktop"));

        if unsorted.is_empty() {
            current_app = current_appid.and_then(|appid| assocs.get(appid));
        }

        unsorted.extend(
            mime_apps::apps_for_mime(&mime, assocs)
                .map(|(app_id, app)| (app_id.clone(), app.clone())),
        );
    }

    unsorted.sort_unstable_by_key(|(_, app)| app.name.clone());
    unsorted.dedup_by_key(|(app_id, _)| app_id.clone());

    let mut selected = None;
    let mut app_ids = Vec::new();
    let mut apps = Vec::new();
    let mut icons = Vec::new();

    for (id, (appid, app)) in unsorted.iter().enumerate() {
        if let Some(current_app) = current_app {
            if app.name.as_ref() == current_app.name.as_ref() {
                selected = Some(id);
            }
        }

        app_ids.push(appid.as_ref().into());
        apps.push(app.name.as_ref().into());
        icons.push(if app.icon.starts_with('/') {
            icon::from_path(PathBuf::from(app.icon.as_ref()))
        } else {
            icon::from_name(app.icon.as_ref()).size(20).handle()
        });
    }

    AppMeta {
        selected,
        app_ids,
        apps,
        icons,
    }
}

async fn xdg_mime_query_default(mime_type: &str) -> Option<String> {
    let output = tokio::process::Command::new("xdg-mime")
        .args(&["query", "default", mime_type])
        .output()
        .await
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8(output.stdout)
        .ok()
        .map(|string| string.trim().to_owned())
}

async fn load_terminal_apps(assocs: &BTreeMap<Arc<str>, Arc<App>>) -> AppMeta {
    let mut terminals = Vec::new();
    let mut current_app = None;

    // Get the current default terminal if set
    let current_entry = xdg_mime_query_default("x-scheme-handler/terminal").await;
    let current_appid = current_entry
        .as_ref()
        .and_then(|entry| entry.strip_suffix(".desktop"));

    // Also check x-terminal-emulator
    if current_app.is_none() {
        let current_entry = xdg_mime_query_default("application/x-terminal-emulator").await;
        let current_appid = current_entry
            .as_ref()
            .and_then(|entry| entry.strip_suffix(".desktop"));
        if let Some(appid) = current_appid {
            current_app = assocs.get(appid).cloned();
        }
    }

    // Scan desktop entries for terminal applications
    for path in DesktopEntryIter::new(default_paths()) {
        if let Ok(bytes) = std::fs::read_to_string(&path) {
            if let Ok(entry) = DesktopEntry::from_str(&path, &bytes, None::<&[&str]>) {
                // Check if it's a terminal application
                if entry
                    .categories()
                    .map(|cats| cats.iter().any(|c| *c == "TerminalEmulator"))
                    .unwrap_or(false)
                {
                    let id = entry.id();
                    if let Some(app) = assocs.get(id) {
                        if current_appid.as_ref().map(|c| *c == id).unwrap_or(false) {
                            current_app = Some(app.clone());
                        }
                        terminals.push((Arc::from(id), app.clone()));
                    }
                }
            }
        }
    }

    terminals.sort_unstable_by_key(|(_, app): &(Arc<str>, Arc<App>)| app.name.clone());
    terminals.dedup_by(|a, b| a.0 == b.0);

    let mut selected = None;
    let mut app_ids = Vec::new();
    let mut apps = Vec::new();
    let mut icons = Vec::new();

    for (id, (appid, app)) in terminals.iter().enumerate() {
        if let Some(ref current_app) = current_app {
            if app.name.as_ref() == current_app.name.as_ref() {
                selected = Some(id);
            }
        }

        app_ids.push(appid.to_string());
        apps.push(app.name.as_ref().into());
        icons.push(if app.icon.starts_with('/') {
            icon::from_path(PathBuf::from(app.icon.as_ref()))
        } else {
            icon::from_name(app.icon.as_ref()).size(20).handle()
        });
    }

    AppMeta {
        selected,
        app_ids,
        apps,
        icons,
    }
}
