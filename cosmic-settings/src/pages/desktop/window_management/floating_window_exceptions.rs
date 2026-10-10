// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::app::{ContextDrawer, context_drawer};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, settings};
use cosmic::{Apply, Element, Task};
use cosmic_client_toolkit as cctk;
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_config::window_rules::{self, PreciseApplicationException};
use cosmic_settings_page::{self as page, Section, section};
use regex::Regex;
use slotmap::SlotMap;
use std::collections::HashSet;
use tracing::error;

#[derive(Clone, Debug)]
pub enum Message {
    DeleteCustomExceptionAt(usize),
    OpenAddCustomException,
    OpenBuiltInExceptions,
    RunningWindowsLoaded(Vec<RunningWindow>),
    SelectRunningWindow(usize),
    SetBuiltInExceptionEnabled(usize, bool),
}

#[derive(Clone, Copy, Debug)]
enum ContextView {
    AddCustomException,
    BuiltInExceptions,
}

#[derive(Clone, Debug)]
pub struct RunningWindow {
    app_id: String,
    title: String,
}

pub struct Page {
    entity: page::Entity,
    context_view: Option<ContextView>,
    window_rules_config: Option<cosmic_config::Config>,
    built_in_exceptions: Vec<PreciseApplicationException>,
    custom_exceptions: Vec<PreciseApplicationException>,
    running_windows: Option<Vec<RunningWindow>>,
}

