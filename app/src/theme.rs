// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{iced_widget::core::BorderRadius, theme};

#[must_use]
pub fn display_container_frame() -> cosmic::theme::Container {
    theme::Container::custom(|_theme| cosmic::iced::widget::container::Appearance {
        text_color: None,
        background: Some(cosmic::iced::Background::Color(cosmic::iced::Color::WHITE)),
        border_color: cosmic::iced::Color::WHITE,
        border_radius: BorderRadius::from(4.0),
        border_width: 3.0,
    })
}

#[must_use]
pub fn display_container_screen() -> cosmic::theme::Container {
    theme::Container::custom(|_theme| cosmic::iced::widget::container::Appearance {
        text_color: None,
        background: Some(cosmic::iced::Background::Color(cosmic::iced::Color::BLACK)),
        border_color: cosmic::iced::Color::BLACK,
        border_radius: BorderRadius::from(0.0),
        border_width: 0.0,
    })
}
