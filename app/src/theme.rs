// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::theme;

#[must_use]
pub fn display_container_frame() -> cosmic::theme::Container {
    theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        cosmic::widget::container::Appearance {
            icon_color: None,
            text_color: None,
            background: Some(cosmic::iced::Background::Color(cosmic::iced::Color::WHITE)),
            border_color: cosmic::iced::Color::WHITE,
            border_radius: cosmic.corner_radii.radius_xs.into(),
            border_width: 3.0,
        }
    })
}

#[must_use]
pub fn display_container_screen() -> cosmic::theme::Container {
    theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        cosmic::widget::container::Appearance {
            icon_color: None,
            text_color: None,
            background: Some(cosmic::iced::Background::Color(cosmic::iced::Color::BLACK)),
            border_color: cosmic::iced::Color::BLACK,
            border_radius: cosmic.corner_radii.radius_0.into(),
            border_width: 0.0,
        }
    })
}
