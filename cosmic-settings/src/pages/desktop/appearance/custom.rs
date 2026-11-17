// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::app::ContextDrawer;
use cosmic::cosmic_config::ConfigSet;
use cosmic::cosmic_theme::Roundness;
use cosmic::cosmic_theme::palette::{FromColor, Hsv, Srgb};
use cosmic::iced::core::{Alignment, Length};
use cosmic::widget::color_picker::ColorPickerUpdate;
use cosmic::widget::text_input::focus;
use cosmic::widget::{button, container, icon, settings, text};
use cosmic::{Apply, Element, Task, task};
use cosmic_settings_page::{self as page, Section, section};
use cosmic_theme_bundle_config::Mode;
use slotmap::{Key, SlotMap};

use crate::pages::desktop::appearance::font_config::FONT_SEARCH;

use super::icon_themes::IconTheme;
use super::theme_manager;
use super::{
    ContextView, Message, colors_and_style, drawer, font_config, style, theme_bundles, wallpaper,
};

pub struct Page {
    pub(crate) entity: page::Entity,
    pub(crate) mode: Mode,
    pub(crate) theme_manager: theme_manager::Manager,
    pub(crate) drawer: drawer::Content,
    pub(crate) context_view: Option<ContextView>,
    pub(crate) roundness: Roundness,
    pub(crate) save_as: Option<String>,
}

impl Default for Page {
    fn default() -> Self {
        let theme_manager = theme_manager::Manager::default();
        Self {
            entity: page::Entity::null(),
            mode: Mode::from_is_dark(theme_manager.mode().is_dark),
            roundness: theme_manager.builder().corner_radii.into(),
            drawer: drawer::Content::from(&theme_manager),
            theme_manager,
            context_view: None,
            save_as: None,
        }
    }
}

fn here(message: Message) -> crate::app::Message {
    crate::app::Message::PageMessage(crate::pages::Message::AppearanceTheme(message))
}

impl Page {
    fn reload(&mut self) {
        let mut manager = theme_manager::Manager::default();
        let mut mode = *manager.mode();
        mode.is_dark = self.mode.is_dark();
        manager.sync_mode(mode);

        self.theme_manager = manager;
        self.roundness = self.theme_manager.theme().corner_radii.into();
        self.drawer.reset(&self.theme_manager);
        self.drawer.sync_fonts(&self.theme_manager);
        self.context_view = None;
    }

    fn preview_theme(&self) -> cosmic::theme::Theme {
        let mut theme = self.theme_manager.builder().clone().build();
        // TODO remove me later after the fix has been in for a bit.
        theme
            .icon_theme
            .clone_from(&self.theme_manager.theme().icon_theme);
        cosmic::theme::Theme::custom(std::sync::Arc::new(theme))
    }

    fn is_active_mode(&self) -> bool {
        self.mode.is_dark() == theme_bundles::on_screen_mode().is_dark()
    }

