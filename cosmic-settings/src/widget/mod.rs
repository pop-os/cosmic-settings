// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::borrow::Cow;

use cosmic::cosmic_theme::Spacing;
use cosmic::iced::{alignment, Length};
use cosmic::iced_core::text::Wrap;
use cosmic::prelude::CollectionWidget;
use cosmic::widget::color_picker::ColorPickerUpdate;
use cosmic::widget::{
    self, button, column, container, divider, horizontal_space, icon, row, settings, text,
    vertical_space, ColorPickerModel,
};
use cosmic::{theme, Apply, Element};
use cosmic_settings_page as page;

pub fn color_picker_context_view<'a, Message: Clone + 'static>(
    description: Option<Cow<'static, str>>,
    reset: Cow<'static, str>,
    on_update: fn(ColorPickerUpdate) -> Message,
    model: &'a ColorPickerModel,
) -> Element<'a, Message> {
    let theme = theme::active();
    let spacing = &theme.cosmic().spacing;

    cosmic::widget::column()
        .push_maybe(description.map(|description| text(description).width(Length::Fill)))
        .push(
            model
                .builder(on_update)
                .reset_label(reset)
                .height(Length::Fixed(158.0))
                .build(
                    fl!("recent-colors"),
                    fl!("copy-to-clipboard"),
                    fl!("copied-to-clipboard"),
                )
                .apply(container)
                .width(Length::Fixed(248.0))
                .align_x(alignment::Horizontal::Center)
                .apply(container)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center),
        )
        .padding(spacing.space_l)
        .align_items(cosmic::iced_core::Alignment::Center)
        .spacing(spacing.space_m)
        .width(Length::Fill)
        .apply(Element::from)
}

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
            text::body(parent_meta.title.as_str())
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
    settings::section().title("")
        .add(text::body("We haven't created that panel yet, and/or it is using a similar idea as current Pop! designs."))
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

#[must_use]
pub fn page_list_item<'a, Message: 'static + Clone>(
    title: impl Into<Cow<'a, str>>,
    description: impl Into<Cow<'a, str>>,
    info: impl Into<Cow<'a, str>>,
    icon: &'a str,
    message: Message,
) -> Element<'a, Message> {
    let Spacing {
        space_xxs,
        space_s,
        space_m,
        ..
    } = cosmic::theme::active().cosmic().spacing;

    let mut builder = cosmic::widget::settings::item::builder(title);

    let description = description.into();

    let info = info.into();

    if !description.is_empty() {
        builder = builder.description(description);
    }

    builder
        .icon(container(icon::from_name(icon).size(20)).padding(8))
        .control(
            row::with_capacity(2)
                .push(text::body(info))
                .push(container(icon::from_name("go-next-symbolic").size(20)).padding(8))
                .align_items(alignment::Alignment::Center),
        )
        .padding(0)
        .spacing(space_xxs)
        .apply(container)
        .padding([space_s, space_m])
        .align_x(alignment::Horizontal::Center)
        .style(theme::Container::List)
        .apply(button::custom)
        .padding(0)
        .style(theme::Button::Transparent)
        .on_press(message)
        .into()
}

#[must_use]
pub fn sub_page_header<'a, Message: 'static + Clone>(
    sub_page: &'a str,
    parent_page: &'a str,
    on_press: Message,
) -> Element<'a, Message> {
    let previous_button = button::icon(icon::from_name("go-previous-symbolic"))
        .extra_small()
        .padding(0)
        .label(parent_page)
        .spacing(4)
        .style(button::Style::Link)
        .on_press(on_press);

    let sub_page_header = row::with_capacity(2).push(text::title3(sub_page));

    column::with_capacity(2)
        .push(previous_button)
        .push(sub_page_header)
        .spacing(6)
        .width(Length::Shrink)
        .into()
}

pub fn go_next_item<Msg: Clone + 'static>(description: &str, msg: Msg) -> cosmic::Element<'_, Msg> {
    settings::item_row(vec![
        text::body(description).wrap(Wrap::Word).into(),
        horizontal_space(Length::Fill).into(),
        icon::from_name("go-next-symbolic").size(16).icon().into(),
    ])
    .apply(widget::container)
    .style(cosmic::theme::Container::List)
    .apply(button::custom)
    .style(theme::Button::Transparent)
    .on_press(msg)
    .into()
}

pub fn go_next_with_item<'a, Msg: Clone + 'static>(
    description: &'a str,
    item: impl Into<cosmic::Element<'a, Msg>>,
    msg: Msg,
) -> cosmic::Element<'_, Msg> {
    settings::item_row(vec![
        text::body(description).wrap(Wrap::Word).into(),
        horizontal_space(Length::Fill).into(),
        widget::row::with_capacity(2)
            .push(item)
            .push(icon::from_name("go-next-symbolic").size(16).icon())
            .align_items(alignment::Alignment::Center)
            .spacing(cosmic::theme::active().cosmic().spacing.space_s)
            .into(),
    ])
    .apply(widget::container)
    .style(cosmic::theme::Container::List)
    .apply(button::custom)
    .style(theme::Button::Transparent)
    .on_press(msg)
    .into()
}
