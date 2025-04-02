// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    Apply, Element,
    cosmic_config::{self, ConfigGet, ConfigSet},
    iced::Length,
    widget::{self, text},
};
use cosmic_comp_config::{EavesdroppingKeyboardMode, XwaylandEavesdropping};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slab::Slab;
use slotmap::SlotMap;
use tracing::error;

#[derive(Clone, Debug)]
pub enum Message {
    SetXwaylandDescaling(bool),
    SetXwaylandKeyboardMode(EavesdroppingKeyboardMode),
    SetXwaylandMouseButtonMode(bool),
}

pub struct Page {
    comp_config: cosmic_config::Config,
    comp_config_descale_xwayland: bool,
    comp_config_xwayland_eavesdropping: XwaylandEavesdropping,
}

impl Default for Page {
    fn default() -> Self {
        let comp_config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let comp_config_descale_xwayland =
            comp_config.get("descale_xwayland").unwrap_or_else(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'descale_xwayland'");
                }

                false
            });
        let comp_config_xwayland_eavesdropping = comp_config
            .get("xwayland_eavesdropping")
            .unwrap_or_else(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'xwayland_eavesdropping'");
                }

                Default::default()
            });

        Self {
            comp_config,
            comp_config_descale_xwayland,
            comp_config_xwayland_eavesdropping,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(legacy_application_global_shortcuts()),
            sections.insert(legacy_application_scaling()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new(
            "legacy-applications",
            "preferences-X11-applications-symbolic",
        )
        .title(fl!("legacy-applications"))
        .description(fl!("legacy-applications", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SetXwaylandDescaling(descale) => {
                self.comp_config_descale_xwayland = descale;
                if let Err(err) = self
                    .comp_config
                    .set("descale_xwayland", self.comp_config_descale_xwayland)
                {
                    error!(?err, "Failed to set config 'descale_xwayland'");
                }
            }
            Message::SetXwaylandKeyboardMode(mode) => {
                self.comp_config_xwayland_eavesdropping.keyboard = mode;
                if let Err(err) = self.comp_config.set(
                    "xwayland_eavesdropping",
                    self.comp_config_xwayland_eavesdropping,
                ) {
                    error!(?err, "Failed to set config 'xwayland_eavesdropping'");
                }
            }
            Message::SetXwaylandMouseButtonMode(mode) => {
                self.comp_config_xwayland_eavesdropping.pointer = mode;
                if let Err(err) = self.comp_config.set(
                    "xwayland_eavesdropping",
                    self.comp_config_xwayland_eavesdropping,
                ) {
                    error!(?err, "Failed to set config 'xwayland_eavesdropping'");
                }
            }
        }
    }
}

pub fn legacy_application_global_shortcuts() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let desc = descriptions.insert(fl!("legacy-app-global-shortcuts", "desc"));
    let none = descriptions.insert(fl!("legacy-app-global-shortcuts", "none"));
    let modifiers = descriptions.insert(fl!("legacy-app-global-shortcuts", "modifiers"));
    let combination = descriptions.insert(fl!("legacy-app-global-shortcuts", "combination"));
    let all = descriptions.insert(fl!("legacy-app-global-shortcuts", "all"));
    let mouse = descriptions.insert(fl!("legacy-app-global-shortcuts", "mouse"));

    Section::default()
        .title(fl!("legacy-app-global-shortcuts"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let title = widget::text::body(&section.title).font(cosmic::font::bold());
            let description = widget::text::body(&section.descriptions[desc]);

            let content = widget::settings::section()
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        text::body(&section.descriptions[none]),
                        EavesdroppingKeyboardMode::None,
                        Some(page.comp_config_xwayland_eavesdropping.keyboard),
                        Message::SetXwaylandKeyboardMode,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        text::body(&section.descriptions[modifiers]),
                        EavesdroppingKeyboardMode::Modifiers,
                        Some(page.comp_config_xwayland_eavesdropping.keyboard),
                        Message::SetXwaylandKeyboardMode,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        text::body(&section.descriptions[combination]),
                        EavesdroppingKeyboardMode::Combinations,
                        Some(page.comp_config_xwayland_eavesdropping.keyboard),
                        Message::SetXwaylandKeyboardMode,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        text::body(&section.descriptions[all]),
                        EavesdroppingKeyboardMode::All,
                        Some(page.comp_config_xwayland_eavesdropping.keyboard),
                        Message::SetXwaylandKeyboardMode,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item(
                    &section.descriptions[mouse],
                    widget::toggler(page.comp_config_xwayland_eavesdropping.pointer)
                        .on_toggle(Message::SetXwaylandMouseButtonMode),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::LegacyApplications);

            widget::column::with_capacity(3)
                .push(title)
                .push(description)
                .push(content)
                .spacing(cosmic::theme::active().cosmic().spacing.space_xxs)
                .apply(cosmic::Element::from)
                .map(Into::into)
        })
}

pub fn legacy_application_scaling() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let system = descriptions.insert(fl!("legacy-app-scaling", "scaled-by-system"));
    let system_desc = descriptions.insert(fl!("legacy-app-scaling", "system-description"));
    let native = descriptions.insert(fl!("legacy-app-scaling", "scaled-natively"));
    let native_desc = descriptions.insert(fl!("legacy-app-scaling", "native-description"));

    Section::default()
        .title(fl!("legacy-app-scaling"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            widget::settings::section()
                .title(&section.title)
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        widget::column()
                            .push(text::body(&descriptions[system]))
                            .push(text::caption(&descriptions[system_desc])),
                        false,
                        Some(page.comp_config_descale_xwayland),
                        Message::SetXwaylandDescaling,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        widget::column()
                            .push(text::body(&descriptions[native]))
                            .push(text::caption(&descriptions[native_desc])),
                        true,
                        Some(page.comp_config_descale_xwayland),
                        Message::SetXwaylandDescaling,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .apply(Element::from)
                .map(crate::pages::Message::LegacyApplications)
        })
}
