// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::Message;
use apply::Apply;
use cosmic::iced_core::{self, gradient::Linear, Background, BorderRadius, Color, Degrees, Length};
use cosmic::iced_runtime::core::image::Handle as ImageHandle;
use cosmic::{
    iced,
    iced_widget::{column, row},
    Element,
};
use cosmic_settings_desktop::wallpaper;
use slotmap::DefaultKey;

/// A button for selecting a color or gradient.
pub fn color_button(color: wallpaper::Color) -> Element<'static, Message> {
    iced::widget::button(color_image(color.clone(), 70, 70, 8.0))
        .width(Length::Fixed(71.0))
        .height(Length::Fixed(71.0))
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
        .into()
}

/// Color selection list
pub fn color_select_options() -> Element<'static, Message> {
    let mut color_column = Vec::with_capacity(wallpaper::DEFAULT_COLORS.len() / 8);
    let mut colors = wallpaper::DEFAULT_COLORS.iter();

    while let Some(color) = colors.next() {
        let mut color_row = Vec::with_capacity(8);

        color_row.push(color_button(color.clone()));

        for color in colors.by_ref().take(7) {
            color_row.push(color_button(color.clone()));
        }

        color_column.push(row(color_row).spacing(16).into());
    }

    column(color_column).spacing(12).padding(0).into()
}

/// Background selection list
pub fn wallpaper_select_options(page: &super::Page) -> Element<Message> {
    let mut image_column = Vec::with_capacity(page.selection.selection_handles.len() / 4);
    let mut image_handles = page.selection.selection_handles.iter();

    while let Some((id, handle)) = image_handles.next() {
        let mut image_row = Vec::with_capacity(4);

        image_row.push(wallpaper_button(handle, id));

        for (id, handle) in image_handles.by_ref().take(3) {
            image_row.push(wallpaper_button(handle, id));
        }

        image_column.push(row(image_row).spacing(16).into());
    }

    column(image_column).spacing(12).padding(0).into()
}

fn wallpaper_button(handle: &ImageHandle, id: DefaultKey) -> Element<Message> {
    let image = iced::widget::image(handle.clone()).apply(iced::Element::from);

    iced::widget::button(image)
        .width(Length::Fixed(158.0))
        .height(Length::Fixed(105.0))
        .style(cosmic::theme::Button::Transparent)
        .on_press(Message::Select(id))
        .into()
}
