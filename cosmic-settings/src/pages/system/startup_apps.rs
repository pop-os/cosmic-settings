use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, icon, settings, text, vertical_space};
use cosmic::{widget, Apply, Element, Task};
use cosmic_settings_page::section::Entity;
use cosmic_settings_page::{self as page, Content, Info, Section};
use freedesktop_desktop_entry::DesktopEntry;
use itertools::Itertools;
use slotmap::{Key, SlotMap};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tracing::error;

#[derive(Clone, Debug)]
pub struct CachedApps {
    apps: HashMap<DirectoryType, Vec<DesktopEntry>>,
    all_apps: Vec<DesktopEntry>,
    locales: Vec<String>,
}
#[derive(Clone, Debug)]
pub struct Page {
    entity: page::Entity,
    on_enter_handle: Option<cosmic::iced::task::Handle>,
    cached_startup_apps: Option<CachedApps>,
    application_search: String,
    context: Option<Context>,
    app_to_remove: Option<DesktopEntry>,
    target_directory_type: Option<DirectoryType>,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            entity: page::Entity::null(),
            on_enter_handle: None,
            cached_startup_apps: None,
            application_search: String::new(),
            context: None,
            app_to_remove: None,
            target_directory_type: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    ShowApplicationSidebar(DirectoryType),
    UpdateStartupApplications(CachedApps),
    UpdateApplications(CachedApps),

