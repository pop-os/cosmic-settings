use cosmic::app::{ContextDrawer, context_drawer};
use cosmic::cosmic_config::ConfigSet;
use cosmic::cosmic_theme::Spacing;
use cosmic::iced::core::{Color, Length};
use cosmic::widget::color_picker::ColorPickerUpdate;
use cosmic::widget::{ColorPickerModel, container, flex_row, settings, text};
use cosmic::{Apply, Element, Task, task, widget};
use cosmic_config::ConfigGet;
use std::sync::Arc;
use tracing::error;

use crate::app;
use crate::pages::desktop::appearance::theme_manager::Manager;
use crate::widget::color_picker_context_view;

use super::icon_themes::{IconHandles, IconThemes};
use super::{ContextView, Message, font_config, icon_themes, theme_manager};

pub struct Content {
    context_view: Option<ContextView>,
    pub custom_accent: ColorPickerModel,
    pub accent_window_hint: ColorPickerModel,
    pub application_background: ColorPickerModel,
    pub container_background: ColorPickerModel,
    pub interface_text: ColorPickerModel,
    pub control_component: ColorPickerModel,

    font_config: font_config::Model,

    icons_fetched: bool,
    icon_fetch_handle: Option<cosmic::iced::task::Handle>,

    icon_theme_active: Option<usize>,
    icon_themes: IconThemes,
    icon_handles: IconHandles,

    comp_config: cosmic_config::Config,
    #[cfg(feature = "cosmic-comp-config")]
    appearance_conf: cosmic_comp_config::AppearanceConfig,
}

#[cfg(feature = "cosmic-comp-config")]
#[derive(Debug, Clone)]
pub enum CornerMessage {
    ClipFloating(bool),
    ClipTiled(bool),
    ShadowTiled(bool),
}

#[derive(Debug, Clone)]
pub enum FontMessage {
    FontLoaded(Vec<Arc<str>>, Vec<Arc<str>>),
    Search(String),
    Select(Arc<str>),
}

#[derive(Debug, Clone)]
pub enum IconMessage {
    IconLoaded((IconThemes, IconHandles)),
    IconTheme(usize),
}

crate::cache_dynamic_lazy! {
    static HEX: String = fl!("hex");
    static RGB: String = fl!("rgb");
    static ICON_THEME: String = fl!("icon-theme");
    static RESET_TO_DEFAULT: String = fl!("reset-to-default");
}

impl From<&theme_manager::Manager> for Content {
    fn from(theme_manager: &theme_manager::Manager) -> Self {
        let theme = theme_manager.theme();
        let comp_config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        #[cfg(feature = "cosmic-comp-config")]
        let appearance_conf = comp_config
            .get::<cosmic_comp_config::AppearanceConfig>("appearance_settings")
            .unwrap_or_default();
        Self {
            context_view: None,
            custom_accent: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                None,
                theme_manager.get_color(&ContextView::CustomAccent),
            ),
            application_background: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                Some(theme.background(false).base.into()),
                theme_manager.get_color(&ContextView::ApplicationBackground),
            ),
            container_background: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                None,
                theme_manager.get_color(&ContextView::ContainerBackground),
            ),
            interface_text: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                Some(theme.background(false).on.into()),
                theme_manager.get_color(&ContextView::InterfaceText),
            ),
            control_component: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                Some(theme.palette.neutral_5.into()),
                theme_manager.get_color(&ContextView::ControlComponent),
            ),
            accent_window_hint: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                None,
                theme_manager.get_color(&ContextView::AccentWindowHint),
            ),
            font_config: font_config::Model::from_theme(theme),

            icons_fetched: false,
            icon_fetch_handle: None,
            icon_theme_active: None,
            icon_themes: Vec::new(),
            icon_handles: Vec::new(),
            comp_config,
            #[cfg(feature = "cosmic-comp-config")]
            appearance_conf,
        }
    }
}

impl Content {
    pub fn current_font_family(&self, context_view: &ContextView) -> String {
        match *context_view {
            ContextView::SystemFont => self.font_config.interface_font.family.clone(),
            ContextView::MonospaceFont => self.font_config.monospace_font.family.clone(),
            _ => "".to_string(),
        }
    }

    pub fn preserve_from(&mut self, previous: &mut Content) {
        self.font_config
            .take_families_from(&mut previous.font_config);

        self.icons_fetched = previous.icons_fetched;
        self.icon_theme_active = previous.icon_theme_active;
        self.icon_themes = std::mem::take(&mut previous.icon_themes);
        self.icon_handles = std::mem::take(&mut previous.icon_handles);
    }

