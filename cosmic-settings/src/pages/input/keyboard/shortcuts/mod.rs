pub mod custom;
pub mod manage_windows;
pub mod move_window;
pub mod nav;
pub mod system;
pub mod tiling;

use cosmic::iced::alignment::Horizontal;
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::CollectionWidget;
use cosmic::widget::{self, button, icon, settings, text, Column};
use cosmic::{command, theme, Apply, Command, Element};
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_config::shortcuts::action::{
    Direction, FocusDirection, Orientation, ResizeDirection,
};
use cosmic_settings_config::shortcuts::{self, Action, Binding, Shortcuts};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use shortcuts::action::System as SystemAction;
use slab::Slab;
use slotmap::{Key, SlotMap};
use std::borrow::Cow;
use std::io;
use std::str::FromStr;

pub struct Page {
    custom_page: page::Entity,
    manage_window_page: page::Entity,
    move_window_page: page::Entity,
    nav_page: page::Entity,
    system_page: page::Entity,
    window_tiling_page: page::Entity,
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Category(Category),
}

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

#[derive(Clone, Copy, Debug)]
pub enum Category {
    Custom,
    ManageWindow,
    MoveWindow,
    Nav,
    System,
    WindowTiling,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            custom_page: page::Entity::null(),
            manage_window_page: page::Entity::null(),
            move_window_page: page::Entity::null(),
            nav_page: page::Entity::null(),
            system_page: page::Entity::null(),
            window_tiling_page: page::Entity::null(),
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(shortcuts())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("keyboard-shortcuts", "input-keyboard-symbolic")
            .title(fl!("keyboard-shortcuts"))
            .description(fl!("keyboard-shortcuts", "desc"))
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        match message {
            Message::Category(category) => match category {
                Category::Custom => command::message(crate::app::Message::Page(self.custom_page)),

                Category::ManageWindow => {
                    command::message(crate::app::Message::Page(self.manage_window_page))
                }

                Category::MoveWindow => {
                    command::message(crate::app::Message::Page(self.move_window_page))
                }

                Category::Nav => command::message(crate::app::Message::Page(self.nav_page)),

                Category::System => command::message(crate::app::Message::Page(self.system_page)),

                Category::WindowTiling => {
                    command::message(crate::app::Message::Page(self.window_tiling_page))
                }
            },
        }
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: cosmic_settings_page::Insert<crate::pages::Message>,
    ) -> cosmic_settings_page::Insert<crate::pages::Message> {
        let custom_page = page.sub_page_with_id::<custom::Page>();
        let manage_window_page = page.sub_page_with_id::<manage_windows::Page>();
        let move_window_page = page.sub_page_with_id::<move_window::Page>();
        let nav_page = page.sub_page_with_id::<nav::Page>();
        let system_page = page.sub_page_with_id::<system::Page>();
        let window_tiling_page = page.sub_page_with_id::<tiling::Page>();

        let model = page.model.page_mut::<Page>().unwrap();
        model.custom_page = custom_page;
        model.manage_window_page = manage_window_page;
        model.move_window_page = move_window_page;
        model.nav_page = nav_page;
        model.system_page = system_page;
        model.window_tiling_page = window_tiling_page;

        page
    }
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
            description: localize_action(&action),
            action,
        }
    }
}

#[must_use]
pub struct Model {
    replace_dialog: Option<(usize, Binding, Action, String)>,
    shortcut_models: Slab<ShortcutModel>,
    shortcut_context: Option<usize>,
    config: cosmic_config::Config,
    custom: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self {
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

    fn context_drawer(&self) -> Option<Element<'_, ShortcutMessage>> {
        self.shortcut_context
            .as_ref()
            .map(|id| context_drawer(&self.shortcut_models, *id, self.custom))
    }

    fn dialog(&self) -> Option<Element<'_, ShortcutMessage>> {
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

    fn on_enter(&mut self, bindings: fn(&Shortcuts) -> Slab<ShortcutModel>) {
        let mut shortcuts = self.config.get::<Shortcuts>("defaults").unwrap_or_default();

        if let Ok(custom) = self.config.get::<Shortcuts>("custom") {
            shortcuts.0.extend(custom.0);
        }

        self.shortcut_models = bindings(&shortcuts);
    }

    fn on_clear(&mut self) {
        self.shortcut_models.clear();
    }

    fn view(&self) -> Element<ShortcutMessage> {
        self.shortcut_models
            .iter()
            .map(|(id, shortcut)| shortcut_item(self.custom, id, shortcut))
            .fold(settings::view_section(""), settings::Section::add)
            .into()
    }

