// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

// TODO make settings work

use cosmic::cosmic_config::{self, ConfigGet, ConfigSet};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, icon, settings, text};
use cosmic::{Apply, Element, surface};
use cosmic_comp_config::workspace::{Action, WorkspaceConfig, WorkspaceLayout, WorkspaceMode};
use cosmic_comp_config::WorkspaceAssignment;
use cosmic_settings_page::{self as page, Section, section};
use slotmap::SlotMap;
use tracing::error;

#[derive(Clone, Debug)]
pub enum Message {
    SetActionOnTyping(usize),
    SetWorkspaceMode(WorkspaceMode),
    SetWorkspaceLayout(WorkspaceLayout),
    SetWorkspaceWraparound(bool),
    SetGridColumns(u32),
    SetGridRows(u32),
    SetShowName(bool),
    SetShowNumber(bool),
    Surface(surface::Action),
    SetAssignmentWorkspace(usize, u32),
    RemoveAssignment(usize),
    NewAssignmentAppId(String),
    NewAssignmentWorkspace(u32),
    AddAssignment,
}

pub struct Page {
    config: cosmic_config::Config,
    comp_config: cosmic_config::Config,
    comp_workspace_config: WorkspaceConfig,
    action_on_typing_selections: Vec<String>,
    action_on_typing_active: Option<usize>,
    show_workspace_name: bool,
    show_workspace_number: bool,
    workspace_assignments: Vec<WorkspaceAssignment>,
    new_assignment_app_id: String,
    new_assignment_workspace: u32,
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
        let workspace_assignments: Vec<WorkspaceAssignment> = comp_config
            .get("workspace_assignments")
            .unwrap_or_else(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'workspace_assignments'");
                }

                Vec::new()
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
            workspace_assignments,
            new_assignment_app_id: String::new(),
            new_assignment_workspace: 1,
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
            sections.insert(workspace_navigation()),
            sections.insert(workspace_assignments()),
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

    fn save_assignments(&self) {
        if let Err(err) = self
            .comp_config
            .set("workspace_assignments", &self.workspace_assignments)
        {
            error!(?err, "Failed to set config 'workspace_assignments'");
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
            Message::SetWorkspaceWraparound(value) => {
                self.comp_workspace_config.workspace_wraparound = value;
                self.save_comp_config();
            }
            Message::SetGridColumns(value) => {
                self.comp_workspace_config.workspace_grid_columns = value.max(1);
                self.save_comp_config();
            }
            Message::SetGridRows(value) => {
                self.comp_workspace_config.workspace_grid_rows = value.max(1);
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
            Message::SetAssignmentWorkspace(index, value) => {
                if let Some(rule) = self.workspace_assignments.get_mut(index) {
                    rule.workspace = value.max(1);
                    self.save_assignments();
                }
            }
            Message::RemoveAssignment(index) => {
                if index < self.workspace_assignments.len() {
                    self.workspace_assignments.remove(index);
                    self.save_assignments();
                }
            }
            Message::NewAssignmentAppId(value) => {
                self.new_assignment_app_id = value;
            }
            Message::NewAssignmentWorkspace(value) => {
                self.new_assignment_workspace = value.max(1);
            }
            Message::AddAssignment => {
                let app_id = self.new_assignment_app_id.trim().to_string();
                if !app_id.is_empty() {
                    self.workspace_assignments.push(WorkspaceAssignment {
                        app_id,
                        workspace: self.new_assignment_workspace,
                    });
                    self.new_assignment_app_id.clear();
                    self.new_assignment_workspace = 1;
                    self.save_assignments();
                }
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
    crate::slab!(descriptions {
        action_on_typing = fl!("workspaces-overview", "action-on-typing");
    });

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
    crate::slab!(descriptions {
        span = fl!("workspaces-multi-behavior", "span");
        separate = fl!("workspaces-multi-behavior", "separate");
    });

    Section::default()
        .title(fl!("workspaces-multi-behavior"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            settings::section()
                .title(&section.title)
                .add(settings::item::builder(&descriptions[span]).radio(
                    WorkspaceMode::Global,
                    Some(page.comp_workspace_config.workspace_mode),
                    Message::SetWorkspaceMode,
                ))
                .add(settings::item::builder(&descriptions[separate]).radio(
                    WorkspaceMode::OutputBound,
                    Some(page.comp_workspace_config.workspace_mode),
                    Message::SetWorkspaceMode,
                ))
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWorkspaces)
        })
}

fn workspace_orientation() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        vertical = fl!("workspaces-orientation", "vertical");
        horizontal = fl!("workspaces-orientation", "horizontal");
        grid = fl!("workspaces-orientation", "grid");
        grid_columns = fl!("workspaces-orientation", "grid-columns");
        grid_rows = fl!("workspaces-orientation", "grid-rows");
    });

