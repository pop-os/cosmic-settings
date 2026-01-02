// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod common;

pub use common::{Model, ShortcutBinding, ShortcutMessage, ShortcutModel};

pub mod accessibility;
pub mod custom;
pub mod manage_windows;
pub mod move_window;
pub mod nav;
pub mod system;
pub mod tiling;

use cosmic::app::ContextDrawer;
use cosmic::iced::Length;
use cosmic::widget::{self, icon, settings, text};
use cosmic::{Apply, Element, Task, theme};
use cosmic_config::ConfigGet;
use cosmic_settings_config::Binding;
use cosmic_settings_config::shortcuts::action::{
    Direction, FocusDirection, Orientation, ResizeDirection,
};
use cosmic_settings_config::shortcuts::{self, Action, Shortcuts};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use itertools::Itertools;
use shortcuts::action::System as SystemAction;
use slab::Slab;
use slotmap::{DefaultKey, Key, SecondaryMap, SlotMap};
use std::collections::BTreeSet;
use std::io;

pub struct Page {
    entity: page::Entity,
    modified: Modified,
    search: Search,
    search_model: Model,
    shortcuts_context: Option<cosmic_config::Config>,
    sub_pages: SubPages,
}

#[derive(Default)]
struct Modified {
    accessibility: u16,
    manage_windows: u16,
    move_windows: u16,
    nav: u16,
    system: u16,
    window_tiling: u16,
    custom: u16,
}

struct SubPages {
    accessibility: page::Entity,
    custom: page::Entity,
    manage_window: page::Entity,
    move_window: page::Entity,
    nav: page::Entity,
    system: page::Entity,
    window_tiling: page::Entity,
}

struct Search {
    input: String,
    actions: SlotMap<DefaultKey, Action>,
    localized: SecondaryMap<DefaultKey, String>,
    config: cosmic_config::Config,
    shortcuts: Shortcuts,
    defaults: Shortcuts,
}

#[derive(Clone, Debug)]
pub enum Message {
    Category(Category),
    Search(String),
    SearchShortcut(ShortcutMessage),
}

