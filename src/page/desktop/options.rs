// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::Message;
use apply::Apply;
use cosmic::{
    iced::widget::horizontal_space,
    iced::Length,
    widget::{settings, toggler},
    Element,
};

use slotmap::SlotMap;

use crate::page::{self, section, Content, Section};

pub struct Page;

impl page::Page for Page {
    type Model = super::Model;

    const PERSISTENT_ID: &'static str = "desktop-options";

    fn page() -> page::Meta {
        page::Meta::default()
            .title(fl!("desktop-options"))
            .description(fl!("desktop-options", "desc"))
            .icon_name("video-display-symbolic")
    }

    #[allow(clippy::too_many_lines)]
    fn content(sections: &mut SlotMap<section::Entity, Section>) -> Option<Content> {
        Some(vec![
            sections.insert(super_key_action()),
            sections.insert(hot_corner()),
            sections.insert(top_panel()),
            sections.insert(window_controls()),
        ])
    }
}

pub fn hot_corner() -> Section {
    Section::new()
        .title(fl!("hot-corner"))
        .descriptions(vec![fl!("hot-corner", "top-left-corner")])
        .view_fn(|app, section| {
            let desktop = app
                .pages
                .resource::<super::Model>()
                .expect("desktop model is missing");
            let descriptions = &section.descriptions;
            settings::view_section(&section.title)
                .add(settings::item(
                    &descriptions[0],
                    toggler(None, desktop.top_left_hot_corner, |value| {
                        Message::TopLeftHotCorner(value)
                    }),
                ))
                .apply(Element::from)
                .map(crate::Message::Desktop)
        })
}

pub fn super_key_action() -> Section {
    Section::new()
        .title(fl!("super-key-action"))
        .descriptions(vec![
            fl!("super-key-action", "launcher"),
            fl!("super-key-action", "workspaces"),
            fl!("super-key-action", "applications"),
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
                .add(settings::item(
                    &descriptions[2],
                    horizontal_space(Length::Fill),
                ))
                .into()
        })
}

pub fn top_panel() -> Section {
    Section::new()
        .title(fl!("top-panel"))
        .descriptions(vec![
            fl!("top-panel", "workspaces"),
            fl!("top-panel", "applications"),
        ])
        .view_fn(|app, section| {
            let desktop = app
                .pages
                .resource::<super::Model>()
                .expect("desktop model is missing");
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(settings::item(
                    &descriptions[0],
                    toggler(
                        None,
                        desktop.show_workspaces_button,
                        Message::ShowWorkspacesButton,
                    ),
                ))
                .add(settings::item(
                    &descriptions[1],
                    toggler(
                        None,
                        desktop.show_applications_button,
                        Message::ShowApplicationsButton,
                    ),
                ))
                .apply(Element::from)
                .map(crate::Message::Desktop)
        })
}

pub fn window_controls() -> Section {
    Section::new()
        .title(fl!("window-controls"))
        .descriptions(vec![
            fl!("window-controls", "minimize"),
            fl!("window-controls", "maximize"),
        ])
        .view_fn(|app, section| {
            let desktop = app
                .pages
                .resource::<super::Model>()
                .expect("desktop model is missing");
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(settings::item(
                    &descriptions[0],
                    toggler(
                        None,
                        desktop.show_minimize_button,
                        Message::ShowMinimizeButton,
                    ),
                ))
                .add(settings::item(
                    &descriptions[1],
                    toggler(
                        None,
                        desktop.show_maximize_button,
                        Message::ShowMaximizeButton,
                    ),
                ))
                .apply(Element::from)
                .map(crate::Message::Desktop)
        })
}
