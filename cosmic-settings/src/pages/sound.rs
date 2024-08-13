// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::HashMap, time::Duration};

use cosmic::{
    widget::{self, settings},
    Apply, Command, Element,
};
use cosmic_settings_page::{self as page, section, Section};
use cosmic_settings_subscriptions::{pipewire, pulse};
use futures::StreamExt;
use slab::Slab;
use slotmap::SlotMap;

#[derive(Clone, Debug)]
pub enum Message {
    /// Get default sinks/sources and their volumes/mute status.
    Pulse(pulse::Event),
    /// Get ALSA cards and their profiles.
    Pipewire(pipewire::DeviceEvent),
    /// Change the default microphone input.
    SinkChanged(usize),
    /// Request to change the default microphone volume.
    SinkVolumeChanged(u32),
    /// Change the microphone volume.
    SinkVolumeApply(NodeId),
    /// Toggle the mute status of the microphone.
    SinkMuteToggle,
    /// Change the default speaker output.
    SourceChanged(usize),
    /// Request to change the speaker volume.
    SourceVolumeChanged(u32),
    /// Change the speaker volume.
    SourceVolumeApply(NodeId),
    /// Toggle the mute status of the speaker output.
    SourceMuteToggle,
}

pub type CardId = u32;
pub type NodeId = u32;
pub type ProfileId = u32;

struct Card {
    class: pipewire::MediaClass,
    // name: String,
    profiles: HashMap<NodeId, Profile>,
}

struct Profile {
    // device: ProfileId,
    identifier: String,
}

#[derive(Default)]
pub struct Page {
    pipewire_thread: Option<(tokio::sync::oneshot::Sender<()>, pipewire::Sender<()>)>,
    pulse_thread: Option<tokio::sync::oneshot::Sender<()>>,
    alsa_cards: HashMap<CardId, Card>,
    default_sink: String,
    default_source: String,

    sink_volume: u32,
    sink_volume_text: String,
    sink_mute: bool,
    sink_volume_debounce: bool,

    source_volume: u32,
    source_volume_text: String,
    source_mute: bool,
    source_volume_debounce: bool,

