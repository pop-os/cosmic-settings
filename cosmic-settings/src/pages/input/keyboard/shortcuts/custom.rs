// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only
//
use std::str::FromStr;
use std::time::Duration;

use super::{ShortcutBinding, ShortcutMessage, ShortcutModel};

use cosmic::app::ContextDrawer;
use cosmic::iced::keyboard::key::Named;
use cosmic::iced::keyboard::{Key, Location, Modifiers};
use cosmic::iced::{Alignment, Length};
use cosmic::iced_winit;
use cosmic::widget::{self, button, icon};
use cosmic::{Apply, Element, Task};
use cosmic_settings_config::Binding;
use cosmic_settings_config::shortcuts::{Action, Shortcuts};
use cosmic_settings_page::{self as page, Section, section};
use slab::Slab;
use slotmap::{Key as SlotKey, SlotMap};

pub struct Page {
    entity: page::Entity,
    model: super::Model,
    add_shortcut: AddShortcut,
    replace_dialog: Vec<(Binding, Action, String)>,
    task_id: widget::Id,
    name_id: widget::Id,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            entity: page::Entity::null(),
            model: super::Model::default().custom().actions(bindings),
            add_shortcut: AddShortcut::default(),
            replace_dialog: Vec::new(),
            task_id: widget::Id::unique(),
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
    /// Update the Task text input
    TaskInput(String),
    /// Toggle editing of the key text input
    EditCombination,
    /// Toggle editability of the key text input
    KeyEditing(usize, bool),
    /// Update the key text input
    KeyInput(usize, String),
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
    ModifiersChanged(Modifiers),
    KeyReleased(u32, Key, Location),
    KeyPressed(u32, Key, Location, Modifiers),
}

#[derive(Default)]
struct AddShortcut {
    pub active: bool,
    pub editing: Option<usize>,
    pub name: String,
    pub task: String,
    pub keys: Slab<(String, widget::Id)>,
    pub binding: Binding,
}

