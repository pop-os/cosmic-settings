// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

// TODO make settings work

use cosmic::{
    Apply, Element,
    cosmic_config::{self, ConfigGet, ConfigSet},
    iced::Length,
    surface,
    widget::{self, radio, settings, text},
};
use cosmic_comp_config::workspace::{Action, WorkspaceConfig, WorkspaceLayout, WorkspaceMode};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slab::Slab;
use slotmap::SlotMap;
use tracing::error;

#[derive(Clone, Debug)]
pub enum Message {
    SetActionOnTyping(usize),
    SetWorkspaceMode(WorkspaceMode),
    SetWorkspaceLayout(WorkspaceLayout),
    SetShowName(bool),
    SetShowNumber(bool),
    Surface(surface::Action),
}

pub struct Page {
    config: cosmic_config::Config,
    comp_config: cosmic_config::Config,
    comp_workspace_config: WorkspaceConfig,
    action_on_typing_selections: Vec<String>,
    action_on_typing_active: Option<usize>,
    show_workspace_name: bool,
    show_workspace_number: bool,
}

impl Default for Page {
    fn default() -> Self {
        let comp_config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let comp_workspace_config = comp_config.get("workspaces").unwrap_or_else(|err| {
            if err.is_err() {
                error!(?err, "Failed to read config 'workspaces'");
            }

            WorkspaceConfig::default()
        });
        let config = cosmic_config::Config::new("com.system76.CosmicWorkspaces", 1).unwrap();
        let action_on_typing_active =
            into_active_selection(&comp_workspace_config.action_on_typing);
        let show_workspace_name = config.get("show_workspace_name").unwrap_or_else(|err| {
            if err.is_err() {
                error!(?err, "Failed to read config 'show_workspace_name'");
            }

            false
        });
        let show_workspace_number = config.get("show_workspace_number").unwrap_or_else(|err| {
            if err.is_err() {
                error!(?err, "Failed to read config 'show_workspace_number'");
            }

            false
        });
        Self {
            config,
            comp_config,
            comp_workspace_config,
            action_on_typing_selections: vec![
                fl!("workspaces-overview", "none"),
                fl!("workspaces-overview", "launcher"),
                fl!("workspaces-overview", "applications"),
            ],
            action_on_typing_active,
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
            sections.insert(action_on_typing()),
            sections.insert(multi_behavior()),
            sections.insert(workspace_orientation()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("workspaces", "preferences-workspaces-symbolic")
            .title(fl!("workspaces"))
            .description(fl!("xdg-entry-workspaces-comment"))
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

    pub fn update(&mut self, message: Message) -> cosmic::iced::Task<crate::app::Message> {
        match message {
            Message::SetWorkspaceMode(value) => {
                self.comp_workspace_config.workspace_mode = value;
                self.save_comp_config();
            }
            Message::SetWorkspaceLayout(value) => {
                self.comp_workspace_config.workspace_layout = value;
                self.save_comp_config();
            }
            Message::SetActionOnTyping(value) => {
                self.comp_workspace_config.action_on_typing = into_action(value);
                self.action_on_typing_active =
                    into_active_selection(&self.comp_workspace_config.action_on_typing);
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
            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }
        }
        cosmic::iced::Task::none()
    }
}

fn into_active_selection(action_on_typing: &Action) -> Option<usize> {
    match action_on_typing {
        Action::None => Some(0),
        Action::OpenLauncher => Some(1),
        Action::OpenApplications => Some(2),
    }
}

fn into_action(value: usize) -> Action {
    match value {
        1 => Action::OpenLauncher,
        2 => Action::OpenApplications,
        _ => Action::None,
    }
}

pub fn action_on_typing() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let action_on_typing = descriptions.insert(fl!("workspaces-overview", "action-on-typing"));

    Section::default()
        .title(fl!("workspaces-overview"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(
                    settings::item::builder(&descriptions[action_on_typing]).control(
                        widget::dropdown::popup_dropdown(
                            &page.action_on_typing_selections,
                            page.action_on_typing_active,
                            Message::SetActionOnTyping,
                            cosmic::iced::window::Id::RESERVED,
                            Message::Surface,
                            |a| {
                                crate::app::Message::PageMessage(crate::pages::Message::Workspaces(
                                    a,
                                ))
                            },
                        ),
                    ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Workspaces)
        })
}

fn multi_behavior() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let span = descriptions.insert(fl!("workspaces-multi-behavior", "span"));
    let separate = descriptions.insert(fl!("workspaces-multi-behavior", "separate"));

    Section::default()
        .title(fl!("workspaces-multi-behavior"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            settings::section()
                .title(&section.title)
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[span]),
                        WorkspaceMode::Global,
                        Some(page.comp_workspace_config.workspace_mode),
                        Message::SetWorkspaceMode,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[separate]),
                        WorkspaceMode::OutputBound,
                        Some(page.comp_workspace_config.workspace_mode),
                        Message::SetWorkspaceMode,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWorkspaces)
        })
}

fn workspace_orientation() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let vertical = descriptions.insert(fl!("workspaces-orientation", "vertical"));
    let horizontal = descriptions.insert(fl!("workspaces-orientation", "horizontal"));

    Section::default()
        .title(fl!("workspaces-orientation"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            settings::section()
                .title(&section.title)
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[vertical]),
                        WorkspaceLayout::Vertical,
                        Some(page.comp_workspace_config.workspace_layout),
                        Message::SetWorkspaceLayout,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[horizontal]),
                        WorkspaceLayout::Horizontal,
                        Some(page.comp_workspace_config.workspace_layout),
                        Message::SetWorkspaceLayout,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWorkspaces)
        })
}
