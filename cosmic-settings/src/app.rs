// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::config::Config;
use crate::pages::desktop::{
    self, appearance, dock,
    panel::{
        self,
        applets_inner::{self, APPLET_DND_ICON_ID},
        inner as _panel,
    },
};
use crate::pages::input::{self};
use crate::pages::{self, display, power, sound, system, time};
use crate::subscription::desktop_files;
use crate::widget::{page_title, search_header};
use crate::PageCommands;
use cosmic::app::DbusActivationMessage;
use cosmic::cctk::sctk::output::OutputInfo;
use cosmic::cctk::wayland_client::protocol::wl_output::WlOutput;
use cosmic::iced::futures::SinkExt;
use cosmic::iced::Subscription;
use cosmic::widget::{self, button, row, text_input};
use cosmic::{
    app::{Command, Core},
    iced::{
        self,
        event::{self, wayland, PlatformSpecific},
        window, Length,
    },
    prelude::*,
    widget::{
        column, container, icon, id_container, nav_bar, scrollable, segmented_button, settings,
    },
    Element,
};
use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::{self as page, section};
use page::Entity;
use std::{borrow::Cow, str::FromStr};

#[allow(clippy::struct_excessive_bools)]
#[allow(clippy::module_name_repetitions)]
pub struct SettingsApp {
    active_page: page::Entity,
    config: Config,
    core: Core,
    nav_model: nav_bar::Model,
    page_sender: Option<tokio::sync::mpsc::Sender<crate::pages::Message>>,
    pages: page::Binder<crate::pages::Message>,
    search_active: bool,
    search_id: cosmic::widget::Id,
    search_input: String,
    search_selections: Vec<(page::Entity, section::Entity)>,
}

impl SettingsApp {
    fn subcommand_to_page(&self, cmd: &PageCommands) -> Option<Entity> {
        match cmd {
            PageCommands::About => self.pages.page_id::<system::about::Page>(),
            PageCommands::Appearance => self.pages.page_id::<desktop::appearance::Page>(),
            PageCommands::Bluetooth => None,
            PageCommands::DateTime => self.pages.page_id::<time::date::Page>(),
            PageCommands::Displays => self.pages.page_id::<display::Page>(),
            PageCommands::Dock => self.pages.page_id::<desktop::dock::Page>(),
            PageCommands::Firmware => self.pages.page_id::<system::firmware::Page>(),
            PageCommands::Keyboard => self.pages.page_id::<input::keyboard::Page>(),
            PageCommands::Mouse => self.pages.page_id::<input::mouse::Page>(),
            PageCommands::Network => None,
            PageCommands::Panel => self.pages.page_id::<desktop::panel::Page>(),
            PageCommands::Power => self.pages.page_id::<power::Page>(),
            PageCommands::RegionLanguage => self.pages.page_id::<time::region::Page>(),
            PageCommands::Sound => self.pages.page_id::<sound::Page>(),
            PageCommands::Time => self.pages.page_id::<time::Page>(),
            PageCommands::Touchpad => self.pages.page_id::<input::touchpad::Page>(),
            PageCommands::Users => self.pages.page_id::<system::users::Page>(),
            PageCommands::Wallpaper => self.pages.page_id::<desktop::wallpaper::Page>(),
            PageCommands::WindowManagement => {
                self.pages.page_id::<desktop::window_management::Page>()
            }
            PageCommands::Workspaces => self.pages.page_id::<desktop::workspaces::Page>(),
        }
    }

