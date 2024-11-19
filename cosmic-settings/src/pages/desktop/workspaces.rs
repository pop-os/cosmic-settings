// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

// TODO make settings work

use cosmic::{
    cosmic_config::{self, ConfigGet, ConfigSet},
    iced::{widget, Alignment, Length},
    widget::{icon, radio, settings, text, ListColumn},
    Apply, Element,
};
use cosmic_comp_config::workspace::{
    WorkspaceConfig, WorkspaceLayout, WorkspaceMode, WorkspaceThumbnailPlacement,
};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slab::Slab;
use slotmap::SlotMap;
use tracing::error;

use crate::utils::system_has_touchpad;

#[derive(Clone, Debug)]
pub enum Message {
    SetWorkspaceMode(WorkspaceMode),
    SetWorkspaceLayout(cosmic::widget::segmented_button::Entity),
    SetWorkspaceThumbnailPlacement(usize),
    ShowTrackpadGestureInfo(bool),
    SetShowName(bool),
    SetShowNumber(bool),
}

pub struct Page {
    config: cosmic_config::Config,
    comp_config: cosmic_config::Config,
    comp_workspace_config: WorkspaceConfig,
    show_workspace_name: bool,
    show_workspace_number: bool,
    show_trackpad_gesture: bool,
    workspace_thumbnail_placement_options: Vec<String>,
    workspace_layout_model: cosmic::widget::segmented_button::SingleSelectModel,
    selected_workspace_thumbnail_placement: usize,
}

struct ThemedAsset {
    light: &'static [u8], 
    dark: &'static [u8],
}

impl ThemedAsset {
    const fn new(light: &'static [u8], dark: &'static [u8]) -> Self {
        Self {
            light,
            dark,
        }
    }
}

impl From<ThemedAsset> for widget::svg::Handle {
    fn from(val: ThemedAsset) -> Self {
        if cosmic::theme::active().cosmic().is_dark {
            cosmic::iced_core::svg::Handle::from_memory(val.dark)
        } else {
            cosmic::iced_core::svg::Handle::from_memory(val.light)
        }
    }
}

macro_rules! themed_asset {
    ($slug:literal) => {
        ThemedAsset::new(include_bytes!(concat!("../../../../resources/assets/",  $slug, "-light.svg")), include_bytes!(concat!("../../../../resources/assets/",  $slug, "-dark.svg")))
    }
}

const WORKSPACE_SPAN_DISPLAY: ThemedAsset = themed_asset!("workspace-span-display");
const WORKSPACE_SEPARATE_DISPLAY: ThemedAsset = themed_asset!("workspace-separate-display");
const WORKSPACE_ORIENTATION_LEFT: ThemedAsset = themed_asset!("workspace-orientation-left");
const WORKSPACE_ORIENTATION_RIGHT: ThemedAsset = themed_asset!("workspace-orientation-right");
const WORKSPACE_ORIENTATION_TOP: ThemedAsset = themed_asset!("workspace-orientation-top");
const WORKSPACE_ORIENTATION_BOTTOM: ThemedAsset = themed_asset!("workspace-orientation-bottom");
const TRACKPAD_GESTURE_SWIPE_VERTICAL: ThemedAsset = themed_asset!("trackpad-gesture-swipe-vertical");
const TRACKPAD_GESTURE_SWIPE_HORIZONTAL: ThemedAsset = themed_asset!("trackpad-gesture-swipe-horizontal");
const TRACKPAD_GESTURE_SWIPE_LEFT: ThemedAsset = themed_asset!("trackpad-gesture-swipe-left");
const TRACKPAD_GESTURE_SWIPE_UP: ThemedAsset = themed_asset!("trackpad-gesture-swipe-up");
const TRACKPAD_GESTURE_SWIPE_RIGHT: ThemedAsset = themed_asset!("trackpad-gesture-swipe-right");
const TRACKPAD_GESTURE_SWIPE_DOWN: ThemedAsset = themed_asset!("trackpad-gesture-swipe-down");