impl Default for Page {
    fn default() -> Self {
        let window_rules_config = window_rules::context()
            .inspect_err(|err| error!(?err, "Failed to load window rules config"))
            .ok();
        let built_in_exceptions = window_rules_config
            .as_ref()
            .map(load_built_in_exceptions)
            .unwrap_or_default();
        let custom_exceptions = window_rules_config
            .as_ref()
            .map(|config| load_custom_exceptions(config, &built_in_exceptions))
            .unwrap_or_default();

        Self {
            entity: page::Entity::default(),
            context_view: None,
            window_rules_config,
            built_in_exceptions,
            custom_exceptions,
            running_windows: None,
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::app::Message> {
        match message {
            Message::DeleteCustomExceptionAt(index) => {
                self.delete_custom_exception_at(index);
                cosmic::Task::none()
            }
            Message::OpenAddCustomException => {
                self.running_windows = None;
                self.context_view = Some(ContextView::AddCustomException);
                cosmic::task::message(crate::app::Message::OpenContextDrawer(self.entity)).chain(
                    Task::future(async {
                        let windows = std::thread::spawn(load_running_windows)
                            .join()
                            .unwrap_or_default();
                        crate::app::Message::PageMessage(
                            crate::pages::Message::FloatingWindowExceptions(
                                Message::RunningWindowsLoaded(windows),
                            ),
                        )
                    }),
                )
            }
            Message::OpenBuiltInExceptions => {
                self.context_view = Some(ContextView::BuiltInExceptions);
                cosmic::task::message(crate::app::Message::OpenContextDrawer(self.entity))
            }
            Message::RunningWindowsLoaded(windows) => {
                self.running_windows = Some(windows);
                cosmic::Task::none()
            }
            Message::SelectRunningWindow(index) => {
                if self.save_running_window_exception(index) {
                    cosmic::task::message(crate::pages::Message::CloseContextDrawer)
                } else {
                    cosmic::Task::none()
                }
            }
            Message::SetBuiltInExceptionEnabled(index, enabled) => {
                self.set_built_in_exception_enabled(index, enabled);
                cosmic::Task::none()
            }
        }
    }

    fn reload_built_in_exceptions(&mut self) {
        if self.window_rules_config.is_none() {
            self.window_rules_config = window_rules::context()
                .inspect_err(|err| error!(?err, "Failed to load window rules config"))
                .ok();
        }

        self.built_in_exceptions = self
            .window_rules_config
            .as_ref()
            .map(load_built_in_exceptions)
            .unwrap_or_default();
        self.custom_exceptions = self
            .window_rules_config
            .as_ref()
            .map(|config| load_custom_exceptions(config, &self.built_in_exceptions))
            .unwrap_or_default();
    }

    fn set_built_in_exception_enabled(&mut self, index: usize, enabled: bool) {
        let Some(exception) = self.built_in_exceptions.get(index).cloned() else {
            return;
        };
        let Some(config) = self.window_rules_config.as_ref() else {
            return;
        };

        let mut custom = config
            .get::<Vec<PreciseApplicationException>>("tiling_exception_custom")
            .unwrap_or_default();

        if enabled {
            custom.retain(|custom_exception| {
                custom_exception.appid != exception.appid
                    || custom_exception.title != exception.title
            });
        } else if let Some(existing) = custom
            .iter_mut()
            .find(|existing| existing.appid == exception.appid && existing.title == exception.title)
        {
            existing.enabled = false;
        } else {
            custom.push(PreciseApplicationException {
                appid: exception.appid,
                title: exception.title,
                enabled: false,
            });
        }

        if let Err(err) = config.set("tiling_exception_custom", custom) {
            error!(?err, "Failed to set config tiling_exception_custom");
            return;
        }

        self.reload_built_in_exceptions();
    }

    fn delete_custom_exception_at(&mut self, index: usize) {
        let Some(exception) = self.custom_exceptions.get(index).cloned() else {
            return;
        };
        let Some(config) = self.window_rules_config.as_ref() else {
            return;
        };

        let mut custom = config
            .get::<Vec<PreciseApplicationException>>("tiling_exception_custom")
            .unwrap_or_default();
        custom.retain(|custom_exception| {
            custom_exception.appid != exception.appid || custom_exception.title != exception.title
        });

        if let Err(err) = config.set("tiling_exception_custom", custom) {
            error!(?err, "Failed to set config tiling_exception_custom");
            return;
        }

        self.reload_built_in_exceptions();
    }

    fn save_running_window_exception(&mut self, index: usize) -> bool {
        let Some(window) = self
            .running_windows
            .as_ref()
            .and_then(|windows| windows.get(index))
        else {
            return false;
        };
        let Some(config) = self.window_rules_config.as_ref() else {
            return false;
        };

        let appid = regex::escape(window.app_id.trim());
        let title = regex::escape(window.title.trim());
        let new_exception = PreciseApplicationException {
            appid,
            title,
            enabled: true,
        };

        let mut custom = config
            .get::<Vec<PreciseApplicationException>>("tiling_exception_custom")
            .unwrap_or_default();

        if is_built_in_exception(&new_exception, &self.built_in_exceptions) {
            custom.retain(|exception| {
                exception.appid != new_exception.appid || exception.title != new_exception.title
            });
        } else if !custom.iter().any(|exception| {
            exception.appid == new_exception.appid && exception.title == new_exception.title
        }) {
            custom.push(new_exception);
        }

        if let Err(err) = config.set("tiling_exception_custom", custom) {
            error!(?err, "Failed to set config tiling_exception_custom");
            return false;
        }

        self.reload_built_in_exceptions();
        true
    }

    fn running_windows_drawer(&self) -> Element<'_, crate::pages::Message> {
        let mut section = settings::section().title("");

        if let Some(running_windows) = &self.running_windows {
            let available_windows = running_windows
                .iter()
                .enumerate()
                .filter(|(_, window)| !self.window_has_enabled_exception(window))
                .collect::<Vec<_>>();

            if available_windows.is_empty() {
                section = section.add(widget::text::body(fl!(
                    "floating-window-exceptions",
                    "running-empty"
                )));
            } else {
                for (index, window) in available_windows {
                    let title = if window.title.is_empty() {
                        &window.app_id
                    } else {
                        &window.title
                    };
                    section = section.add(
                        widget::list::button(
                            settings::item::builder(title)
                                .description(&window.app_id)
                                .control(widget::icon::from_name("list-add-symbolic")),
                        )
                        .on_press(Message::SelectRunningWindow(index)),
                    );
                }
            }
        }

        section
            .apply(Element::from)
            .map(crate::pages::Message::FloatingWindowExceptions)
    }

    fn window_has_enabled_exception(&self, window: &RunningWindow) -> bool {
        self.built_in_exceptions
            .iter()
            .chain(self.custom_exceptions.iter())
            .any(|exception| exception.enabled && window_matches_exception(window, exception))
    }

    fn built_in_exceptions_drawer(&self) -> Element<'_, crate::pages::Message> {
        let mut section = settings::section().title("");

        for (index, exception) in self.built_in_exceptions.iter().enumerate() {
            let description = if exception.title == ".*" {
                fl!("floating-window-exceptions", "all-titles")
            } else {
                format!(
                    "{}: {}",
                    fl!("floating-window-exceptions", "title-pattern"),
                    exception.title
                )
            };

            section = section.add(
                settings::item::builder(&exception.appid)
                    .description(description)
                    .toggler(exception.enabled, move |enabled| {
                        Message::SetBuiltInExceptionEnabled(index, enabled)
                    }),
            );
        }

        widget::column::with_capacity(2)
            .push(widget::text::body(fl!(
                "floating-window-exceptions",
                "built-in-description"
            )))
            .push(section)
            .apply(Element::from)
            .map(crate::pages::Message::FloatingWindowExceptions)
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        self.reload_built_in_exceptions();
        cosmic::Task::none()
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(custom_exceptions()),
            sections.insert(built_in_exceptions()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new(
            "floating-window-exceptions",
            "preferences-window-management-symbolic",
        )
        .title(fl!("floating-window-exceptions"))
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        Some(match self.context_view? {
            ContextView::AddCustomException => context_drawer(
                self.running_windows_drawer(),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("floating-window-exceptions", "add-custom")),
            ContextView::BuiltInExceptions => context_drawer(
                self.built_in_exceptions_drawer(),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("floating-window-exceptions", "built-in")),
        })
    }

    fn on_context_drawer_close(&mut self) -> cosmic::Task<crate::pages::Message> {
        self.context_view = None;
        cosmic::Task::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn built_in_exceptions() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        title = fl!("floating-window-exceptions", "built-in");
        description = fl!("floating-window-exceptions", "built-in-description");
    });

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(
                    widget::list::button(
                        settings::item::builder(&descriptions[title])
                            .description(&descriptions[description])
                            .control(widget::icon::from_name("go-next-symbolic")),
                    )
                    .on_press(Message::OpenBuiltInExceptions),
                )
                .apply(Element::from)
                .map(crate::pages::Message::FloatingWindowExceptions)
        })
}

