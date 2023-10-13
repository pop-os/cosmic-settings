// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;

use apply::Apply;
use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
use cosmic::cosmic_config::{Config, ConfigSet, CosmicConfigEntry};
use cosmic::cosmic_theme::palette::{self, FromColor, Hsv, Srgb, Srgba};
use cosmic::cosmic_theme::{CornerRadii, Theme, ThemeBuilder, ThemeMode};
use cosmic::iced::wayland::actions::window::SctkWindowSettings;
use cosmic::iced::widget::{column, row};
use cosmic::iced::window;
use cosmic::iced_core::{layout, Color, Length};
use cosmic::iced_sctk::commands::window::{close_window, get_window};
use cosmic::iced_widget::scrollable;
use cosmic::widget::icon::{from_name, icon};
use cosmic::widget::{
    button, color_picker::ColorPickerUpdate, container, header_bar, horizontal_space, settings,
    spin_button, text, ColorPickerModel,
};
use cosmic::{Command, Element};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use ron::ser::PrettyConfig;
use slotmap::SlotMap;
use tracing::warn;

use crate::app;
pub const COLOR_PICKER_DIALOG_ID: window::Id = window::Id(1003);

enum NamedColorPicker {
    CustomAccent,
    ApplicationBackground,
    InterfaceText,
    ControlComponent,
    AccentWindowHint,
}
// TODO integrate with settings backend
pub struct Page {
    can_reset: bool,
    custom_accent: ColorPickerModel,
    accent_window_hint: ColorPickerModel,
    application_background: ColorPickerModel,
    container_background: ColorPickerModel,
    interface_text: ColorPickerModel,
    control_component: ColorPickerModel,
    active_dialog: Option<NamedColorPicker>,
    roundness: Roundness,
    no_custom_window_hint: bool,

    theme_mode: ThemeMode,
    theme_builder: ThemeBuilder,

