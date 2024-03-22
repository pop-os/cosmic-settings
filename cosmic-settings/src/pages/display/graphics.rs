// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::{Message, NightLight};
use crate::pages;
use cosmic::iced_core::{Alignment, Length, Padding};
use cosmic::prelude::CollectionWidget;
use cosmic::widget::{button, column, icon, list_column, row, toggler};
use cosmic::{Apply, Command, Element};
use std::sync::Arc;

pub const INTEGRATED: &str = "integrated";
pub const NVIDIA: &str = "nvidia";
pub const HYBRID: &str = "hybrid";
pub const COMPUTE: &str = "compute";

pub async fn fetch() -> Option<Arc<std::io::Result<Mode>>> {
    let switchable = tokio::process::Command::new("system76-power")
        .args(["graphics", "switchable"])
        .output()
        .await
        .map(|output| {
            std::str::from_utf8(&output.stdout).map_or(false, |text| text.trim() == "switchable")
        });

    match switchable {
        Ok(false) => None,

        Ok(true) => Some(Arc::new(
            tokio::process::Command::new("system76-power")
                .arg("graphics")
                .output()
                .await
                .and_then(|output| {
                    if let Ok(mut mode) = std::str::from_utf8(&output.stdout) {
                        mode = mode.trim();
                        if mode == COMPUTE {
                            Ok(Mode::Compute)
                        } else if mode == HYBRID {
                            Ok(Mode::Hybrid)
                        } else if mode == INTEGRATED {
                            Ok(Mode::Integrated)
                        } else if mode == NVIDIA {
                            Ok(Mode::Nvidia)
                        } else {
                            Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "unknown graphics mode",
                            ))
                        }
                    } else {
                        Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "system76-power output was not UTF-8",
                        ))
                    }
                }),
        )),

        Err(why) => Some(Arc::new(Err(why))),
    }
}

pub fn view(
    mode: &'static str,
    description: &'static str,
    button: Option<(&'static str, Message)>,
) -> Element<'static, Message> {
    let theme = cosmic::theme::active();
    let theme = theme.cosmic();
    let has_checkmark = button.is_none();

    let content = column::with_capacity(3)
        .padding(Padding::from([theme.space_xxs(), theme.space_l()]))
        .push(cosmic::widget::text::body(mode))
        .push(cosmic::widget::text::caption(description))
        .push(cosmic::widget::Space::new(Length::Fill, 12))
        .push_maybe(button.map(|(text, message)| {
            button::text(text)
                .style(cosmic::theme::Button::Link)
                .trailing_icon(icon::from_name("go-next-symbolic").size(16))
                .padding(0)
                .on_press(message)
        }));

    if has_checkmark {
        row::with_capacity(2)
            .align_items(Alignment::Center)
            .push(content)
            .push(icon::from_name("object-select-symbolic").size(24))
            .apply(Element::from)
            .apply(cosmic::widget::list::container)
            .into()
    } else {
        cosmic::widget::list::container(content).into()
    }
}

/// Switchable graphics mode
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Compute,
    Hybrid,
    Integrated,
    Nvidia,
}

impl Mode {
    #[must_use]
    pub fn argument_str(self) -> &'static str {
        match self {
            Self::Compute => COMPUTE,
            Self::Hybrid => HYBRID,
            Self::Integrated => INTEGRATED,
            Self::Nvidia => NVIDIA,
        }
    }

    #[must_use]
    pub fn localized_str(self) -> &'static str {
        match self {
            Self::Compute => &super::text::GRAPHICS_MODE_COMPUTE,
            Self::Hybrid => &super::text::GRAPHICS_MODE_HYBRID,
            Self::Integrated => &super::text::GRAPHICS_MODE_INTEGRATED,
            Self::Nvidia => &super::text::GRAPHICS_MODE_NVIDIA,
        }
    }
}