impl Default for Page {
    fn default() -> Self {
        let comp_config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let comp_workspace_config = comp_config.get("workspaces").unwrap_or_else(|err| {
            if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                error!(?err, "Failed to read config 'workspaces'");
            }

            WorkspaceConfig::default()
        });
        let config = cosmic_config::Config::new("com.system76.CosmicWorkspaces", 1).unwrap();
        let show_workspace_name = config.get("show_workspace_name").unwrap_or_else(|err| {
            if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                error!(?err, "Failed to read config 'show_workspace_name'");
            }

            false
        });
        let show_workspace_number = config.get("show_workspace_number").unwrap_or_else(|err| {
            if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                error!(?err, "Failed to read config 'show_workspace_number'");
            }

            false
        });
        let workspace_thumbnail_placement_options = match comp_workspace_config.workspace_layout {
            WorkspaceLayout::Horizontal => vec![
                fl!("workspaces-orientation", "top"),
                fl!("workspaces-orientation", "bottom"),
            ],
            WorkspaceLayout::Vertical => vec![
                fl!("workspaces-orientation", "left"),
                fl!("workspaces-orientation", "right"),
            ],
        };
        let mut workspace_layout_model =
            cosmic::widget::segmented_button::SingleSelectModel::builder()
                .insert(|b| {
                    b.text(fl!("workspaces-orientation", "vertical"))
                        .data(WorkspaceLayout::Vertical)
                })
                .insert(|b| {
                    b.text(fl!("workspaces-orientation", "horizontal"))
                        .data(WorkspaceLayout::Horizontal)
                })
                .build();
        workspace_layout_model.activate_position(match comp_workspace_config.workspace_layout {
            WorkspaceLayout::Vertical => 0,
            WorkspaceLayout::Horizontal => 1,
        });
        let selected_workspace_thumbnail_placement =
            comp_workspace_config.workspace_thumbnail_placement as usize % 2;
        let show_trackpad_gesture = false;
        Self {
            config,
            comp_config,
            comp_workspace_config,
            show_workspace_name,
            show_workspace_number,
            show_trackpad_gesture,
            workspace_thumbnail_placement_options,
            workspace_layout_model,
            selected_workspace_thumbnail_placement,
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
            sections.insert(workspace_overview()),
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
                self.comp_workspace_config.workspace_layout = *self
                    .workspace_layout_model
                    .data::<WorkspaceLayout>(value)
                    .unwrap_or(&WorkspaceLayout::Vertical);
                self.workspace_layout_model.activate_position(
                    match self.comp_workspace_config.workspace_layout {
                        WorkspaceLayout::Vertical => 0,
                        WorkspaceLayout::Horizontal => 1,
                    },
                );
                self.workspace_thumbnail_placement_options =
                    match self.comp_workspace_config.workspace_layout {
                        WorkspaceLayout::Horizontal => vec![
                            fl!("workspaces-orientation", "top"),
                            fl!("workspaces-orientation", "bottom"),
                        ],
                        WorkspaceLayout::Vertical => vec![
                            fl!("workspaces-orientation", "left"),
                            fl!("workspaces-orientation", "right"),
                        ],
                    };
                self.comp_workspace_config.workspace_thumbnail_placement =
                    match self.comp_workspace_config.workspace_layout {
                        WorkspaceLayout::Vertical => {
                            if self.selected_workspace_thumbnail_placement == 0 {
                                WorkspaceThumbnailPlacement::Left
                            } else {
                                WorkspaceThumbnailPlacement::Right
                            }
                        }
                        WorkspaceLayout::Horizontal => {
                            if self.selected_workspace_thumbnail_placement == 0 {
                                WorkspaceThumbnailPlacement::Top
                            } else {
                                WorkspaceThumbnailPlacement::Bottom
                            }
                        }
                    };
                self.save_comp_config();
            }
            Message::SetWorkspaceThumbnailPlacement(value) => {
                self.comp_workspace_config.workspace_thumbnail_placement =
                    match self.comp_workspace_config.workspace_layout {
                        WorkspaceLayout::Vertical => {
                            if value == 0 {
                                WorkspaceThumbnailPlacement::Left
                            } else {
                                WorkspaceThumbnailPlacement::Right
                            }
                        }
                        WorkspaceLayout::Horizontal => {
                            if value == 0 {
                                WorkspaceThumbnailPlacement::Top
                            } else {
                                WorkspaceThumbnailPlacement::Bottom
                            }
                        }
                    };
                self.selected_workspace_thumbnail_placement = value;
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
            Message::ShowTrackpadGestureInfo(value) => {
                self.show_trackpad_gesture = value;
            }
        }
    }
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
            cosmic::widget::settings::section::with_column(
                ListColumn::default()
                    .add(
                        cosmic::iced::widget::column!(
                            widget::vertical_space().height(1),
                            settings::item_row(vec![radio(
                                text::body(&descriptions[span]),
                                WorkspaceMode::Global,
                                Some(page.comp_workspace_config.workspace_mode),
                                Message::SetWorkspaceMode,
                            )
                            .width(Length::Fill)
                            .into()]),
                            cosmic::iced::widget::svg(WORKSPACE_SPAN_DISPLAY)
                        )
                        .spacing(cosmic::theme::active().cosmic().space_s())
                        .align_x(Alignment::Center),
                    )
                    .add(
                        cosmic::iced::widget::column!(
                            widget::vertical_space().height(1),
                            settings::item_row(vec![radio(
                                text::body(&descriptions[separate]),
                                WorkspaceMode::OutputBound,
                                Some(page.comp_workspace_config.workspace_mode),
                                Message::SetWorkspaceMode,
                            )
                            .width(Length::Fill)
                            .into()]),
                            cosmic::iced::widget::svg(
                                WORKSPACE_SEPARATE_DISPLAY
                            )
                        )
                        .spacing(cosmic::theme::active().cosmic().space_s())
                        .align_x(Alignment::Center),
                    )
                    .spacing(0),
            )
            .title(&section.title)
            .apply(Element::from)
            .map(crate::pages::Message::DesktopWorkspaces)
        })
}

