// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::app::ContextDrawer;
use cosmic::cosmic_theme::Roundness;
use cosmic::cosmic_theme::palette::{FromColor, Hsv, Srgb};
use cosmic::iced::core::{Alignment, Length};
use cosmic::widget::color_picker::ColorPickerUpdate;
use cosmic::widget::{button, container};
use cosmic::{Apply, Element, Task};
use cosmic_settings_page::{self as page, Section, section};
use cosmic_theme_bundle_config::Mode;
use slotmap::{Key, SlotMap};

use super::theme_manager;
use super::{ContextView, Message, colors_and_style, drawer, style, theme_bundles, wallpaper};

pub struct Page {
    pub(crate) entity: page::Entity,
    pub(crate) mode: Mode,
    pub(crate) theme_manager: theme_manager::Manager,
    pub(crate) drawer: drawer::Content,
    pub(crate) context_view: Option<ContextView>,
    pub(crate) roundness: Roundness,
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
        self.context_view = None;
    }

    fn preview_theme(&self) -> cosmic::theme::Theme {
        let theme = self.theme_manager.builder().clone().build();
        cosmic::theme::Theme::custom(std::sync::Arc::new(theme))
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
                tasks.push(self.drawer.on_open(&context_view));
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

            Message::Roundness(r) => {
                self.roundness = r;

                let radii = self.roundness.into();
                theme_staged = self.theme_manager.set_corner_radii(radii);

                #[cfg(feature = "wayland")]
                if self.mode == theme_bundles::on_screen_mode() {
                    tokio::task::spawn(async move {
                        super::Page::update_dock_padding(r);
                        super::Page::update_panel_radii(r);
                    });
                }
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

                        let path = match chosen {
                            Ok(file) => file
                                .0
                                .uris()
                                .first()
                                .filter(|uri| uri.scheme() == "file")
                                .and_then(|uri| uri.to_file_path().ok()),
                            Err(why) => {
                                tracing::error!(
                                    ?why,
                                    "failed to select a file to export the theme to"
                                );
                                None
                            }
                        };
                        here(Message::ExportThemeFile(path))
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
        ])
    }

    fn header_view(&self) -> Option<Element<'_, crate::pages::Message>> {
        let spacing = cosmic::theme::spacing();

        let reset = button::standard(fl!("reset-to-default"))
            .on_press(crate::pages::Message::AppearanceTheme(Message::Reset));
        let export = button::standard(fl!("export"))
            .on_press(crate::pages::Message::AppearanceTheme(Message::ExportTheme));

        Some(
            container(
                cosmic::iced::widget::row![reset, export]
                    .spacing(spacing.space_xs)
                    .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .align_x(Alignment::End)
            .apply(Element::from),
        )
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        self.drawer
            .context_drawer(self.context_view, crate::pages::Message::AppearanceTheme)
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        self.reload();
        cosmic::task::message(self.preview())
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        cosmic::task::message(crate::pages::Message::AppearancePreview(None))
    }
}