impl AddShortcut {
    pub fn enable(&mut self) {
        self.active = true;
        self.name.clear();
        self.task.clear();

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
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::TaskInput(text) => {
                self.add_shortcut.task = text;
            }

            Message::KeyInput(..) => {}

            Message::KeyEditing(id, enable) => {
                if enable {
                    self.add_shortcut.editing = Some(id);
                    return iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(true).discard();
                } else if self.add_shortcut.editing == Some(id) {
                    self.add_shortcut.editing = None;

                    return Task::batch(vec![
                        widget::text_input::focus(widget::Id::unique()),
                        iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(false).discard(),
                    ]);
                }
            }

            Message::NameInput(text) => {
                self.add_shortcut.name = text;
            }

            Message::AddKeybinding => return self.add_keybinding(),

            Message::AddShortcut => {
                let name = self.add_shortcut.name.trim();
                let task = self.add_shortcut.task.trim();

                if name.is_empty() || task.is_empty() {
                    return Task::none();
                }

                let mut addable_bindings = Vec::new();

                for (index, (keys, id)) in &self.add_shortcut.keys {
                    if !keys.is_empty() {
                        continue;
                    }

                    addable_bindings.push((index, id.clone()));
                }

                if let Some((index, binding)) = addable_bindings.first() {
                    self.add_shortcut.editing = Some(*index);
                    return Task::batch(vec![
                        widget::text_input::focus(binding.clone()),
                        iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(false).discard(),
                    ]);
                } else {
                    // make a new empty binding if none exist
                    let new_id = widget::Id::unique();
                    self.add_shortcut.editing = Some(
                        self.add_shortcut
                            .keys
                            .insert((String::new(), new_id.clone())),
                    );
                    return Task::batch(vec![
                        widget::text_input::focus(new_id.clone()),
                        iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(true).discard(),
                    ]);
                }
            }

            Message::EditCombination => {
                if let Some((slab_index, (_, id))) = self.add_shortcut.keys.iter().next() {
                    self.add_shortcut.editing = Some(slab_index);
                    return Task::batch(vec![
                        widget::text_input::focus(id.clone()),
                        widget::text_input::select_all(id.clone()),
                        iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(
                            true,
                        )
                        .discard()
                    ]);
                }
            }

            Message::NameSubmit => {
                if !self.add_shortcut.name.trim().is_empty() {
                    return widget::text_input::focus(self.task_id.clone());
                }
            }

            Message::ReplaceApply => {
                if let Some((mut binding, ..)) = self.replace_dialog.pop() {
                    self.model.config_remove(&binding);
                    binding.keycode = None;
                    self.add_shortcut(binding);

                    if self.replace_dialog.is_empty() {
                        self.add_shortcut = Default::default();
                        _ = self.model.on_enter();
                    }
                }
            }

            Message::ReplaceCancel => {
                _ = self.replace_dialog.pop();
                if self.replace_dialog.is_empty() {
                    self.add_shortcut = Default::default();
                    _ = self.model.on_enter();
                }
            }

            Message::Shortcut(message) => {
                if let ShortcutMessage::ShowShortcut(..) = message {
                    self.add_shortcut.active = false;
                }

                return self.model.update(message);
            }

            Message::ShortcutContext => {
                let name_id = self.name_id.clone();
                self.add_shortcut.enable();
                return Task::batch(vec![
                    cosmic::task::message(crate::app::Message::OpenContextDrawer(self.entity)),
                    // XX hack: wait a bit before focusing the input to avoid it being ignored before it exists
                    cosmic::task::future(async move {
                        tokio::time::sleep(Duration::from_millis(10)).await;
                    })
                    .then(move |_: ()| widget::text_input::focus(name_id.clone())),
                ]);
            }

            Message::ModifiersChanged(modifiers) => {
                if self.add_shortcut.active {
                    let mut cfg_modifiers = cosmic_settings_config::shortcuts::Modifiers::new();
                    if modifiers.alt() {
                        cfg_modifiers = cfg_modifiers.alt()
                    }
                    if modifiers.control() {
                        cfg_modifiers = cfg_modifiers.ctrl()
                    }
                    if modifiers.shift() {
                        cfg_modifiers = cfg_modifiers.shift()
                    }
                    if modifiers.logo() {
                        cfg_modifiers = cfg_modifiers.logo()
                    }
                    let old =
                        std::mem::replace(&mut self.add_shortcut.binding.modifiers, cfg_modifiers);

                    if self.add_shortcut.binding.keycode.is_none() && modifiers.is_empty() {
                        if old.logo {
                            // XX for now avoid applying the keycode
                            let binding = Binding {
                                modifiers: self.add_shortcut.binding.modifiers.clone(),
                                key: self.add_shortcut.binding.key,
                                keycode: None,
                                description: None,
                            };
                            let Some(k) = self
                                .add_shortcut
                                .keys
                                .get_mut(self.add_shortcut.editing.unwrap())
                            else {
                                return iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(false).discard();
                            };
                            k.0 = binding.to_string();

                            if self.add_shortcut.name.trim().is_empty()
                                || self.add_shortcut.task.trim().is_empty()
                            {
                                return Task::batch(vec![
                                    widget::text_input::focus(widget::Id::unique()),
                                    iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(false).discard(),
                                ]);
                            }
                            self.add_shortcut(binding);
                            _ = self.model.on_enter();

                            return Task::batch(vec![
                                widget::text_input::focus(widget::Id::unique()),
                                iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(false).discard(),
                            ]);
                        } else if old.alt || old.ctrl || old.shift {
                            self.add_shortcut = Default::default();
                            _ = self.model.on_enter();

                            return Task::batch(vec![
                                iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(false).discard()
                            ]);
                        }
                    }
                    if let Some(k) = self
                        .add_shortcut
                        .keys
                        .get_mut(self.add_shortcut.editing.unwrap())
                    {
                        k.0 = self.add_shortcut.binding.to_string();
                    }
                }
            }
            Message::KeyReleased(keycode, _, _) => {
                // if the currently selected shortcut matches, finish selecting shortcut
                if self.add_shortcut.editing.is_some()
                    && self.add_shortcut.active
                    && self.add_shortcut.binding.key.is_some()
                    && self
                        .add_shortcut
                        .binding
                        .keycode
                        .is_some_and(|k| k == keycode)
                    && self.add_shortcut.binding.modifiers
                        != cosmic_settings_config::shortcuts::Modifiers::new()
                    || self.add_shortcut.binding.key.is_some_and(|key| {
                        key.is_misc_function_key() || matches!(key.raw(), 0x10080001..=0x1008FFFF)
                    })
                {
                    // XX for now avoid applying the keycode
                    let binding = Binding {
                        modifiers: self.add_shortcut.binding.modifiers.clone(),
                        key: self.add_shortcut.binding.key,
                        keycode: None,
                        description: None,
                    };
                    let Some(k) = self
                        .add_shortcut
                        .keys
                        .get_mut(self.add_shortcut.editing.unwrap())
                    else {
                        return iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(false).discard();
                    };
                    k.0 = binding.to_string();

                    if self.add_shortcut.name.trim().is_empty()
                        || self.add_shortcut.task.trim().is_empty()
                    {
                        return Task::batch(vec![
                            widget::text_input::focus(widget::Id::unique()),
                            iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(false).discard(),
                        ]);
                    }
                    self.add_shortcut(binding);
                    _ = self.model.on_enter();

                    return Task::batch(vec![
                        widget::text_input::focus(widget::Id::unique()),
                        iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(false).discard(),
                    ]);
                }
            }
            Message::KeyPressed(keycode, unmodified_keysym, location, modifiers) => {
                if unmodified_keysym == Key::Named(Named::Escape) && modifiers.is_empty() {
                    self.add_shortcut.editing = None;
                    return Task::batch(vec![
                        widget::text_input::focus(widget::Id::unique()),
                        iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(false).discard(),
                    ]);
                }
                if self.add_shortcut.active {
                    self.add_shortcut.binding.keycode = Some(keycode);
                    self.add_shortcut.binding.key =
                        iced_winit::platform_specific::wayland::keymap::key_to_keysym(
                            unmodified_keysym,
                            location,
                        );
                    if let Some(k) = self
                        .add_shortcut
                        .keys
                        .get_mut(self.add_shortcut.editing.unwrap())
                    {
                        k.0 = self.add_shortcut.binding.to_string();
                    }
                }
            }
        }

