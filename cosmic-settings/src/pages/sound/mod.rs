// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod device_profiles;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use cosmic::iced::{self, Alignment, Length, window};
use cosmic::widget::space::horizontal as horizontal_space;
use cosmic::widget::{self, settings};
use cosmic::{Apply, Element, Task, surface};
use cosmic_config::{Config, ConfigGet, ConfigSet};
use cosmic_settings_audio_client::{self as audio_client, CosmicAudioProxy};
use cosmic_settings_page::{self as page, Section, section};
use cosmic_settings_sound::model;
use futures::executor::block_on;
use slotmap::SlotMap;

const AUDIO_CONFIG: &str = "com.system76.CosmicAudio";
const AMPLIFICATION_SINK: &str = "amplification_sink";
const AMPLIFICATION_SOURCE: &str = "amplification_source";

#[derive(Clone, Debug)]
pub enum Message {
    /// Updates for the model.
    Model(cosmic_settings_sound::Message),
    /// Change the default output.
    SetDefaultSink(usize),
    /// Change the default input output.
    SetDefaultSource(usize),
    /// Change the balance of the active sink.
    SetSinkBalance(f32),
    /// Request to change the default output volume.
    SetSinkVolume(u32),
    /// Request to change the input volume.
    SetSourceVolume(u32),
    /// Surface Action
    Surface(surface::Action),
    /// Toggle the mute status of the output.
    ToggleSinkMute,
    /// Toggle the mute status of the input output.
    ToggleSourceMute,
    /// Toggle amplification for sink
    ToggleOverAmplificationSink(bool),
    /// Toggle amplification for sink
    ToggleOverAmplificationSource(bool),
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Sound(message)
    }
}

impl From<Message> for crate::Message {
    fn from(message: Message) -> Self {
        crate::Message::PageMessage(message.into())
    }
}

pub struct Page {
    entity: page::Entity,
    device_profiles: page::Entity,
    client: Option<Rc<RefCell<audio_client::Client>>>,
    model: model::Model,
    sound_config: Option<Config>,
    amplification_sink: bool,
    amplification_source: bool,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            entity: page::Entity::default(),
            device_profiles: page::Entity::default(),
            client: None,
            model: model::Model {
                text: model::Text {
                    hd_audio: fl!("sound-hd-audio"),
                    usb_audio: fl!("sound-usb-audio"),
                },
                ..model::Model::default()
            },
            sound_config: None,
            amplification_sink: false,
            amplification_source: false,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        match Config::new(AUDIO_CONFIG, 1) {
            Ok(config) => {
                self.amplification_sink = config.get::<bool>(AMPLIFICATION_SINK).unwrap_or(true);
                self.amplification_source =
                    config.get::<bool>(AMPLIFICATION_SOURCE).unwrap_or(false);
                self.sound_config = Some(config);
            }
            Err(why) => {
                tracing::error!(?why, "Failed to load sound config");
                self.amplification_sink = true;
                self.amplification_source = false;
            }
        }
        Task::none()
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(output()),
            sections.insert(input()),
            sections.insert(device_profiles()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("sound", "preferences-sound-symbolic")
            .title(fl!("sound"))
            .description(fl!("xdg-entry-sound-comment"))
    }

    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
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

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        *self = Page {
            entity: self.entity,
            device_profiles: self.device_profiles,
            ..Page::default()
        };

