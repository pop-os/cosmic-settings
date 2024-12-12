// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::{Message, NightLight};
use crate::pages;
use cosmic::iced_core::{Alignment, Length};
use cosmic::widget::{
    button, column, container, icon, list_column, row, settings, text, toggler, vertical_space,
};
use cosmic::{Apply, Element, Task};
use std::sync::Arc;

pub fn view(
    mode: &'static str,
    description: &'static str,
    button: Option<(&'static str, Message)>,
) -> Element<'static, Message> {
    let cosmic::cosmic_theme::Spacing {
        space_xxs, space_l, ..
    } = cosmic::theme::active().cosmic().spacing;
    let has_checkmark = button.is_none();

    let content = column::with_capacity(4)
        .padding([space_xxs, space_l])
        .push(text::body(mode))
        .push(text::caption(description))
        .push(vertical_space().height(12))
        .push_maybe(button.map(|(text, message)| {
            button::text(text)
                .class(cosmic::theme::Button::Link)
                .trailing_icon(icon::from_name("go-next-symbolic").size(16))
                .on_press(message)
        }));

    if has_checkmark {
        row::with_capacity(2)
            .align_y(Alignment::Center)
            .push(content)
            .push(icon::from_name("object-select-symbolic").size(24))
            .apply(Element::from)
            .apply(container)
            .class(cosmic::theme::Container::List)
            .padding(8)
            .width(Length::Fill)
            .into()
    } else {
        container(content)
            .class(cosmic::theme::Container::List)
            .padding(8)
            .width(Length::Fill)
            .into()
    }
}

impl super::Page {
    pub fn night_light_view(&self) -> Element<pages::Message> {
        let mut container = list_column();

        // Displays the night light status, and a button for configuring it.
        container = container.add(
            settings::item::builder(&*super::text::NIGHT_LIGHT)
                .description(&*super::text::NIGHT_LIGHT_DESCRIPTION)
                .control(
                    row()
                        .align_y(Alignment::Center)
                        .push(
                            toggler(self.config.night_light_enabled)
                                .on_toggle(Message::NightLight(NightLight::Toggle)),
                        )
                        .push(
                            button::icon(icon::from_name("go-next-symbolic"))
                                .extra_small()
                                .on_press(Message::NightLightContext),
                        ),
                ),
        );

        container
            .apply(Element::from)
            .map(crate::pages::Message::Displays)
    }
}
