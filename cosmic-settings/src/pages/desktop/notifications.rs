// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! User configuration for `cosmic-notifications`.

use cosmic::{
    Apply, Element, Task,
    cosmic_config::Config,
    widget::{self, settings},
};
use cosmic_notifications_config::{Anchor, NotificationsConfig};
use cosmic_settings_page::{self as page, AutoBind, Content, Info, Section, section};
use slotmap::SlotMap;
use tracing::{debug, error, instrument, warn};

use crate::{app, pages, set_max_timeout};

mod helpers;
use helpers::{anchor_to_pos, load_config};

pub struct Page {
    entity: page::Entity,
    config_helper: Option<Config>,
    config: NotificationsConfig,

    // Appearance UI helpers
    anchor_dropdown: [String; 8],

    // Timeout UI helpers
    max_notif: String,
    max_per_app: String,
    max_timeout_urgent: String,
    max_timeout_normal: String,
    max_timeout_low: String,
}

impl Page {
    /// Reload [`NotificationsConfig`] if it exists.
    #[inline]
    fn refresh(&mut self) {
        self.config = load_config(self.config_helper.as_ref());
    }

    /// View for notification appearance config (e.g. anchor position and others).
    fn appearance_view() -> Section<pages::Message> {
        crate::slab!(descriptions {
            anchor = fl!("notifications", "anchor");
            anchor_desc = fl!("notifications", "anchor-desc");
        });

        Section::default()
            .title(fl!("notifications", "appearance"))
            .descriptions(descriptions)
            .view::<Page>(move |_binder, page, section| {
                // XXX: Anchor is trivially copyable but cosmic-notifications doesn't derive Copy.
                let anchor_choice = Some(anchor_to_pos(page.config.anchor.clone()));

                settings::section()
                    .title(&*section.title)
                    .add(
                        settings::item::builder(&*section.descriptions[anchor])
                            .description(&*section.descriptions[anchor_desc])
                            .control(widget::dropdown(
                                &page.anchor_dropdown,
                                anchor_choice,
                                Message::Anchor,
                            )),
                    )
                    .apply(Element::from)
                    .map(pages::Message::from)
            })
    }

    /// View for notification timeout config (e.g. maximum timeout).
    fn timeout_view() -> Section<pages::Message> {
        crate::slab!(descriptions {
            max_notif = fl!("notifications", "max");
            max_notif_desc = fl!("notifications", "max-desc");
            max_per_app = fl!("notifications", "max-per-app");
            max_per_app_desc = fl!("notifications", "max-per-app-desc");
            max_timeout_urgent = fl!("notifications", "max-timeout-urgent");
            max_timeout_urgent_desc = fl!("notifications", "max-timeout-urgent-desc");
            max_timeout_normal = fl!("notifications", "max-timeout-normal");
            max_timeout_normal_desc = fl!("notifications", "max-timeout-normal-desc");
            max_timeout_low = fl!("notifications", "max-timeout-low");
            max_timeout_low_desc = fl!("notifications", "max-timeout-low-desc");
        });

        Section::default()
            .title(fl!("notifications", "timeout"))
            .descriptions(descriptions)
            .view::<Page>(move |_binder, page, section| {
                settings::section()
                    .title(&*section.title)
                    .add(
                        settings::item::builder(&*section.descriptions[max_notif])
                            .description(&*section.descriptions[max_notif_desc])
                            .control(
                                widget::text_input("", &page.max_notif)
                                    .on_input(Message::MaxNotifications),
                            ),
                    )
                    .add(
                        settings::item::builder(&*section.descriptions[max_per_app])
                            .description(&*section.descriptions[max_per_app_desc])
                            .control(
                                widget::text_input("", &page.max_per_app)
                                    .on_input(Message::MaxPerApp),
                            ),
                    )
                    .add(
                        settings::item::builder(&*section.descriptions[max_timeout_urgent])
                            .description(&*section.descriptions[max_timeout_urgent_desc])
                            .control(
                                widget::text_input("", &page.max_timeout_urgent)
                                    .on_input(Message::MaxTimeoutUrgent),
                            ),
                    )
                    .add(
                        settings::item::builder(&*section.descriptions[max_timeout_normal])
                            .description(&*section.descriptions[max_timeout_normal_desc])
                            .control(
                                widget::text_input("", &page.max_timeout_normal)
                                    .on_input(Message::MaxTimeoutNormal),
                            ),
                    )
                    .add(
                        settings::item::builder(&*section.descriptions[max_timeout_low])
                            .description(&*section.descriptions[max_timeout_low_desc])
                            .control(
                                widget::text_input("", &page.max_timeout_low)
                                    .on_input(Message::MaxTimeoutLow),
                            ),
                    )
                    .apply(Element::from)
                    .map(pages::Message::from)
            })
    }

