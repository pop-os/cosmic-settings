// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::{widget::horizontal_space, Length};
use cosmic::widget::settings;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(behavior()),
            sections.insert(multi_behavior()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("workspaces", "preferences-pop-desktop-workspaces-symbolic")
            .title(fl!("workspaces"))
            .description(fl!("workspaces", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn behavior() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("workspaces-behavior"))
        .descriptions(vec![
            fl!("workspaces-behavior", "dynamic"),
            fl!("workspaces-behavior", "fixed"),
        ])
        .view::<Page>(|_binder, _page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(settings::item(
                    &descriptions[0],
                    horizontal_space(Length::Fill),
                ))
                .add(settings::item(
                    &descriptions[1],
                    horizontal_space(Length::Fill),
                ))
                .into()
        })
}

fn multi_behavior() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("workspaces-multi-behavior"))
        .descriptions(vec![
            fl!("workspaces-multi-behavior", "span"),
            fl!("workspaces-multi-behavior", "separate"),
        ])
        .view::<Page>(|_binder, _page, section| {
            let descriptions = &section.descriptions;
            settings::view_section(&section.title)
                .add(settings::item(
                    &descriptions[0],
                    horizontal_space(Length::Fill),
                ))
                .add(settings::item(
                    &descriptions[1],
                    horizontal_space(Length::Fill),
                ))
                .into()
        })
}