    fn preview(&self) -> crate::pages::Message {
        crate::pages::Message::AppearancePreview(Some(self.preview_theme()))
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        let mut tasks = Vec::<Task<crate::app::Message>>::new();
        let mut theme_staged = None;
        let mut theme_changed = false;

        match message {
            Message::Open(mode) => {
                self.mode = mode;
                self.save_as = None;
                self.reload();

                return cosmic::task::message(crate::app::Message::Page(self.entity));
            }

            Message::DrawerOpen(context_view) => {
                self.context_view = Some(context_view);
                let mut previous =
                    std::mem::replace(&mut self.drawer, drawer::Content::from(&self.theme_manager));
                self.drawer.preserve_from(&mut previous);
                tasks.push(cosmic::task::message(
                    crate::app::Message::OpenContextDrawer(self.entity),
                ));
                tasks.push(self.drawer.on_open(
                    &context_view,
                    &self.theme_manager,
                    crate::pages::Message::AppearanceTheme,
                ));
            }

            Message::DrawerColor(u) => {
                if let Some(context_view) = self.context_view.as_ref()
                    && self.drawer.update_color(&mut tasks, u, context_view)
                {
                    theme_staged = self
                        .theme_manager
                        .set_color(self.drawer.current_color(context_view), context_view);
                }
            }

            Message::PaletteAccent(c) => {
                theme_staged = self
                    .theme_manager
                    .selected_customizer_mut()
                    .set_accent(Some(Srgb::from(c)));
            }

            Message::UseDefaultWindowHint(v) => {
                if v {
                    let _ = self
                        .theme_manager
                        .selected_customizer_mut()
                        .set_window_hint(None)
                        .is_some_and(|_| true);
                    return Task::none();
                }

                let window_hint = self
                    .theme_manager
                    .builder()
                    .window_hint
                    .or(self.theme_manager.builder().accent);

                _ = self
                    .drawer
                    .accent_window_hint
                    .update::<crate::app::Message>(ColorPickerUpdate::ActiveColor(
                        Hsv::from_color(window_hint.unwrap_or_default()),
                    ));

                _ = self
                    .drawer
                    .accent_window_hint
                    .update::<crate::app::Message>(ColorPickerUpdate::AppliedColor);

                theme_staged = self
                    .theme_manager
                    .selected_customizer_mut()
                    .set_window_hint(window_hint);
            }

            Message::CursorTheme(idx) => {
                let cursor_theme = (idx > 0)
                    .then(|| {
                        self.drawer
                            .cursor_themes
                            .get(idx)
                            .map(|(t, _)| t.id.clone())
                    })
                    .flatten();
                self.drawer.cursor_theme_active.clone_from(&cursor_theme);
                self.theme_manager.set_cursor_theme(cursor_theme.clone());

                let is_dark = self.mode.is_dark();
                let configs = if is_dark {
                    (
                        cosmic::cosmic_theme::Theme::dark_config(),
                        cosmic::cosmic_theme::ThemeBuilder::dark_config(),
                    )
                } else {
                    (
                        cosmic::cosmic_theme::Theme::light_config(),
                        cosmic::cosmic_theme::ThemeBuilder::light_config(),
                    )
                };
                if let (Ok(config), Ok(builder_config)) = configs {
                    _ = config.set("cursor_theme", &cursor_theme);
                    _ = builder_config.set("cursor_theme", &cursor_theme);
                    if let Ok(store) = cosmic_theme_bundle_config::Store::new()
                        && let Err(error) = store.sync_active(Mode::from_is_dark(is_dark))
                    {
                        tracing::warn!(?error, "failed to sync the active theme bundle");
                    }
                }

                theme_changed = self.is_active_mode();
            }

            Message::Roundness(r) => {
                self.roundness = r;
                theme_staged = self
                    .theme_manager
                    .selected_customizer_mut()
                    .set_corner_radii(r.into());
            }

            Message::Reset => {
                let stored = theme_bundles::reset_to_stored(self.mode);
                match stored {
                    Ok(true) => {}
                    Ok(false) => {
                        if let Err(error) = theme_bundles::reset_to_installed(self.mode) {
                            tracing::warn!(?error, "failed to reset to the installed preset");
                        }
                    }
                    Err(error) => {
                        tracing::warn!(?error, "failed to read the look's own preset");
                        if let Err(error) = theme_bundles::reset_to_installed(self.mode) {
                            tracing::warn!(?error, "failed to reset to the installed preset");
                        }
                    }
                }

                self.reload();
                theme_changed = true;

                tasks.push(cosmic::task::future(async {
                    crate::app::Message::SetTheme(cosmic::theme::system_preference())
                }));
            }

            Message::WallpaperPage => {
                tasks.push(cosmic::task::message(crate::Message::WallpaperPage(
                    self.mode,
                )));
            }

            Message::SaveAs => {
                self.save_as = Some(String::new());
                tasks.push(theme_bundles::focus_input(
                    theme_bundles::SAVE_AS_INPUT.clone(),
                    false,
                ));
            }

            Message::SaveAsNameChanged(name) => {
                self.save_as = Some(theme_bundles::limit_theme_name(name));
            }

            Message::SaveAsCancel => self.save_as = None,

            Message::SaveAsConfirm => {
                if let Some(name) = self
                    .save_as
                    .as_ref()
                    .map(|name| theme_bundles::limit_theme_name(name.trim().to_owned()))
                    .filter(|name| !name.is_empty())
                {
                    self.save_as = None;
                    let mode = self.mode;
                    tasks.push(cosmic::task::future(async move {
                        here(Message::SavedAs(theme_bundles::save_live_as(mode, &name)))
                    }));
                }
            }

            Message::SavedAs(result) => match result {
                Ok(_) => {
                    self.reload();
                    theme_changed = true;
                }
                Err(error) => tracing::error!(?error, "failed to save the theme as a new bundle"),
            },

            Message::DrawerFont(message) => {
                let font_selected = matches!(&message, drawer::FontMessage::Select(_));
                let active = font_selected && self.context_view.is_some() && self.is_active_mode();

                tasks.push(self.drawer.update_font(
                    message,
                    self.context_view.as_ref(),
                    &mut self.theme_manager,
                ));

                if active {
                    theme_changed = true;
                }
            }

            Message::DrawerIcon(message) => {
                let icon_selected = matches!(&message, drawer::IconMessage::IconTheme(_));
                let active = icon_selected && self.context_view.is_some() && self.is_active_mode();

                if let Some(context_view) = self.context_view.as_ref() {
                    tasks.push(self.drawer.update_icon(
                        message,
                        context_view,
                        &mut self.theme_manager,
                    ));
                }

                if active {
                    theme_changed = true;
                }
            }

            Message::FocusFontInput => {
                if matches!(
                    self.context_view,
                    Some(ContextView::SystemFont | ContextView::MonospaceFont)
                ) {
                    return cosmic::iced::runtime::task::widget(
                        cosmic::iced::core::widget::operation::focusable::find_focused(),
                    )
                    .collect()
                    .then(|id| {
                        if id.first().is_some_and(|id| *id == FONT_SEARCH.clone()) {
                            Task::none()
                        } else {
                            focus(FONT_SEARCH.clone()).chain(task::message(Message::FocusFontInput))
                        }
                    });
                }
            }

            Message::ExportTheme => {
                #[cfg(feature = "xdg-portal")]
                {
                    let mode = self.mode;
                    tasks.push(cosmic::task::future(async move {
                        let name = theme_bundles::on_screen_name(mode)
                            .filter(|name| !name.is_empty())
                            .map_or_else(
                                || "theme.zip".to_owned(),
                                |name| format!("{}.zip", name.replace(['/', '\\'], "-")),
                            );

                        let chosen = cosmic::dialog::file_chooser::save::Dialog::new()
                            .modal(true)
                            .file_name(name)
                            .save_file()
                            .await;

                        let path_res = match chosen {
                            Ok(file) => file.url().and_then(|url| {
                                (url.scheme() == "file")
                                    .then(|| url.to_file_path().ok())
                                    .flatten()
                            }),
                            Err(why) => {
                                tracing::error!(
                                    ?why,
                                    "failed to select a file to export the theme to"
                                );
                                None
                            }
                        };
                        here(Message::ExportThemeFile(path_res))
                    }));
                }
            }

            Message::ExportThemeFile(Some(path)) => {
                let mode = self.mode;
                tasks.push(cosmic::task::future(async move {
                    here(Message::Exported(theme_bundles::export_live(&path, mode)))
                }));
            }

            Message::Exported(result) => match result {
                Ok(_) => {}
                Err(error) => tracing::error!(?error, "failed to write the look out"),
            },

            _ => {}
        }

        let changed = theme_staged.is_some() || theme_changed;
        let mut tasks = Task::batch(tasks);

        if let Some(stage) = theme_staged {
            tasks = tasks.chain(self.theme_manager.build_theme(stage));
        }

        if changed {
            let preview = self.preview();
            tasks = tasks.chain(cosmic::task::message(preview));
        }

        tasks
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> page::Info {
        page::Info::new("appearance-theme", "preferences-desktop-theme-symbolic")
            .title(fl!("appearance", "title"))
            .description(fl!("appearance", "description"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(wallpaper::section()),
            sections.insert(colors_and_style::section()),
            sections.insert(style::section()),
            sections.insert(font_and_icons()),
            sections.insert(export_action()),
        ])
    }

    fn header_view(&self) -> Option<Element<'_, crate::pages::Message>> {
        let spacing = cosmic::theme::spacing();

        let reset = button::standard(fl!("reset-to-default"))
            .on_press(crate::pages::Message::AppearanceTheme(Message::Reset));
        let save_as = button::standard(fl!("theme-bundles-save-as"))
            .on_press(crate::pages::Message::AppearanceTheme(Message::SaveAs));

        Some(
            cosmic::iced::widget::row![reset, save_as]
                .spacing(spacing.space_xs)
                .align_y(Alignment::End)
                .into(),
        )
    }

