use cosmic::app::{ContextDrawer, context_drawer};
use cosmic::config::CosmicTk;
use cosmic::cosmic_config::{Config, ConfigSet};
use cosmic::cosmic_theme::Spacing;
use cosmic::iced_core::{Color, Length};
use cosmic::widget::{
    ColorPickerModel, color_picker::ColorPickerUpdate, container, flex_row, settings, text,
};
use cosmic::{Apply, Task};
use cosmic::{Element, widget};
use std::sync::Arc;

use crate::app;
use crate::widget::color_picker_context_view;

use super::{
    ContextView, Message, font_config, icon_themes,
    icon_themes::{IconHandles, IconThemes},
    theme_manager,
};

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
    icon_global: bool,
    icon_themes: IconThemes,
    icon_handles: IconHandles,
    tk_config: Option<Config>,
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
    ApplyThemeGlobal(bool),
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
                Some(theme.background.base.into()),
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
                Some(theme.background.on.into()),
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
            font_config: font_config::Model::new(),
            icons_fetched: false,
            icon_global: cosmic::config::apply_theme_global(),
            icon_fetch_handle: None,
            icon_theme_active: None,
            icon_themes: Vec::new(),
            icon_handles: Vec::new(),
            tk_config: CosmicTk::config().ok(),
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

    pub fn update_font(
        &mut self,
        message: FontMessage,
        context_view: Option<&ContextView>,
    ) -> Task<app::Message> {
        match message {
            FontMessage::FontLoaded(interface, mono) => {
                return self.font_config.font_loaded(mono, interface);
            }
            FontMessage::Search(input) => match context_view {
                None => Task::none(),
                Some(c) => self.font_config.search(input.to_string(), c),
            },
            FontMessage::Select(font) => {
                if let Some(context_view) = context_view {
                    if let Some(task) = self.font_config.select(font.to_string(), context_view) {
                        return task;
                    }
                }
                return Task::none();
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
    ) -> Task<app::Message> {
        match message {
            IconMessage::IconTheme(id) => {
                if let Some(theme) = self.icon_themes.get(id).cloned() {
                    self.icon_theme_active = Some(id);

                    if let Some(ref config) = self.tk_config {
                        _ = config.set::<String>("icon_theme", theme.id);
                    }

                    tokio::spawn(icon_themes::set_gnome_icon_theme(theme.name));
                }
            }
            IconMessage::ApplyThemeGlobal(enabled) => {
                if let Some(config) = self.tk_config.as_ref() {
                    _ = config.set("apply_theme_global", enabled);
                    self.icon_global = enabled;
                } else {
                    tracing::error!(
                        "Failed to apply theme to GNOME config because the CosmicTK config does not exist."
                    );
                }
            }
            IconMessage::IconLoaded((icon_themes, icon_handles)) => {
                let active_icon_theme = cosmic::config::icon_theme();

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

    pub fn on_open(&mut self, context_view: &ContextView) -> Task<app::Message> {
        match *context_view {
            ContextView::IconsAndToolkit => {
                if self.icons_fetched {
                    return Task::none();
                }

                self.icons_fetched = true;
                let (task, handle) = cosmic::task::future(icon_themes::fetch()).abortable();
                self.icon_fetch_handle = Some(handle);

                return task;
            }
            ContextView::MonospaceFont | ContextView::SystemFont => {
                self.font_config.reset();
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

    pub fn reset(&mut self, manager: &theme_manager::Manager) {
        self.application_background = ColorPickerModel::new(
            &*HEX,
            &*RGB,
            Some(manager.theme().background.base.into()),
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
            Some(manager.theme().background.on.into()),
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

    pub fn context_drawer(
        &self,
        context_view: Option<ContextView>,
    ) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        Some(match context_view? {
            ContextView::AccentWindowHint => context_drawer(
                color_picker_context_view(
                    None,
                    RESET_TO_DEFAULT.as_str().into(),
                    Message::DrawerColor,
                    &self.accent_window_hint,
                )
                .map(crate::pages::Message::Appearance),
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
                .map(crate::pages::Message::Appearance),
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
                .map(crate::pages::Message::Appearance),
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
                .map(crate::pages::Message::Appearance),
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
                .map(crate::pages::Message::Appearance),
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
                .map(crate::pages::Message::Appearance),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("text-tint")),

            ContextView::SystemFont => context_drawer(
                self.font_config
                    .selection_context(&ContextView::SystemFont, |name| {
                        Message::DrawerFont(FontMessage::Select(name))
                    })
                    .map(crate::pages::Message::Appearance),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("interface-font"))
            .header(self.font_config.search_input()),

            ContextView::MonospaceFont => context_drawer(
                self.font_config
                    .selection_context(&ContextView::MonospaceFont, |name| {
                        Message::DrawerFont(FontMessage::Select(name))
                    })
                    .map(crate::pages::Message::Appearance),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("monospace-font"))
            .header(self.font_config.search_input()),

            ContextView::IconsAndToolkit => context_drawer(
                self.icons_and_toolkit(),
                crate::pages::Message::CloseContextDrawer,
            ),
        })
    }

    pub fn icons_and_toolkit(&self) -> Element<'_, crate::pages::Message> {
        let Spacing {
            space_xxs,
            space_xs,
            space_m,
            ..
        } = cosmic::theme::spacing();

        let active = self.icon_theme_active;

        cosmic::iced::widget::column![
            // Export theme choice
            settings::section().add(
                settings::item::builder(fl!("enable-export"))
                    .description(fl!("enable-export", "desc"))
                    .toggler(self.icon_global, |b| {
                        Message::DrawerIcon(IconMessage::ApplyThemeGlobal(b))
                    })
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
                .into()
            ])
            .spacing(space_xxs)
        ]
        .spacing(space_m)
        .width(Length::Fill)
        .apply(Element::from)
        .map(crate::pages::Message::Appearance)
    }
}
