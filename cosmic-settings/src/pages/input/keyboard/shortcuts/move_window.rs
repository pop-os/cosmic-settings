use super::{ShortcutMessage, ShortcutModel};
use cosmic::{Command, Element};
use cosmic_settings_config::shortcuts::action::Direction;
use cosmic_settings_config::shortcuts::Action;
use cosmic_settings_page::{self as page, section, Section};
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
    pub fn update(&mut self, message: ShortcutMessage) -> Command<crate::app::Message> {
        self.model.update(message)
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("move-windows", "input-keyboard-symbolic").title(fl!("move-windows"))
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(shortcuts())])
    }

    fn context_drawer(&self) -> Option<Element<'_, crate::pages::Message>> {
        self.model
            .context_drawer()
            .map(|el| el.map(crate::pages::Message::MoveWindowShortcuts))
    }

    fn dialog(&self) -> Option<Element<'_, crate::pages::Message>> {
        self.model
            .dialog()
            .map(|el| el.map(crate::pages::Message::MoveWindowShortcuts))
    }

    fn on_enter(
        &mut self,
        _page: cosmic_settings_page::Entity,
        _sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> Command<crate::pages::Message> {
        self.model.on_enter();

        Command::none()
    }

    fn on_leave(&mut self) -> Command<crate::pages::Message> {
        self.model.on_clear();
        Command::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

#[must_use]
pub const fn actions() -> &'static [Action] {
    &[
        Action::Move(Direction::Down),
        Action::Move(Direction::Left),
        Action::Move(Direction::Right),
        Action::Move(Direction::Up),
        Action::MoveToPreviousWorkspace,
        Action::MoveToNextWorkspace,
        Action::MoveToLastWorkspace,
        Action::MoveToWorkspace(1),
        Action::MoveToWorkspace(2),
        Action::MoveToWorkspace(3),
        Action::MoveToWorkspace(4),
        Action::MoveToWorkspace(5),
        Action::MoveToWorkspace(6),
        Action::MoveToWorkspace(7),
        Action::MoveToWorkspace(8),
        Action::MoveToWorkspace(9),
        Action::MoveToPreviousOutput,
        Action::MoveToNextOutput,
        Action::MoveToOutput(Direction::Down),
        Action::MoveToOutput(Direction::Left),
        Action::MoveToOutput(Direction::Right),
        Action::MoveToOutput(Direction::Up),
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
                .map(crate::pages::Message::MoveWindowShortcuts)
        })
}
