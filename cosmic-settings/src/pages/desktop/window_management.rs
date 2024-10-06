// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    config::CosmicTk,
    iced::Length,
    widget::{self, settings, toggler},
    Apply, Element,
};

use cosmic_config::{ConfigGet, ConfigSet, CosmicConfigEntry};
use cosmic_settings_config::{shortcuts, Action, Binding, Shortcuts};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slab::Slab;
use slotmap::SlotMap;
use tracing::error;

#[derive(Clone, Debug)]
pub enum Message {
    SuperKey(usize),
    SetFocusFollowsCursor(bool),
    SaveFocusFollowsCursorDelay(bool),
    SetFocusFollowsCursorDelay(String),
    SetCursorFollowsFocus(bool),
    ShowActiveWindowHint(bool),
    ShowMaximizeButton(bool),
    ShowMinimizeButton(bool),
}

pub struct Page {
    pub super_key_selections: Vec<String>,
    pub super_key_active: Option<usize>,
    comp_config: cosmic_config::Config,
    cosmic_tk: CosmicTk,
    focus_follows_cursor: bool,
    focus_follows_cursor_delay: u64,
    focus_delay_text: String,
    cursor_follows_focus: bool,
    show_active_hint: bool,
}

impl Default for Page {
    fn default() -> Self {
        let (cosmic_tk, comp_config) = CosmicTk::config().map_or_else(
            |why| {
                tracing::error!(?why, "failed to read CosmicTk config");
                let default_comp_config =
                    cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();

                (CosmicTk::default(), default_comp_config)
            },
            |config| match CosmicTk::get_entry(&config) {
                Ok(tk) => (tk, config),
                Err((_errors, tk)) => (tk, config),
            },
        );
        let focus_follows_cursor = comp_config
            .get("focus_follows_cursor")
            .unwrap_or_else(|err| {
                if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                    error!(?err, "Failed to read config 'focus_follows_cursor'");
                }
                false
            });
        let cursor_follows_focus = comp_config
            .get("cursor_follows_focus")
            .unwrap_or_else(|err| {
                if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                    error!(?err, "Failed to read config 'cursor_follows_focus'");
                }
                false
            });

        let focus_follows_cursor_delay = comp_config
            .get("focus_follows_cursor_delay")
            .inspect_err(|err| {
                if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                    error!(?err, "Failed to read config 'focus_follows_cursor_delay'")
                }
            })
            .unwrap_or(250);

        let show_active_hint = comp_config
            .get("active_hint")
            .inspect_err(|err| {
                if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                    error!(?err, "Failed to read config 'active_hint'")
                }
            })
            .unwrap_or(true);

        Page {
            super_key_selections: vec![
                fl!("super-key", "launcher"),
                fl!("super-key", "workspaces"),
                fl!("super-key", "applications"),
                fl!("super-key", "disable"),
            ],
            super_key_active: super_key_active_config(),
            comp_config,
            cosmic_tk,
            focus_follows_cursor,
            focus_follows_cursor_delay,
            focus_delay_text: format!("{focus_follows_cursor_delay}"),
            cursor_follows_focus,
            show_active_hint,
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SuperKey(id) => {
                let action = match id {
                    0 => Some(shortcuts::action::System::Launcher),
                    1 => Some(shortcuts::action::System::WorkspaceOverview),
                    2 => Some(shortcuts::action::System::AppLibrary),
                    3 => None,
                    _ => return,
                };

                self.super_key_active = Some(id);
                super_key_set(action);
            }
            Message::SetFocusFollowsCursor(value) => {
                self.focus_follows_cursor = value;
                if let Err(err) = self
                    .comp_config
                    .set("focus_follows_cursor", &self.focus_follows_cursor)
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
                    .set("cursor_follows_focus", &self.cursor_follows_focus)
                {
                    error!(?err, "Failed to set config 'cursor_follows_focus'");
                }
            }
            Message::ShowActiveWindowHint(value) => {
                self.show_active_hint = value;
                if let Err(err) = self.comp_config.set("active_hint", value) {
                    error!(?err, "Failed to set config 'active_hint'");
                }
            }
            Message::ShowMaximizeButton(value) => {
                let _res = self.cosmic_tk.set_show_maximize(&self.comp_config, value);
            }

            Message::ShowMinimizeButton(value) => {
                let _res = self.cosmic_tk.set_show_minimize(&self.comp_config, value);
            }
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(super_key_action()),
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

pub fn super_key_action() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let super_key = descriptions.insert(fl!("super-key"));
    let _launcher = descriptions.insert(fl!("super-key", "launcher"));
    let _workspaces = descriptions.insert(fl!("super-key", "workspaces"));
    let _applications = descriptions.insert(fl!("super-key", "applications"));
    let _disable = descriptions.insert(fl!("super-key", "disable"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(
                    settings::item::builder(&descriptions[super_key]).control(widget::dropdown(
                        &page.super_key_selections,
                        page.super_key_active,
                        Message::SuperKey,
                    )),
                )
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
                    toggler(None, page.show_active_hint, Message::ShowActiveWindowHint),
                ))
                .add(settings::item(
                    &descriptions[maximize],
                    toggler(
                        None,
                        page.cosmic_tk.show_maximize,
                        Message::ShowMaximizeButton,
                    ),
                ))
                .add(settings::item(
                    &descriptions[minimize],
                    toggler(
                        None,
                        page.cosmic_tk.show_minimize,
                        Message::ShowMinimizeButton,
                    ),
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
                    toggler(
                        None,
                        page.focus_follows_cursor,
                        Message::SetFocusFollowsCursor,
                    ),
                ))
                .add(settings::item(
                    &descriptions[focus_follows_cursor_delay],
                    widget::editable_input("", &page.focus_delay_text, false, |editing| {
                        Message::SaveFocusFollowsCursorDelay(!editing)
                    })
                    .select_on_focus(true)
                    .on_input(Message::SetFocusFollowsCursorDelay)
                    .on_submit(Message::SaveFocusFollowsCursorDelay(true))
                    .width(Length::Fixed(80.0)),
                ))
                .add(settings::item(
                    &descriptions[cursor_follows_focus],
                    toggler(
                        None,
                        page.cursor_follows_focus,
                        Message::SetCursorFollowsFocus,
                    ),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::WindowManagement)
        })
}

fn super_key_active_config() -> Option<usize> {
    let super_binding = Binding::new(shortcuts::Modifiers::new().logo(), None);

    let config = shortcuts::context().ok()?;
    let shortcuts = shortcuts::shortcuts(&config);

    let new_id = shortcuts
        .iter()
        .find(|(binding, _action)| binding == &&super_binding)
        .and_then(|(_, action)| match action {
            Action::System(shortcuts::action::System::Launcher) => Some(0),
            Action::System(shortcuts::action::System::WorkspaceOverview) => Some(1),
            Action::System(shortcuts::action::System::AppLibrary) => Some(2),
            _ => None,
        });

    new_id
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
