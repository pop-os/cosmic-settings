// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::Message;
use cosmic::iced::Radius;
use cosmic::iced_core::Border;
use cosmic::iced_core::{self, gradient::Linear, Background, Color, Degrees, Length};
use cosmic::iced_runtime::core::image::Handle as ImageHandle;
use cosmic::prelude::*;
use cosmic::widget::{button, container, space};
use cosmic::{iced, Element};
use cosmic_settings_wallpaper as wallpaper;
use slotmap::DefaultKey;
use std::sync::OnceLock;

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

                border: Border {
                    radius: border_radius.map_or_else(
                        || Radius::from(theme.cosmic().corner_radii.radius_s),
                        |br| br.into(),
                    ),
                    ..Default::default()
                },
                shadow: Default::default(),
            }
        }))
        .padding(0)
        .into()
}

/// Color selection list
///
/// Begin with removable custom colors, and chain with non-removable default colors.
#[must_use]
pub fn color_select_options(
    context: &super::Context,
    selected: Option<&wallpaper::Color>,
) -> Element<'static, Message> {
    static SORTED: OnceLock<Vec<wallpaper::Color>> = OnceLock::new();
    let sorted = &**SORTED.get_or_init(|| {
        let mut sorted = wallpaper::DEFAULT_COLORS.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).expect("Neither slices should have NaNs"));
        sorted
    });

    flex_select_row(
        context
            .custom_colors
            .iter()
            .rev()
            .filter_map(|color| {
                sorted
                    .binary_search_by(|probe| {
                        probe
                            .partial_cmp(color)
                            .expect("Neither slices should have NaNs")
                    })
                    .is_err()
                    .then_some((color, true))
            })
            .chain(wallpaper::DEFAULT_COLORS.iter().map(|color| (color, false)))
            .map(|(color, removable)| {
                color_button(
                    color.clone(),
                    removable,
                    selected.map_or(false, |selection| selection == color),
                )
            })
            .collect::<Vec<_>>(),
    )
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
