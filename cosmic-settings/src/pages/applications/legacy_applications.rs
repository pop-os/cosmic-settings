// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    cosmic_config::{self, ConfigGet, ConfigSet},
    iced::Length,
    widget::{self, text},
    Apply, Element,
};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slab::Slab;
use slotmap::SlotMap;
use tracing::error;

#[derive(Clone, Debug)]
pub enum Message {
    SetXwaylandDescaling(bool),
}

pub struct Page {
    comp_config: cosmic_config::Config,
    comp_config_descale_xwayland: bool,
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
        Self {
            comp_config,
            comp_config_descale_xwayland,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(
            // Xwayland scaling options
            legacy_application_scaling(),
        )])
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
        }
    }
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
                .add(widget::settings::item_row(vec![widget::radio(
                    widget::column()
                        .push(text::body(&descriptions[system]))
                        .push(text::caption(&descriptions[system_desc])),
                    false,
                    Some(page.comp_config_descale_xwayland),
                    Message::SetXwaylandDescaling,
                )
                .width(Length::Fill)
                .into()]))
                .add(widget::settings::item_row(vec![widget::radio(
                    widget::column()
                        .push(text::body(&descriptions[native]))
                        .push(text::caption(&descriptions[native_desc])),
                    true,
                    Some(page.comp_config_descale_xwayland),
                    Message::SetXwaylandDescaling,
                )
                .width(Length::Fill)
                .into()]))
                .apply(Element::from)
                .map(crate::pages::Message::LegacyApplications)
        })
}