    fn id(&self) -> cosmic::iced_core::id::Id {
        let cur_page_name = self.pages.info[self.active_page].id.as_ref();
        cosmic::iced_core::id::Id::new(cur_page_name.to_owned())
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    CloseContextDrawer,
    DelayedInit(page::Entity),
    DesktopInfo,
    Error(String),
    OpenContextDrawer(Cow<'static, str>),
    OutputAdded(OutputInfo, WlOutput),
    OutputRemoved(WlOutput),
    Page(page::Entity),
    PageMessage(crate::pages::Message),
    PanelConfig(CosmicPanelConfig),
    RegisterSubscriptionSender(tokio::sync::mpsc::Sender<pages::Message>),
    SearchActivate,
    SearchChanged(String),
    SearchClear,
    SearchSubmit,
    SetTheme(cosmic::theme::Theme),
    SetWindowTitle,
}

impl cosmic::Application for SettingsApp {
    type Executor = cosmic::executor::single::Executor;
    type Flags = crate::Args;
    type Message = Message;

    const APP_ID: &'static str = "com.system76.CosmicSettings";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = SettingsApp {
            active_page: page::Entity::default(),
            config: Config::new(),
            core,
            nav_model: nav_bar::Model::default(),
            page_sender: None,
            pages: page::Binder::default(),
            search_active: false,
            search_id: cosmic::widget::Id::unique(),
            search_input: String::new(),
            search_selections: Vec::default(),
        };

        let desktop_id = app.insert_page::<desktop::Page>().id();
        app.insert_page::<input::Page>();
        app.insert_page::<display::Page>();
        //app.insert_page::<sound::Page>();
        app.insert_page::<system::Page>();
        app.insert_page::<time::Page>();
        app.insert_page::<power::Page>();

        let active_id = match flags.subcommand {
            Some(p) => app.subcommand_to_page(&p),
            None => app
                .pages
                .find_page_by_id(&app.config.active_page)
                .map(|(id, _info)| id),
        }
        .unwrap_or(desktop_id);

        (
            app,
            cosmic::command::message(Message::DelayedInit(active_id)),
        )
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let mut widgets = Vec::new();

        widgets.push(if self.search_active {
            text_input::search_input("", &self.search_input)
                .width(Length::Fixed(240.0))
                .id(self.search_id.clone())
                .on_clear(Message::SearchClear)
                .on_input(Message::SearchChanged)
                .on_submit(Message::SearchSubmit)
                .into()
        } else {
            icon::from_name("system-search-symbolic")
                .apply(button::icon)
                .on_press(Message::SearchActivate)
                .into()
        });

        widgets
    }

