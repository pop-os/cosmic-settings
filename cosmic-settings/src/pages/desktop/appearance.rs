// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
use cosmic::config::CosmicTk;
use cosmic::cosmic_config::{Config, ConfigSet, CosmicConfigEntry};
use cosmic::cosmic_theme::palette::{FromColor, Hsv, Srgb, Srgba};
use cosmic::cosmic_theme::{
    CornerRadii, Density, Theme, ThemeBuilder, ThemeMode, DARK_THEME_BUILDER_ID,
    LIGHT_THEME_BUILDER_ID,
};
use cosmic::iced_core::{alignment, Background, Color, Length};
use cosmic::iced_widget::scrollable;
use cosmic::prelude::CollectionWidget;
use cosmic::widget::icon::{self, from_name, icon};
use cosmic::widget::{
    button, color_picker::ColorPickerUpdate, container, flex_row, horizontal_space, radio, row,
    settings, spin_button, text, ColorPickerModel,
};
use cosmic::Apply;
use cosmic::{command, Command, Element};
use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use cosmic_settings_wallpaper as wallpaper;
use ron::ser::PrettyConfig;
use serde::Serialize;
use slab::Slab;
use slotmap::SlotMap;
use tokio::io::AsyncBufReadExt;

use crate::app;
use crate::widget::color_picker_context_view;

use super::wallpaper::widgets::color_image;

const ICON_PREV_N: usize = 6;
const ICON_PREV_ROW: usize = 3;
const ICON_TRY_SIZES: [u16; 3] = [32, 48, 64];
const ICON_THUMB_SIZE: u16 = 32;
const ICON_NAME_TRUNC: usize = 20;

pub type IconThemes = Vec<IconTheme>;
pub type IconHandles = Vec<[icon::Handle; ICON_PREV_N]>;

crate::cache_dynamic_lazy! {
    static HEX: String = fl!("hex");
    static RGB: String = fl!("rgb");
    static RESET_TO_DEFAULT: String = fl!("reset-to-default");
    static ICON_THEME: String = fl!("icon-theme");
}

#[derive(Clone, Copy, Debug)]
enum ContextView {
    AccentWindowHint,
    ApplicationBackground,
    ContainerBackground,
    ControlComponent,
    CustomAccent,
    Experimental,
    InterfaceText,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IconTheme {
    // COSMIC uses the file name of the folder containing the theme
    id: String,
    // GTK uses the name of the theme as specified in its index file
    name: String,
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
    density: Density,

    icon_theme_active: Option<usize>,
    icon_themes: IconThemes,
    icon_handles: IconHandles,

    theme: Theme,
    theme_mode: ThemeMode,
    theme_mode_config: Option<Config>,
    theme_builder: ThemeBuilder,
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
            context_view: None,
            roundness: theme_builder.corner_radii.into(),
            density: tk.interface_density.into(),
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
            no_custom_window_hint: theme_builder.window_hint.is_none(),
            icon_theme_active: None,
            icon_themes: Vec::new(),
            icon_handles: Vec::new(),
            theme,
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
    Density(Density),
    Entered((IconThemes, IconHandles)),
    ExperimentalContextDrawer,
    ExportError,
    ExportFile(Arc<SelectedFiles>),
    ExportSuccess,
    GapSize(spin_button::Message),
    IconTheme(usize),
    ImportError,
    ImportFile(Arc<SelectedFiles>),
    ImportSuccess(Box<ThemeBuilder>),
    InterfaceText(ColorPickerUpdate),
    Left,
    NewTheme(Theme),
    PaletteAccent(cosmic::iced::Color),
    Reset,
    Roundness(Roundness),
    StartExport,
    StartImport,
    UseDefaultWindowHint(bool),
    WindowHintSize(spin_button::Message),
    Daytime(bool),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Appearance(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Appearance(message)
    }
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
    fn experimental_context_view(&self) -> Element<'_, crate::pages::Message> {
        let active = self.icon_theme_active;
        let theme = cosmic::theme::active();
        let theme = theme.cosmic();
        cosmic::iced::widget::column![
            // Export theme choice
            settings::section().add(
                settings::item::builder(fl!("enable-export"))
                    .description(fl!("enable-export", "desc"))
                    .toggler(self.tk.apply_theme_global, Message::ApplyThemeGlobal)
            ),
            // Icon theme previews
            cosmic::widget::column::with_children(vec![
                text::heading(&*ICON_THEME).into(),
                flex_row(
                    self.icon_themes
                        .iter()
                        .zip(self.icon_handles.iter())
                        .enumerate()
                        .map(|(i, (theme, handles))| {
                            let selected = active.map(|j| i == j).unwrap_or_default();
                            icon_theme_button(&theme.name, handles, i, selected)
                        })
                        .collect(),
                )
                .row_spacing(theme.space_xs())
                .column_spacing(theme.space_xs())
                .into()
            ])
            .spacing(theme.space_xxs())
        ]
        .spacing(theme.space_m())
        .width(Length::Fill)
        .apply(Element::from)
        .map(crate::pages::Message::Appearance)
    }

    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        let mut commands = Vec::new();

