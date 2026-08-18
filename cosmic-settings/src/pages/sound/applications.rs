// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::model;
use cosmic::desktop::{
    DesktopEntryCache, DesktopLookupContext, DesktopResolveOptions, resolve_desktop_entry,
};
use cosmic::iced::futures;
use cosmic::iced::{self, Alignment, Length, window};
use cosmic::widget::space::horizontal as horizontal_space;
use cosmic::{Apply, Element, surface, widget};
use cosmic_config::{Config, ConfigGet};
use cosmic_settings_audio_client::{self as audio_client, CosmicAudioProxy};
use cosmic_settings_page::{self as page, Section, section};
use futures::executor::block_on;
use intmap::IntMap;
use slotmap::SlotMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

const AUDIO_CONFIG: &str = "com.system76.CosmicAudio";
const AMPLIFICATION_SINK: &str = "amplification_sink";

#[derive(Clone, Debug)]
pub enum Message {
    Model(cosmic_settings_sound::Message),
    SetBalance(u32, f32),
    SetMute(u32, bool),
    SetSink(u32, usize),
    SetVolume(u32, u32),
    Surface(surface::Action),
    ToggleExpanded(u32),
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::SoundApplications(message)
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
    desktop_entries: DesktopEntryCache,
    playback_icons: IntMap<u32, String>,
    amplification_sink: bool,
    expanded: Option<u32>,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        self.amplification_sink = Config::new(AUDIO_CONFIG, 1)
            .ok()
            .and_then(|config| config.get::<bool>(AMPLIFICATION_SINK).ok())
            .unwrap_or(true);
        cosmic::Task::none()
    }

    fn info(&self) -> page::Info {
        page::Info::new("sound-applications", "preferences-sound-symbolic")
            .title(fl!("sound-applications"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(view())])
    }

    fn set_id(&mut self, entity: page::Entity) {
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
    fn resolve_playback_icon(&mut self, node_id: u32, info: &audio_client::PlaybackInfo) {
        let Some(candidate) = [
            info.application_id.as_deref(),
            info.icon_name.as_deref(),
            info.application_name.as_deref(),
        ]
        .into_iter()
        .flatten()
        .find(|value| !value.is_empty()) else {
            return;
        };

        let mut context = DesktopLookupContext::new(candidate);
        if let Some(icon_name) = info.icon_name.as_deref().filter(|value| !value.is_empty()) {
            context = context.with_identifier(icon_name);
        }
        if let Some(application_name) = info
            .application_name
            .as_deref()
            .filter(|value| !value.is_empty())
        {
            context = context.with_title(application_name);
        }

        let entry = resolve_desktop_entry(
            &mut self.desktop_entries,
            &context,
            &DesktopResolveOptions::default(),
        );
        let entry_icon = entry.icon();
        let icon_name = entry_icon
            .filter(|value| !value.is_empty())
            .or(info.icon_name.as_deref())
            .or(info.application_id.as_deref())
            .unwrap_or("application-default");

        self.playback_icons.insert(node_id, icon_name.to_owned());
    }

    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::app::Message> {
        match message {
            Message::Model(cosmic_settings_sound::Message::Subscription(message)) => {
                match &message {
                    audio_client::Event::Playback(node_id, info) => {
                        self.resolve_playback_icon(*node_id, info);
                    }
                    audio_client::Event::RemoveNode(node_id) => {
                        self.playback_icons.remove(*node_id);
                    }
                    _ => {}
                }
                self.model.update(message);
            }

            Message::Model(cosmic_settings_sound::Message::Client(client)) => {
                if let Some(client) = Arc::into_inner(client) {
                    self.client = Some(Rc::new(RefCell::new(client)));
                    self.model = model::Model::default();
                    self.playback_icons = IntMap::new();
                }
            }

            Message::SetMute(node_id, mute) => {
                self.model
                    .update(audio_client::Event::NodeMute(node_id, mute));
                if let Some(client) = &self.client {
                    match block_on(client.borrow_mut().conn.set_node_mute(node_id, mute)) {
                        Ok(Ok(_)) => {}
                        Ok(Err(why)) => {
                            tracing::error!(?why, node_id, mute, "daemon rejected playback mute");
                        }
                        Err(why) => {
                            tracing::error!(?why, node_id, mute, "failed to set playback mute");
                        }
                    }
                } else {
                    tracing::warn!(node_id, mute, "audio client is not connected");
                }
            }

            Message::SetBalance(playback_id, balance) => {
                if let Some(playback) = self
                    .model
                    .playback
                    .iter_mut()
                    .find(|playback| playback.id == playback_id)
                {
                    playback.balance = Some(balance);
                }

                if let Some(client) = &self.client {
                    match block_on(
                        client
                            .borrow_mut()
                            .conn
                            .set_node_volume_balance(playback_id, Some(balance)),
                    ) {
                        Ok(Ok(volume)) => {
                            self.model.update(audio_client::Event::NodeVolume(
                                playback_id,
                                volume.volume,
                                volume.balance,
                            ));
                        }
                        Ok(Err(why)) => {
                            tracing::error!(
                                ?why,
                                playback_id,
                                balance,
                                "daemon rejected playback balance"
                            );
                        }
                        Err(why) => {
                            tracing::error!(
                                ?why,
                                playback_id,
                                balance,
                                "failed to set playback balance"
                            );
                        }
                    }
                } else {
                    tracing::warn!(playback_id, balance, "audio client is not connected");
                }
            }

            Message::SetSink(playback_id, sorted_pos) => {
                let Some(&sink_pos) = self.model.sinks.sorted_index.get(sorted_pos) else {
                    return cosmic::Task::none();
                };
                let Some(&sink_id) = self.model.sinks.id.get(sink_pos as usize) else {
                    return cosmic::Task::none();
                };

                if let Some(client) = &self.client {
                    match block_on(
                        client
                            .borrow_mut()
                            .conn
                            .set_playback_sink(playback_id, sink_id),
                    ) {
                        Ok(Ok(())) => {
                            if let Some(playback) = self
                                .model
                                .playback
                                .iter_mut()
                                .find(|playback| playback.id == playback_id)
                            {
                                playback.sink_id = Some(sink_id);
                            }
                        }
                        Ok(Err(why)) => {
                            tracing::error!(
                                ?why,
                                playback_id,
                                sink_id,
                                "daemon rejected playback output"
                            );
                        }
                        Err(why) => {
                            tracing::error!(
                                ?why,
                                playback_id,
                                sink_id,
                                "failed to set playback output"
                            );
                        }
                    }
                } else {
                    tracing::warn!(playback_id, sink_id, "audio client is not connected");
                }
            }

            Message::SetVolume(node_id, volume) => {
                if let Some(playback) = self
                    .model
                    .playback
                    .iter_mut()
                    .find(|playback| playback.id == node_id)
                {
                    playback.volume = volume;
                }
                if let Some(client) = &self.client {
                    match block_on(client.borrow_mut().conn.set_node_volume(node_id, volume)) {
                        Ok(Ok(_)) => {}
                        Ok(Err(why)) => {
                            tracing::error!(
                                ?why,
                                node_id,
                                volume,
                                "daemon rejected playback volume"
                            );
                        }
                        Err(why) => {
                            tracing::error!(?why, node_id, volume, "failed to set playback volume");
                        }
                    }
                } else {
                    tracing::warn!(node_id, volume, "audio client is not connected");
                }
            }

            Message::Surface(action) => {
                return cosmic::task::message(crate::app::Message::Surface(action));
            }

            Message::ToggleExpanded(node_id) => {
                self.expanded = if self.expanded == Some(node_id) {
                    None
                } else {
                    Some(node_id)
                };
            }
        }

        cosmic::Task::none()
    }
}

