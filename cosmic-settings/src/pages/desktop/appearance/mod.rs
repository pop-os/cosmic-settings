// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod drawer;
pub mod font_config;
pub mod icon_themes;
pub mod mode_and_colors;
pub mod style;
pub mod theme_manager;

use std::sync::Arc;

use cosmic::app::ContextDrawer;
//TODO: use embedded cosmic-files for portability
use cosmic::config::CosmicTk;
use cosmic::cosmic_config::{Config, ConfigSet, CosmicConfigEntry};
use cosmic::cosmic_theme::palette::{FromColor, Hsv, Srgb};
use cosmic::cosmic_theme::{CornerRadii, Density, ThemeBuilder};
#[cfg(feature = "xdg-portal")]
use cosmic::dialog::file_chooser::{self, FileFilter};
use cosmic::iced_core::{Alignment, Length};
use cosmic::widget::{
    button, color_picker::ColorPickerUpdate, container, horizontal_space, radio, row, settings,
    text,
};
use cosmic::{Apply, Element, Task, widget};
#[cfg(feature = "wayland")]
use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use ron::ser::PrettyConfig;
use slab::Slab;
use slotmap::{Key, SlotMap};

use crate::app;

#[derive(Clone, Copy, Debug)]
pub enum ContextView {
    AccentWindowHint,
    ApplicationBackground,
    ContainerBackground,
    ControlComponent,
    ShadowAndCorners,
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
    can_reset: bool,
    context_view: Option<ContextView>,
    drawer: drawer::Content,
    roundness: Roundness,
    density: Density,

    theme_manager: theme_manager::Manager,

    tk_config: Option<Config>,
    day_time: bool,
}

impl Default for Page {
    fn default() -> Self {
        theme_manager::Manager::default().into()
    }
}

