// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

// TODO make settings work

use cosmic::widget::settings;
use cosmic::{
    cosmic_config::{self, ConfigGet, ConfigSet},
    widget::radio,
    Apply, Element,
};
use cosmic_comp_config::workspace::{WorkspaceConfig, WorkspaceLayout, WorkspaceMode};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;
use tracing::error;

#[derive(Clone, Debug)]
pub enum Message {
    SetWorkspaceMode(WorkspaceMode),
    SetWorkspaceLayout(WorkspaceLayout),
    SetShowName(bool),
    SetShowNumber(bool),
}

pub struct Page {
    config: cosmic_config::Config,
    comp_config: cosmic_config::Config,
    comp_workspace_config: WorkspaceConfig,
    show_workspace_name: bool,
    show_workspace_number: bool,
}

impl Default for Page {
    fn default() -> Self {
        let comp_config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let comp_workspace_config = comp_config.get("workspaces").unwrap_or_else(|err| {
            error!(?err, "Failed to read config 'workspaces'");
            WorkspaceConfig::default()
        });
        let config = cosmic_config::Config::new("com.system76.CosmicWorkspaces", 1).unwrap();
        let show_workspace_name = config.get("show_workspace_name").unwrap_or_else(|err| {
            error!(?err, "Failed to read config 'show_workspace_name'");
            false
        });
        let show_workspace_number = config.get("show_workspace_number").unwrap_or_else(|err| {
            error!(?err, "Failed to read config 'show_workspace_number'");
            false
        });
        Self {
            config,
            comp_config,
            comp_workspace_config,
            show_workspace_name,
            show_workspace_number,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(multi_behavior()),
            sections.insert(workspace_orientation()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("workspaces", "preferences-workspaces-symbolic")
            .title(fl!("workspaces"))
            .description(fl!("workspaces", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl Page {
    fn save_comp_config(&self) {
        if let Err(err) = self
            .comp_config
            .set("workspaces", &self.comp_workspace_config)
        {
            error!(?err, "Failed to set config 'workspaces'");
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SetWorkspaceMode(value) => {
                self.comp_workspace_config.workspace_mode = value;
                self.save_comp_config();
            }
            Message::SetWorkspaceLayout(value) => {
                self.comp_workspace_config.workspace_layout = value;
                self.save_comp_config();
            }
            Message::SetShowName(value) => {
                self.show_workspace_name = value;
                if let Err(err) = self.config.set("show_workspace_name", value) {
                    error!(?err, "Failed to set config 'show_workspace_name'");
                }
            }
            Message::SetShowNumber(value) => {
                self.show_workspace_number = value;
                if let Err(err) = self.config.set("show_workspace_number", value) {
                    error!(?err, "Failed to set config 'show_workspace_number'");
                }
            }
        }
    }
}

fn multi_behavior() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("workspaces-multi-behavior"))
        .descriptions(vec![
            fl!("workspaces-multi-behavior", "span").into(),
            fl!("workspaces-multi-behavior", "separate").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;
            settings::view_section(&section.title)
                .add(settings::item_row(vec![radio(
                    &*descriptions[0],
                    WorkspaceMode::Global,
                    Some(page.comp_workspace_config.workspace_mode),
                    Message::SetWorkspaceMode,
                )
                .into()]))
                .add(settings::item_row(vec![radio(
                    &*descriptions[1],
                    WorkspaceMode::OutputBound,
                    Some(page.comp_workspace_config.workspace_mode),
                    Message::SetWorkspaceMode,
                )
                .into()]))
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWorkspaces)
        })
}

fn workspace_orientation() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("workspaces-orientation"))
        .descriptions(vec![
            fl!("workspaces-orientation", "vertical").into(),
            fl!("workspaces-orientation", "horizontal").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;
            settings::view_section(&section.title)
                .add(settings::item_row(vec![radio(
                    &*descriptions[0],
                    WorkspaceLayout::Vertical,
                    Some(page.comp_workspace_config.workspace_layout),
                    Message::SetWorkspaceLayout,
                )
                .into()]))
                .add(settings::item_row(vec![radio(
                    &*descriptions[1],
                    WorkspaceLayout::Horizontal,
                    Some(page.comp_workspace_config.workspace_layout),
                    Message::SetWorkspaceLayout,
                )
                .into()]))
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWorkspaces)
        })
}
