// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, row, settings, text};
use cosmic::{Apply, Element};
use cosmic_comp_config::input::AccelProfile;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slab::Slab;
use slotmap::SlotMap;

use super::Message;

pub fn default_primary_button() -> cosmic::widget::segmented_button::SingleSelectModel {
    let mut model = cosmic::widget::segmented_button::SingleSelectModel::builder()
        .insert(|b| b.text(fl!("primary-button", "left")))
        .insert(|b| b.text(fl!("primary-button", "right")))
        .build();
    model.activate_position(0);
    model
}

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(mouse()), sections.insert(scrolling())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("mouse", "input-mouse-symbolic")
            .title(fl!("mouse"))
            .description(fl!("mouse", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn mouse() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let mouse_acceleration = descriptions.insert(fl!("mouse", "acceleration"));
    let mouse_speed = descriptions.insert(fl!("mouse", "speed"));
    let primary_button = descriptions.insert(fl!("primary-button"));
    let acceleration_desc = descriptions.insert(fl!("acceleration-desc"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |binder, _page, section| {
            let descriptions = &section.descriptions;
            let input = binder.page::<super::Page>().expect("input page not found");

            settings::section()
                .title(&section.title)
                .add(settings::flex_item(
                    &descriptions[primary_button],
                    cosmic::widget::segmented_control::horizontal(&input.primary_button)
                        .minimum_button_width(0)
                        .on_activate(|x| Message::PrimaryButtonSelected(x, false)),
                ))
                .add(
                    settings::item::builder(&descriptions[mouse_speed]).flex_control({
                        let value = (input
                            .input_default
                            .acceleration
                            .as_ref()
                            .map_or(0.0, |x| x.speed)
                            + 0.81)
                            * 70.71;

                        let slider = widget::slider(0.0..=100.0, value, |value| {
                            Message::SetMouseSpeed((value / 70.71) - 0.81, false)
                        })
                        .width(Length::Fill)
                        .breakpoints(&[50.0])
                        .apply(widget::container)
                        .max_width(250);

                        row::with_capacity(2)
                            .align_y(Alignment::Center)
                            .spacing(8)
                            .push(
                                text::body(format!("{:.0}", value.round()))
                                    .width(Length::Fixed(22.0))
                                    .align_x(Alignment::Center),
                            )
                            .push(slider)
                    }),
                )
                .add(
                    settings::item::builder(&descriptions[mouse_acceleration])
                        .description(&descriptions[acceleration_desc])
                        .toggler(
                            input
                                .input_default
                                .acceleration
                                .as_ref()
                                .is_none_or(|x| x.profile == Some(AccelProfile::Adaptive)),
                            |x| Message::SetAcceleration(x, false),
                        ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn scrolling() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let natural = descriptions.insert(fl!("scrolling", "natural"));
    let natural_desc = descriptions.insert(fl!("scrolling", "natural-desc"));
    let scroll_speed = descriptions.insert(fl!("scrolling", "speed"));

    Section::default()
        .title(fl!("scrolling"))
        .descriptions(descriptions)
        .view::<Page>(move |binder, _page, section| {
            let descriptions = &section.descriptions;
            let input = binder.page::<super::Page>().expect("input page not found");

            settings::section()
                .title(&section.title)
                .add(settings::flex_item(&descriptions[scroll_speed], {
                    let value = input
                        .input_default
                        .scroll_config
                        .as_ref()
                        .and_then(|x| x.scroll_factor)
                        .unwrap_or(1.)
                        .log(2.)
                        * 10.0
                        + 50.0;

                    let slider = widget::slider(1.0..=100.0, value, |value| {
                        Message::SetScrollFactor(2f64.powf((value - 50.0) / 10.0), false)
                    })
                    .width(Length::Fill)
                    .breakpoints(&[50.0])
                    .apply(widget::container)
                    .max_width(250);

                    row::with_capacity(2)
                        .align_y(Alignment::Center)
                        .spacing(8)
                        .push(
                            text::body(format!("{:.0}", value.round()))
                                .width(Length::Fixed(22.0))
                                .align_x(Alignment::Center),
                        )
                        .push(slider)
                }))
                .add(
                    settings::item::builder(&descriptions[natural])
                        .description(&descriptions[natural_desc])
                        .toggler(
                            input
                                .input_default
                                .scroll_config
                                .as_ref()
                                .and_then(|x| x.natural_scroll)
                                .unwrap_or(false),
                            |x| Message::SetNaturalScroll(x, false),
                        ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}
