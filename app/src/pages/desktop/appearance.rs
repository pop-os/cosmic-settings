// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use apply::Apply;
use cosmic::cosmic_config::{Config, ConfigSet, CosmicConfigEntry};
use cosmic::cosmic_theme::palette::Srgba;
use cosmic::cosmic_theme::{Theme, ThemeBuilder, ThemeMode};
use cosmic::iced::wayland::actions::window::SctkWindowSettings;
use cosmic::iced::widget::{column, row};
use cosmic::iced::{window, Color};
use cosmic::iced_core::{layout, Length};
use cosmic::iced_sctk::commands::window::{close_window, get_window};
use cosmic::widget::{
    button, container, header_bar, horizontal_space, settings, spin_button, text, ColorPickerModel,
    ColorPickerUpdate,
};
use cosmic::{Command, Element};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;
use tracing::warn;

use crate::app;
pub const COLOR_PICKER_DIALOG_ID: window::Id = window::Id(1003);

enum NamedColorPicker {
    CustomAccent,
    ApplicationBackground,
    ContainerBackground,
    InterfaceText,
    ControlComponent,
}
// TODO integrate with settings backend
pub struct Page {
    accent_window_hint: bool,
    frosted: bool,
    window_hint_size: u16,
    gap_size: u16,
    can_reset: bool,
    custom_accent: ColorPickerModel,
    application_background: ColorPickerModel,
    container_background: ColorPickerModel,
    interface_text: ColorPickerModel,
    control_component: ColorPickerModel,
    active_dialog: Option<NamedColorPicker>,

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

        let theme_builder_config = if theme_mode.is_dark {
            ThemeBuilder::dark_config()
        } else {
            ThemeBuilder::light_config()
        }
        .ok();
        let theme_builder = theme_mode_config
            .as_ref()
            .map(|c| match ThemeBuilder::get_entry(c) {
                Ok(t) => t,
                Err((errors, t)) => {
                    for e in errors {
                        tracing::error!("{e}");
                    }
                    t
                }
            })
            .unwrap_or_default();
        Self {
            accent_window_hint: Default::default(),
            frosted: Default::default(),
            window_hint_size: Default::default(),
            gap_size: Default::default(),
            can_reset: Default::default(),
            custom_accent: ColorPickerModel::new(fl!("hex"), fl!("rgb"), None, None),
            application_background: ColorPickerModel::new(fl!("hex"), fl!("rgb"), None, None),
            container_background: ColorPickerModel::new(fl!("hex"), fl!("rgb"), None, None),
            interface_text: ColorPickerModel::new(fl!("hex"), fl!("rgb"), None, None),
            control_component: ColorPickerModel::new(fl!("hex"), fl!("rgb"), None, None),
            active_dialog: None,
            theme_mode_config,
            theme_builder_config,
            theme_mode,
            theme_builder,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    DarkMode(bool),
    Autoswitch(bool),
    AccentWindowHint(bool),
    Frosted(bool),
    WindowHintSize(spin_button::Message),
    GapSize(spin_button::Message),
    ApplicationBackground(ColorPickerUpdate),
    ContainerBackground(ColorPickerUpdate),
    CustomAccent(ColorPickerUpdate),
    InterfaceText(ColorPickerUpdate),
    ControlComponent(ColorPickerUpdate),
    CloseRequested(window::Id),
}

impl Page {
    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        let mut theme_builder_needs_update = false;

