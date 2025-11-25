// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::app::ContextDrawer;
use cosmic::iced::{Alignment, Length, Subscription};
use cosmic::widget::{button, icon, row, settings, text};
use cosmic::{Apply, Task};
use cosmic_settings_page::{self as page, Section, section};
use slotmap::{Key, SlotMap};
use std::time::Duration;

use super::app_details;
use super::{FlatpakApp, StorageInfo, format_bytes, loading_spinner};

#[derive(Clone, Debug)]
pub enum Message {
    LoadApps(Vec<FlatpakApp>),
    LoadAppsWithSizes(Vec<FlatpakApp>),
    SetApps(Vec<FlatpakApp>),
    SelectApp(String),
    AnimationTick,
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::StorageApplicationsCategory(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::StorageApplicationsCategory(message)
    }
}

#[derive(Clone, Debug)]
pub struct Page {
    entity: page::Entity,
    flatpak_apps: Vec<FlatpakApp>,
    app_details_page: page::Entity,
    animation_state: u8,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            entity: page::Entity::null(),
            flatpak_apps: Vec::new(),
            app_details_page: page::Entity::null(),
            animation_state: 0,
        }
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: page::Insert<crate::pages::Message>,
    ) -> page::Insert<crate::pages::Message> {
        let app_details = page.sub_page_with_id::<app_details::Page>();

        let model = page.model.page_mut::<Page>().unwrap();
        model.app_details_page = app_details;

        page
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(flatpak_apps())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("storage-applications", "application-default-symbolic")
            .title(fl!("storage-category-apps"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        // Data is managed by parent storage page, just display what we have
        Task::none()
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        None
    }

    fn subscription(&self, _core: &cosmic::Core) -> Subscription<crate::pages::Message> {
        // Animate while any app is loading
        if self.flatpak_apps.iter().any(|app| app.loading) {
            cosmic::iced::time::every(Duration::from_millis(500))
                .map(|_| crate::pages::Message::StorageApplicationsCategory(Message::AnimationTick))
        } else {
            Subscription::none()
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::app::Task<crate::Message> {
        match message {
            Message::LoadApps(apps) => {
                let apps_to_load = apps.clone();
                self.flatpak_apps = apps;

                // Trigger background task to calculate sizes
                return cosmic::Task::future(async move {
                    Message::LoadAppsWithSizes(StorageInfo::load_flatpak_apps_with_sizes(
                        apps_to_load,
                    ))
                })
                .map(crate::app::Message::from)
                .map(Into::into);
            }
            Message::LoadAppsWithSizes(apps_with_sizes) | Message::SetApps(apps_with_sizes) => {
                self.flatpak_apps = apps_with_sizes;
            }
            Message::SelectApp(app_id) => {
                // Find the app and send it to the app details page
                if let Some(app) = self
                    .flatpak_apps
                    .iter()
                    .find(|a| a.app_id == app_id)
                    .cloned()
                {
                    let load_app_task = cosmic::task::message(crate::app::Message::from(
                        crate::pages::Message::StorageAppDetails(
                            app_details::Message::LoadAppDetails(app),
                        ),
                    ));

                    let navigate_task =
                        cosmic::task::message(crate::app::Message::Page(self.app_details_page));

                    return cosmic::Task::batch(vec![load_app_task, navigate_task]);
                }
            }
            Message::AnimationTick => {
                self.animation_state = (self.animation_state + 1) % 3;
            }
        }

        Task::none()
    }
}

fn flatpak_apps() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("storage-flatpak-apps"))
        .view::<Page>(move |_binder, page, section| {
            let mut section_widget = settings::section().title(&section.title);
            let animation_state = page.animation_state;

            if page.flatpak_apps.is_empty() {
                section_widget = section_widget.add(settings::item(
                    fl!("storage-flatpak-apps-none"),
                    text::caption(fl!("storage-flatpak-apps-none-desc")),
                ));
            } else {
                for app in &page.flatpak_apps {
                    let app_id = app.app_id.clone();

                    // Show spinner if still loading, otherwise show size
                    let size_element: cosmic::Element<Message> = if app.loading {
                        loading_spinner(animation_state)
                    } else {
                        text::body(format_bytes(app.total_size())).into()
                    };

                    // Create a compact row similar to flex_item but clickable
                    let app_row = row::with_capacity(4)
                        .spacing(12)
                        .align_y(Alignment::Center)
                        .push(icon::from_name(&*app.icon).size(24))
                        .push(text::body(&app.name).width(Length::Fill))
                        .push(size_element)
                        .push(icon::from_name("go-next-symbolic").size(16));

                    let app_button = button::custom(app_row)
                        .padding([12, 16])
                        .on_press(Message::SelectApp(app_id))
                        .width(Length::Fill)
                        .class(cosmic::theme::Button::MenuItem);

                    section_widget = section_widget.add(app_button);
                }
            }

            section_widget
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::StorageApplicationsCategory)
        })
}
