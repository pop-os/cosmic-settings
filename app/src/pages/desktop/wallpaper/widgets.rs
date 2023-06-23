// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::Message;
use apply::Apply;
use cosmic::iced_core::{self, gradient::Linear, Background, BorderRadius, Color, Degrees};
use cosmic::iced_runtime::core::image::Handle as ImageHandle;
use cosmic::{
    iced,
    iced_widget::{column, row},
    Element,
};
use cosmic_settings_desktop::wallpaper;
use slotmap::DefaultKey;

const COLOR_WIDTH: u16 = 70;
const WALLPAPER_WIDTH: f32 = 158.0;

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
    cosmic::iced::widget::responsive(|size| {
        let items_per_row =
            flex_row_items(size.width, COLOR_WIDTH as f32, ROW_SPACING as f32, 8) as usize;

        let mut color_column = Vec::with_capacity(wallpaper::DEFAULT_COLORS.len() / items_per_row);
        let mut colors = wallpaper::DEFAULT_COLORS.iter();

        while let Some(color) = colors.next() {
            let mut color_row = Vec::with_capacity(items_per_row);

            color_row.push(color_button(color.clone()));

            for color in colors.by_ref().take(items_per_row - 1) {
                color_row.push(color_button(color.clone()));
            }

            color_column.push(row(color_row).spacing(ROW_SPACING).into());
        }

        column(color_column)
            .spacing(COLUMN_SPACING)
            .padding(0)
            .apply(cosmic::iced::widget::container)
            .align_x(iced_core::alignment::Horizontal::Center)
            .width(size.width)
            .into()
    })
    .into()
}

fn flex_row_items(available: f32, item_width: f32, spacing: f32, max_items: u32) -> u32 {
    let mut items = 2;

    while items <= max_items && available >= (item_width + spacing) * items as f32 - spacing {
        items += 1;
    }

    items - 1
}

/// Background selection list
pub fn wallpaper_select_options(page: &super::Page) -> Element<Message> {
    cosmic::iced::widget::responsive(|size| {
        let items_per_row =
            flex_row_items(size.width, WALLPAPER_WIDTH, ROW_SPACING as f32, 4) as usize;

        let mut image_column =
            Vec::with_capacity(page.selection.selection_handles.len() / items_per_row);

        let mut image_handles = page.selection.selection_handles.iter();

        while let Some((id, handle)) = image_handles.next() {
            let mut image_row = Vec::with_capacity(items_per_row);

            image_row.push(wallpaper_button(handle, id));

            for (id, handle) in image_handles.by_ref().take(items_per_row - 1) {
                image_row.push(wallpaper_button(handle, id));
            }

            image_column.push(row(image_row).spacing(ROW_SPACING).into());
        }

        column(image_column)
            .spacing(COLUMN_SPACING)
            .padding(0)
            .apply(cosmic::iced::widget::container)
            .align_x(iced_core::alignment::Horizontal::Center)
            .width(size.width)
            .into()
    })
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
