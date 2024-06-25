use cosmic::widget::{self, button, icon, settings};
use cosmic::{command, Command, Element};
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_config::shortcuts::{self, Action, Binding, Shortcuts};
use slab::Slab;
use std::io;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum ShortcutMessage {
    AddKeybinding,
    ApplyReplace,
    CancelReplace,
    ClearBinding(usize),
    DeleteBinding(usize),
    DeleteShortcut(usize),
    EditBinding(usize, bool),
    InputBinding(usize, String),
    ShowShortcut(usize, String),
    SubmitBinding(usize),
}

#[must_use]
#[derive(Debug)]
pub struct ShortcutModel {
    pub action: Action,
    pub bindings: Slab<(widget::Id, Binding, String, bool)>,
    pub description: String,
}

impl ShortcutModel {
    pub fn new(shortcuts: &Shortcuts, action: Action) -> Self {
        Self {
            bindings: shortcuts
                .shortcuts(&action)
                .fold(Slab::new(), |mut slab, binding| {
                    let id = widget::Id::unique();
                    slab.insert((id, binding.clone(), String::new(), false));
                    slab
                }),
            description: super::localize_action(&action),
            action,
        }
    }
}

#[must_use]
pub struct Model {
    pub defaults: Shortcuts,
    pub replace_dialog: Option<(usize, Binding, Action, String)>,
    pub shortcut_models: Slab<ShortcutModel>,
    pub shortcut_context: Option<usize>,
    pub config: cosmic_config::Config,
    pub custom: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            defaults: Shortcuts::default(),
            replace_dialog: None,
            shortcut_models: Slab::new(),
            shortcut_context: None,
            config: shortcuts::context().unwrap(),
            custom: false,
        }
    }
}

impl Model {
    pub fn custom(mut self) -> Self {
        self.custom = true;
        self
    }