        Task::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: page::Insert<crate::pages::Message>,
    ) -> page::Insert<crate::pages::Message> {
        let id = page.sub_page_with_id::<device_profiles::Page>();
        let model = page.model.page_mut::<Page>().unwrap();
        model.device_profiles = id;
        page
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        tracing::debug!(target: "sound", ?message, "update");
        match message {
            Message::Surface(a) => return cosmic::task::message(crate::app::Message::Surface(a)),

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

            Message::SetDefaultSink(pos) => {
                if let Some(&node_id) = self.model.sinks.id.get(pos) {
                    if let Some(client) = self.client.as_mut() {
                        block_on(async {
                            _ = client.borrow_mut().conn.set_default(node_id, true).await;
                        });
                    }
                }
            }

            Message::SetDefaultSource(pos) => {
                if let Some(&node_id) = self.model.sources.id.get(pos) {
                    if let Some(client) = self.client.as_mut() {
                        block_on(async {
                            _ = client.borrow_mut().conn.set_default(node_id, true).await;
                        });
                    }
                }
            }

            Message::ToggleSinkMute => {
                if let Some(ref mut client) = self.client {
                    block_on(async {
                        _ = client.borrow_mut().conn.sink_mute_toggle().await;
                    });
                }
            }

            Message::ToggleSourceMute => {
                if let Some(ref mut client) = self.client {
                    block_on(async {
                        _ = client.borrow_mut().conn.source_mute_toggle().await;
                    });
                }
            }

            Message::SetSinkVolume(volume) => {
                if let Some(ref mut client) = self.client {
                    self.model.active_sink.volume = volume;
                    self.model.active_sink.volume_text = volume.to_string();
                    block_on(async {
                        _ = client.borrow_mut().conn.set_sink_volume(volume).await;
                    });
                }
            }

            Message::SetSourceVolume(volume) => {
                if let Some(ref mut client) = self.client {
                    self.model.active_source.volume = volume;
                    self.model.active_source.volume_text = volume.to_string();
                    block_on(async {
                        _ = client.borrow_mut().conn.set_source_volume(volume).await;
                    });
                }
            }

            Message::SetSinkBalance(balance) => {
                if let Some((client, sink_id)) = self.client.as_ref().zip(self.model.default_sink) {
                    self.model.active_sink.balance = Some(balance);
                    block_on(async {
                        _ = client
                            .borrow_mut()
                            .conn
                            .set_node_volume_balance(sink_id, Some(balance))
                            .await;
                    });
                }
            }

            Message::ToggleOverAmplificationSink(enabled) => {
                self.amplification_sink = enabled;

                if let Some(config) = &self.sound_config
                    && let Err(why) = config.set(AMPLIFICATION_SINK, enabled)
                {
                    tracing::error!(?why, "Failed to save over amplification setting");
                }
            }

            Message::ToggleOverAmplificationSource(enabled) => {
                self.amplification_source = enabled;

                if let Some(config) = &self.sound_config
                    && let Err(why) = config.set(AMPLIFICATION_SOURCE, enabled)
                {
                    tracing::error!(?why, "Failed to save over amplification setting");
                }
            }
        }

        Task::none()
    }
}

fn input() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        volume = fl!("sound-input", "volume");
        device = fl!("sound-input", "device");
        _level = fl!("sound-input", "level");
        amplification = fl!("amplification");
        amplification_desc = fl!("amplification", "desc");
    });

    Section::default()
        .title(fl!("sound-input"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            if page.model.sources.id.is_empty() {
                return widget::space().into();
            }

            let slider = if page.amplification_source {
                widget::slider(0..=150, page.model.active_source.volume, |change| {
                    Message::SetSourceVolume(change).into()
                })
                .breakpoints(&[100])
            } else {
                widget::slider(0..=100, page.model.active_source.volume, |change| {
                    Message::SetSourceVolume(change).into()
                })
            }
            .width(Length::Fill)
            .apply(widget::container)
            .max_width(250.);

            let volume_control = widget::row::with_capacity(4)
                .align_y(Alignment::Center)
                .push(
                    widget::button::icon(widget::icon::from_name(
                        if page.model.active_source.mute {
                            "microphone-sensitivity-muted-symbolic"
                        } else {
                            "audio-input-microphone-symbolic"
                        },
                    ))
                    .on_press(Message::ToggleSourceMute.into()),
                )
                .push(
                    widget::text::body(&page.model.active_source.volume_text)
                        .width(Length::Fixed(22.0))
                        .align_x(Alignment::Center),
                )
                .push(horizontal_space().width(8.))
                .push(slider);
            let devices = widget::dropdown::popup_dropdown(
                &page.model.sources.display,
                Some(page.model.sources.active.unwrap_or(0)),
                Message::SetDefaultSource,
                window::Id::RESERVED,
                Message::Surface,
                crate::Message::from,
            )
            .apply(Element::from)
            .map(crate::pages::Message::from);

            let mut controls = settings::section()
                .title(&section.title)
                .add(
                    settings::item::builder(&*section.descriptions[volume])
                        .flex_control(volume_control)
                        .align_items(Alignment::Center),
                )
                .add(settings::item(&*section.descriptions[device], devices));

            controls = controls.add(
                settings::item::builder(&*section.descriptions[amplification])
                    .description(&*section.descriptions[amplification_desc])
                    .toggler(page.amplification_source, |t| {
                        Message::ToggleOverAmplificationSource(t).into()
                    }),
            );

            Element::from(controls)
        })
}