fn workspace_orientation() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let thumbnail_placement_label =
        descriptions.insert(fl!("workspaces-orientation", "thumbnail-placement"));
    let trackpad_gestures = descriptions.insert(fl!("workspaces-orientation", "trackpad-gestures"));

    let switch_workspace = descriptions.insert(fl!("workspaces-orientation", "switch-workspace"));
    let open_workspaces = descriptions.insert(fl!("workspaces-orientation", "open-workspaces"));
    let open_applications = descriptions.insert(fl!("workspaces-orientation", "open-applications"));

    let swipe_horizontal = descriptions.insert(fl!("workspaces-orientation", "swipe-horizontal"));
    let swipe_vertical = descriptions.insert(fl!("workspaces-orientation", "swipe-vertical"));
    let swipe_up = descriptions.insert(fl!("workspaces-orientation", "swipe-up"));
    let swipe_down = descriptions.insert(fl!("workspaces-orientation", "swipe-down"));
    let swipe_left = descriptions.insert(fl!("workspaces-orientation", "swipe-left"));
    let swipe_right = descriptions.insert(fl!("workspaces-orientation", "swipe-right"));

    Section::default()
        .title(fl!("workspaces-orientation"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            let thumbnail_placement = cosmic::widget::dropdown(
                &page.workspace_thumbnail_placement_options,
                Some(page.selected_workspace_thumbnail_placement),
                Message::SetWorkspaceThumbnailPlacement,
            );
            let mut section = settings::section()
                .title(&section.title)
                .add(
                    cosmic::iced::widget::column!(
                        cosmic::iced::widget::svg(
                            match page.comp_workspace_config.workspace_thumbnail_placement {
                                WorkspaceThumbnailPlacement::Left =>
                                    WORKSPACE_ORIENTATION_LEFT,
                                WorkspaceThumbnailPlacement::Right =>
                                    WORKSPACE_ORIENTATION_RIGHT,
                                WorkspaceThumbnailPlacement::Top =>
                                    WORKSPACE_ORIENTATION_TOP,
                                WorkspaceThumbnailPlacement::Bottom =>
                                    WORKSPACE_ORIENTATION_BOTTOM,
                            }
                        ),
                        cosmic::iced::widget::container(
                            cosmic::widget::segmented_control::horizontal(
                                &page.workspace_layout_model
                            )
                            .minimum_button_width(0)
                            .on_activate(Message::SetWorkspaceLayout)
                        )
                        .width(320.0)
                        .height(32.0),
                    )
                    .spacing(cosmic::theme::active().cosmic().space_m())
                    .align_x(Alignment::Center),
                )
                .add(settings::item(
                    &descriptions[thumbnail_placement_label],
                    thumbnail_placement,
                ));
            if system_has_touchpad() {
                section = section
                    .add(
                        cosmic::iced::widget::MouseArea::new(settings::item(
                            &descriptions[trackpad_gestures],
                            cosmic::iced::widget::container(
                                icon::from_name(if page.show_trackpad_gesture {
                                    "go-up-symbolic"
                                } else {
                                    "go-down-symbolic"
                                })
                                .size(16),
                            )
                            .width(Length::Shrink),
                        ))
                        .on_press(Message::ShowTrackpadGestureInfo(
                            !page.show_trackpad_gesture,
                        )),
                    );
                if page.show_trackpad_gesture {
                    let (switch_ws, open_ws, open_app) =
                        match page.comp_workspace_config.workspace_layout {
                            WorkspaceLayout::Vertical => (
                                TRACKPAD_GESTURE_SWIPE_VERTICAL,
                                TRACKPAD_GESTURE_SWIPE_LEFT,
                                TRACKPAD_GESTURE_SWIPE_RIGHT,
                            ),
                            WorkspaceLayout::Horizontal => (
                                TRACKPAD_GESTURE_SWIPE_HORIZONTAL,
                                TRACKPAD_GESTURE_SWIPE_UP,
                                TRACKPAD_GESTURE_SWIPE_DOWN,
                            ),
                        };
                    let (switch_ws_label, open_ws_label, open_app_label) =
                        match page.comp_workspace_config.workspace_layout {
                            WorkspaceLayout::Vertical => (swipe_vertical, swipe_left, swipe_right),
                            WorkspaceLayout::Horizontal => (swipe_horizontal, swipe_up, swipe_down),
                        };
                    section = section.add(
                        cosmic::widget::list_column()
                            .padding([0, 32])
                            .add(
                                cosmic::iced::widget::row!(
                                    text(&descriptions[switch_workspace]),
                                    cosmic::iced::widget::horizontal_space().width(2),
                                    text(&descriptions[switch_ws_label]).font(cosmic::font::bold()),
                                    cosmic::iced::widget::horizontal_space().width(Length::Fill),
                                    cosmic::iced::widget::container(cosmic::iced::widget::svg(
                                        switch_ws
                                    ))
                                    .width(115)
                                    .height(92)
                                )
                                .width(Length::Fill)
                                .align_y(Alignment::Center)
                                .padding([0, 16]),
                            )
                            .add(
                                cosmic::iced::widget::row!(
                                    text(&descriptions[open_workspaces]),
                                    cosmic::iced::widget::horizontal_space().width(2),
                                    text(&descriptions[open_ws_label]).font(cosmic::font::bold()),
                                    cosmic::iced::widget::horizontal_space().width(Length::Fill),
                                    cosmic::iced::widget::container(cosmic::iced::widget::svg(open_ws))
                                        .width(115)
                                        .height(92)
                                )
                                .width(Length::Fill)
                                .align_y(Alignment::Center)
                                .padding([0, 16]),
                            )
                            .add(
                                cosmic::widget::list_column().add(
                                    cosmic::iced::widget::row!(
                                        text(&descriptions[open_applications]),
                                        cosmic::iced::widget::horizontal_space().width(2),
                                        text(&descriptions[open_app_label]).font(cosmic::font::bold()),
                                        cosmic::iced::widget::horizontal_space().width(Length::Fill),
                                        cosmic::iced::widget::container(cosmic::iced::widget::svg(
                                            open_app
                                        ))
                                        .width(115)
                                        .height(92)
                                    )
                                    .width(Length::Fill)
                                    .align_y(Alignment::Center)
                                    .padding([0, 16]),
                                ),
                            ),
                    );
                }
            }

            section
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWorkspaces)
        })
}

fn workspace_overview() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let show_number = descriptions.insert(fl!("workspaces-overview-thumbnails", "show-number"));
    let show_name = descriptions.insert(fl!("workspaces-overview-thumbnails", "show-name"));

    Section::default()
        .title(fl!("workspaces-overview-thumbnails"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            settings::section()
                .title(&section.title)
                .add(
                    settings::item::builder(&descriptions[show_number])
                        .toggler(page.show_workspace_name, Message::SetShowName),
                )
                .add(
                    settings::item::builder(&descriptions[show_name])
                        .toggler(page.show_workspace_number, Message::SetShowNumber),
                )
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWorkspaces)
        })
}
