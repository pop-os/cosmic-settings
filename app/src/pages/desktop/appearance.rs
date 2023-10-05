// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;

use apply::Apply;
use cosmic::cosmic_config::{Config, ConfigSet, CosmicConfigEntry};
use cosmic::cosmic_theme::palette::{Srgb, Srgba};
use cosmic::cosmic_theme::{CornerRadii, Theme, ThemeBuilder, ThemeMode};
use cosmic::iced::wayland::actions::window::SctkWindowSettings;
use cosmic::iced::widget::{column, row};
use cosmic::iced::window;
use cosmic::iced_core::{layout, Color, Length};
use cosmic::iced_sctk::commands::window::{close_window, get_window};
use cosmic::iced_widget::scrollable;
use cosmic::widget::icon::{from_name, icon};
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
    roundness: Roundness,

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

impl From<(Option<Config>, ThemeMode)> for Page {
    fn from((theme_mode_config, theme_mode): (Option<Config>, ThemeMode)) -> Self {
        let theme_builder_config = if theme_mode.is_dark {
            ThemeBuilder::dark_config()
        } else {
            ThemeBuilder::light_config()
        }
        .ok();
        let theme_builder = theme_builder_config
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
            .unwrap_or_else(|| {
                if theme_mode.is_dark {
                    ThemeBuilder::dark()
                } else {
                    ThemeBuilder::light()
                }
            });
        // TODO fill all these values with the current values
        Self {
            accent_window_hint: Default::default(),
            frosted: Default::default(),
            window_hint_size: Default::default(),
            gap_size: Default::default(),
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
                theme_builder.accent.map(Color::from),
            ),
            application_background: ColorPickerModel::new(
                fl!("hex"),
                fl!("rgb"),
                None,
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
                None,
                theme_builder.text_tint.map(Color::from),
            ),
            control_component: ColorPickerModel::new(
                fl!("hex"),
                fl!("rgb"),
                None,
                theme_builder.neutral_tint.map(Color::from),
            ),
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
    Entered,
    DarkMode(bool),
    Autoswitch(bool),
    AccentWindowHint(bool),
    Frosted(bool),
    WindowHintSize(spin_button::Message),
    GapSize(spin_button::Message),
    ApplicationBackground(ColorPickerUpdate),
    ContainerBackground(ColorPickerUpdate),
    PaletteAccent(cosmic::iced::Color),
    CustomAccent(ColorPickerUpdate),
    InterfaceText(ColorPickerUpdate),
    ControlComponent(ColorPickerUpdate),
    CloseRequested(window::Id),
    Roundness(Roundness),
    Left,
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
        if value.radius_m[0] == 16.0 {
            Self::Round
        } else if value.radius_m[0] == 8.0 {
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
                        self.active_dialog = Some(NamedColorPicker::ContainerBackground);
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
                        // TODO set the values of the theme builder
                        theme_builder.build(),
                    )))
                }),
            ])
        }

        // TODO if there were some changes, rebuild and apply to the config
        ret
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
            // 13
            fl!("dark"),
            fl!("light"),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;
            let palette = &page.theme_builder.palette.as_ref();
            let cur_accent = page
                .theme_builder
                .accent
                .map(|a| Srgba::from(a))
                .unwrap_or(palette.accent_blue);
            settings::view_section(&section.title)
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
                                style_color_button(
                                    page.custom_accent.picker_button(Message::CustomAccent),
                                    page.custom_accent
                                        .get_applied_color()
                                        .unwrap_or(cosmic::iced::Color::BLACK),
                                    page.custom_accent.get_applied_color()
                                        == Some(cur_accent.into())
                                )
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
                .add(settings::item::builder(&descriptions[0]).control(
                    cosmic::widget::spin_button(
                        page.window_hint_size.to_string(),
                        Message::WindowHintSize,
                    ),
                ))
                .add(settings::item::builder(&descriptions[1]).control(
                    cosmic::widget::spin_button(page.gap_size.to_string(), Message::GapSize),
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

// TODO replace with image button / toggle buttons
fn color_button<'a, Message: 'a>(
    on_press: Option<Message>,
    color: cosmic::iced::Color,
    selected: bool,
) -> cosmic::widget::Button<'a, Message, cosmic::Renderer> {
    let ret = button(cosmic::widget::vertical_space(Length::Fixed(f32::from(
        48.0,
    ))))
    .width(Length::Fixed(f32::from(48.0)))
    .height(Length::Fixed(f32::from(48.0)))
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