pub fn view() -> Section<crate::pages::Message> {
    Section::default().view::<Page>(move |_, page, _section| {
        if page.model.playback.is_empty() {
            return widget::settings::section()
                .add(
                    widget::text::body(fl!("sound-applications", "none"))
                        .width(Length::Fill)
                        .align_x(Alignment::Center),
                )
                .into();
        }

        let mut applications = widget::settings::section();

        for playback in page.model.playback.iter().cloned() {
            let node_id = playback.id;
            let mute = playback.mute;
            let volume = playback.volume;
            let expanded = page.expanded == Some(node_id);
            let icon_name = page
                .playback_icons
                .get(node_id)
                .map(String::as_str)
                .unwrap_or("application-default");

            let slider = if page.amplification_sink {
                widget::slider(0..=150, volume, move |volume| {
                    Message::SetVolume(node_id, volume)
                })
                .breakpoints(&[100])
            } else {
                widget::slider(0..=100, volume, move |volume| {
                    Message::SetVolume(node_id, volume)
                })
            }
            .width(Length::Fill)
            .apply(widget::container)
            .max_width(250.);

            let volume_controls = widget::row::with_capacity(5)
                .align_y(Alignment::Center)
                .push(
                    widget::button::icon(widget::icon::from_name(if mute {
                        "audio-volume-muted-symbolic"
                    } else {
                        "audio-volume-high-symbolic"
                    }))
                    .on_press(Message::SetMute(node_id, !mute)),
                )
                .push(
                    widget::text::body(volume.to_string())
                        .width(Length::Fixed(22.))
                        .align_x(Alignment::Center),
                )
                .push(horizontal_space().width(8.))
                .push(slider)
                .push(
                    widget::button::icon(widget::icon::from_name(if expanded {
                        "go-up-symbolic"
                    } else {
                        "go-down-symbolic"
                    }))
                    .on_press(Message::ToggleExpanded(node_id)),
                )
                .apply(Element::from)
                .map(crate::pages::Message::from);

            applications = applications.add(
                widget::settings::item::builder(playback.name().to_owned())
                    .icon(widget::icon::from_name(icon_name).size(16))
                    .flex_control(volume_controls)
                    .align_items(Alignment::Center),
            );

            if !expanded {
                continue;
            }

            let selected_sink =
                playback
                    .selected_sink(page.model.default_sink)
                    .and_then(|sink_id| {
                        let sink_pos = page.model.sinks.id.iter().position(|id| *id == sink_id)?;
                        page.model
                            .sinks
                            .sorted_index
                            .iter()
                            .position(|pos| usize::from(*pos) == sink_pos)
                    });
            let output = widget::dropdown::popup_dropdown(
                &page.model.sinks.sorted_display,
                selected_sink,
                move |pos| Message::SetSink(node_id, pos),
                window::Id::RESERVED,
                Message::Surface,
                crate::Message::from,
            )
            .apply(Element::from)
            .map(crate::pages::Message::from);

            let balance = widget::row::with_capacity(5)
                .align_y(Alignment::Center)
                .push(
                    widget::column::with_capacity(2)
                        .align_x(Alignment::Center)
                        .push(widget::text::body(fl!("sound-output", "left")))
                        .push(horizontal_space().width(22.)),
                )
                .push(horizontal_space().width(8.))
                .push(
                    widget::slider(0.0..=2.0, playback.balance.unwrap_or(1.0), move |balance| {
                        Message::SetBalance(node_id, balance)
                    })
                    .step(0.01)
                    .breakpoints(&[1.0]),
                )
                .push(horizontal_space().width(8.))
                .push(
                    widget::column::with_capacity(2)
                        .align_x(Alignment::Center)
                        .push(widget::text::body(fl!("sound-output", "right")))
                        .push(horizontal_space().width(22.)),
                )
                .apply(Element::from)
                .map(crate::pages::Message::from);

            applications = applications
                .add(widget::settings::item(
                    fl!("sound-output", "device"),
                    output,
                ))
                .add(widget::settings::item(
                    fl!("sound-output", "balance"),
                    balance,
                ));
        }

        applications.into()
    })
}