#[derive(Clone, Copy, Debug)]
pub enum Category {
    Accessibility,
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
            entity: page::Entity::default(),
            modified: Modified::default(),
            search: Search::default(),
            search_model: Model::default(),
            shortcuts_context: None,
            sub_pages: SubPages {
                accessibility: page::Entity::null(),
                custom: page::Entity::null(),
                manage_window: page::Entity::null(),
                move_window: page::Entity::null(),
                nav: page::Entity::null(),
                system: page::Entity::null(),
                window_tiling: page::Entity::null(),
            },
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
        self.search_model.entity = entity;
    }

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

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        if self.search_model.shortcut_models.is_empty() {
            None
        } else {
            self.search_model.context_drawer(|msg| {
                crate::pages::Message::KeyboardShortcuts(Message::SearchShortcut(msg))
            })
        }
    }

    fn dialog(&self) -> Option<Element<'_, crate::pages::Message>> {
        if self.search_model.shortcut_models.is_empty() {
            None
        } else {
            self.search_model.dialog().map(|el| {
                el.map(|msg| crate::pages::Message::KeyboardShortcuts(Message::SearchShortcut(msg)))
            })
        }
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        if self.shortcuts_context.is_none() {
            self.shortcuts_context = cosmic_settings_config::shortcuts::context().ok();
        }

        self.reload_search();

        Task::none()
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        self.clear();
        Task::none()
    }

    #[cfg(feature = "wayland")]
    fn subscription(
        &self,
        core: &cosmic::Core,
    ) -> cosmic::iced::Subscription<crate::pages::Message> {
        self.search_model
            .subscription(core)
            .map(|m| crate::pages::Message::KeyboardShortcuts(Message::SearchShortcut(m)))
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::Category(category) => match category {
                Category::Accessibility => {
                    cosmic::task::message(crate::app::Message::Page(self.sub_pages.accessibility))
                }

                Category::Custom => {
                    cosmic::task::message(crate::app::Message::Page(self.sub_pages.custom))
                }

                Category::ManageWindow => {
                    cosmic::task::message(crate::app::Message::Page(self.sub_pages.manage_window))
                }

                Category::MoveWindow => {
                    cosmic::task::message(crate::app::Message::Page(self.sub_pages.move_window))
                }

                Category::Nav => {
                    cosmic::task::message(crate::app::Message::Page(self.sub_pages.nav))
                }

                Category::System => {
                    cosmic::task::message(crate::app::Message::Page(self.sub_pages.system))
                }

                Category::WindowTiling => {
                    cosmic::task::message(crate::app::Message::Page(self.sub_pages.window_tiling))
                }
            },

            Message::Search(input) => {
                self.search(input);
                Task::none()
            }

            Message::SearchShortcut(message) => self.search_model.update(message),
        }
    }

    fn clear(&mut self) {
        self.search.actions = SlotMap::new();
        self.search.localized = SecondaryMap::new();
        self.search.input = String::new();
        self.search_model.on_clear();
        self.modified.custom = 0;
        self.modified.manage_windows = 0;
        self.modified.move_windows = 0;
        self.modified.nav = 0;
        self.modified.system = 0;
    }

    fn reload_search(&mut self) {
        self.clear();

        if let Some(context) = self.shortcuts_context.as_ref() {
            let mut defaults = context.get::<Shortcuts>("defaults").unwrap_or_default();
            let custom = context.get::<Shortcuts>("custom").unwrap_or_default();

            for (custom_binding, custom_action) in &custom.0 {
                // Skip bindings for the super key
                if custom_binding.is_super() {
                    continue;
                }

                // Check if a custom binding overrides a default binding, or is in addition to it.
                match defaults.0.get(custom_binding) {
                    Some(default_action) if default_action == custom_action => continue,
                    _ => (),
                }

                match action_category(custom_action) {
                    Some(Category::Accessibility) => self.modified.accessibility += 1,
                    Some(Category::ManageWindow) => self.modified.manage_windows += 1,
                    Some(Category::MoveWindow) => self.modified.move_windows += 1,
                    Some(Category::Nav) => self.modified.nav += 1,
                    Some(Category::System) => self.modified.system += 1,
                    Some(Category::WindowTiling) => self.modified.window_tiling += 1,
                    None | Some(Category::Custom) => (),
                }
            }

            // Check if default bindings are missing
            for (binding, action) in &defaults.0 {
                if binding.is_super() {
                    continue;
                }

                match custom.0.get(binding) {
                    Some(custom_action) if action != custom_action => (),
                    _ => continue,
                };

                match action_category(action) {
                    Some(Category::Accessibility) => self.modified.accessibility += 1,
                    Some(Category::ManageWindow) => self.modified.manage_windows += 1,
                    Some(Category::MoveWindow) => self.modified.move_windows += 1,
                    Some(Category::Nav) => self.modified.nav += 1,
                    Some(Category::System) => self.modified.system += 1,
                    Some(Category::WindowTiling) => self.modified.window_tiling += 1,
                    None | Some(Category::Custom) => (),
                }
            }

            self.search.defaults = defaults.clone();
            defaults.0.extend(custom.0);
            self.search.shortcuts = defaults;
        }
    }

    fn search(&mut self, input: String) {
        self.reload_search();

        if input.is_empty() {
            self.search_model.on_clear();
            return;
        }

        if self.search.actions.is_empty() {
            self.search.cache_localized_actions();
        }

        self.search.input = input;
        self.search_model.shortcut_models = self.search.shortcut_models();
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: cosmic_settings_page::Insert<crate::pages::Message>,
    ) -> cosmic_settings_page::Insert<crate::pages::Message> {
        let accessibility = page.sub_page_with_id::<accessibility::Page>();
        let custom = page.sub_page_with_id::<custom::Page>();
        let manage_window = page.sub_page_with_id::<manage_windows::Page>();
        let move_window = page.sub_page_with_id::<move_window::Page>();
        let nav = page.sub_page_with_id::<nav::Page>();
        let system = page.sub_page_with_id::<system::Page>();
        let window_tiling = page.sub_page_with_id::<tiling::Page>();

        let model = page.model.page_mut::<Page>().unwrap();
        model.sub_pages.accessibility = accessibility;
        model.sub_pages.custom = custom;
        model.sub_pages.manage_window = manage_window;
        model.sub_pages.move_window = move_window;
        model.sub_pages.nav = nav;
        model.sub_pages.system = system;
        model.sub_pages.window_tiling = window_tiling;

        page
    }
}

