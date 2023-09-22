// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use apply::Apply;
use cosmic::iced_core::Length;
use cosmic::widget::{container, settings, text};
use cosmic::Element;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page;

#[derive(Debug, Copy, Clone)]
pub enum Message {}

impl Page {
    pub fn update(&mut self, message: Message) {
        // TODO
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(mode_and_colors()),
            sections.insert(style()),
            sections.insert(window_management()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("appearance", "preferences-pop-desktop-appearance-symbolic")
            .title(fl!("appearance"))
            .description(fl!("appearance", "desc"))
    }
}

#[allow(clippy::too_many_lines)]
pub fn mode_and_colors() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("mode-and-colors"))
        .descriptions(vec![
            fl!("auto-switch"),
            fl!("auto-switch", "desc"),
            fl!("accent-color"),
            fl!("app-background"),
            fl!("container-background"),
            fl!("container-background", "desc-detail"),
            fl!("container-background", "reset"),
            fl!("container-background", "desc"),
            fl!("text-tint"),
            fl!("text-tint", "desc"),
            fl!("control-tint"),
            fl!("control-tint", "desc"),
            fl!("window-hint-accent"),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(container(text("TODO")).width(Length::Fill))
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
pub fn style() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("style"))
        .descriptions(vec![
            fl!("style"),
            fl!("style", "round"),
            fl!("style", "slightly-round"),
            fl!("style", "square"),
            fl!("frosted"),
            fl!("frosted", "desc"),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(container(text("TODO")).width(Length::Fill))
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
pub fn window_management() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("window-management"))
        .descriptions(vec![
            fl!("window-management", "active-hint"),
            fl!("window-management", "gaps"),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(container(text("TODO")).width(Length::Fill))
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

impl page::AutoBind<crate::pages::Message> for Page {}