        Task::none()
    }

    fn add_keybinding(&mut self) -> Task<crate::app::Message> {
        // If an empty entry exists, focus it instead of creating a new input.
        for (_, (binding, id)) in &mut self.add_shortcut.keys {
            if Binding::from_str(binding).is_ok() {
                continue;
            }

            binding.clear();

            return Task::batch(vec![
            widget::text_input::focus(id.clone()),
            iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(
                    true,
                )
                .discard(),
            ]);
        }

        let new_id = widget::Id::unique();
        self.add_shortcut.editing = Some(
            self.add_shortcut
                .keys
                .insert((String::new(), new_id.clone())),
        );

        Task::batch(vec![
            widget::text_input::focus(new_id.clone()),
            widget::text_input::select_all(new_id),
            iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(
                true,
            )
            .discard(),
        ])
    }

    fn add_keybinding_context(&self) -> Element<'_, Message> {
        let name_input = widget::text_input("", &self.add_shortcut.name)
            .padding([6, 12])
            .on_input(Message::NameInput)
            .on_submit(|_| Message::NameSubmit)
            .id(self.name_id.clone());

        let task_input = widget::text_input("", &self.add_shortcut.task)
            .padding([6, 12])
            .on_input(Message::TaskInput)
            .on_submit(|_| Message::EditCombination)
            .id(self.task_id.clone());

        let name_control = widget::column()
            .spacing(4)
            .push(widget::text::body(fl!("shortcut-name")))
            .push(name_input);

        let command_control = widget::column()
            .spacing(4)
            .push(widget::text::body(fl!("command")))
            .push(task_input);

        let input_fields = widget::column()
            .spacing(12)
            .push(name_control)
            .push(command_control)
            .padding([16, 24]);

        let keys = self.add_shortcut.keys.iter().fold(
            widget::list_column().spacing(0),
            |column, (id, (text, widget_id))| {
                let key_combination = widget::editable_input(
                    fl!("type-key-combination"),
                    text,
                    self.add_shortcut.editing == Some(id),
                    move |enable| Message::KeyEditing(id, enable),
                )
                .on_focus(Message::KeyEditing(id, true))
                .select_on_focus(true)
                .on_input(move |input| Message::KeyInput(id, input))
                .on_submit(|_| Message::AddKeybinding)
                .padding([0, 12])
                .id(widget_id.clone())
                .apply(widget::container)
                .padding([8, 24]);

                column.add(key_combination)
            },
        );

        let controls = widget::list_column().add(input_fields).add(keys).spacing(0);

        let add_keybinding_button = widget::button::standard(fl!("add-another-keybinding"))
            .on_press(Message::AddShortcut)
            .apply(widget::container)
            .width(Length::Fill)
            .align_x(Alignment::End);

        widget::column()
            .spacing(32)
            .push(controls)
            .push(add_keybinding_button)
            .into()
    }

    fn add_shortcut(&mut self, mut binding: Binding) {
        if let Some(action) = self.model.config_contains(&binding) {
            let action_str = super::localize_action(&action);
            self.replace_dialog.push((binding, action, action_str));
            return;
        }
        binding.description = Some(self.add_shortcut.name.clone());
        let new_action = Action::Spawn(self.add_shortcut.task.clone());
        self.model.config_add(new_action, binding);
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
        self.model.entity = entity;
    }

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

            let dialog = widget::dialog()
                .title(fl!("replace-shortcut-dialog"))
                .icon(icon::from_name("dialog-warning").size(64))
                .body(fl!(
                    "replace-shortcut-dialog",
                    "desc",
                    shortcut = binding.to_string(),
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

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        if self.add_shortcut.active {
            Some(
                cosmic::app::context_drawer(
                    self.add_keybinding_context()
                        .map(crate::pages::Message::CustomShortcuts),
                    crate::pages::Message::CloseContextDrawer,
                )
                .title(fl!("custom-shortcuts", "context")),
            )
        } else {
            self.model.context_drawer(|msg| {
                crate::pages::Message::CustomShortcuts(Message::Shortcut(msg))
            })
        }
    }

    fn on_context_drawer_close(&mut self) -> Task<crate::pages::Message> {
        self.model.on_context_drawer_close();
        Task::none()
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        self.add_shortcut = Default::default();
        _ = self.model.on_enter();
        Task::none()
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        self.model.on_clear();
        iced_winit::platform_specific::commands::keyboard_shortcuts_inhibit::inhibit_shortcuts(
            false,
        )
        .discard()
    }

    #[cfg(feature = "wayland")]
    fn subscription(
        &self,
        core: &cosmic::Core,
    ) -> cosmic::iced::Subscription<crate::pages::Message> {
        use cosmic::iced::{self, event::listen_with};

        cosmic::iced::Subscription::batch(vec![
            if self.add_shortcut.active
                && self.add_shortcut.editing.is_some()
                && self.replace_dialog.is_empty()
            {
                listen_with(|event, _, _| match event {
                    iced::event::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                        key,
                        physical_key,
                        location,
                        modifiers,
                        ..
                    }) => {
                        use cosmic::iced::keyboard::{Key, key::Named};
                        if matches!(
                            key,
                            Key::Named(Named::Super | Named::Alt | Named::Control | Named::Shift)
                        ) || matches!((&key, modifiers), (Key::Named(Named::Tab), modifiers) if modifiers.is_empty() || modifiers == Modifiers::SHIFT)
                        {
                            return None;
                        }
                        cosmic::iced_winit::conversion::physical_to_scancode(physical_key).map(
                            |code| {
                                crate::pages::Message::CustomShortcuts(Message::KeyPressed(
                                    code, key, location, modifiers,
                                ))
                            },
                        )
                    }
                    iced::event::Event::Keyboard(iced::keyboard::Event::KeyReleased {
                        key,
                        physical_key,
                        location,
                        ..
                    }) => {
                        use cosmic::iced::keyboard::{Key, key::Named};
                        if matches!(
                            key,
                            Key::Named(Named::Super | Named::Alt | Named::Control | Named::Shift)
                        ) {
                            return None;
                        }
                        cosmic::iced_winit::conversion::physical_to_scancode(physical_key).map(
                            |code| {
                                crate::pages::Message::CustomShortcuts(Message::KeyReleased(
                                    code, key, location,
                                ))
                            },
                        )
                    }
                    iced::event::Event::Keyboard(iced::keyboard::Event::ModifiersChanged(e)) => {
                        Some(crate::pages::Message::CustomShortcuts(
                            Message::ModifiersChanged(e),
                        ))
                    }

                    _ => None,
                })
            } else {
                cosmic::iced::Subscription::none()
            },
            self.model
                .subscription(core)
                .map(|m| crate::pages::Message::CustomShortcuts(Message::Shortcut(m))),
        ])
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn bindings(_defaults: &Shortcuts, keybindings: &Shortcuts) -> Slab<ShortcutModel> {
    keybindings
        .iter()
        .fold(Slab::new(), |mut slab, (binding, action)| {
            if let Action::Spawn(task) = action {
                let description = binding
                    .description
                    .clone()
                    .unwrap_or_else(|| task.to_owned());

                let new_binding = ShortcutBinding {
                    id: widget::Id::unique(),
                    binding: binding.clone(),
                    pending: binding.clone(),
                    input: String::new(),
                    is_default: false,
                    is_saved: true,
                };

                if let Some((_, existing_model)) =
                    slab.iter_mut().find(|(_, m)| &m.action == action)
                {
                    existing_model.description = description;
                    existing_model.bindings.insert(new_binding);
                } else {
                    slab.insert(ShortcutModel {
                        action: action.clone(),
                        bindings: {
                            let mut slab = Slab::new();
                            slab.insert(new_binding);
                            slab
                        },
                        description,
                        modified: 0,
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
                widget::settings::section()
                    .add(widget::settings::item_row(vec![
                        widget::text::body(fl!("custom-shortcuts", "none")).into(),
                    ]))
                    .into()
            } else {
                page.model.view().map(Message::Shortcut)
            };

            let add_shortcut = widget::button::standard(fl!("custom-shortcuts", "add"))
                .on_press(Message::ShortcutContext)
                .apply(widget::container)
                .width(Length::Fill)
                .align_x(Alignment::End);

            widget::column()
                .push(content)
                .push(add_shortcut)
                .spacing(24)
                .apply(Element::from)
                .map(crate::pages::Message::CustomShortcuts)
        })
}
