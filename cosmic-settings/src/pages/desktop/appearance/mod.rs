// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod colors_and_style;
pub mod commands;
pub mod custom;
pub mod drawer;
pub mod font_config;
pub mod frosted;
pub mod icon_themes;
pub mod mode_and_auto;
pub mod style;
pub mod theme_bundles;
pub mod theme_manager;
pub mod wallpaper;

use cosmic::app::ContextDrawer;
//TODO: use embedded cosmic-files for portability
use cosmic::config::CosmicTk;
use cosmic::cosmic_config::{Config, ConfigSet, CosmicConfigEntry};
use cosmic::cosmic_theme::palette::{FromColor, Hsv, Srgb};
use cosmic::cosmic_theme::{CornerRadii, Density, Roundness, ThemeBuilder, ThemeMode};
#[cfg(feature = "xdg-portal")]
use cosmic::dialog::file_chooser::{self, FileFilter};
use cosmic::iced::Subscription;
use cosmic::widget::color_picker::ColorPickerUpdate;
use cosmic::widget::segmented_button::Model;
use cosmic::widget::text_input::focus;
use cosmic::widget::{segmented_button, settings, text};
use cosmic::{Apply, Element, Task, task, widget};
#[cfg(feature = "wayland")]
use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::{self as page, Section, section};
use slotmap::{Key, SlotMap};

use crate::app;
use crate::pages::desktop::appearance::font_config::FONT_SEARCH;
use crate::pages::desktop::appearance::frosted::Frosted;

#[derive(Clone, Copy, Debug)]
pub enum ContextView {
    AccentWindowHint,
    ApplicationBackground,
    ContainerBackground,
    ControlComponent,
    #[cfg(feature = "cosmic-comp-config")]
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
    context_view: Option<ContextView>,
    drawer: drawer::Content,
    roundness: Roundness,
    density: Density,
    mode_model: segmented_button::SingleSelectModel,

    frosted_mode: Frosted,
    frosted: u8,
    glass: f32,
    frosted_system_interface: bool,
    frosted_windows: bool,
    frosted_panel: bool,
    frosted_applets: bool,

    theme_manager: theme_manager::Manager,

    decks: Vec<theme_bundles::Deck>,

    import: Option<theme_bundles::Import>,

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

