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

use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(super_key_action()),
            sections.insert(hot_corner()),
            sections.insert(top_panel()),
            sections.insert(window_controls()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("desktop-options", "video-display-symbolic")
            .title(fl!("desktop-options"))
            .description(fl!("desktop-options", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

pub fn hot_corner() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("hot-corner"))
        .descriptions(vec![fl!("hot-corner", "top-left-corner")])
        .view::<Page>(|binder, _page, section| {
            let desktop = binder
                .page::<super::Page>()
                .expect("desktop page not found");

            let descriptions = &section.descriptions;
            settings::view_section(&section.title)
                .add(settings::item(
                    &descriptions[0],
                    toggler(None, desktop.top_left_hot_corner, |value| {
                        Message::TopLeftHotCorner(value)
                    }),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::Desktop)
        })
}

pub fn super_key_action() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("super-key-action"))
        .descriptions(vec![
            fl!("super-key-action", "launcher"),
            fl!("super-key-action", "workspaces"),
            fl!("super-key-action", "applications"),
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
                .add(settings::item(
                    &descriptions[2],
                    horizontal_space(Length::Fill),
                ))
                .into()
        })
}

pub fn top_panel() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("top-panel"))
        .descriptions(vec![
            fl!("top-panel", "workspaces"),
            fl!("top-panel", "applications"),
        ])
        .view::<Page>(|binder, _page, section| {
            let desktop = binder
                .page::<super::Page>()
                .expect("desktop page not found");
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
                .map(crate::pages::Message::Desktop)
        })
}

pub fn window_controls() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("window-controls"))
        .descriptions(vec![
            fl!("window-controls", "minimize"),
            fl!("window-controls", "maximize"),
        ])
        .view::<Page>(|binder, _page, section| {
            let desktop = binder
                .page::<super::Page>()
                .expect("desktop page not found");
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
                .map(crate::pages::Message::Desktop)
        })
}
