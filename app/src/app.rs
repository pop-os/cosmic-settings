// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use apply::Apply;

use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::{self as page, section};

use cosmic::iced::Subscription;

use cosmic::{
    app::{ApplicationExt, Command, Core},
    cosmic_config::config_subscription,
    iced::widget::{horizontal_space, row},
    iced::{self, event::wayland, event::PlatformSpecific, subscription, window, Length},
    widget::{nav_bar, scrollable, search, segmented_button, settings, IconSource},
    Element, ElementExt,
};

use crate::config::Config;
use crate::pages::desktop::{self, panel};
use crate::pages::{sound, system, time};
use crate::subscription::desktop_files;
use crate::widget::{page_title, parent_page_button, search_header, sub_page_button};
use panel::applets::{self, APPLET_DND_ICON_ID};
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
    Page(page::Entity),
    PageMessage(crate::pages::Message),
    Search(search::Message),
    PanelConfig(CosmicPanelConfig),
    DesktopInfo,
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

        let active_id = app
            .pages
            .info
            .iter()
            .find(|(_id, info)| info.id == *app.config.active_page)
            .map_or(desktop_id, |(id, _info)| id);

        let command = Command::batch(vec![
            app.set_title(crate::fl!("app")),
            app.activate_page(active_id),
        ]);

        (app, command)
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Self::Message> {
        if id == applets::ADD_APPLET_DIALOGUE_ID {
            Some(Message::PageMessage(crate::pages::Message::Applet(
                applets::Message::ClosedAppletDialogue,
            )))
        } else {
            None
        }
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

    fn subscription(&self) -> Subscription<Self::Message> {
        let window_break = subscription::events_with(|event, _| match event {
            iced::Event::PlatformSpecific(PlatformSpecific::Wayland(wayland::Event::Output(
                wayland::OutputEvent::Created(Some(info)),
                o,
            ))) if info.name.is_some() => Some(Message::PageMessage(crate::pages::Message::Panel(
                panel::Message::OutputAdded(info.name.unwrap(), o),
            ))),
            iced::Event::PlatformSpecific(PlatformSpecific::Wayland(wayland::Event::Output(
                wayland::OutputEvent::Removed,
                o,
            ))) => Some(Message::PageMessage(crate::pages::Message::Panel(
                panel::Message::OutputRemoved(o),
            ))),
            _ => None,
        });

        let config = config_subscription(0, "com.system76.CosmicPanel.Panel".into(), 1).map(
            |(_, e)| match e {
                Ok(config) => Message::PanelConfig(config),
                Err((errors, config)) => {
                    for why in errors {
                        tracing::error!(?why, "panel config load error");
                    }
                    Message::PanelConfig(config)
                }
            },
        );

        let subscriptions = vec![
            window_break,
            desktop_files(0).map(|_| Message::DesktopInfo),
            config,
        ];

        Subscription::batch(subscriptions)
    }

    #[allow(clippy::too_many_lines)]
    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Page(page) => return self.activate_page(page),

            Message::Search(search::Message::Activate) => {
                return self.search.focus();
            }

            Message::Search(search::Message::Changed(phrase)) => {
                self.search_changed(phrase);
            }

            Message::Search(search::Message::Clear) => {
                self.search_clear();
            }

            Message::Search(_) => {}

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
                crate::pages::Message::External { .. } => {
                    todo!("external plugins not supported yet");
                }
                crate::pages::Message::Page(page) => {
                    return self.activate_page(page);
                }
                crate::pages::Message::Panel(message) => {
                    page::update!(self.pages, message, panel::Page);
                }
                crate::pages::Message::Applet(message) => {
                    if let Some(page) = self.pages.page_mut::<applets::Page>() {
                        return page.update(message).map(cosmic::app::Message::App);
                    }
                }
            },

            Message::PanelConfig(config) if config.name.to_lowercase().contains("panel") => {
                if let Some(page) = self.pages.page_mut::<panel::Page>() {
                    page.update(panel::Message::PanelConfig(config.clone()));
                }
                if let Some(page) = self.pages.page_mut::<applets::Page>() {
                    _ = page.update(applets::Message::PanelConfig(config));
                }
            }

            Message::PanelConfig(_) => {} // ignore other config changes for now,

            Message::DesktopInfo => {
                if let Some(page) = self.pages.page_mut::<applets::Page>() {
                    // collect the potential applets
                    return page
                        .update(applets::Message::Applets(
                            freedesktop_desktop_entry::Iter::new(
                                freedesktop_desktop_entry::default_paths(),
                            )
                            .filter_map(|p| applets::Applet::try_from(Cow::from(p)).ok())
                            .collect(),
                        ))
                        .map(cosmic::app::Message::App);
                }
            }
        }

        Command::none()
    }

    #[allow(clippy::too_many_lines)]
    fn view(&self) -> Element<Message> {
        scrollable(row![
            horizontal_space(Length::Fill),
            (if self.search.is_active() {
                self.search_view()
            } else if let Some(content) = self.pages.content(self.active_page) {
                self.page_view(content)
            } else if let Some(sub_pages) = self.pages.sub_pages(self.active_page) {
                self.sub_page_view(sub_pages)
            } else {
                panic!("page without sub-pages or content");
            })
            .debug(self.core.debug),
            horizontal_space(Length::Fill),
        ])
        .apply(cosmic::Element::from)
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let search = search::search(&self.search, Message::Search);

        vec![search]
    }

    fn view_window(&self, id: window::Id) -> Element<Message> {
        if let Some(Some(page)) =
            (id == APPLET_DND_ICON_ID).then(|| self.pages.page::<applets::Page>())
        {
            return page.dnd_icon().map(Into::into);
        }

        if let Some(Some(page)) =
            (id == applets::ADD_APPLET_DIALOGUE_ID).then(|| self.pages.page::<applets::Page>())
        {
            return page.add_applet_view().map(Into::into);
        }

        panic!("no view for window {id:?}");
    }
}

impl SettingsApp {
    /// Activates a page.
    fn activate_page(&mut self, page: page::Entity) -> Command<crate::Message> {
        let current_page = self.active_page;
        self.active_page = page;

        if current_page != page {
            self.config.active_page = Box::from(&*self.pages.info[page].id);
            self.config
                .set_active_page(Box::from(&*self.pages.info[page].id));
        }

        self.search_clear();
        self.search.state = search::State::Inactive;
        self.activate_navbar(page);

        self.pages
            .page_reload(page)
            .unwrap_or(iced::Command::none())
            .map(Message::PageMessage)
            .map(cosmic::app::Message::App)
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
            .icon(IconSource::from(page.icon_name.clone()))
            .data(id)
            .with_id(|nav_id| self.pages.data_set(id, nav_id))
    }

    /// Displays the view of a page.
    fn page_view(&self, content: &[section::Entity]) -> cosmic::Element<Message> {
        let page = &self.pages.info[self.active_page];
        let mut column_widgets = Vec::with_capacity(1);

        if let Some(parent) = page.parent {
            column_widgets.push(parent_page_button(
                &self.pages.info[parent],
                page,
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

        settings::view_column(column_widgets)
            .max_width(683)
            .padding(0)
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
        let page = &self.pages.info[self.active_page];

        let mut column_widgets = Vec::with_capacity(sub_pages.len());
        column_widgets.push(page_title(page));

        for entity in sub_pages.iter().copied() {
            let sub_page = &self.pages.info[entity];
            column_widgets.push(sub_page_button(entity, sub_page));
        }

        settings::view_column(column_widgets)
            .apply(cosmic::Element::from)
            .map(Message::Page)
    }
}
