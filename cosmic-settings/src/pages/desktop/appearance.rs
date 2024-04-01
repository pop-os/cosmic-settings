// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::borrow::Cow;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
use cosmic::config::CosmicTk;
use cosmic::cosmic_config::{Config, ConfigSet, CosmicConfigEntry};
use cosmic::cosmic_theme::palette::{FromColor, Hsv, Srgb, Srgba};
use cosmic::cosmic_theme::{
    CornerRadii, Theme, ThemeBuilder, ThemeMode, DARK_THEME_BUILDER_ID, LIGHT_THEME_BUILDER_ID,
};
use cosmic::iced_core::{alignment, Color, Length};
use cosmic::iced_widget::scrollable;
use cosmic::prelude::CollectionWidget;
use cosmic::widget::dropdown;
use cosmic::widget::icon::{from_name, icon};
use cosmic::widget::{
    button, color_picker::ColorPickerUpdate, container, horizontal_space, row, settings,
    spin_button, text, ColorPickerModel,
};
use cosmic::Apply;
use cosmic::{command, Command, Element};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use cosmic_settings_wallpaper as wallpaper;
use ron::ser::PrettyConfig;
use slotmap::SlotMap;
use tokio::io::AsyncBufReadExt;

use crate::app;

use super::wallpaper::widgets::color_image;

type IconThemes = Vec<String>;

crate::cache_dynamic_lazy! {
    static HEX: String = fl!("hex");
    static RGB: String = fl!("rgb");
    static RESET_TO_DEFAULT: String = fl!("reset-to-default");
    static ICON_THEME: String = fl!("icon-theme");
    static ICON_THEME_DESC: String = fl!("icon-theme", "desc");
}

#[derive(Clone, Copy, Debug)]
enum ContextView {
    AccentWindowHint,
    ApplicationBackground,
    ContainerBackground,
    ControlComponent,
    CustomAccent,
    InterfaceText,
}

pub struct Page {
    can_reset: bool,
    no_custom_window_hint: bool,
    context_view: Option<ContextView>,
    custom_accent: ColorPickerModel,
    accent_window_hint: ColorPickerModel,
    application_background: ColorPickerModel,
    container_background: ColorPickerModel,
    interface_text: ColorPickerModel,
    control_component: ColorPickerModel,
    roundness: Roundness,

    icon_theme_active: Option<usize>,
    icon_themes: Vec<String>,

    theme_mode: ThemeMode,
    theme_mode_config: Option<Config>,
    theme_builder: ThemeBuilder,
    theme_builder_needs_update: bool,
    theme_builder_config: Option<Config>,

    auto_switch_descs: [Cow<'static, str>; 4],

    tk: CosmicTk,
    tk_config: Option<Config>,

    day_time: bool,
}

impl Default for Page {
    fn default() -> Self {
        let theme_mode_config = ThemeMode::config().ok();
        let theme_mode = theme_mode_config
            .as_ref()
            .map(|c| match ThemeMode::get_entry(c) {
                Ok(t) => t,
                Err((errors, t)) => {
                    for e in errors {
                        tracing::error!("{e}");
                    }
                    t
                }
            })
            .unwrap_or_default();

        (theme_mode_config, theme_mode).into()
    }
}

impl
    From<(
        Option<Config>,
        ThemeMode,
        Option<Config>,
        ThemeBuilder,
        Option<Config>,
        CosmicTk,
    )> for Page
{
    fn from(
        (theme_mode_config, theme_mode, theme_builder_config, theme_builder, tk_config, tk): (
            Option<Config>,
            ThemeMode,
            Option<Config>,
            ThemeBuilder,
            Option<Config>,
            CosmicTk,
        ),
    ) -> Self {
        let theme = if theme_mode.is_dark {
            Theme::dark_default()
        } else {
            Theme::light_default()
        };
        let custom_accent = theme_builder.accent.filter(|c| {
            let c = Srgba::new(c.red, c.green, c.blue, 1.0);
            c != theme.palette.accent_blue
                && c != theme.palette.accent_green
                && c != theme.palette.accent_indigo
                && c != theme.palette.accent_orange
                && c != theme.palette.accent_pink
                && c != theme.palette.accent_purple
                && c != theme.palette.accent_red
                && c != theme.palette.accent_warm_grey
                && c != theme.palette.accent_yellow
        });

        Self {
            can_reset: if theme_mode.is_dark {
                theme_builder == ThemeBuilder::dark()
            } else {
                theme_builder == ThemeBuilder::light()
            },
            theme_builder_needs_update: false,
            context_view: None,
            roundness: theme_builder.corner_radii.into(),
            custom_accent: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                None,
                custom_accent.map(Color::from),
            ),
            application_background: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                Some(theme.background.base.into()),
                theme_builder.bg_color.map(Color::from),
            ),
            container_background: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                None,
                theme_builder.primary_container_bg.map(Color::from),
            ),
            interface_text: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                Some(theme.background.on.into()),
                theme_builder.text_tint.map(Color::from),
            ),
            control_component: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                Some(theme.palette.neutral_5.into()),
                theme_builder.neutral_tint.map(Color::from),
            ),
            accent_window_hint: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                None,
                theme_builder.window_hint.map(Color::from),
            ),
            no_custom_window_hint: theme_builder.accent.is_some(),
            icon_theme_active: None,
            icon_themes: Vec::new(),
            theme_mode_config,
            theme_builder_config,
            theme_mode,
            theme_builder,
            tk_config,
            tk,
            day_time: true,
            auto_switch_descs: [
                fl!("auto-switch", "sunrise").into(),
                fl!("auto-switch", "sunset").into(),
                fl!("auto-switch", "next-sunrise").into(),
                fl!("auto-switch", "next-sunset").into(),
            ],
        }
    }
}