        let mut needs_build = false;
        let mut needs_sync = false;

        match message {
            Message::NewTheme(theme) => {
                self.theme = theme;
            }
            Message::DarkMode(enabled) => {
                if let Some(config) = self.theme_mode_config.as_ref() {
                    if let Err(err) = self.theme_mode.set_is_dark(config, enabled) {
                        tracing::error!(?err, "Error setting dark mode");
                    }

                    self.reload_theme_mode();
                }
            }

            Message::Autoswitch(enabled) => {
                self.theme_mode.auto_switch = enabled;
                if let Some(config) = self.theme_mode_config.as_ref() {
                    _ = config.set::<bool>("auto_switch", enabled);
                }
            }

            Message::AccentWindowHint(u) => {
                needs_sync = true;

                let (command, needs_update) = self.update_color_picker(
                    &u,
                    ContextView::AccentWindowHint,
                    fl!("window-hint-accent").into(),
                );

                commands.push(command);
                commands.push(self.accent_window_hint.update::<app::Message>(u));

                if needs_update {
                    let Some(config) = self.theme_builder_config.as_ref() else {
                        return cosmic::command::batch(commands);
                    };

                    let color = self.accent_window_hint.get_applied_color().map(Srgb::from);

                    needs_build = self
                        .theme_builder
                        .set_window_hint(config, color.clone())
                        .unwrap_or_default();
                }
            }

            Message::IconTheme(id) => {
                if let Some(theme) = self.icon_themes.get(id).cloned() {
                    self.icon_theme_active = Some(id);

                    if let Some(ref config) = self.tk_config {
                        let _ = self.tk.write_entry(config);
                        _ = self.tk.set_icon_theme(config, theme.id.clone());
                    }

                    tokio::spawn(set_gnome_icon_theme(theme.name));
                }
            }

            Message::WindowHintSize(msg) => {
                needs_sync = true;

                let Some(config) = self.theme_builder_config.as_ref() else {
                    return Command::none();
                };

                let active_hint = match msg {
                    spin_button::Message::Increment => {
                        self.theme_builder.active_hint.saturating_add(1)
                    }
                    spin_button::Message::Decrement => {
                        self.theme_builder.active_hint.saturating_sub(1)
                    }
                };

                if self
                    .theme_builder
                    .set_active_hint(config, active_hint)
                    .unwrap_or_default()
                {
                    self.theme_config_write("active_hint", active_hint);
                }
            }

            Message::GapSize(msg) => {
                needs_sync = true;

                let Some(config) = self.theme_builder_config.as_ref() else {
                    return Command::none();
                };

                let mut gaps = self.theme_builder.gaps.clone();

                gaps.1 = match msg {
                    spin_button::Message::Increment => self.theme_builder.gaps.1.saturating_add(1),
                    spin_button::Message::Decrement => self.theme_builder.gaps.1.saturating_sub(1),
                };

                if self
                    .theme_builder
                    .set_gaps(config, gaps)
                    .unwrap_or_default()
                {
                    self.theme_config_write("gaps", gaps);
                }
            }

            Message::ApplicationBackground(u) => {
                let (command, needs_update) = self.update_color_picker(
                    &u,
                    ContextView::ApplicationBackground,
                    fl!("app-background").into(),
                );

                commands.push(command);
                commands.push(self.application_background.update::<app::Message>(u));

                if needs_update {
                    let Some(config) = self.theme_builder_config.as_ref() else {
                        return cosmic::command::batch(commands);
                    };

                    needs_build = self
                        .theme_builder
                        .set_bg_color(
                            config,
                            self.application_background
                                .get_applied_color()
                                .map(Srgba::from),
                        )
                        .unwrap_or_default();
                }
            }

            Message::ContainerBackground(u) => {
                let (command, needs_update) = self.update_color_picker(
                    &u,
                    ContextView::ContainerBackground,
                    fl!("container-background").into(),
                );

                commands.push(command);
                commands.push(self.container_background.update::<app::Message>(u));

                if needs_update {
                    let Some(config) = self.theme_builder_config.as_ref() else {
                        return cosmic::command::batch(commands);
                    };

                    needs_build = self
                        .theme_builder
                        .set_primary_container_bg(
                            config,
                            self.container_background
                                .get_applied_color()
                                .map(Srgba::from),
                        )
                        .unwrap_or_default();
                }
            }

            Message::CustomAccent(u) => {
                let (command, needs_update) = self.update_color_picker(
                    &u,
                    ContextView::CustomAccent,
                    fl!("accent-color").into(),
                );

                commands.push(command);
                commands.push(self.custom_accent.update::<app::Message>(u));

                if needs_update {
                    let Some(config) = self.theme_builder_config.as_ref() else {
                        return cosmic::command::batch(commands);
                    };

                    needs_build = self
                        .theme_builder
                        .set_accent(
                            config,
                            self.custom_accent.get_applied_color().map(Srgb::from),
                        )
                        .unwrap_or_default();
                }
            }

            Message::InterfaceText(u) => {
                let (command, needs_update) = self.update_color_picker(
                    &u,
                    ContextView::InterfaceText,
                    fl!("text-tint").into(),
                );

                commands.push(command);
                commands.push(self.interface_text.update::<app::Message>(u));

                if needs_update {
                    let Some(config) = self.theme_builder_config.as_ref() else {
                        return cosmic::command::batch(commands);
                    };

                    needs_build = self
                        .theme_builder
                        .set_text_tint(
                            config,
                            self.interface_text.get_applied_color().map(Srgb::from),
                        )
                        .unwrap_or_default();
                }
            }

            Message::ControlComponent(u) => {
                let (command, needs_update) = self.update_color_picker(
                    &u,
                    ContextView::ControlComponent,
                    fl!("control-tint").into(),
                );

                commands.push(command);
                commands.push(self.control_component.update::<app::Message>(u));

                if needs_update {
                    let Some(config) = self.theme_builder_config.as_ref() else {
                        return cosmic::command::batch(commands);
                    };

                    needs_build = self
                        .theme_builder
                        .set_neutral_tint(
                            config,
                            self.control_component.get_applied_color().map(Srgb::from),
                        )
                        .unwrap_or_default();
                }
            }

            Message::Roundness(r) => {
                needs_sync = true;
                self.roundness = r;

                let Some(config) = self.theme_builder_config.as_ref() else {
                    return Command::none();
                };

                let radii = self.roundness.into();

                if self
                    .theme_builder
                    .set_corner_radii(config, radii)
                    .unwrap_or_default()
                {
                    self.theme_config_write("corner_radii", radii);
                }

                tokio::task::spawn(async move {
                    Self::update_panel_radii(r);
                });
            }

            Message::Density(d) => {
                needs_sync = true;
                self.density = d;

                if let Some(config) = self.tk_config.as_mut() {
                    let _density = self.tk.set_interface_density(config, d);
                    let _header = self.tk.set_header_size(config, d);
                }

                let Some(config) = self.theme_builder_config.as_ref() else {
                    return Command::none();
                };

                let spacing = self.density.into();

                if self
                    .theme_builder
                    .set_spacing(config, spacing)
                    .unwrap_or_default()
                {
                    self.theme_config_write("spacing", spacing);
                }

                tokio::task::spawn(async move {
                    Self::update_panel_spacing(d);
                });
            }

            Message::Entered((icon_themes, icon_handles)) => {
                *self = Self::default();

                // Set the icon themes, and define the active icon theme.
                self.icon_themes = icon_themes;
                self.icon_theme_active = self
                    .icon_themes
                    .iter()
                    .position(|theme| &theme.id == &self.tk.icon_theme);
                self.icon_handles = icon_handles;
            }

            Message::Left => {
                commands.push(cosmic::command::message(app::Message::SetTheme(
                    cosmic::theme::system_preference(),
                )));
            }

            Message::PaletteAccent(c) => {
                let Some(config) = self.theme_builder_config.as_ref() else {
                    return Command::none();
                };

                needs_build = self
                    .theme_builder
                    .set_accent(config, Some(c.into()))
                    .unwrap_or_default();
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

                let r = self.roundness;
                tokio::task::spawn(async move {
                    Self::update_panel_radii(r);
                });

                self.reload_theme_mode();
            }

            Message::StartImport => {
                commands.push(cosmic::command::future(async move {
                    let res = SelectedFiles::open_file()
                        .modal(true)
                        .filter(FileFilter::glob(FileFilter::new("ron"), "*.ron"))
                        .send()
                        .await
                        .and_then(|request| request.response());

                    if let Ok(f) = res {
                        Message::ImportFile(Arc::new(f))
                    } else {
                        // TODO Error toast?
                        tracing::error!("failed to select a file for importing a custom theme.");
                        Message::ImportError
                    }
                }));
            }

            Message::StartExport => {
                let is_dark = self.theme_mode.is_dark;
                let name = format!("{}.ron", if is_dark { fl!("dark") } else { fl!("light") });

                commands.push(cosmic::command::future(async move {
                    let res = SelectedFiles::save_file()
                        .modal(true)
                        .current_name(Some(name.as_str()))
                        .send()
                        .await
                        .and_then(|request| request.response());

                    if let Ok(f) = res {
                        Message::ExportFile(Arc::new(f))
                    } else {
                        // TODO Error toast?
                        tracing::error!("failed to select a file for importing a custom theme.");
                        Message::ExportError
                    }
                }));
            }

            Message::ImportFile(f) => {
                let path_res = f
                    .uris()
                    .first()
                    .filter(|f| f.scheme() == "file")
                    .and_then(|f| f.to_file_path().ok());

                let Some(path) = path_res else {
                    return Command::none();
                };

                commands.push(cosmic::command::future(async move {
                    let res = tokio::fs::read_to_string(path).await;
                    if let Some(b) = res.ok().and_then(|s| ron::de::from_str(&s).ok()) {
                        Message::ImportSuccess(Box::new(b))
                    } else {
                        // TODO Error toast?
                        tracing::error!("failed to import a file for a custom theme.");
                        Message::ImportError
                    }
                }));
            }

            Message::ExportFile(f) => {
                let path_res = f
                    .uris()
                    .first()
                    .filter(|f| f.scheme() == "file")
                    .and_then(|f| f.to_file_path().ok());

                let Some(path) = path_res else {
                    return Command::none();
                };

                let theme_builder = self.theme_builder.clone();

                commands.push(cosmic::command::future(async move {
                    let Ok(builder) =
                        ron::ser::to_string_pretty(&theme_builder, PrettyConfig::default())
                    else {
                        return crate::app::Message::None;
                    };

                    match tokio::fs::write(path, builder).await {
                        Ok(_) => Message::ExportSuccess,
                        Err(_why) => {
                            // TODO Error toast?
                            tracing::error!(
                                "failed to select a file for importing a custom theme."
                            );
                            Message::ExportError
                        }
                    }
                    .into()
                }));
            }

            // TODO: error message toast?
            Message::ExportError | Message::ImportError => return Command::none(),

            Message::ExportSuccess => {
                tracing::trace!("Export successful");
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
            }

            Message::UseDefaultWindowHint(v) => {
                self.no_custom_window_hint = v;

                let Some(config) = self.theme_builder_config.as_ref() else {
                    return Command::none();
                };

                needs_build = self
                    .theme_builder
                    .set_window_hint(
                        config,
                        if v {
                            None
                        } else {
                            let theme = if self.theme_mode.is_dark {
                                Theme::dark_default()
                            } else {
                                Theme::light_default()
                            };

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

                            self.accent_window_hint.get_applied_color().map(Srgb::from)
                        },
                    )
                    .unwrap_or_default();
            }

            Message::ApplyThemeGlobal(enabled) => {
                if let Some(tk_config) = self.tk_config.as_ref() {
                    _ = self.tk.set_apply_theme_global(tk_config, enabled);
                } else {
                    tracing::error!("Failed to apply theme to GNOME config because the CosmicTK config does not exist.");
                }

                return Command::none();
            }

            Message::ExperimentalContextDrawer => {
                self.context_view = Some(ContextView::Experimental);
                return cosmic::command::message(crate::app::Message::OpenContextDrawer("".into()));
            }

            Message::Daytime(day_time) => {
                self.day_time = day_time;
                return Command::none();
            }
        }

