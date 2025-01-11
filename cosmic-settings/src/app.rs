// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::config::Config;
#[cfg(feature = "page-bluetooth")]
use crate::pages::bluetooth;
use crate::pages::desktop::{self, appearance};
#[cfg(feature = "page-input")]
use crate::pages::input;
#[cfg(feature = "page-networking")]
use crate::pages::networking;
#[cfg(feature = "page-power")]
use crate::pages::power;
#[cfg(feature = "page-sound")]
use crate::pages::sound;
use crate::pages::{self, display, system, time};
use crate::subscription::desktop_files;
use crate::widget::{page_title, search_header};
use crate::PageCommands;
use cosmic::app::command::set_theme;
use cosmic::app::context_drawer::ContextDrawer;
#[cfg(feature = "single-instance")]
use cosmic::app::DbusActivationMessage;
#[cfg(feature = "wayland")]
use cosmic::cctk::{sctk::output::OutputInfo, wayland_client::protocol::wl_output::WlOutput};
use cosmic::iced::futures::SinkExt;
use cosmic::iced::{stream, Subscription};
use cosmic::widget::{self, button, row, text_input};
use cosmic::{
    app::{Core, Task},
    iced::{
        self,
        event::{self, PlatformSpecific},
        window, Length,
    },
    prelude::*,
    widget::{
        column, container, icon, id_container, nav_bar, scrollable, segmented_button, settings,
    },
    Element,
};
#[cfg(feature = "wayland")]
use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::{self as page, section};
#[cfg(feature = "wayland")]
use desktop::{
    dock,
    panel::{self, applets_inner, inner as _panel},
};
#[cfg(feature = "wayland")]
use event::wayland;
use page::Entity;
use std::collections::BTreeSet;
use std::{borrow::Cow, str::FromStr};

#[allow(clippy::struct_excessive_bools)]
#[allow(clippy::module_name_repetitions)]
pub struct SettingsApp {
    active_page: page::Entity,
    active_context_page: Option<page::Entity>,
    loaded_pages: BTreeSet<page::Entity>,
    config: Config,
    core: Core,
    nav_model: nav_bar::Model,
    page_sender: Option<tokio::sync::mpsc::Sender<crate::pages::Message>>,
    pages: page::Binder<crate::pages::Message>,
    search_active: bool,
    search_id: cosmic::widget::Id,
    search_input: String,
    search_selections: Vec<(page::Entity, section::Entity)>,
    context_title: Option<String>,
}