    pub fn update_font(
        &mut self,
        message: FontMessage,
        context_view: Option<&ContextView>,
        manager: &mut Manager,
    ) -> Task<app::Message> {
        match message {
            FontMessage::FontLoaded(interface, mono) => {
                self.font_config.font_loaded(mono, interface)
            }
            FontMessage::Search(input) => match context_view {
                None => Task::none(),
                Some(c) => self.font_config.search(input.to_string(), c),
            },
            FontMessage::Select(font) => {
                if let Some(context_view) = context_view {
                    self.font_config
                        .select(font.to_string(), context_view, manager);
                }

                Task::none()
            }
        }
    }

    pub fn update_color(
        &mut self,
        tasks: &mut Vec<Task<app::Message>>,
        message: ColorPickerUpdate,
        context_view: &ContextView,
    ) -> bool {
        let mut needs_update = false;

        match message {
            ColorPickerUpdate::ActionFinished => {
                needs_update = true;
            }

            ColorPickerUpdate::AppliedColor | ColorPickerUpdate::Reset => {
                needs_update = true;
                self.context_view = None;
                tasks.push(cosmic::task::message(
                    crate::pages::Message::CloseContextDrawer,
                ));
            }

            ColorPickerUpdate::Cancel => {
                self.context_view = None;
                tasks.push(cosmic::task::message(
                    crate::pages::Message::CloseContextDrawer,
                ));
            }

            _ => (),
        }

        tasks.push(match *context_view {
            ContextView::CustomAccent => self.custom_accent.update(message),
            ContextView::ApplicationBackground => self.application_background.update(message),
            ContextView::ContainerBackground => self.container_background.update(message),
            ContextView::InterfaceText => self.interface_text.update(message),
            ContextView::ControlComponent => self.control_component.update(message),
            ContextView::AccentWindowHint => self.accent_window_hint.update(message),
            _ => return needs_update,
        });

        needs_update
    }

    pub fn update_icon(
        &mut self,
        message: IconMessage,
        _context_view: &ContextView,
        manager: &mut Manager,
    ) -> Task<app::Message> {
        match message {
            IconMessage::IconTheme(id) => {
                if let Some(theme) = self.icon_themes.get(id).cloned() {
                    self.icon_theme_active = Some(id);

                    let is_dark = manager.mode().is_dark;
                    manager.set_icon_theme(theme.id.clone());

                    let (config, builder_config) = if is_dark {
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

                    if let Some((config, builder_config)) = config.ok().zip(builder_config.ok()) {
                        _ = config.set::<String>("icon_theme", theme.id.clone());
                        _ = builder_config.set::<String>("icon_theme", theme.id);

                        if let Ok(store) = cosmic_theme_bundle_config::Store::new()
                            && let Err(error) = store.sync_active(
                                cosmic_theme_bundle_config::Mode::from_is_dark(is_dark),
                            )
                        {
                            tracing::warn!(?error, "failed to sync the active theme bundle");
                        }
                    }
                }
            }
            IconMessage::IconLoaded((icon_themes, icon_handles)) => {
                let active_icon_theme = manager.theme().icon_theme.clone();

                // Set the icon themes, and define the active icon theme.
                self.icon_themes = icon_themes;
                self.icon_theme_active = self
                    .icon_themes
                    .iter()
                    .position(|theme| theme.id == active_icon_theme);
                self.icon_handles = icon_handles;

                return cosmic::task::message(app::Message::SetTheme(
                    cosmic::theme::system_preference(),
                ));
            }
        }
        Task::none()
    }

    #[cfg(feature = "cosmic-comp-config")]
    pub fn update_shadow_and_corners(
        &mut self,
        message: CornerMessage,
        _context_view: &ContextView,
    ) -> Task<app::Message> {
        match message {
            CornerMessage::ClipFloating(enabled) => {
                self.appearance_conf.clip_floating_windows = enabled;
            }
            CornerMessage::ClipTiled(enabled) => {
                self.appearance_conf.clip_tiled_windows = enabled;
            }
            CornerMessage::ShadowTiled(enabled) => {
                self.appearance_conf.shadow_tiled_windows = enabled;
            }
        }

        if let Err(err) = self
            .comp_config
            .set("appearance_settings", self.appearance_conf)
        {
            error!(?err, "Failed to set config 'appearance_settings'");
        }

        Task::none()
    }

    pub fn on_open<F>(
        &mut self,
        context_view: &ContextView,
        manager: &Manager,
        wrap: F,
    ) -> Task<app::Message>
    where
        F: Fn(super::Message) -> crate::pages::Message + Send + 'static,
    {
        match *context_view {
            ContextView::IconThemes => {
                self.icon_theme_active = self
                    .icon_themes
                    .iter()
                    .position(|theme| theme.id == manager.theme().icon_theme);

                if self.icons_fetched {
                    return Task::none();
                }

                self.icons_fetched = true;
                let (task, handle) = cosmic::task::future(icon_themes::fetch()).abortable();
                self.icon_fetch_handle = Some(handle);

                return task.map(wrap).map(crate::app::Message::PageMessage);
            }
            ContextView::MonospaceFont | ContextView::SystemFont => {
                self.font_config.reset();
                return task::message(super::Message::FocusFontInput)
                    .map(wrap)
                    .map(crate::app::Message::PageMessage);
            }
            _ => {}
        }
        Task::none()
    }

    pub fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some(handle) = self.icon_fetch_handle.take() {
            handle.abort();
        }
        Task::none()
    }