    ApplicationSearch(String),
    AddStartupApplication(DirectoryType, DesktopEntry),
    RemoveStartupApplication(DirectoryType, DesktopEntry, bool),
    CancelRemoveStartupApplication,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum DirectoryType {
    User,
}

#[derive(Clone, Debug)]
enum Context {
    AddApplication(DirectoryType),
}

impl Into<Vec<PathBuf>> for DirectoryType {
    fn into(self) -> Vec<PathBuf> {
        match self {
            DirectoryType::User => vec![dirs::config_dir()
                .expect("config dir not found")
                .join("autostart")],
        }
    }
}

impl std::fmt::Display for DirectoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DirectoryType::User => write!(f, "{}", fl!("startup-apps", "user")),
        }
    }
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::StartupApps(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::StartupApps(message)
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> Info {
        page::Info::new("startup-apps", "preferences-default-applications-symbolic")
            .title(fl!("startup-apps"))
            .description(fl!("startup-apps", "desc"))
    }
    fn content(
        &self,
        sections: &mut SlotMap<Entity, Section<crate::pages::Message>>,
    ) -> Option<Content> {
        Some(vec![sections.insert(apps())])
    }

    fn context_drawer(&self) -> Option<Element<'_, crate::pages::Message>> {
        match &self.context {
            Some(Context::AddApplication(directory_type)) => {
                Some(self.add_application_context_view(directory_type.clone()))
            }
            None => None,
        }
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        let (task, on_enter_handle) = Task::future(async move {
            let locales = freedesktop_desktop_entry::get_languages_from_env();

            let user_dirs: Vec<PathBuf> = DirectoryType::User.into();
            let user_entries =
                freedesktop_desktop_entry::Iter::new(user_dirs.into_iter()).entries(Some(&locales));

            let mut apps_hash = HashMap::with_capacity(1);
            apps_hash.insert(DirectoryType::User, user_entries.collect_vec());

            Message::UpdateStartupApplications(CachedApps {
                apps: apps_hash,
                all_apps: get_all_apps(locales.clone()),
                locales,
            })
            .into()
        })
        .abortable();

        self.on_enter_handle = Some(on_enter_handle);

        task
    }

    fn dialog(&self) -> Option<Element<'_, crate::pages::Message>> {
        if let Some(app_to_remove) = &self.app_to_remove {
            if let Some(cached_startup_apps) = &self.cached_startup_apps {
                if let Some(target_directory_type) = &self.target_directory_type {
                    return Some(
                        widget::dialog()
                            .title(fl!(
                                "startup-apps",
                                "remove-dialog-title",
                                name = app_to_remove.name(&cached_startup_apps.locales)
                            ))
                            .icon(icon::from_name("dialog-warning").size(64))
                            .body(fl!("startup-apps", "remove-dialog-description"))
                            .primary_action(button::suggested(fl!("remove")).on_press(
                                Message::RemoveStartupApplication(
                                    target_directory_type.clone(),
                                    app_to_remove.clone(),
                                    true,
                                ),
                            ))
                            .secondary_action(
                                button::standard(fl!("cancel"))
                                    .on_press(Message::CancelRemoveStartupApplication),
                            )
                            .apply(Element::from)
                            .map(crate::pages::Message::StartupApps),
                    );
                }
            }
        }
        None
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }

        Task::none()
    }

    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::Message> {
        match message {
            Message::UpdateStartupApplications(cached_apps) => {
                self.cached_startup_apps = Some(cached_apps);
            }
            Message::ApplicationSearch(search) => {
                self.application_search = search;
            }
            Message::ShowApplicationSidebar(directory_type) => {
                self.context = Some(Context::AddApplication(directory_type));
                return cosmic::task::message(crate::app::Message::OpenContextDrawer(
                    self.entity,
                    fl!("startup-apps", "search-for-application").into(),
                ));
            }
            Message::AddStartupApplication(directory_type, app) => {
                let mut file_name = app.clone().appid;
                file_name.push_str(".desktop");

                let directories: Vec<PathBuf> = directory_type.clone().into();

                let directory_to_target =
                    directories.get(0).expect("Always at least one directory");
                if let Ok(exists) = std::fs::exists(directory_to_target.join(file_name.clone())) {
                    if !exists {
                        // when adding an application, we want to symlink to be more user-friendly
                        // this ensures that, as an application gets updated / removed, so does the
                        // symlink
                        match std::os::unix::fs::symlink(
                            app.clone().path,
                            directory_to_target.join(file_name),
                        ) {
                            Ok(_) => {
                                if let Some(ref mut cached_startup_apps) = self.cached_startup_apps
                                {
                                    let target_apps = cached_startup_apps.apps.get(&directory_type);
                                    if let Some(target_apps) = target_apps {
                                        let mut new_apps = target_apps.clone();
                                        new_apps.push(app.clone());

                                        cached_startup_apps
                                            .apps
                                            .insert(directory_type.clone(), new_apps);
                                    }
                                }
                            }
                            Err(e) => {
                                error!(?e, "Failed to symlink");
                            }
                        }
                    }
                }

                self.context = None;
            }
            Message::RemoveStartupApplication(directory_type, app, confirm) => {
                if !confirm {
                    self.app_to_remove = Some(app);
                    self.target_directory_type = Some(directory_type);
                } else {
                    let mut file_name = app.clone().appid;
                    file_name.push_str(".desktop");

                    let directories: Vec<PathBuf> = directory_type.clone().into();

                    let directory_to_target =
                        directories.get(0).expect("Always at least one directory");
                    if let Ok(exists) = std::fs::exists(directory_to_target.join(file_name.clone()))
                    {
                        if exists {
                            // remove_file works for both regular files & symlinks
                            match std::fs::remove_file(directory_to_target.join(file_name)) {
                                Ok(_) => {
                                    if let Some(ref mut cached_startup_apps) =
                                        self.cached_startup_apps
                                    {
                                        let target_apps =
                                            cached_startup_apps.apps.get(&directory_type);
                                        if let Some(target_apps) = target_apps {
                                            let mut new_apps = Vec::new();
                                            for old_app in target_apps {
                                                if old_app != &app {
                                                    new_apps.push(old_app.clone());
                                                }
                                            }

                                            cached_startup_apps
                                                .apps
                                                .insert(directory_type.clone(), new_apps);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!(?e, "Failed to remove file");
                                }
                            }
                        }
                    }
                    self.app_to_remove = None;
                    self.target_directory_type = None;
                    self.context = None;
                }
            }
            Message::CancelRemoveStartupApplication => {
                self.app_to_remove = None;
                self.target_directory_type = None;
                self.context = None;
            }
            _ => {}
        }

        Task::none()
    }

    pub fn add_application_context_view(
        &self,
        directory_type: DirectoryType,
    ) -> Element<'_, crate::pages::Message> {
        let cosmic::cosmic_theme::Spacing {
            space_s, space_l, ..
        } = cosmic::theme::active().cosmic().spacing;

        let search = widget::search_input(fl!("type-to-search"), &self.application_search)
            .on_input(Message::ApplicationSearch)
            .on_clear(Message::ApplicationSearch(String::new()));

        let mut list = widget::list_column();
        let search_input = &self.application_search.trim().to_lowercase();

        if let Some(startup_apps) = &self.cached_startup_apps {
            for app in &startup_apps.all_apps {
                if let Some(name) = app.name(&startup_apps.locales) {
                    if let Some(exec) = app.exec() {
                        if search_input.is_empty()
                            || exec.to_lowercase().contains(search_input)
                            || name.to_lowercase().contains(search_input)
                        {
                            let mut row = widget::row::with_capacity(2).spacing(space_s);

                            if let Some(icon) = app.icon() {
                                row = row.push(icon::from_name(icon));
                            } else {
                                row = row.push(vertical_space().height(16.0));
                            }

                            if let Some(name) = app.name(&startup_apps.locales) {
                                row = row.push(text(name));
                            }

                            list = list.add(settings::flex_item_row(vec![
                                row.into(),
                                widget::button::text(fl!("add"))
                                    .on_press(Message::AddStartupApplication(
                                        directory_type.clone(),
                                        app.clone(),
                                    ))
                                    .into(),
                            ]))
                        }
                    }
                }
            }
        }

        widget::column()
            .padding([2, 0])
            .spacing(space_l)
            .push(search)
            .push(list)
            .apply(Element::from)
            .map(crate::pages::Message::StartupApps)
    }
}

