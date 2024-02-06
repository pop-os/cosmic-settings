// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{iced_core::Border, theme};

#[must_use]
pub fn display_container_frame() -> cosmic::theme::Container {
    theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        cosmic::widget::container::Appearance {
            icon_color: None,
            text_color: None,
            background: Some(cosmic::iced::Background::Color(cosmic::iced::Color::WHITE)),
            border: Border {
                color: cosmic::iced::Color::WHITE,
                radius: cosmic.corner_radii.radius_xs.into(),
                width: 3.0,
            },
            shadow: Default::default(),
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
            border: Border {
                color: cosmic::iced::Color::BLACK,
                radius: cosmic.corner_radii.radius_0.into(),
                width: 0.0,
            },
            shadow: Default::default(),
        }
    })
}
