use std::str::FromStr;

use super::{ShortcutMessage, ShortcutModel};
use cosmic::iced::alignment::Horizontal;
use cosmic::iced::Length;
use cosmic::widget::{self, button, icon};
use cosmic::{Apply, Command, Element};
use cosmic_settings_config::shortcuts::{Action, Shortcuts};
use cosmic_settings_config::Binding;
use cosmic_settings_page::{self as page, section, Section};
use slab::Slab;
use slotmap::SlotMap;

pub struct Page {
    model: super::Model,
    add_shortcut: AddShortcut,
    replace_dialog: Option<(Binding, Action, String)>,
    command_id: widget::Id,
    name_id: widget::Id,
    key_id: widget::Id,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            model: super::Model::default().custom(),
            add_shortcut: AddShortcut::default(),
            replace_dialog: None,
            command_id: widget::Id::unique(),
            name_id: widget::Id::unique(),
            key_id: widget::Id::unique(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    /// Add a new custom shortcut to the config
    AddShortcut,
    /// Clear the command text input
    CommandClear,
    /// Update the command text input
    CommandInput(String),
    /// Toggle editing of the key text input
    EditCombination(bool),
    /// Focus the edit input
    FocusEditInput,
    /// Clear the key text input
    KeyClear,
    /// Update the key text input
    KeyInput(String),
    /// Clear the name text input
    NameClear,
    /// Update the name text input
    NameInput(String),
    /// Enter key pressed in the name text input
    NameSubmit,
    /// Apply a requested shortcut replace operation
    ReplaceApply,
    /// Cancel a requested shortcut replace operation
    ReplaceCancel,
    /// Emit a generic shortcut message
    Shortcut(ShortcutMessage),
    /// Open the add shortcut context drawer
    ShortcutContext,
}

#[derive(Default)]
struct AddShortcut {
    pub active: bool,
    pub name: String,
    pub command: String,
    pub combination: String,
    pub editing_combination: bool,
}

impl AddShortcut {
    pub fn enable(&mut self) {
        self.active = true;
        self.editing_combination = false;
        self.name.clear();
        self.command.clear();
        self.combination.clear();
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        match message {
            Message::CommandInput(text) => {
                self.add_shortcut.command = text;
            }

            Message::KeyInput(text) => {
                self.add_shortcut.combination = text;
            }

            Message::NameInput(text) => {
                self.add_shortcut.name = text;
            }

            Message::AddShortcut => {
                let name = self.add_shortcut.name.trim();
                let command = self.add_shortcut.command.trim();
                let keys = self.add_shortcut.combination.trim();

                if name.is_empty() || command.is_empty() || keys.is_empty() {
                    return Command::none();
                }

                let Ok(binding) = Binding::from_str(keys) else {
                    self.add_shortcut.combination.clear();
                    return Command::none();
                };

                if let Some(action) = self.model.config_contains(&binding) {
                    let action_str = super::localize_action(&action);
                    self.replace_dialog = Some((binding, action, action_str));
                    return Command::none();
                }

                self.add_shortcut(binding);
            }

            Message::CommandClear => {
                self.add_shortcut.command.clear();
            }

            Message::EditCombination(enable) => {
                self.add_shortcut.editing_combination = enable;
                if enable {
                    return cosmic::command::future(async {
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        crate::pages::Message::CustomShortcuts(Message::FocusEditInput)
                    });
                }
            }

            Message::FocusEditInput => {
                return widget::text_input::focus(self.key_id.clone());
            }

            Message::KeyClear => {
                self.add_shortcut.combination.clear();
            }

            Message::NameClear => {
                self.add_shortcut.name.clear();
            }

            Message::NameSubmit => {
                if !self.add_shortcut.name.trim().is_empty() {
                    return widget::text_input::focus(self.command_id.clone());
                }
            }

            Message::ReplaceApply => {
                if let Some((binding, old_action, _)) = self.replace_dialog.take() {
                    self.model.config_remove(&old_action, &binding);
                    self.add_shortcut(binding);
                }
            }

            Message::ReplaceCancel => {
                self.replace_dialog = None;
            }

            Message::Shortcut(message) => {
                if let ShortcutMessage::ShowShortcut(..) = message {
                    self.add_shortcut.active = false;
                }

                return self.model.update(message);
            }

            Message::ShortcutContext => {
                self.add_shortcut.enable();
                return Command::batch(vec![
                    cosmic::command::message(crate::app::Message::OpenContextDrawer(
                        fl!("custom-shortcuts", "context").into(),
                    )),
                    widget::text_input::focus(self.name_id.clone()),
                ]);
            }
        }

        Command::none()
    }

    fn add_keybinding_context(&self) -> Element<'_, Message> {
        let name_input = widget::text_input("", &self.add_shortcut.name)
            .on_clear(Message::NameClear)
            .on_input(Message::NameInput)
            .on_submit(Message::NameSubmit)
            .id(self.name_id.clone());

