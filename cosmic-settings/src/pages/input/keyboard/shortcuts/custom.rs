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
    replace_dialog: Vec<(Binding, Action, String)>,
    command_id: widget::Id,
    name_id: widget::Id,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            model: super::Model::default().custom(),
            add_shortcut: AddShortcut::default(),
            replace_dialog: Vec::new(),
            command_id: widget::Id::unique(),
            name_id: widget::Id::unique(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    /// Adds a new key binding input
    AddKeybinding,
    /// Add a new custom shortcut to the config
    AddShortcut,
    /// Clear the command text input
    CommandClear,
    /// Update the command text input
    CommandInput(String),
    /// Toggle editing of the key text input
    EditCombination,
    /// Clear the key text input
    KeyClear(usize),
    /// Update the key text input
    KeyInput(usize, String),
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
    pub keys: Slab<(String, widget::Id)>,
}

impl AddShortcut {
    pub fn enable(&mut self) {
        self.active = true;
        self.name.clear();
        self.command.clear();

        if self.keys.is_empty() {
            self.keys.insert((String::new(), widget::Id::unique()));
        } else {
            while self.keys.len() > 1 {
                self.keys.remove(self.keys.len() - 1);
            }

            self.keys[0].0.clear();
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        match message {
            Message::CommandInput(text) => {
                self.add_shortcut.command = text;
            }

            Message::KeyInput(id, text) => {
                self.add_shortcut.keys[id].0 = text;
            }

            Message::NameInput(text) => {
                self.add_shortcut.name = text;
            }

            Message::AddKeybinding => {
                let new_id = widget::Id::unique();
                self.add_shortcut
                    .keys
                    .insert((String::new(), new_id.clone()));
                return widget::text_input::focus(new_id);
            }

            Message::AddShortcut => {
                let name = self.add_shortcut.name.trim();
                let command = self.add_shortcut.command.trim();

                if name.is_empty() || command.is_empty() {
                    return Command::none();
                }

                let mut addable_bindings = Vec::new();

                for (id, (keys, key_id)) in &self.add_shortcut.keys {
                    if keys.is_empty() {
                        continue;
                    }

                    let Ok(binding) = Binding::from_str(keys) else {
                        return Command::none();
                    };

                    if !binding.is_set() {
                        return Command::none();
                    }

                    if let Some(action) = self.model.config_contains(&binding) {
                        let action_str = super::localize_action(&action);
                        self.replace_dialog.push((binding, action, action_str));
                        continue;
                    }

                    addable_bindings.push(binding);
                }

                for binding in addable_bindings {
                    self.add_shortcut(binding);
                }

                self.model.on_enter(bindings);
            }

            Message::CommandClear => {
                self.add_shortcut.command.clear();
            }

            Message::EditCombination => {
                return widget::text_input::focus(self.add_shortcut.keys[0].1.clone());
            }

            Message::KeyClear(id) => {
                self.add_shortcut.keys[id].0.clear();
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
                if let Some((binding, old_action, _)) = self.replace_dialog.pop() {
                    self.model.config_remove(&old_action, &binding);
                    self.add_shortcut(binding);

                    if self.replace_dialog.is_empty() {
                        self.model.on_enter(bindings);
                    }
                }
            }

            Message::ReplaceCancel => {
                _ = self.replace_dialog.pop();
                if self.replace_dialog.is_empty() {
                    self.model.on_enter(bindings);
                }
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
            .on_submit(Message::EditCombination)
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

        let keys = self.add_shortcut.keys.iter().fold(
            widget::column(),
            |column, (id, (text, widget_id))| {
                let key_combination = widget::text_input(fl!("type-key-combination"), text)
                    .style(cosmic::theme::TextInput::Inline)
                    .on_clear(Message::KeyClear(id))
                    .on_input(move |input| Message::KeyInput(id, input))
                    .on_submit(Message::AddKeybinding)
                    .id(widget_id.clone());

                column.push(key_combination)
            },
        );

        let controls = widget::settings::view_section("")
            .add(input_fields)
            .add(keys);

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
        self.add_shortcut.active = !self.replace_dialog.is_empty();
        binding.description = Some(self.add_shortcut.name.clone());
        let new_action = Action::Spawn(self.add_shortcut.command.clone());
        self.model.config_add(new_action, binding);
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
        if let Some((binding, _action, action_str)) = self.replace_dialog.last() {
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
        .fold(Slab::new(), |mut slab, (binding, action)| {
            if let Action::Spawn(command) = action {
                let description = binding
                    .description
                    .clone()
                    .unwrap_or_else(|| command.to_owned());

                if let Some((_, existing_model)) =
                    slab.iter_mut().find(|(_, m)| &m.action == action)
                {
                    let id = widget::Id::unique();
                    existing_model.description = description;
                    existing_model
                        .bindings
                        .insert((id, binding.clone(), String::new(), false));
                } else {
                    slab.insert(ShortcutModel {
                        action: action.clone(),
                        bindings: {
                            let id = widget::Id::unique();
                            let mut slab = Slab::new();
                            slab.insert((id, binding.clone(), String::new(), false));
                            slab
                        },
                        description,
                    });
                }
            }

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
