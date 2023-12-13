// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::Message;
use cosmic::iced_core::{self, gradient::Linear, Background, BorderRadius, Color, Degrees, Length};
use cosmic::iced_runtime::core::image::Handle as ImageHandle;
use cosmic::prelude::*;
use cosmic::widget::{button, container, space};
use cosmic::{iced, Element};
use cosmic_settings_desktop::wallpaper;
use slotmap::DefaultKey;

const COLOR_WIDTH: u16 = 70;
const COLUMN_SPACING: u16 = 12;
const ROW_SPACING: u16 = 16;

/// A button for selecting a color or gradient.
#[must_use]
pub fn color_button(
    color: wallpaper::Color,
    removable: bool,
    selected: bool,
) -> Element<'static, Message> {
    let content = color_image(color.clone(), COLOR_WIDTH, COLOR_WIDTH, None);
    let on_remove = if removable {
        Some(Message::ColorRemove(color.clone()))
    } else {
        None
    };

    button::custom_image_button(content, on_remove)
        .padding(0)
        .selected(selected)
        .style(button::Style::Image)
        .on_press(Message::ColorSelect(color))
        .into()
}

/// A sized container that's filled with a color or gradient.
#[must_use]
pub fn color_image<'a, M: 'a>(
    color: wallpaper::Color,
    width: u16,
    height: u16,
    border_radius: Option<f32>,
) -> Element<'a, M> {
    container(space::Space::new(width, height))
        .style(cosmic::theme::Container::custom(move |theme| {
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
                border_radius: border_radius
                    .map(|br| br.into())
                    .unwrap_or_else(|| theme.cosmic().corner_radii.radius_s.into()),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }))
        .padding(0)
        .into()
}

/// Color selection list
#[must_use]
pub fn color_select_options(
    context: &super::Context,
    selected: Option<&wallpaper::Color>,
) -> Element<'static, Message> {
    let mut vec = Vec::with_capacity(wallpaper::DEFAULT_COLORS.len());

    // Place removable custom colors first
    for color in context.custom_colors.iter().rev() {
        vec.push(color_button(
            color.clone(),
            true,
            selected.map_or(false, |selection| selection == color),
        ));
    }

    // Then non-removable default colors
    for color in wallpaper::DEFAULT_COLORS {
        vec.push(color_button(
            color.clone(),
            false,
            selected.map_or(false, |selection| selection == color),
        ));
    }

    flex_select_row(vec)
}

/// Background selection list
#[must_use]
pub fn wallpaper_select_options(
    page: &super::Page,
    selected: Option<DefaultKey>,
    show_custom_images: bool,
) -> Element<Message> {
    let mut vec = Vec::with_capacity(page.selection.selection_handles.len());

    if show_custom_images {
        // Place removable custom images first
        for id in page.selection.custom_images.iter().rev() {
            let handle = &page.selection.selection_handles[*id];

            vec.push(wallpaper_button(
                handle,
                *id,
                true,
                selected.map_or(false, |selection| id == &selection),
            ));
        }
    }

    // Then place non-removable images from the current folder
    for (id, handle) in &page.selection.selection_handles {
        if page.selection.is_custom.contains_key(id) {
            continue;
        }

        vec.push(wallpaper_button(
            handle,
            id,
            false,
            selected.map_or(false, |selection| id == selection),
        ));
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

fn wallpaper_button(
    handle: &ImageHandle,
    id: DefaultKey,
    removable: bool,
    selected: bool,
) -> Element<Message> {
    cosmic::widget::button::image(handle.clone())
        .selected(selected)
        .on_press(Message::Select(id))
        .on_remove_maybe(if removable {
            Some(Message::ImageRemove(id))
        } else {
            None
        })
        .into()
}