    // View for per app notification settings.
    // pub fn per_app_view(&self) -> Element<'static, pages::Message> {
    //     unimplemented!()
    // }

    #[instrument(skip(self), fields(id = %cosmic_notifications_config::ID))]
    pub fn update(&mut self, message: Message) -> Task<app::Message> {
        match message {
            Message::Anchor(i) => {
                let anchor = match i {
                    0 => Anchor::Top,
                    1 => Anchor::Bottom,
                    2 => Anchor::Right,
                    3 => Anchor::Left,
                    4 => Anchor::TopLeft,
                    5 => Anchor::TopRight,
                    6 => Anchor::BottomLeft,
                    7 => Anchor::BottomRight,
                    n => unreachable!("Dropdown for 'anchor' returned an out of bounds value: {n}"),
                };

                if let Some(helper) = self.config_helper.as_ref() {
                    if let Err(e) = self.config.set_anchor(helper, anchor) {
                        error!("Failed to set new anchor position: {e}");
                    }
                } else {
                    warn!("Unable to set new anchor position due to missing config helper");
                }
            }
            Message::MaxNotifications(s) => {
                set_max_timeout!(
                    self,
                    "max_notifications",
                    max_notifications,
                    set_max_notifications,
                    s
                );
                self.max_notif = s;
            }
            Message::MaxPerApp(s) => {
                set_max_timeout!(self, "max_per_app", max_per_app, set_max_per_app, s);
                self.max_per_app = s;
            }
            Message::MaxTimeoutUrgent(s) => {
                set_max_timeout!(
                    self,
                    "max_timeout_urgent",
                    max_timeout_urgent,
                    set_max_timeout_urgent,
                    s
                );
                self.max_timeout_urgent = s;
            }
            Message::MaxTimeoutLow(s) => {
                set_max_timeout!(
                    self,
                    "max_timeout_low",
                    max_timeout_low,
                    set_max_timeout_low,
                    s
                );
                self.max_timeout_low = s;
            }
            Message::MaxTimeoutNormal(s) => {
                set_max_timeout!(
                    self,
                    "max_timeout_normal",
                    max_timeout_normal,
                    set_max_timeout_normal,
                    s
                );
                self.max_timeout_normal = s;
            }
        }

        Task::none()
    }
}

impl Default for Page {
    fn default() -> Self {
        debug!(id = %cosmic_notifications_config::ID, "Loading Notifications config for the first time this instance");

        let config_helper = Config::new(cosmic_notifications_config::ID, 1).ok();
        let config = load_config(config_helper.as_ref());
        let max_notif = config.max_notifications.to_string();
        let max_per_app = config.max_per_app.to_string();
        let max_timeout_urgent = config
            .max_timeout_urgent
            .map(|i| i.to_string())
            .unwrap_or_default();
        let max_timeout_normal = config
            .max_timeout_normal
            .map(|i| i.to_string())
            .unwrap_or_default();
        let max_timeout_low = config
            .max_timeout_low
            .map(|i| i.to_string())
            .unwrap_or_default();

        Self {
            entity: Default::default(),
            config_helper,
            config,
            anchor_dropdown: [
                fl!("notifications", "anchor-top"),
                fl!("notifications", "anchor-bottom"),
                fl!("notifications", "anchor-right"),
                fl!("notifications", "anchor-left"),
                fl!("notifications", "anchor-top-left"),
                fl!("notifications", "anchor-top-right"),
                fl!("notifications", "anchor-bottom-left"),
                fl!("notifications", "anchor-bottom-right"),
            ],
            max_notif,
            max_per_app,
            max_timeout_urgent,
            max_timeout_normal,
            max_timeout_low,
        }
    }
}

impl page::Page<pages::Message> for Page {
    fn info(&self) -> Info {
        Info::new("notifications", "notification-symbolic")
            .title(fl!("notifications"))
            .description(fl!("notifications", "desc"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<pages::Message>>,
    ) -> Option<Content> {
        Some(vec![
            sections.insert(Self::appearance_view()),
            sections.insert(Self::timeout_view()),
        ])
    }

    fn on_enter(&mut self) -> Task<pages::Message> {
        self.refresh();
        Task::none()
    }

    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }
}

impl AutoBind<pages::Message> for Page {}

/// Notification [`Page`] message.
#[derive(Clone, Debug)]
pub enum Message {
    Anchor(usize),
    MaxNotifications(String),
    MaxPerApp(String),
    MaxTimeoutUrgent(String),
    MaxTimeoutNormal(String),
    MaxTimeoutLow(String),
}

impl From<Message> for pages::Message {
    fn from(message: Message) -> Self {
        pages::Message::Notifications(message)
    }
}