    sinks: Vec<String>,
    sink_ids: Vec<NodeId>,
    sources: Vec<String>,
    source_ids: Vec<NodeId>,
    active_sink: Option<usize>,
    active_source: Option<usize>,
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(output()),
            sections.insert(input()),
            // sections.insert(alerts()),
            // sections.insert(applications()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("sound", "multimedia-volume-control-symbolic")
            .title(fl!("sound"))
            .description(fl!("sound", "desc"))
    }

    fn on_enter(
        &mut self,
        _page: cosmic_settings_page::Entity,
        sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> Command<crate::pages::Message> {
        if self.pulse_thread.is_none() {
            let sender = sender.clone();

            let (tx, mut rx) = futures::channel::mpsc::channel(1);
            let (cancel_tx, cancel_rx) = tokio::sync::oneshot::channel::<()>();

            // Listen to events from the pulse thread until the tx channel is closed.
            _ = std::thread::spawn(move || {
                pulse::thread(tx);
            });

            // Forward events from the pulse thread to the application until
            // the application requests to stop listening to the pulse thread.
            tokio::task::spawn(async move {
                let forwarder = std::pin::pin!(async move {
                    while let Some(event) = rx.next().await {
                        let event = crate::pages::Message::Sound(Message::Pulse(event));
                        if sender.send(event).await.is_err() {
                            break;
                        }
                    }
                });

                futures::future::select(std::pin::pin!(cancel_rx), forwarder).await;
            });

            self.pulse_thread = Some(cancel_tx);
        }

        if self.pipewire_thread.is_none() {
            let (tx, mut rx) = futures::channel::mpsc::channel(1);
            let (cancel_tx, cancel_rx) = tokio::sync::oneshot::channel::<()>();

            // Listen to events from the pipewire thread until the tx channel is closed.
            let (_handle, terminate) = pipewire::thread(tx);

            // Forward events from the pipewire thread to the application until
            // the application requests to stop listening to the pulse thread.
            tokio::task::spawn(async move {
                let forwarder = std::pin::pin!(async move {
                    while let Some(event) = rx.next().await {
                        let event = crate::pages::Message::Sound(Message::Pipewire(event));
                        if sender.send(event).await.is_err() {
                            break;
                        }
                    }
                });

                futures::future::select(std::pin::pin!(cancel_rx), forwarder).await;
            });

            self.pipewire_thread = Some((cancel_tx, terminate));
        }

        Command::none()
    }

    fn on_leave(&mut self) -> Command<crate::pages::Message> {
        if let Some(cancellation) = self.pulse_thread.take() {
            _ = cancellation.send(());
        }

        if let Some((cancellation, terminate)) = self.pipewire_thread.take() {
            _ = cancellation.send(());
            _ = terminate.send(());
        }

        *self = Page::default();

        Command::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl Page {
    fn set_default_sink(&mut self) {
        for card in self.alsa_cards.values() {
            if let pipewire::MediaClass::Sink = card.class {
                for (&node_id, profile) in &card.profiles {
                    if profile.identifier == self.default_sink {
                        self.active_sink = self.sink_ids.iter().position(|&id| id == node_id);
                        return;
                    }
                }
            }
        }
    }

    fn set_default_source(&mut self) {
        for card in self.alsa_cards.values() {
            if let pipewire::MediaClass::Source = card.class {
                for (&node_id, profile) in &card.profiles {
                    if profile.identifier == self.default_source {
                        self.active_source = self.source_ids.iter().position(|&id| id == node_id);
                        return;
                    }
                }
            }
        }
    }

    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        match message {
            Message::SourceVolumeChanged(volume) => {
                self.source_volume = volume;
                self.source_volume_text = volume.to_string();
                if self.source_volume_debounce {
                    return Command::none();
                }

                let mut command = None;
                if let Some(&node_id) = self.source_ids.get(self.active_source.unwrap_or(0)) {
                    command = Some(cosmic::command::future(async move {
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        crate::pages::Message::Sound(Message::SourceVolumeApply(node_id))
                    }));
                }

                if let Some(command) = command {
                    self.source_volume_debounce = true;
                    return command;
                }
            }

            Message::Pulse(pulse::Event::SourceVolume(volume)) => {
                if self.sink_volume_debounce {
                    return Command::none();
                }

                self.source_volume = volume;
                self.source_volume_text = volume.to_string();
            }

            Message::SinkVolumeChanged(volume) => {
                self.sink_volume = volume;
                self.sink_volume_text = volume.to_string();
                if self.sink_volume_debounce {
                    return Command::none();
                }

                let mut command = None;
                if let Some(&node_id) = self.sink_ids.get(self.active_sink.unwrap_or(0)) {
                    command = Some(cosmic::command::future(async move {
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        crate::pages::Message::Sound(Message::SinkVolumeApply(node_id))
                    }));
                }

                if let Some(command) = command {
                    self.source_volume_debounce = true;
                    return command;
                }
            }

            Message::Pulse(pulse::Event::SinkVolume(volume)) => {
                if self.sink_volume_debounce {
                    return Command::none();
                }

                self.sink_volume = volume;
                self.sink_volume_text = volume.to_string();
            }

            Message::Pulse(pulse::Event::DefaultSink(sink)) => {
                self.default_sink = sink;
                self.set_default_sink();
            }

            Message::Pulse(pulse::Event::DefaultSource(source)) => {
                self.default_source = source;
                self.set_default_source();
            }

            Message::Pulse(pulse::Event::SinkMute(mute)) => {
                self.sink_mute = mute;
            }

            Message::Pulse(pulse::Event::SourceMute(mute)) => {
                self.source_mute = mute;
            }

            Message::Pipewire(pipewire::DeviceEvent::Add(device)) => {
                match device.media_class {
                    pipewire::MediaClass::Sink => {
                        self.sinks.push(device.node_description.clone());
                        self.sink_ids.push(device.object_id);
                        if self.default_sink == device.node_name {
                            self.active_sink = Some(self.sinks.len() - 1);
                        }
                    }

                    pipewire::MediaClass::Source => {
                        self.sources.push(device.node_description.clone());
                        self.source_ids.push(device.object_id);
                        if self.default_source == device.node_name {
                            self.active_source = Some(self.sources.len() - 1);
                        }
                    }
                }

                let card = self
                    .alsa_cards
                    .entry(device.alsa_card)
                    .or_insert_with(|| Card {
                        class: device.media_class,
                        // name: device.alsa_card_name,
                        profiles: HashMap::new(),
                    });

                card.profiles.insert(
                    device.object_id,
                    Profile {
                        // device: device.card_profile_device,
                        identifier: device.node_name,
                    },
                );
            }

            Message::Pipewire(pipewire::DeviceEvent::Remove(device_id)) => {
                let mut remove = None;
                for (card_id, card) in &mut self.alsa_cards {
                    if card.profiles.remove(&device_id).is_some() {
                        if card.profiles.is_empty() {
                            remove = Some(*card_id);
                        }
                        break;
                    }
                }

                if let Some(card_id) = remove {
                    _ = self.alsa_cards.remove(&card_id);
                }

                if let Some(pos) = self.sink_ids.iter().position(|&id| id == device_id) {
                    _ = self.sink_ids.remove(pos);
                    _ = self.sinks.remove(pos);
                    if self.active_sink == Some(pos) {
                        self.active_sink = None;
                    }
                } else if let Some(pos) = self.source_ids.iter().position(|&id| id == device_id) {
                    _ = self.source_ids.remove(pos);
                    _ = self.sources.remove(pos);
                    if self.active_source == Some(pos) {
                        self.active_source = None;
                    }
                }
            }

            Message::SinkChanged(pos) => {
                if let Some(&node_id) = self.sink_ids.get(pos) {
                    self.active_sink = Some(pos);
                    wpctl_set_default(node_id);
                }
            }

            Message::SourceChanged(pos) => {
                if let Some(&node_id) = self.sink_ids.get(pos) {
                    self.active_source = Some(pos);
                    wpctl_set_default(node_id);
                }
            }

            Message::SinkVolumeApply(node_id) => {
                self.sink_volume_debounce = false;
                wpctl_set_volume(node_id, self.sink_volume);
            }

            Message::SourceVolumeApply(node_id) => {
                self.source_volume_debounce = false;
                wpctl_set_volume(node_id, self.source_volume);
            }

            Message::SinkMuteToggle => {
                self.sink_mute = !self.sink_mute;
                if let Some(&node_id) = self.sink_ids.get(self.active_sink.unwrap_or(0)) {
                    wpctl_set_mute(node_id, self.sink_mute);
                }
            }

            Message::SourceMuteToggle => {
                self.source_mute = !self.source_mute;
                if let Some(&node_id) = self.source_ids.get(self.active_source.unwrap_or(0)) {
                    wpctl_set_mute(node_id, self.source_mute);
                }
            }
        }
        Command::none()
    }
}

fn input() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let volume = descriptions.insert(fl!("sound-input", "volume"));
    let device = descriptions.insert(fl!("sound-input", "device"));
    let _level = descriptions.insert(fl!("sound-input", "level"));

    Section::default()
        .title(fl!("sound-input"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let volume_control = widget::row::with_capacity(3)
                .align_items(cosmic::iced::Alignment::Center)
                .spacing(4)
                .push(
                    widget::button::icon(if page.source_mute {
                        widget::icon::from_name("microphone-sensitivity-muted-symbolic")
                    } else {
                        widget::icon::from_name("audio-input-microphone-symbolic")
                    })
                    .on_press(Message::SourceMuteToggle),
                )
                .push(widget::text::body(&page.source_volume_text))
                .push(
                    widget::slider(0..=150, page.source_volume, Message::SourceVolumeChanged)
                        .width(250)
                        .breakpoints(&[100]),
                );

            let devices = widget::dropdown(
                &page.sources,
                Some(page.active_source.unwrap_or(0)),
                Message::SourceChanged,
            );

            settings::view_section(&section.title)
                .add(settings::item(
                    &*section.descriptions[volume],
                    volume_control,
                ))
                .add(settings::item(&*section.descriptions[device], devices))
                .apply(Element::from)
                .map(crate::pages::Message::Sound)
        })
}

