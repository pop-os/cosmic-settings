// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::widget;
use cosmic_settings_page::{self as page, Section, section};
use cosmic_settings_sound_subscription::{self as subscription, pipewire};
use itertools::Itertools;
use slotmap::SlotMap;

#[derive(Clone, Debug)]
pub enum Message {
    /// Messages handled by the sound module in cosmic-settings-subscriptions
    Subscription(subscription::Message),
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::SoundDeviceProfiles(message)
    }
}

impl From<Message> for crate::Message {
    fn from(message: Message) -> Self {
        crate::Message::PageMessage(message.into())
    }
}

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("sound-device-profiles", "preferences-sound-symbolic")
            .title(fl!("sound-device-profiles"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(view())])
    }

    fn set_id(&mut self, entity: cosmic_settings_page::Entity) {
        self.entity = entity;
    }

    fn subscription(
        &self,
        _core: &cosmic::Core,
    ) -> cosmic::iced::Subscription<crate::pages::Message> {
        cosmic::iced::Subscription::run(subscription::watch)
            .map(|message| super::Message::Subscription(message).into())
    }
}

impl Page {
    pub fn update(&mut self, _message: Message) -> cosmic::Task<crate::app::Message> {
        cosmic::Task::none()
    }
}

pub fn view() -> Section<crate::pages::Message> {
    Section::default().view::<Page>(move |binder, _page, _section| {
        let sound_page_id = binder.find_page_by_id("sound").unwrap().0;
        let sound_page = binder.page[sound_page_id]
            .downcast_ref::<super::Page>()
            .unwrap();

        let devices = sound_page
            .model
            .device_profiles
            .iter()
            .filter_map(|(object_id, profiles)| {
                let name = sound_page.model.device_names.get(object_id)?.as_str();

                // TODO: cache
                let active_profile =
                    sound_page
                        .model
                        .active_profiles
                        .get(object_id)
                        .and_then(|profile| {
                            profiles
                                .iter()
                                .enumerate()
                                .find(|(_, p)| p.index == profile.index)
                                .map(|(pos, _)| pos)
                        });

                // TODO: cache
                let profiles = profiles
                    .iter()
                    .filter(|p| {
                        matches!(
                            p.available,
                            pipewire::Availability::Yes | pipewire::Availability::Unknown
                        )
                    })
                    .map(|p| p.description.clone());

                let dropdown =
                    widget::dropdown(Vec::from_iter(profiles), active_profile, move |pos| {
                        super::Message::SetProfile(object_id, pos).into()
                    });
                Some((
                    name,
                    widget::settings::item::builder(name).control(dropdown),
                ))
            })
            .sorted_by(|a, b| a.0.cmp(b.0))
            .map(|(_, element)| element);

        widget::settings::section().extend(devices).into()
    })
}
