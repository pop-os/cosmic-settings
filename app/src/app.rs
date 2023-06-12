// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use apply::Apply;

use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::{self as page, section};

use cosmic::{
    cosmic_config::config_subscription,
    iced::{
        self,
        event::wayland::{self, WindowEvent, WindowState},
        event::PlatformSpecific,
        subscription, window, Application, Color, Command, Length, Subscription,
    },
    iced::{
        widget::{self, column, container, horizontal_space, row},
        window::Mode,
    },
    iced_sctk::commands::window::{set_mode_window, start_drag_window},
    iced_style::application,
    keyboard_nav,
    theme::{self, theme_subscription, Theme},
    widget::{
        header_bar, nav_bar, nav_bar_toggle, scrollable, search, segmented_button, settings,
        IconSource,
    },
    Element, ElementExt,
};

use crate::{
    config::Config,
    pages::{
        desktop::{
            self,
            panel::{
                self,
                applets::{self, APPLET_DND_ICON_ID},
            },
        },
        sound, system, time,
    },
    subscription::desktop_files,
    widget::{page_title, parent_page_button, search_header, sub_page_button},
};

use std::{borrow::Cow, process};

#[allow(clippy::struct_excessive_bools)]
#[allow(clippy::module_name_repetitions)]
pub struct SettingsApp {
    pub active_page: page::Entity,
    pub config: Config,
    pub debug: bool,
    pub is_condensed: bool,
    pub nav_bar_toggled_condensed: bool,
    pub nav_bar_toggled: bool,
    pub nav_bar: segmented_button::SingleSelectModel,
    pub pages: page::Binder<crate::pages::Message>,
    pub scaling_factor: f32,
    pub search: search::Model,
    pub search_selections: Vec<(page::Entity, section::Entity)>,
    pub show_maximize: bool,
    pub sharp_corners: bool,
    pub show_minimize: bool,
    pub theme: Theme,
    pub title: String,
    pub window_width: u32,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Message {
    Close,
    Drag,
    KeyboardNav(keyboard_nav::Message),
    Maximize,
    Minimize,
    NavBar(segmented_button::Entity),
    None,
    Page(page::Entity),
    PageMessage(crate::pages::Message),
    Search(search::Message),
    ToggleNavBar,
    ToggleNavBarCondensed,
    WindowResize(u32, u32),
    WindowState(WindowState),
    PanelConfig(CosmicPanelConfig),
    DesktopInfo,
    ThemeChanged(Theme),
}

impl Application for SettingsApp {
    type Executor = cosmic::executor::single::Executor;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = SettingsApp {
            sharp_corners: false,
            active_page: page::Entity::default(),
            config: Config::new(),
            debug: false,
            is_condensed: false,
            nav_bar: segmented_button::Model::default(),
            nav_bar_toggled: true,
            nav_bar_toggled_condensed: false,
            pages: page::Binder::default(),
            title: crate::fl!("app"),
            scaling_factor: std::env::var("COSMIC_SCALE")
                .ok()
                .and_then(|scale| scale.parse::<f32>().ok())
                .unwrap_or(1.0),
            search: search::Model::default(),
            search_selections: Vec::default(),
            show_maximize: true,
            show_minimize: true,
            window_width: 0,
            theme: cosmic::theme::theme(),
        };

        // app.insert_page::<wifi::Page>();
        // app.insert_page::<networking::Page>();
        // app.insert_page::<bluetooth::Page>();

        let desktop_id = app.insert_page::<desktop::Page>().id();
        // app.insert_page::<panel::Page>();
        // app.insert_page::<applets::Page>();

        // app.insert_page::<input::Page>();

        // app.insert_page::<displays::Page>();
        // app.insert_page::<power::Page>();

        app.insert_page::<sound::Page>();

        // app.insert_page::<printers::Page>();
        // app.insert_page::<privacy::Page>();

        app.insert_page::<system::Page>();
        app.insert_page::<time::Page>();