fn output() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let volume = descriptions.insert(fl!("sound-output", "volume"));
    let device = descriptions.insert(fl!("sound-output", "device"));
    let _level = descriptions.insert(fl!("sound-output", "level"));
    // let config = descriptions.insert(fl!("sound-output", "config"));
    // let balance = descriptions.insert(fl!("sound-output", "balance"));

    Section::default()
        .title(fl!("sound-output"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let volume_control = widget::row::with_capacity(3)
                .align_items(cosmic::iced::Alignment::Center)
                .spacing(4)
                .push(
                    widget::button::icon(if page.sink_mute {
                        widget::icon::from_name("audio-volume-muted-symbolic")
                    } else {
                        widget::icon::from_name("audio-volume-high-symbolic")
                    })
                    .on_press(Message::SinkMuteToggle),
                )
                .push(widget::text::body(&page.sink_volume_text))
                .push(
                    widget::slider(0..=150, page.sink_volume, Message::SinkVolumeChanged)
                        .width(250)
                        .breakpoints(&[100]),
                );

            let devices = widget::dropdown(
                &page.sinks,
                Some(page.active_sink.unwrap_or(0)),
                Message::SinkChanged,
            );

            settings::view_section(&section.title)
                .add(settings::item(
                    &*section.descriptions[volume],
                    volume_control,
                ))
                .add(settings::item(&*section.descriptions[device], devices))
                .apply(Element::from)
                .map(crate::pages::Message::Sound)
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
//             settings::view_section(&section.title)
//                 .add(settings::item(&section.descriptions[volume], text("TODO")))
//                 .add(settings::item(&section.descriptions[sound], text("TODO")))
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
//             settings::view_section(&section.title)
//                 .add(settings::item(
//                     &*section.descriptions[applications],
//                     text("TODO"),
//                 ))
//                 .into()
//         })
// }

fn wpctl_set_default(id: u32) {
    tokio::task::spawn(async move {
        let default = id.to_string();
        _ = tokio::process::Command::new("wpctl")
            .args(&["set-default", default.as_str()])
            .status()
            .await;
    });
}

fn wpctl_set_volume(id: u32, volume: u32) {
    tokio::task::spawn(async move {
        let id = id.to_string();
        let volume = format!("{}.{:02}", volume / 100, volume % 100);
        _ = tokio::process::Command::new("wpctl")
            .args(&["set-volume", id.as_str(), volume.as_str()])
            .status()
            .await;
    });
}

fn wpctl_set_mute(id: u32, mute: bool) {
    tokio::task::spawn(async move {
        let default = id.to_string();
        _ = tokio::process::Command::new("wpctl")
            .args(&["set-mute", default.as_str(), if mute { "1" } else { "0" }])
            .status()
            .await;
    });
}
