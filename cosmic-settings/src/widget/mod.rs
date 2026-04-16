// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::borrow::Cow;
use std::rc::Rc;

use cosmic::cosmic_theme::Spacing;
use cosmic::iced::core::text::Wrapping;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::color_picker::ColorPickerUpdate;
use cosmic::widget::{
    self, ColorPickerModel, button, column, container, divider, icon, list, row, settings,
    space::{horizontal, vertical},
    text,
};
use cosmic::{Apply, Element, theme};
use cosmic_settings_page as page;

pub fn color_picker_context_view<'a, Message: Clone + 'static>(
    description: Option<Cow<'static, str>>,
    reset: Cow<'static, str>,
    on_update: fn(ColorPickerUpdate) -> Message,
    model: &'a ColorPickerModel,
) -> Element<'a, Message> {
    let theme = theme::active();
    let spacing = &theme.cosmic().spacing;

    let description = description.map(text::caption);

    let color_picker = model
        .builder(on_update)
        .reset_label(reset)
        .height(Length::Fixed(158.0))
        .build(
            fl!("recent-colors"),
            fl!("copy-to-clipboard"),
            fl!("copied-to-clipboard"),
        )
        .apply(container)
        .center_x(Length::Fixed(248.0))
        .apply(container)
        .center_x(Length::Fill);

    column::with_capacity(2)
        .push_maybe(description)
        .push(color_picker)
        .align_x(Alignment::Center)
        .spacing(spacing.space_m)
        .width(Length::Fill)
        .apply(Element::from)
}

#[must_use]
pub fn search_header<Message>(
    pages: &page::Binder<Message>,
    page: page::Entity,
) -> cosmic::Element<'_, crate::Message> {
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

    column_children.push(vertical().height(Length::Fixed(8.)).into());
    column_children.push(divider::horizontal::heavy().into());

    column::with_children(column_children).into()
}

pub fn search_page_link<Message: 'static>(title: &str) -> button::TextButton<'_, Message> {
    button::text(title).class(button::ButtonClass::Link)
}

#[must_use]
pub fn page_title<Message: 'static>(page: &page::Info) -> Element<'_, Message> {
    row::with_capacity(2)
        .push(text::title3(page.title.as_str()))
        .push(horizontal())
        .into()
}

#[must_use]
pub fn unimplemented_page<Message: Clone + 'static>() -> Element<'static, Message> {
    settings::section().title("")
        .add(text::body("We haven't created that panel yet, and/or it is using a similar idea as current Pop! designs."))
        .into()
}

#[must_use]
pub fn display_container<'a, Message: 'a>(widget: Element<'a, Message>) -> Element<'a, Message> {
    container(widget)
        .class(crate::theme::display_container_screen())
        .apply(container)
        .padding(4)
        .class(crate::theme::display_container_frame())
        .apply(container)
        .center_x(Length::Fill)
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
    } = cosmic::theme::spacing();

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
                .align_y(Alignment::Center),
        )
        .padding(0)
        .spacing(space_xxs)
        .apply(container)
        .padding([space_s, space_m])
        .align_x(Alignment::Center)
        .class(theme::Container::List)
        .width(Length::Fill)
        .apply(button::custom)
        .padding(0)
        .class(theme::Button::Transparent)
        .on_press(message)
        .width(Length::Fill)
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
        .class(button::ButtonClass::Link)
        .on_press(on_press);

    let sub_page_header = row::with_capacity(2).push(text::title3(sub_page));

    column::with_capacity(2)
        .push(previous_button)
        .push(sub_page_header)
        .spacing(6)
        .width(Length::Shrink)
        .into()
}

pub fn go_next_item<Msg: 'static>(
    description: &str,
    msg_opt: impl Into<Option<Msg>>,
) -> list::ListButton<'_, Msg> {
    settings::item_row(vec![
        text::body(description)
            .width(Length::Fill)
            .wrapping(Wrapping::Word)
            .into(),
        icon::from_name("go-next-symbolic").size(16).icon().into(),
    ])
    .apply(list::button)
    .on_press_maybe(msg_opt.into())
}

pub fn go_next_with_item<'a, Msg: 'static>(
    description: &'a str,
    item: impl Into<cosmic::Element<'a, Msg>>,
    msg_opt: impl Into<Option<Msg>>,
) -> list::ListButton<'a, Msg> {
    settings::item_row(vec![
        text::body(description).wrapping(Wrapping::Word).into(),
        horizontal().into(),
        row::with_capacity(2)
            .push(item)
            .push(icon::from_name("go-next-symbolic").size(16).icon())
            .align_y(Alignment::Center)
            .spacing(theme::spacing().space_s)
            .into(),
    ])
    .apply(list::button)
    .on_press_maybe(msg_opt.into())
}

pub fn selection_context_item<'a, Msg: 'static>(
    name: &'a str,
    selected: bool,
    msg_opt: impl Into<Option<Msg>>,
) -> list::ListButton<'a, Msg> {
    let svg_accent = Rc::new(|theme: &cosmic::Theme| widget::svg::Style {
        color: Some(theme.cosmic().accent_text_color().into()),
    });

    settings::item_row(vec![
        text::body(name)
            .class(if selected {
                theme::Text::Accent
            } else {
                theme::Text::Default
            })
            .wrapping(Wrapping::Word)
            .width(Length::Fill)
            .into(),
        if selected {
            icon::from_name("object-select-symbolic")
                .size(16)
                .icon()
                .class(theme::Svg::Custom(svg_accent.clone()))
                .into()
        } else {
            horizontal().width(16.).into()
        },
    ])
    .apply(list::button)
    .on_press_maybe(msg_opt.into())
}