        // If the theme builder changed, write a new theme to disk on a background thread.
        if needs_build {
            let theme_builder = self.theme_builder.clone();
            let is_dark = self.theme_mode.is_dark;
            let current_theme = self.theme.clone();

            commands.push(cosmic::command::future(async move {
                let config = if is_dark {
                    Theme::dark_config()
                } else {
                    Theme::light_config()
                };

                if let Ok(config) = config {
                    let new_theme = theme_builder.build();

                    macro_rules! theme_transaction {
                        ($config:ident, $current_theme:ident, $new_theme:ident, { $($name:ident;)+ }) => {
                            let tx = $config.transaction();

                            $(
                                if $current_theme.$name != $new_theme.$name {
                                    _ = tx.set(stringify!($name), $new_theme.$name.clone());
                                }
                            )+

                            _ = tx.commit();
                        }
                    }

                    theme_transaction!(config, current_theme, new_theme, {
                        accent;
                        accent_button;
                        background;
                        button;
                        destructive;
                        destructive_button;
                        link_button;
                        icon_button;
                        palette;
                        primary;
                        secondary;
                        shade;
                        success;
                        text_button;
                        warning;
                        warning_button;
                        window_hint;
                    });

                    Message::NewTheme(new_theme).into()
                } else {
                    tracing::error!("Failed to get the theme config.");
                    crate::app::Message::None
                }
            }));
        }