    #[allow(clippy::too_many_lines)]
    fn update(&mut self, message: ShortcutMessage) -> Command<crate::app::Message> {
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
                    eprintln!("removing {binding:?} for {:?}", model.action);
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
                    if let Some(model) = self.shortcut_models.get(short_id) {
                        if let Some((_, _, editing, _)) = model.bindings.get(id) {
                            match Binding::from_str(editing) {
                                Ok(new_binding) => {
                                    if let Some(action) = self.config_contains(&new_binding) {
                                        let action_str = localize_action(&action);
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
    fn shortcuts_config(&self) -> Shortcuts {
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
    fn shortcuts_system_config(&self) -> Shortcuts {
        let mut shortcuts = self.config.get::<Shortcuts>("defaults").unwrap_or_default();

        if let Ok(custom) = self.config.get::<Shortcuts>("custom") {
            shortcuts.0.extend(custom.0);
        }

        shortcuts
    }

    /// Writes a new configuration to the keyboard shortcuts config file.
    fn shortcuts_config_set(&self, shortcuts: Shortcuts) {
        if let Err(why) = self.config.set("custom", shortcuts) {
            tracing::error!(?why, "failed to write shortcuts config");
        }
    }

    /// Adds a new binding to the shortcuts config
    fn config_add(&self, action: Action, binding: Binding) {
        let mut shortcuts = self.shortcuts_config();
        shortcuts.0.insert(binding, action);
        self.shortcuts_config_set(shortcuts);
    }

    /// Check if a binding is already set
    fn config_contains(&self, binding: &Binding) -> Option<Action> {
        self.shortcuts_system_config().0.get(binding).cloned()
    }

    /// Removes a binding from the shortcuts config
    fn config_remove(&self, action: &Action, binding: &Binding) {
        let mut shortcuts = self.shortcuts_config();
        shortcuts.0.retain(|b, a| a != action && b != binding);
        self.shortcuts_config_set(shortcuts);
    }
}

fn shortcuts() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let custom_label = descriptions.insert(fl!("custom"));
    let manage_window_label = descriptions.insert(fl!("manage-windows"));
    let move_window_label = descriptions.insert(fl!("move-windows"));
    let nav_label = descriptions.insert(fl!("nav-shortcuts"));
    let system_label = descriptions.insert(fl!("system-shortcut"));
    let window_tiling_label = descriptions.insert(fl!("window-tiling"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            let descriptions = &section.descriptions;

            settings::view_section("")
                .add(category_item(
                    Category::ManageWindow,
                    &descriptions[manage_window_label],
                ))
                .add(category_item(
                    Category::MoveWindow,
                    &descriptions[move_window_label],
                ))
                .add(category_item(Category::Nav, &descriptions[nav_label]))
                .add(category_item(Category::System, &descriptions[system_label]))
                .add(category_item(
                    Category::WindowTiling,
                    &descriptions[window_tiling_label],
                ))
                .add(category_item(Category::Custom, &descriptions[custom_label]))
                .apply(Element::from)
                .map(crate::pages::Message::KeyboardShortcuts)
        })
}

/// Display a category as a list item
fn category_item(category: Category, name: &str) -> Element<Message> {
    settings::item::builder(name)
        .control(icon::from_name("go-next-symbolic").size(16))
        .spacing(16)
        .apply(widget::container)
        .style(theme::Container::List)
        .apply(widget::button)
        .style(theme::Button::Transparent)
        .on_press(Message::Category(category))
        .into()
}

/// Display a shortcut as a list item
fn shortcut_item(custom: bool, id: usize, data: &ShortcutModel) -> Element<ShortcutMessage> {
    #[derive(Copy, Clone, Debug)]
    enum LocalMessage {
        Remove,
        Show,
    }

    let shortcuts: Element<LocalMessage> = if data.bindings.is_empty() {
        widget::text::body(fl!("disabled")).into()
    } else {
        data.bindings
            .iter()
            .take(3)
            .filter(|(_, (_, b, ..))| b.is_set())
            .map(|(_, (_, b, ..))| widget::text::body(b.to_string()))
            .fold(widget::column(), Column::push)
            .align_items(Alignment::End)
            .into()
    };

    let control = widget::row::with_capacity(if custom { 3 } else { 2 })
        .push(shortcuts)
        .push(icon::from_name("go-next-symbolic").size(16))
        .push_maybe(custom.then(|| {
            widget::button::icon(icon::from_name("edit-delete-symbolic"))
                .on_press(LocalMessage::Remove)
        }))
        .align_items(Alignment::Center)
        .spacing(4);

    settings::item::builder(&data.description)
        .flex_control(control)
        .spacing(16)
        .apply(widget::container)
        .style(theme::Container::List)
        .apply(widget::button)
        .style(theme::Button::Transparent)
        .on_press(LocalMessage::Show)
        .apply(Element::from)
        .map(move |message| match message {
            LocalMessage::Show => ShortcutMessage::ShowShortcut(id, data.description.clone()),
            LocalMessage::Remove => ShortcutMessage::DeleteShortcut(id),
        })
}

fn context_drawer(
    shortcuts: &Slab<ShortcutModel>,
    id: usize,
    show_action: bool,
) -> Element<ShortcutMessage> {
    let model = &shortcuts[id];

    let action = show_action.then(|| {
        let description = if let Action::Spawn(command) = &model.action {
            Cow::Borrowed(command.as_str())
        } else {
            Cow::Owned(localize_action(&model.action))
        };

        text::body(description)
    });

    let bindings = model.bindings.iter().enumerate().fold(
        settings::view_section(""),
        |section, (_, (bind_id, (id, binding, edit, is_editing)))| {
            let text: Cow<'_, str> = if *is_editing {
                Cow::Borrowed(edit)
            } else if binding.is_set() {
                Cow::Owned(binding.to_string())
            } else {
                Cow::Borrowed("")
            };

            let input = widget::editable_input("", text, *is_editing, move |enable| {
                ShortcutMessage::EditBinding(bind_id, enable)
            })
            .on_clear(ShortcutMessage::ClearBinding(bind_id))
            .on_input(move |text| ShortcutMessage::InputBinding(bind_id, text))
            .on_submit(ShortcutMessage::SubmitBinding(bind_id))
            .id(id.clone())
            .into();

            let delete_button = widget::button::icon(icon::from_name("edit-delete-symbolic"))
                .on_press(ShortcutMessage::DeleteBinding(bind_id))
                .into();

            section.add(settings::flex_item_row(vec![input, delete_button]))
        },
    );

    let add_keybinding_button = widget::button::standard(fl!("add-keybinding"))
        .on_press(ShortcutMessage::AddKeybinding)
        .apply(widget::container)
        .width(Length::Fill)
        .align_x(Horizontal::Right);

    widget::column::with_capacity(if show_action { 3 } else { 2 })
        .spacing(24)
        .push_maybe(action)
        .push(bindings)
        .push(add_keybinding_button)
        .into()
}

fn localize_action(action: &Action) -> String {
    match action {
        Action::Close => fl!("manage-windows", "close"),
        Action::Focus(FocusDirection::Down) => fl!("nav-shortcuts", "focus", direction = "down"),
        Action::Focus(FocusDirection::In) => fl!("nav-shortcuts", "focus", direction = "in"),
        Action::Focus(FocusDirection::Left) => fl!("nav-shortcuts", "focus", direction = "left"),
        Action::Focus(FocusDirection::Out) => fl!("nav-shortcuts", "focus", direction = "out"),
        Action::Focus(FocusDirection::Right) => fl!("nav-shortcuts", "focus", direction = "right"),
        Action::Focus(FocusDirection::Up) => fl!("nav-shortcuts", "focus", direction = "up"),
        Action::Workspace(i) => fl!("nav-shortcuts", "workspace", num = (*i as usize)),
        Action::LastWorkspace => fl!("nav-shortcuts", "last-workspace"),
        Action::Maximize => fl!("manage-windows", "maximize"),
        Action::Minimize => fl!("manage-windows", "minimize"),
        Action::Move(Direction::Down) => fl!("move-windows", "direction", direction = "down"),
        Action::Move(Direction::Right) => fl!("move-windows", "direction", direction = "right"),
        Action::Move(Direction::Left) => fl!("move-windows", "direction", direction = "left"),
        Action::Move(Direction::Up) => fl!("move-windows", "direction", direction = "up"),
        Action::MoveToLastWorkspace | Action::SendToLastWorkspace => {
            fl!("move-windows", "last-workspace")
        }
        Action::MoveToNextOutput | Action::SendToNextOutput => fl!("move-windows", "next-display"),
        Action::MoveToNextWorkspace | Action::SendToNextWorkspace => {
            fl!("move-windows", "next-workspace")
        }
        Action::MoveToPreviousWorkspace | Action::SendToPreviousWorkspace => {
            fl!("move-windows", "prev-workspace")
        }
        Action::MoveToOutput(Direction::Down) | Action::SendToOutput(Direction::Down) => {
            fl!("move-windows", "display", direction = "down")
        }
        Action::MoveToOutput(Direction::Left) | Action::SendToOutput(Direction::Left) => {
            fl!("move-windows", "display", direction = "left")
        }
        Action::MoveToOutput(Direction::Right) | Action::SendToOutput(Direction::Right) => {
            fl!("move-windows", "display", direction = "right")
        }
        Action::MoveToOutput(Direction::Up) | Action::SendToOutput(Direction::Up) => {
            fl!("move-windows", "display", direction = "up")
        }
        Action::MoveToPreviousOutput | Action::SendToPreviousOutput => {
            fl!("move-windows", "prev-display")
        }
        Action::MoveToWorkspace(i) | Action::SendToWorkspace(i) => {
            fl!("move-windows", "workspace-num", num = (*i as usize))
        }
        Action::NextOutput => fl!("nav-shortcuts", "next-output"),
        Action::NextWorkspace => fl!("nav-shortcuts", "next-workspace"),
        Action::Orientation(Orientation::Horizontal) => fl!("window-tiling", "horizontal"),
        Action::Orientation(Orientation::Vertical) => fl!("window-tiling", "vertical"),
        Action::PreviousOutput => fl!("nav-shortcuts", "prev-output"),
        Action::PreviousWorkspace => fl!("nav-shortcuts", "prev-workspace"),
        Action::Resizing(ResizeDirection::Inwards) => fl!("manage-windows", "resize-inwards"),
        Action::Resizing(ResizeDirection::Outwards) => fl!("manage-windows", "resize-outwards"),
        Action::SwapWindow => fl!("window-tiling", "swap-window"),
        Action::SwitchOutput(Direction::Down) => fl!("nav-shortcuts", "output", direction = "down"),
        Action::SwitchOutput(Direction::Left) => fl!("nav-shortcuts", "output", direction = "left"),
        Action::SwitchOutput(Direction::Right) => {
            fl!("nav-shortcuts", "output", direction = "right")
        }
        Action::SwitchOutput(Direction::Up) => fl!("nav-shortcuts", "output", direction = "up"),
        Action::ToggleOrientation => fl!("window-tiling", "toggle-orientation"),
        Action::ToggleStacking => fl!("window-tiling", "toggle-stacking"),
        Action::ToggleSticky => fl!("manage-windows", "toggle-sticky"),
        Action::ToggleTiling => fl!("window-tiling", "toggle-tiling"),
        Action::ToggleWindowFloating => fl!("window-tiling", "toggle-floating"),

        // Currently unused by any settings pages
        Action::Debug => fl!("debug"),

        Action::MigrateWorkspaceToNextOutput => fl!("migrate-workspace-next"),
        Action::MigrateWorkspaceToOutput(Direction::Down) => {
            fl!("migrate-workspace", direction = "down")
        }
        Action::MigrateWorkspaceToOutput(Direction::Left) => {
            fl!("migrate-workspace", direction = "left")
        }
        Action::MigrateWorkspaceToOutput(Direction::Right) => {
            fl!("migrate-workspace", direction = "right")
        }
        Action::MigrateWorkspaceToOutput(Direction::Up) => {
            fl!("migrate-workspace", direction = "up")
        }
        Action::MigrateWorkspaceToPreviousOutput => fl!("migrate-workspace-prev"),

        Action::Terminate => fl!("terminate"),

        Action::System(system) => match system {
            SystemAction::AppLibrary => fl!("system-shortcut", "app-library"),
            SystemAction::BrightnessDown => fl!("system-shortcut", "brightness-down"),
            SystemAction::BrightnessUp => fl!("system-shortcut", "brightness-up"),
            SystemAction::HomeFolder => fl!("system-shortcut", "home-folder"),
            SystemAction::KeyboardBrightnessDown => {
                fl!("system-shortcut", "keyboard-brightness-down")
            }
            SystemAction::KeyboardBrightnessUp => fl!("system-shortcut", "keyboard-brightness-up"),
            SystemAction::Launcher => fl!("system-shortcut", "launcher"),
            SystemAction::LockScreen => fl!("system-shortcut", "lock-screen"),
            SystemAction::Mute => fl!("system-shortcut", "mute"),
            SystemAction::MuteMic => fl!("system-shortcut", "mute-mic"),
            SystemAction::Screenshot => fl!("system-shortcut", "screenshot"),
            SystemAction::Terminal => fl!("system-shortcut", "terminal"),
            SystemAction::VolumeLower => fl!("system-shortcut", "volume-lower"),
            SystemAction::VolumeRaise => fl!("system-shortcut", "volume-raise"),
            SystemAction::WebBrowser => fl!("system-shortcut", "web-browser"),
            SystemAction::WindowSwitcher => fl!("system-shortcut", "window-switcher"),
            SystemAction::WorkspaceOverview => fl!("system-shortcut", "workspace-overview"),
        },

        Action::Spawn(command) => command.clone(),
    }
}
