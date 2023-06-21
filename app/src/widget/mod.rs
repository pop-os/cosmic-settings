// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use apply::Apply;
use cosmic::iced::{
    self,
    widget::{button, column, container, horizontal_space, row, vertical_space, Button},
    Length,
};
use cosmic::widget::{divider, icon, list, settings, text};
use cosmic::{theme, Element};
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

    column(column_children).into()
}

#[must_use]
pub fn search_page_link<Message: 'static>(title: &str) -> Button<Message, cosmic::Renderer> {
    text(title)
        .size(24)
        .horizontal_alignment(iced::alignment::Horizontal::Left)
        .apply(button)
        .style(cosmic::theme::Button::Link)
}

#[must_use]
pub fn page_title<Message: 'static>(page: &page::Info) -> Element<Message> {
    row!(
        text(page.title.as_str()).size(24),
        horizontal_space(Length::Fill)
    )
    .into()
}

#[must_use]
pub fn parent_page_button<'a, Message: Clone + 'static>(
    parent: &'a page::Info,
    sub_page: &'a page::Info,
    on_press: Message,
) -> Element<'a, Message> {
    column!(
        button(row!(
            icon("go-previous-symbolic", 20).style(theme::Svg::SymbolicLink),
            text(parent.title.as_str()).size(14),
        ))
        .padding(0)
        .style(theme::Button::Link)
        .on_press(on_press),
        row!(
            text(sub_page.title.as_str()).size(24),
            horizontal_space(Length::Fill),
        )
        .align_items(iced::alignment::Alignment::Center),
    )
    .spacing(6)
    .into()
}

#[must_use]
pub fn sub_page_button(entity: page::Entity, page: &page::Info) -> Element<page::Entity> {
    settings::item::builder(page.title.as_str())
        .description(page.description.as_str())
        .icon(icon(&*page.icon_name, 20).style(theme::Svg::Symbolic))
        .control(row!(
            horizontal_space(Length::Fill),
            icon("go-next-symbolic", 20).style(theme::Svg::Symbolic)
        ))
        .spacing(16)
        .apply(container)
        .padding([20, 24])
        .style(theme::Container::custom(list::column::style))
        .apply(button)
        .padding(0)
        .style(theme::Button::Transparent)
        .on_press(entity)
        .into()
}

#[must_use]
pub fn sub_page_section(entity: page::Entity, page: &page::Info) -> Element<page::Entity> {
    settings::item::builder(page.title.as_str())
        .description(page.description.as_str())
        .control(row!(
            horizontal_space(Length::Fill),
            icon("go-next-symbolic", 20).style(theme::Svg::Symbolic)
        ))
        .spacing(16)
        .apply(container)
        .padding([20, 24])
        .style(theme::Container::custom(list::column::style))
        .apply(button)
        .padding(0)
        .style(theme::Button::Transparent)
        .on_press(entity)
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
    row!(
        horizontal_space(Length::Fill),
        container(widget)
            .padding(4)
            .style(crate::theme::display_container()),
        horizontal_space(Length::Fill),
    )
    .padding([0, 0, 8, 0])
    .into()
}