impl Default for Search {
    fn default() -> Self {
        Self {
            input: String::default(),
            defaults: Shortcuts::default(),
            config: shortcuts::context().unwrap(),
            localized: SecondaryMap::default(),
            actions: SlotMap::new(),
            shortcuts: Shortcuts::default(),
        }
    }
}

impl Search {
    fn cache_localized_actions(&mut self) {
        self.actions.clear();
        self.localized.clear();
        let custom_actions = self.retrieve_custom_actions();
        for action in all_system_actions() {
            let localized = localize_action(action);
            let id = self.actions.insert(action.clone());
            self.localized.insert(id, localized);
        }
        for (binding, action) in custom_actions {
            let localized = localize_custom_action(&action, &binding);
            let id = self.actions.insert(action.clone());
            self.localized.insert(id, localized);
        }
        // Remove unicode isolation characters to fix searching localized text that has them.
        for (_, localized) in self.localized.iter_mut() {
            *localized = localized.replace("\u{2068}", "").replace("\u{2069}", "");
        }
    }

    fn retrieve_custom_actions(&self) -> Vec<(Binding, Action)> {
        let custom_shortcusts = match self.config.get::<Shortcuts>("custom") {
            Ok(shortcuts) => shortcuts,
            Err(cosmic_config::Error::GetKey(_, why)) if why.kind() == io::ErrorKind::NotFound => {
                Shortcuts::default()
            }
            Err(why) => {
                tracing::error!(?why, "unable to get the current shortcuts config");
                Shortcuts::default()
            }
        };
        custom_shortcusts
            .0
            .into_iter()
            .unique_by(|(_, action)| localize_action(action))
            .collect::<Vec<(Binding, Action)>>()
    }

    fn shortcut_models(&mut self) -> Slab<ShortcutModel> {
        let shortcut_search_actions = match Binding::from_str_partial(&self.input) {
            Ok(input_binding) => self
                .shortcuts
                .iter()
                .filter_map(|(binding, action)| {
                    if input_binding.is_subset(binding) {
                        Some(action)
                    } else {
                        None
                    }
                })
                .collect::<BTreeSet<_>>(),
            Err(_) => Default::default(),
        };

        let input = self.input.to_lowercase();

        self.actions
            .iter()
            .filter(|(id, action)| {
                self.localized[*id].to_lowercase().contains(&input)
                    || shortcut_search_actions.contains(action)
            })
            .fold(Slab::new(), |mut slab, (_, action)| {
                slab.insert(ShortcutModel::new(
                    &self.defaults,
                    &self.shortcuts,
                    action.clone(),
                ));

                slab
            })
    }
}

fn shortcuts() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let accessibility = descriptions.insert(fl!("accessibility"));
    let custom_label = descriptions.insert(fl!("custom"));
    let manage_window_label = descriptions.insert(fl!("manage-windows"));
    let move_window_label = descriptions.insert(fl!("move-windows"));
    let nav_label = descriptions.insert(fl!("nav-shortcuts"));
    let system_label = descriptions.insert(fl!("system-shortcut"));
    let window_tiling_label = descriptions.insert(fl!("window-tiling"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            let search = widget::search_input(fl!("type-to-search"), &page.search.input)
                .width(314)
                .on_clear(Message::Search(String::new()))
                .on_input(Message::Search)
                .apply(widget::container)
                .padding([2, 0, 0, 0])
                .center_x(Length::Fill);

            // If the search input is not empty, show the category view, else the search results.
            let content = if page.search.input.is_empty() {
                settings::section()
                    .add(category_item(
                        Category::Accessibility,
                        &descriptions[accessibility],
                        page.modified.accessibility,
                    ))
                    .add(category_item(
                        Category::ManageWindow,
                        &descriptions[manage_window_label],
                        page.modified.manage_windows,
                    ))
                    .add(category_item(
                        Category::MoveWindow,
                        &descriptions[move_window_label],
                        page.modified.move_windows,
                    ))
                    .add(category_item(
                        Category::Nav,
                        &descriptions[nav_label],
                        page.modified.nav,
                    ))
                    .add(category_item(
                        Category::System,
                        &descriptions[system_label],
                        page.modified.system,
                    ))
                    .add(category_item(
                        Category::WindowTiling,
                        &descriptions[window_tiling_label],
                        page.modified.window_tiling,
                    ))
                    .add(category_item(
                        Category::Custom,
                        &descriptions[custom_label],
                        page.modified.custom,
                    ))
                    .apply(Element::from)
            } else {
                page.search_model.view().map(Message::SearchShortcut)
            };

            widget::column::with_capacity(2)
                .spacing(32)
                .push(search)
                .push(content)
                .apply(Element::from)
                .map(crate::pages::Message::KeyboardShortcuts)
        })
}