impl SettingsApp {
    fn subtask_to_page(&self, cmd: &PageCommands) -> Option<Entity> {
        match cmd {
            #[cfg(feature = "page-about")]
            PageCommands::About => self.pages.page_id::<system::about::Page>(),
            PageCommands::Appearance => self.pages.page_id::<desktop::appearance::Page>(),
            #[cfg(feature = "page-bluetooth")]
            PageCommands::Bluetooth => self.pages.page_id::<bluetooth::Page>(),
            #[cfg(feature = "page-date")]
            PageCommands::DateTime => self.pages.page_id::<time::date::Page>(),
            #[cfg(feature = "page-default-apps")]
            PageCommands::DefaultApps => self.pages.page_id::<system::default_apps::Page>(),
            PageCommands::Desktop => self.pages.page_id::<desktop::Page>(),
            PageCommands::Displays => self.pages.page_id::<display::Page>(),
            #[cfg(feature = "wayland")]
            PageCommands::Dock => self.pages.page_id::<desktop::dock::Page>(),
            PageCommands::Firmware => self.pages.page_id::<system::firmware::Page>(),
            #[cfg(feature = "page-input")]
            PageCommands::Input => self.pages.page_id::<input::Page>(),
            #[cfg(feature = "page-input")]
            PageCommands::Keyboard => self.pages.page_id::<input::keyboard::Page>(),
            #[cfg(feature = "page-input")]
            PageCommands::Mouse => self.pages.page_id::<input::mouse::Page>(),
            #[cfg(feature = "page-networking")]
            PageCommands::Network => self.pages.page_id::<networking::Page>(),
            #[cfg(feature = "wayland")]
            PageCommands::Panel => self.pages.page_id::<desktop::panel::Page>(),
            #[cfg(feature = "page-power")]
            PageCommands::Power => self.pages.page_id::<power::Page>(),
            #[cfg(feature = "page-region")]
            PageCommands::RegionLanguage => self.pages.page_id::<time::region::Page>(),
            #[cfg(feature = "page-sound")]
            PageCommands::Sound => self.pages.page_id::<sound::Page>(),
            PageCommands::System => self.pages.page_id::<system::Page>(),
            PageCommands::Time => self.pages.page_id::<time::Page>(),
            #[cfg(feature = "page-input")]
            PageCommands::Touchpad => self.pages.page_id::<input::touchpad::Page>(),
            #[cfg(feature = "page-users")]
            PageCommands::Users => self.pages.page_id::<system::users::Page>(),
            #[cfg(feature = "page-networking")]
            PageCommands::Vpn => self.pages.page_id::<networking::vpn::Page>(),
            PageCommands::Wallpaper => self.pages.page_id::<desktop::wallpaper::Page>(),
            #[cfg(feature = "page-window-management")]
            PageCommands::WindowManagement => {
                self.pages.page_id::<desktop::window_management::Page>()
            }
            #[cfg(feature = "page-networking")]
            PageCommands::Wired => self.pages.page_id::<networking::wired::Page>(),
            #[cfg(feature = "page-networking")]
            PageCommands::Wireless => self.pages.page_id::<networking::wifi::Page>(),
            #[cfg(feature = "page-workspaces")]
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
    #[cfg(feature = "wayland")]
    DesktopInfo,
    Error(String),
    None,
    OpenContextDrawer(Entity, Cow<'static, str>),
    #[cfg(feature = "wayland")]
    OutputAdded(OutputInfo, WlOutput),
    #[cfg(feature = "wayland")]
    OutputRemoved(WlOutput),
    Page(page::Entity),
    PageMessage(crate::pages::Message),
    #[cfg(feature = "wayland")]
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

    fn init(core: Core, flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut app = SettingsApp {
            active_page: page::Entity::default(),
            active_context_page: None,
            loaded_pages: BTreeSet::new(),
            config: Config::new(),
            core,
            nav_model: nav_bar::Model::default(),
            page_sender: None,
            pages: page::Binder::default(),
            search_active: false,
            search_id: cosmic::widget::Id::unique(),
            search_input: String::new(),
            search_selections: Vec::default(),
            context_title: None,
        };

        #[cfg(feature = "page-networking")]
        app.insert_page::<networking::Page>();
        #[cfg(feature = "page-bluetooth")]
        app.insert_page::<bluetooth::Page>();
        let desktop_id = app.insert_page::<desktop::Page>().id();
        app.insert_page::<display::Page>();
        #[cfg(feature = "page-sound")]
        app.insert_page::<sound::Page>();
        #[cfg(feature = "page-power")]
        app.insert_page::<power::Page>();
        #[cfg(feature = "page-input")]
        app.insert_page::<input::Page>();
        app.insert_page::<time::Page>();
        app.insert_page::<system::Page>();

        let active_id = match flags.sub_command {
            Some(p) => app.subtask_to_page(&p),
            None => app
                .pages
                .find_page_by_id(&app.config.active_page)
                .map(|(id, _info)| id),
        }
        .unwrap_or(desktop_id);

        (app, cosmic::task::message(Message::DelayedInit(active_id)))
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
                .padding(8)
                .on_press(Message::SearchActivate)
                .into()
        });

        widgets
    }

    fn on_app_exit(&mut self) -> Option<Self::Message> {
        self.pages.on_leave(self.active_page);
        None
    }

    fn on_escape(&mut self) -> Task<Self::Message> {
        if self.search_active {
            self.search_active = false;
            self.search_clear();
        }

        Task::none()
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
        if let Some(page) = self.nav_model.data::<page::Entity>(id).copied() {
            return self.activate_page(page);
        }

        Task::none()
    }

    fn on_search(&mut self) -> Task<Self::Message> {
        self.search_active = true;
        cosmic::widget::text_input::focus(self.search_id.clone())
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            // Creates a channel that listens to messages from pages.
            // The sender is given back to the application so that it may pass it on.
            Subscription::run_with_id(
                std::any::TypeId::of::<Self>(),
                stream::channel(4, move |mut output| async move {
                    let (tx, mut rx) = tokio::sync::mpsc::channel::<pages::Message>(4);

                    let _res = output.send(Message::RegisterSubscriptionSender(tx)).await;

                    while let Some(event) = rx.recv().await {
                        let _res = output.send(Message::PageMessage(event)).await;
                    }

                    futures::future::pending::<()>().await;
                }),
            ),
            #[cfg(feature = "ashpd")]
            crate::subscription::daytime().map(|daytime| {
                Message::PageMessage(pages::Message::Appearance(appearance::Message::Daytime(
                    daytime,
                )))
            }),
            #[cfg(feature = "wayland")]
            event::listen_with(|event, _, _id| match event {
                #[cfg(feature = "wayland")]
                iced::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::Output(wayland::OutputEvent::Created(Some(info)), o),
                )) if info.name.is_some() => Some(Message::OutputAdded(info, o)),
                #[cfg(feature = "wayland")]
                iced::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::Output(wayland::OutputEvent::Removed, o),
                )) => Some(Message::OutputRemoved(o)),
                _ => None,
            }),
            #[cfg(feature = "wayland")]
            // Watch for changes to installed desktop entries
            desktop_files(0).map(|_| Message::DesktopInfo),
            // Watch for configuration changes to the panel.
            #[cfg(feature = "wayland")]
            self.core()
                .watch_config::<CosmicPanelConfig>("com.system76.CosmicPanel.Panel")
                .map(|update| {
                    for why in update.errors {
                        tracing::error!(?why, "panel config load error");
                    }

                    Message::PanelConfig(update.config)
                }),
            #[cfg(feature = "wayland")]
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
    fn update(&mut self, message: Message) -> Task<Self::Message> {
        match message {
            Message::Page(page) => return self.activate_page(page),

            Message::None => (),

            Message::SetWindowTitle => return self.set_title(),

            Message::SearchChanged(phrase) => {
                return self.search_changed(phrase);
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
                #[cfg(feature = "page-about")]
                crate::pages::Message::About(message) => {
                    page::update!(self.pages, message, system::about::Page);
                }

                crate::pages::Message::Appearance(message) => {
                    if let Some(page) = self.pages.page_mut::<appearance::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-bluetooth")]
                crate::pages::Message::Bluetooth(message) => {
                    if let Some(page) = self.pages.page_mut::<bluetooth::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-date")]
                crate::pages::Message::DateAndTime(message) => {
                    if let Some(page) = self.pages.page_mut::<time::date::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-default-apps")]
                crate::pages::Message::DefaultApps(message) => {
                    if let Some(page) = self.pages.page_mut::<system::default_apps::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::Desktop(message) => {
                    page::update!(self.pages, message, desktop::Page);
                }

                crate::pages::Message::DesktopWallpaper(message) => {
                    if let Some(page) = self.pages.page_mut::<desktop::wallpaper::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-workspaces")]
                crate::pages::Message::DesktopWorkspaces(message) => {
                    page::update!(self.pages, message, desktop::workspaces::Page);
                }

                crate::pages::Message::Displays(message) => {
                    if let Some(page) = self.pages.page_mut::<display::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "wayland")]
                crate::pages::Message::Dock(message) => {
                    if let Some(page) = self.pages.page_mut::<dock::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "wayland")]
                crate::pages::Message::DockApplet(message) => {
                    if let Some(page) = self.pages.page_mut::<dock::applets::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-input")]
                crate::pages::Message::Input(message) => {
                    if let Some(page) = self.pages.page_mut::<input::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-input")]
                crate::pages::Message::Keyboard(message) => {
                    if let Some(page) = self.pages.page_mut::<input::keyboard::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-input")]
                crate::pages::Message::KeyboardShortcuts(message) => {
                    if let Some(page) = self.pages.page_mut::<input::keyboard::shortcuts::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-input")]
                crate::pages::Message::CustomShortcuts(message) => {
                    if let Some(page) = self
                        .pages
                        .page_mut::<input::keyboard::shortcuts::custom::Page>()
                    {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-input")]
                crate::pages::Message::ManageWindowShortcuts(message) => {
                    if let Some(page) = self
                        .pages
                        .page_mut::<input::keyboard::shortcuts::manage_windows::Page>()
                    {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-input")]
                crate::pages::Message::MoveWindowShortcuts(message) => {
                    if let Some(page) = self
                        .pages
                        .page_mut::<input::keyboard::shortcuts::move_window::Page>()
                    {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-input")]
                crate::pages::Message::NavShortcuts(message) => {
                    if let Some(page) = self
                        .pages
                        .page_mut::<input::keyboard::shortcuts::nav::Page>()
                    {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-region")]
                crate::pages::Message::Region(message) => {
                    if let Some(page) = self.pages.page_mut::<time::region::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-sound")]
                crate::pages::Message::Sound(message) => {
                    if let Some(page) = self.pages.page_mut::<sound::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-users")]
                crate::pages::Message::User(message) => {
                    if let Some(page) = self.pages.page_mut::<system::users::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-input")]
                crate::pages::Message::SystemShortcuts(message) => {
                    if let Some(page) = self
                        .pages
                        .page_mut::<input::keyboard::shortcuts::system::Page>()
                    {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-input")]
                crate::pages::Message::TilingShortcuts(message) => {
                    if let Some(page) = self
                        .pages
                        .page_mut::<input::keyboard::shortcuts::tiling::Page>()
                    {
                        return page.update(message).map(Into::into);
                    }
                }

                crate::pages::Message::External { .. } => {
                    todo!("external plugins not supported yet");
                }

                crate::pages::Message::Page(page) => {
                    return self.activate_page(page);
                }

                #[cfg(feature = "page-networking")]
                crate::pages::Message::Networking(message) => {
                    if let Some(page) = self.pages.page_mut::<networking::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "wayland")]
                crate::pages::Message::Panel(message) => {
                    if let Some(page) = self.pages.page_mut::<panel::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "wayland")]
                crate::pages::Message::PanelApplet(message) => {
                    if let Some(page) = self.pages.page_mut::<applets_inner::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-power")]
                crate::pages::Message::Power(message) => {
                    page::update!(self.pages, message, power::Page);
                }

                #[cfg(feature = "page-networking")]
                crate::pages::Message::Vpn(message) => {
                    if let Some(page) = self.pages.page_mut::<networking::vpn::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-networking")]
                crate::pages::Message::WiFi(message) => {
                    if let Some(page) = self.pages.page_mut::<networking::wifi::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }

                #[cfg(feature = "page-window-management")]
                crate::pages::Message::WindowManagement(message) => {
                    page::update!(self.pages, message, desktop::window_management::Page);
                }

                #[cfg(feature = "page-networking")]
                crate::pages::Message::Wired(message) => {
                    if let Some(page) = self.pages.page_mut::<networking::wired::Page>() {
                        return page.update(message).map(Into::into);
                    }
                }
            },

            #[cfg(feature = "wayland")]
            Message::OutputAdded(info, output) => {
                let mut commands = vec![];
                if let Some(page) = self.pages.page_mut::<panel::Page>() {
                    commands.push(
                        page.update(panel::Message(_panel::Message::OutputAdded(
                            info.name.clone().unwrap_or_default(),
                            output.clone(),
                        )))
                        .map(Into::into),
                    );
                }

                if let Some(page) = self.pages.page_mut::<dock::Page>() {
                    commands.push(
                        page.update(dock::Message::Inner(_panel::Message::OutputAdded(
                            info.name.unwrap_or_default(),
                            output,
                        )))
                        .map(Into::into),
                    );
                }
                return Task::batch(commands);
            }

            #[cfg(feature = "wayland")]
            Message::OutputRemoved(output) => {
                let mut commands = vec![];
                if let Some(page) = self.pages.page_mut::<panel::Page>() {
                    commands.push(
                        page.update(panel::Message(_panel::Message::OutputRemoved(
                            output.clone(),
                        )))
                        .map(Into::into),
                    );
                }

                if let Some(page) = self.pages.page_mut::<dock::Page>() {
                    commands.push(
                        page.update(dock::Message::Inner(_panel::Message::OutputRemoved(output)))
                            .map(Into::into),
                    );
                }
                return Task::batch(commands);
            }

            #[cfg(feature = "wayland")]
            Message::PanelConfig(config) if config.name.to_lowercase().contains("panel") => {
                let mut tasks = Vec::new();

                if let Some(page) = self.pages.page_mut::<panel::Page>() {
                    tasks.push(
                        page.update(panel::Message(_panel::Message::PanelConfig(config.clone())))
                            .map(Into::into),
                    );
                }

                if let Some(page) = self.pages.page_mut::<applets_inner::Page>() {
                    tasks.push(
                        page.update(applets_inner::Message::PanelConfig(config))
                            .map(Into::into),
                    );
                }

                return Task::batch(tasks);
            }

            #[cfg(feature = "wayland")]
            Message::PanelConfig(config) if config.name.to_lowercase().contains("dock") => {
                let mut tasks = Vec::new();

                if let Some(page) = self.pages.page_mut::<dock::Page>() {
                    tasks.push(
                        page.update(dock::Message::Inner(_panel::Message::PanelConfig(
                            config.clone(),
                        )))
                        .map(Into::into),
                    );
                }

                if let Some(page) = self.pages.page_mut::<dock::applets::Page>() {
                    tasks.push(
                        page.update(dock::applets::Message(applets_inner::Message::PanelConfig(
                            config,
                        )))
                        .map(Into::into),
                    );
                }

                return Task::batch(tasks);
            }

            #[cfg(feature = "wayland")]
            Message::PanelConfig(_) => {}
            #[cfg(feature = "wayland")]
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

            Message::SetTheme(t) => return set_theme(t),

            Message::OpenContextDrawer(page, title) => {
                self.core.window.show_context = true;
                self.active_context_page = Some(page);
                self.context_title = Some(title.to_string());
            }

            Message::CloseContextDrawer => {
                self.core.window.show_context = false;
                self.active_context_page = None;
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
                    return cosmic::task::message(Message::DelayedInit(active_id));
                }

                return self.activate_page(active_id);
            }
        }

        Task::none()
    }

    #[cfg(feature = "single-instance")]
    fn dbus_activation(&mut self, msg: DbusActivationMessage) -> Task<Self::Message> {
        match msg.msg {
            cosmic::app::DbusActivationDetails::Activate
            | cosmic::app::DbusActivationDetails::Open { .. } => None,
            cosmic::app::DbusActivationDetails::ActivateAction { action, .. } => {
                PageCommands::from_str(&action)
                    .ok()
                    .and_then(|action| self.subtask_to_page(&action))
                    .map(|e| self.activate_page(e))
            }
        }
        .unwrap_or_else(Task::none)
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
        panic!("unknown window ID: {id:?}");
    }

    fn context_drawer(&self) -> Option<ContextDrawer<Message>> {
        if self.core.window.show_context {
            self.active_context_page.and_then(|context_page| {
                self.pages.context_drawer(context_page).map(|cd| {
                    let cd = cosmic::app::context_drawer::context_drawer(
                        cd.map(Message::PageMessage),
                        Message::CloseContextDrawer,
                    );
                    if let Some(title) = self.context_title.as_ref() {
                        cd.title(title)
                    } else {
                        cd
                    }
                })
            })
        } else {
            None
        }
    }

    fn dialog(&self) -> Option<Element<Self::Message>> {
        self.pages
            .dialog(self.active_page)
            .map(|e| e.map(Message::PageMessage))
    }
}

impl SettingsApp {
    /// Activates a page.
    fn activate_page(&mut self, page: page::Entity) -> Task<crate::Message> {
        let current_page = self.active_page;
        self.active_page = page;

        let mut leave_task = iced::Task::none();

        if current_page != page {
            self.loaded_pages.remove(&current_page);
            leave_task = self
                .pages
                .on_leave(current_page)
                .unwrap_or(iced::Task::none())
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

        self.loaded_pages.insert(page);

        let page_task = self
            .pages
            .on_enter(page, sender)
            .map(Message::PageMessage)
            .map(Into::into);

        Task::batch(vec![
            leave_task,
            page_task,
            cosmic::task::future(async { Message::SetWindowTitle }),
        ])
    }

    fn set_title(&mut self) -> Task<crate::Message> {
        self.set_window_title(
            format!(
                "{} - COSMIC Settings",
                self.pages.info[self.active_page].title
            ),
            self.core.main_window_id().unwrap(),
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
                page.title().unwrap_or(page_info.title.as_str()),
                self.pages.info[parent].title.as_str(),
                Message::Page(parent),
            );

            let mut page_header_content: cosmic::iced_widget::Row<'_, Message, Theme> =
                row::with_capacity(2)
                    .align_y(iced::Alignment::End)
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
            .page_container(settings::view_column(sections_column))
            .apply(scrollable)
            .height(Length::Fill)
            .apply(|w| id_container(w, self.id()));

        widget::column::with_capacity(3)
            .push(self.page_container(header))
            .push(widget::vertical_space().height(Length::Fixed(
                cosmic::theme::active().cosmic().space_m().into(),
            )))
            .push(view)
            .height(Length::Fill)
            .into()
    }

    fn search_changed(&mut self, phrase: String) -> Task<crate::Message> {
        // If the text was cleared, clear the search results too.
        if phrase.is_empty() {
            self.search_clear();
            return Task::none();
        }

        let mut tasks = Vec::new();

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

                let mut unload = BTreeSet::new();
                let mut load = BTreeSet::new();

                'outer: for loaded_page in &self.loaded_pages {
                    for (page, _) in &self.search_selections {
                        if loaded_page == page {
                            continue 'outer;
                        }
                    }

                    unload.insert(*loaded_page);
                }

                for (page, _) in &self.search_selections {
                    if !self.loaded_pages.contains(page) {
                        load.insert(*page);
                    }
                }

                if let Some(ref sender) = self.page_sender {
                    for page in load {
                        self.loaded_pages.insert(page);
                        tasks.push(self.pages.on_enter(page, sender.clone()));
                    }
                }

                for page in unload {
                    self.loaded_pages.remove(&page);
                    self.pages.on_leave(page);
                }
            }
        }

        self.search_input = phrase;

        if tasks.is_empty() {
            Task::none()
        } else {
            cosmic::task::batch(tasks)
                .map(Message::PageMessage)
                .map(Into::into)
        }
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

        self.page_container(settings::view_column(sections))
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
                        "",
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
            .push(widget::vertical_space().height(theme.cosmic().space_m()))
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
            theme.cosmic().space_s()
        } else {
            theme.cosmic().space_l()
        };

        container(content.into())
            .max_width(800)
            .width(Length::Fill)
            .apply(container)
            .center_x(Length::Fill)
            .padding([0, padding])
            .into()
    }
}
