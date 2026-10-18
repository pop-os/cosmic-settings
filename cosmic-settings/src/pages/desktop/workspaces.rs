// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

// TODO make settings work

use cosmic::cosmic_config::{self, ConfigGet, ConfigSet};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, settings};
use cosmic::{Apply, Element, surface};
use cosmic_comp_config::workspace::{Action, WorkspaceConfig, WorkspaceLayout, WorkspaceMode};
use cosmic_settings_page::{self as page, Section, section};
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;
use std::collections::HashMap;
use tracing::error;

/// Mirrors `OverviewStyle` in cosmic-workspaces.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
enum OverviewStyle {
    #[default]
    Normal,
    Compact,
}

/// Mirrors `cosmic_panel_config::PanelAnchor`, which cosmic-workspaces uses for
/// the workspace bar position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum BarPosition {
    Left,
    Right,
    Top,
    Bottom,
}

/// Options of the bar position dropdown; `None` follows the workspace orientation.
const BAR_POSITIONS: [Option<BarPosition>; 5] = [
    None,
    Some(BarPosition::Left),
    Some(BarPosition::Right),
    Some(BarPosition::Top),
    Some(BarPosition::Bottom),
];

const COMPACT_WORKSPACE_SIZE_DEFAULT: u16 = 96;

#[derive(Clone, Debug)]
pub enum Message {
    SetActionOnTyping(usize),
    SetOverviewStyle(usize),
    SetBarPosition(usize),
    CompactWorkspaceSize(u16),
    CompactWorkspaceSizeCommit,
    SetWorkspaceMode(WorkspaceMode),
    SetWorkspaceLayout(WorkspaceLayout),
    SetWorkspaceWraparound(bool),
    SetShowName(bool),
    SetShowNumber(bool),
    Surface(surface::Action<crate::app::Message>),
}

pub struct Page {
    config: cosmic_config::Config,
    comp_config: cosmic_config::Config,
    comp_workspace_config: WorkspaceConfig,
    action_on_typing_selections: Vec<String>,
    action_on_typing_active: Option<usize>,
    show_workspace_name: bool,
    show_workspace_number: bool,
    overview_style_selections: Vec<String>,
    overview_style: OverviewStyle,
    bar_position_selections: Vec<String>,
    bar_position: Option<BarPosition>,
    compact_workspace_size: u16,
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
        let overview_style = read_config(&config, "overview_style", OverviewStyle::default());
        let bar_position = read_config(&config, "bar_position", None);
        let compact_workspace_size = read_config(
            &config,
            "compact_workspace_size",
            COMPACT_WORKSPACE_SIZE_DEFAULT,
        );
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
            overview_style_selections: vec![
                fl!("workspaces-overview", "normal"),
                fl!("workspaces-overview", "compact"),
            ],
            overview_style,
            bar_position_selections: vec![
                fl!("workspaces-overview", "automatic"),
                fl!("panel-left"),
                fl!("panel-right"),
                fl!("panel-top"),
                fl!("panel-bottom"),
            ],
            bar_position,
            compact_workspace_size,
        }
    }
}

fn read_config<T: serde::de::DeserializeOwned>(
    config: &cosmic_config::Config,
    key: &str,
    default: T,
) -> T {
    config.get(key).unwrap_or_else(|err| {
        if err.is_err() {
            error!(?err, "Failed to read config '{key}'");
        }

        default
    })
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

    fn set_config<T: Serialize>(&self, key: &str, value: T) {
        if let Err(err) = self.config.set(key, value) {
            error!(?err, "Failed to set config '{key}'");
        }
    }

    pub fn update(&mut self, message: Message) -> cosmic::iced::Task<crate::app::Message> {
        match message {
            Message::SetOverviewStyle(idx) => {
                self.overview_style = if idx == 1 {
                    OverviewStyle::Compact
                } else {
                    OverviewStyle::Normal
                };
                self.set_config("overview_style", self.overview_style);
            }
            Message::SetBarPosition(idx) => {
                if let Some(position) = BAR_POSITIONS.get(idx) {
                    self.bar_position = *position;
                    self.set_config("bar_position", self.bar_position);
                }
            }
            // Only moves the slider; the overview is updated once it is released.
            Message::CompactWorkspaceSize(size) => {
                self.compact_workspace_size = size;
            }
            Message::CompactWorkspaceSizeCommit => {
                self.set_config("compact_workspace_size", self.compact_workspace_size);
            }
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
    crate::slab!(descriptions {
        action_on_typing = fl!("workspaces-overview", "action-on-typing");
        style = fl!("workspaces-overview", "style");
        bar_position = fl!("workspaces-overview", "bar-position");
        workspace_size = fl!("workspaces-overview", "workspace-size");
    });

    Section::default()
        .title(fl!("workspaces-overview"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            let on_surface_action =
                |a| crate::app::Message::PageMessage(crate::pages::Message::Workspaces(a));

            let mut section = settings::section()
                .title(&section.title)
                .add(
                    settings::item::builder(&descriptions[action_on_typing]).control(
                        widget::dropdown::popup_dropdown(
                            &page.action_on_typing_selections,
                            page.action_on_typing_active,
                            Message::SetActionOnTyping,
                            cosmic::iced::window::Id::RESERVED,
                            Message::Surface,
                            on_surface_action,
                        ),
                    ),
                )
                .add(settings::item::builder(&descriptions[style]).control(
                    widget::dropdown::popup_dropdown(
                        &page.overview_style_selections,
                        Some(match page.overview_style {
                            OverviewStyle::Normal => 0,
                            OverviewStyle::Compact => 1,
                        }),
                        Message::SetOverviewStyle,
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        on_surface_action,
                    ),
                ))
                .add(
                    settings::item::builder(&descriptions[bar_position]).control(
                        widget::dropdown::popup_dropdown(
                            &page.bar_position_selections,
                            BAR_POSITIONS.iter().position(|p| *p == page.bar_position),
                            Message::SetBarPosition,
                            cosmic::iced::window::Id::RESERVED,
                            Message::Surface,
                            on_surface_action,
                        ),
                    ),
                );

            if page.overview_style == OverviewStyle::Compact {
                section = section.add(
                    settings::item::builder(&descriptions[workspace_size]).flex_control(
                        widget::row::with_capacity(2)
                            .align_y(Alignment::Center)
                            .spacing(8)
                            .width(Length::Fill)
                            .push(
                                widget::text::body(fl!(
                                    "number",
                                    HashMap::from_iter(vec![(
                                        "number",
                                        i32::from(page.compact_workspace_size)
                                    )])
                                ))
                                .width(Length::Fixed(28.0))
                                .align_x(Alignment::Center),
                            )
                            .push(
                                widget::slider(
                                    64..=160,
                                    page.compact_workspace_size,
                                    Message::CompactWorkspaceSize,
                                )
                                .step(8u16)
                                .on_release(Message::CompactWorkspaceSizeCommit)
                                .width(Length::Fill)
                                .apply(widget::container)
                                .max_width(250),
                            ),
                    ),
                );
            }

            section
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
    });

    Section::default()
        .title(fl!("workspaces-orientation"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            settings::section()
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