fn custom_exceptions() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        description = fl!("floating-window-exceptions", "select-description");
        select = fl!("floating-window-exceptions", "select");
    });

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            let mut content = widget::column::with_capacity(3)
                .spacing(24)
                .align_x(Alignment::Center)
                .push(
                    widget::text::body(&descriptions[description])
                        .width(Length::Fill)
                        .align_x(Alignment::Center),
                )
                .push(
                    widget::button::suggested(&descriptions[select])
                        .on_press(Message::OpenAddCustomException),
                );

            if !page.custom_exceptions.is_empty() {
                let mut settings_section = settings::section();
                for (index, exception) in page.custom_exceptions.iter().enumerate() {
                    let name = custom_exception_name(exception);
                    settings_section = settings_section.add(
                        settings::item::builder(name).control(
                            widget::button::icon(widget::icon::from_name("edit-delete-symbolic"))
                                .extra_small()
                                .on_press(Message::DeleteCustomExceptionAt(index)),
                        ),
                    );
                }
                content = content.push(settings_section);
            }

            content
                .apply(Element::from)
                .map(crate::pages::Message::FloatingWindowExceptions)
        })
}

fn custom_exception_name(exception: &PreciseApplicationException) -> String {
    if exception.title == ".*" || exception.title.is_empty() {
        exception.appid.clone()
    } else {
        exception.title.clone()
    }
}

fn load_built_in_exceptions(config: &cosmic_config::Config) -> Vec<PreciseApplicationException> {
    let defaults = config
        .get::<Vec<window_rules::DefaultApplicationException>>("tiling_exception_defaults")
        .unwrap_or_else(|err| {
            error!(?err, "Failed to read config tiling_exception_defaults");
            Vec::new()
        });
    let custom = config
        .get::<Vec<PreciseApplicationException>>("tiling_exception_custom")
        .unwrap_or_default();

    let mut exceptions = defaults
        .into_iter()
        .flat_map(window_rules::DefaultApplicationException::expand)
        .collect::<Vec<_>>();

    for custom_exception in custom {
        if let Some(exception) = exceptions.iter_mut().find(|exception| {
            exception.appid == custom_exception.appid && exception.title == custom_exception.title
        }) {
            exception.enabled = custom_exception.enabled;
        }
    }

    exceptions
}

