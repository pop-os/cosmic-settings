// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use apply::Apply;
use cosmic::iced::widget::{column, row};
use cosmic::iced_core::Length;
use cosmic::widget::divider::horizontal;
use cosmic::widget::{button, container, horizontal_space, settings, spin_button, text};
use cosmic::Element;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

// TODO integrate with settings backend
#[derive(Default)]
pub struct Page {
    autoswitch: bool,
    accent_window_hint: bool,
    frosted: bool,
    window_hint_size: u16,
    gap_size: u16,
    can_reset: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Autoswitch(bool),
    AccentWindowHint(bool),
    Frosted(bool),
    WindowHintSize(spin_button::Message),
    GapSize(spin_button::Message),
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Autoswitch(enabled) => {
                self.autoswitch = true;
            }
            Message::AccentWindowHint(enabled) => {
                self.accent_window_hint = true;
            }
            Message::Frosted(enabled) => {
                self.frosted = enabled;
            }
            Message::WindowHintSize(msg) => {
                // self.window_hint_size = s;
            }
            Message::GapSize(msg) => {}
        }
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
            sections.insert(reset_button()),
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
            // 0
            fl!("auto-switch"),
            fl!("auto-switch", "desc"),
            //2
            fl!("accent-color"),
            //3
            fl!("app-background"),
            //4
            fl!("container-background"),
            fl!("container-background", "desc"),
            fl!("container-background", "desc-detail"),
            fl!("container-background", "reset"),
            // 8
            fl!("text-tint"),
            fl!("text-tint", "desc"),
            // 10
            fl!("control-tint"),
            fl!("control-tint", "desc"),
            // 12
            fl!("window-hint-accent"),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(container(text("TODO")).width(Length::Fill))
                .add(
                    settings::item::builder(&descriptions[0])
                        .description(&descriptions[1])
                        .toggler(page.autoswitch, Message::Autoswitch),
                )
                .add(column![text(&descriptions[2]), container(text("TODO",))])
                .add(settings::item::builder(&descriptions[3]).control(container(text("TODO"))))
                .add(
                    settings::item::builder(&descriptions[4])
                        .description(&descriptions[5])
                        .control(container(text("TODO"))),
                )
                .add(
                    settings::item::builder(&descriptions[8])
                        .description(&descriptions[9])
                        .control(container(text("TODO"))),
                )
                .add(
                    settings::item::builder(&descriptions[10])
                        .description(&descriptions[11])
                        .control(container(text("TODO"))),
                )
                .add(
                    settings::item::builder(&descriptions[12])
                        .toggler(page.accent_window_hint, Message::AccentWindowHint),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
pub fn style() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("style"))
        .descriptions(vec![
            fl!("style", "round"),
            fl!("style", "slightly-round"),
            fl!("style", "square"),
            fl!("frosted"),
            fl!("frosted", "desc"),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(container(text("TODO Toggle Image buttons for roundness")).width(Length::Fill))
                .add(
                    settings::item::builder(&descriptions[3])
                        .description(&descriptions[4])
                        .toggler(page.frosted, Message::Frosted),
                )
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
                .add(
                    settings::item::builder(&descriptions[0])
                        .control(cosmic::widget::spin_button("", Message::WindowHintSize)),
                )
                .add(
                    settings::item::builder(&descriptions[1])
                        .control(cosmic::widget::spin_button("", Message::GapSize)),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[allow(clippy::too_many_lines)]
pub fn reset_button() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![fl!("reset-default")])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;
            if page.can_reset {
                row![button(text(&descriptions[0]))].apply(Element::from)
            } else {
                horizontal_space(1).apply(Element::from)
            }
            .map(crate::pages::Message::Appearance)
        })
}
impl page::AutoBind<crate::pages::Message> for Page {}