impl From<(Option<Config>, ThemeMode)> for Page {
    fn from((theme_mode_config, theme_mode): (Option<Config>, ThemeMode)) -> Self {
        let theme_builder_config = if theme_mode.is_dark {
            ThemeBuilder::dark_config()
        } else {
            ThemeBuilder::light_config()
        }
        .ok();
        let theme_builder = theme_builder_config.as_ref().map_or_else(
            || {
                if theme_mode.is_dark {
                    ThemeBuilder::dark()
                } else {
                    ThemeBuilder::light()
                }
            },
            |c| match ThemeBuilder::get_entry(c) {
                Ok(t) => t,
                Err((errors, t)) => {
                    for e in errors {
                        tracing::error!("{e}");
                    }
                    t
                }
            },
        );

        let tk_config = CosmicTk::config().ok();
        let tk = match tk_config.as_ref().map(CosmicTk::get_entry) {
            Some(Ok(c)) => c,
            Some(Err((errs, c))) => {
                for err in errs {
                    tracing::error!(?err, "Error loading CosmicTk");
                }
                c
            }
            None => CosmicTk::default(),
        };
        (
            theme_mode_config,
            theme_mode,
            theme_builder_config,
            theme_builder,
            tk_config,
            tk,
        )
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    AccentWindowHint(ColorPickerUpdate),
    ApplicationBackground(ColorPickerUpdate),
    ApplyThemeGlobal(bool),
    Autoswitch(bool),
    ContainerBackground(ColorPickerUpdate),
    ControlComponent(ColorPickerUpdate),
    CustomAccent(ColorPickerUpdate),
    DarkMode(bool),
    Entered(IconThemes),
    ExportError,
    ExportFile(Arc<SelectedFiles>),
    ExportSuccess,
    Frosted(bool),
    GapSize(spin_button::Message),
    IconTheme(usize),
    ImportError,
    ImportFile(Arc<SelectedFiles>),
    ImportSuccess(Box<ThemeBuilder>),
    InterfaceText(ColorPickerUpdate),
    Left,
    PaletteAccent(cosmic::iced::Color),
    Reset,
    Roundness(Roundness),
    StartExport,
    StartImport,
    UseDefaultWindowHint(bool),
    WindowHintSize(spin_button::Message),
    Daytime(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum Roundness {
    Round,
    SlightlyRound,
    Square,
}

impl From<Roundness> for CornerRadii {
    fn from(value: Roundness) -> Self {
        match value {
            Roundness::Round => CornerRadii {
                radius_0: [0.0; 4],
                radius_xs: [4.0; 4],
                radius_s: [8.0; 4],
                radius_m: [16.0; 4],
                radius_l: [32.0; 4],
                radius_xl: [160.0; 4],
            },
            Roundness::SlightlyRound => CornerRadii {
                radius_0: [0.0; 4],
                radius_xs: [2.0; 4],
                radius_s: [8.0; 4],
                radius_m: [8.0; 4],
                radius_l: [8.0; 4],
                radius_xl: [8.0; 4],
            },
            Roundness::Square => CornerRadii {
                radius_0: [0.0; 4],
                radius_xs: [2.0; 4],
                radius_s: [2.0; 4],
                radius_m: [2.0; 4],
                radius_l: [2.0; 4],
                radius_xl: [2.0; 4],
            },
        }
    }
}

impl From<CornerRadii> for Roundness {
    fn from(value: CornerRadii) -> Self {
        if (value.radius_m[0] - 16.0).abs() < 0.01 {
            Self::Round
        } else if (value.radius_m[0] - 8.0).abs() < 0.01 {
            Self::SlightlyRound
        } else {
            Self::Square
        }
    }
}

impl Page {
    /// Syncs changes for dark and light theme.
    /// Roundness and window management settings should be consistent between dark / light mode.
    fn sync_changes(&self) -> Result<(), cosmic::cosmic_config::Error> {
        let (other_builder_config, other_theme_config) = if self.theme_mode.is_dark {
            (ThemeBuilder::light_config()?, Theme::light_config()?)
        } else {
            (ThemeBuilder::dark_config()?, Theme::dark_config()?)
        };

        let mut theme_builder = match ThemeBuilder::get_entry(&other_builder_config) {
            Ok(t) => t,
            Err((errs, t)) => {
                for err in errs {
                    tracing::error!(?err, "Error loading theme builder");
                }
                t
            }
        };
        let mut theme = match Theme::get_entry(&other_theme_config) {
            Ok(t) => t,
            Err((errs, t)) => {
                for err in errs {
                    tracing::error!(?err, "Error loading theme");
                }
                t
            }
        };
        if theme_builder.active_hint != self.theme_builder.active_hint {
            if let Err(err) =
                theme_builder.set_active_hint(&other_builder_config, self.theme_builder.active_hint)
            {
                tracing::error!(?err, "Error setting active hint");
            }
            if let Err(err) =
                theme.set_active_hint(&other_theme_config, self.theme_builder.active_hint)
            {
                tracing::error!(?err, "Error setting active hint");
            }
        }
        if theme_builder.gaps != self.theme_builder.gaps {
            if let Err(err) = theme_builder.set_gaps(&other_builder_config, self.theme_builder.gaps)
            {
                tracing::error!(?err, "Error setting gaps");
            }
            if let Err(err) = theme.set_gaps(&other_theme_config, self.theme_builder.gaps) {
                tracing::error!(?err, "Error setting gaps");
            }
        }
        if theme_builder.corner_radii != self.theme_builder.corner_radii {
            if let Err(err) = theme_builder
                .set_corner_radii(&other_builder_config, self.theme_builder.corner_radii)
            {
                tracing::error!(?err, "Error setting corner radii");
            }

            if let Err(err) =
                theme.set_corner_radii(&other_theme_config, self.theme_builder.corner_radii)
            {
                tracing::error!(?err, "Error setting corner radii");
            }
        }

        Ok(())
    }

    fn color_picker_context_view(
        &self,
        description: Option<Cow<'static, str>>,
        reset: Cow<'static, str>,
        on_update: fn(ColorPickerUpdate) -> Message,
        model: impl Fn(&Self) -> &ColorPickerModel,
    ) -> Element<'_, crate::pages::Message> {
        cosmic::widget::column()
            .push_maybe(description.map(|description| text(description).width(Length::Fill)))
            .push(
                model(self)
                    .builder(on_update)
                    .reset_label(reset)
                    .height(Length::Fixed(158.0))
                    .build(
                        fl!("recent-colors"),
                        fl!("copy-to-clipboard"),
                        fl!("copied-to-clipboard"),
                    )
                    .apply(container)
                    .width(Length::Fixed(248.0))
                    .align_x(alignment::Horizontal::Center)
                    .apply(container)
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center),
            )
            .padding(self.theme_builder.spacing.space_l)
            .align_items(cosmic::iced_core::Alignment::Center)
            .spacing(self.theme_builder.spacing.space_m)
            .width(Length::Fill)
            .apply(Element::from)
            .map(crate::pages::Message::Appearance)
    }

    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        self.theme_builder_needs_update = false;
        let mut needs_sync = false;
        let ret = match message {
            Message::DarkMode(enabled) => {
                if let Some(config) = self.theme_mode_config.as_ref() {
                    if let Err(err) = self.theme_mode.set_is_dark(config, enabled) {
                        tracing::error!(?err, "Error setting dark mode");
                    }
                }
                Command::none()
            }
            Message::Autoswitch(enabled) => {
                self.theme_mode.auto_switch = enabled;
                if let Some(config) = self.theme_mode_config.as_ref() {
                    _ = config.set::<bool>("auto_switch", enabled);
                }
                Command::none()
            }
            Message::AccentWindowHint(u) => {
                needs_sync = true;
                let cmd = self.update_color_picker(
                    &u,
                    ContextView::AccentWindowHint,
                    fl!("window-hint-accent").into(),
                );
                Command::batch(vec![cmd, self.accent_window_hint.update::<app::Message>(u)])
            }
            Message::Frosted(enabled) => {
                self.theme_builder_needs_update = true;
                self.theme_builder.is_frosted = enabled;
                Command::none()
            }
            Message::IconTheme(id) => {
                if let Some(theme) = self.icon_themes.get(id).cloned() {
                    self.icon_theme_active = Some(id);
                    self.tk.icon_theme = theme.clone();

                    if let Some(ref config) = self.tk_config {
                        let _ = self.tk.write_entry(config);
                    }

                    tokio::spawn(set_gnome_icon_theme(theme));
                }

                Command::none()
            }
            Message::WindowHintSize(msg) => {
                needs_sync = true;
                self.theme_builder_needs_update = true;
                self.theme_builder.active_hint = match msg {
                    spin_button::Message::Increment => {
                        self.theme_builder.active_hint.saturating_add(1)
                    }
                    spin_button::Message::Decrement => {
                        self.theme_builder.active_hint.saturating_sub(1)
                    }
                };
                Command::none()
            }
            Message::GapSize(msg) => {
                needs_sync = true;
                self.theme_builder_needs_update = true;
                self.theme_builder.gaps.1 = match msg {
                    spin_button::Message::Increment => self.theme_builder.gaps.1.saturating_add(1),
                    spin_button::Message::Decrement => self.theme_builder.gaps.1.saturating_sub(1),
                };
                Command::none()
            }
            Message::ApplicationBackground(u) => {
                let cmd = self.update_color_picker(
                    &u,
                    ContextView::ApplicationBackground,
                    fl!("app-background").into(),
                );

                Command::batch(vec![
                    cmd,
                    self.application_background.update::<app::Message>(u),
                ])
            }
            Message::ContainerBackground(u) => {
                let cmd = self.update_color_picker(
                    &u,
                    ContextView::ContainerBackground,
                    fl!("container-background").into(),
                );

                Command::batch(vec![
                    cmd,
                    self.container_background.update::<app::Message>(u),
                ])
            }
            Message::CustomAccent(u) => {
                let cmd = self.update_color_picker(
                    &u,
                    ContextView::CustomAccent,
                    fl!("accent-color").into(),
                );

                let cmd2 = self.custom_accent.update::<app::Message>(u);

                self.theme_builder.accent = self.custom_accent.get_applied_color().map(Srgb::from);
                Command::batch(vec![cmd, cmd2])
            }
            Message::InterfaceText(u) => {
                let cmd = self.update_color_picker(
                    &u,
                    ContextView::InterfaceText,
                    fl!("text-tint").into(),
                );

                Command::batch(vec![cmd, self.interface_text.update::<app::Message>(u)])
            }
            Message::ControlComponent(u) => {
                let cmd = self.update_color_picker(
                    &u,
                    ContextView::ControlComponent,
                    fl!("control-tint").into(),
                );
                Command::batch(vec![cmd, self.control_component.update::<app::Message>(u)])
            }
            Message::Roundness(r) => {
                needs_sync = true;
                self.roundness = r;
                self.theme_builder.corner_radii = self.roundness.into();
                self.theme_builder_needs_update = true;
                Command::none()
            }
            Message::Entered(icon_themes) => {
                *self = Self::default();

                // Set the icon themes, and define the active icon theme.
                self.icon_themes = icon_themes;
                self.icon_theme_active = self
                    .icon_themes
                    .iter()
                    .position(|theme| theme == &self.tk.icon_theme);
                Command::none()
            }
            Message::Left => Command::perform(async {}, |()| {
                app::Message::SetTheme(cosmic::theme::system_preference())
            }),
            Message::PaletteAccent(c) => {
                self.theme_builder.accent = Some(c.into());
                self.theme_builder_needs_update = true;
                Command::none()
            }
            Message::Reset => {
                self.theme_builder = if self.theme_mode.is_dark {
                    cosmic::cosmic_config::Config::system(
                        DARK_THEME_BUILDER_ID,
                        ThemeBuilder::VERSION,
                    )
                    .map_or_else(
                        |_| ThemeBuilder::dark(),
                        |config| match ThemeBuilder::get_entry(&config) {
                            Ok(t) => t,
                            Err((errs, t)) => {
                                for err in errs {
                                    tracing::warn!(?err, "Error getting system theme builder");
                                }
                                t
                            }
                        },
                    )
                } else {
                    cosmic::cosmic_config::Config::system(
                        LIGHT_THEME_BUILDER_ID,
                        ThemeBuilder::VERSION,
                    )
                    .map_or_else(
                        |_| ThemeBuilder::light(),
                        |config| match ThemeBuilder::get_entry(&config) {
                            Ok(t) => t,
                            Err((errs, t)) => {
                                for err in errs {
                                    tracing::warn!(?err, "Error getting system theme builder");
                                }
                                t
                            }
                        },
                    )
                };
                if let Some(config) = self.theme_builder_config.as_ref() {
                    _ = self.theme_builder.write_entry(config);
                };

                let config = if self.theme_mode.is_dark {
                    Theme::dark_config()
                } else {
                    Theme::light_config()
                };
                let new_theme = self.theme_builder.clone().build();
                if let Ok(config) = config {
                    _ = new_theme.write_entry(&config);
                } else {
                    tracing::error!("Failed to get the theme config.");
                }

                self.reload_theme_mode();
                Command::none()
            }
            Message::StartImport => Command::perform(
                async {
                    SelectedFiles::open_file()
                        .modal(true)
                        .filter(FileFilter::glob(FileFilter::new("ron"), "*.ron"))
                        .send()
                        .await?
                        .response()
                },
                |res| {
                    if let Ok(f) = res {
                        crate::Message::PageMessage(crate::pages::Message::Appearance(
                            Message::ImportFile(Arc::new(f)),
                        ))
                    } else {
                        // TODO Error toast?
                        tracing::error!("failed to select a file for importing a custom theme.");
                        crate::Message::PageMessage(crate::pages::Message::Appearance(
                            Message::ImportError,
                        ))
                    }
                },
            ),
            Message::StartExport => {
                let is_dark = self.theme_mode.is_dark;
                let name = format!("{}.ron", if is_dark { fl!("dark") } else { fl!("light") });
                Command::perform(
                    async move {
                        SelectedFiles::save_file()
                            .modal(true)
                            .current_name(Some(name.as_str()))
                            .send()
                            .await?
                            .response()
                    },
                    |res| {
                        if let Ok(f) = res {
                            crate::Message::PageMessage(crate::pages::Message::Appearance(
                                Message::ExportFile(Arc::new(f)),
                            ))
                        } else {
                            // TODO Error toast?
                            tracing::error!(
                                "failed to select a file for importing a custom theme."
                            );
                            crate::Message::PageMessage(crate::pages::Message::Appearance(
                                Message::ExportError,
                            ))
                        }
                    },
                )
            }
            Message::ImportFile(f) => {
                let Some(f) = f.uris().first() else {
                    return Command::none();
                };
                if f.scheme() != "file" {
                    return Command::none();
                }
                let Ok(path) = f.to_file_path() else {
                    return Command::none();
                };
                Command::perform(
                    async move { tokio::fs::read_to_string(path).await },
                    |res| {
                        if let Some(b) = res.ok().and_then(|s| ron::de::from_str(&s).ok()) {
                            crate::Message::PageMessage(crate::pages::Message::Appearance(
                                Message::ImportSuccess(Box::new(b)),
                            ))
                        } else {
                            // TODO Error toast?
                            tracing::error!("failed to import a file for a custom theme.");
                            crate::Message::PageMessage(crate::pages::Message::Appearance(
                                Message::ImportError,
                            ))
                        }
                    },
                )
            }
            Message::ExportFile(f) => {
                let Some(f) = f.uris().first() else {
                    return Command::none();
                };
                if f.scheme() != "file" {
                    return Command::none();
                }
                let Ok(path) = f.to_file_path() else {
                    return Command::none();
                };
                let Ok(builder) =
                    ron::ser::to_string_pretty(&self.theme_builder, PrettyConfig::default())
                else {
                    return Command::none();
                };
                Command::perform(
                    async move { tokio::fs::write(path, builder).await },
                    |res| {
                        if res.is_ok() {
                            crate::Message::PageMessage(crate::pages::Message::Appearance(
                                Message::ExportSuccess,
                            ))
                        } else {
                            // TODO Error toast?
                            tracing::error!(
                                "failed to select a file for importing a custom theme."
                            );
                            crate::Message::PageMessage(crate::pages::Message::Appearance(
                                Message::ExportError,
                            ))
                        }
                    },
                )
            }
            // TODO: error message toast?
            Message::ExportError | Message::ImportError => Command::none(),
            Message::ExportSuccess => {
                tracing::trace!("Export successful");
                Command::none()
            }
            Message::ImportSuccess(builder) => {
                tracing::trace!("Import successful");
                self.theme_builder = *builder;

                if let Some(config) = self.theme_builder_config.as_ref() {
                    _ = self.theme_builder.write_entry(config);
                };

                let config = if self.theme_mode.is_dark {
                    Theme::dark_config()
                } else {
                    Theme::light_config()
                };
                let new_theme = self.theme_builder.clone().build();
                if let Ok(config) = config {
                    _ = new_theme.write_entry(&config);
                } else {
                    tracing::error!("Failed to get the theme config.");
                }

                self.reload_theme_mode();
                Command::none()
            }
            Message::UseDefaultWindowHint(v) => {
                self.no_custom_window_hint = v;
                self.theme_builder_needs_update = true;
                let theme = if self.theme_mode.is_dark {
                    Theme::dark_default()
                } else {
                    Theme::light_default()
                };
                if !v {
                    let window_hint = self
                        .theme_builder
                        .window_hint
                        .filter(|c| {
                            let c = Srgba::new(c.red, c.green, c.blue, 1.0);
                            c != theme.palette.accent_blue
                                && c != theme.palette.accent_green
                                && c != theme.palette.accent_indigo
                                && c != theme.palette.accent_orange
                                && c != theme.palette.accent_pink
                                && c != theme.palette.accent_purple
                                && c != theme.palette.accent_red
                                && c != theme.palette.accent_warm_grey
                                && c != theme.palette.accent_yellow
                        })
                        .unwrap_or(
                            self.custom_accent
                                .get_applied_color()
                                .unwrap_or_default()
                                .into(),
                        );
                    _ = self.accent_window_hint.update::<app::Message>(
                        ColorPickerUpdate::ActiveColor(Hsv::from_color(window_hint)),
                    );
                };
                Command::none()
            }
            Message::ApplyThemeGlobal(enabled) => {
                if let Some(tk_config) = self.tk_config.as_ref() {
                    _ = self.tk.set_apply_theme_global(tk_config, enabled);
                } else {
                    tracing::error!("Failed to apply theme to GNOME config because the CosmicTK config does not exist.");
                }
                Command::none()
            }
            Message::Daytime(day_time) => {
                self.day_time = day_time;
                Command::none()
            }
        };

        if self.theme_builder_needs_update {
            let Some(config) = self.theme_builder_config.as_ref() else {
                return ret;
            };
            let mut theme_builder = std::mem::take(&mut self.theme_builder);
            theme_builder.bg_color = self
                .application_background
                .get_applied_color()
                .map(Srgba::from);
            theme_builder.primary_container_bg = self
                .container_background
                .get_applied_color()
                .map(Srgba::from);
            theme_builder.text_tint = self.interface_text.get_applied_color().map(Srgb::from);
            theme_builder.neutral_tint = self.control_component.get_applied_color().map(Srgb::from);
            theme_builder.window_hint = if self.no_custom_window_hint {
                None
            } else {
                self.accent_window_hint.get_applied_color().map(Srgb::from)
            };

            _ = theme_builder.write_entry(config);

            self.theme_builder = theme_builder;

            let config = if self.theme_mode.is_dark {
                Theme::dark_config()
            } else {
                Theme::light_config()
            };
            if let Ok(config) = config {
                let new_theme = self.theme_builder.clone().build();
                _ = new_theme.write_entry(&config);
            } else {
                tracing::error!("Failed to get the theme config.");
            }
        }

        self.can_reset = if self.theme_mode.is_dark {
            self.theme_builder != ThemeBuilder::dark()
        } else {
            self.theme_builder != ThemeBuilder::light()
        };

        if needs_sync {
            if let Err(err) = self.sync_changes() {
                tracing::error!(?err, "Error syncing theme changes.");
            }
        }

        ret
    }