    fn dialog(&self) -> Option<Element<'_, crate::pages::Message>> {
        self.save_as.as_ref().map(|name| {
            theme_bundles::save_as_dialog(name).map(crate::pages::Message::AppearanceTheme)
        })
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        self.drawer
            .context_drawer(self.context_view, crate::pages::Message::AppearanceTheme)
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        self.reload();
        let fonts = cosmic::task::future(async {
            let (interface, mono) = font_config::load_font_families();
            Message::DrawerFont(drawer::FontMessage::FontLoaded(interface, mono))
        })
        .map(crate::pages::Message::AppearanceTheme);

        Task::batch(vec![cosmic::task::message(self.preview()), fonts])
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        self.save_as = None;
        Task::batch(vec![
            self.drawer.on_leave(),
            cosmic::task::message(crate::pages::Message::AppearancePreview(None)),
        ])
    }
}

fn font_and_icons() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        interface_font_txt = fl!("interface-font");
        monospace_font_txt = fl!("monospace-font");
        icons_theme_txt = fl!("icons-theme");
        cursor_theme_txt = fl!("cursor-theme");
    });

    Section::default()
        .title(fl!("font-and-icons"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            let system_font = crate::widget::go_next_with_item(
                &descriptions[interface_font_txt],
                text::body(page.drawer.current_font_family(&ContextView::SystemFont)),
                Message::DrawerOpen(ContextView::SystemFont),
            );

            let mono_font = crate::widget::go_next_with_item(
                &descriptions[monospace_font_txt],
                text::body(page.drawer.current_font_family(&ContextView::MonospaceFont)),
                Message::DrawerOpen(ContextView::MonospaceFont),
            );

            let icon_theme = crate::widget::go_next_item(
                &descriptions[icons_theme_txt],
                Message::DrawerOpen(ContextView::IconThemes),
            );

            let cursor_theme = crate::widget::go_next_item(
                &descriptions[cursor_theme_txt],
                Message::DrawerOpen(ContextView::CursorThemes),
            );

            settings::section()
                .title(&*section.title)
                .add(system_font)
                .add(mono_font)
                .add(icon_theme)
                .add(cursor_theme)
                .apply(Element::from)
                .map(crate::pages::Message::AppearanceTheme)
        })
}