    fn on_escape(&mut self) -> Command<Self::Message> {
        if self.search_active {
            self.search_active = false;
            self.search_clear();
        }

        Command::none()
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Command<Self::Message> {
        if let Some(page) = self.nav_model.data::<page::Entity>(id).copied() {
            return self.activate_page(page);
        }

        Command::none()
    }

    fn on_search(&mut self) -> Command<Self::Message> {
        self.search_active = true;
        cosmic::widget::text_input::focus(self.search_id.clone())
    }

    fn subscription(&self) -> Subscription<Message> {
        // Handling of Wayland-specific events received.
        let wayland_events =
            event::listen_with(|event, _| match event {
                iced::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::Output(wayland::OutputEvent::Created(Some(info)), o),
                )) if info.name.is_some() => Some(Message::OutputAdded(info, o)),
                iced::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::Output(wayland::OutputEvent::Removed, o),
                )) => Some(Message::OutputRemoved(o)),
                _ => None,
            });

        Subscription::batch(vec![
            // Creates a channel that listens to messages from pages.
            // The sender is given back to the application so that it may pass it on.
            cosmic::iced::subscription::channel(
                std::any::TypeId::of::<Self>(),
                4,
                move |mut output| async move {
                    let (tx, mut rx) = tokio::sync::mpsc::channel::<pages::Message>(4);

                    let _res = output.send(Message::RegisterSubscriptionSender(tx)).await;

                    while let Some(event) = rx.recv().await {
                        let _res = output.send(Message::PageMessage(event)).await;
                    }

                    futures::future::pending().await
                },
            ),
            crate::subscription::daytime().map(|daytime| {
                Message::PageMessage(pages::Message::Appearance(appearance::Message::Daytime(
                    daytime,
                )))
            }),
            wayland_events,
            // Watch for changes to installed desktop entries
            desktop_files(0).map(|_| Message::DesktopInfo),
            // Watch for configuration changes to the panel.
            self.core()
                .watch_config::<CosmicPanelConfig>("com.system76.CosmicPanel.Panel")
                .map(|update| {
                    for why in update.errors {
                        tracing::error!(?why, "panel config load error");
                    }

                    Message::PanelConfig(update.config)
                }),
            // Watch for configuration changes to the dock
            self.core()
                .watch_config::<CosmicPanelConfig>("com.system76.CosmicPanel.Dock")
                .map(|update| {
                    for why in update.errors {
                        tracing::error!(?why, "dock config load error");
                    }

                    Message::PanelConfig(update.config)
                }),
            // Watch for state changes from the cosmic-bg session service.
            self.core()
                .watch_state::<cosmic_bg_config::state::State>(cosmic_bg_config::NAME)
                .map(|update| {
                    Message::PageMessage(pages::Message::DesktopWallpaper(
                        pages::desktop::wallpaper::Message::UpdateState(update.config),
                    ))
                }),
        ])
    }

    #[allow(clippy::too_many_lines)]
    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Page(page) => return self.activate_page(page),

            Message::SetWindowTitle => return self.set_title(),

            Message::SearchChanged(phrase) => {
                self.search_changed(phrase);
            }

            Message::SearchActivate => {
                self.search_active = true;
                return cosmic::widget::text_input::focus(self.search_id.clone());
            }

            Message::SearchClear => {
                self.search_clear();
            }

            Message::SearchSubmit => {
                self.search_active = true;
            }

            Message::PageMessage(message) => match message {
                crate::pages::Message::About(message) => {
                    page::update!(self.pages, message, system::about::Page);
                }

                crate::pages::Message::DateAndTime(message) => {
                    page::update!(self.pages, message, time::date::Page);
                }

                crate::pages::Message::Desktop(message) => {
                    page::update!(self.pages, message, desktop::Page);
                }

                crate::pages::Message::DesktopWallpaper(message) => {
                    if let Some(page) = self.pages.page_mut::<desktop::wallpaper::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::DesktopWorkspaces(message) => {
                    page::update!(self.pages, message, desktop::workspaces::Page);
                }

                crate::pages::Message::Displays(message) => {
                    if let Some(page) = self.pages.page_mut::<display::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::Keyboard(message) => {
                    if let Some(page) = self.pages.page_mut::<input::keyboard::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::KeyboardShortcuts(message) => {
                    if let Some(page) = self.pages.page_mut::<input::keyboard::shortcuts::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::CustomShortcuts(message) => {
                    if let Some(page) = self
                        .pages
                        .page_mut::<input::keyboard::shortcuts::custom::Page>()
                    {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::ManageWindowShortcuts(message) => {
                    if let Some(page) = self
                        .pages
                        .page_mut::<input::keyboard::shortcuts::manage_windows::Page>()
                    {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::MoveWindowShortcuts(message) => {
                    if let Some(page) = self
                        .pages
                        .page_mut::<input::keyboard::shortcuts::move_window::Page>()
                    {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::NavShortcuts(message) => {
                    if let Some(page) = self
                        .pages
                        .page_mut::<input::keyboard::shortcuts::nav::Page>()
                    {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::SystemShortcuts(message) => {
                    if let Some(page) = self
                        .pages
                        .page_mut::<input::keyboard::shortcuts::system::Page>()
                    {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::TilingShortcuts(message) => {
                    if let Some(page) = self
                        .pages
                        .page_mut::<input::keyboard::shortcuts::tiling::Page>()
                    {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::Input(message) => {
                    if let Some(page) = self.pages.page_mut::<input::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::External { .. } => {
                    todo!("external plugins not supported yet");
                }

                crate::pages::Message::Page(page) => {
                    return self.activate_page(page);
                }

                crate::pages::Message::Panel(message) => {
                    page::update!(self.pages, message, panel::Page);
                }

                crate::pages::Message::PanelApplet(message) => {
                    if let Some(page) = self.pages.page_mut::<applets_inner::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::Dock(message) => {
                    page::update!(self.pages, message, dock::Page);
                }

                crate::pages::Message::DockApplet(message) => {
                    if let Some(page) = self.pages.page_mut::<dock::applets::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::Appearance(message) => {
                    if let Some(page) = self.pages.page_mut::<appearance::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::Power(message) => {
                    page::update!(self.pages, message, power::Page);
                }

                crate::pages::Message::WindowManagement(message) => {
                    page::update!(self.pages, message, desktop::window_management::Page);
                }
            },

            Message::OutputAdded(info, output) => {
                if let Some(page) = self.pages.page_mut::<panel::Page>() {
                    page.update(panel::Message(_panel::Message::OutputAdded(
                        info.name.clone().unwrap_or_default(),
                        output.clone(),
                    )));
                }
                // dock
                if let Some(page) = self.pages.page_mut::<dock::Page>() {
                    page.update(dock::Message::Inner(_panel::Message::OutputAdded(
                        info.name.unwrap_or_default(),
                        output,
                    )));
                }
            }

            Message::OutputRemoved(output) => {
                if let Some(page) = self.pages.page_mut::<panel::Page>() {
                    page.update(panel::Message(_panel::Message::OutputRemoved(
                        output.clone(),
                    )));
                }
                // dock
                if let Some(page) = self.pages.page_mut::<dock::Page>() {
                    page.update(dock::Message::Inner(_panel::Message::OutputRemoved(output)));
                }
            }

            Message::PanelConfig(config) if config.name.to_lowercase().contains("panel") => {
                page::update!(
                    self.pages,
                    panel::Message(_panel::Message::PanelConfig(config.clone())),
                    panel::Page
                );

                if let Some(page) = self.pages.page_mut::<applets_inner::Page>() {
                    return page
                        .update(applets_inner::Message::PanelConfig(config))
                        .map(Into::into);
                }
            }

            Message::PanelConfig(config) if config.name.to_lowercase().contains("dock") => {
                page::update!(
                    self.pages,
                    dock::Message::Inner(_panel::Message::PanelConfig(config.clone(),)),
                    dock::Page
                );
                page::update!(
                    self.pages,
                    dock::applets::Message(applets_inner::Message::PanelConfig(config,)),
                    dock::applets::Page
                );
            }

            Message::PanelConfig(_) => {}

            Message::DesktopInfo => {
                let info_list: Vec<_> = freedesktop_desktop_entry::Iter::new(
                    freedesktop_desktop_entry::default_paths(),
                )
                .filter_map(|p| applets_inner::Applet::try_from(Cow::from(p)).ok())
                .collect();

                page::update!(
                    self.pages,
                    dock::applets::Message(applets_inner::Message::Applets(info_list.clone())),
                    dock::applets::Page
                );
                if let Some(page) = self.pages.page_mut::<applets_inner::Page>() {
                    return page
                        .update(applets_inner::Message::Applets(info_list))
                        .map(Into::into);
                }
            }

            Message::SetTheme(t) => return cosmic::app::command::set_theme(t),

            Message::OpenContextDrawer(title) => {
                self.core.window.show_context = true;
                self.set_context_title(title.to_string());
            }

            Message::CloseContextDrawer => {
                self.core.window.show_context = false;
            }

            Message::Error(error) => {
                tracing::error!(error, "error occurred");
            }

            Message::RegisterSubscriptionSender(sender) => {
                self.page_sender = Some(sender);
            }

            // It is necessary to delay init to allow time for the page sender to be initialized
            Message::DelayedInit(active_id) => {
                if self.page_sender.is_none() {
                    return cosmic::command::message(Message::DelayedInit(active_id));
                }

                return self.activate_page(active_id);
            }
        }

        Command::none()
    }

    fn dbus_activation(&mut self, msg: DbusActivationMessage) -> Command<Self::Message> {
        match msg.msg {
            cosmic::app::DbusActivationDetails::Activate
            | cosmic::app::DbusActivationDetails::Open { .. } => None,
            cosmic::app::DbusActivationDetails::ActivateAction { action, .. } => {
                PageCommands::from_str(&action)
                    .ok()
                    .and_then(|action| self.subcommand_to_page(&action))
                    .map(|e| self.activate_page(e))
            }
        }
        .unwrap_or_else(Command::none)
    }

    fn view(&self) -> Element<Message> {
        let view = if self.search_active && !self.search_input.is_empty() {
            self.search_view()
        } else if let Some(content) = self.pages.content(self.active_page) {
            self.page_view(content)
        } else if let Some(sub_pages) = self.pages.sub_pages(self.active_page) {
            self.sub_page_view(sub_pages)
        } else {
            return self.page_container(row::row());
        };

        container(view)
            .padding([cosmic::theme::active().cosmic().space_xxs(), 0])
            .into()
    }

    #[allow(clippy::too_many_lines)]
    fn view_window(&self, id: window::Id) -> Element<Message> {
        if let Some(Some(page)) =
            (id == *APPLET_DND_ICON_ID).then(|| self.pages.page::<applets_inner::Page>())
        {
            return page.dnd_icon();
        }

        if let Some(page) = self.pages.page::<desktop::wallpaper::Page>() {
            if id == page.color_dialog {
                return page.show_color_dialog();
            }
        }

        panic!("unknown window ID: {id:?}");
    }

    fn context_drawer(&self) -> Option<Element<Message>> {
        if self.core.window.show_context {
            self.pages
                .context_drawer(self.active_page)
                .map(|e| e.map(Message::PageMessage))
        } else {
            None
        }
    }

    fn dialog(&self) -> Option<Element<Self::Message>> {
        self.pages
            .dialog(self.active_page)
            .map(|e| e.map(Message::PageMessage))
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Self::Message> {
        if id == window::Id::MAIN {
            std::thread::spawn(|| {
                std::thread::sleep(tokio::time::Duration::from_millis(100));
                std::process::exit(0);
            });
        }
        None
    }
}

impl SettingsApp {
    /// Activates a page.
    fn activate_page(&mut self, page: page::Entity) -> Command<crate::Message> {
        let current_page = self.active_page;
        self.active_page = page;

        let mut leave_command = iced::Command::none();

        if current_page != page {
            leave_command = self
                .pages
                .on_leave(current_page)
                .unwrap_or(iced::Command::none())
                .map(Message::PageMessage)
                .map(Into::into);
            self.config.active_page = Box::from(&*self.pages.info[page].id);
            self.config
                .set_active_page(Box::from(&*self.pages.info[page].id));
        }

        self.search_clear();
        self.search_active = false;
        self.activate_navbar(page);

        let sender = self
            .page_sender
            .clone()
            .expect("sender should be available");

        let page_command = self
            .pages
            .on_enter(page, sender)
            .map(Message::PageMessage)
            .map(Into::into);

        Command::batch(vec![
            leave_command,
            page_command,
            cosmic::command::future(async { Message::SetWindowTitle }),
        ])
    }

    fn set_title(&mut self) -> Command<crate::Message> {
        self.set_window_title(
            format!(
                "{} - COSMIC Settings",
                self.pages.info[self.active_page].title
            ),
            window::Id::MAIN,
        )
    }

    /// Activates the navbar item associated with a page.
    fn activate_navbar(&mut self, mut page: page::Entity) {
        if let Some(parent) = self.pages.info[page].parent {
            page = parent;
        }

        if let Some(nav_id) = self.pages.data(page) {
            self.nav_model.activate(*nav_id);
        }
    }

    /// Adds a main page to the settings application.
    fn insert_page<P: page::AutoBind<crate::pages::Message>>(
        &mut self,
    ) -> page::Insert<crate::pages::Message> {
        let id = self.pages.register::<P>().id();
        self.navbar_insert(id);

        page::Insert {
            model: &mut self.pages,
            id,
        }
    }

    fn navbar_insert(&mut self, id: page::Entity) -> segmented_button::SingleSelectEntityMut {
        let page = &self.pages.info[id];

        self.nav_model
            .insert()
            .text(page.title.clone())
            .icon(icon::from_name(&*page.icon_name))
            .data(id)
            .with_id(|nav_id| self.pages.data_set(id, nav_id))
    }

    /// Displays the view of a page.
    fn page_view(&self, content: &[section::Entity]) -> cosmic::Element<Message> {
        let page = &self.pages.page[self.active_page];
        let page_info = &self.pages.info[self.active_page];
        let mut sections_column = Vec::with_capacity(content.len());

        let header = if let Some(custom_header) = page.header() {
            custom_header.map(Message::from)
        } else if let Some(parent) = page_info.parent {
            let page_header = crate::widget::sub_page_header(
                page_info.title.as_str(),
                self.pages.info[parent].title.as_str(),
                Message::Page(parent),
            );

            let mut page_header_content: cosmic::iced_widget::Row<'_, Message, Theme> =
                row::with_capacity(2)
                    .align_items(iced::Alignment::End)
                    .push(page_header);

            if let Some(element) = page.header_view() {
                page_header_content = page_header_content.push(element.map(Message::from));
            }

            page_header_content.into()
        } else {
            cosmic::widget::text::title3(&page_info.title).into()
        };

        for id in content.iter().copied() {
            let section = &self.pages.sections[id];
            let model = &self.pages.page[self.active_page];

            if section
                .show_while
                .as_ref()
                .map_or(true, |func| func(model.as_ref()))
            {
                sections_column.push(
                    (section.view_fn)(&self.pages, model.as_ref(), section)
                        .map(Message::PageMessage),
                );
            }
        }

        let view = self
            .page_container(settings::view_column(sections_column).padding(0))
            .apply(scrollable)
            .height(Length::Fill)
            .apply(|w| id_container(w, self.id()));

        widget::column::with_capacity(3)
            .push(self.page_container(header))
            .push(widget::vertical_space(24))
            .push(view)
            .height(Length::Fill)
            .into()
    }

    fn search_changed(&mut self, phrase: String) {
        // If the text was cleared, clear the search results too.
        if phrase.is_empty() {
            self.search_clear();
            return;
        }

        // Create a case-insensitive regular expression for the search function.
        let search_expression = regex::RegexBuilder::new(&phrase)
            .case_insensitive(true)
            .unicode(true)
            .ignore_whitespace(true)
            .size_limit(16 * 1024)
            .build();

        if let Ok(expression) = search_expression {
            // With the new search expression, generate new search results.
            let results: Vec<_> = self.pages.search(&expression).collect();

            // Use the results if results were found.
            if !results.is_empty() {
                self.search_selections = results;
            }
        }

        self.search_input = phrase;
    }

    /// Clears the search results so that the search page will not be shown.
    fn search_clear(&mut self) {
        self.search_selections.clear();
        self.search_input.clear();
    }

    /// Displays the search view.
    fn search_view(&self) -> cosmic::Element<Message> {
        let mut sections: Vec<cosmic::Element<Message>> = Vec::new();

        let mut current_page = page::Entity::default();
        for (page, section) in self.search_selections.iter().copied() {
            let section = &self.pages.sections[section];
            let model = &self.pages.page[page];

            if page != current_page {
                current_page = page;
                sections.push(search_header(&self.pages, page));
            }

            if section
                .show_while
                .as_ref()
                .map_or(true, |func| func(model.as_ref()))
            {
                let section = (section.view_fn)(&self.pages, model.as_ref(), section)
                    .map(Message::PageMessage)
                    .apply(iced::widget::container)
                    .padding([0, 0, 0, cosmic::theme::active().cosmic().space_xl()]);

                sections.push(section.into());
            }
        }

        self.page_container(settings::view_column(sections).padding(0))
            .apply(scrollable)
            .into()
    }

    /// Displays the sub-pages view of a page.
    fn sub_page_view(&self, sub_pages: &[page::Entity]) -> cosmic::Element<Message> {
        let theme = cosmic::theme::active();

        let page_list = sub_pages
            .iter()
            .copied()
            .fold(
                column::with_capacity(sub_pages.len()),
                |page_list, entity| {
                    let sub_page = &self.pages.info[entity];

                    page_list.push(crate::widget::page_list_item(
                        sub_page.title.as_str(),
                        sub_page.description.as_str(),
                        &sub_page.icon_name,
                        entity,
                    ))
                },
            )
            .spacing(theme.cosmic().space_s())
            .padding(0)
            .apply(|widget| scrollable(self.page_container(widget)).height(Length::Fill))
            .apply(Element::from)
            .map(Message::Page);

        widget::column::with_capacity(3)
            .push(self.page_container(page_title(&self.pages.info[self.active_page])))
            .push(widget::vertical_space(theme.cosmic().space_m()))
            .push(page_list)
            .height(Length::Fill)
            .into()
    }

    fn page_container<'a, Message: 'static>(
        &self,
        content: impl Into<cosmic::Element<'a, Message>>,
    ) -> cosmic::Element<'a, Message> {
        let theme = cosmic::theme::active();

        let padding = if self.core.is_condensed() {
            0
        } else {
            theme.cosmic().space_l()
        };

        container(content.into())
            .max_width(800)
            .width(Length::Fill)
            .apply(container)
            .center_x()
            .padding([0, padding])
            .width(Length::Fill)
            .into()
    }
}