        let command_input = widget::text_input("", &self.add_shortcut.command)
            .on_clear(Message::CommandClear)
            .on_input(Message::CommandInput)
            .on_submit(Message::EditCombination(true))
            .id(self.command_id.clone());

        let name_control = widget::column()
            .spacing(4)
            .push(widget::text::body(fl!("shortcut-name")))
            .push(name_input);

        let command_control = widget::column()
            .spacing(4)
            .push(widget::text::body(fl!("command")))
            .push(command_input);

        let input_fields = widget::column()
            .spacing(24)
            .push(name_control)
            .push(command_control);

        let key_combination =
            widget::text_input(fl!("type-key-combination"), &self.add_shortcut.combination)
                .style(cosmic::theme::TextInput::Inline)
                .on_clear(Message::KeyClear)
                .on_input(Message::KeyInput)
                .on_submit(Message::AddShortcut)
                .id(self.key_id.clone());

        let controls = widget::settings::view_section("")
            .add(input_fields)
            .add(key_combination);

        let add_keybinding_button = widget::button::standard(fl!("add-keybinding"))
            .on_press(Message::AddShortcut)
            .apply(widget::container)
            .width(Length::Fill)
            .align_x(Horizontal::Right);

        widget::column()
            .spacing(24)
            .push(controls)
            .push(add_keybinding_button)
            .padding([24, 24])
            .into()
    }

    fn add_shortcut(&mut self, mut binding: Binding) {
        binding.description = Some(std::mem::take(&mut self.add_shortcut.name));
        let new_action = Action::Spawn(std::mem::take(&mut self.add_shortcut.command));
        self.model.config_add(new_action, binding);
        self.model.on_enter(bindings);
        self.add_shortcut.active = false;
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("custom-shortcuts", "input-keyboard-symbolic")
            .title(fl!("custom-shortcuts"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(shortcuts())])
    }

    fn dialog(&self) -> Option<Element<'_, crate::pages::Message>> {
        // Check if a new shortcut is being added that requires a replace dialog.
        if let Some((binding, _action, action_str)) = self.replace_dialog.as_ref() {
            let primary_action = button::suggested(fl!("replace")).on_press(Message::ReplaceApply);

            let secondary_action = button::standard(fl!("cancel")).on_press(Message::ReplaceCancel);

            let dialog = cosmic::widget::dialog(fl!("replace-shortcut-dialog"))
                .icon(icon::from_name("dialog-warning").size(64))
                .body(fl!(
                    "replace-shortcut-dialog",
                    "desc",
                    shortcut = binding
                        .description
                        .clone()
                        .unwrap_or_else(|| fl!("unknown")),
                    name = action_str.clone()
                ))
                .primary_action(primary_action)
                .secondary_action(secondary_action)
                .apply(Element::from)
                .map(crate::pages::Message::CustomShortcuts);

            return Some(dialog);
        }

        // Check if a keybinding is being added that requires a replace dialog.
        self.model
            .dialog()
            .map(|el| el.map(|m| crate::pages::Message::CustomShortcuts(Message::Shortcut(m))))
    }

    fn context_drawer(&self) -> Option<Element<'_, crate::pages::Message>> {
        if self.add_shortcut.active {
            Some(self.add_keybinding_context())
        } else {
            self.model
                .context_drawer()
                .map(|el| el.map(Message::Shortcut))
        }
        .map(|el| el.map(crate::pages::Message::CustomShortcuts))
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

fn bindings(keybindings: &Shortcuts) -> Slab<ShortcutModel> {
    keybindings
        .iter()
        .filter_map(|(binding, action)| {
            if let Action::Spawn(command) = action {
                return Some(ShortcutModel {
                    action: action.clone(),
                    bindings: {
                        let id = widget::Id::unique();
                        let mut slab = Slab::new();
                        slab.insert((id, binding.clone(), String::new(), false));
                        slab
                    },
                    description: binding
                        .description
                        .clone()
                        .unwrap_or_else(|| command.to_owned()),
                });
            }

            None
        })
        .fold(Slab::new(), |mut slab, model| {
            slab.insert(model);
            slab
        })
}

fn shortcuts() -> Section<crate::pages::Message> {
    let descriptions = Slab::new();

    // TODO: Add shortcuts to descriptions

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, _section| {
            let content = if page.model.shortcut_models.is_empty() {
                widget::settings::view_section("")
                    .add(widget::settings::item_row(vec![widget::text::body(fl!(
                        "custom-shortcuts",
                        "none"
                    ))
                    .into()]))
                    .into()
            } else {
                page.model.view().map(Message::Shortcut)
            };

            let add_shortcut = widget::button::standard(fl!("custom-shortcuts", "add"))
                .on_press(Message::ShortcutContext)
                .apply(widget::container)
                .width(Length::Fill)
                .align_x(Horizontal::Right);

            widget::column()
                .push(content)
                .push(add_shortcut)
                .spacing(24)
                .apply(Element::from)
                .map(crate::pages::Message::CustomShortcuts)
        })
}
