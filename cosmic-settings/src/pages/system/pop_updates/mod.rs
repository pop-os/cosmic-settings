// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod system_updates;

use cosmic::Task;
use cosmic_settings_page as page;
use pop_system_updater::Frequency;
use pop_system_updater::client::ClientProxy;
use pop_system_updater::client::notification::ClientProxy as NotificationClientProxy;

#[derive(Clone, Debug)]
pub enum Message {
    SetAutomaticUpdates(bool),
    SetNotificationFrequency(Option<Frequency>),
    Surface(cosmic::surface::Action),
    SystemUpdateClient(ClientProxy<'static>, NotificationClientProxy<'static>),
    SystemUpdateConfigs(
        pop_system_updater::Config,
        pop_system_updater::NotificationConfig,
    ),
    Error(String),
}

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
    system_updates: system_updates::Model,
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::SetAutomaticUpdates(enable) => {
                self.system_updates.config.auto_update = enable;

                if let Some(mut client) =
                    self.system_updates.clients.as_ref().map(|(c, _)| c.clone())
                {
                    tokio::task::spawn(async move {
                        _ = client.auto_update_set(enable).await;
                    });
                }
            }

            Message::SetNotificationFrequency(frequency) => {
                match frequency {
                    Some(frequency) => {
                        self.system_updates.notification_config.enabled = true;
                        self.system_updates
                            .notification_config
                            .notification_frequency = frequency;
                    }
                    None => self.system_updates.notification_config.enabled = false,
                }

                if let Some(mut client) =
                    self.system_updates.clients.as_ref().map(|(_, c)| c.clone())
                {
                    tokio::task::spawn(async move {
                        match frequency {
                            Some(frequency) => {
                                _ = client.set_notification_frequency(frequency).await;
                                _ = client.notifications_enabled(true).await;
                            }
                            None => {
                                _ = client.notifications_enabled(false).await;
                            }
                        }
                    });
                }
            }

            Message::SystemUpdateClient(system, session) => {
                self.system_updates.clients = Some((system, session))
            }

            Message::SystemUpdateConfigs(config, notification_config) => {
                self.system_updates.config = config;
                self.system_updates.notification_config = notification_config;
            }

            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }

            Message::Error(why) => {
                eprintln!("system update page error: {why}");
            }
        }

        Task::none()
    }
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::SystemUpdater(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::SystemUpdater(message)
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> cosmic_settings_page::Info {
        page::Info::new("os-updates", "software-update-available-symbolic").title(fl!("os-updates"))
    }

    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<
            page::section::Entity,
            page::Section<crate::pages::Message>,
        >,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(system_updates::section())])
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        Task::batch([
            Task::future(async { system_updates::load_clients().await }),
            Task::future(async { system_updates::load_configs().await }),
        ])
    }
}