impl From<theme_manager::Manager> for Page {
    fn from(theme_manager: theme_manager::Manager) -> Self {
        let tk_config = CosmicTk::config().ok();
        let theme_mode = theme_manager.mode();
        let theme_builder = theme_manager.builder().clone();

        Self {
            entity: page::Entity::null(),
            on_enter_handle: None,
            can_reset: if theme_mode.is_dark {
                theme_builder == ThemeBuilder::dark()
            } else {
                theme_builder == ThemeBuilder::light()
            },
            context_view: None,
            drawer: drawer::Content::from(&theme_manager),
            roundness: theme_builder.corner_radii.into(),
            density: cosmic::config::interface_density(),
            theme_manager,
            tk_config,
            day_time: true,
        }
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
    Autoswitch(bool),
    DarkMode(bool),
    Density(Density),

    DrawerOpen(ContextView),
    DrawerColor(ColorPickerUpdate),
    DrawerCorners(drawer::CornerMessage),
    DrawerFont(drawer::FontMessage),
    DrawerIcon(drawer::IconMessage),

    #[cfg(feature = "xdg-portal")]
    ExportError,
    #[cfg(feature = "xdg-portal")]
    ExportFile(SaveResponse),
    #[cfg(feature = "xdg-portal")]
    ExportSuccess,

    GapSize(u32),
    #[cfg(feature = "xdg-portal")]
    ImportError,
    #[cfg(feature = "xdg-portal")]
    ImportFile(OpenResponse),
    #[cfg(feature = "xdg-portal")]
    ImportSuccess(Box<ThemeBuilder>),
    Left,
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
    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) -> Task<app::Message> {
        let mut tasks = Vec::new();
        let mut theme_staged: Option<theme_manager::ThemeStaged> = None;

        match message {
            Message::Autoswitch(enabled) => self.theme_manager.auto_switch(enabled),

            Message::DarkMode(enabled) => {
                if let Err(err) = self.theme_manager.dark_mode(enabled) {
                    tracing::error!(?err, "Error setting dark mode");
                }

                self.drawer.reset(&self.theme_manager);
                theme_staged = Some(theme_manager::ThemeStaged::Current);
            }

            Message::DrawerOpen(context_view) => {
                self.context_view = Some(context_view);
                tasks.push(cosmic::task::message(
                    crate::app::Message::OpenContextDrawer(self.entity),
                ));
                tasks.push(self.drawer.on_open(&context_view));
            }

            Message::DrawerFont(message) => {
                tasks.push(self.drawer.update_font(message, self.context_view.as_ref()));
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

            Message::DrawerIcon(message) => {
                if let Some(context_view) = self.context_view.as_ref() {
                    tasks.push(self.drawer.update_icon(message, context_view));
                }
            }

            Message::DrawerCorners(message) => {
                if let Some(context_view) = self.context_view.as_ref() {
                    tasks.push(self.drawer.update_shadow_and_corners(message, context_view));
                }
            }

            Message::WindowHintSize(active_hint) => {
                self.theme_manager.set_active_hint(active_hint);
            }
            Message::GapSize(gap) => {
                self.theme_manager.set_gap_size(gap);
            }

            Message::Roundness(r) => {
                self.roundness = r;

                let radii = self.roundness.into();
                theme_staged = self.theme_manager.set_corner_radii(radii);

                #[cfg(feature = "wayland")]
                tokio::task::spawn(async move {
                    Self::update_panel_radii(r);
                    Self::update_dock_padding(r);
                });
            }

            Message::Density(density) => {
                self.density = density;
                theme_staged = self.theme_manager.set_spacing(density.into());

                if let Some(config) = self.tk_config.as_mut() {
                    _ = config.set("interface_density", density);
                    _ = config.set("header_size", density);
                }

                #[cfg(feature = "wayland")]
                tokio::task::spawn(async move {
                    Self::update_panel_spacing(density);
                });
            }

            Message::Left => {}

            Message::PaletteAccent(c) => {
                theme_staged = self
                    .theme_manager
                    .selected_customizer_mut()
                    .set_accent(Some(Srgb::from(c)));
            }

            Message::Reset => {
                let theme_type = self.theme_manager.cosmic_theme().theme_type;

                let builder = if theme_type.is_dark() {
                    ThemeBuilder::dark()
                } else {
                    ThemeBuilder::light()
                };
                self.theme_manager.set_active_hint(builder.active_hint);

                self.theme_manager
                    .selected_customizer_mut()
                    .set_builder(builder.clone())
                    .set_theme(builder.build())
                    .apply_theme();

                if let Some(config) = self.tk_config.as_mut() {
                    _ = config.set("interface_density", Density::Standard);
                    _ = config.set("header_size", Density::Standard);
                }

                let r = self.roundness;
                self.drawer.reset(&self.theme_manager);

                tasks.push(cosmic::task::future(async move {
                    #[cfg(feature = "wayland")]
                    {
                        Self::update_panel_radii(r);
                        Self::update_panel_spacing(Density::Standard);
                    }

                    app::Message::SetTheme(cosmic::theme::system_preference())
                }));
            }

            #[cfg(feature = "xdg-portal")]
            Message::StartImport => {
                tasks.push(cosmic::task::future(async move {
                    let res = file_chooser::open::Dialog::new()
                        .modal(true)
                        .filter(FileFilter::glob(FileFilter::new("ron"), "*.ron"))
                        .open_file()
                        .await;

                    match res {
                        Ok(f) => Message::ImportFile(OpenResponse(Arc::new(f))),
                        Err(why) => {
                            tracing::error!(
                                ?why,
                                "failed to select a file for importing a custom theme."
                            );
                            Message::ImportError
                        }
                    }
                }));
            }

            #[cfg(feature = "xdg-portal")]
            Message::StartExport => {
                let is_dark = self.theme_manager.mode().is_dark;
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

                return cosmic::Task::batch(tasks);
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

                let theme_builder = self.theme_manager.builder().clone();

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

                return cosmic::Task::batch(tasks);
            }

            // TODO: error message toast?
            #[cfg(feature = "xdg-portal")]
            Message::ExportError | Message::ImportError => return Task::none(),

            #[cfg(feature = "xdg-portal")]
            Message::ExportSuccess => {
                tracing::trace!("Export successful");
                return Task::none();
            }

            #[cfg(feature = "xdg-portal")]
            Message::ImportSuccess(builder) => {
                tracing::trace!("Import successful");
                let new_is_dark = builder.palette.is_dark();
                if new_is_dark != self.theme_manager.mode().is_dark
                    && let Err(err) = self.theme_manager.dark_mode(new_is_dark)
                {
                    tracing::error!(?err, "Error setting dark mode");
                }

                self.theme_manager
                    .selected_customizer_mut()
                    .set_builder(*builder.clone())
                    .set_theme(builder.build())
                    .apply_builder()
                    .apply_theme();

                self.drawer.reset(&self.theme_manager);

                return cosmic::task::future(async move {
                    app::Message::SetTheme(cosmic::theme::system_preference())
                });
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

                _ = self.drawer.accent_window_hint.update::<app::Message>(
                    ColorPickerUpdate::ActiveColor(Hsv::from_color(
                        window_hint.unwrap_or_default(),
                    )),
                );

                _ = self
                    .drawer
                    .accent_window_hint
                    .update::<app::Message>(ColorPickerUpdate::AppliedColor);

                theme_staged = self
                    .theme_manager
                    .selected_customizer_mut()
                    .set_window_hint(window_hint);
            }

            Message::Daytime(day_time) => {
                self.day_time = day_time;
                return Task::none();
            }
        }

        let mut tasks = cosmic::Task::batch(tasks);

        if let Some(stage) = theme_staged {
            tasks = tasks.chain(self.theme_manager.build_theme(stage))
        }

        self.can_reset = if self.theme_manager.mode().is_dark {
            *self.theme_manager.builder() != ThemeBuilder::dark()
        } else {
            *self.theme_manager.builder() != ThemeBuilder::light()
        };

        tasks
    }

    // TODO: cache panel and dock configs so that they needn't be re-read
    #[cfg(feature = "wayland")]
    pub fn update_panel_radii(roundness: Roundness) {
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

        if let Some(panel_config_helper) = panel_config_helper.as_ref()
            && let Some(panel_config) = panel_config.as_mut()
        {
            let radii = if panel_config.anchor_gap {
                let cornder_radii: CornerRadii = roundness.into();
                cornder_radii.radius_xl[0] as u32
            } else if matches!(roundness, Roundness::Round) && !panel_config.expand_to_edges {
                12
            } else {
                0
            };

            if let Err(why) = panel_config.set_border_radius(panel_config_helper, radii) {
                tracing::error!(?why, "Error updating panel corner radii");
            }
        }

        if let Some(dock_config_helper) = dock_config_helper.as_ref()
            && let Some(dock_config) = dock_config.as_mut()
        {
            let radii = if dock_config.anchor_gap {
                let cornder_radii: CornerRadii = roundness.into();
                cornder_radii.radius_xl[0] as u32
            } else if matches!(roundness, Roundness::Round) && !dock_config.expand_to_edges {
                12
            } else {
                0
            };

            if let Err(why) = dock_config.set_border_radius(dock_config_helper, radii) {
                tracing::error!(?why, "Error updating dock corner radii");
            }
        }
    }

    pub fn update_dock_padding(roundness: Roundness) {
        let dock_config_helper = CosmicPanelConfig::cosmic_config("Dock").ok();

        let mut dock_config = dock_config_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "Dock").then_some(panel_config)
        });

        if let Some(dock_config_helper) = dock_config_helper.as_ref()
            && let Some(dock_config) = dock_config.as_mut()
        {
            let padding = match roundness {
                Roundness::Round => 4,
                Roundness::SlightlyRound => 4,
                Roundness::Square => 0,
            };

            if let Err(why) = dock_config.set_padding(dock_config_helper, padding) {
                tracing::error!(?why, "Error updating dock padding");
            }
        }
    }

    // TODO: cache panel and dock configs so that they needn't be re-read
    #[cfg(feature = "wayland")]
    pub fn update_panel_spacing(density: Density) {
        let spacing: cosmic::cosmic_theme::Spacing = density.into();
        let space_none = spacing.space_none;
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

        if let Some(panel_config_helper) = panel_config_helper.as_ref()
            && let Some(panel_config) = panel_config.as_mut()
        {
            let update = panel_config.set_spacing(panel_config_helper, space_none as u32);
            if let Err(err) = update {
                tracing::error!(?err, "Error updating panel spacing");
            }
        };

        if let Some(dock_config_helper) = dock_config_helper.as_ref()
            && let Some(dock_config) = dock_config.as_mut()
        {
            let update = dock_config.set_spacing(dock_config_helper, space_none as u32);
            if let Err(err) = update {
                tracing::error!(?err, "Error updating dock spacing");
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
            sections.insert(mode_and_colors::section()),
            sections.insert(style::section()),
            sections.insert(interface_density()),
            sections.insert(window_management()),
            sections.insert(experimental()),
            sections.insert(reset_button()),
        ])
    }

    #[cfg(feature = "xdg-portal")]
    fn header_view(&self) -> Option<Element<'_, crate::pages::Message>> {
        let content = row::with_capacity(2)
            .spacing(self.theme_manager.builder().spacing.space_xxs)
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
                let (interface, mono) = font_config::load_font_families();
                Message::DrawerFont(drawer::FontMessage::FontLoaded(interface, mono))
            })
            .map(crate::pages::Message::Appearance),
        ])
        .abortable();

        self.on_enter_handle = Some(handle);
        task
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        let mut tasks = Vec::new();
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }
        tasks.push(self.drawer.on_leave());
        tasks.push(cosmic::task::message(crate::pages::Message::Appearance(
            Message::Left,
        )));

        cosmic::task::batch(tasks)
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        self.drawer.context_drawer(self.context_view)
    }
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
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[compact]),
                        Density::Compact,
                        Some(page.density),
                        Message::Density,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[comfortable]),
                        Density::Standard,
                        Some(page.density),
                        Message::Density,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[spacious]),
                        Density::Spacious,
                        Some(page.density),
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
                        page.theme_manager.builder().active_hint.to_string(),
                        "active hint",
                        page.theme_manager.builder().active_hint,
                        1,
                        0,
                        64,
                        Message::WindowHintSize,
                    ),
                ))
                .add(
                    settings::item::builder(&descriptions[gaps]).control(widget::spin_button(
                        page.theme_manager.builder().gaps.1.to_string(),
                        "gaps",
                        page.theme_manager.builder().gaps.1,
                        1,
                        page.theme_manager.builder().active_hint,
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
        shadow_and_corners_txt = fl!("shadow-and-corners");
    });

    Section::default()
        .title(fl!("experimental-settings"))
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

            let icons_and_toolkit = crate::widget::go_next_item(
                &descriptions[icons_and_toolkit_txt],
                Message::DrawerOpen(ContextView::IconsAndToolkit),
            );

            let shadow_and_corners = crate::widget::go_next_item(
                &descriptions[shadow_and_corners_txt],
                Message::DrawerOpen(ContextView::ShadowAndCorners),
            );

            settings::section()
                .title(&*section.title)
                .add(system_font)
                .add(mono_font)
                .add(icons_and_toolkit)
                .add(shadow_and_corners)
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