        self.can_reset = if self.theme_mode.is_dark {
            self.theme_builder != ThemeBuilder::dark()
        } else {
            self.theme_builder != ThemeBuilder::light()
        };

        if needs_sync {
            let theme_builder = self.theme_builder.clone();
            let is_dark = self.theme_mode.is_dark;

            tokio::task::spawn(async move {
                if let Err(why) = Self::sync_theme_changes_between_modes(theme_builder, is_dark) {
                    tracing::error!(?why, "Error syncing theme changes.");
                }
            });
        }

        cosmic::command::batch(commands)
    }

    fn reload_theme_mode(&mut self) {
        let icon_themes = std::mem::take(&mut self.icon_themes);
        let icon_handles = std::mem::take(&mut self.icon_handles);
        let icon_theme_active = self.icon_theme_active.take();
        let day_time = self.day_time;

        *self = Self::from((self.theme_mode_config.clone(), self.theme_mode));
        self.day_time = day_time;
        self.icon_themes = icon_themes;
        self.icon_handles = icon_handles;
        self.icon_theme_active = icon_theme_active;
    }

    fn update_color_picker(
        &mut self,
        message: &ColorPickerUpdate,
        context_view: ContextView,
        context_title: Cow<'static, str>,
    ) -> (Command<app::Message>, bool) {
        let mut needs_update = false;

        let command = match message {
            ColorPickerUpdate::AppliedColor | ColorPickerUpdate::Reset => {
                needs_update = true;
                cosmic::command::message(crate::app::Message::CloseContextDrawer)
            }

            ColorPickerUpdate::ActionFinished => {
                needs_update = true;
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
        };

        (command, needs_update)
    }

    /// Syncs changes for dark and light theme.
    /// Roundness and window management settings should be consistent between dark / light mode.
    fn sync_theme_changes_between_modes(
        current_theme_builder: ThemeBuilder,
        is_dark: bool,
    ) -> Result<(), cosmic::cosmic_config::Error> {
        let (other_builder_config, other_theme_config) = if is_dark {
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

        if theme_builder.active_hint != current_theme_builder.active_hint {
            if let Err(err) = theme_builder
                .set_active_hint(&other_builder_config, current_theme_builder.active_hint)
            {
                tracing::error!(?err, "Error setting active hint");
            }
            if let Err(err) =
                theme.set_active_hint(&other_theme_config, current_theme_builder.active_hint)
            {
                tracing::error!(?err, "Error setting active hint");
            }
        }

        if theme_builder.gaps != current_theme_builder.gaps {
            if let Err(err) =
                theme_builder.set_gaps(&other_builder_config, current_theme_builder.gaps)
            {
                tracing::error!(?err, "Error setting gaps");
            }
            if let Err(err) = theme.set_gaps(&other_theme_config, current_theme_builder.gaps) {
                tracing::error!(?err, "Error setting gaps");
            }
        }

        if theme_builder.corner_radii != current_theme_builder.corner_radii {
            if let Err(err) = theme_builder
                .set_corner_radii(&other_builder_config, current_theme_builder.corner_radii)
            {
                tracing::error!(?err, "Error setting corner radii");
            }

            if let Err(err) =
                theme.set_corner_radii(&other_theme_config, current_theme_builder.corner_radii)
            {
                tracing::error!(?err, "Error setting corner radii");
            }
        }

        Ok(())
    }

    fn theme_config_write<T: Serialize>(&self, name: &str, value: T) {
        let config_res = if self.theme_mode.is_dark {
            Theme::dark_config()
        } else {
            Theme::light_config()
        };

        if let Ok(config) = config_res {
            _ = config.set(name, value);
        }
    }

    // TODO: cache panel and dock configs so that they needn't be re-read
    fn update_panel_radii(roundness: Roundness) {
        let panel_config_helper = CosmicPanelConfig::cosmic_config("Panel").ok();
        let dock_config_helper = CosmicPanelConfig::cosmic_config("Dock").ok();

        let mut panel_config = panel_config_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "Panel").then_some(panel_config)
        });

        let mut dock_config = dock_config_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "Dock").then_some(panel_config)
        });

        if let Some(panel_config_helper) = panel_config_helper.as_ref() {
            if let Some(panel_config) = panel_config.as_mut() {
                let radii = if panel_config.anchor_gap || !panel_config.expand_to_edges {
                    let cornder_radii: CornerRadii = roundness.into();
                    cornder_radii.radius_xl[0] as u32
                } else {
                    0
                };

                if let Err(why) = panel_config.set_border_radius(panel_config_helper, radii) {
                    tracing::error!(?why, "Error updating panel corner radii");
                }
            }
        };

        if let Some(dock_config_helper) = dock_config_helper.as_ref() {
            if let Some(dock_config) = dock_config.as_mut() {
                let radii = if dock_config.anchor_gap || !dock_config.expand_to_edges {
                    let cornder_radii: CornerRadii = roundness.into();
                    cornder_radii.radius_xl[0] as u32
                } else {
                    0
                };

                if let Err(why) = dock_config.set_border_radius(dock_config_helper, radii) {
                    tracing::error!(?why, "Error updating dock corner radii");
                }
            }
        };
    }

    fn update_panel_spacing(density: Density) {
        let panel_config_helper = CosmicPanelConfig::cosmic_config("Panel").ok();
        let dock_config_helper = CosmicPanelConfig::cosmic_config("Dock").ok();
        let mut panel_config = panel_config_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "Panel").then_some(panel_config)
        });
        let mut dock_config = dock_config_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "Dock").then_some(panel_config)
        });

        if let Some(panel_config_helper) = panel_config_helper.as_ref() {
            if let Some(panel_config) = panel_config.as_mut() {
                let spacing = match density {
                    Density::Compact => 0,
                    _ => 4,
                };
                let update = panel_config.set_spacing(panel_config_helper, spacing);
                if let Err(err) = update {
                    tracing::error!(?err, "Error updating panel spacing");
                }
            }
        };

        if let Some(dock_config_helper) = dock_config_helper.as_ref() {
            if let Some(dock_config) = dock_config.as_mut() {
                let spacing = match density {
                    Density::Compact => 0,
                    _ => 4,
                };
                let update = dock_config.set_spacing(dock_config_helper, spacing);
                if let Err(err) = update {
                    tracing::error!(?err, "Error updating dock spacing");
                }
            }
        };
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
            sections.insert(interface_density()),
            sections.insert(window_management()),
            sections.insert(experimental()),
            sections.insert(reset_button()),
        ])
    }

    fn header_view(&self) -> Option<Element<'_, crate::pages::Message>> {
        let content = row::with_capacity(2)
            .spacing(self.theme_builder.spacing.space_xxs)
            .push(button::standard(fl!("import")).on_press(Message::StartImport))
            .push(button::standard(fl!("export")).on_press(Message::StartExport))
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

    fn on_enter(
        &mut self,
        _: page::Entity,
        _sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> Command<crate::pages::Message> {
        command::future(fetch_icon_themes()).map(crate::pages::Message::Appearance)
    }

    fn on_leave(&mut self) -> Command<crate::pages::Message> {
        command::message(crate::pages::Message::Appearance(Message::Left))
    }

    fn context_drawer(&self) -> Option<Element<'_, crate::pages::Message>> {
        let view = match self.context_view? {
            ContextView::AccentWindowHint => color_picker_context_view(
                None,
                RESET_TO_DEFAULT.as_str().into(),
                Message::AccentWindowHint,
                &self.accent_window_hint,
            )
            .map(crate::pages::Message::Appearance),

            ContextView::ApplicationBackground => color_picker_context_view(
                None,
                RESET_TO_DEFAULT.as_str().into(),
                Message::ApplicationBackground,
                &self.application_background,
            )
            .map(crate::pages::Message::Appearance),

            ContextView::ContainerBackground => color_picker_context_view(
                Some(fl!("container-background", "desc-detail").into()),
                fl!("container-background", "reset").into(),
                Message::ContainerBackground,
                &self.container_background,
            )
            .map(crate::pages::Message::Appearance),

            ContextView::ControlComponent => color_picker_context_view(
                None,
                RESET_TO_DEFAULT.as_str().into(),
                Message::ControlComponent,
                &self.control_component,
            )
            .map(crate::pages::Message::Appearance),

            ContextView::CustomAccent => color_picker_context_view(
                None,
                RESET_TO_DEFAULT.as_str().into(),
                Message::CustomAccent,
                &self.custom_accent,
            )
            .map(crate::pages::Message::Appearance),

            ContextView::Experimental => self.experimental_context_view(),

            ContextView::InterfaceText => color_picker_context_view(
                None,
                RESET_TO_DEFAULT.as_str().into(),
                Message::InterfaceText,
                &self.interface_text,
            )
            .map(crate::pages::Message::Appearance),
        };

        Some(view)
    }
}