        let ret = match message {
            Message::DarkMode(enabled) => {
                self.theme_mode.is_dark = enabled;
                if let Some(config) = self.theme_mode_config.as_ref() {
                    // only update dark mode if autoswitch is disabled
                    if !self.theme_mode.auto_switch {
                        _ = config.set::<bool>("is_dark", enabled);
                    }
                }
                let theme_builder_config = if enabled {
                    ThemeBuilder::dark_config()
                } else {
                    ThemeBuilder::light_config()
                }
                .ok();
                if let Some(config) = theme_builder_config.as_ref() {
                    self.theme_builder = match ThemeBuilder::get_entry(config) {
                        Ok(tb) => tb,
                        Err((errors, tb)) => {
                            for e in errors {
                                tracing::error!("{e}");
                            }
                            tb
                        }
                    };
                } else {
                    tracing::error!("Failed to load the theme builder config.");
                }
                Command::none()
            }
            Message::Autoswitch(enabled) => {
                self.theme_mode.auto_switch = enabled;
                if let Some(config) = self.theme_mode_config.as_ref() {
                    _ = config.set::<bool>("auto_switch", enabled);
                }
                if !enabled {}
                Command::none()
            }
            Message::AccentWindowHint(enabled) => {
                // TODO write to cosmic comp config
                self.accent_window_hint = enabled;
                Command::none()
            }
            Message::Frosted(enabled) => {
                // TODO add variable to the config
                self.frosted = enabled;
                Command::none()
            }
            Message::WindowHintSize(msg) => {
                // TODO write to cosmic comp config
                self.window_hint_size = match msg {
                    spin_button::Message::Increment => self.window_hint_size.saturating_add(1),
                    spin_button::Message::Decrement => self.window_hint_size.saturating_sub(1),
                };
                Command::none()
            }
            Message::GapSize(msg) => {
                self.gap_size = match msg {
                    spin_button::Message::Increment => self.gap_size.saturating_add(1),
                    spin_button::Message::Decrement => self.gap_size.saturating_sub(1),
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
                        // close the color picker dialog
                        // apply changes
                        theme_builder_needs_update = true;
                        close_window::<app::Message>(COLOR_PICKER_DIALOG_ID)
                    }
                    // TODO apply changes
                    ColorPickerUpdate::ActionFinished => {
                        theme_builder_needs_update = true;
                        _ = self
                            .container_background
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
                Command::batch(vec![cmd, self.custom_accent.update::<app::Message>(u)])
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
            Message::CloseRequested(id) => {
                match self.active_dialog.take() {
                    Some(NamedColorPicker::ApplicationBackground) => {
                        _ = self
                            .application_background
                            .update::<app::Message>(ColorPickerUpdate::Cancel);
                    }
                    Some(NamedColorPicker::ContainerBackground) => {
                        _ = self
                            .container_background
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
                    None => {
                        theme_builder_needs_update = false;
                        warn!("Unknown appearance dialog closed.");
                    }
                };
                Command::none()
            }
        };

        if theme_builder_needs_update {
            let Some(config) = self.theme_builder_config.as_ref() else {
                return ret;
            };
            let mut theme_builder = std::mem::take(&mut self.theme_builder);
            if let Some(c) = self.application_background.get_applied_color() {
                theme_builder = theme_builder.bg_color(c.into());
            }
            if let Some(c) = self.container_background.get_applied_color() {
                theme_builder = theme_builder.primary_container_bg(c.into());
            }
            if let Some(c) = self.custom_accent.get_applied_color() {
                theme_builder = theme_builder.accent(c.into());
            }
            if let Some(c) = self.interface_text.get_applied_color() {
                theme_builder = theme_builder.text_tint(c.into());
            }
            if let Some(c) = self.control_component.get_applied_color() {
                theme_builder = theme_builder.neutral_tint(c.into());
            }

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
        }

        // TODO if there were some changes, rebuild and apply to the config
        return ret;
    }

    pub fn color_picker_view(&self) -> Element<crate::app::Message> {
        let (picker, msg): (_, fn(ColorPickerUpdate) -> Message) = match self.active_dialog {
            Some(NamedColorPicker::ApplicationBackground) => {
                (&self.application_background, Message::ApplicationBackground)
            }
            Some(NamedColorPicker::ContainerBackground) => {
                (&self.container_background, Message::ContainerBackground)
            }
            Some(NamedColorPicker::ControlComponent) => {
                (&self.control_component, Message::ControlComponent)
            }
            Some(NamedColorPicker::CustomAccent) => (&self.custom_accent, Message::CustomAccent),
            Some(NamedColorPicker::InterfaceText) => (&self.interface_text, Message::InterfaceText),
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
            fl!("window-hint-accent"),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(container(text("TODO")).width(Length::Fill))
                .add(
                    settings::item::builder(&descriptions[0])
                        .description(&descriptions[1])
                        .toggler(page.theme_mode.auto_switch, Message::Autoswitch),
                )
                .add(column![
                    text(&descriptions[2]),
                    // if page.container_background.get_is_active() {
                    //     container(
                    //         page.
                    //             .builder(Message::ContainerBackground)
                    //             .build(
                    //                 fl!("recent-colors"),
                    //                 fl!("copy-to-clipboard"),
                    //                 fl!("copied-to-clipboard"),
                    //             ),
                    //     )
                    // } else {
                    //     container(
                    //         page.bg_color_model
                    //             .picker_button(Message::ContainerBackground),
                    //     )
                    // }
                ])
                .add(
                    settings::item::builder(&descriptions[3]).control(
                        page.application_background
                            .picker_button(Message::ApplicationBackground)
                            .width(Length::Fixed(48.0))
                            .height(Length::Fixed(24.0)),
                    ),
                )
                .add(
                    settings::item::builder(&descriptions[4])
                        .description(&descriptions[5])
                        .control(
                            page.container_background
                                .picker_button(Message::ContainerBackground)
                                .width(Length::Fixed(48.0))
                                .height(Length::Fixed(24.0)),
                        ),
                )
                .add(
                    settings::item::builder(&descriptions[8])
                        .description(&descriptions[9])
                        .control(
                            page.interface_text
                                .picker_button(Message::InterfaceText)
                                .width(Length::Fixed(48.0))
                                .height(Length::Fixed(24.0)),
                        ),
                )
                .add(
                    settings::item::builder(&descriptions[10])
                        .description(&descriptions[11])
                        .control(
                            page.control_component
                                .picker_button(Message::ControlComponent)
                                .width(Length::Fixed(48.0))
                                .height(Length::Fixed(24.0)),
                        ),
                )
                .add(
                    settings::item::builder(&descriptions[12])
                        .toggler(page.accent_window_hint, Message::AccentWindowHint),
                )
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
                .add(container(text("TODO Toggle Image buttons for roundness")).width(Length::Fill))
                .add(
                    settings::item::builder(&descriptions[3])
                        .description(&descriptions[4])
                        .toggler(page.frosted, Message::Frosted),
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
                .add(
                    settings::item::builder(&descriptions[0])
                        .control(cosmic::widget::spin_button("", Message::WindowHintSize)),
                )
                .add(
                    settings::item::builder(&descriptions[1])
                        .control(cosmic::widget::spin_button("", Message::GapSize)),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
pub fn reset_button() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![fl!("reset-default")])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;
            if page.can_reset {
                row![button(text(&descriptions[0]))].apply(Element::from)
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
