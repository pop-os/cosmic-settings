// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::{ShortcutMessage, ShortcutModel};
use cosmic::{Element, Task, app::ContextDrawer};
use cosmic_settings_config::shortcuts::Action;
use cosmic_settings_config::shortcuts::action::System as SystemAction;
use cosmic_settings_page::{self as page, Section, section};
use slab::Slab;

pub struct Page {
    model: super::Model,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            model: super::Model::default().actions(|defaults, keybindings| {
                actions().iter().fold(Slab::new(), |mut slab, action| {
                    slab.insert(ShortcutModel::new(defaults, keybindings, action.clone()));
                    slab
                })
            }),
        }
    }
}

impl Page {
    pub fn update(&mut self, message: ShortcutMessage) -> Task<crate::app::Message> {
        self.model.update(message)
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.model.entity = entity;
    }

    fn info(&self) -> page::Info {
        page::Info::new("system-shortcut", "input-keyboard-symbolic").title(fl!("system-shortcut"))
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(shortcuts())])
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        self.model
            .context_drawer(crate::pages::Message::SystemShortcuts)
    }

    fn dialog(&self) -> Option<Element<'_, crate::pages::Message>> {
        self.model
            .dialog()
            .map(|el| el.map(crate::pages::Message::SystemShortcuts))
    }

    fn on_context_drawer_close(&mut self) -> Task<crate::pages::Message> {
        self.model.on_context_drawer_close();
        Task::none()
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        _ = self.model.on_enter();
        Task::none()
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        _ = self.model.on_clear();
        cosmic::iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(
            false,
        )
        .discard()
    }

    #[cfg(feature = "wayland")]
    fn subscription(
        &self,
        core: &cosmic::Core,
    ) -> cosmic::iced::Subscription<crate::pages::Message> {
        self.model
            .subscription(core)
            .map(crate::pages::Message::SystemShortcuts)
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

#[must_use]
pub const fn actions() -> &'static [Action] {
    &[
        Action::System(SystemAction::AppLibrary),
        Action::System(SystemAction::Launcher),
        Action::System(SystemAction::WorkspaceOverview),
        Action::System(SystemAction::WindowSwitcher),
        Action::System(SystemAction::WindowSwitcherPrevious),
        Action::System(SystemAction::LogOut),
        Action::System(SystemAction::LockScreen),
        Action::System(SystemAction::VolumeLower),
        Action::System(SystemAction::VolumeRaise),
        Action::System(SystemAction::Mute),
        Action::System(SystemAction::MuteMic),
        Action::System(SystemAction::PlayPause),
        Action::System(SystemAction::PlayNext),
        Action::System(SystemAction::PlayPrev),
        Action::System(SystemAction::BrightnessDown),
        Action::System(SystemAction::BrightnessUp),
        Action::System(SystemAction::KeyboardBrightnessDown),
        Action::System(SystemAction::KeyboardBrightnessUp),
        Action::System(SystemAction::InputSourceSwitch),
        Action::System(SystemAction::Screenshot),
        Action::System(SystemAction::Terminal),
        Action::System(SystemAction::HomeFolder),
        Action::System(SystemAction::WebBrowser),
        Action::System(SystemAction::TouchpadToggle),
    ]
}

fn shortcuts() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    // Make these searchable in the global settings search.
    for action in actions() {
        descriptions.insert(super::localize_action(action));
    }

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, _section| {
            page.model
                .view()
                .map(crate::pages::Message::SystemShortcuts)
        })
}