    // Configs
    theme_mode_config: Option<Config>,
    theme_builder_config: Option<Config>,
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

impl From<(Option<Config>, ThemeMode, Option<Config>, ThemeBuilder)> for Page {
    fn from(
        (theme_mode_config, theme_mode, theme_builder_config, theme_builder): (
            Option<Config>,
            ThemeMode,
            Option<Config>,
            ThemeBuilder,
        ),
    ) -> Self {
        let theme: Theme<palette::Srgba> = if theme_mode.is_dark {
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
            roundness: theme_builder.corner_radii.into(),
            custom_accent: ColorPickerModel::new(
                fl!("hex"),
                fl!("rgb"),
                None,
                custom_accent.map(Color::from),
            ),
            application_background: ColorPickerModel::new(
                fl!("hex"),
                fl!("rgb"),
                Some(theme.background.base.into()),
                theme_builder.bg_color.map(Color::from),
            ),
            container_background: ColorPickerModel::new(
                fl!("hex"),
                fl!("rgb"),
                None,
                theme_builder.primary_container_bg.map(Color::from),
            ),
            interface_text: ColorPickerModel::new(
                fl!("hex"),
                fl!("rgb"),
                Some(theme.background.on.into()),
                theme_builder.text_tint.map(Color::from),
            ),
            control_component: ColorPickerModel::new(
                fl!("hex"),
                fl!("rgb"),
                Some(theme.palette.neutral_5.into()),
                theme_builder.neutral_tint.map(Color::from),
            ),
            accent_window_hint: ColorPickerModel::new(
                fl!("hex"),
                fl!("rgb"),
                None,
                theme_builder.window_hint.map(Color::from),
            ),
            no_custom_window_hint: theme_builder.accent.is_some(),
            active_dialog: None,
            theme_mode_config,
            theme_builder_config,
            theme_mode,
            theme_builder,
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
        (
            theme_mode_config,
            theme_mode,
            theme_builder_config,
            theme_builder,
        )
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Entered,
    DarkMode(bool),
    Autoswitch(bool),
    Frosted(bool),
    WindowHintSize(spin_button::Message),
    GapSize(spin_button::Message),
    AccentWindowHint(ColorPickerUpdate),
    ApplicationBackground(ColorPickerUpdate),
    ContainerBackground(ColorPickerUpdate),
    PaletteAccent(cosmic::iced::Color),
    CustomAccent(ColorPickerUpdate),
    InterfaceText(ColorPickerUpdate),
    ControlComponent(ColorPickerUpdate),
    CloseRequested(window::Id),
    Roundness(Roundness),
    StartImport,
    StartExport,
    ImportFile(Arc<SelectedFiles>),
    ExportFile(Arc<SelectedFiles>),
    ExportSuccess,
    ImportSuccess(Box<ThemeBuilder>),
    ImportError,
    ExportError,
    Reset,
    Left,
    UseDefaultWindowHint(bool),
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
    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        let mut theme_builder_needs_update = false;

        let mut ret = match message {
            Message::DarkMode(enabled) => {
                self.theme_mode.is_dark = enabled;
                if let Some(config) = self.theme_mode_config.as_ref() {
                    // only update dark mode if autoswitch is disabled
                    if !self.theme_mode.auto_switch {
                        _ = config.set::<bool>("is_dark", enabled);
                    }
                }
                *self = Self::from((self.theme_mode_config.clone(), self.theme_mode));

                let theme_builder = self.theme_builder.clone();
                Command::perform(async {}, |_| {
                    crate::Message::SetTheme(cosmic::theme::Theme::custom(Arc::new(
                        // TODO set the values of the theme builder
                        theme_builder.build(),
                    )))
                })
            }
            Message::Autoswitch(enabled) => {
                self.theme_mode.auto_switch = enabled;
                if let Some(config) = self.theme_mode_config.as_ref() {
                    _ = config.set::<bool>("auto_switch", enabled);
                }
                if !enabled {}
                Command::none()
            }
            Message::AccentWindowHint(u) => {
                let cmd = match &u {
                    ColorPickerUpdate::AppliedColor | ColorPickerUpdate::Reset => {
                        // close the color picker dialog
                        // apply changes
                        theme_builder_needs_update = true;
                        close_window::<app::Message>(COLOR_PICKER_DIALOG_ID)
                    }
                    // TODO apply changes
                    ColorPickerUpdate::ActionFinished => {
                        theme_builder_needs_update = true;
                        _ = self
                            .accent_window_hint
                            .update::<app::Message>(ColorPickerUpdate::AppliedColor);
                        Command::none()
                    }
                    ColorPickerUpdate::Cancel => {
                        // close the color picker dialog
                        close_window(COLOR_PICKER_DIALOG_ID)
                    }
                    ColorPickerUpdate::ToggleColorPicker => {
                        // create the color picker dialog
                        // set the active picker
                        self.active_dialog = Some(NamedColorPicker::AccentWindowHint);
                        get_window(color_picker_window_settings())
                    }
                    _ => Command::none(),
                };
                Command::batch(vec![cmd, self.accent_window_hint.update::<app::Message>(u)])
            }
            Message::Frosted(enabled) => {
                theme_builder_needs_update = true;
                self.theme_builder.is_frosted = enabled;
                Command::none()
            }
            Message::WindowHintSize(msg) => {
                theme_builder_needs_update = true;
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
                theme_builder_needs_update = true;
                self.theme_builder.gaps.1 = match msg {
                    spin_button::Message::Increment => self.theme_builder.gaps.1.saturating_add(1),
                    spin_button::Message::Decrement => self.theme_builder.gaps.1.saturating_sub(1),
                };
                Command::none()
            }
            Message::ApplicationBackground(u) => {
                let cmd = match &u {
                    ColorPickerUpdate::AppliedColor | ColorPickerUpdate::Reset => {
                        // close the color picker dialog
                        // apply changes
                        theme_builder_needs_update = true;
                        close_window::<app::Message>(COLOR_PICKER_DIALOG_ID)
                    }
                    // TODO apply changes
                    ColorPickerUpdate::ActionFinished => {
                        theme_builder_needs_update = true;
                        _ = self
                            .application_background
                            .update::<app::Message>(ColorPickerUpdate::AppliedColor);
                        Command::none()
                    }
                    ColorPickerUpdate::Cancel => {
                        // close the color picker dialog
                        close_window(COLOR_PICKER_DIALOG_ID)
                    }
                    ColorPickerUpdate::ToggleColorPicker => {
                        // create the color picker dialog
                        // set the active picker
                        self.active_dialog = Some(NamedColorPicker::ApplicationBackground);
                        get_window(color_picker_window_settings())
                    }
                    _ => Command::none(),
                };
                Command::batch(vec![
                    cmd,
                    self.application_background.update::<app::Message>(u),
                ])
            }
            Message::ContainerBackground(u) => {
                let cmd = match &u {
                    ColorPickerUpdate::AppliedColor | ColorPickerUpdate::Reset => {
                        theme_builder_needs_update = true;
                        Command::perform(async {}, |_| crate::app::Message::CloseContextDrawer)
                    }
                    ColorPickerUpdate::ActionFinished => {
                        theme_builder_needs_update = true;
                        _ = self
                            .container_background
                            .update::<app::Message>(ColorPickerUpdate::AppliedColor);
                        Command::none()
                    }
                    ColorPickerUpdate::Cancel => {
                        Command::perform(async {}, |_| crate::app::Message::CloseContextDrawer)
                    }
                    ColorPickerUpdate::ToggleColorPicker => Command::perform(async {}, |_| {
                        crate::app::Message::OpenContextDrawer(fl!("container-background").into())
                    }),
                    _ => Command::none(),
                };
                Command::batch(vec![
                    cmd,
                    self.container_background.update::<app::Message>(u),
                ])
            }
            Message::CustomAccent(u) => {
                let cmd = match &u {
                    ColorPickerUpdate::AppliedColor | ColorPickerUpdate::Reset => {
                        // close the color picker dialog
                        // apply changes
                        theme_builder_needs_update = true;
                        close_window::<app::Message>(COLOR_PICKER_DIALOG_ID)
                    }
                    // TODO apply changes
                    ColorPickerUpdate::ActionFinished => {
                        theme_builder_needs_update = true;
                        _ = self
                            .custom_accent
                            .update::<app::Message>(ColorPickerUpdate::AppliedColor);
                        Command::none()
                    }
                    ColorPickerUpdate::Cancel => {
                        // close the color picker dialog
                        close_window(COLOR_PICKER_DIALOG_ID)
                    }
                    ColorPickerUpdate::ToggleColorPicker => {
                        // create the color picker dialog
                        // set the active picker
                        self.active_dialog = Some(NamedColorPicker::CustomAccent);
                        get_window(color_picker_window_settings())
                    }
                    _ => Command::none(),
                };
                let cmd2 = self.custom_accent.update::<app::Message>(u);

                self.theme_builder.accent = self.custom_accent.get_applied_color().map(Srgb::from);
                Command::batch(vec![cmd, cmd2])
            }
            Message::InterfaceText(u) => {
                let cmd = match &u {
                    ColorPickerUpdate::AppliedColor | ColorPickerUpdate::Reset => {
                        // close the color picker dialog
                        // apply changes
                        theme_builder_needs_update = true;
                        close_window::<app::Message>(COLOR_PICKER_DIALOG_ID)
                    }
                    // TODO apply changes
                    ColorPickerUpdate::ActionFinished => {
                        theme_builder_needs_update = true;
                        _ = self
                            .interface_text
                            .update::<app::Message>(ColorPickerUpdate::AppliedColor);
                        Command::none()
                    }
                    ColorPickerUpdate::Cancel => {
                        // close the color picker dialog
                        close_window(COLOR_PICKER_DIALOG_ID)
                    }
                    ColorPickerUpdate::ToggleColorPicker => {
                        // create the color picker dialog
                        // set the active picker
                        self.active_dialog = Some(NamedColorPicker::InterfaceText);
                        get_window(color_picker_window_settings())
                    }
                    _ => Command::none(),
                };
                Command::batch(vec![cmd, self.interface_text.update::<app::Message>(u)])
            }
            Message::ControlComponent(u) => {
                let cmd = match &u {
                    ColorPickerUpdate::AppliedColor | ColorPickerUpdate::Reset => {
                        // close the color picker dialog
                        // apply changes
                        theme_builder_needs_update = true;
                        close_window::<app::Message>(COLOR_PICKER_DIALOG_ID)
                    }
                    // TODO apply changes
                    ColorPickerUpdate::ActionFinished => {
                        theme_builder_needs_update = true;
                        _ = self
                            .control_component
                            .update::<app::Message>(ColorPickerUpdate::AppliedColor);
                        Command::none()
                    }
                    ColorPickerUpdate::Cancel => {
                        // close the color picker dialog
                        close_window(COLOR_PICKER_DIALOG_ID)
                    }
                    ColorPickerUpdate::ToggleColorPicker => {
                        // create the color picker dialog
                        // set the active picker
                        self.active_dialog = Some(NamedColorPicker::ControlComponent);
                        get_window(color_picker_window_settings())
                    }
                    _ => Command::none(),
                };
                Command::batch(vec![cmd, self.control_component.update::<app::Message>(u)])
            }
            Message::CloseRequested(_) => {
                match self.active_dialog.take() {
                    Some(NamedColorPicker::ApplicationBackground) => {
                        _ = self
                            .application_background
                            .update::<app::Message>(ColorPickerUpdate::Cancel);
                    }
                    Some(NamedColorPicker::ControlComponent) => {
                        _ = self
                            .control_component
                            .update::<app::Message>(ColorPickerUpdate::Cancel);
                    }
                    Some(NamedColorPicker::CustomAccent) => {
                        _ = self
                            .custom_accent
                            .update::<app::Message>(ColorPickerUpdate::Cancel);
                    }
                    Some(NamedColorPicker::InterfaceText) => {
                        _ = self
                            .interface_text
                            .update::<app::Message>(ColorPickerUpdate::AppliedColor);
                    }
                    Some(NamedColorPicker::AccentWindowHint) => {
                        _ = self
                            .accent_window_hint
                            .update::<app::Message>(ColorPickerUpdate::AppliedColor);
                    }
                    None => {
                        theme_builder_needs_update = false;
                        warn!("Unknown appearance dialog closed.");
                    }
                };
                Command::none()
            }
            Message::Roundness(r) => {
                self.roundness = r;
                Command::none()
            }
            Message::Entered => {
                *self = Self::default();
                let theme_builder = self.theme_builder.clone();
                Command::perform(async {}, |_| {
                    crate::Message::SetTheme(cosmic::theme::Theme::custom(Arc::new(
                        // TODO set the values of the theme builder
                        theme_builder.build(),
                    )))
                })
                // Load the current theme builders and mode
                // Set the theme for the application to match the current mode instead of the system theme?
            }
            Message::Left => Command::perform(async {}, |_| {
                app::Message::SetTheme(cosmic::theme::system_preference())
            }),
            Message::PaletteAccent(c) => {
                self.theme_builder.accent = Some(c.into());
                theme_builder_needs_update = true;
                Command::none()
            }
            Message::Reset => {
                self.theme_builder = if self.theme_mode.is_dark {
                    ThemeBuilder::dark()
                } else {
                    ThemeBuilder::light()
                };
                if let Some(config) = self.theme_builder_config.as_ref() {
                    _ = self.theme_builder.write_entry(config);
                };

                let config = if self.theme_mode.is_dark {
                    Theme::<Srgba>::dark_config()
                } else {
                    Theme::<Srgba>::light_config()
                };
                let new_theme = self.theme_builder.clone().build();
                if let Ok(config) = config {
                    _ = new_theme.write_entry(&config);
                } else {
                    tracing::error!("Failed to get the theme config.");
                }

                *self = Self::from((self.theme_mode_config.clone(), self.theme_mode));
                Command::perform(async {}, |_| {
                    crate::Message::SetTheme(cosmic::theme::Theme::custom(Arc::new(new_theme)))
                })
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
                let Some(f) = f.uris().get(0) else {
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
                let Some(f) = f.uris().get(0) else {
                    return Command::none();
                };
                if f.scheme() != "file" {
                    return Command::none();
                }
                let Ok(path) = f.to_file_path() else {
                    return Command::none();
                };
                let Ok(builder) = ron::ser::to_string_pretty(&self.theme_builder, PrettyConfig::default()) else {
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
                    Theme::<Srgba>::dark_config()
                } else {
                    Theme::<Srgba>::light_config()
                };
                let new_theme = self.theme_builder.clone().build();
                if let Ok(config) = config {
                    _ = new_theme.write_entry(&config);
                } else {
                    tracing::error!("Failed to get the theme config.");
                }

                *self = Self::from((self.theme_mode_config.clone(), self.theme_mode));
                Command::perform(async {}, |_| {
                    crate::Message::SetTheme(cosmic::theme::Theme::custom(Arc::new(new_theme)))
                })
            }
            Message::UseDefaultWindowHint(v) => {
                self.no_custom_window_hint = v;
                theme_builder_needs_update = true;
                let theme: Theme<palette::Srgba> = if self.theme_mode.is_dark {
                    Theme::dark_default()
                } else {
                    Theme::light_default()
                };
                let window_hint = self
                    .theme_builder
                    .window_hint
                    .filter(|c| {
                        let c = Srgba::new(c.red, c.green, c.blue, 1.0);
                        !v && c != theme.palette.accent_blue
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
                self.accent_window_hint
                    .update(ColorPickerUpdate::ActiveColor(Hsv::from_color(window_hint)))
            }
        };

        if theme_builder_needs_update {
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
            theme_builder.window_hint = self.accent_window_hint.get_applied_color().map(Srgb::from);

            _ = theme_builder.write_entry(config);

            self.theme_builder = theme_builder;

            let config = if self.theme_mode.is_dark {
                Theme::<Srgba>::dark_config()
            } else {
                Theme::<Srgba>::light_config()
            };
            if let Ok(config) = config {
                let new_theme = self.theme_builder.clone().build();
                _ = new_theme.write_entry(&config);
            } else {
                tracing::error!("Failed to get the theme config.");
            }
            let theme_builder = self.theme_builder.clone();
            ret = Command::batch(vec![
                ret,
                Command::perform(async {}, |_| {
                    crate::Message::SetTheme(cosmic::theme::Theme::custom(Arc::new(
                        theme_builder.build(),
                    )))
                }),
            ]);
        }

        self.can_reset = if self.theme_mode.is_dark {
            self.theme_builder != ThemeBuilder::dark()
        } else {
            self.theme_builder != ThemeBuilder::light()
        };

        ret
    }

    pub fn color_picker_view(&self) -> Element<crate::app::Message> {
        let (picker, msg): (_, fn(ColorPickerUpdate) -> Message) = match self.active_dialog {
            Some(NamedColorPicker::ApplicationBackground) => {
                (&self.application_background, Message::ApplicationBackground)
            }
            Some(NamedColorPicker::ControlComponent) => {
                (&self.control_component, Message::ControlComponent)
            }
            Some(NamedColorPicker::CustomAccent) => (&self.custom_accent, Message::CustomAccent),
            Some(NamedColorPicker::InterfaceText) => (&self.interface_text, Message::InterfaceText),
            Some(NamedColorPicker::AccentWindowHint) => {
                (&self.accent_window_hint, Message::AccentWindowHint)
            }
            None => return text("OOPS!").into(),
        };
        column![
            header_bar()
                .title(fl!("color-picker"))
                .on_close(msg(ColorPickerUpdate::AppliedColor)),
            picker
                .builder(msg)
                .width(Length::Fixed(254.0))
                .height(Length::Fixed(174.0))
                .reset_label(fl!("reset-to-default"))
                .build(
                    fl!("recent-colors"),
                    fl!("copy-to-clipboard"),
                    fl!("copied-to-clipboard"),
                )
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(cosmic::iced_core::Alignment::Center)
        .apply(Element::from)
        .map(crate::pages::Message::Appearance)
        .map(crate::app::Message::PageMessage)
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(import_export()),
            sections.insert(mode_and_colors()),
            sections.insert(style()),
            sections.insert(window_management()),
            sections.insert(reset_button()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("appearance", "preferences-pop-desktop-appearance-symbolic")
            .title(fl!("appearance"))
            .description(fl!("appearance", "desc"))
    }

    fn load(&self, _: page::Entity) -> Option<page::Task<crate::pages::Message>> {
        Some(Box::pin(async move {
            crate::pages::Message::Appearance(Message::Entered)
        }))
    }

    fn on_leave(&mut self) -> Command<crate::pages::Message> {
        Command::perform(async {}, |_| {
            crate::pages::Message::Appearance(Message::Left)
        })
    }

    fn context_drawer(&self) -> Option<Element<'_, crate::pages::Message>> {
        Some(
            column![
                text(fl!("container-background", "desc-detail")).width(Length::Fill),
                self.container_background
                    .builder(Message::ContainerBackground)
                    .width(Length::Fixed(248.0))
                    .height(Length::Fixed(158.0))
                    .reset_label(fl!("container-background", "reset"))
                    .build(
                        fl!("recent-colors"),
                        fl!("copy-to-clipboard"),
                        fl!("copied-to-clipboard"),
                    )
            ]
            .padding(self.theme_builder.spacing.space_l)
            .align_items(cosmic::iced_core::Alignment::Center)
            .spacing(self.theme_builder.spacing.space_m)
            .apply(Element::from)
            .map(crate::pages::Message::Appearance),
        )
    }
}

fn import_export() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![fl!("import"), fl!("export")])
        .view::<Page>(|_binder, page, section| {
            let spacing = &page.theme_builder.spacing;
            let descriptions = &section.descriptions;
            container(
                row![
                    button(text(&descriptions[0]))
                        .on_press(Message::StartImport)
                        .padding([spacing.space_xxs, spacing.space_xs]),
                    button(text(&descriptions[1]))
                        .on_press(Message::StartExport)
                        .padding([spacing.space_xxs, spacing.space_xs])
                ]
                .spacing(8.0),
            )
            .width(Length::Fill)
            .align_x(cosmic::iced_core::alignment::Horizontal::Right)
            .apply(Element::from)
            .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
pub fn mode_and_colors() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("mode-and-colors"))
        .descriptions(vec![
            // 0
            fl!("auto-switch"),
            fl!("auto-switch", "desc"),
            //2
            fl!("accent-color"),
            //3
            fl!("app-background"),
            //4
            fl!("container-background"),
            fl!("container-background", "desc"),
            fl!("container-background", "desc-detail"),
            fl!("container-background", "reset"),
            // 8
            fl!("text-tint"),
            fl!("text-tint", "desc"),
            // 10
            fl!("control-tint"),
            fl!("control-tint", "desc"),
            // 12
            fl!("window-hint-accent-toggle"),
            fl!("window-hint-accent"),
            // 14
            fl!("dark"),
            fl!("light"),
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
                        row![
                            column![
                                button(
                                    icon(from_name("illustration-appearance-mode-dark").into(),)
                                        .width(Length::Fill)
                                        .height(Length::Fixed(100.0))
                                )
                                .style(cosmic::theme::Button::IconVertical)
                                .padding(8)
                                .on_press(Message::DarkMode(true)),
                                text(&descriptions[13])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_items(cosmic::iced_core::Alignment::Center),
                            column![
                                button(
                                    icon(from_name("illustration-appearance-mode-light").into(),)
                                        .width(Length::Fill)
                                        .height(Length::Fixed(100.0))
                                )
                                .style(cosmic::theme::Button::IconVertical)
                                .padding(8)
                                .on_press(Message::DarkMode(false)),
                                text(&descriptions[14])
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
                    settings::item::builder(&descriptions[0])
                        .description(&descriptions[1])
                        .toggler(page.theme_mode.auto_switch, Message::Autoswitch),
                )
                .add(
                    column![
                        text(&descriptions[2]),
                        scrollable(
                            row![
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_blue.into())),
                                    palette.accent_blue.into(),
                                    cur_accent == palette.accent_blue,
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_indigo.into())),
                                    palette.accent_indigo.into(),
                                    cur_accent == palette.accent_indigo,
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_purple.into())),
                                    palette.accent_purple.into(),
                                    cur_accent == palette.accent_purple,
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_pink.into())),
                                    palette.accent_pink.into(),
                                    cur_accent == palette.accent_pink,
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_red.into())),
                                    palette.accent_red.into(),
                                    cur_accent == palette.accent_red,
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_orange.into())),
                                    palette.accent_orange.into(),
                                    cur_accent == palette.accent_orange,
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_yellow.into())),
                                    palette.accent_yellow.into(),
                                    cur_accent == palette.accent_yellow,
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_green.into())),
                                    palette.accent_green.into(),
                                    cur_accent == palette.accent_green,
                                ),
                                color_button(
                                    Some(Message::PaletteAccent(palette.accent_warm_grey.into())),
                                    palette.accent_warm_grey.into(),
                                    cur_accent == palette.accent_warm_grey,
                                ),
                                page.custom_accent
                                    .picker_button(Message::CustomAccent, None)
                                    .width(Length::Fixed(48.0))
                                    .height(Length::Fixed(48.0))
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
                    settings::item::builder(&descriptions[3]).control(
                        page.application_background
                            .picker_button(Message::ApplicationBackground, Some(24))
                            .width(Length::Fixed(48.0))
                            .height(Length::Fixed(24.0)),
                    ),
                )
                .add(
                    settings::item::builder(&descriptions[4])
                        .description(&descriptions[5])
                        .control(
                            if let Some(c) = page.container_background.get_applied_color() {
                                container(
                                    color_button(
                                        Some(Message::ContainerBackground(
                                            ColorPickerUpdate::ToggleColorPicker,
                                        )),
                                        c,
                                        true,
                                    )
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
                            },
                        ),
                )
                .add(
                    settings::item::builder(&descriptions[8])
                        .description(&descriptions[9])
                        .control(
                            page.interface_text
                                .picker_button(Message::InterfaceText, Some(24))
                                .width(Length::Fixed(48.0))
                                .height(Length::Fixed(24.0)),
                        ),
                )
                .add(
                    settings::item::builder(&descriptions[10])
                        .description(&descriptions[11])
                        .control(
                            page.control_component
                                .picker_button(Message::ControlComponent, Some(24))
                                .width(Length::Fixed(48.0))
                                .height(Length::Fixed(24.0)),
                        ),
                )
                .add(
                    settings::item::builder(&descriptions[12])
                        .toggler(page.no_custom_window_hint, Message::UseDefaultWindowHint),
                );
            if !page.no_custom_window_hint {
                section = section.add(
                    settings::item::builder(&descriptions[13]).control(
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
            fl!("style", "round"),
            fl!("style", "slightly-round"),
            fl!("style", "square"),
            fl!("frosted"),
            fl!("frosted", "desc"),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(
                    container(
                        row![
                            column![
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
                                .style(cosmic::theme::Button::IconVertical)
                                .padding(8)
                                .on_press(Message::Roundness(Roundness::Round)),
                                text(&descriptions[0])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_items(cosmic::iced_core::Alignment::Center),
                            column![
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
                                .style(cosmic::theme::Button::IconVertical)
                                .padding(8)
                                .on_press(Message::Roundness(Roundness::SlightlyRound)),
                                text(&descriptions[1])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_items(cosmic::iced_core::Alignment::Center),
                            column![
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
                                .style(cosmic::theme::Button::IconVertical)
                                .padding(8)
                                .on_press(Message::Roundness(Roundness::Square)),
                                text(&descriptions[2])
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
                    settings::item::builder(&descriptions[3])
                        .description(&descriptions[4])
                        .toggler(page.theme_builder.is_frosted, Message::Frosted),
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
            fl!("window-management", "active-hint"),
            fl!("window-management", "gaps"),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(settings::item::builder(&descriptions[0]).control(
                    cosmic::widget::spin_button(
                        page.theme_builder.active_hint.to_string(),
                        Message::WindowHintSize,
                    ),
                ))
                .add(settings::item::builder(&descriptions[1]).control(
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
        .descriptions(vec![fl!("reset-default")])
        .view::<Page>(|_binder, page, section| {
            let spacing = &page.theme_builder.spacing;
            let descriptions = &section.descriptions;
            if page.can_reset {
                row![button(text(&descriptions[0]))
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

fn color_picker_window_settings() -> SctkWindowSettings {
    SctkWindowSettings {
        window_id: COLOR_PICKER_DIALOG_ID,
        app_id: Some("com.system76.CosmicSettings".to_string()),
        title: Some(fl!("color-picker")),
        parent: Some(window::Id(0)),
        autosize: false,
        size_limits: layout::Limits::NONE
            .min_width(300.0)
            .max_width(800.0)
            .min_height(200.0)
            .max_height(1080.0),
        size: (286, 424),
        resizable: None,
        client_decorations: true,
        transparent: true,
    }
}

// TODO replace with image button / toggle buttons
fn color_button<'a, Message: 'a>(
    on_press: Option<Message>,
    color: cosmic::iced::Color,
    selected: bool,
) -> cosmic::widget::Button<'a, Message, cosmic::Renderer> {
    let ret = button(cosmic::widget::vertical_space(Length::Fixed(48.0)))
        .width(Length::Fixed(48.0))
        .height(Length::Fixed(48.0))
        .on_press_maybe(on_press);
    style_color_button(ret, color, selected)
}

fn style_color_button<'a, Message: 'a>(
    b: button::Button<'a, Message, cosmic::Renderer>,
    color: cosmic::iced::Color,
    selected: bool,
) -> button::Button<'a, Message, cosmic::Renderer> {
    b.style(cosmic::theme::Button::Custom {
        active: Box::new(move |focused, theme| {
            let cosmic = theme.cosmic();

            let (outline_width, outline_color) = if focused {
                (1.0, cosmic.accent_color().into())
            } else {
                (0.0, cosmic::iced::Color::TRANSPARENT)
            };
            cosmic::widget::button::Appearance {
                shadow_offset: cosmic::iced_core::Vector::default(),
                background: Some(color.into()),
                border_radius: cosmic.radius_xs().into(),
                border_width: 1.0,
                border_color: if selected {
                    cosmic.on_bg_color().into()
                } else {
                    cosmic::iced::Color::TRANSPARENT
                },
                outline_width,
                outline_color,
                icon_color: None,
                text_color: None,
            }
        }),
        disabled: Box::new(move |theme| {
            let cosmic = theme.cosmic();

            cosmic::widget::button::Appearance {
                shadow_offset: cosmic::iced_core::Vector::default(),
                background: Some(color.into()),
                border_radius: cosmic.radius_xs().into(),
                border_width: 1.0,
                border_color: if selected {
                    cosmic.on_bg_color().into()
                } else {
                    cosmic::iced::Color::TRANSPARENT
                },
                outline_width: 0.0,
                outline_color: cosmic::iced::Color::TRANSPARENT,
                icon_color: None,
                text_color: None,
            }
        }),
        hovered: Box::new(move |focused, theme| {
            let cosmic = theme.cosmic();

            let (outline_width, outline_color) = if focused {
                (1.0, cosmic.accent_color().into())
            } else {
                (0.0, cosmic::iced::Color::TRANSPARENT)
            };
            cosmic::widget::button::Appearance {
                shadow_offset: cosmic::iced_core::Vector::default(),
                background: Some(color.into()),
                border_radius: cosmic.radius_xs().into(),
                border_width: 1.0,
                border_color: if selected {
                    cosmic.on_bg_color().into()
                } else {
                    cosmic::iced::Color::TRANSPARENT
                },
                outline_width,
                outline_color,
                icon_color: None,
                text_color: None,
            }
        }),
        pressed: Box::new(move |focused, theme| {
            let cosmic = theme.cosmic();

            let (outline_width, outline_color) = if focused {
                (1.0, cosmic.accent_color().into())
            } else {
                (0.0, cosmic::iced::Color::TRANSPARENT)
            };
            cosmic::widget::button::Appearance {
                shadow_offset: cosmic::iced_core::Vector::default(),
                background: Some(color.into()),
                border_radius: cosmic.radius_xs().into(),
                border_width: 1.0,
                border_color: if selected {
                    cosmic.on_bg_color().into()
                } else {
                    cosmic::iced::Color::TRANSPARENT
                },
                outline_width,
                outline_color,
                icon_color: None,
                text_color: None,
            }
        }),
    })
}
