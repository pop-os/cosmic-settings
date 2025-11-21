// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{Apply, widget};
use cosmic_settings_page::{self as page, Section, section};
use cosmic_settings_sound_subscription::{self as subscription};
use slotmap::SlotMap;

#[derive(Clone, Debug)]
pub enum Message {}

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

    fn on_leave(&mut self) -> cosmic::Task<crate::pages::Message> {
        cosmic::Task::done(crate::pages::Message::Sound(super::Message::Reload))
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
            .device_profile_dropdowns
            .iter()
            .cloned()
            .map(|(device_id, name, active_profile, indexes, descriptions)| {
                let dropdown = widget::dropdown::popup_dropdown(
                    descriptions,
                    active_profile,
                    move |id| super::Message::SetProfile(device_id, indexes[id]),
                    cosmic::iced::window::Id::RESERVED,
                    super::Message::Surface,
                    crate::Message::from,
                )
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::from);

                widget::settings::item::builder(name).control(dropdown)
            });

        widget::settings::section().extend(devices).into()
    })
}
