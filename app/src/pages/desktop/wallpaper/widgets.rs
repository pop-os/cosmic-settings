// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::Message;
use cosmic::iced_core::{self, gradient::Linear, Background, BorderRadius, Color, Degrees, Length};
use cosmic::iced_runtime::core::image::Handle as ImageHandle;
use cosmic::prelude::*;
use cosmic::widget::{button, container, image, space};
use cosmic::{iced, Element};
use cosmic_settings_desktop::wallpaper;
use slotmap::DefaultKey;

const COLOR_WIDTH: u16 = 70;
const COLUMN_SPACING: u16 = 12;
const ROW_SPACING: u16 = 16;

/// A button for selecting a color or gradient.
pub fn color_button(color: wallpaper::Color) -> Element<'static, Message> {
    button(color_image(color.clone(), COLOR_WIDTH, COLOR_WIDTH, 8.0))
        .padding(0)
        .style(button::Style::IconVertical)
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
    container(space::Space::new(width, height))
        .style(cosmic::theme::Container::custom(move |_theme| {
            container::Appearance {
                icon_color: None,
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
    let mut vec = Vec::with_capacity(wallpaper::DEFAULT_COLORS.len());

    for color in wallpaper::DEFAULT_COLORS {
        vec.push(color_button(color.clone()));
    }

    flex_select_row(vec)
}

/// Background selection list
pub fn wallpaper_select_options(page: &super::Page) -> Element<Message> {
    let mut vec = Vec::with_capacity(page.selection.selection_handles.len());

    for (id, handle) in &page.selection.selection_handles {
        vec.push(wallpaper_button(handle, id));
    }

    flex_select_row(vec)
}

fn flex_select_row(elements: Vec<Element<Message>>) -> Element<Message> {
    cosmic::widget::flex_row(elements)
        .column_spacing(COLUMN_SPACING)
        .row_spacing(ROW_SPACING)
        .apply(container)
        .width(Length::Fill)
        .center_x()
        .into()
}

fn wallpaper_button(handle: &ImageHandle, id: DefaultKey) -> Element<Message> {
    let image = image(handle.clone()).apply(iced::Element::from);

    button(image)
        .padding(0)
        .style(cosmic::theme::Button::Transparent)
        .on_press(Message::Select(id))
        .into()
}
