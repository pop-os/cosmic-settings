// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod font_config;
pub mod icon_themes;

use std::borrow::Cow;
use std::sync::Arc;

use cosmic::app::ContextDrawer;
//TODO: use embedded cosmic-files for portability
use cosmic::config::CosmicTk;
use cosmic::cosmic_config::{Config, ConfigSet, CosmicConfigEntry};
use cosmic::cosmic_theme::palette::{FromColor, Hsv, Srgb, Srgba};
use cosmic::cosmic_theme::{
    CornerRadii, DARK_THEME_BUILDER_ID, Density, LIGHT_THEME_BUILDER_ID, Spacing, Theme,
    ThemeBuilder, ThemeMode,
};
#[cfg(feature = "xdg-portal")]
use cosmic::dialog::file_chooser::{self, FileFilter};
use cosmic::iced_core::{Alignment, Color, Length};
use cosmic::iced_widget::scrollable::{Direction, Scrollbar};
use cosmic::widget::icon::{from_name, icon};
use cosmic::widget::{
    ColorPickerModel, button, color_picker::ColorPickerUpdate, container, flex_row,
    horizontal_space, radio, row, scrollable, settings, text,
};
use cosmic::{Apply, Element, Task, widget};
#[cfg(feature = "wayland")]
use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use cosmic_settings_wallpaper as wallpaper;
use icon_themes::{IconHandles, IconThemes};
use ron::ser::PrettyConfig;
use serde::Serialize;
use slab::Slab;
use slotmap::{Key, SlotMap};

use crate::app;
use crate::widget::color_picker_context_view;

use super::wallpaper::widgets::color_image;

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
    IconsAndToolkit,
    InterfaceText,
    MonospaceFont,
    SystemFont,
}

#[allow(clippy::struct_excessive_bools)]
pub struct Page {
    entity: page::Entity,
    on_enter_handle: Option<cosmic::iced::task::Handle>,
    accent_palette: AccentPalette,
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

    font_config: font_config::Model,
    font_filter: Vec<Arc<str>>,
    font_search: String,

    /**  Only fetch icons once. Allows for better cleanup. Also icon fetching can take for ages. */
    icons_fetched: bool,
    icon_fetch_handle: Option<cosmic::iced::task::Handle>,

    icon_theme_active: Option<usize>,
    icon_themes: IconThemes,
    icon_handles: IconHandles,

    theme: Theme,
    theme_mode: ThemeMode,
    theme_mode_config: Option<Config>,
    theme_builder: ThemeBuilder,
    theme_builder_config: Option<Config>,