    // Returns the color associated with the color picker for the context view.
    // Returns None if the context view is not associated to any color picker.
    pub fn current_color(&self, context_view: &ContextView) -> Option<Color> {
        match *context_view {
            ContextView::CustomAccent => self.custom_accent.get_applied_color(),
            ContextView::ApplicationBackground => self.application_background.get_applied_color(),
            ContextView::ContainerBackground => self.container_background.get_applied_color(),
            ContextView::InterfaceText => self.interface_text.get_applied_color(),
            ContextView::ControlComponent => self.control_component.get_applied_color(),
            ContextView::AccentWindowHint => self.accent_window_hint.get_applied_color(),
            _ => None,
        }
    }

    pub fn sync_fonts(&mut self, manager: &theme_manager::Manager) {
        self.font_config.set_theme(manager.theme());
    }

    pub fn reset(&mut self, manager: &theme_manager::Manager) {
        self.application_background = ColorPickerModel::new(
            &*HEX,
            &*RGB,
            Some(manager.theme().background(false).base.into()),
            manager.get_color(&ContextView::ApplicationBackground),
        );
        self.custom_accent = ColorPickerModel::new(
            &*HEX,
            &*RGB,
            None,
            manager.get_color(&ContextView::CustomAccent),
        );
        self.container_background = ColorPickerModel::new(
            &*HEX,
            &*RGB,
            None,
            manager.get_color(&ContextView::ContainerBackground),
        );
        self.interface_text = ColorPickerModel::new(
            &*HEX,
            &*RGB,
            Some(manager.theme().background(false).on.into()),
            manager.get_color(&ContextView::InterfaceText),
        );
        self.control_component = ColorPickerModel::new(
            &*HEX,
            &*RGB,
            Some(manager.theme().palette.neutral_5.into()),
            manager.get_color(&ContextView::ControlComponent),
        );
        self.accent_window_hint = ColorPickerModel::new(
            &*HEX,
            &*RGB,
            None,
            manager.get_color(&ContextView::AccentWindowHint),
        );
    }

