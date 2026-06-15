// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::model;
use cosmic::iced::futures;
use cosmic::{Apply, iced, widget};
use cosmic_settings_audio_client::{self as audio_client, CosmicAudioProxy};
use cosmic_settings_page::{self as page, Section, section};
use futures::executor::block_on;
use slotmap::SlotMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Message {
    /// Update for the model.
    Model(cosmic_settings_sound::Message),
    /// Set the profile of a sound device.
    SetProfile(u32, u32),
    /// Surface Action
    Surface(cosmic::surface::Action),
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
    model: model::Model,
    client: Option<Rc<RefCell<audio_client::Client>>>,
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

    fn on_leave(&mut self) -> cosmic::Task<crate::pages::Message> {
        *self = Page {
            entity: self.entity,
            ..Page::default()
        };
        cosmic::Task::none()
    }

    fn subscription(&self, _core: &cosmic::Core) -> iced::Subscription<crate::pages::Message> {
        iced::Subscription::run(|| {
            iced::stream::channel(
                1,
                move |emitter: futures::channel::mpsc::Sender<crate::pages::Message>| async move {
                    cosmic_settings_sound::subscribe(emitter, |m| Message::Model(m).into()).await
                },
            )
        })
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::app::Message> {
        match message {
            Message::Model(cosmic_settings_sound::Message::Subscription(message)) => {
                self.model.update(message);
            }

            Message::Model(cosmic_settings_sound::Message::Client(client)) => {
                if let Some(client) = Arc::into_inner(client) {
                    self.client = Some(Rc::new(RefCell::new(client)));
                    self.model = model::Model {
                        text: model::Text {
                            hd_audio: fl!("sound-hd-audio"),
                            usb_audio: fl!("sound-usb-audio"),
                        },
                        ..model::Model::default()
                    };
                }
            }

            Message::Surface(a) => return cosmic::task::message(crate::app::Message::Surface(a)),

            Message::SetProfile(id, index) => {
                if let Some(client) = self.client.clone() {
                    block_on(async move {
                        _ = client.borrow_mut().conn.set_profile(id, index, true).await;
                    });
                }
            }
        }

        cosmic::Task::none()
    }
}

pub fn view() -> Section<crate::pages::Message> {
    Section::default().view::<Page>(move |_, page, _section| {
        let devices = page.model.device_profile_dropdowns.iter().cloned().map(
            |(device_id, name, active_profile, indexes, descriptions)| {
                let dropdown = widget::dropdown::popup_dropdown(
                    descriptions,
                    active_profile,
                    move |id| Message::SetProfile(device_id, indexes[id]),
                    cosmic::iced::window::Id::RESERVED,
                    Message::Surface,
                    crate::Message::from,
                )
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::from);

                widget::settings::item::builder(name).control(dropdown)
            },
        );

        widget::settings::section().extend(devices).into()
    })
}
