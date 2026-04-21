// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::model;
use cosmic::iced::futures;
use cosmic::{Apply, iced, widget};
use cosmic_settings_audio_client::{self as audio_client, CosmicAudioProxy};
use cosmic_settings_page::{self as page, Section, section};
use futures::executor::block_on;
use futures::{SinkExt, StreamExt};
use slotmap::SlotMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Message {
    /// Varlink client connection to the audio settings daemon.
    Client(Arc<audio_client::Client>),
    /// Set the profile of a sound device.
    SetProfile(u32, u32),
    /// Audio events from the audio settings daemon.
    Subscription(audio_client::Event),
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
                move |mut emitter: futures::channel::mpsc::Sender<crate::pages::Message>| async move {
                    loop {
                        let mut client = match audio_client::connect().await {
                            Ok(client) => client,
                            Err(why) => {
                                if let zlink::Error::Io(ref why) = why
                                    && why.kind() == std::io::ErrorKind::NotFound
                                {
                                    tracing::error!(
                                        "cosmic-settings-daemon varlink service not found. Restarting cosmic-settings-daemon"
                                    );
                                    _ = std::process::Command::new("killall")
                                        .args(&["-2", "cosmic-settings-daemon"])
                                        .status();
                                } else {
                                    tracing::error!(
                                        ?why,
                                        "failed to connect to cosmic-settings's varlink service"
                                    );
                                }

                                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                                continue;
                            }
                        };

                        if let Ok(Ok(mut stream)) = client.recv_events().await {
                            _ = emitter.send(Message::Client(Arc::new(client)).into()).await;
                            while let Some(message) = stream.next().await {
                                match message {
                                    Ok(event) => {
                                        _ = emitter.send(Message::Subscription(event).into()).await;
                                    }
                                    Err(why) => {
                                        tracing::error!(?why, "event error");
                                    }
                                }
                            }
                        }
                    }
                },
            )
        })
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::app::Message> {
        match message {
            Message::Subscription(event) => {
                self.model.update(event);
            }

            Message::Surface(a) => return cosmic::task::message(crate::app::Message::Surface(a)),

            Message::SetProfile(id, index) => {
                if let Some(client) = self.client.clone() {
                    block_on(async move {
                        _ = client.borrow_mut().conn.set_profile(id, index, true).await;
                    });
                }
            }

            Message::Client(client) => {
                if let Some(client) = Arc::into_inner(client) {
                    self.client = Some(Rc::new(RefCell::new(client)));
                    self.model = model::Model::default();
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