fn load_custom_exceptions(
    config: &cosmic_config::Config,
    built_in_exceptions: &[PreciseApplicationException],
) -> Vec<PreciseApplicationException> {
    config
        .get::<Vec<PreciseApplicationException>>("tiling_exception_custom")
        .unwrap_or_default()
        .into_iter()
        .filter(|exception| !is_built_in_exception(exception, built_in_exceptions))
        .collect()
}

fn is_built_in_exception(
    exception: &PreciseApplicationException,
    built_in_exceptions: &[PreciseApplicationException],
) -> bool {
    built_in_exceptions
        .iter()
        .any(|built_in| built_in.appid == exception.appid && built_in.title == exception.title)
}

fn regex_matches(pattern: &str, value: &str) -> bool {
    Regex::new(pattern.trim()).is_ok_and(|regex| regex.is_match(value))
}

fn window_matches_exception(
    window: &RunningWindow,
    exception: &PreciseApplicationException,
) -> bool {
    regex_matches(&exception.appid, &window.app_id)
        && regex_matches(&exception.title, &window.title)
}

fn load_running_windows() -> Vec<RunningWindow> {
    use cctk::sctk::{
        output::{OutputHandler, OutputState},
        registry::{ProvidesRegistryState, RegistryState},
    };
    use cctk::toplevel_info::{ToplevelInfoHandler, ToplevelInfoState};
    use cctk::wayland_client::{
        Connection, QueueHandle, globals::registry_queue_init, protocol::wl_output,
    };
    use cctk::wayland_protocols::ext::foreign_toplevel_list::v1::client::ext_foreign_toplevel_handle_v1;

    struct AppData {
        output_state: OutputState,
        registry_state: RegistryState,
        toplevel_info_state: ToplevelInfoState,
        done: bool,
    }

    impl AppData {
        fn windows(&self) -> Vec<RunningWindow> {
            let mut seen = HashSet::new();
            let mut windows = self
                .toplevel_info_state
                .toplevels()
                .filter(|info| !info.app_id.is_empty() || !info.title.is_empty())
                .filter_map(|info| {
                    let app_id = info.app_id.trim().to_string();
                    let title = info.title.trim().to_string();
                    if seen.insert((app_id.clone(), title.clone())) {
                        Some(RunningWindow { app_id, title })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            windows.sort_by(|a, b| a.app_id.cmp(&b.app_id).then_with(|| a.title.cmp(&b.title)));
            windows
        }
    }

    impl ProvidesRegistryState for AppData {
        fn registry(&mut self) -> &mut RegistryState {
            &mut self.registry_state
        }

        cctk::sctk::registry_handlers!(OutputState);
    }

    impl OutputHandler for AppData {
        fn output_state(&mut self) -> &mut OutputState {
            &mut self.output_state
        }

        fn new_output(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _output: wl_output::WlOutput,
        ) {
        }

        fn update_output(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _output: wl_output::WlOutput,
        ) {
        }

        fn output_destroyed(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _output: wl_output::WlOutput,
        ) {
        }
    }

    impl ToplevelInfoHandler for AppData {
        fn toplevel_info_state(&mut self) -> &mut ToplevelInfoState {
            &mut self.toplevel_info_state
        }

        fn new_toplevel(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _toplevel: &ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1,
        ) {
        }

        fn update_toplevel(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _toplevel: &ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1,
        ) {
        }

        fn toplevel_closed(
            &mut self,
            _conn: &Connection,
            _qh: &QueueHandle<Self>,
            _toplevel: &ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1,
        ) {
        }

        fn info_done(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>) {
            self.done = true;
        }
    }

    cctk::sctk::delegate_output!(AppData);
    cctk::sctk::delegate_registry!(AppData);
    cctk::delegate_toplevel_info!(AppData);

    let Ok(conn) = Connection::connect_to_env() else {
        return Vec::new();
    };
    let Ok((globals, mut event_queue)) = registry_queue_init(&conn) else {
        return Vec::new();
    };
    let qh = event_queue.handle();
    let registry_state = RegistryState::new(&globals);
    let Some(toplevel_info_state) = ToplevelInfoState::try_new(&registry_state, &qh) else {
        return Vec::new();
    };

    let mut app_data = AppData {
        output_state: OutputState::new(&globals, &qh),
        registry_state,
        toplevel_info_state,
        done: false,
    };

    while !app_data.done {
        if let Err(err) = event_queue.blocking_dispatch(&mut app_data) {
            error!(?err, "Failed to read running windows");
            break;
        }
    }

    app_data.windows()
}