fn apps() -> Section<crate::pages::Message> {
    let cosmic::cosmic_theme::Spacing { space_s, .. } = cosmic::theme::active().cosmic().spacing;

    Section::default()
        .title(fl!("startup-apps"))
        .view::<Page>(move |_binder, page, _section| {
            let mut view = widget::column::with_capacity(4)
                .spacing(cosmic::theme::active().cosmic().space_xxs());

            if let Some(startup_apps) = &page.cached_startup_apps {
                let order = vec![DirectoryType::User];
                for directory_type in order {
                    let mut section = settings::section();

                    view = view
                        .push(text::heading(directory_type.to_string()))
                        .push(text(match directory_type {
                            DirectoryType::User => fl!("startup-apps", "user-description"),
                        }));

                    if let Some(apps) = startup_apps.apps.get(&directory_type) {
                        for app in apps {
                            let mut row = widget::row::with_capacity(2).spacing(space_s);

                            if let Some(icon) = app.icon() {
                                row = row.push(icon::from_name(icon));
                            } else {
                                row = row.push(vertical_space().height(16.0));
                            }

                            if let Some(name) = app.name(&startup_apps.locales) {
                                row = row.push(text(name));
                            }

                            section = section.add(settings::flex_item_row(vec![
                                row.into(),
                                button::icon(icon::from_name("edit-delete-symbolic"))
                                    .extra_small()
                                    .on_press(Message::RemoveStartupApplication(
                                        directory_type.clone(),
                                        app.clone(),
                                        false,
                                    ))
                                    .into(),
                            ]))
                        }
                    }

                    let add_input_source = widget::button::standard(fl!("startup-apps", "add"))
                        .on_press(Message::ShowApplicationSidebar(directory_type.clone()));

                    view = view.push(section).push(widget::container(
                        widget::container(add_input_source)
                            .width(Length::Fill)
                            .align_x(Alignment::End),
                    ));
                }
            }

            view.apply(Element::from)
                .map(crate::pages::Message::StartupApps)
        })
}

fn get_all_apps(locales: Vec<String>) -> Vec<DesktopEntry> {
    let mut dedupe = HashSet::new();

    let entries = freedesktop_desktop_entry::Iter::new(freedesktop_desktop_entry::default_paths())
        .entries(Some(&locales));

    let mut result = Vec::new();

    for entry in entries {
        let app_id = entry.flatpak().unwrap_or_else(|| entry.appid.as_ref());

        if dedupe.contains(app_id) {
            continue;
        }

        if entry.exec().is_none() {
            continue;
        }

        // skip if we can't run this in COSMIC
        if let Some(only_show_in) = entry.only_show_in() {
            if !only_show_in.contains(&"COSMIC") {
                continue;
            }
        }

        if let Some(not_show_in) = entry.not_show_in() {
            if not_show_in.contains(&"COSMIC") {
                continue;
            }
        }

        result.push(entry.clone());
        dedupe.insert(app_id.to_owned());
    }

    result
}