fn export_action() -> Section<crate::pages::Message> {
    Section::default().search_ignore().view::<Page>(|_, _, _| {
        container(
            button::standard(fl!("export"))
                .on_press(crate::pages::Message::AppearanceTheme(Message::ExportTheme)),
        )
        .width(Length::Fill)
        .align_x(Alignment::End)
        .apply(Element::from)
    })
}

/// Installed cursor themes as (directory id, display name), sorted by id.
pub fn cursor_themes() -> Vec<(String, String)> {
    let home = dirs::home_dir();
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .filter(|v| !v.is_empty())
        .map(std::path::PathBuf::from)
        .or_else(|| home.as_ref().map(|h| h.join(".local/share")));
    let data_dirs = std::env::var_os("XDG_DATA_DIRS")
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "/usr/local/share:/usr/share".into());

    let search = home
        .map(|h| h.join(".icons"))
        .into_iter()
        .chain(data_home.map(|d| d.join("icons")))
        .chain(std::env::split_paths(&data_dirs).map(|d| d.join("icons")))
        .chain(std::iter::once("/usr/share/pixmaps".into()));

    let mut themes = std::collections::BTreeMap::new();
    for dir in search {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.join("cursors").is_dir()
                && let Ok(id) = entry.file_name().into_string()
            {
                // First match wins, that's how cursor loading works too. So keeping them insync.
                themes.entry(id).or_insert_with_key(|id| {
                    std::fs::read_to_string(path.join("index.theme"))
                        .ok()
                        .and_then(|index| {
                            index
                                .lines()
                                .find_map(|l| l.strip_prefix("Name="))
                                .map(|n| n.trim().to_owned())
                        })
                        .filter(|n| !n.is_empty())
                        .unwrap_or_else(|| id.clone())
                });
            }
        }
    }
    themes.into_iter().collect()
}

