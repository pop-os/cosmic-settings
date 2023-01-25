// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::{widget::horizontal_space, Length};
use cosmic::widget::settings;
use slotmap::SlotMap;

use crate::page::{self, section, Content, Section};

pub struct Page;

impl page::Page for Page {
    type Model = super::Model;

    const PERSISTENT_ID: &'static str = "workspaces";

    fn page() -> page::Meta {
        page::Meta::default()
            .title(fl!("workspaces"))
            .description(fl!("workspaces", "desc"))
            .icon_name("preferences-pop-desktop-workspaces-symbolic")
    }

    fn content(sections: &mut SlotMap<section::Entity, Section>) -> Option<Content> {
        Some(vec![
            sections.insert(behavior()),
            sections.insert(multi_behavior()),
        ])
    }
}

fn behavior() -> Section {
    Section::new()
        .title(fl!("workspaces-behavior"))
        .descriptions(vec![
            fl!("workspaces-behavior", "dynamic"),
            fl!("workspaces-behavior", "fixed"),
        ])
        .view_fn(|app, section| {
            let _desktop = app
                .pages
                .resource::<super::Model>()
                .expect("desktop model is missing");

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

fn multi_behavior() -> Section {
    Section::new()
        .title(fl!("workspaces-multi-behavior"))
        .descriptions(vec![
            fl!("workspaces-multi-behavior", "span"),
            fl!("workspaces-multi-behavior", "separate"),
        ])
        .view_fn(|app, section| {
            let _desktop = app
                .pages
                .resource::<super::Model>()
                .expect("desktop model is missing");
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