    Section::default()
        .title(fl!("workspaces-orientation"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let is_grid =
                page.comp_workspace_config.workspace_layout == WorkspaceLayout::Grid;
            let mut section = settings::section()
                .title(&section.title)
                .add(settings::item::builder(&descriptions[vertical]).radio(
                    WorkspaceLayout::Vertical,
                    Some(page.comp_workspace_config.workspace_layout),
                    Message::SetWorkspaceLayout,
                ))
                .add(settings::item::builder(&descriptions[horizontal]).radio(
                    WorkspaceLayout::Horizontal,
                    Some(page.comp_workspace_config.workspace_layout),
                    Message::SetWorkspaceLayout,
                ))
                .add(settings::item::builder(&descriptions[grid]).radio(
                    WorkspaceLayout::Grid,
                    Some(page.comp_workspace_config.workspace_layout),
                    Message::SetWorkspaceLayout,
                ));
            if is_grid {
                section = section
                    .add(
                        settings::item::builder(&descriptions[grid_columns]).control(
                            widget::spin_button(
                                page.comp_workspace_config.workspace_grid_columns.to_string(),
                                "grid columns",
                                page.comp_workspace_config.workspace_grid_columns,
                                1,
                                1,
                                10,
                                Message::SetGridColumns,
                            ),
                        ),
                    )
                    .add(
                        settings::item::builder(&descriptions[grid_rows]).control(
                            widget::spin_button(
                                page.comp_workspace_config.workspace_grid_rows.to_string(),
                                "grid rows",
                                page.comp_workspace_config.workspace_grid_rows,
                                1,
                                1,
                                10,
                                Message::SetGridRows,
                            ),
                        ),
                    );
            }
            section
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWorkspaces)
        })
}

fn workspace_navigation() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        description = fl!("workspaces-navigation", "wraparound");
    });

    Section::default()
        .title(fl!("workspaces-navigation"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            settings::section()
                .title(&section.title)
                .add(settings::item::builder(&descriptions[description]).toggler(
                    page.comp_workspace_config.workspace_wraparound,
                    Message::SetWorkspaceWraparound,
                ))
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWorkspaces)
        })
}

fn workspace_assignments() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        description = fl!("workspaces-assignments", "description");
    });

    Section::default()
        .title(fl!("workspaces-assignments"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let cosmic::cosmic_theme::Spacing { space_xs, .. } = cosmic::theme::spacing();
            let descriptions = &section.descriptions;

            let mut section = settings::section()
                .title(&section.title)
                .add(text::body(&descriptions[description]));

            if page.workspace_assignments.is_empty() {
                section = section.add(text::body(fl!("workspaces-assignments", "none")));
            } else {
                for (index, rule) in page.workspace_assignments.iter().enumerate() {
                    let row = widget::row::with_capacity(4)
                        .spacing(space_xs)
                        .align_y(Alignment::Center)
                        .push(icon::from_name("application-x-executable-symbolic").size(24))
                        .push(text::body(rule.app_id.clone()).width(Length::Fill))
                        .push(widget::spin_button(
                            rule.workspace.to_string(),
                            "workspace",
                            rule.workspace,
                            1,
                            1,
                            99,
                            move |value| Message::SetAssignmentWorkspace(index, value),
                        ))
                        .push(
                            widget::button::icon(icon::from_name("edit-delete-symbolic"))
                                .extra_small()
                                .on_press(Message::RemoveAssignment(index)),
                        );
                    section = section.add(row);
                }
            }

            let add_row = widget::row::with_capacity(3)
                .spacing(space_xs)
                .align_y(Alignment::Center)
                .push(
                    widget::text_input(
                        fl!("workspaces-assignments", "app-id-placeholder"),
                        &page.new_assignment_app_id,
                    )
                    .on_input(Message::NewAssignmentAppId)
                    .width(Length::Fill),
                )
                .push(widget::spin_button(
                    page.new_assignment_workspace.to_string(),
                    "new-assignment-workspace",
                    page.new_assignment_workspace,
                    1,
                    1,
                    99,
                    Message::NewAssignmentWorkspace,
                ))
                .push(
                    widget::button::standard(fl!("workspaces-assignments", "add")).on_press_maybe(
                        (!page.new_assignment_app_id.trim().is_empty())
                            .then_some(Message::AddAssignment),
                    ),
                );

            section
                .add(add_row)
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWorkspaces)
        })
}