/// Cursor themes with preview icons
/// NOTE: the first entry is the system default, with an empty id.
pub fn load_cursor_themes() -> Vec<(IconTheme, Vec<icon::Handle>)> {
    let default = IconTheme {
        id: String::new(),
        name: fl!("default"),
    };
    std::iter::once((default, cursor_previews("default")))
        .chain(cursor_themes().into_iter().map(|(id, name)| {
            let previews = cursor_previews(&id);
            (IconTheme { id, name }, previews)
        }))
        .collect()
}

/// Each slot lists the legacy X11 name and its CSS alias, since themes may ship either.
const CURSOR_PREVIEWS: [[&str; 2]; 6] = [
    ["left_ptr", "default"],
    ["hand2", "pointer"],
    ["xterm", "text"],
    ["watch", "wait"],
    ["crosshair", "cross"],
    ["fleur", "move"],
];

fn cursor_previews(theme: &str) -> Vec<icon::Handle> {
    let theme = xcursor::CursorTheme::load(theme);
    CURSOR_PREVIEWS
        .iter()
        .filter_map(|names| {
            let path = names.iter().find_map(|name| theme.load_icon(name))?;
            let images = xcursor::parser::parse_xcursor(&std::fs::read(path).ok()?)?;
            // First frame of a large size, so the preview scales down rather than up.
            let image = images.into_iter().min_by_key(|i| i.size.abs_diff(64))?;
            let (side, pixels) = crop_to_square(
                &xcursor_to_rgba(&image.pixels_rgba),
                image.width as usize,
                image.height as usize,
            );
            Some(icon::from_raster_pixels(side as u32, side as u32, pixels))
        })
        .collect()
}

/// XCursor pixels are premultiplied little-endian ARGB, i.e. BGRA bytes in file order.
fn xcursor_to_rgba(bgra: &[u8]) -> Vec<u8> {
    bgra.chunks_exact(4)
        .flat_map(|p| {
            let a = u16::from(p[3]);
            let un = |c: u8| {
                (u16::from(c) * 255)
                    .checked_div(a)
                    .map_or(0, |v| v.min(255) as u8)
            };
            [un(p[2]), un(p[1]), un(p[0]), p[3]]
        })
        .collect()
}

/// Trims the transparent margin (cursors pad around the hotspot)
fn crop_to_square(rgba: &[u8], width: usize, height: usize) -> (usize, Vec<u8>) {
    let opaque = |x: usize, y: usize| rgba[(y * width + x) * 4 + 3] > 0;
    let (mut x0, mut y0, mut x1, mut y1) = (width, height, 0, 0);
    for y in 0..height {
        for x in 0..width {
            if opaque(x, y) {
                (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x + 1), y1.max(y + 1));
            }
        }
    }
    if x0 >= x1 {
        return (width.max(height), rgba.to_vec());
    }

    let (w, h) = (x1 - x0, y1 - y0);
    let side = w.max(h);
    let (dx, dy) = ((side - w) / 2, (side - h) / 2);
    let mut out = vec![0; side * side * 4];
    for y in 0..h {
        let src = ((y0 + y) * width + x0) * 4;
        let dst = ((dy + y) * side + dx) * 4;
        out[dst..dst + w * 4].copy_from_slice(&rgba[src..src + w * 4]);
    }
    (side, out)
}