    auto_switch_descs: [Cow<'static, str>; 4],

    tk_config: Option<Config>,
    settings_config: crate::config::Config,
    day_time: bool,
}

#[derive(Default)]
pub struct AccentPalette {
    dark: Option<Vec<Srgba>>,
    light: Option<Vec<Srgba>>,
    theme: Vec<Srgba>,
}

impl Default for Page {
    fn default() -> Self {
        let settings_config = crate::config::Config::new();

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

        let accent_palette = AccentPalette {
            dark: settings_config.accent_palette_dark().ok(),
            light: settings_config.accent_palette_light().ok(),
            theme: Vec::new(),
        };

        let mut page: Page = (
            settings_config,
            theme_mode_config,
            theme_mode,
            accent_palette,
        )
            .into();
        page.update_accent_palette();
        page
    }
}

impl
    From<(
        crate::config::Config,
        Option<Config>,
        ThemeMode,
        Option<Config>,
        ThemeBuilder,
        Option<Config>,
        AccentPalette,
    )> for Page
{
    fn from(
        (
            settings_config,
            theme_mode_config,
            theme_mode,
            theme_builder_config,
            mut theme_builder,
            tk_config,
            accent_palette,
        ): (
            crate::config::Config,
            Option<Config>,
            ThemeMode,
            Option<Config>,
            ThemeBuilder,
            Option<Config>,
            AccentPalette,
        ),
    ) -> Self {
        let theme = if theme_mode.is_dark {
            Theme::dark_default()
        } else {
            Theme::light_default()
        };
        theme_builder = theme_builder
            .clone()
            .accent(theme.accent.base.color)
            .bg_color(theme.bg_color())
            .corner_radii(theme.corner_radii)
            .destructive(theme.destructive.base.color)
            .spacing(theme.spacing)
            .success(theme.success.base.color)
            .warning(theme.warning.base.color)
            .neutral_tint(theme.palette.neutral_5.color)
            .text_tint(theme.background.on.color);
        theme_builder.gaps = theme.gaps;

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
            entity: page::Entity::null(),
            on_enter_handle: None,
            can_reset: if theme_mode.is_dark {
                theme_builder == ThemeBuilder::dark()
            } else {
                theme_builder == ThemeBuilder::light()
            },
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
            no_custom_window_hint: theme_builder.window_hint.is_none(),
            font_config: font_config::Model::new(),
            font_filter: Vec::new(),
            font_search: String::new(),
            icons_fetched: false,
            icon_fetch_handle: None,
            icon_theme_active: None,
            icon_themes: Vec::new(),
            icon_handles: Vec::new(),
            accent_palette,
            theme,
            theme_mode_config,
            theme_builder_config,
            theme_mode,
            theme_builder,
            tk_config,
            settings_config,
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

impl
    From<(
        crate::config::Config,
        Option<Config>,
        ThemeMode,
        AccentPalette,
    )> for Page
{
    fn from(
        (settings_config, theme_mode_config, theme_mode, accent_palette): (
            crate::config::Config,
            Option<Config>,
            ThemeMode,
            AccentPalette,
        ),
    ) -> Self {
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

        Self::from((
            settings_config,
            theme_mode_config,
            theme_mode,
            theme_builder_config,
            theme_builder,
            tk_config,
            accent_palette,
        ))
    }
}

#[cfg(feature = "xdg-portal")]
#[derive(Clone)]
pub struct SaveResponse(pub Arc<file_chooser::save::Response>);

#[cfg(feature = "xdg-portal")]
impl std::fmt::Debug for SaveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SaveResponse")
    }
}

#[cfg(feature = "xdg-portal")]
#[derive(Clone)]
pub struct OpenResponse(pub Arc<file_chooser::open::FileResponse>);

#[cfg(feature = "xdg-portal")]
impl std::fmt::Debug for OpenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("OpenResponse")
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
    DisplayMonoFont,
    DisplaySystemFont,
    Entered((IconThemes, IconHandles)),
    IconsAndToolkit,
    #[cfg(feature = "xdg-portal")]
    ExportError,
    #[cfg(feature = "xdg-portal")]
    ExportFile(SaveResponse),
    #[cfg(feature = "xdg-portal")]
    ExportSuccess,
    FontConfig(font_config::Message),
    FontSearch(String),
    FontSelect(bool, Arc<str>),
    GapSize(u32),
    IconTheme(usize),
    #[cfg(feature = "xdg-portal")]
    ImportError,
    #[cfg(feature = "xdg-portal")]
    ImportFile(OpenResponse),
    #[cfg(feature = "xdg-portal")]
    ImportSuccess(Box<ThemeBuilder>),
    InterfaceText(ColorPickerUpdate),
    Left,
    NewTheme(Box<Theme>),
    PaletteAccent(cosmic::iced::Color),
    Reset,
    Roundness(Roundness),
    #[cfg(feature = "xdg-portal")]
    StartExport,
    #[cfg(feature = "xdg-portal")]
    StartImport,
    UseDefaultWindowHint(bool),
    WindowHintSize(u32),
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
    fn icons_and_toolkit(&self) -> Element<'_, crate::pages::Message> {
        let active = self.icon_theme_active;
        let theme = cosmic::theme::active();
        let theme = theme.cosmic();
        cosmic::iced::widget::column![
            // Export theme choice
            settings::section().add(
                settings::item::builder(fl!("enable-export"))
                    .description(fl!("enable-export", "desc"))
                    .toggler(
                        cosmic::config::apply_theme_global(),
                        Message::ApplyThemeGlobal
                    )
            ),
            // Icon theme previews
            widget::column::with_children(vec![
                text::heading(&*ICON_THEME).into(),
                flex_row(
                    self.icon_themes
                        .iter()
                        .zip(self.icon_handles.iter())
                        .enumerate()
                        .map(|(i, (theme, handles))| {
                            let selected = active.map(|j| i == j).unwrap_or_default();
                            icon_themes::button(&theme.name, handles, i, selected)
                        })
                        .collect(),
                )
                .row_spacing(theme.space_xs())
                .column_spacing(theme.space_xs())
                .apply(container)
                .center_x(Length::Fill)
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
    pub fn update(&mut self, message: Message) -> Task<app::Message> {
        let mut tasks = Vec::new();

        let mut needs_build = false;
        let mut needs_sync = false;

        match message {
            Message::DisplayMonoFont => {
                self.context_view = Some(ContextView::MonospaceFont);
                self.font_search.clear();

                return cosmic::task::message(crate::app::Message::OpenContextDrawer(
                    self.entity,
                    fl!("monospace-font").into(),
                ));
            }

            Message::DisplaySystemFont => {
                self.context_view = Some(ContextView::SystemFont);
                self.font_search.clear();

                return cosmic::task::message(crate::app::Message::OpenContextDrawer(
                    self.entity,
                    fl!("interface-font").into(),
                ));
            }

            Message::FontConfig(message) => {
                return self.font_config.update(message);
            }

            Message::FontSearch(input) => {
                self.font_search = input.to_lowercase();
                self.font_filter.clear();

                match self.context_view {
                    Some(ContextView::SystemFont) => {
                        self.font_config
                            .interface_font_families
                            .iter()
                            .filter(|f| f.to_lowercase().contains(&self.font_search))
                            .for_each(|f| self.font_filter.push(f.clone()));
                    }

                    Some(ContextView::MonospaceFont) => {
                        self.font_config
                            .monospace_font_families
                            .iter()
                            .filter(|f| f.to_lowercase().contains(&self.font_search))
                            .for_each(|f| self.font_filter.push(f.clone()));
                    }

                    _ => (),
                }
            }

            Message::FontSelect(is_system, family) => {
                if is_system {
                    if let Some(id) = self
                        .font_config
                        .interface_font_families
                        .iter()
                        .position(|f| f == &family)
                    {
                        return self
                            .font_config
                            .update(font_config::Message::InterfaceFontFamily(id));
                    }
                } else if let Some(id) = self
                    .font_config
                    .monospace_font_families
                    .iter()
                    .position(|f| f == &family)
                {
                    return self
                        .font_config
                        .update(font_config::Message::MonospaceFontFamily(id));
                }
            }

            Message::NewTheme(theme) => {
                self.theme = *theme;
                self.theme_builder = self
                    .theme_builder
                    .clone()
                    .accent(self.theme.accent.base.color)
                    .bg_color(self.theme.bg_color())
                    .corner_radii(self.theme.corner_radii)
                    .destructive(self.theme.destructive.base.color)
                    .spacing(self.theme.spacing)
                    .success(self.theme.success.base.color)
                    .warning(self.theme.warning.base.color)
                    .neutral_tint(self.theme.palette.neutral_5.color)
                    .text_tint(self.theme.background.on.color);
                self.theme_builder.gaps = self.theme.gaps;
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

                let (task, needs_update) = self.update_color_picker(
                    &u,
                    ContextView::AccentWindowHint,
                    fl!("window-hint-accent").into(),
                );

                tasks.push(task);
                tasks.push(self.accent_window_hint.update::<app::Message>(u));

                if needs_update {
                    let Some(config) = self.theme_builder_config.as_ref() else {
                        return cosmic::Task::batch(tasks);
                    };

                    let color = self.accent_window_hint.get_applied_color().map(Srgb::from);

                    needs_build = self
                        .theme_builder
                        .set_window_hint(config, color)
                        .unwrap_or_default();
                }
            }

            Message::IconTheme(id) => {
                if let Some(theme) = self.icon_themes.get(id).cloned() {
                    self.icon_theme_active = Some(id);

                    if let Some(ref config) = self.tk_config {
                        _ = config.set::<String>("icon_theme", theme.id);
                    }

                    tokio::spawn(icon_themes::set_gnome_icon_theme(theme.name));
                }
            }

            Message::WindowHintSize(active_hint) => {
                needs_sync = true;

                let Some(config) = self.theme_builder_config.as_ref() else {
                    return Task::none();
                };

                if self
                    .theme_builder
                    .set_active_hint(config, active_hint)
                    .unwrap_or_default()
                {
                    // Update the gap if it's less than the active hint
                    if active_hint > self.theme_builder.gaps.1 {
                        let mut gaps = self.theme_builder.gaps;
                        gaps.1 = active_hint;
                        if self
                            .theme_builder
                            .set_gaps(config, gaps)
                            .unwrap_or_default()
                        {
                            self.theme_config_write("gaps", gaps);
                        }
                    }

                    // Update the active_hint in the config
                    self.theme_config_write("active_hint", active_hint);
                }
            }

            Message::GapSize(gap) => {
                needs_sync = true;

                let Some(config) = self.theme_builder_config.as_ref() else {
                    return Task::none();
                };

                let mut gaps = self.theme_builder.gaps;

                // Ensure that the gap is never less than what the active hint size is.
                gaps.1 = if gap < self.theme_builder.active_hint {
                    self.theme_builder.active_hint
                } else {
                    gap
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
                let (task, needs_update) = self.update_color_picker(
                    &u,
                    ContextView::ApplicationBackground,
                    fl!("app-background").into(),
                );

                tasks.push(task);
                tasks.push(self.application_background.update::<app::Message>(u));

                if needs_update {
                    let Some(config) = self.theme_builder_config.as_ref() else {
                        return cosmic::Task::batch(tasks);
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
                let (task, needs_update) = self.update_color_picker(
                    &u,
                    ContextView::ContainerBackground,
                    fl!("container-background").into(),
                );

                tasks.push(task);
                tasks.push(self.container_background.update::<app::Message>(u));

                if needs_update {
                    let Some(config) = self.theme_builder_config.as_ref() else {
                        return cosmic::Task::batch(tasks);
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
                let (task, needs_update) = self.update_color_picker(
                    &u,
                    ContextView::CustomAccent,
                    fl!("accent-color").into(),
                );

                tasks.push(task);
                tasks.push(self.custom_accent.update::<app::Message>(u));

                if needs_update {
                    let Some(config) = self.theme_builder_config.as_ref() else {
                        return cosmic::Task::batch(tasks);
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
                let (task, needs_update) = self.update_color_picker(
                    &u,
                    ContextView::InterfaceText,
                    fl!("text-tint").into(),
                );

                tasks.push(task);
                tasks.push(self.interface_text.update::<app::Message>(u));

                if needs_update {
                    let Some(config) = self.theme_builder_config.as_ref() else {
                        return cosmic::Task::batch(tasks);
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
                let (task, needs_update) = self.update_color_picker(
                    &u,
                    ContextView::ControlComponent,
                    fl!("control-tint").into(),
                );

                tasks.push(task);
                tasks.push(self.control_component.update::<app::Message>(u));

                if needs_update {
                    let Some(config) = self.theme_builder_config.as_ref() else {
                        return cosmic::Task::batch(tasks);
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
                    return Task::none();
                };

                let radii = self.roundness.into();

                if self
                    .theme_builder
                    .set_corner_radii(config, radii)
                    .unwrap_or_default()
                {
                    self.theme_config_write("corner_radii", radii);
                }

                #[cfg(feature = "wayland")]
                tokio::task::spawn(async move {
                    Self::update_panel_radii(r);
                });
            }

            Message::Density(density) => {
                needs_sync = true;

                if let Some(config) = self.tk_config.as_mut() {
                    _ = config.set("interface_density", density);
                    _ = config.set("header_size", density);
                }

                let Some(config) = self.theme_builder_config.as_ref() else {
                    return Task::none();
                };

                let spacing = density.into();

                if self
                    .theme_builder
                    .set_spacing(config, spacing)
                    .unwrap_or_default()
                {
                    self.theme_config_write("spacing", spacing);
                }

                #[cfg(feature = "wayland")]
                tokio::task::spawn(async move {
                    Self::update_panel_spacing(density);
                });
            }

            Message::Entered((icon_themes, icon_handles)) => {
                let active_icon_theme = cosmic::config::icon_theme();

                // Set the icon themes, and define the active icon theme.
                self.icon_themes = icon_themes;
                self.icon_theme_active = self
                    .icon_themes
                    .iter()
                    .position(|theme| theme.id == active_icon_theme);
                self.icon_handles = icon_handles;
            }

            Message::Left => {
                tasks.push(cosmic::task::message(app::Message::SetTheme(
                    cosmic::theme::system_preference(),
                )));
            }

            Message::PaletteAccent(c) => {
                let Some(config) = self.theme_builder_config.as_ref() else {
                    return Task::none();
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
                if let Some(config) = self.tk_config.as_mut() {
                    _ = config.set("interface_density", Density::Standard);
                    _ = config.set("header_size", Density::Standard);
                }

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
                #[cfg(feature = "wayland")]
                tokio::task::spawn(async move {
                    Self::update_panel_radii(r);
                    Self::update_panel_spacing(Density::Standard);
                });

                self.reload_theme_mode();
            }

            #[cfg(feature = "xdg-portal")]
            Message::StartImport => {
                tasks.push(cosmic::task::future(async move {
                    let res = file_chooser::open::Dialog::new()
                        .modal(true)
                        .filter(FileFilter::glob(FileFilter::new("ron"), "*.ron"))
                        .open_file()
                        .await;

                    if let Ok(f) = res {
                        Message::ImportFile(OpenResponse(Arc::new(f)))
                    } else {
                        // TODO Error toast?
                        tracing::error!("failed to select a file for importing a custom theme.");
                        Message::ImportError
                    }
                }));
            }

            #[cfg(feature = "xdg-portal")]
            Message::StartExport => {
                let is_dark = self.theme_mode.is_dark;
                let name = format!("{}.ron", if is_dark { fl!("dark") } else { fl!("light") });

                tasks.push(cosmic::task::future(async move {
                    let res = file_chooser::save::Dialog::new()
                        .modal(true)
                        .file_name(name)
                        .save_file()
                        .await;

                    if let Ok(f) = res {
                        Message::ExportFile(SaveResponse(Arc::new(f)))
                    } else {
                        // TODO Error toast?
                        tracing::error!("failed to select a file for importing a custom theme.");
                        Message::ExportError
                    }
                }));
            }

            #[cfg(feature = "xdg-portal")]
            Message::ImportFile(f) => {
                let path_res =
                    f.0.0
                        .uris()
                        .first()
                        .filter(|f| f.scheme() == "file")
                        .and_then(|f| f.to_file_path().ok());

                let Some(path) = path_res else {
                    return Task::none();
                };

                tasks.push(cosmic::task::future(async move {
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

            #[cfg(feature = "xdg-portal")]
            Message::ExportFile(f) => {
                let path_res =
                    f.0.0
                        .uris()
                        .first()
                        .filter(|f| f.scheme() == "file")
                        .and_then(|f| f.to_file_path().ok());

                let Some(path) = path_res else {
                    return Task::none();
                };

                let theme_builder = self.theme_builder.clone();

                tasks.push(cosmic::task::future(async move {
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
            #[cfg(feature = "xdg-portal")]
            Message::ExportError | Message::ImportError => return Task::none(),

            #[cfg(feature = "xdg-portal")]
            Message::ExportSuccess => {
                tracing::trace!("Export successful");
            }

            #[cfg(feature = "xdg-portal")]
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
                    return Task::none();
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
                if let Some(config) = self.tk_config.as_ref() {
                    _ = config.set("apply_theme_global", enabled);
                } else {
                    tracing::error!(
                        "Failed to apply theme to GNOME config because the CosmicTK config does not exist."
                    );
                }

                return Task::none();
            }

            Message::IconsAndToolkit => {
                self.context_view = Some(ContextView::IconsAndToolkit);
                let mut tasks = Vec::new();
                tasks.push(cosmic::task::message(
                    crate::app::Message::OpenContextDrawer(self.entity, "".into()),
                ));
                if !self.icons_fetched {
                    self.icons_fetched = true;
                    let (task, handle) = cosmic::task::future(icon_themes::fetch()).abortable();
                    self.icon_fetch_handle = Some(handle);
                    tasks.push(task);
                }
                return Task::batch(tasks);
            }

            Message::Daytime(day_time) => {
                self.day_time = day_time;
                return Task::none();
            }
        }

        // If the theme builder changed, write a new theme to disk on a background thread.
        if needs_build {
            self.update_accent_palette();
            let theme_builder = self.theme_builder.clone();
            let is_dark = self.theme_mode.is_dark;
            let current_theme = self.theme.clone();

            tasks.push(cosmic::task::future(async move {
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

                    Message::NewTheme(Box::new(new_theme)).into()
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

        cosmic::Task::batch(tasks)
    }

    fn update_accent_palette(&mut self) {
        let palette = self.theme_builder.palette.as_ref();
        self.accent_palette.theme = vec![
            palette.accent_blue,
            palette.accent_indigo,
            palette.accent_purple,
            palette.accent_pink,
            palette.accent_red,
            palette.accent_orange,
            palette.accent_yellow,
            palette.accent_green,
            palette.accent_warm_grey,
        ];
    }

    fn reload_theme_mode(&mut self) {
        let entity = self.entity;
        let font_config = std::mem::take(&mut self.font_config);
        let icon_themes = std::mem::take(&mut self.icon_themes);
        let icon_handles = std::mem::take(&mut self.icon_handles);
        let icon_theme_active = self.icon_theme_active.take();
        let day_time = self.day_time;

        *self = Self::from((
            self.settings_config.clone(),
            self.theme_mode_config.take(),
            self.theme_mode,
            std::mem::take(&mut self.accent_palette),
        ));

        self.update_accent_palette();

        self.entity = entity;
        self.day_time = day_time;
        self.icon_themes = icon_themes;
        self.icon_handles = icon_handles;
        self.icon_theme_active = icon_theme_active;
        self.font_config = font_config;
    }

    fn update_color_picker(
        &mut self,
        message: &ColorPickerUpdate,
        context_view: ContextView,
        context_title: Cow<'static, str>,
    ) -> (Task<app::Message>, bool) {
        let mut needs_update = false;

        let task = match message {
            ColorPickerUpdate::AppliedColor | ColorPickerUpdate::Reset => {
                needs_update = true;
                cosmic::task::message(crate::pages::Message::CloseContextDrawer)
            }

            ColorPickerUpdate::ActionFinished => {
                needs_update = true;
                Task::none()
            }

            ColorPickerUpdate::Cancel => {
                cosmic::task::message(crate::pages::Message::CloseContextDrawer)
            }

            ColorPickerUpdate::ToggleColorPicker => {
                self.context_view = Some(context_view);
                cosmic::task::message(crate::app::Message::OpenContextDrawer(
                    self.entity,
                    context_title,
                ))
            }

            _ => Task::none(),
        };

        (task, needs_update)
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

        if theme_builder.spacing != current_theme_builder.spacing {
            if let Err(err) =
                theme_builder.set_spacing(&other_builder_config, current_theme_builder.spacing)
            {
                tracing::error!(?err, "Error setting spacing");
            }

            if let Err(err) = theme.set_spacing(&other_theme_config, current_theme_builder.spacing)
            {
                tracing::error!(?err, "Error setting spacing");
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
    #[cfg(feature = "wayland")]
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

    #[cfg(feature = "wayland")]
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
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

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

    #[cfg(feature = "xdg-portal")]
    fn header_view(&self) -> Option<Element<'_, crate::pages::Message>> {
        let content = row::with_capacity(2)
            .spacing(self.theme_builder.spacing.space_xxs)
            .push(button::standard(fl!("import")).on_press(Message::StartImport))
            .push(button::standard(fl!("export")).on_press(Message::StartExport))
            .apply(container)
            .width(Length::Fill)
            .align_x(Alignment::End)
            .apply(Element::from)
            .map(crate::pages::Message::Appearance);

        Some(content)
    }

    fn info(&self) -> page::Info {
        page::Info::new("appearance", "preferences-appearance-symbolic")
            .title(fl!("appearance"))
            .description(fl!("appearance", "desc"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        let (task, handle) = cosmic::task::batch(vec![
            // Load icon themes
            // cosmic::task::future(icon_themes::fetch()).map(crate::pages::Message::Appearance),
            // Load font families
            cosmic::task::future(async move {
                let (mono, interface) = font_config::load_font_families();
                Message::FontConfig(font_config::Message::LoadedFonts(mono, interface))
            })
            .map(crate::pages::Message::Appearance),
        ])
        .abortable();

        self.on_enter_handle = Some(handle);
        task
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }
        if let Some(handle) = self.icon_fetch_handle.take() {
            handle.abort();
        }
        cosmic::task::message(crate::pages::Message::Appearance(Message::Left))
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
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

            ContextView::InterfaceText => color_picker_context_view(
                None,
                RESET_TO_DEFAULT.as_str().into(),
                Message::InterfaceText,
                &self.interface_text,
            )
            .map(crate::pages::Message::Appearance),

            ContextView::SystemFont => {
                let filter = if self.font_search.is_empty() {
                    &self.font_config.interface_font_families
                } else {
                    &self.font_filter
                };

                let current_font = cosmic::config::interface_font();

                font_config::selection_context(
                    filter,
                    &self.font_search,
                    current_font.family.as_str(),
                    true,
                )
                .map(crate::pages::Message::Appearance)
            }

            ContextView::MonospaceFont => {
                let filter = if self.font_search.is_empty() {
                    &self.font_config.monospace_font_families
                } else {
                    &self.font_filter
                };

                let current_font = cosmic::config::monospace_font();

                font_config::selection_context(
                    filter,
                    &self.font_search,
                    current_font.family.as_str(),
                    false,
                )
                .map(crate::pages::Message::Appearance)
            }

            ContextView::IconsAndToolkit => self.icons_and_toolkit(),
        };

        Some(cosmic::app::context_drawer(
            view,
            crate::pages::Message::CloseContextDrawer,
        ))
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
            let Spacing { space_xxs, .. } = cosmic::theme::active().cosmic().spacing;

            let descriptions = &section.descriptions;
            let palette = &page.theme_builder.palette.as_ref();
            let cur_accent = page
                .theme_builder
                .accent
                .map_or(palette.accent_blue, Srgba::from);

            let accent_palette_values = match (
                page.theme_mode.is_dark,
                page.accent_palette.dark.as_ref(),
                page.accent_palette.light.as_ref(),
            ) {
                (true, Some(dark_palette), _) => &dark_palette,
                (false, _, Some(light_palette)) => &light_palette,
                _ => &page.accent_palette.theme,
            };

            let mut accent_palette_row =
                cosmic::widget::row::with_capacity(accent_palette_values.len());

            for &color in accent_palette_values {
                accent_palette_row = accent_palette_row.push(color_button(
                    Some(Message::PaletteAccent(color.into())),
                    color.into(),
                    cur_accent == color,
                    48,
                    48,
                ));
            }

            let accent_color_palette = cosmic::iced::widget::column![
                text::body(&descriptions[accent_color]),
                scrollable(
                    accent_palette_row
                        .push(if let Some(c) = page.custom_accent.get_applied_color() {
                            container(color_button(
                                Some(Message::CustomAccent(ColorPickerUpdate::ToggleColorPicker)),
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
                        })
                        .padding([0, 0, 16, 0])
                        .spacing(16)
                )
                .direction(Direction::Horizontal(Scrollbar::new()))
            ]
            .padding([16, 0, 0, 0])
            .spacing(space_xxs);

            let mut section = settings::section()
                .title(&section.title)
                .add(
                    container(
                        cosmic::iced::widget::row![
                            cosmic::iced::widget::column![
                                button::custom(
                                    icon(dark_mode_illustration.clone())
                                        .width(Length::Fixed(191.0))
                                        .height(Length::Fixed(100.0))
                                )
                                .class(button::ButtonClass::Image)
                                .padding([8, 0])
                                .selected(page.theme_mode.is_dark)
                                .on_press(Message::DarkMode(true)),
                                text::body(&descriptions[dark])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_x(Alignment::Center),
                            cosmic::iced::widget::column![
                                button::custom(
                                    icon(light_mode_illustration.clone(),)
                                        .width(Length::Fixed(191.0))
                                        .height(Length::Fixed(100.0))
                                )
                                .class(button::ButtonClass::Image)
                                .selected(!page.theme_mode.is_dark)
                                .padding([8, 0])
                                .on_press(Message::DarkMode(false)),
                                text::body(&descriptions[light])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_x(Alignment::Center)
                        ]
                        .spacing(8)
                        .width(Length::Fixed(478.0))
                        .align_y(Alignment::Center),
                    )
                    .center_x(Length::Fill),
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
                .add(accent_color_palette)
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

    fn style_container() -> cosmic::theme::Container<'static> {
        cosmic::theme::Container::custom(|theme| {
            let mut background = theme.cosmic().palette.neutral_9;
            background.alpha = 0.1;
            container::Style {
                background: Some(cosmic::iced::Background::Color(background.into())),
                border: cosmic::iced::Border {
                    radius: theme.cosmic().radius_s().into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
    }

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
                                .class(button::ButtonClass::Image)
                                .padding(0)
                                .on_press(Message::Roundness(Roundness::Round))
                                .apply(container)
                                .width(Length::Fixed(191.0))
                                .class(style_container()),
                                text::body(&descriptions[round])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_x(Alignment::Center),
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
                                .class(button::ButtonClass::Image)
                                .padding(0)
                                .on_press(Message::Roundness(Roundness::SlightlyRound))
                                .apply(container)
                                .width(Length::Fixed(191.0))
                                .class(style_container()),
                                text::body(&descriptions[slightly_round])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_x(Alignment::Center),
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
                                .class(button::ButtonClass::Image)
                                .padding(0)
                                .on_press(Message::Roundness(Roundness::Square))
                                .apply(container)
                                .width(Length::Fixed(191.0))
                                .class(style_container()),
                                text::body(&descriptions[square])
                            ]
                            .spacing(8)
                            .align_x(Alignment::Center)
                            .width(Length::FillPortion(1))
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                    )
                    .center_x(Length::Fill),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

pub fn interface_density() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        comfortable = fl!("interface-density", "comfortable");
        compact = fl!("interface-density", "compact");
        spacious = fl!("interface-density", "spacious");
    });

    Section::default()
        .title(fl!("interface-density"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            let descriptions = &section.descriptions;

            let density = cosmic::config::interface_density();

            settings::section()
                .title(&section.title)
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[compact]),
                        Density::Compact,
                        Some(density),
                        Message::Density,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[comfortable]),
                        Density::Standard,
                        Some(density),
                        Message::Density,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[spacious]),
                        Density::Spacious,
                        Some(density),
                        Message::Density,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
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
                    widget::spin_button(
                        page.theme_builder.active_hint.to_string(),
                        page.theme_builder.active_hint,
                        1,
                        0,
                        64,
                        Message::WindowHintSize,
                    ),
                ))
                .add(
                    settings::item::builder(&descriptions[gaps]).control(widget::spin_button(
                        page.theme_builder.gaps.1.to_string(),
                        page.theme_builder.gaps.1,
                        1,
                        page.theme_builder.active_hint,
                        500,
                        Message::GapSize,
                    )),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

pub fn experimental() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        interface_font_txt = fl!("interface-font");
        monospace_font_txt = fl!("monospace-font");
        icons_and_toolkit_txt = fl!("icons-and-toolkit");
    });

    Section::default()
        .title(fl!("experimental-settings"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            let descriptions = &section.descriptions;

            let system_font = crate::widget::go_next_with_item(
                &descriptions[interface_font_txt],
                text::body(cosmic::config::interface_font().family),
                Message::DisplaySystemFont,
            );

            let mono_font = crate::widget::go_next_with_item(
                &descriptions[monospace_font_txt],
                text::body(cosmic::config::monospace_font().family),
                Message::DisplayMonoFont,
            );

            let icons_and_toolkit = crate::widget::go_next_item(
                &descriptions[icons_and_toolkit_txt],
                Message::IconsAndToolkit,
            );

            settings::section()
                .title(&*section.title)
                .add(system_font)
                .add(mono_font)
                .add(icons_and_toolkit)
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
                horizontal_space().width(1).apply(Element::from)
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
    .class(button::ButtonClass::Image)
    .on_press_maybe(on_press)
    .width(Length::Fixed(f32::from(width)))
    .height(Length::Fixed(f32::from(height)))
    .into()
}