        let frosted = theme_manager.theme().frosted as u8;
        let glass = theme_manager.theme().alpha_map.guess_offset();
        let frosted_mode = frosted_mode_of(frosted, glass);
        let is_dark = theme_mode.is_dark;
        Self {
            entity: page::Entity::null(),
            on_enter_handle: None,
            context_view: None,
            drawer: drawer::Content::from(&theme_manager),
            roundness: theme_builder.corner_radii.into(),
            frosted_mode,
            density: cosmic::config::interface_density(),
            mode_model: Model::builder()
                .insert(|mut b| {
                    b = b.text(fl!("dark")).data(true);
                    if is_dark { b.activate() } else { b }
                })
                .insert(|mut b| {
                    b = b.text(fl!("light")).data(false);
                    if is_dark { b } else { b.activate() }
                })
                .build(),

            frosted,
            glass,
            frosted_system_interface: theme_manager.theme().frosted_system_interface,
            frosted_windows: theme_manager.theme().frosted_windows,
            frosted_panel: theme_manager.theme().frosted_panel,
            frosted_applets: theme_manager.theme().frosted_applets,

            theme_manager,
            decks: Vec::new(),
            import: None,
            tk_config,
            day_time: true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ActivateBundle(String),
    Autoswitch(bool),
    BundleActivated(String, Result<(), String>),
    BundleDeleted(String, Result<(), String>),
    BundleImport,
    BundleImported(Result<String, String>),
    Open(cosmic_theme_bundle_config::Mode),
    ExportTheme,
    ExportThemeFile(Option<std::path::PathBuf>),
    Exported(Result<String, String>),
    ImportCancel,
    ImportChooseFile,
    ImportConfirm,
    ImportFileChosen(Option<std::path::PathBuf>),
    ImportNameChanged(String),
    BundleWallpaper(
        Vec<(cosmic_theme_bundle_config::Mode, String)>,
        Option<cosmic::widget::image::Handle>,
    ),
    BundlesLoaded(Vec<theme_bundles::Deck>),
    DarkMode(segmented_button::Entity),
    DeleteBundle(String),
    Density(Density),
    ThemeModeUpdate(ThemeMode),

    DrawerOpen(ContextView),
    DrawerColor(ColorPickerUpdate),
    #[cfg(feature = "cosmic-comp-config")]
    DrawerCorners(drawer::CornerMessage),
    DrawerFont(drawer::FontMessage),
    DrawerIcon(drawer::IconMessage),

    FocusFontInput,
    GapSize(u32),
    Left,
    PaletteAccent(cosmic::iced::Color),
    Reset,
    Roundness(Roundness),
    Frosted(frosted::Frosted),
    UseDefaultWindowHint(bool),
    WindowHintSize(u32),
    Daytime(bool),
    Blur(u8),
    Glass(f32),
    FrostedPanel(bool),
    FrostedApplets(bool),
    FrostedSystemInterface(bool),
    FrostedWindows(bool),
    WallpaperPage,
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

impl Page {
    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) -> Task<app::Message> {
        let mut tasks = Vec::new();
        let mut theme_staged: Option<theme_manager::ThemeStaged> = None;

        match message {
            Message::ActivateBundle(slug) => {
                tasks.push(cosmic::task::future(async move {
                    let result = theme_bundles::activate(&slug);
                    Message::BundleActivated(slug, result)
                }));
            }

            Message::BundleActivated(slug, result) => match result {
                Ok(()) => {
                    tasks.push(self.reload_theme());
                    tasks.push(cosmic::task::future(async {
                        Message::BundlesLoaded(theme_bundles::load())
                    }));
                }
                Err(error) => {
                    tracing::error!(%slug, ?error, "failed to activate theme bundle");
                }
            },

            Message::BundlesLoaded(decks) => {
                for (slugs, path) in theme_bundles::wallpapers(&decks) {
                    tasks.push(cosmic::task::future(async move {
                        let handle = theme_bundles::wallpaper_handle(path).await;
                        Message::BundleWallpaper(slugs, handle)
                    }));
                }
                self.decks = decks;
            }

            Message::BundleWallpaper(slugs, handle) => {
                if let Some(handle) = handle {
                    theme_bundles::wallpaper_loaded(&mut self.decks, &slugs, handle);
                }
            }

            Message::DeleteBundle(slug) => {
                tasks.push(cosmic::task::future(async move {
                    let result = theme_bundles::delete(&slug);
                    Message::BundleDeleted(slug, result)
                }));
            }

            Message::BundleDeleted(slug, result) => {
                match result {
                    Ok(()) => {}
                    Err(error) => tracing::error!(%slug, ?error, "failed to remove theme bundle"),
                }
                tasks.push(cosmic::task::future(async {
                    Message::BundlesLoaded(theme_bundles::load())
                }));
            }

            Message::BundleImport => {
                self.import = Some(theme_bundles::Import::default());
            }

            Message::Open(_) => {}

            Message::ExportTheme | Message::ExportThemeFile(_) | Message::Exported(_) => {}

            Message::ImportNameChanged(name) => {
                if let Some(import) = self.import.as_mut() {
                    import.name = name;
                }
            }

            Message::ImportChooseFile => {
                #[cfg(feature = "xdg-portal")]
                tasks.push(cosmic::task::future(async {
                    let description = fl!("theme-bundles-archive");
                    let chosen = file_chooser::open::Dialog::new()
                        .modal(true)
                        .filter(FileFilter::glob(FileFilter::new(&description), "*.zip"))
                        .open_file()
                        .await;

                    match chosen {
                        Ok(file) => Message::ImportFileChosen(
                            file.0
                                .uris()
                                .first()
                                .filter(|uri| uri.scheme() == "file")
                                .and_then(|uri| uri.to_file_path().ok()),
                        ),
                        Err(why) => {
                            tracing::error!(?why, "failed to select a theme bundle archive");
                            Message::ImportFileChosen(None)
                        }
                    }
                }));
            }

            Message::ImportFileChosen(path) => {
                if let (Some(path), Some(import)) = (path, self.import.as_mut()) {
                    if import.name.is_empty()
                        && let Ok(name) = cosmic_theme_bundle_config::transfer::archive_name(&path)
                    {
                        import.name = name;
                    }
                    import.path = Some(path);
                }
            }

            Message::ImportConfirm => {
                let chosen = self.import.as_ref().and_then(|import| {
                    let path = import.path.clone()?;
                    let name = import.name.trim().to_owned();
                    Some((path, name))
                });
                if let Some((path, name)) = chosen {
                    self.import = None;
                    tasks.push(cosmic::task::future(async move {
                        Message::BundleImported(theme_bundles::import_archive(
                            &path,
                            (!name.is_empty()).then_some(name.as_str()),
                        ))
                    }));
                }
            }

            Message::ImportCancel => self.import = None,

            Message::BundleImported(result) => match result {
                Ok(_) => {
                    tasks.push(cosmic::task::future(async {
                        Message::BundlesLoaded(theme_bundles::load())
                    }));
                }
                Err(error) => tracing::error!(?error, "failed to read a theme bundle archive"),
            },

            Message::Autoswitch(enabled) => self.theme_manager.auto_switch(enabled),

            Message::DarkMode(enabled) => {
                self.mode_model.activate(enabled);
                let enabled = self.mode_model.active_data().copied().unwrap_or_default();
                if let Err(err) = self.theme_manager.dark_mode(enabled) {
                    tracing::error!(?err, "Error setting dark mode");
                }

                tasks.push(cosmic::task::future(async {
                    Message::BundlesLoaded(theme_bundles::load())
                }));

                self.drawer.reset(&self.theme_manager);
                self.drawer.sync_fonts(&self.theme_manager);
                theme_staged = Some(theme_manager::ThemeStaged::Current);
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

            #[cfg(feature = "cosmic-comp-config")]
            Message::DrawerCorners(message) => {
                if let Some(context_view) = self.context_view.as_ref() {
                    tasks.push(self.drawer.update_shadow_and_corners(message, context_view));
                }
            }

            Message::WindowHintSize(active_hint) => {
                self.theme_manager.set_active_hint(active_hint);
            }

            Message::Frosted(f) => {
                tasks.push(self.update_frosted_mode(f));
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
                    Self::update_dock_padding(r);
                    Self::update_panel_radii(r);
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

                self.roundness = builder.corner_radii.into();
                self.density = Density::Standard;

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
                self.drawer.sync_fonts(&self.theme_manager);

                tasks.push(cosmic::task::future(async move {
                    #[cfg(feature = "wayland")]
                    {
                        Self::update_panel_radii(r);
                        Self::update_panel_spacing(Density::Standard);
                    }

                    app::Message::SetTheme(cosmic::theme::system_preference())
                }));
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

            Message::ThemeModeUpdate(mode) => {
                let was_dark = self.theme_manager.mode().is_dark;
                let was_auto = self.theme_manager.mode().auto_switch;

                self.theme_manager.sync_mode(mode);

                // If auto-switch flipped while the page is open, keep the UI in sync.
                if was_dark != self.theme_manager.mode().is_dark
                    || was_auto != self.theme_manager.mode().auto_switch
                {
                    self.drawer.reset(&self.theme_manager);
                    self.drawer.sync_fonts(&self.theme_manager);
                    return cosmic::task::future(async {
                        Message::BundlesLoaded(theme_bundles::load())
                    });
                }
            }

            Message::FocusFontInput => {
                // retry until the widget is in the tree and focused or the dialog is removed.
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
            Message::Blur(b) => {
                let Ok(blur_strength) = b.try_into() else {
                    tracing::error!("Invalid blur strength: {b}");
                    return Task::none();
                };
                theme_staged = self.theme_manager.set_frosted(blur_strength).map(|_| {
                    self.drawer.reset(&self.theme_manager);
                    theme_manager::ThemeStaged::Current
                });

                self.update_blur(blur_strength as u8);
            }
            Message::Glass(glass) => {
                let mut alpha_map = cosmic::cosmic_theme::AlphaMap::default();
                let glass_offset_min = alpha_map.extremely_high_2;
                let glass_offset_max = 1.0 - alpha_map.extremely_low;
                let offset = (glass - 0.5) / 0.5
                    * if glass < 0.5 {
                        glass_offset_min
                    } else {
                        glass_offset_max
                    };
                alpha_map = alpha_map.offset(offset);

                // TODO optimize and apply to both light and dark themes
                theme_staged = self.theme_manager.set_glass(alpha_map).map(|_| {
                    self.drawer.reset(&self.theme_manager);
                    theme_manager::ThemeStaged::Current
                });

                self.update_glass(glass);
            }
            Message::FrostedSystemInterface(v) => {
                theme_staged = self.theme_manager.set_frosted_system_interface(v).map(|_| {
                    self.drawer.reset(&self.theme_manager);
                    theme_manager::ThemeStaged::Current
                });
                self.update_frosted_system_interface(v);
            }
            Message::FrostedWindows(v) => {
                theme_staged = self.theme_manager.set_frosted_windows(v).map(|_| {
                    self.drawer.reset(&self.theme_manager);
                    theme_manager::ThemeStaged::Current
                });
                self.update_frosted_windows(v);
            }
            Message::FrostedPanel(v) => {
                theme_staged = self.theme_manager.set_frosted_panel(v).map(|_| {
                    self.drawer.reset(&self.theme_manager);
                    theme_manager::ThemeStaged::Current
                });
                self.update_frosted_panel(v);
            }
            Message::FrostedApplets(v) => {
                theme_staged = self.theme_manager.set_frosted_applets(v).map(|_| {
                    self.drawer.reset(&self.theme_manager);
                    theme_manager::ThemeStaged::Current
                });
                self.update_frosted_applets(v);
            }

            Message::WallpaperPage => {}
        }

        let mut tasks = cosmic::Task::batch(tasks);

        if let Some(stage) = theme_staged {
            tasks = tasks.chain(self.theme_manager.build_theme(stage))
        }

        tasks
    }

    // TODO: cache panel and dock configs so that they needn't be re-read
    #[cfg(feature = "wayland")]
    fn load_panel_config(name: &str) -> Option<(Config, CosmicPanelConfig)> {
        let helper = CosmicPanelConfig::cosmic_config(name).ok()?;
        let config = CosmicPanelConfig::get_entry(&helper).ok()?;
        (config.name == name).then_some((helper, config))
    }

    #[cfg(feature = "wayland")]
    pub fn update_panel_radii(roundness: Roundness) {
        let corner_radii: CornerRadii = roundness.into();
        let radius = corner_radii.radius_xl[0] as u32;

        for name in ["Panel", "Dock"] {
            let Some((helper, mut config)) = Self::load_panel_config(name) else {
                continue;
            };

            let new_radius = if config.anchor_gap {
                radius
            } else if !config.expand_to_edges {
                radius.min(12)
            } else {
                0
            };

            if let Err(why) = config.set_border_radius(&helper, new_radius) {
                tracing::error!(?why, "Error updating {name} corner radii");
            }
        }
    }

    #[cfg(feature = "wayland")]
    pub fn update_dock_padding(roundness: Roundness) {
        let Some((helper, mut config)) = Self::load_panel_config("Dock") else {
            return;
        };

        let padding = match roundness {
            Roundness::Round | Roundness::SlightlyRound => 4,
            Roundness::Square => 0,
        };

        if let Err(why) = config.set_padding(&helper, padding) {
            tracing::error!(?why, "Error updating dock padding");
        }
    }

    #[cfg(feature = "wayland")]
    pub fn update_panel_spacing(density: Density) {
        let spacing: cosmic::cosmic_theme::Spacing = density.into();
        let space_none = spacing.space_none as u32;

        for name in ["Panel", "Dock"] {
            let Some((helper, mut config)) = Self::load_panel_config(name) else {
                continue;
            };
            if let Err(err) = config.set_spacing(&helper, space_none) {
                tracing::error!(?err, "Error updating {name} spacing");
            }
        }
    }

    pub fn update_frosted_system_interface(&mut self, v: bool) {
        self.frosted_system_interface = v;
    }

    pub fn update_frosted_windows(&mut self, v: bool) {
        self.frosted_windows = v;
    }

    pub fn update_frosted_panel(&mut self, v: bool) {
        self.frosted_panel = v;
    }

    pub fn update_frosted_applets(&mut self, v: bool) {
        self.frosted_applets = v;
    }

    pub fn update_blur(&mut self, v: u8) {
        self.frosted = v;
    }

    pub fn update_glass(&mut self, v: f32) {
        self.glass = v;
    }

    pub fn update_frosted_mode(&mut self, f: Frosted) -> Task<app::Message> {
        self.frosted_mode = f;
        match f {
            Frosted::Frosted => Task::batch(vec![
                self.update(Message::Blur(6)),
                self.update(Message::Glass(0.5)),
            ]),

            Frosted::Glassy => Task::batch(vec![
                self.update(Message::Blur(5)),
                self.update(Message::Glass(3. / 14.)),
            ]),

            _ => Task::none(),
        }
    }

    fn refresh_theme(&mut self) {
        let manager = theme_manager::Manager::default();

        self.roundness = manager.builder().corner_radii.into();
        self.frosted = manager.theme().frosted as u8;
        self.glass = manager.theme().alpha_map.guess_offset();
        self.frosted_mode = frosted_mode_of(self.frosted, self.glass);
        self.frosted_system_interface = manager.theme().frosted_system_interface;
        self.frosted_windows = manager.theme().frosted_windows;
        self.frosted_panel = manager.theme().frosted_panel;
        self.frosted_applets = manager.theme().frosted_applets;
        self.density = cosmic::config::interface_density();

        self.theme_manager = manager;
        self.drawer.reset(&self.theme_manager);
        self.drawer.sync_fonts(&self.theme_manager);
    }

    fn reload_theme(&mut self) -> Task<app::Message> {
        self.refresh_theme();

        cosmic::task::future(async { app::Message::SetTheme(cosmic::theme::system_preference()) })
    }
}

fn frosted_mode_of(frosted: u8, glass: f32) -> Frosted {
    if frosted == 6 && (glass - 0.5).abs() < 0.1 {
        Frosted::Frosted
    } else if frosted == 5 && (glass - (3. / 14.)).abs() < 0.1 {
        Frosted::Glassy
    } else {
        Frosted::Custom
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn dialog(&self) -> Option<Element<'_, crate::pages::Message>> {
        self.import.as_ref().map(|import| {
            theme_bundles::import_dialog(import).map(crate::pages::Message::Appearance)
        })
    }

    fn header(&self) -> Option<Element<'_, crate::pages::Message>> {
        Some(
            text::title3(fl!("appearance"))
                .apply(Element::from)
                .map(crate::pages::Message::Appearance),
        )
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(mode_and_auto::section()),
            sections.insert(theme_bundles::section(
                cosmic_theme_bundle_config::Mode::Dark,
            )),
            sections.insert(theme_bundles::section(
                cosmic_theme_bundle_config::Mode::Light,
            )),
            sections.insert(interface_density()),
            sections.insert(frosted::section()),
            sections.insert(window_management()),
            sections.insert(experimental()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("appearance", "preferences-appearance-symbolic")
            .title(fl!("appearance"))
            .description(fl!("xdg-entry-appearance-comment"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        let (task, handle) = cosmic::task::batch(vec![
            // Load icon themes
            // cosmic::task::future(icon_themes::fetch()).map(crate::pages::Message::Appearance),
            cosmic::task::future(async { Message::BundlesLoaded(theme_bundles::load()) })
                .map(crate::pages::Message::Appearance),
            {
                self.refresh_theme();
                cosmic::task::none()
            },
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

    fn subscription(&self, core: &cosmic::Core) -> Subscription<crate::pages::Message> {
        // Keep the Appearance page in sync when the daemon auto-switches light/dark.
        core.watch_config::<ThemeMode>("com.system76.CosmicTheme.Mode")
            .map(|update| {
                for why in update.errors {
                    tracing::error!(?why, "theme mode config load error");
                }
                crate::pages::Message::Appearance(Message::ThemeModeUpdate(update.config))
            })
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        self.drawer
            .context_drawer(self.context_view, crate::pages::Message::Appearance)
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
                .add(settings::item::builder(&descriptions[compact]).radio(
                    Density::Compact,
                    Some(page.density),
                    Message::Density,
                ))
                .add(settings::item::builder(&descriptions[comfortable]).radio(
                    Density::Standard,
                    Some(page.density),
                    Message::Density,
                ))
                .add(settings::item::builder(&descriptions[spacious]).radio(
                    Density::Spacious,
                    Some(page.density),
                    Message::Density,
                ))
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
pub fn window_management() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        active_hint = fl!("window-management-appearance", "active-hint");
        gaps = fl!("window-management-appearance", "gaps");
    });

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

            let mut section = settings::section()
                .title(&*section.title)
                .add(system_font)
                .add(mono_font)
                .add(icons_and_toolkit);

            #[cfg(feature = "cosmic-comp-config")]
            {
                section = section.add(crate::widget::go_next_item(
                    &descriptions[shadow_and_corners_txt],
                    Message::DrawerOpen(ContextView::ShadowAndCorners),
                ));
            }

            section
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: cosmic_settings_page::Insert<crate::pages::Message>,
    ) -> cosmic_settings_page::Insert<crate::pages::Message> {
        let _ = page.sub_page_with_id::<custom::Page>();

        page
    }
}