    fn reload_theme_mode(&mut self) {
        let icon_themes = std::mem::take(&mut self.icon_themes);
        let icon_theme_active = self.icon_theme_active.take();
        let day_time = self.day_time;
        *self = Self::from((self.theme_mode_config.clone(), self.theme_mode));
        self.day_time = day_time;
        self.icon_themes = icon_themes;
        self.icon_theme_active = icon_theme_active;
    }

    fn update_color_picker(
        &mut self,
        message: &ColorPickerUpdate,
        context_view: ContextView,
        context_title: Cow<'static, str>,
    ) -> Command<app::Message> {
        match message {
            ColorPickerUpdate::AppliedColor | ColorPickerUpdate::Reset => {
                self.theme_builder_needs_update = true;
                cosmic::command::message(crate::app::Message::CloseContextDrawer)
            }

            ColorPickerUpdate::ActionFinished => {
                self.theme_builder_needs_update = true;
                Command::none()
            }

            ColorPickerUpdate::Cancel => {
                cosmic::command::message(crate::app::Message::CloseContextDrawer)
            }

            ColorPickerUpdate::ToggleColorPicker => {
                self.context_view = Some(context_view);
                cosmic::command::message(crate::app::Message::OpenContextDrawer(context_title))
            }

            _ => Command::none(),
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(mode_and_colors()),
            sections.insert(style()),
            sections.insert(window_management()),
            sections.insert(reset_button()),
        ])
    }

    fn header_view(&self) -> Option<Element<'_, crate::pages::Message>> {
        let spacing = self.theme_builder.spacing;
        let content = row::with_capacity(2)
            .spacing(self.theme_builder.spacing.space_xxs)
            .push(
                button(text(fl!("import")))
                    .on_press(Message::StartImport)
                    .padding([spacing.space_xxs, spacing.space_xs]),
            )
            .push(
                button(text(fl!("export")))
                    .on_press(Message::StartExport)
                    .padding([spacing.space_xxs, spacing.space_xs]),
            )
            .apply(container)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .apply(Element::from)
            .map(crate::pages::Message::Appearance);

        Some(content)
    }

    fn info(&self) -> page::Info {
        page::Info::new("appearance", "preferences-appearance-symbolic")
            .title(fl!("appearance"))
            .description(fl!("appearance", "desc"))
    }

    fn reload(&mut self, _: page::Entity) -> Command<crate::pages::Message> {
        command::future(fetch_icon_themes()).map(crate::pages::Message::Appearance)
    }

    fn on_leave(&mut self) -> Command<crate::pages::Message> {
        command::message(crate::pages::Message::Appearance(Message::Left))
    }

    fn context_drawer(&self) -> Option<Element<'_, crate::pages::Message>> {
        let view = match self.context_view? {
            ContextView::AccentWindowHint => self.color_picker_context_view(
                None,
                RESET_TO_DEFAULT.as_str().into(),
                Message::AccentWindowHint,
                |this| &this.accent_window_hint,
            ),

            ContextView::ApplicationBackground => self.color_picker_context_view(
                None,
                RESET_TO_DEFAULT.as_str().into(),
                Message::ApplicationBackground,
                |this| &this.application_background,
            ),

            ContextView::ContainerBackground => self.color_picker_context_view(
                Some(fl!("container-background", "desc-detail").into()),
                fl!("container-background", "reset").into(),
                Message::ContainerBackground,
                |this| &this.container_background,
            ),

            ContextView::ControlComponent => self.color_picker_context_view(
                None,
                RESET_TO_DEFAULT.as_str().into(),
                Message::ControlComponent,
                |this| &this.control_component,
            ),

            ContextView::CustomAccent => self.color_picker_context_view(
                None,
                RESET_TO_DEFAULT.as_str().into(),
                Message::CustomAccent,
                |this| &this.custom_accent,
            ),

            ContextView::InterfaceText => self.color_picker_context_view(
                None,
                RESET_TO_DEFAULT.as_str().into(),
                Message::InterfaceText,
                |this| &this.interface_text,
            ),
        };

        Some(view)
    }
}

