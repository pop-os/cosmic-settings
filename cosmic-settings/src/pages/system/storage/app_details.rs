// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::{Alignment, Subscription};
use cosmic::widget::{column, icon, row, settings, text};
use cosmic::{Apply, Task};
use cosmic_settings_page::{self as page, Section, section};
use slotmap::SlotMap;
use std::time::Duration;

use super::{FlatpakApp, format_bytes, loading_spinner};

#[derive(Clone, Debug)]
pub enum Message {
    LoadAppDetails(FlatpakApp),
    AnimationTick,
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::StorageAppDetails(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::StorageAppDetails(message)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    entity: page::Entity,
    app: Option<FlatpakApp>,
    animation_state: u8,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(app_details())])
    }

    fn info(&self) -> page::Info {
        let title = self
            .app
            .as_ref()
            .map(|a| a.name.clone())
            .unwrap_or_else(|| fl!("storage-app-details"));

        page::Info::new("storage-app-details", "application-default-symbolic").title(title)
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        Task::none()
    }

    fn context_drawer(
        &self,
    ) -> Option<cosmic::app::context_drawer::ContextDrawer<'_, crate::pages::Message>> {
        None
    }

    fn subscription(&self, _core: &cosmic::Core) -> Subscription<crate::pages::Message> {
        if self.app.as_ref().map(|a| a.loading).unwrap_or(false) {
            cosmic::iced::time::every(Duration::from_millis(500))
                .map(|_| crate::pages::Message::StorageAppDetails(Message::AnimationTick))
        } else {
            Subscription::none()
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::app::Task<crate::Message> {
        match message {
            Message::LoadAppDetails(app) => {
                self.app = Some(app);
            }
            Message::AnimationTick => {
                self.animation_state = (self.animation_state + 1) % 3;
            }
        }

        Task::none()
    }

    pub fn set_app(&mut self, app: FlatpakApp) {
        self.app = Some(app);
    }
}

fn app_details() -> Section<crate::pages::Message> {
    Section::default().view::<Page>(move |_binder, page, _section| {
        let animation_state = page.animation_state;
        let content = if let Some(app) = &page.app {
            let mut column_widget = column::with_capacity(4).spacing(16).padding(16);

            // Row 1: App icon on left, app name/version/developer on right
            let mut info_column = column::with_capacity(3)
                .spacing(4)
                .push(text::heading(&app.name));

            if app.loading {
                info_column = info_column.push(text::caption("Loading..."));
            } else {
                if !app.version.is_empty() {
                    info_column = info_column.push(text::body(&app.version));
                }
                if !app.developer.is_empty() {
                    info_column = info_column.push(text::caption(&app.developer));
                }
            }

            let header_row = row::with_capacity(2)
                .spacing(16)
                .align_y(Alignment::Center)
                .push(icon::from_name(&*app.icon).size(64))
                .push(info_column);

            column_widget = column_widget.push(header_row);

            let mut size_section = settings::section().title(fl!("storage-app-size-details"));

            // Row 2: App Size
            let installed_row = if app.loading {
                settings::flex_item(
                    fl!("storage-app-installed"),
                    loading_spinner(animation_state),
                )
            } else {
                settings::flex_item(
                    fl!("storage-app-installed"),
                    text::body(format_bytes(app.installed_size)),
                )
            };
            size_section = size_section.add(installed_row);

            // Row 3: Data & Config
            let data_row = if app.loading {
                settings::flex_item(fl!("storage-app-data"), loading_spinner(animation_state))
            } else {
                settings::flex_item(
                    fl!("storage-app-data"),
                    text::body(format_bytes(app.data_size)),
                )
            };
            size_section = size_section.add(data_row);

            // Row 4: Total Size
            let total_row = if app.loading {
                settings::flex_item(fl!("storage-app-total"), loading_spinner(animation_state))
            } else {
                settings::flex_item(
                    fl!("storage-app-total"),
                    text::body(format_bytes(app.total_size())),
                )
            };
            size_section = size_section.add(total_row);

            column_widget = column_widget.push(size_section);

            column_widget.apply(cosmic::Element::from)
        } else {
            column::with_capacity(1)
                .padding(16)
                .push(text::body("Select an application"))
                .apply(cosmic::Element::from)
        };

        content.map(crate::pages::Message::StorageAppDetails)
    })
}
