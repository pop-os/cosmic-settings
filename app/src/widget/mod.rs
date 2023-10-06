// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use apply::Apply;
use cosmic::iced::Length;
use cosmic::widget::{
    button, column, container, divider, horizontal_space, row, settings, text, vertical_space,
};
use cosmic::Element;
use cosmic_settings_page as page;

#[must_use]
pub fn search_header<Message>(
    pages: &page::Binder<Message>,
    page: page::Entity,
) -> cosmic::Element<crate::Message> {
    let page_meta = &pages.info[page];

    let mut column_children = Vec::with_capacity(4);

    if let Some(parent) = page_meta.parent {
        let parent_meta = &pages.info[parent];

        column_children.push(
            text(parent_meta.title.as_str())
                .size(14)
                .apply(container)
                .padding([0, 0, 0, 6])
                .into(),
        );
    }

    column_children.push(
        crate::widget::search_page_link(&page_meta.title)
            .on_press(crate::Message::Page(page))
            .into(),
    );

    column_children.push(vertical_space(Length::Fixed(8.)).into());
    column_children.push(divider::horizontal::heavy().into());

    column::with_children(column_children).into()
}

pub fn search_page_link<Message: 'static>(title: &str) -> button::TextButton<Message> {
    button::text(title).style(button::Style::Link)
}

#[must_use]
pub fn page_title<Message: 'static>(page: &page::Info) -> Element<Message> {
    row::with_capacity(2)
        .push(text::title3(page.title.as_str()))
        .push(horizontal_space(Length::Fill))
        .into()
}

#[must_use]
pub fn unimplemented_page<Message: 'static>() -> Element<'static, Message> {
    settings::view_section("")
        .add(text("We haven't created that panel yet, and/or it is using a similar idea as current Pop! designs."))
        .into()
}

#[must_use]
pub fn display_container<'a, Message: 'a>(widget: Element<'a, Message>) -> Element<'a, Message> {
    let display = container(widget)
        .style(crate::theme::display_container_screen())
        .apply(container)
        .padding(4)
        .style(crate::theme::display_container_frame());

    row::with_capacity(3)
        .push(horizontal_space(Length::Fill))
        .push(display)
        .push(horizontal_space(Length::Fill))
        .padding([0, 0, 8, 0])
        .into()
}