/// Display a category as a list item
fn category_item(category: Category, name: &str, modified: u16) -> Element<'_, Message> {
    let icon = icon::from_name("go-next-symbolic").size(16);

    let control = if modified == 0 {
        Element::from(icon)
    } else {
        widget::row()
            .push(text::body(fl!("modified", count = modified)))
            .push(icon)
            .into()
    };

    settings::item::builder(name)
        .control(control)
        .spacing(16)
        .apply(widget::container)
        .class(theme::Container::List)
        .apply(widget::button::custom)
        .class(theme::Button::Transparent)
        .on_press(Message::Category(category))
        .into()
}

fn action_category(action: &Action) -> Option<Category> {
    Some(if manage_windows::actions().contains(action) {
        Category::ManageWindow
    } else if move_window::actions().contains(action) {
        Category::MoveWindow
    } else if nav::actions().contains(action) {
        Category::Nav
    } else if system::actions().contains(action) {
        Category::System
    } else {
        return None;
    })
}

fn all_system_actions() -> &'static [Action] {
    &[
        Action::Close,
        Action::Debug,
        Action::Focus(FocusDirection::Down),
        Action::Focus(FocusDirection::In),
        Action::Focus(FocusDirection::Left),
        Action::Focus(FocusDirection::Out),
        Action::Focus(FocusDirection::Right),
        Action::Focus(FocusDirection::Up),
        Action::LastWorkspace,
        Action::Maximize,
        Action::Fullscreen,
        Action::MigrateWorkspaceToOutput(Direction::Down),
        Action::MigrateWorkspaceToOutput(Direction::Left),
        Action::MigrateWorkspaceToOutput(Direction::Right),
        Action::MigrateWorkspaceToOutput(Direction::Up),
        Action::Minimize,
        Action::Move(Direction::Down),
        Action::Move(Direction::Left),
        Action::Move(Direction::Right),
        Action::Move(Direction::Up),
        Action::MoveToLastWorkspace,
        Action::MoveToNextWorkspace,
        Action::MoveToOutput(Direction::Down),
        Action::MoveToOutput(Direction::Left),
        Action::MoveToOutput(Direction::Right),
        Action::MoveToOutput(Direction::Up),
        Action::MoveToPreviousWorkspace,
        Action::MoveToWorkspace(1),
        Action::MoveToWorkspace(2),
        Action::MoveToWorkspace(3),
        Action::MoveToWorkspace(4),
        Action::MoveToWorkspace(5),
        Action::MoveToWorkspace(6),
        Action::MoveToWorkspace(7),
        Action::MoveToWorkspace(8),
        Action::MoveToWorkspace(9),
        Action::NextWorkspace,
        Action::Orientation(Orientation::Horizontal),
        Action::Orientation(Orientation::Vertical),
        Action::PreviousWorkspace,
        Action::Resizing(ResizeDirection::Inwards),
        Action::Resizing(ResizeDirection::Outwards),
        Action::SwapWindow,
        Action::SwitchOutput(Direction::Down),
        Action::SwitchOutput(Direction::Left),
        Action::SwitchOutput(Direction::Right),
        Action::SwitchOutput(Direction::Up),
        Action::System(SystemAction::AppLibrary),
        Action::System(SystemAction::BrightnessDown),
        Action::System(SystemAction::BrightnessUp),
        Action::System(SystemAction::DisplayToggle),
        Action::System(SystemAction::HomeFolder),
        Action::System(SystemAction::KeyboardBrightnessDown),
        Action::System(SystemAction::KeyboardBrightnessUp),
        Action::System(SystemAction::Launcher),
        Action::System(SystemAction::LogOut),
        Action::System(SystemAction::LockScreen),
        Action::System(SystemAction::Mute),
        Action::System(SystemAction::MuteMic),
        Action::System(SystemAction::PlayPause),
        Action::System(SystemAction::PlayNext),
        Action::System(SystemAction::PlayPrev),
        Action::System(SystemAction::ScreenReader),
        Action::System(SystemAction::Screenshot),
        Action::System(SystemAction::Suspend),
        Action::System(SystemAction::Terminal),
        Action::System(SystemAction::TouchpadToggle),
        Action::System(SystemAction::VolumeLower),
        Action::System(SystemAction::VolumeRaise),
        Action::System(SystemAction::WebBrowser),
        Action::System(SystemAction::WindowSwitcher),
        Action::System(SystemAction::WindowSwitcherPrevious),
        Action::System(SystemAction::WorkspaceOverview),
        Action::System(SystemAction::InputSourceSwitch),
        Action::Terminate,
        Action::ToggleOrientation,
        Action::ToggleStacking,
        Action::ToggleSticky,
        Action::ToggleTiling,
        Action::ToggleWindowFloating,
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

fn localize_action(action: &Action) -> String {
    #[allow(deprecated)]
    match action {
        Action::Close => fl!("manage-windows", "close"),
        Action::Disable => fl!("disabled"),
        Action::Focus(FocusDirection::Down) => fl!("nav-shortcuts", "focus", direction = "down"),
        Action::Focus(FocusDirection::In) => fl!("nav-shortcuts", "focus", direction = "in"),
        Action::Focus(FocusDirection::Left) => fl!("nav-shortcuts", "focus", direction = "left"),
        Action::Focus(FocusDirection::Out) => fl!("nav-shortcuts", "focus", direction = "out"),
        Action::Focus(FocusDirection::Right) => fl!("nav-shortcuts", "focus", direction = "right"),
        Action::Focus(FocusDirection::Up) => fl!("nav-shortcuts", "focus", direction = "up"),
        Action::Workspace(i) => fl!("nav-shortcuts", "workspace", num = (*i as usize)),
        Action::LastWorkspace => fl!("nav-shortcuts", "last-workspace"),
        Action::Maximize => fl!("manage-windows", "maximize"),
        Action::Fullscreen => fl!("manage-windows", "fullscreen"),
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
            SystemAction::InputSourceSwitch => fl!("input-source-switch"),
            SystemAction::HomeFolder => fl!("system-shortcut", "home-folder"),
            SystemAction::KeyboardBrightnessDown => {
                fl!("system-shortcut", "keyboard-brightness-down")
            }
            SystemAction::KeyboardBrightnessUp => fl!("system-shortcut", "keyboard-brightness-up"),
            SystemAction::Launcher => fl!("system-shortcut", "launcher"),
            SystemAction::LogOut => fl!("system-shortcut", "log-out"),
            SystemAction::LockScreen => fl!("system-shortcut", "lock-screen"),
            SystemAction::Mute => fl!("system-shortcut", "mute"),
            SystemAction::MuteMic => fl!("system-shortcut", "mute-mic"),
            SystemAction::PlayPause => fl!("system-shortcut", "play-pause"),
            SystemAction::PlayNext => fl!("system-shortcut", "play-next"),
            SystemAction::PlayPrev => fl!("system-shortcut", "play-prev"),
            SystemAction::PowerOff => fl!("system-shortcut", "poweroff"),
            SystemAction::Screenshot => fl!("system-shortcut", "screenshot"),
            SystemAction::Suspend => fl!("system-shortcut", "suspend"),
            SystemAction::ScreenReader => fl!("accessibility", "screen-reader"),
            SystemAction::Terminal => fl!("system-shortcut", "terminal"),
            SystemAction::TouchpadToggle => fl!("system-shortcut", "touchpad-toggle"),
            SystemAction::VolumeLower => fl!("system-shortcut", "volume-lower"),
            SystemAction::VolumeRaise => fl!("system-shortcut", "volume-raise"),
            SystemAction::WebBrowser => fl!("system-shortcut", "web-browser"),
            SystemAction::WindowSwitcher => fl!("system-shortcut", "window-switcher"),
            SystemAction::WindowSwitcherPrevious => {
                fl!("system-shortcut", "window-switcher-previous")
            }
            SystemAction::WorkspaceOverview => fl!("system-shortcut", "workspace-overview"),
            SystemAction::DisplayToggle => fl!("system-shortcut", "display-toggle"),
        },

        Action::ZoomIn => fl!("zoom-in"),

        Action::ZoomOut => fl!("zoom-out"),

        Action::Spawn(task) => task.clone(),
    }
}

fn localize_custom_action(action: &Action, binding: &Binding) -> String {
    if let Some(description) = &binding.description {
        description.to_string()
    } else {
        localize_action(action)
    }
}