impl super::Page {
    pub fn graphics_mode_view(&self) -> Element<pages::Message> {
        let mut container = list_column();

        if let Some(graphics_mode) = self.config.graphics_mode {
            // Displays the active graphics mode, and a button for configuring it.
            container = container.add(cosmic::widget::settings::item(
                &*super::text::GRAPHICS_MODE,
                row()
                    .align_items(Alignment::Center)
                    .push(cosmic::widget::text::body(graphics_mode.localized_str()))
                    .push(
                        button::icon(icon::from_name("go-next-symbolic"))
                            .extra_small()
                            .on_press(Message::GraphicsModeContext),
                    ),
            ));
        }

        // Displays the night light status, and a button for configuring it.
        container = container.add(
            cosmic::widget::settings::item::builder(&*super::text::NIGHT_LIGHT)
                .description(&*super::text::NIGHT_LIGHT_DESCRIPTION)
                .control(
                    row()
                        .align_items(Alignment::Center)
                        .push(toggler(None, self.config.night_light_enabled, |enable| {
                            Message::NightLight(NightLight::Toggle(enable))
                        }))
                        .push(
                            button::icon(icon::from_name("go-next-symbolic"))
                                .extra_small()
                                .on_press(Message::NightLightContext),
                        ),
                ),
        );

        container
            .apply(Element::from)
            .map(crate::pages::Message::Displays)
    }

    pub fn graphics_mode_context_view(&self) -> Element<pages::Message> {
        let theme = cosmic::theme::active();
        let theme = theme.cosmic();

        column::with_capacity(4)
            .spacing(theme.space_xs())
            .push(view(
                &super::text::GRAPHICS_MODE_INTEGRATED,
                &super::text::GRAPHICS_MODE_INTEGRATED_DESC,
                if let Some(Mode::Integrated) = self.config.graphics_mode {
                    None
                } else {
                    Some((
                        &super::text::GRAPHICS_MODE_INTEGRATED_ENABLE,
                        Message::GraphicsMode(Mode::Integrated),
                    ))
                },
            ))
            .push(view(
                &super::text::GRAPHICS_MODE_NVIDIA,
                &super::text::GRAPHICS_MODE_NVIDIA_DESC,
                if let Some(Mode::Nvidia) = self.config.graphics_mode {
                    None
                } else {
                    Some((
                        &super::text::GRAPHICS_MODE_NVIDIA_ENABLE,
                        Message::GraphicsMode(Mode::Nvidia),
                    ))
                },
            ))
            .push(view(
                &super::text::GRAPHICS_MODE_HYBRID,
                &super::text::GRAPHICS_MODE_HYBRID_DESC,
                if let Some(Mode::Hybrid) = self.config.graphics_mode {
                    None
                } else {
                    Some((
                        &super::text::GRAPHICS_MODE_HYBRID_ENABLE,
                        Message::GraphicsMode(Mode::Hybrid),
                    ))
                },
            ))
            .push(view(
                &super::text::GRAPHICS_MODE_COMPUTE,
                &super::text::GRAPHICS_MODE_COMPUTE_DESC,
                if let Some(Mode::Compute) = self.config.graphics_mode {
                    None
                } else {
                    Some((
                        &super::text::GRAPHICS_MODE_COMPUTE_ENABLE,
                        Message::GraphicsMode(Mode::Compute),
                    ))
                },
            ))
            .apply(Element::from)
            .map(pages::Message::Displays)
    }

    /// Change the graphics mode.
    pub fn set_graphics_mode(&mut self, mode: Mode) -> Command<crate::app::Message> {
        self.config.graphics_mode = Some(mode);

        // Runs `system76-power graphics {{mode}}`
        cosmic::command::future(async move {
            let result = tokio::process::Command::new("system76-power")
                .args(["graphics", mode.argument_str()])
                .status()
                .await;
            let page_message =
                crate::pages::Message::Displays(Message::GraphicsModeResult(Arc::new(result)));
            crate::app::Message::PageMessage(page_message)
        })
    }
}
