// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    Apply, Element,
    iced::Length,
    surface,
    widget::{self, settings, toggler},
};

use cosmic_comp_config::CosmicCompConfig;
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_config::{Action, Binding, Shortcuts, shortcuts};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slab::Slab;
use slotmap::SlotMap;
use tracing::error;

#[derive(Clone, Debug)]
pub enum Message {
    SuperKey(usize),
    CompConfigUpdate(Box<CosmicCompConfig>),
    SetFocusFollowsCursor(bool),
    SaveFocusFollowsCursorDelay(bool),
    SetFocusFollowsCursorDelay(String),
    SetCursorFollowsFocus(bool),
    ShowActiveWindowHint(bool),
    ShowMaximizeButton(bool),
    ShowMinimizeButton(bool),
    SetEdgeSnapThreshold(u32),
    Surface(surface::Action),
}

pub struct Page {
    pub super_key_selections: Vec<String>,
    pub super_key_active: Option<usize>,
    comp_config: cosmic_config::Config,
    focus_follows_cursor: bool,
    focus_follows_cursor_delay: u64,
    focus_delay_text: String,
    cursor_follows_focus: bool,
    show_active_hint: bool,
    edge_snap_threshold: u32,
}

impl Default for Page {
    fn default() -> Self {
        let comp_config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let focus_follows_cursor = comp_config
            .get("focus_follows_cursor")
            .unwrap_or_else(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'focus_follows_cursor'");
                }
                false
            });
        let cursor_follows_focus = comp_config
            .get("cursor_follows_focus")
            .unwrap_or_else(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'cursor_follows_focus'");
                }
                false
            });

        let focus_follows_cursor_delay = comp_config
            .get("focus_follows_cursor_delay")
            .inspect_err(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'focus_follows_cursor_delay'")
                }
            })
            .unwrap_or(250);

        let show_active_hint = comp_config
            .get("active_hint")
            .inspect_err(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'active_hint'")
                }
            })
            .unwrap_or(true);

        let edge_snap_threshold = comp_config
            .get("edge_snap_threshold")
            .inspect_err(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'edge_snap_threshold'")
                }
            })
            .unwrap_or(0);

        Page {
            super_key_selections: vec![
                fl!("super-key", "launcher"),
                fl!("super-key", "workspaces"),
                fl!("super-key", "applications"),
                fl!("super-key", "disable"),
            ],
            super_key_active: super_key_active_config(),
            comp_config,
            focus_follows_cursor,
            focus_follows_cursor_delay,
            focus_delay_text: format!("{focus_follows_cursor_delay}"),
            cursor_follows_focus,
            show_active_hint,
            edge_snap_threshold,
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::iced::Task<crate::app::Message> {
        match message {
            Message::SuperKey(id) => {
                let action = match id {
                    0 => Some(shortcuts::action::System::Launcher),
                    1 => Some(shortcuts::action::System::WorkspaceOverview),
                    2 => Some(shortcuts::action::System::AppLibrary),
                    3 => None,
                    _ => return cosmic::iced::Task::none(),
                };

                self.super_key_active = Some(id);
                super_key_set(action);
            }
            Message::SetFocusFollowsCursor(value) => {
                self.focus_follows_cursor = value;
                if let Err(err) = self
                    .comp_config
                    .set("focus_follows_cursor", self.focus_follows_cursor)
                {
                    error!(?err, "Failed to set config 'focus_follows_cursor'");
                }
            }
            Message::SaveFocusFollowsCursorDelay(save) => {
                // Debounce to avoid spam writing config on user input
                if save {
                    if let Err(err) = self.comp_config.set(
                        "focus_follows_cursor_delay",
                        self.focus_follows_cursor_delay,
                    ) {
                        error!(?err, "Failed to set config 'focus_follows_cursor_delay'");
                    }

                    self.focus_delay_text = format!("{}", self.focus_follows_cursor_delay);
                }
            }
            Message::SetFocusFollowsCursorDelay(value) => {
                if let Ok(delay) = value.parse() {
                    self.focus_follows_cursor_delay = delay;
                }
                self.focus_delay_text = value;
            }
            Message::SetCursorFollowsFocus(value) => {
                self.cursor_follows_focus = value;
                if let Err(err) = self
                    .comp_config
                    .set("cursor_follows_focus", self.cursor_follows_focus)
                {
                    error!(?err, "Failed to set config 'cursor_follows_focus'");
                }
            }
            Message::ShowMaximizeButton(value) => {
                if let Ok(config) = cosmic::config::CosmicTk::config() {
                    let _res = cosmic::config::COSMIC_TK
                        .write()
                        .unwrap()
                        .set_show_maximize(&config, value);
                }
            }
            Message::ShowMinimizeButton(value) => {
                if let Ok(config) = cosmic::config::CosmicTk::config() {
                    let _res = cosmic::config::COSMIC_TK
                        .write()
                        .unwrap()
                        .set_show_minimize(&config, value);
                }
            }
            Message::ShowActiveWindowHint(value) => {
                self.show_active_hint = value;
                if let Err(err) = self.comp_config.set("active_hint", value) {
                    error!(?err, "Failed to set config 'active_hint'");
                }
            }
            Message::CompConfigUpdate(comp_config) => {
                if comp_config.active_hint != self.show_active_hint {
                    self.show_active_hint = comp_config.active_hint;
                }
            }
            Message::SetEdgeSnapThreshold(value) => {
                self.edge_snap_threshold = value;
                if let Err(err) = self.comp_config.set("edge_snap_threshold", value) {
                    error!(?err, "Failed to set config 'edge_snap_threshold'");
                }
            }
            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }
        };
        cosmic::iced::Task::none()
    }
}