#[allow(clippy::too_many_lines)]
pub fn mode_and_colors() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("mode-and-colors"))
        .descriptions(vec![
            // 0
            fl!("auto-switch").into(),
            //1
            fl!("accent-color").into(),
            //2
            fl!("app-background").into(),
            //3
            fl!("container-background").into(),
            fl!("container-background", "desc").into(),
            fl!("container-background", "desc-detail").into(),
            fl!("container-background", "reset").into(),
            // 7
            fl!("text-tint").into(),
            fl!("text-tint", "desc").into(),
            // 9
            fl!("control-tint").into(),
            fl!("control-tint", "desc").into(),
            // 11
            fl!("window-hint-accent-toggle").into(),
            fl!("window-hint-accent").into(),
            // 13
            fl!("dark").into(),
            fl!("light").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;
            let palette = &page.theme_builder.palette.as_ref();
            let cur_accent = page
                .theme_builder
                .accent
                .map_or(palette.accent_blue, Srgba::from);
            let mut section = settings::view_section(&section.title)
                .add(
                    container(
                        cosmic::iced::widget::row![
                            cosmic::iced::widget::column![
                                button(
                                    icon(from_name("illustration-appearance-mode-dark").into(),)
                                        .width(Length::Fill)
                                        .height(Length::Fixed(100.0))
                                )
                                .style(button::Style::Image)
                                .padding([8, 0])
                                .selected(page.theme_mode.is_dark)
                                .on_press(Message::DarkMode(true)),
                                text(&*descriptions[13])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_items(cosmic::iced_core::Alignment::Center),
                            cosmic::iced::widget::column![
                                button(
                                    icon(from_name("illustration-appearance-mode-light").into(),)
                                        .width(Length::Fill)
                                        .height(Length::Fixed(100.0))
                                )
                                .style(button::Style::Image)
                                .selected(!page.theme_mode.is_dark)
                                .padding([8, 0])
                                .on_press(Message::DarkMode(false)),
                                text(&*descriptions[14])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_items(cosmic::iced_core::Alignment::Center)
                        ]
                        .spacing(48)
                        .align_items(cosmic::iced_core::Alignment::Center)
                        .width(Length::Fixed(424.0)),
                    )
                    .width(Length::Fill)
                    .align_x(cosmic::iced_core::alignment::Horizontal::Center),
                )
                .add(
                    settings::item::builder(&*descriptions[0])
                        .description(
                            if !page.day_time && page.theme_mode.is_dark {
                                &page.auto_switch_descs[0]
                            } else if page.day_time && !page.theme_mode.is_dark {
                                &page.auto_switch_descs[1]
                            } else if page.day_time && page.theme_mode.is_dark {
                                &page.auto_switch_descs[2]
                            } else {
                                &page.auto_switch_descs[3]
                            }
                            .clone(),
                        )
                        .toggler(page.theme_mode.auto_switch, Message::Autoswitch),
                )
                .add(
                    cosmic::iced::widget::column![
                        text(&*descriptions[1]),
                        scrollable(
                            cosmic::iced::widget::row![
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_blue.into())),
                                    palette.accent_blue.into(),
                                    cur_accent == palette.accent_blue,
                                    48,
                                    48
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_indigo.into())),
                                    palette.accent_indigo.into(),
                                    cur_accent == palette.accent_indigo,
                                    48,
                                    48
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_purple.into())),
                                    palette.accent_purple.into(),
                                    cur_accent == palette.accent_purple,
                                    48,
                                    48
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_pink.into())),
                                    palette.accent_pink.into(),
                                    cur_accent == palette.accent_pink,
                                    48,
                                    48
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_red.into())),
                                    palette.accent_red.into(),
                                    cur_accent == palette.accent_red,
                                    48,
                                    48
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_orange.into())),
                                    palette.accent_orange.into(),
                                    cur_accent == palette.accent_orange,
                                    48,
                                    48
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_yellow.into())),
                                    palette.accent_yellow.into(),
                                    cur_accent == palette.accent_yellow,
                                    48,
                                    48
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_green.into())),
                                    palette.accent_green.into(),
                                    cur_accent == palette.accent_green,
                                    48,
                                    48
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_warm_grey.into())),
                                    palette.accent_warm_grey.into(),
                                    cur_accent == palette.accent_warm_grey,
                                    48,
                                    48
                                ),
                                if let Some(c) = page.custom_accent.get_applied_color() {
                                    container(color_button(
                                        Some(Message::CustomAccent(
                                            ColorPickerUpdate::ToggleColorPicker,
                                        )),
                                        c,
                                        cosmic::iced::Color::from(cur_accent) == c,
                                        48,
                                        48,
                                    ))
                                } else {
                                    container(
                                        page.custom_accent
                                            .picker_button(Message::CustomAccent, None)
                                            .width(Length::Fixed(48.0))
                                            .height(Length::Fixed(48.0)),
                                    )
                                },
                            ]
                            .padding([0, 0, 16, 0])
                            .spacing(16)
                        )
                        .direction(scrollable::Direction::Horizontal(
                            scrollable::Properties::new()
                        ))
                    ]
                    .padding([16, 24, 0, 24])
                    .spacing(8),
                )
                .add(
                    settings::item::builder(&*descriptions[2]).control(
                        page.application_background
                            .picker_button(Message::ApplicationBackground, Some(24))
                            .width(Length::Fixed(48.0))
                            .height(Length::Fixed(24.0)),
                    ),
                )
                .add(
                    settings::item::builder(&*descriptions[3])
                        .description(&*descriptions[4])
                        .control(if page.container_background.get_applied_color().is_some() {
                            Element::from(
                                page.container_background
                                    .picker_button(Message::ContainerBackground, Some(24))
                                    .width(Length::Fixed(48.0))
                                    .height(Length::Fixed(24.0)),
                            )
                        } else {
                            container(
                                button::text(fl!("auto"))
                                    .trailing_icon(from_name("go-next-symbolic"))
                                    .on_press(Message::ContainerBackground(
                                        ColorPickerUpdate::ToggleColorPicker,
                                    )),
                            )
                            .into()
                        }),
                )
                .add(
                    settings::item::builder(&*descriptions[7])
                        .description(&*descriptions[8])
                        .control(
                            page.interface_text
                                .picker_button(Message::InterfaceText, Some(24))
                                .width(Length::Fixed(48.0))
                                .height(Length::Fixed(24.0)),
                        ),
                )
                .add(
                    settings::item::builder(&*descriptions[9])
                        .description(&*descriptions[10])
                        .control(
                            page.control_component
                                .picker_button(Message::ControlComponent, Some(24))
                                .width(Length::Fixed(48.0))
                                .height(Length::Fixed(24.0)),
                        ),
                )
                .add(
                    settings::item::builder(&*descriptions[11])
                        .toggler(page.no_custom_window_hint, Message::UseDefaultWindowHint),
                );
            if !page.no_custom_window_hint {
                section = section.add(
                    settings::item::builder(&*descriptions[12]).control(
                        page.accent_window_hint
                            .picker_button(Message::AccentWindowHint, Some(24))
                            .width(Length::Fixed(48.0))
                            .height(Length::Fixed(24.0)),
                    ),
                );
            }
            section
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
pub fn style() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("style"))
        .descriptions(vec![
            fl!("style", "round").into(),
            fl!("style", "slightly-round").into(),
            fl!("style", "square").into(),
            fl!("frosted").into(),
            fl!("frosted", "desc").into(),
            fl!("enable-export").into(),
            fl!("enable-export", "desc").into(),
            ICON_THEME.as_str().into(),
            ICON_THEME_DESC.as_str().into(),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(
                    container(
                        cosmic::iced::widget::row![
                            cosmic::iced::widget::column![
                                button(
                                    icon(
                                        from_name(if page.theme_mode.is_dark {
                                            "illustration-appearance-dark-style-round"
                                        } else {
                                            "illustration-appearance-light-style-round"
                                        })
                                        .into()
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fixed(100.0))
                                )
                                .selected(matches!(page.roundness, Roundness::Round))
                                .style(button::Style::Image)
                                .padding(8)
                                .on_press(Message::Roundness(Roundness::Round)),
                                text(&*descriptions[0])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_items(cosmic::iced_core::Alignment::Center),
                            cosmic::iced::widget::column![
                                button(
                                    icon(
                                        from_name(if page.theme_mode.is_dark {
                                            "illustration-appearance-dark-style-slightly-round"
                                        } else {
                                            "illustration-appearance-light-style-slightly-round"
                                        })
                                        .into()
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fixed(100.0))
                                )
                                .selected(matches!(page.roundness, Roundness::SlightlyRound))
                                .style(button::Style::Image)
                                .padding(8)
                                .on_press(Message::Roundness(Roundness::SlightlyRound)),
                                text(&*descriptions[1])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_items(cosmic::iced_core::Alignment::Center),
                            cosmic::iced::widget::column![
                                button(
                                    icon(
                                        from_name(if page.theme_mode.is_dark {
                                            "illustration-appearance-dark-style-square"
                                        } else {
                                            "illustration-appearance-light-style-square"
                                        })
                                        .into(),
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fixed(100.0))
                                )
                                .width(Length::FillPortion(1))
                                .selected(matches!(page.roundness, Roundness::Square))
                                .style(button::Style::Image)
                                .padding(8)
                                .on_press(Message::Roundness(Roundness::Square)),
                                text(&*descriptions[2])
                            ]
                            .spacing(8)
                            .align_items(cosmic::iced_core::Alignment::Center)
                            .width(Length::FillPortion(1))
                        ]
                        .spacing(12)
                        .width(Length::Fixed(628.0))
                        .align_items(cosmic::iced_core::Alignment::Center),
                    )
                    .width(Length::Fill)
                    .align_x(cosmic::iced_core::alignment::Horizontal::Center),
                )
                .add(
                    settings::item::builder(&*descriptions[3])
                        .description(&*descriptions[4])
                        .toggler(page.theme_builder.is_frosted, Message::Frosted),
                )
                .add(
                    settings::item::builder(&*descriptions[5])
                        .description(&*descriptions[6])
                        .toggler(page.tk.apply_theme_global, Message::ApplyThemeGlobal),
                )
                .add(
                    settings::item::builder(&*ICON_THEME)
                        .description(&*ICON_THEME_DESC)
                        .control(dropdown(
                            &page.icon_themes,
                            page.icon_theme_active,
                            Message::IconTheme,
                        )),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
pub fn window_management() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("window-management"))
        .descriptions(vec![
            fl!("window-management", "active-hint").into(),
            fl!("window-management", "gaps").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(settings::item::builder(&*descriptions[0]).control(
                    cosmic::widget::spin_button(
                        page.theme_builder.active_hint.to_string(),
                        Message::WindowHintSize,
                    ),
                ))
                .add(settings::item::builder(&*descriptions[1]).control(
                    cosmic::widget::spin_button(
                        page.theme_builder.gaps.1.to_string(),
                        Message::GapSize,
                    ),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
pub fn reset_button() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![fl!("reset-to-default").into()])
        .view::<Page>(|_binder, page, section| {
            let spacing = &page.theme_builder.spacing;
            let descriptions = &section.descriptions;
            if page.can_reset {
                cosmic::iced::widget::row![button(text(&*descriptions[0]))
                    .on_press(Message::Reset)
                    .padding([spacing.space_xxs, spacing.space_xs])]
                .apply(Element::from)
            } else {
                horizontal_space(1).apply(Element::from)
            }
            .map(crate::pages::Message::Appearance)
        })
}
impl page::AutoBind<crate::pages::Message> for Page {}

/// A button for selecting a color or gradient.
pub fn color_button<'a, Message: 'a + Clone>(
    on_press: Option<Message>,
    color: cosmic::iced::Color,
    selected: bool,
    width: u16,
    height: u16,
) -> Element<'a, Message> {
    button(color_image(
        wallpaper::Color::Single([color.r, color.g, color.b]),
        width,
        height,
        None,
    ))
    .padding(0)
    .selected(selected)
    .style(button::Style::Image)
    .on_press_maybe(on_press)
    .width(Length::Fixed(f32::from(width)))
    .height(Length::Fixed(f32::from(height)))
    .into()
}

/// Find all icon themes available on the system.
async fn fetch_icon_themes() -> Message {
    let mut icon_themes = BTreeSet::new();

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

            let manifest = path.join("index.theme");

            if !manifest.exists() {
                continue;
            }

            let Ok(file) = tokio::fs::File::open(&manifest).await else {
                continue;
            };

            buffer.clear();
            let mut name = None;

            let mut line_reader = tokio::io::BufReader::new(file);
            while let Ok(read) = line_reader.read_line(&mut buffer).await {
                if read == 0 {
                    break;
                }

                if let Some(is_hidden) = buffer.strip_prefix("Hidden=") {
                    if is_hidden.trim() == "true" {
                        continue 'icon_dir;
                    }
                } else if name.is_none() {
                    if let Some(value) = buffer.strip_prefix("Name=") {
                        name = Some(value.trim().to_owned());
                    }
                }

                buffer.clear();
            }

            if let Some(name) = name {
                icon_themes.insert(name);
            }
        }
    }

    Message::Entered(icon_themes.into_iter().collect())
}

/// Set the preferred icon theme for GNOME/GTK applications.
async fn set_gnome_icon_theme(theme: String) {
    let _res = tokio::process::Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.interface",
            "icon-theme",
            theme.as_str(),
        ])
        .status()
        .await;
}
