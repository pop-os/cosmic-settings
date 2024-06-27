use super::{ShortcutMessage, ShortcutModel};
use cosmic::{Command, Element};
use cosmic_settings_config::shortcuts::action::{Direction, FocusDirection};
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
        page::Info::new("nav-shortcuts", "input-keyboard-symbolic").title(fl!("nav-shortcuts"))
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
            .map(|el| el.map(crate::pages::Message::NavShortcuts))
    }

    fn dialog(&self) -> Option<Element<'_, crate::pages::Message>> {
        self.model
            .dialog()
            .map(|el| el.map(crate::pages::Message::NavShortcuts))
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
        Action::Focus(FocusDirection::Left),
        Action::Focus(FocusDirection::Right),
        Action::Focus(FocusDirection::Up),
        Action::Focus(FocusDirection::Down),
        Action::Focus(FocusDirection::In),
        Action::Focus(FocusDirection::Out),
        Action::PreviousWorkspace,
        Action::NextWorkspace,
        Action::LastWorkspace,
        Action::PreviousOutput,
        Action::NextOutput,
        Action::SwitchOutput(Direction::Left),
        Action::SwitchOutput(Direction::Right),
        Action::SwitchOutput(Direction::Up),
        Action::SwitchOutput(Direction::Down),
    ]
}

fn shortcuts() -> Section<crate::pages::Message> {
    let descriptions = Slab::new();

    // TODO: Add shortcuts to descriptions

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, _section| {
            page.model.view().map(crate::pages::Message::NavShortcuts)
        })
}
