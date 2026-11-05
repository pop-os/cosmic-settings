// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;

use super::Page;
use cosmic::iced::alignment::Vertical;
use cosmic::{Apply, Element, surface, widget};
use cosmic_settings_page::{self as page, Section};
use pop_system_updater_zbus::client::ClientProxy;
use pop_system_updater_zbus::client::notification::ClientProxy as NotificationClientProxy;
use pop_system_updater_zbus::{Frequency, Interval, Schedule, client};

#[derive(Clone, Debug)]
pub enum Message {
    SetAutomaticUpdates(bool),
    SetNotificationFrequency(Option<Frequency>),
    SetSchedule(Schedule),
    /// Handling of internal messages
    Surface(surface::Action<crate::Message>),
    SystemUpdateClient(ClientProxy<'static>, NotificationClientProxy<'static>),
    SystemUpdateConfigs(
        pop_system_updater_zbus::Config,
        pop_system_updater_zbus::Local,
    ),
    Error(String),
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Updates(super::Message::SystemUpdates(message))
    }
}

impl From<Message> for crate::Message {
    fn from(message: Message) -> Self {
        crate::Message::PageMessage(crate::pages::Message::from(message))
    }
}

#[derive(Default)]
pub(super) struct Model {
    pub clients: Option<(
        client::ClientProxy<'static>,
        client::notification::ClientProxy<'static>,
    )>,
    pub notification_config: pop_system_updater_zbus::Local,
    pub config: pop_system_updater_zbus::Config,
    pub frequency_labels: Vec<String>,
    pub interval_labels: Vec<String>,
    pub time_labels: Vec<String>,
    pub hour_label: String,
    pub minute_label: String,
    pub is_pm: usize,
    pub hour: u8,
    pub minute: u8,
}

impl Model {
    pub fn update_labels(&mut self) {
        self.time_labels = vec![crate::fl!("time-am"), crate::fl!("time-pm")];

        self.frequency_labels = vec![
            crate::fl!("disabled"),
            crate::fl!("frequency-daily"),
            crate::fl!("frequency-weekly"),
            crate::fl!("frequency-monthly"),
        ];

        self.interval_labels = vec![
            crate::fl!("time-monday"),
            crate::fl!("time-tuesday"),
            crate::fl!("time-wednesday"),
            crate::fl!("time-thursday"),
            crate::fl!("time-friday"),
            crate::fl!("time-saturday"),
            crate::fl!("time-sunday"),
            crate::fl!("time-weekdays"),
        ];
    }

    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::Message> {
        match message {
            Message::SetSchedule(schedule) => {
                self.apply_schedule(&schedule);
                self.config.schedule = Some(schedule.clone());

                if let Some((client, _)) = self.clients.as_ref() {
                    let mut client = client.clone();
                    tokio::task::spawn(async move {
                        _ = client.update_scheduling_set(schedule).await;
                    });
                }
            }

            Message::SetAutomaticUpdates(enable) => {
                self.config.auto_update = enable;

                if let Some((client, _)) = self.clients.as_ref() {
                    let mut client = client.clone();
                    tokio::task::spawn(async move {
                        _ = client.auto_update_set(enable).await;
                    });
                }
            }

            Message::SetNotificationFrequency(frequency) => {
                match frequency {
                    Some(frequency) => {
                        self.notification_config.enabled = true;
                        self.notification_config.notification_frequency = frequency;
                    }
                    None => self.notification_config.enabled = false,
                }

                if let Some((_, client)) = self.clients.as_ref() {
                    let mut client = client.clone();
                    tokio::task::spawn(async move {
                        match frequency {
                            Some(frequency) => {
                                _ = client.notifications_enabled(true).await;
                                _ = client.set_notification_frequency(frequency).await;
                            }

                            None => {
                                _ = client.notifications_enabled(false).await;
                            }
                        }
                    });
                }
            }

            Message::SystemUpdateClient(client_proxy, notification_proxy) => {
                self.clients = Some((client_proxy, notification_proxy));
            }

            Message::SystemUpdateConfigs(config, notification_config) => {
                if let Some(ref schedule) = config.schedule {
                    self.apply_schedule(schedule);
                }
                self.config = config;
                self.notification_config = notification_config;
            }

            Message::Error(why) => {
                tracing::error!(?why, "updates page");
            }

            Message::Surface(a) => {
                return cosmic::task::message(crate::Message::Surface(a));
            }
        }

        cosmic::Task::none()
    }

