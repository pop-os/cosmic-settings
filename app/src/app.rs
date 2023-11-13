// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::app::DbusActivationMessage;
use cosmic::iced::Subscription;
use cosmic::iced_core::window::Id;
use cosmic::{
    app::{Command, Core},
    cosmic_config::config_subscription,
    iced::{self, event::wayland, event::PlatformSpecific, subscription, window, Length},
    prelude::*,
    widget::{
        column, container, icon, nav_bar, navigation, scrollable, search, segmented_button,
        settings,
    },
    Element,
};
use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::{self as page, section};

use crate::config::Config;

use crate::pages::desktop::appearance::COLOR_PICKER_DIALOG_ID;
use crate::pages::desktop::{
    self, appearance,
    dock::{self, applets::ADD_DOCK_APPLET_DIALOGUE_ID},
    panel::{
        self,
        applets_inner::{self, AppletsPage, APPLET_DND_ICON_ID},
        inner as _panel,
    },
};
use crate::pages::input::{self, keyboard};
use crate::pages::{sound, system, time};
use crate::subscription::desktop_files;
use crate::widget::{page_title, search_header};
use std::borrow::Cow;

#[allow(clippy::struct_excessive_bools)]
#[allow(clippy::module_name_repetitions)]
pub struct SettingsApp {
    active_page: page::Entity,
    config: Config,
    core: Core,
    nav_model: nav_bar::Model,
    pages: page::Binder<crate::pages::Message>,
    search: search::Model,
    search_selections: Vec<(page::Entity, section::Entity)>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Message {
    DesktopInfo,
    Page(page::Entity),
    PageMessage(crate::pages::Message),
    PanelConfig(CosmicPanelConfig),
    Search(search::Message),
    SetWindowTitle,
    OpenContextDrawer(Cow<'static, str>),
    CloseContextDrawer,
    SetTheme(cosmic::theme::Theme),
    DbusActivation(DbusActivationMessage),
}

impl From<DbusActivationMessage> for Message {
    fn from(msg: DbusActivationMessage) -> Self {
        Message::DbusActivation(msg)
    }
}

impl cosmic::Application for SettingsApp {
    type Executor = cosmic::executor::single::Executor;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.system76.CosmicSettings";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = SettingsApp {
            active_page: page::Entity::default(),
            config: Config::new(),
            core,
            nav_model: nav_bar::Model::default(),
            pages: page::Binder::default(),
            search: search::Model::default(),
            search_selections: Vec::default(),
        };

        let desktop_id = app.insert_page::<desktop::Page>().id();
        app.insert_page::<sound::Page>();
        app.insert_page::<system::Page>();
        app.insert_page::<time::Page>();
        app.insert_page::<input::Page>();

        let active_id = app
            .pages
            .find_page_by_id(&app.config.active_page)
            .map_or(desktop_id, |(id, _info)| id);

        let command = app.activate_page(active_id);

        (app, command)
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Self::Message> {
        let message = if id == applets_inner::ADD_PANEL_APPLET_DIALOGUE_ID {
            Message::PageMessage(crate::pages::Message::PanelApplet(
                applets_inner::Message::ClosedAppletDialogue,
            ))
        } else if id == ADD_DOCK_APPLET_DIALOGUE_ID {
            Message::PageMessage(crate::pages::Message::DockApplet(dock::applets::Message(
                applets_inner::Message::ClosedAppletDialogue,
            )))
        } else if id == COLOR_PICKER_DIALOG_ID {
            Message::PageMessage(crate::pages::Message::Appearance(
                appearance::Message::CloseRequested,
            ))
        } else {
            return None;
        };

        Some(message)
    }

    fn on_escape(&mut self) -> Command<Self::Message> {
        if self.search.is_active() {
            self.search.state = search::State::Inactive;
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
        self.search.focus()
    }

    fn subscription(&self) -> Subscription<Message> {
        let window_break = subscription::events_with(|event, _| match event {
            iced::Event::PlatformSpecific(PlatformSpecific::Wayland(wayland::Event::Output(
                wayland::OutputEvent::Created(Some(info)),
                o,
            ))) if info.name.is_some() => Some(Message::PageMessage(crate::pages::Message::Panel(
                panel::Message(_panel::Message::OutputAdded(info.name.unwrap(), o)),
            ))),
            iced::Event::PlatformSpecific(PlatformSpecific::Wayland(wayland::Event::Output(
                wayland::OutputEvent::Removed,
                o,
            ))) => Some(Message::PageMessage(crate::pages::Message::Panel(
                panel::Message(_panel::Message::OutputRemoved(o)),
            ))),
            _ => None,
        });

        Subscription::batch(vec![
            window_break,
            desktop_files(0).map(|_| Message::DesktopInfo),
            config_subscription(0, "com.system76.CosmicPanel.Panel".into(), 1).map(
                |(_, e)| match e {
                    Ok(config) => Message::PanelConfig(config),
                    Err((errors, config)) => {
                        for why in errors {
                            tracing::error!(?why, "panel config load error");
                        }
                        Message::PanelConfig(config)
                    }
                },
            ),
            config_subscription(0, "com.system76.CosmicPanel.Dock".into(), 1).map(
                |(_, e)| match e {
                    Ok(config) => Message::PanelConfig(config),
                    Err((errors, config)) => {
                        for why in errors {
                            tracing::error!(?why, "dock config load error");
                        }
                        Message::PanelConfig(config)
                    }
                },
            ),
        ])
    }

    #[allow(clippy::too_many_lines)]
    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Page(page) => return self.activate_page(page),

            Message::SetWindowTitle => return self.set_title(),

            Message::Search(search::Message::Activate) => {
                return self.search.focus();
            }

            Message::Search(search::Message::Changed(phrase)) => {
                self.search_changed(phrase);
            }

            Message::Search(search::Message::Clear) => {
                self.search_clear();
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
                    page::update!(self.pages, message, desktop::wallpaper::Page);
                }
                crate::pages::Message::Input(message) => {
                    if let Some(page) = self.pages.page_mut::<input::Page>() {
                        return page.update(message).map(cosmic::app::Message::App);
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
                        return page
                            .update(message, applets_inner::ADD_PANEL_APPLET_DIALOGUE_ID)
                            .map(cosmic::app::Message::App);
                    }
                }
                crate::pages::Message::Dock(message) => {
                    page::update!(self.pages, message, dock::Page);
                }
                crate::pages::Message::DockApplet(message) => {
                    if let Some(page) = self.pages.page_mut::<dock::applets::Page>() {
                        return page.update(message).map(cosmic::app::Message::App);
                    }
                }
                crate::pages::Message::Appearance(message) => {
                    if let Some(page) = self.pages.page_mut::<appearance::Page>() {
                        return page.update(message).map(cosmic::app::Message::App);
                    }
                    // TODO
                }
            },

            Message::PanelConfig(config) if config.name.to_lowercase().contains("panel") => {
                page::update!(
                    self.pages,
                    panel::Message(_panel::Message::PanelConfig(config.clone())),
                    panel::Page
                );

                if let Some(page) = self.pages.page_mut::<applets_inner::Page>() {
                    return page
                        .update(
                            applets_inner::Message::PanelConfig(config),
                            applets_inner::ADD_PANEL_APPLET_DIALOGUE_ID,
                        )
                        .map(cosmic::app::Message::App);
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
                )
            }

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
                        .update(
                            applets_inner::Message::Applets(info_list),
                            applets_inner::ADD_PANEL_APPLET_DIALOGUE_ID,
                        )
                        .map(cosmic::app::Message::App);
                }
            }
            Message::PanelConfig(_) | Message::Search(_) => {}
            Message::SetTheme(t) => return cosmic::app::command::set_theme(t),
            Message::OpenContextDrawer(title) => {
                self.core.window.show_context = true;
                self.set_context_title(title.to_string());
            }
            Message::CloseContextDrawer => {
                self.core.window.show_context = false;
            }
            Message::DbusActivation(mut msg) => {
                let mut cmds = Vec::with_capacity(1);
                dbg!(&msg);
                // try to use token for xdg-activation
                if let Some(token) = msg.activation_token.take() {
                    cmds.push(cosmic::iced_sctk::commands::activation::activate(
                        Id::default(),
                        token,
                    ));
                }
                // if flag args are passed, use those to change the page

                return Command::batch(cmds);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let page_view = if self.search.is_active() {
            self.search_view()
        } else if let Some(content) = self.pages.content(self.active_page) {
            self.page_view(content)
        } else if let Some(sub_pages) = self.pages.sub_pages(self.active_page) {
            self.sub_page_view(sub_pages)
        } else {
            panic!("page without sub-pages or content");
        };

        let padding = if self.core.is_condensed() { 0 } else { 64 };

        container(page_view)
            .max_width(800)
            .width(Length::Fill)
            .apply(container)
            .center_x()
            .padding([0, padding])
            .width(Length::Fill)
            .apply(scrollable)
            .into()
    }

    #[allow(clippy::too_many_lines)]
    fn view_window(&self, id: window::Id) -> Element<Message> {
        if let Some(Some(page)) =
            (id == APPLET_DND_ICON_ID).then(|| self.pages.page::<applets_inner::Page>())
        {
            return page.dnd_icon();
        }
        if let Some(Some(page)) = (id == applets_inner::ADD_PANEL_APPLET_DIALOGUE_ID)
            .then(|| self.pages.page::<applets_inner::Page>())
        {
            return page.add_applet_view(crate::pages::Message::PanelApplet);
        }
        if let Some(Some(page)) = (id == appearance::COLOR_PICKER_DIALOG_ID)
            .then(|| self.pages.page::<appearance::Page>())
        {
            return page.color_picker_view();
        }
        if let Some(Some(page)) =
            (id == ADD_DOCK_APPLET_DIALOGUE_ID).then(|| self.pages.page::<dock::applets::Page>())
        {
            return page.inner().add_applet_view(|msg| {
                crate::pages::Message::DockApplet(dock::applets::Message(msg))
            });
        }
        if let Some(Some(page)) =
            (id == keyboard::ADD_INPUT_SOURCE_DIALOGUE_ID).then(|| self.pages.page::<input::Page>())
        {
            return page.add_input_source_view();
        }
        if let Some(Some(page)) = (id == keyboard::SPECIAL_CHARACTER_DIALOGUE_ID)
            .then(|| self.pages.page::<input::Page>())
        {
            return page.special_character_key_view();
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
}

impl SettingsApp {
    /// Activates a page.
    fn activate_page(&mut self, page: page::Entity) -> Command<crate::Message> {
        let current_page = self.active_page;
        self.active_page = page;

        let mut leave_command = iced::Command::none()
            .map(Message::PageMessage)
            .map(cosmic::app::Message::App);

        if current_page != page {
            leave_command = self
                .pages
                .on_leave(current_page)
                .unwrap_or(iced::Command::none())
                .map(Message::PageMessage)
                .map(cosmic::app::Message::App);
            self.config.active_page = Box::from(&*self.pages.info[page].id);
            self.config
                .set_active_page(Box::from(&*self.pages.info[page].id));
        }

        self.search_clear();
        self.search.state = search::State::Inactive;
        self.activate_navbar(page);

        let page_command = self
            .pages
            .page_reload(page)
            .unwrap_or(iced::Command::none())
            .map(Message::PageMessage)
            .map(cosmic::app::Message::App);

        Command::batch(vec![
            leave_command,
            page_command,
            cosmic::command::future(async { Message::SetWindowTitle })
                .map(cosmic::app::Message::App),
        ])
    }

    fn set_title(&mut self) -> Command<crate::Message> {
        self.set_window_title(format!(
            "{} - COSMIC Settings",
            self.pages.info[self.active_page].title
        ))
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
            .icon(icon::from_name(&*page.icon_name).into())
            .data(id)
            .with_id(|nav_id| self.pages.data_set(id, nav_id))
    }

    /// Displays the view of a page.
    fn page_view(&self, content: &[section::Entity]) -> cosmic::Element<Message> {
        let page = &self.pages.info[self.active_page];
        let mut column_widgets = Vec::with_capacity(1);

        if let Some(parent) = page.parent {
            column_widgets.push(navigation::sub_page_header(
                page.title.as_str(),
                self.pages.info[parent].title.as_str(),
                Message::Page(parent),
            ));
        }

        column_widgets.reserve_exact(1 + content.len());
        for id in content.iter().copied() {
            let section = &self.pages.sections[id];
            let model = &self.pages.page[self.active_page];

            column_widgets.push(
                (section.view_fn)(&self.pages, model.as_ref(), section).map(Message::PageMessage),
            );
        }

        settings::view_column(column_widgets).padding(0).into()
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

        self.search.phrase = phrase;
    }

    /// Clears the search results so that the search page will not be shown.
    fn search_clear(&mut self) {
        self.search_selections.clear();
        self.search.phrase.clear();
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

            let section = (section.view_fn)(&self.pages, model.as_ref(), section)
                .map(Message::PageMessage)
                .apply(iced::widget::container)
                .padding([0, 0, 0, 48]);

            sections.push(section.into());
        }

        settings::view_column(sections).into()
    }

    /// Displays the sub-pages view of a page.
    fn sub_page_view(&self, sub_pages: &[page::Entity]) -> cosmic::Element<Message> {
        let mut page_list = column::with_capacity(sub_pages.len()).spacing(18);

        for entity in sub_pages.iter().copied() {
            let sub_page = &self.pages.info[entity];
            page_list = page_list.push(navigation::page_list_item(
                sub_page.title.as_str(),
                sub_page.description.as_str(),
                &sub_page.icon_name,
                entity,
            ));
        }

        column::with_capacity(2)
            .push(page_title(&self.pages.info[self.active_page]))
            .push(Element::from(page_list).map(Message::Page))
            .spacing(24)
            .into()
    }
}