        // app.insert_page::<accessibility::Page>();
        // app.insert_page::<applications::Page>();

        let active_id = app
            .pages
            .info
            .iter()
            .find(|(_id, info)| info.id == *app.config.active_page)
            .map_or(desktop_id, |(id, _info)| id);

        let command = app.activate_page(active_id);

        (app, command)
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        let window_break = subscription::events_with(|event, _| match event {
            iced::Event::Window(_window_id, window::Event::Resized { width, height }) => {
                Some(Message::WindowResize(width, height))
            }
            iced::Event::PlatformSpecific(PlatformSpecific::Wayland(wayland::Event::Window(
                WindowEvent::State(s),
                ..,
            ))) => Some(Message::WindowState(s)),
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

        Subscription::batch(vec![
            window_break,
            keyboard_nav::subscription().map(Message::KeyboardNav),
            desktop_files(0).map(|_| Message::DesktopInfo),
            config_subscription(0, "com.system76.CosmicPanel.panel".into(), 1).map(
                |(_, e)| match e {
                    Ok(config) => Message::PanelConfig(config),
                    Err((errors, config)) => {
                        for error in errors {
                            log::error!("Error loading panel config: {:?}", error);
                        }
                        Message::PanelConfig(config)
                    }
                },
            ),
            theme_subscription(0).map(Message::ThemeChanged),
        ])
    }

    #[allow(clippy::too_many_lines)]
    fn update(&mut self, message: Message) -> iced::Command<Self::Message> {
        let mut ret = Command::none();
        match message {
            Message::WindowResize(width, _height) => {
                let break_point = (600.0 * self.scaling_factor) as u32;
                self.window_width = width;
                self.is_condensed = self.window_width < break_point;
            }
            Message::KeyboardNav(message) => match message {
                keyboard_nav::Message::Unfocus => ret = keyboard_nav::unfocus(),
                keyboard_nav::Message::FocusNext => ret = widget::focus_next(),
                keyboard_nav::Message::FocusPrevious => ret = widget::focus_previous(),
                keyboard_nav::Message::Escape => {
                    if self.search.is_active() {
                        self.search.state = search::State::Inactive;
                        self.search_clear();
                    }
                }
                keyboard_nav::Message::Search => {
                    return self.search.focus();
                }
            },
            Message::Page(page) => return self.activate_page(page),
            Message::Drag => return start_drag_window(window::Id(0)),
            Message::Close => {
                process::exit(0);
            }
            Message::Minimize => return set_mode_window(window::Id(0), Mode::Hidden),
            Message::Maximize => {
                if self.sharp_corners {
                    self.sharp_corners = false;
                    return set_mode_window(window::Id(0), Mode::Windowed);
                }

                self.sharp_corners = true;
                return set_mode_window(window::Id(0), Mode::Fullscreen);
            }
            Message::NavBar(key) => {
                if let Some(page) = self.nav_bar.data::<page::Entity>(key).copied() {
                    return self.activate_page(page);
                }
            }
            Message::ToggleNavBar => self.nav_bar_toggled = !self.nav_bar_toggled,
            Message::ToggleNavBarCondensed => {
                self.nav_bar_toggled_condensed = !self.nav_bar_toggled_condensed;
            }
            Message::Search(search::Message::Activate) => {
                return self.search.focus();
            }
            Message::Search(search::Message::Changed(phrase)) => {
                self.search_changed(phrase);
            }
            Message::Search(search::Message::Clear) => {
                self.search_clear();
            }
            Message::None | Message::Search(_) => {}
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
                    if let Some(page) = self.pages.page_mut::<panel::Page>() {
                        page.update(message);
                    }
                }
                crate::pages::Message::Applet(message) => {
                    if let Some(page) = self.pages.page_mut::<applets::Page>() {
                        ret = page.update(message);
                    }
                }
            },
            Message::WindowState(state) => {
                self.sharp_corners = matches!(state, WindowState::Activated);
            }
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
                    ret = page.update(applets::Message::Applets(
                        freedesktop_desktop_entry::Iter::new(
                            freedesktop_desktop_entry::default_paths(),
                        )
                        .filter_map(|p| applets::Applet::try_from(Cow::from(p)).ok())
                        .collect(),
                    ));
                }
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
        }
        ret
    }

    #[allow(clippy::too_many_lines)]
    fn view(&self, id: window::Id) -> Element<Message> {
        if let Some(Some(page)) =
            (id == APPLET_DND_ICON_ID).then(|| self.pages.page::<applets::Page>())
        {
            return page.dnd_icon();
        }
        if let Some(Some(page)) =
            (id == applets::ADD_APPLET_DIALOGUE_ID).then(|| self.pages.page::<applets::Page>())
        {
            return page.add_applet_view();
        }

        let (nav_bar_message, nav_bar_toggled) = if self.is_condensed {
            (
                Message::ToggleNavBarCondensed,
                self.nav_bar_toggled_condensed,
            )
        } else {
            (Message::ToggleNavBar, self.nav_bar_toggled)
        };

        let mut header = header_bar()
            .title("")
            .on_close(Message::Close)
            .on_drag(Message::Drag)
            .start(
                iced::widget::row!(
                    nav_bar_toggle()
                        .on_nav_bar_toggled(nav_bar_message)
                        .nav_bar_active(nav_bar_toggled),
                    search::search(&self.search, Message::Search)
                )
                .align_items(iced::Alignment::Center)
                .into(),
            );

        if self.show_maximize {
            header = header.on_maximize(Message::Maximize);
        }

        if self.show_minimize {
            header = header.on_minimize(Message::Minimize);
        }

        let header = Into::<Element<Message>>::into(header).debug(self.debug);

        let mut widgets = Vec::with_capacity(2);

        if nav_bar_toggled {
            let mut nav_bar = nav_bar(&self.nav_bar, Message::NavBar);

            if !self.is_condensed {
                nav_bar = nav_bar.max_width(300);
            }

            let nav_bar: Element<_> = nav_bar.into();
            widgets.push(nav_bar.debug(self.debug));
        }

        if !(self.is_condensed && nav_bar_toggled) {
            widgets.push(
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
                    .debug(self.debug),
                    horizontal_space(Length::Fill),
                ])
                .into(),
            );
        }

        let content = container(row(widgets).spacing(8))
            .padding([0, 8, 8, 8])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Container::Background)
            .into();

        column(vec![header, content]).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn scale_factor(&self) -> f64 {
        self.scaling_factor as f64
    }

    fn close_requested(&self, id: window::Id) -> Self::Message {
        if id == window::Id(0) {
            Message::Close
        } else if id == applets::ADD_APPLET_DIALOGUE_ID {
            Message::PageMessage(crate::pages::Message::Applet(
                applets::Message::ClosedAppletDialogue,
            ))
        } else {
            Message::None
        }
    }

    fn style(&self) -> <Self::Theme as cosmic::iced_style::application::StyleSheet>::Style {
        if self.sharp_corners {
            cosmic::theme::Application::default()
        } else {
            cosmic::theme::Application::Custom(Box::new(|theme| application::Appearance {
                background_color: Color::TRANSPARENT,
                text_color: theme.cosmic().on_bg_color().into(),
            }))
        }
    }
}

impl SettingsApp {
    /// Activates a page.
    fn activate_page(&mut self, page: page::Entity) -> Command<crate::Message> {
        self.nav_bar_toggled_condensed = false;
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
            .unwrap_or(Command::none())
            .map(Message::PageMessage)
    }

    /// Activates the navbar item associated with a page.
    fn activate_navbar(&mut self, mut page: page::Entity) {
        if let Some(parent) = self.pages.info[page].parent {
            page = parent;
        }

        if let Some(nav_id) = self.pages.data(page) {
            self.nav_bar.activate(*nav_id);
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

        self.nav_bar
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
            .apply(Element::from)
            .map(Message::Page)
    }
}
