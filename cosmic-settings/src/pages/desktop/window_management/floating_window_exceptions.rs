// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::app::{ContextDrawer, context_drawer};
use cosmic::widget::{self, settings};
use cosmic::{Apply, Element};
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_config::window_rules::{self, PreciseApplicationException};
use cosmic_settings_page::{self as page, Section, section};
use slotmap::SlotMap;
use tracing::error;

#[derive(Clone, Debug)]
pub enum Message {
    OpenBuiltInExceptions,
    SetBuiltInExceptionEnabled(usize, bool),
}

#[derive(Clone, Copy, Debug)]
enum ContextView {
    BuiltInExceptions,
}

pub struct Page {
    entity: page::Entity,
    context_view: Option<ContextView>,
    window_rules_config: Option<cosmic_config::Config>,
    built_in_exceptions: Vec<PreciseApplicationException>,
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

        Self {
            entity: page::Entity::default(),
            context_view: None,
            window_rules_config,
            built_in_exceptions,
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::app::Message> {
        match message {
            Message::OpenBuiltInExceptions => {
                self.context_view = Some(ContextView::BuiltInExceptions);
                cosmic::task::message(crate::app::Message::OpenContextDrawer(self.entity))
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
        Some(vec![sections.insert(built_in_exceptions())])
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
