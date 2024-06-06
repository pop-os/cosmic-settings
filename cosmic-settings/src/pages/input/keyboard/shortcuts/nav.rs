use super::{ShortcutMessage, ShortcutModel};
use cascade::cascade;
use cosmic::{Command, Element};
use cosmic_settings_config::shortcuts::action::{Direction, FocusDirection};
use cosmic_settings_config::shortcuts::{Action, Shortcuts};
use cosmic_settings_page::{self as page, section, Section};
use slab::Slab;

#[derive(Default)]
pub struct Page {
    model: super::Model,
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
        self.model.on_enter(bindings);

        Command::none()
    }

    fn on_leave(&mut self) -> Command<crate::pages::Message> {
        self.model.on_clear();
        Command::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

pub fn bindings(keybindings: &Shortcuts) -> Slab<ShortcutModel> {
    cascade! {
        let shortcuts = Slab::new();
        ..insert(ShortcutModel::new(
            keybindings,
            Action::Focus(FocusDirection::Left),
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::Focus(FocusDirection::Right),
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::Focus(FocusDirection::Up),
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::Focus(FocusDirection::Down),
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::Focus(FocusDirection::In),
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::Focus(FocusDirection::Out),
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::PreviousWorkspace,
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::NextWorkspace,
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::LastWorkspace,
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::PreviousOutput,
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::NextOutput,
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::SwitchOutput(Direction::Left),
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::SwitchOutput(Direction::Right),
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::SwitchOutput(Direction::Up),
        ));
        ..insert(ShortcutModel::new(
            keybindings,
            Action::SwitchOutput(Direction::Down),
        ));
    }
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
