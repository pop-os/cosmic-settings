// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::{Message, NightLight};
use crate::pages;
use cosmic::iced_core::{Alignment, Length, Padding};
use cosmic::prelude::CollectionWidget;
use cosmic::widget::{button, column, icon, list_column, row, toggler};
use cosmic::{Apply, Command, Element};
use std::sync::Arc;

pub fn view(
    mode: &'static str,
    description: &'static str,
    button: Option<(&'static str, Message)>,
) -> Element<'static, Message> {
    let theme = cosmic::theme::active();
    let theme = theme.cosmic();
    let has_checkmark = button.is_none();

    let content = column::with_capacity(3)
        .padding(Padding::from([theme.space_xxs(), theme.space_l()]))
        .push(cosmic::widget::text::body(mode))
        .push(cosmic::widget::text::caption(description))
        .push(cosmic::widget::Space::new(Length::Fill, 12))
        .push_maybe(button.map(|(text, message)| {
            button::text(text)
                .style(cosmic::theme::Button::Link)
                .trailing_icon(icon::from_name("go-next-symbolic").size(16))
                .padding(0)
                .on_press(message)
        }));

    if has_checkmark {
        row::with_capacity(2)
            .align_items(Alignment::Center)
            .push(content)
            .push(icon::from_name("object-select-symbolic").size(24))
            .apply(Element::from)
            .apply(cosmic::widget::list::container)
            .into()
    } else {
        cosmic::widget::list::container(content).into()
    }
}

impl super::Page {
    pub fn night_light_view(&self) -> Element<pages::Message> {
        let mut container = list_column();

        // Displays the night light status, and a button for configuring it.
        container = container.add(
            cosmic::widget::settings::item::builder(&*super::text::NIGHT_LIGHT)
                .description(&*super::text::NIGHT_LIGHT_DESCRIPTION)
                .control(
                    row()
                        .align_items(Alignment::Center)
                        .push(toggler(None, self.config.night_light_enabled, |enable| {
                            Message::NightLight(NightLight::Toggle(enable))
                        }))
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
