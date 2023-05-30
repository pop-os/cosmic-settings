// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{iced_widget::core::renderer::BorderRadius, theme};

#[must_use]
pub fn display_container() -> cosmic::theme::Container {
    theme::Container::custom(|_theme| cosmic::iced::widget::container::Appearance {
        text_color: None,
        background: Some(cosmic::iced::Background::Color(cosmic::iced::Color::WHITE)),
        border_color: cosmic::iced::Color::WHITE,
        border_radius: BorderRadius::from(4.0),
        border_width: 3.0,
    })
}