    pub fn context_drawer<F>(
        &self,
        context_view: Option<ContextView>,
        wrap: F,
    ) -> Option<ContextDrawer<'_, crate::pages::Message>>
    where
        F: Fn(super::Message) -> crate::pages::Message + Clone + 'static,
    {
        Some(match context_view? {
            ContextView::AccentWindowHint => context_drawer(
                color_picker_context_view(
                    None,
                    RESET_TO_DEFAULT.as_str().into(),
                    Message::DrawerColor,
                    &self.accent_window_hint,
                )
                .map(wrap.clone()),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("window-hint-accent")),

            ContextView::ApplicationBackground => context_drawer(
                color_picker_context_view(
                    None,
                    RESET_TO_DEFAULT.as_str().into(),
                    Message::DrawerColor,
                    &self.application_background,
                )
                .map(wrap.clone()),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("app-background")),

            ContextView::ContainerBackground => context_drawer(
                color_picker_context_view(
                    Some(fl!("container-background", "desc-detail").into()),
                    fl!("container-background", "reset").into(),
                    Message::DrawerColor,
                    &self.container_background,
                )
                .map(wrap.clone()),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("container-background")),

            ContextView::ControlComponent => context_drawer(
                color_picker_context_view(
                    None,
                    RESET_TO_DEFAULT.as_str().into(),
                    Message::DrawerColor,
                    &self.control_component,
                )
                .map(wrap.clone()),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("control-tint")),

            ContextView::CustomAccent => context_drawer(
                color_picker_context_view(
                    None,
                    RESET_TO_DEFAULT.as_str().into(),
                    Message::DrawerColor,
                    &self.custom_accent,
                )
                .map(wrap.clone()),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("accent-color")),

            ContextView::InterfaceText => context_drawer(
                color_picker_context_view(
                    None,
                    RESET_TO_DEFAULT.as_str().into(),
                    Message::DrawerColor,
                    &self.interface_text,
                )
                .map(wrap.clone()),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("text-tint")),

            ContextView::SystemFont => context_drawer(
                self.font_config
                    .selection_context(&ContextView::SystemFont, |name| {
                        Message::DrawerFont(FontMessage::Select(name))
                    })
                    .map(wrap.clone()),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("interface-font"))
            .header(self.font_config.search_input().map(wrap.clone())),

            ContextView::MonospaceFont => context_drawer(
                self.font_config
                    .selection_context(&ContextView::MonospaceFont, |name| {
                        Message::DrawerFont(FontMessage::Select(name))
                    })
                    .map(wrap.clone()),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("monospace-font"))
            .header(self.font_config.search_input().map(wrap.clone())),

            ContextView::IconThemes => context_drawer(
                self.icon_theme_choices(wrap.clone()),
                crate::pages::Message::CloseContextDrawer,
            ),

            #[cfg(feature = "cosmic-comp-config")]
            ContextView::ShadowAndCorners => context_drawer(
                self.shadow_and_corners(),
                crate::pages::Message::CloseContextDrawer,
            ),
        })
    }

    pub fn icon_theme_choices<F>(&self, wrap: F) -> Element<'_, crate::pages::Message>
    where
        F: Fn(Message) -> crate::pages::Message + Clone + 'static,
    {
        let Spacing {
            space_xxs,
            space_xs,
            ..
        } = cosmic::theme::spacing();

        let active = self.icon_theme_active;

        widget::column::with_children([
            text::heading(&*ICON_THEME).into(),
            flex_row(
                self.icon_themes
                    .iter()
                    .zip(self.icon_handles.iter())
                    .enumerate()
                    .map(|(i, (theme, handles))| {
                        let selected = active.map(|j| i == j).unwrap_or_default();
                        icon_themes::button(&theme.name, handles, i, selected, |id| {
                            Message::DrawerIcon(IconMessage::IconTheme(id))
                        })
                    })
                    .collect(),
            )
            .row_spacing(space_xs)
            .column_spacing(space_xs)
            .apply(container)
            .center_x(Length::Fill)
            .into(),
        ])
        .spacing(space_xxs)
        .width(Length::Fill)
        .apply(Element::from)
        .map(wrap)
    }

    #[cfg(feature = "cosmic-comp-config")]
    pub fn shadow_and_corners(&self) -> Element<'_, crate::pages::Message> {
        let Spacing { space_m, .. } = cosmic::theme::spacing();

        cosmic::iced::widget::column![
            settings::section().title(fl!("shadows-floating")).add(
                settings::item::builder(fl!("shadows-floating", "clip"))
                    .toggler(self.appearance_conf.clip_floating_windows, |b| {
                        Message::DrawerCorners(CornerMessage::ClipFloating(b))
                    })
            ),
            settings::section()
                .title(fl!("shadows-tiling"))
                .add(
                    settings::item::builder(fl!("shadows-tiling", "clip"))
                        .toggler(self.appearance_conf.clip_tiled_windows, |b| {
                            Message::DrawerCorners(CornerMessage::ClipTiled(b))
                        })
                )
                .add(
                    settings::item::builder(fl!("shadows-tiling", "shadow"))
                        .toggler(self.appearance_conf.shadow_tiled_windows, |b| {
                            Message::DrawerCorners(CornerMessage::ShadowTiled(b))
                        })
                )
        ]
        .spacing(space_m)
        .width(Length::Fill)
        .apply(Element::from)
        .map(crate::pages::Message::Appearance)
    }
}