    #[inline(never)]
    fn apply_schedule(&mut self, schedule: &Schedule) {
        fn as_12(hour: u8) -> (u8, bool) {
            if hour == 0 {
                (12, false)
            } else if hour < 12 {
                (hour, false)
            } else if hour == 12 {
                (12, true)
            } else {
                (hour - 12, true)
            }
        }

        let (hour, is_pm) = match as_12(schedule.hour) {
            (hour, false) => (hour, 0),
            (hour, true) => (hour, 1),
        };

        self.hour_label = format!("{:02}", hour);
        self.hour = hour;
        self.minute_label = format!("{:02}", schedule.minute);
        self.minute = schedule.minute;
        self.is_pm = is_pm;
    }
}

pub fn section() -> page::Section<crate::pages::Message> {
    crate::slab!(descriptions {
        _header_text = fl!("system-and-app-updates");
        automatic_updates_text = fl!("system-and-app-updates", "automatic-toggle");
        notifications_text = fl!("system-and-app-updates", "notifications-toggle");
        schedule_text = fl!("system-and-app-updates", "schedule");
    });

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let desc = &section.descriptions;

            let notification_dropdown = widget::dropdown::popup_dropdown(
                &page.system_updates.frequency_labels,
                Some(if !page.system_updates.notification_config.enabled {
                    0
                } else {
                    match page
                        .system_updates
                        .notification_config
                        .notification_frequency
                    {
                        Frequency::Daily => 1,
                        Frequency::Weekly => 2,
                        Frequency::Monthly => 3,
                    }
                }),
                |selected| {
                    crate::pages::Message::from(Message::SetNotificationFrequency(match selected {
                        1 => Some(Frequency::Daily),
                        2 => Some(Frequency::Weekly),
                        3 => Some(Frequency::Monthly),
                        _ => None,
                    }))
                },
                cosmic::iced::window::Id::RESERVED,
                |a| Message::Surface(a).into(),
                crate::Message::from,
            );

            let update_notification_frequency =
                widget::settings::item::builder(&desc[notifications_text])
                    .control(notification_dropdown);

            let update_schedule = page.system_updates.config.auto_update.then_some({
                let schedule = page
                    .system_updates
                    .config
                    .schedule
                    .clone()
                    .unwrap_or_else(pop_system_updater_zbus::Config::default_schedule);

                let hour = page.system_updates.hour;
                let minute = page.system_updates.minute;
                let is_pm = page.system_updates.is_pm;

                let interval_pos = match schedule.interval {
                    Interval::Monday => 0,
                    Interval::Tuesday => 1,
                    Interval::Wednesday => 2,
                    Interval::Thursday => 3,
                    Interval::Friday => 4,
                    Interval::Saturday => 5,
                    Interval::Sunday => 6,
                    Interval::Weekdays => 7,
                };

                let schedule = Arc::new(schedule);

                let interval = cosmic::widget::dropdown::popup_dropdown(
                    &page.system_updates.interval_labels,
                    Some(interval_pos),
                    {
                        let schedule = schedule.clone();
                        move |pos| {
                            let mut schedule = schedule.as_ref().clone();
                            schedule.interval = match pos {
                                0 => Interval::Monday,
                                1 => Interval::Tuesday,
                                2 => Interval::Wednesday,
                                3 => Interval::Thursday,
                                4 => Interval::Friday,
                                5 => Interval::Saturday,
                                6 => Interval::Sunday,
                                7 => Interval::Weekdays,
                                _ => unreachable!("unknown interval index"),
                            };

                            crate::pages::Message::from(Message::SetSchedule(schedule))
                        }
                    },
                    cosmic::iced::window::Id::RESERVED,
                    |a| Message::Surface(a).into(),
                    crate::Message::from,
                );

                let hour = cosmic::widget::spin_button::vertical(
                    &page.system_updates.hour_label,
                    crate::fl!("time-hour"),
                    hour,
                    1,
                    0,
                    13,
                    {
                        let schedule = schedule.clone();
                        move |mut v: u8| {
                            let mut schedule = schedule.as_ref().clone();
                            if v != hour {
                                if is_pm == 1 {
                                    if v == 0 {
                                        v = 12;
                                    } else if hour == 12 && v == 11 {
                                        v = 11
                                    } else if hour == 11 && v == 12 {
                                        v = 0;
                                    } else if v != 13 {
                                        v += 12;
                                    }
                                } else if hour == 12 && v == 11 {
                                    v = 23;
                                } else if v == 13 {
                                    v = 1;
                                }

                                schedule.hour = v
                            }

                            Message::SetSchedule(schedule).into()
                        }
                    },
                );

                let minute = cosmic::widget::spin_button::vertical(
                    &page.system_updates.minute_label,
                    crate::fl!("time-minute"),
                    minute,
                    1,
                    0,
                    59,
                    {
                        let schedule = schedule.clone();
                        move |v| {
                            let mut schedule = schedule.as_ref().clone();
                            schedule.minute = v;
                            Message::SetSchedule(schedule).into()
                        }
                    },
                );

                let time_of_day = cosmic::widget::dropdown::popup_dropdown(
                    &page.system_updates.time_labels,
                    Some(is_pm),
                    {
                        let schedule = schedule.clone();
                        move |pos| {
                            let mut schedule = schedule.as_ref().clone();
                            if pos != is_pm {
                                if pos == 1 {
                                    schedule.hour += 12
                                } else {
                                    schedule.hour -= 12;
                                }
                            }

                            Message::SetSchedule(schedule).into()
                        }
                    },
                    cosmic::iced::window::Id::RESERVED,
                    |a| Message::Surface(a).into(),
                    crate::Message::from,
                );

                let control = widget::row([
                    interval.into(),
                    hour.into(),
                    minute.into(),
                    time_of_day.into(),
                ])
                .align_y(Vertical::Center);

                cosmic::widget::settings::item::builder(&desc[schedule_text]).control(control)
            });

            let automatic_updates_toggle =
                widget::settings::item::builder(&desc[automatic_updates_text])
                    .toggler(page.system_updates.config.auto_update, |enable| {
                        Message::SetAutomaticUpdates(enable).into()
                    });

            widget::settings::section()
                .add(update_notification_frequency)
                .add(automatic_updates_toggle)
                .add_maybe(update_schedule)
                .apply(Element::from)
        })
}

pub async fn load_clients() -> crate::pages::Message {
    let system = zbus::Connection::system();
    let session = zbus::Connection::session();

    let (system, session) = match futures::try_join!(system, session) {
        Ok(v) => v,
        Err(why) => return Message::Error(format!("DBus connection failed: {why}")).into(),
    };

    let system_client = client::ClientProxy::new(&system);
    let session_client = client::notification::ClientProxy::new(&session);

    match futures::try_join!(system_client, session_client) {
        Ok((system_client, session_client)) => {
            Message::SystemUpdateClient(system_client, session_client).into()
        }
        Err(why) => Message::Error(format!("DBus proxy connection failed: {why}")).into(),
    }
}

pub async fn load_configs() -> crate::pages::Message {
    let (config, notification_config) = futures::join!(
        pop_system_updater_zbus::config::load_system(),
        pop_system_updater_zbus::config::load_session(),
    );
    Message::SystemUpdateConfigs(config, notification_config).into()
}