impl page::Page<crate::pages::Message> for Page {
    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(window_management()),
            sections.insert(window_controls()),
            sections.insert(focus_navigation()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new(
            "window-management",
            "preferences-window-management-symbolic",
        )
        .title(fl!("window-management"))
        .description(fl!("window-management", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

pub fn window_management() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let super_key = descriptions.insert(fl!("super-key"));
    let _launcher = descriptions.insert(fl!("super-key", "launcher"));
    let _workspaces = descriptions.insert(fl!("super-key", "workspaces"));
    let _applications = descriptions.insert(fl!("super-key", "applications"));
    let _disable = descriptions.insert(fl!("super-key", "disable"));

    let edge_gravity = descriptions.insert(fl!("edge-gravity"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(settings::item::builder(&descriptions[super_key]).control(
                    widget::dropdown::popup_dropdown(
                        &page.super_key_selections,
                        page.super_key_active,
                        Message::SuperKey,
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        |a| {
                            crate::app::Message::PageMessage(
                                crate::pages::Message::WindowManagement(a),
                            )
                        },
                    ),
                ))
                .add(settings::flex_item(
                    &descriptions[edge_gravity],
                    toggler(page.edge_snap_threshold != 0).on_toggle(|is_enabled| {
                        Message::SetEdgeSnapThreshold(if is_enabled { 10 } else { 0 })
                    }),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::WindowManagement)
        })
}

pub fn window_controls() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let maximize = descriptions.insert(fl!("window-controls", "maximize"));
    let minimize = descriptions.insert(fl!("window-controls", "minimize"));
    let active_window_hint = descriptions.insert(fl!("window-controls", "active-window-hint"));

    Section::default()
        .title(fl!("window-controls"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(settings::item(
                    &descriptions[active_window_hint],
                    toggler(page.show_active_hint).on_toggle(Message::ShowActiveWindowHint),
                ))
                .add(settings::item(
                    &descriptions[maximize],
                    toggler(cosmic::config::show_maximize()).on_toggle(Message::ShowMaximizeButton),
                ))
                .add(settings::item(
                    &descriptions[minimize],
                    toggler(cosmic::config::show_minimize()).on_toggle(Message::ShowMinimizeButton),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::WindowManagement)
        })
}

pub fn focus_navigation() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let focus_follows_cursor = descriptions.insert(fl!("focus-navigation", "focus-follows-cursor"));
    let focus_follows_cursor_delay =
        descriptions.insert(fl!("focus-navigation", "focus-follows-cursor-delay"));
    let cursor_follows_focus = descriptions.insert(fl!("focus-navigation", "cursor-follows-focus"));

    Section::default()
        .title(fl!("focus-navigation"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(settings::item(
                    &descriptions[focus_follows_cursor],
                    toggler(page.focus_follows_cursor).on_toggle(Message::SetFocusFollowsCursor),
                ))
                .add(settings::item(
                    &descriptions[focus_follows_cursor_delay],
                    widget::editable_input("", &page.focus_delay_text, false, |editing| {
                        Message::SaveFocusFollowsCursorDelay(!editing)
                    })
                    .select_on_focus(true)
                    .on_input(Message::SetFocusFollowsCursorDelay)
                    .on_submit(|_| Message::SaveFocusFollowsCursorDelay(true))
                    .width(Length::Fixed(80.0)),
                ))
                .add(settings::item(
                    &descriptions[cursor_follows_focus],
                    toggler(page.cursor_follows_focus).on_toggle(Message::SetCursorFollowsFocus),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::WindowManagement)
        })
}

fn super_key_active_config() -> Option<usize> {
    let super_binding = Binding::new(shortcuts::Modifiers::new().logo(), None);
    let config = shortcuts::context().ok()?;

    shortcuts::shortcuts(&config)
        .iter()
        .find(|(binding, _action)| binding == &&super_binding)
        .and_then(|(_, action)| match action {
            Action::System(shortcuts::action::System::Launcher) => Some(0),
            Action::System(shortcuts::action::System::WorkspaceOverview) => Some(1),
            Action::System(shortcuts::action::System::AppLibrary) => Some(2),
            _ => None,
        })
}

fn super_key_set(action: Option<shortcuts::action::System>) {
    let Ok(config) = shortcuts::context() else {
        return;
    };

    let mut shortcuts = config.get::<Shortcuts>("custom").unwrap_or_default();
    let action = action.map(Action::System).unwrap_or(Action::Disable);

    shortcuts.0.insert(
        Binding::new(shortcuts::Modifiers::new().logo(), None),
        action,
    );

    _ = config.set("custom", &shortcuts);
}
