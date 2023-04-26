// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use apply::Apply;

use cosmic_settings_page::{self as page, section};

use cosmic::{
    iced::widget::{self, column, container, horizontal_space, row},
    iced::{self, Application, Command, Length, Subscription},
    iced_native::{subscription, window},
    iced_winit::window::{close, drag, minimize, toggle_maximize},
    keyboard_nav,
    theme::Theme,
    widget::{
        header_bar, nav_bar, nav_bar_toggle, scrollable, search, segmented_button, settings,
        IconSource,
    },
    Element, ElementExt,
};

use crate::{
    config::{self, Config},
    pages::{desktop, sound, system, time},
    widget::{page_title, parent_page_button, search_header, sub_page_button},
};

#[allow(clippy::struct_excessive_bools)]
#[allow(clippy::module_name_repetitions)]
pub struct SettingsApp {
    pub active_page: page::Entity,
    pub config: Config,
    pub config_path: config::PathManager,
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
}

impl Application for SettingsApp {
    type Executor = cosmic::executor::single::Executor;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut config_path = config::PathManager::new();

        let mut app = SettingsApp {
            active_page: page::Entity::default(),
            config: config_path.config("main", Config::deserialize),
            config_path,
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
            theme: Theme::Dark,
            window_width: 0,
        };

        // app.insert_page::<wifi::Page>();
        // app.insert_page::<networking::Page>();
        // app.insert_page::<bluetooth::Page>();

        let desktop_id = app.insert_page::<desktop::Page>().id();

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
            _ => None,
        });

        Subscription::batch(vec![
            window_break,
            keyboard_nav::subscription().map(Message::KeyboardNav),
        ])
    }

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
            Message::Drag => return drag(window::Id::new(0)),
            Message::Close => return close(window::Id::new(0)),
            Message::Minimize => return minimize(window::Id::new(0), true),
            Message::Maximize => return toggle_maximize(window::Id::new(0)),
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
                    if let Some(page) = self.pages.page_mut::<system::about::Page>() {
                        page.update(message);
                    }
                }
                crate::pages::Message::DateAndTime(message) => {
                    if let Some(page) = self.pages.page_mut::<time::date::Page>() {
                        page.update(message);
                    }
                }
                crate::pages::Message::Desktop(message) => {
                    if let Some(page) = self.pages.page_mut::<desktop::Page>() {
                        page.update(message);
                    }
                }
                crate::pages::Message::External { .. } => {
                    todo!("external plugins not supported yet");
                }
            },
        }
        ret
    }

    #[allow(clippy::too_many_lines)]
    fn view(&self) -> Element<Message> {
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
                    } else if let Some(sub_pages) = self.pages.sub_pages(self.active_page) {
                        self.sub_page_view(sub_pages)
                    } else if let Some(content) = self.pages.content(self.active_page) {
                        self.page_view(content)
                    } else {
                        panic!("page without sub-pages or content");
                    })
                    .debug(self.debug),
                    horizontal_space(Length::Fill),
                ])
                .into(),
            );
        }

        let content = container(row(widgets))
            .padding([0, 8, 8, 8])
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        column(vec![header, content]).into()
    }

    fn theme(&self) -> Theme {
        self.theme
    }

    fn scale_factor(&self) -> f64 {
        self.scaling_factor as f64
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
            self.config_path
                .config("main", |path| self.config.serialize(path));
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

        settings::view_column(column_widgets).into()
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