    pub(super) fn context_drawer(&self) -> Option<Element<'_, ShortcutMessage>> {
        self.shortcut_context
            .as_ref()
            .map(|id| super::context_drawer(&self.shortcut_models, *id, self.custom))
    }

    pub(super) fn dialog(&self) -> Option<Element<'_, ShortcutMessage>> {
        if let Some(&(id, _, _, ref action)) = self.replace_dialog.as_ref() {
            if let Some(short_id) = self.shortcut_context {
                if let Some(model) = self.shortcut_models.get(short_id) {
                    if let Some((_, binding, shortcut, _)) = model.bindings.get(id) {
                        let primary_action = button::suggested(fl!("replace"))
                            .on_press(ShortcutMessage::ApplyReplace);

                        let secondary_action = button::standard(fl!("cancel"))
                            .on_press(ShortcutMessage::CancelReplace);

                        let dialog = cosmic::widget::dialog(fl!("replace-shortcut-dialog"))
                            .icon(icon::from_name("dialog-warning").size(64))
                            .body(fl!(
                                "replace-shortcut-dialog",
                                "desc",
                                shortcut = shortcut.clone(),
                                name = binding.description.as_ref().unwrap_or(action).to_owned()
                            ))
                            .primary_action(primary_action)
                            .secondary_action(secondary_action);

                        return Some(dialog.into());
                    }
                }
            }
        }

        None
    }

    pub(super) fn on_enter(&mut self, bindings: fn(&Shortcuts) -> Slab<ShortcutModel>) {
        let mut shortcuts = self.config.get::<Shortcuts>("defaults").unwrap_or_default();

        self.defaults = shortcuts.clone();

        if let Ok(custom) = self.config.get::<Shortcuts>("custom") {
            for (binding, action) in custom.0 {
                shortcuts.0.remove(&binding);
                shortcuts.0.insert(binding, action);
            }
        }

        self.shortcut_models = bindings(&shortcuts);
    }

    pub(super) fn on_clear(&mut self) {
        self.shortcut_models.clear();
    }

    pub(super) fn view(&self) -> Element<ShortcutMessage> {
        self.shortcut_models
            .iter()
            .map(|(id, shortcut)| super::shortcut_item(self.custom, id, shortcut))
            .fold(settings::view_section(""), settings::Section::add)
            .into()
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn update(&mut self, message: ShortcutMessage) -> Command<crate::app::Message> {
        match message {
            ShortcutMessage::AddKeybinding => {
                if let Some(short_id) = self.shortcut_context {
                    if let Some(model) = self.shortcut_models.get_mut(short_id) {
                        let id = cosmic::widget::Id::unique();
                        model.bindings.insert((
                            id.clone(),
                            Binding::default(),
                            String::new(),
                            true,
                        ));

                        return cosmic::widget::text_input::focus(id);
                    }
                }
            }

            ShortcutMessage::ApplyReplace => {
                if let Some((id, new_binding, action, _)) = self.replace_dialog.take() {
                    if let Some(short_id) = self.shortcut_context {
                        // Remove conflicting bindings that are saved on disk.
                        self.config_remove(&action, &new_binding);

                        // Clear any binding that matches this in the current model
                        for (_, model) in &mut self.shortcut_models {
                            if let Some(id) = model
                                .bindings
                                .iter()
                                .find(|(_, (_, binding, ..))| binding == &new_binding)
                                .map(|(id, _)| id)
                            {
                                model.bindings.remove(id);
                                break;
                            }
                        }

                        // Update the current model and save the binding to disk.
                        if let Some(model) = self.shortcut_models.get_mut(short_id) {
                            if let Some((_, binding, editing, is_editing)) =
                                model.bindings.get_mut(id)
                            {
                                let prev_binding = binding.clone();

                                *binding = new_binding.clone();
                                editing.clear();
                                *is_editing = false;

                                let action = model.action.clone();
                                self.config_remove(&action, &prev_binding);
                                self.config_add(action, new_binding);
                            }
                        }
                    }
                }
            }

            ShortcutMessage::CancelReplace => self.replace_dialog = None,

            ShortcutMessage::ClearBinding(id) => {
                if let Some(short_id) = self.shortcut_context {
                    if let Some(model) = self.shortcut_models.get_mut(short_id) {
                        if let Some((_, _, editing, _)) = model.bindings.get_mut(id) {
                            editing.clear();
                        }
                    }
                }
            }

            ShortcutMessage::DeleteBinding(id) => {
                if let Some(short_id) = self.shortcut_context {
                    if let Some(model) = self.shortcut_models.get_mut(short_id) {
                        let (_, removed, ..) = model.bindings.remove(id);
                        let action = model.action.clone();
                        self.config_remove(&action, &removed);
                    }
                }
            }

            ShortcutMessage::DeleteShortcut(id) => {
                let model = self.shortcut_models.remove(id);
                for (_, (_, binding, ..)) in model.bindings {
                    self.config_remove(&model.action, &binding);
                }
            }

            ShortcutMessage::EditBinding(id, enable) => {
                if let Some(short_id) = self.shortcut_context {
                    if let Some(model) = self.shortcut_models.get_mut(short_id) {
                        if let Some(&mut (_, _, _, ref mut is_editing)) = model.bindings.get_mut(id)
                        {
                            *is_editing = enable;
                        }
                    }
                }
            }

            ShortcutMessage::InputBinding(id, text) => {
                if let Some(short_id) = self.shortcut_context {
                    if let Some(model) = self.shortcut_models.get_mut(short_id) {
                        if let Some(&mut (_, _, ref mut editing, _)) = model.bindings.get_mut(id) {
                            *editing = text;
                        }
                    }
                }
            }

            ShortcutMessage::ShowShortcut(id, description) => {
                self.shortcut_context = Some(id);
                self.replace_dialog = None;
                return command::message(crate::app::Message::OpenContextDrawer(
                    description.into(),
                ));
            }

            ShortcutMessage::SubmitBinding(id) => {
                if let Some(short_id) = self.shortcut_context {
                    let mut apply_binding = None;

                    // Check for conflicts with the new binding.
                    if let Some(model) = self.shortcut_models.get_mut(short_id) {
                        if let Some((_, _, editing, _)) = model.bindings.get_mut(id) {
                            match Binding::from_str(editing) {
                                Ok(new_binding) => {
                                    if !new_binding.is_set() {
                                        editing.clear();
                                        return Command::none();
                                    }

                                    if let Some(action) = self.config_contains(&new_binding) {
                                        let action_str = super::localize_action(&action);
                                        self.replace_dialog =
                                            Some((id, new_binding, action, action_str));
                                        return Command::none();
                                    }

                                    apply_binding = Some(new_binding);
                                }

                                Err(why) => {
                                    tracing::error!(why, "keybinding input invalid");
                                }
                            }
                        }
                    }

                    // Apply if no conflict was found.
                    if let Some(new_binding) = apply_binding {
                        if let Some(model) = self.shortcut_models.get_mut(short_id) {
                            if let Some((_, binding, editing, is_editing)) =
                                model.bindings.get_mut(id)
                            {
                                let prev_binding = binding.clone();

                                *binding = new_binding.clone();
                                editing.clear();
                                *is_editing = false;

                                let action = model.action.clone();
                                self.config_remove(&action, &prev_binding);
                                self.config_add(action, new_binding);
                            }
                        }
                    }
                }
            }
        }

        Command::none()
    }

    /// Gets the custom configuration for keyboard shortcuts.
    pub(super) fn shortcuts_config(&self) -> Shortcuts {
        match self.config.get::<Shortcuts>("custom") {
            Ok(shortcuts) => shortcuts,
            Err(cosmic_config::Error::GetKey(_, why)) if why.kind() == io::ErrorKind::NotFound => {
                Shortcuts::default()
            }
            Err(why) => {
                tracing::error!(?why, "unable to get the current shortcuts config");
                Shortcuts::default()
            }
        }
    }

    /// Gets the system configuration for keyboard shortcuts.
    pub(super) fn shortcuts_system_config(&self) -> Shortcuts {
        let mut shortcuts = self.config.get::<Shortcuts>("defaults").unwrap_or_default();

        if let Ok(custom) = self.config.get::<Shortcuts>("custom") {
            shortcuts.0.extend(custom.0);
        }

        shortcuts
    }

    /// Writes a new configuration to the keyboard shortcuts config file.
    pub(super) fn shortcuts_config_set(&self, shortcuts: Shortcuts) {
        if let Err(why) = self.config.set("custom", shortcuts) {
            tracing::error!(?why, "failed to write shortcuts config");
        }
    }

    /// Adds a new binding to the shortcuts config
    pub(super) fn config_add(&self, action: Action, binding: Binding) {
        let mut shortcuts = self.shortcuts_config();
        shortcuts.0.insert(binding, action);
        self.shortcuts_config_set(shortcuts);
    }

    /// Check if a binding is already set
    pub(super) fn config_contains(&self, binding: &Binding) -> Option<Action> {
        self.shortcuts_system_config().0.get(binding).cloned()
    }

    /// Removes a binding from the shortcuts config
    pub(super) fn config_remove(&self, action: &Action, binding: &Binding) {
        let mut shortcuts = self.shortcuts_config();
        shortcuts.0.retain(|b, a| a != action && b != binding);
        self.shortcuts_config_set(shortcuts);
    }
}
