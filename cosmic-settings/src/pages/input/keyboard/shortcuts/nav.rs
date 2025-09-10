// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::{ShortcutMessage, ShortcutModel};
use cosmic::app::ContextDrawer;
use cosmic::{Element, Task};
use cosmic_settings_config::shortcuts::Action;
use cosmic_settings_config::shortcuts::action::{Direction, FocusDirection};
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
        page::Info::new("nav-shortcuts", "input-keyboard-symbolic").title(fl!("nav-shortcuts"))
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(shortcuts())])
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        self.model
            .context_drawer(crate::pages::Message::NavShortcuts)
    }

    fn dialog(&self) -> Option<Element<'_, crate::pages::Message>> {
        self.model
            .dialog()
            .map(|el| el.map(crate::pages::Message::NavShortcuts))
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
            .map(crate::pages::Message::NavShortcuts)
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

#[must_use]
pub const fn actions() -> &'static [Action] {
    &[
        Action::Focus(FocusDirection::Left),
        Action::Focus(FocusDirection::Right),
        Action::Focus(FocusDirection::Up),
        Action::Focus(FocusDirection::Down),
        Action::Focus(FocusDirection::In),
        Action::Focus(FocusDirection::Out),
        Action::PreviousWorkspace,
        Action::NextWorkspace,
        Action::LastWorkspace,
        Action::SwitchOutput(Direction::Left),
        Action::SwitchOutput(Direction::Right),
        Action::SwitchOutput(Direction::Up),
        Action::SwitchOutput(Direction::Down),
        Action::Workspace(1),
        Action::Workspace(2),
        Action::Workspace(3),
        Action::Workspace(4),
        Action::Workspace(5),
        Action::Workspace(6),
        Action::Workspace(7),
        Action::Workspace(8),
        Action::Workspace(9),
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
            page.model.view().map(crate::pages::Message::NavShortcuts)
        })
}
