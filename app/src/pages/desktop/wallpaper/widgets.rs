// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::Message;
use apply::Apply;
use cosmic::iced_core::{self, gradient::Linear, Background, BorderRadius, Color, Degrees};
use cosmic::iced_core::{alignment, Length};
use cosmic::iced_runtime::core::image::Handle as ImageHandle;
use cosmic::{iced, Element};
use cosmic_settings_desktop::wallpaper;
use slotmap::DefaultKey;

const COLOR_WIDTH: u16 = 70;
const WALLPAPER_WIDTH: u16 = 158;

const COLUMN_SPACING: u16 = 12;
const ROW_SPACING: u16 = 16;

/// A button for selecting a color or gradient.
pub fn color_button(color: wallpaper::Color) -> Element<'static, Message> {
    iced::widget::button(color_image(color.clone(), COLOR_WIDTH, COLOR_WIDTH, 8.0))
        .padding(0)
        .style(cosmic::theme::Button::Transparent)
        .on_press(Message::ColorSelect(color))
        .into()
}

/// A sized container that's filled with a color or gradient.
pub fn color_image(
    color: wallpaper::Color,
    width: u16,
    height: u16,
    border_radius: f32,
) -> Element<'static, Message> {
    iced::widget::container(iced::widget::space::Space::new(width, height))
        .style(cosmic::theme::Container::custom(move |_theme| {
            iced::widget::container::Appearance {
                text_color: None,
                background: Some(match &color {
                    wallpaper::Color::Single([r, g, b]) => {
                        Background::Color(Color::from_rgb(*r, *g, *b))
                    }

                    wallpaper::Color::Gradient(wallpaper::Gradient { colors, radius }) => {
                        let stop_increment = 1.0 / (colors.len() - 1) as f32;
                        let mut stop = 0.0;

                        let mut linear = Linear::new(Degrees(*radius));

                        for &[r, g, b] in &**colors {
                            linear = linear.add_stop(stop, iced::Color::from_rgb(r, g, b));
                            stop += stop_increment;
                        }

                        Background::Gradient(iced_core::Gradient::Linear(linear))
                    }
                }),
                border_radius: BorderRadius::from(border_radius),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }))
        .padding(0)
        .into()
}

/// Color selection list
pub fn color_select_options() -> Element<'static, Message> {
    flex_select_row(|vec, _size| {
        let elements = wallpaper::DEFAULT_COLORS.iter().cloned().map(color_button);

        vec.extend(elements);

        COLOR_WIDTH
    })
}

/// Background selection list
pub fn wallpaper_select_options(page: &super::Page) -> Element<Message> {
    flex_select_row(move |vec, _size| {
        let elements = page
            .selection
            .selection_handles
            .iter()
            .map(|(id, handle)| wallpaper_button(handle, id));

        vec.extend(elements);

        WALLPAPER_WIDTH
    })
}

fn flex_select_row<'a>(
    elements: impl Fn(&mut Vec<Element<'a, Message>>, iced_core::Size) -> u16 + 'a,
) -> Element<'a, Message> {
    cosmic::widget::flex_row(elements)
        .column_spacing(COLUMN_SPACING)
        .row_spacing(ROW_SPACING)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .into()
}

fn wallpaper_button(handle: &ImageHandle, id: DefaultKey) -> Element<Message> {
    let image = iced::widget::image(handle.clone()).apply(iced::Element::from);

    iced::widget::button(image)
        .padding(0)
        .style(cosmic::theme::Button::Transparent)
        .on_press(Message::Select(id))
        .into()
}