#[allow(clippy::too_many_lines)]
pub fn mode_and_colors() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        auto_txt = fl!("auto");
        auto_switch = fl!("auto-switch");
        accent_color = fl!("accent-color");
        app_bg = fl!("app-background");
        container_bg = fl!("container-background");
        container_bg_desc = fl!("container-background", "desc");
        text_tint = fl!("text-tint");
        text_tint_desc = fl!("text-tint", "desc");
        control_tint = fl!("control-tint");
        control_tint_desc = fl!("control-tint", "desc");
        window_hint_toggle = fl!("window-hint-accent-toggle");
        window_hint = fl!("window-hint-accent");
        dark = fl!("dark");
        light = fl!("light");
    });

    let dark_mode_illustration = from_name("illustration-appearance-mode-dark").handle();
    let light_mode_illustration = from_name("illustration-appearance-mode-light").handle();
    let go_next_icon = from_name("go-next-symbolic").handle();

    Section::default()
        .title(fl!("mode-and-colors"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let palette = &page.theme_builder.palette.as_ref();
            let cur_accent = page
                .theme_builder
                .accent
                .map_or(palette.accent_blue, Srgba::from);
            let mut section = settings::section()
                .title(&section.title)
                .add(
                    container(
                        cosmic::iced::widget::row![
                            cosmic::iced::widget::column![
                                button::custom(
                                    icon(dark_mode_illustration.clone())
                                        .width(Length::Fill)
                                        .height(Length::Fixed(100.0))
                                )
                                .style(button::Style::Image)
                                .padding([8, 0])
                                .selected(page.theme_mode.is_dark)
                                .on_press(Message::DarkMode(true)),
                                text::body(&descriptions[dark])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_items(cosmic::iced_core::Alignment::Center),
                            cosmic::iced::widget::column![
                                button::custom(
                                    icon(light_mode_illustration.clone(),)
                                        .width(Length::Fill)
                                        .height(Length::Fixed(100.0))
                                )
                                .style(button::Style::Image)
                                .selected(!page.theme_mode.is_dark)
                                .padding([8, 0])
                                .on_press(Message::DarkMode(false)),
                                text::body(&descriptions[light])
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
                    settings::item::builder(&descriptions[auto_switch])
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
                        text::body(&descriptions[accent_color]),
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
                    settings::item::builder(&descriptions[app_bg]).control(
                        page.application_background
                            .picker_button(Message::ApplicationBackground, Some(24))
                            .width(Length::Fixed(48.0))
                            .height(Length::Fixed(24.0)),
                    ),
                )
                .add(
                    settings::item::builder(&descriptions[container_bg])
                        .description(&descriptions[container_bg_desc])
                        .control(if page.container_background.get_applied_color().is_some() {
                            Element::from(
                                page.container_background
                                    .picker_button(Message::ContainerBackground, Some(24))
                                    .width(Length::Fixed(48.0))
                                    .height(Length::Fixed(24.0)),
                            )
                        } else {
                            container(
                                button::text(&descriptions[auto_txt])
                                    .trailing_icon(go_next_icon.clone())
                                    .on_press(Message::ContainerBackground(
                                        ColorPickerUpdate::ToggleColorPicker,
                                    )),
                            )
                            .into()
                        }),
                )
                .add(
                    settings::item::builder(&descriptions[text_tint])
                        .description(&descriptions[text_tint_desc])
                        .control(
                            page.interface_text
                                .picker_button(Message::InterfaceText, Some(24))
                                .width(Length::Fixed(48.0))
                                .height(Length::Fixed(24.0)),
                        ),
                )
                .add(
                    settings::item::builder(&descriptions[control_tint])
                        .description(&descriptions[control_tint_desc])
                        .control(
                            page.control_component
                                .picker_button(Message::ControlComponent, Some(24))
                                .width(Length::Fixed(48.0))
                                .height(Length::Fixed(24.0)),
                        ),
                )
                .add(
                    settings::item::builder(&descriptions[window_hint_toggle])
                        .toggler(page.no_custom_window_hint, Message::UseDefaultWindowHint),
                );
            if !page.no_custom_window_hint {
                section = section.add(
                    settings::item::builder(&descriptions[window_hint]).control(
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
    let mut descriptions = Slab::new();

    let round = descriptions.insert(fl!("style", "round"));
    let slightly_round = descriptions.insert(fl!("style", "slightly-round"));
    let square = descriptions.insert(fl!("style", "square"));

    let dark_round_style = from_name("illustration-appearance-dark-style-round").handle();
    let light_round_style = from_name("illustration-appearance-light-style-round").handle();

    let dark_slightly_round_style =
        from_name("illustration-appearance-dark-style-slightly-round").handle();
    let light_slightly_round_style =
        from_name("illustration-appearance-light-style-slightly-round").handle();

    let dark_square_style = from_name("illustration-appearance-dark-style-square").handle();
    let light_square_style = from_name("illustration-appearance-light-style-square").handle();

    Section::default()
        .title(fl!("style"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(
                    container(
                        cosmic::iced::widget::row![
                            cosmic::iced::widget::column![
                                button::custom(
                                    icon(
                                        if page.theme_mode.is_dark {
                                            &dark_round_style
                                        } else {
                                            &light_round_style
                                        }
                                        .clone()
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fixed(100.0))
                                )
                                .selected(matches!(page.roundness, Roundness::Round))
                                .style(button::Style::Image)
                                .padding(8)
                                .on_press(Message::Roundness(Roundness::Round)),
                                text::body(&descriptions[round])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_items(cosmic::iced_core::Alignment::Center),
                            cosmic::iced::widget::column![
                                button::custom(
                                    icon(
                                        if page.theme_mode.is_dark {
                                            &dark_slightly_round_style
                                        } else {
                                            &light_slightly_round_style
                                        }
                                        .clone()
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fixed(100.0))
                                )
                                .selected(matches!(page.roundness, Roundness::SlightlyRound))
                                .style(button::Style::Image)
                                .padding(8)
                                .on_press(Message::Roundness(Roundness::SlightlyRound)),
                                text::body(&descriptions[slightly_round])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_items(cosmic::iced_core::Alignment::Center),
                            cosmic::iced::widget::column![
                                button::custom(
                                    icon(
                                        if page.theme_mode.is_dark {
                                            &dark_square_style
                                        } else {
                                            &light_square_style
                                        }
                                        .clone()
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fixed(100.0))
                                )
                                .width(Length::FillPortion(1))
                                .selected(matches!(page.roundness, Roundness::Square))
                                .style(button::Style::Image)
                                .padding(8)
                                .on_press(Message::Roundness(Roundness::Square)),
                                text::body(&descriptions[square])
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
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

pub fn interface_density() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let comfortable = descriptions.insert(fl!("interface-density", "comfortable"));
    let compact = descriptions.insert(fl!("interface-density", "compact"));
    let spacious = descriptions.insert(fl!("interface-density", "spacious"));

    Section::default()
        .title(fl!("interface-density"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(settings::item_row(vec![radio(
                    text::body(&descriptions[compact]),
                    Density::Compact,
                    Some(page.density),
                    Message::Density,
                )
                .width(Length::Fill)
                .into()]))
                .add(settings::item_row(vec![radio(
                    text::body(&descriptions[comfortable]),
                    Density::Standard,
                    Some(page.density),
                    Message::Density,
                )
                .width(Length::Fill)
                .into()]))
                .add(settings::item_row(vec![radio(
                    text::body(&descriptions[spacious]),
                    Density::Spacious,
                    Some(page.density),
                    Message::Density,
                )
                .width(Length::Fill)
                .into()]))
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
pub fn window_management() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let active_hint = descriptions.insert(fl!("window-management-appearance", "active-hint"));
    let gaps = descriptions.insert(fl!("window-management-appearance", "gaps"));

    Section::default()
        .title(fl!("window-management-appearance"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(settings::item::builder(&descriptions[active_hint]).control(
                    cosmic::widget::spin_button(
                        page.theme_builder.active_hint.to_string(),
                        Message::WindowHintSize,
                    ),
                ))
                .add(settings::item::builder(&descriptions[gaps]).control(
                    cosmic::widget::spin_button(
                        page.theme_builder.gaps.1.to_string(),
                        Message::GapSize,
                    ),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

pub fn experimental() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let experimental_label = descriptions.insert(fl!("experimental-settings"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            let descriptions = &section.descriptions;

            let control = row::with_children(vec![
                horizontal_space(Length::Fill).into(),
                icon::from_name("go-next-symbolic").size(16).into(),
            ]);

            settings::section()
                .add(
                    settings::item::builder(&descriptions[experimental_label])
                        .control(control)
                        .apply(container)
                        .style(cosmic::theme::Container::List)
                        .apply(button::custom)
                        .style(cosmic::theme::Button::Transparent)
                        .on_press(Message::ExperimentalContextDrawer),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
pub fn reset_button() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let reset_to_default = descriptions.insert(fl!("reset-to-default"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            if page.can_reset {
                button::standard(&descriptions[reset_to_default])
                    .on_press(Message::Reset)
                    .into()
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
    button::custom(color_image(
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
                } else if name.is_none() {
                    if let Some(value) = buffer.strip_prefix("Name=") {
                        name = Some(value.trim().to_owned());
                    }
                }

                if valid_dirs.is_empty() {
                    if let Some(value) = buffer.strip_prefix("Inherits=") {
                        valid_dirs.extend(value.trim().split(',').map(|fallback| {
                            if let Some(path) = theme_paths.get(fallback) {
                                path.iter()
                                    .last()
                                    .and_then(|os| os.to_str().map(ToOwned::to_owned))
                                    .unwrap_or_else(|| fallback.to_owned())
                            } else {
                                fallback.to_owned()
                            }
                        }));
                    }
                }

                buffer.clear();
            }

            if let Some(name) = name {
                // Name of the directory theme was found in (e.g. Pop for Pop)
                valid_dirs.push(
                    path.iter()
                        .last()
                        .and_then(|os| os.to_str().map(ToOwned::to_owned))
                        .unwrap_or_else(|| name.clone()),
                );
                theme_paths.entry(name.clone()).or_insert(path);

                let theme = id.clone();
                // `icon::from_name` may perform blocking I/O
                if let Ok(handles) =
                    tokio::task::spawn_blocking(|| preview_handles(theme, valid_dirs)).await
                {
                    icon_themes.insert(IconTheme { id, name }, handles);
                }
            }
        }
    }

    Message::Entered(icon_themes.into_iter().unzip())
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
        .zip(std::iter::repeat(icon_name).take(ICON_TRY_SIZES.len()))
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

                    if let Some(dir_name) =
                        theme_dir.iter().last().and_then(std::ffi::OsStr::to_str)
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

/// Button with a preview of the icon theme.
fn icon_theme_button(
    name: &str,
    handles: &[icon::Handle],
    id: usize,
    selected: bool,
) -> Element<'static, Message> {
    let theme = cosmic::theme::active();
    let theme = theme.cosmic();
    let background = Background::Color(theme.palette.neutral_4.into());

    cosmic::widget::column()
        .push(
            cosmic::widget::button::custom_image_button(
                cosmic::widget::column::with_children(vec![
                    cosmic::widget::row()
                        .extend(
                            handles
                                .iter()
                                .take(ICON_PREV_ROW)
                                .cloned()
                                // TODO: Maybe allow choosable sizes/zooming
                                .map(|handle| handle.icon().size(ICON_THUMB_SIZE)),
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
                                .map(|handle| handle.icon().size(ICON_THUMB_SIZE)),
                        )
                        .spacing(theme.space_xxxs())
                        .into(),
                ])
                .spacing(theme.space_xxxs()),
                None,
            )
            .on_press(Message::IconTheme(id))
            .selected(selected)
            .padding([theme.space_xs(), theme.space_xs() + 1])
            // Image button's style mostly works, but it needs a background to fit the design
            .style(button::Style::Custom {
                active: Box::new(move |focused, theme| {
                    let mut appearance = <cosmic::theme::Theme as button::StyleSheet>::active(
                        theme,
                        focused,
                        selected,
                        &cosmic::theme::Button::Image,
                    );
                    appearance.background = Some(background);
                    appearance
                }),
                disabled: Box::new(move |theme| {
                    let mut appearance = <cosmic::theme::Theme as button::StyleSheet>::disabled(
                        theme,
                        &cosmic::theme::Button::Image,
                    );
                    appearance.background = Some(background);
                    appearance
                }),
                hovered: Box::new(move |focused, theme| {
                    let mut appearance = <cosmic::theme::Theme as button::StyleSheet>::hovered(
                        theme,
                        focused,
                        selected,
                        &cosmic::theme::Button::Image,
                    );
                    appearance.background = Some(background);
                    appearance
                }),
                pressed: Box::new(move |focused, theme| {
                    let mut appearance = <cosmic::theme::Theme as button::StyleSheet>::pressed(
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