fn output() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        volume = fl!("sound-output", "volume");
        device = fl!("sound-output", "device");
        _level = fl!("sound-output", "level");
        balance = fl!("sound-output", "balance");
        left = fl!("sound-output", "left");
        right = fl!("sound-output", "right");
        amplification = fl!("amplification");
        amplification_desc = fl!("amplification", "desc");
    });

    Section::default()
        .title(fl!("sound-output"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let slider = if page.amplification_sink {
                widget::slider(0..=150, page.model.active_sink.volume, |change| {
                    Message::SetSinkVolume(change).into()
                })
                .breakpoints(&[100])
            } else {
                widget::slider(0..=100, page.model.active_sink.volume, |change| {
                    Message::SetSinkVolume(change).into()
                })
            }
            .width(Length::Fill)
            .apply(widget::container)
            .max_width(250.);

            let volume_control = widget::row::with_capacity(4)
                .align_y(Alignment::Center)
                .push(
                    widget::button::icon(if page.model.active_sink.mute {
                        widget::icon::from_name("audio-volume-muted-symbolic")
                    } else {
                        widget::icon::from_name("audio-volume-high-symbolic")
                    })
                    .on_press(Message::ToggleSinkMute.into()),
                )
                .push(
                    widget::text::body(&page.model.active_sink.volume_text)
                        .width(Length::Fixed(22.0))
                        .align_x(Alignment::Center),
                )
                .push(horizontal_space().width(8.))
                .push(slider);

            let devices = widget::dropdown::popup_dropdown(
                &page.model.sinks.display,
                Some(page.model.sinks.active.unwrap_or(0)),
                Message::SetDefaultSink,
                window::Id::RESERVED,
                Message::Surface,
                crate::Message::from,
            )
            .apply(Element::from)
            .map(crate::pages::Message::from);

            let mut controls = settings::section()
                .title(&section.title)
                .add(
                    settings::item::builder(&*section.descriptions[volume])
                        .flex_control(volume_control)
                        .align_items(Alignment::Center),
                )
                .add(settings::item(&*section.descriptions[device], devices))
                .add(settings::item(
                    &*section.descriptions[balance],
                    widget::row::with_capacity(5)
                        .align_y(Alignment::Center)
                        .push(
                            widget::column::with_capacity(2)
                                .align_x(Alignment::Center)
                                .push(
                                    widget::text::body(&*section.descriptions[left])
                                        .align_x(Alignment::Center),
                                )
                                .push(horizontal_space().width(22.)),
                        )
                        .push(horizontal_space().width(8.))
                        .push(
                            widget::slider(
                                0.0..=2.0,
                                page.model.active_sink.balance.unwrap_or(1.0),
                                |change| Message::SetSinkBalance(change).into(),
                            )
                            .step(0.01)
                            .breakpoints(&[1.0]),
                        )
                        .push(horizontal_space().width(8.))
                        .push(
                            widget::column::with_capacity(2)
                                .align_x(Alignment::Center)
                                .push(
                                    widget::text::body(&*section.descriptions[right])
                                        .align_x(Alignment::Center),
                                )
                                .push(horizontal_space().width(22.0)),
                        ),
                ));

            controls = controls.add(
                settings::item::builder(&*section.descriptions[amplification])
                    .description(&*section.descriptions[amplification_desc])
                    .toggler(page.amplification_sink, |t| {
                        Message::ToggleOverAmplificationSink(t).into()
                    }),
            );

            Element::from(controls)
        })
}

/// A section for opening the device profiles sub-page.
fn device_profiles() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        button_txt = fl!("sound-device-profiles");
    });

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .add(crate::widget::go_next_item(
                    &descriptions[button_txt],
                    crate::pages::Message::Page(page.device_profiles),
                ))
                .into()
        })
}

// fn alerts() -> Section<crate::pages::Message> {
//     let mut descriptions = Slab::new();
//     let volume = descriptions.insert(fl!("sound-alerts", "volume"));
//     let sound = descriptions.insert(fl!("sound-alerts", "sound"));

//     Section::default()
//         .title(fl!("sound-alerts"))
//         .descriptions(descriptions)
//         .view::<Page>(move |_binder, _page, section| {
//             settings::section().title(&section.title)
//                 .add(settings::item(&section.descriptions[volume], text::body("TODO")))
//                 .add(settings::item(&section.descriptions[sound], text::body("TODO")))
//                 .into()
//         })
// }

// fn applications() -> Section<crate::pages::Message> {
//     let mut descriptions = Slab::new();

//     let applications = descriptions.insert(fl!("sound-applications", "desc"));

//     Section::default()
//         .title(fl!("sound-applications"))
//         .descriptions(descriptions)
//         .view::<Page>(move |_binder, _page, section| {
//             settings::section().title(&section.title)
//                 .add(settings::item(
//                     &*section.descriptions[applications],
//                     text::body("TODO"),
//                 ))
//                 .into()
//         })
// }
