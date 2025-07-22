// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::BTreeMap, sync::Arc, time::Duration};

use cosmic::{
    Apply, Element, Task,
    iced::{Alignment, Length, window},
    surface,
    widget::{self, settings},
};
use cosmic_settings_page::{self as page, Section, section};
use cosmic_settings_subscriptions::{pipewire, pulse};
use futures::StreamExt;
use indexmap::IndexMap;
use slab::Slab;
use slotmap::SlotMap;

pub type NodeId = u32;
pub type ProfileId = u32;

#[derive(Clone, Debug)]
pub enum Message {
    /// Get default sinks/sources and their volumes/mute status.
    Pulse(pulse::Event),
    /// Get ALSA cards and their profiles.
    Pipewire(pipewire::DeviceEvent),
    /// Change the default output.
    SinkChanged(usize),
    /// Change the active profile for an output.
    SinkProfileChanged(usize),
    /// Select a device from the given card after a profile change.
    SinkProfileSelect(DeviceId),
    /// Request to change the default output volume.
    SinkVolumeChanged(u32),
    /// Request to change the default output balance.
    SinkBalanceChanged(u32),
    /// Change the output volume.
    SinkVolumeApply(NodeId),
    /// Change the output balance.
    SinkBalanceApply,
    /// Toggle the mute status of the output.
    SinkMuteToggle,
    /// Change the default input output.
    SourceChanged(usize),
    /// Change the active profile for an output.
    SourceProfileChanged(usize),
    /// Select a device from the given card after a profile change.
    SourceProfileSelect(DeviceId),
    /// Request to change the input volume.
    SourceVolumeChanged(u32),
    /// Change the input volume.
    SourceVolumeApply(NodeId),
    /// Toggle the mute status of the input output.
    SourceMuteToggle,
    /// On init of the subscription, channels for closing background threads are given to the app.
    SubHandle(Arc<SubscriptionHandle>),
    /// Surface Action
    Surface(surface::Action),
}

pub struct SubscriptionHandle {
    cancel_tx: futures::channel::oneshot::Sender<()>,
    pipewire: pipewire::Sender<()>,
}

impl std::fmt::Debug for SubscriptionHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SubscriptionHandle")
    }
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

#[derive(Debug)]
struct Card {
    devices: IndexMap<NodeId, Device>,
}

#[derive(Debug)]
struct Device {
    class: pipewire::MediaClass,
    identifier: String,
    description: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum DeviceId {
    Alsa(u32),
    Bluez5(String),
    Unknown(),
}

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
    subscription_handle: Option<SubscriptionHandle>,
    sink_channels: Option<pulse::PulseChannels>,

    devices: BTreeMap<DeviceId, Card>,
    card_names: IndexMap<DeviceId, String>,
    card_profiles: IndexMap<DeviceId, Vec<pulse::CardProfile>>,
    active_profiles: IndexMap<DeviceId, Option<String>>,

    /** Sink devices */

    /// Product names for source sink devices.
    sinks: Vec<String>,
    /// Pipewire object IDs for sink devices.
    sink_pw_ids: Vec<NodeId>,
    /// Profile IDs for the actively-selected sink device.
    sink_profiles: Vec<String>,
    /// Names of profiles for the actively-selected sink device.
    sink_profile_names: Vec<String>,
    /// Device ID of active sink device.
    active_sink_device: Option<DeviceId>,
    /// Index of active sink device.
    active_sink: Option<usize>,
    /// Card profile index of active sink device.
    active_sink_profile: Option<usize>,

    /** Source devices */

    /// Product names for source devices.
    sources: Vec<String>,
    /// Pipewire object IDs for source devices.
    source_pw_ids: Vec<NodeId>,
    /// Profile IDs for the actively-selected source device.
    source_profiles: Vec<String>,
    /// Names of profiles for the actively-selected source device.
    source_profile_names: Vec<String>,
    /// Device ID of active source device.
    active_source_device: Option<DeviceId>,
    /// Index of active source device.
    active_source: Option<usize>,
    /// Card profile index of active source device.
    active_source_profile: Option<usize>,

    /// Device identifier of the default sink.
    default_sink: String,
    /// Device identifier of the default source.
    default_source: String,

    sink_volume_text: String,
    source_volume_text: String,

    sink_balance_text: Option<String>,
    sink_balance: Option<f32>,

    sink_volume: u32,
    source_volume: u32,

    sink_mute: bool,
    sink_volume_debounce: bool,
    sink_balance_debounce: bool,
    source_mute: bool,
    source_volume_debounce: bool,
    changing_sink_profile: bool,
    changing_source_profile: bool,
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(output()), sections.insert(input())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("sound", "preferences-sound-symbolic")
            .title(fl!("sound"))
            .description(fl!("sound", "desc"))
    }

    fn subscription(
        &self,
        _core: &cosmic::Core,
    ) -> cosmic::iced::Subscription<crate::pages::Message> {
        cosmic::iced::Subscription::run(|| {
            async_fn_stream::fn_stream(|emitter| async move {
                let (cancel_tx, mut cancel_rx) = futures::channel::oneshot::channel::<()>();

                let (tx, mut pulse_rx) = futures::channel::mpsc::channel(1);
                let _pulse_handle = std::thread::spawn(move || {
                    pulse::thread(tx);
                });

                let (tx, mut pw_rx) = futures::channel::mpsc::channel(1);
                let (_pipewire_handle, pipewire_terminate) = pipewire::thread(tx);

                emitter
                    .emit(
                        Message::SubHandle(Arc::new(SubscriptionHandle {
                            cancel_tx,
                            pipewire: pipewire_terminate,
                        }))
                        .into(),
                    )
                    .await;

                loop {
                    futures::select! {
                        event = pulse_rx.next() => {
                            let Some(event) = event else {
                                break;
                            };

                            emitter
                                .emit(crate::pages::Message::from(Message::Pulse(event)))
                                .await;
                        }

                        event = pw_rx.next() => {
                            let Some(event) = event else {
                                break;
                            };

                            emitter
                                .emit(crate::pages::Message::from(Message::Pipewire(event)))
                                .await;
                        }

                        _ = cancel_rx => break,
                    }
                }

                drop(pulse_rx);
                drop(pw_rx);

                futures::future::pending::<crate::pages::Message>().await;
            })
        })
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some(handle) = self.subscription_handle.take() {
            _ = handle.cancel_tx.send(());
            _ = handle.pipewire.send(());
        }

        if let Some(channel) = self.sink_channels.take() {
            channel.quit();
        }

        *self = Page {
            entity: self.entity,
            ..Page::default()
        };

        Task::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl Page {
    fn device_profiles(&self, device_id: &DeviceId) -> (Vec<String>, Vec<String>, Option<usize>) {
        let (profiles, profile_descriptions): (Vec<String>, Vec<String>) = self
            .card_profiles
            .get(device_id)
            .map_or((Vec::new(), Vec::new()), |profiles| {
                profiles
                    .iter()
                    .filter(|p| p.available && p.name != "off")
                    .map(|p| (p.name.clone(), p.description.clone()))
                    .collect()
            });

        let active_profile = self.active_profiles.get(device_id).and_then(|profile| {
            profile
                .as_ref()
                .and_then(|profile| profiles.iter().position(|p| p == profile))
        });

        (profiles, profile_descriptions, active_profile)
    }

    fn set_sink_profiles(&mut self, device_id: &DeviceId) {
        (
            self.sink_profile_names,
            self.sink_profiles,
            self.active_sink_profile,
        ) = self.device_profiles(device_id);
    }

    fn set_source_profiles(&mut self, device_id: &DeviceId) {
        (
            self.source_profile_names,
            self.source_profiles,
            self.active_source_profile,
        ) = self.device_profiles(device_id);
    }

    fn set_default_sink(&mut self, sink: String) {
        if self.default_sink == sink {
            return;
        }

        self.default_sink = sink;
        self.active_sink_profile = None;
        self.sink_profiles.clear();
        self.sink_profile_names.clear();

        for (device_id, card) in &self.devices {
            for (&node_id, device) in &card.devices {
                if let pipewire::MediaClass::Sink = device.class {
                    if device.identifier == self.default_sink {
                        self.active_sink = self.sink_pw_ids.iter().position(|&id| id == node_id);
                        let device_id = device_id.clone();
                        self.set_sink_profiles(&device_id);
                        self.active_sink_device = Some(device_id);
                        return;
                    }
                }
            }
        }
    }

    fn set_default_source(&mut self, source: String) {
        if self.default_source == source {
            return;
        }

        self.default_source = source;
        self.active_source_profile = None;
        self.source_profiles.clear();
        self.source_profile_names.clear();

        for (device_id, card) in &self.devices {
            for (&node_id, device) in &card.devices {
                if let pipewire::MediaClass::Source = device.class {
                    if device.identifier == self.default_source {
                        self.active_source =
                            self.source_pw_ids.iter().position(|&id| id == node_id);
                        let device_id = device_id.clone();
                        self.set_source_profiles(&device_id);
                        self.active_source_device = Some(device_id);
                        return;
                    }
                }
            }
        }
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::SourceVolumeChanged(volume) => {
                self.source_volume = volume;
                self.source_volume_text = volume.to_string();
                if self.source_volume_debounce {
                    return Task::none();
                }

                let mut command = None;
                if let Some(&node_id) = self.source_pw_ids.get(self.active_source.unwrap_or(0)) {
                    command = Some(cosmic::Task::future(async move {
                        tokio::time::sleep(Duration::from_millis(64)).await;
                        Message::SourceVolumeApply(node_id).into()
                    }));
                }

                if let Some(command) = command {
                    self.source_volume_debounce = true;
                    return command;
                }
            }
            Message::Pulse(pulse::Event::SourceVolume(volume)) => {
                if self.sink_volume_debounce {
                    return Task::none();
                }

                self.source_volume = volume;
                self.source_volume_text = volume.to_string();
            }
            Message::SinkVolumeChanged(volume) => {
                self.sink_volume = volume;
                self.sink_volume_text = volume.to_string();
                if self.sink_volume_debounce {
                    return Task::none();
                }

                let mut command = None;
                if let Some(&node_id) = self.sink_pw_ids.get(self.active_sink.unwrap_or(0)) {
                    command = Some(cosmic::Task::future(async move {
                        tokio::time::sleep(Duration::from_millis(64)).await;
                        Message::SinkVolumeApply(node_id).into()
                    }));
                }

                if let Some(command) = command {
                    self.sink_volume_debounce = true;
                    return command;
                }
            }
            Message::SinkBalanceChanged(balance) => {
                self.sink_balance = Some((balance as f32 - 100.) / 100.);
                self.sink_balance_text = Some(format!("{balance:.2}"));
                if self.sink_balance_debounce {
                    return Task::none();
                }

                let mut command = None;
                if !self
                    .sink_pw_ids
                    .get(self.active_sink.unwrap_or(0))
                    .is_none()
                {
                    command = Some(cosmic::Task::future(async move {
                        tokio::time::sleep(Duration::from_millis(64)).await;
                        Message::SinkBalanceApply.into()
                    }));
                }

                if let Some(command) = command {
                    self.sink_balance_debounce = true;
                    return command;
                }
            }
            Message::Pulse(pulse::Event::SinkVolume(volume)) => {
                if self.sink_volume_debounce {
                    return Task::none();
                }

                self.sink_volume = volume;
                self.sink_volume_text = volume.to_string();
            }
            Message::Pulse(pulse::Event::DefaultSink(sink)) => {
                if !self.changing_sink_profile {
                    self.set_default_sink(sink);
                }
            }
            Message::Pulse(pulse::Event::DefaultSource(source)) => {
                if !self.changing_source_profile {
                    self.set_default_source(source);
                }
            }
            Message::Pulse(pulse::Event::SinkMute(mute)) => {
                self.sink_mute = mute;
            }
            Message::Pulse(pulse::Event::SourceMute(mute)) => {
                self.source_mute = mute;
            }
            Message::Pulse(pulse::Event::CardInfo(card)) => {
                let device_id = match card.variant {
                    pulse::DeviceVariant::Alsa { alsa_card, .. } => DeviceId::Alsa(alsa_card),
                    pulse::DeviceVariant::Bluez5 { address, .. } => DeviceId::Bluez5(address),
                };

                self.card_names.insert(device_id.clone(), card.name);
                self.card_profiles.insert(device_id.clone(), card.profiles);
                self.active_profiles
                    .insert(device_id, card.active_profile.map(|p| p.name));
            }
            Message::Pulse(pulse::Event::Balance(balance)) => {
                self.sink_balance = balance;
                self.sink_balance_text = balance.map(|b| format!("{b:.2}"));
            }
            Message::Pulse(pulse::Event::Channels(channels)) => {
                self.sink_channels = Some(channels);
            }
            Message::Pipewire(pipewire::DeviceEvent::Add(device)) => {
                let device_id = match device.variant {
                    pipewire::DeviceVariant::Alsa { alsa_card, .. } => DeviceId::Alsa(alsa_card),
                    pipewire::DeviceVariant::Bluez5 { address, .. } => DeviceId::Bluez5(address),
                    pipewire::DeviceVariant::Unknown {} => DeviceId::Unknown {},
                };

                match device.media_class {
                    pipewire::MediaClass::Sink => {
                        self.sinks.push(device.product_name.clone());
                        self.sink_pw_ids.push(device.object_id);

                        sort_pulse_devices(&mut self.sinks, &mut self.sink_pw_ids);

                        if self.default_sink == device.node_name {
                            self.active_sink =
                                self.sinks.iter().position(|s| *s == device.product_name);
                            self.active_sink_device = Some(device_id.clone());
                            self.set_sink_profiles(&device_id);
                        }
                    }

                    pipewire::MediaClass::Source => {
                        self.sources.push(device.product_name.clone());
                        self.source_pw_ids.push(device.object_id);

                        sort_pulse_devices(&mut self.sources, &mut self.source_pw_ids);

                        if self.default_source == device.node_name {
                            self.active_source =
                                self.sources.iter().position(|s| *s == device.product_name);
                            self.active_source_device = Some(device_id.clone());
                            self.set_source_profiles(&device_id);
                        }
                    }
                }

                let card = self.devices.entry(device_id).or_insert_with(|| Card {
                    devices: IndexMap::new(),
                });

                card.devices.insert(
                    device.object_id,
                    Device {
                        class: device.media_class,
                        identifier: device.node_name,
                        description: device.product_name,
                    },
                );

                card.devices
                    .sort_unstable_by(|_, av, _, bv| av.description.cmp(&bv.description));
            }
            Message::Pipewire(pipewire::DeviceEvent::Remove(node_id)) => {
                let mut remove = None;
                for (card_id, card) in &mut self.devices {
                    if card.devices.shift_remove(&node_id).is_some() {
                        if card.devices.is_empty() {
                            remove = Some(card_id.clone());
                        }
                        break;
                    }
                }

                if let Some(card_id) = remove {
                    _ = self.devices.remove(&card_id);
                }

                if let Some(pos) = self.sink_pw_ids.iter().position(|&id| id == node_id) {
                    _ = self.sink_pw_ids.remove(pos);
                    _ = self.sinks.remove(pos);
                    if self.active_sink == Some(pos) {
                        self.active_sink = None;
                        self.active_sink_device = None;
                        self.active_sink_profile = None;
                    }
                } else if let Some(pos) = self.source_pw_ids.iter().position(|&id| id == node_id) {
                    _ = self.source_pw_ids.remove(pos);
                    _ = self.sources.remove(pos);
                    if self.active_source == Some(pos) {
                        self.active_source = None;
                        self.active_source_device = None;
                        self.active_source_profile = None;
                    }
                }
            }
            Message::SinkChanged(pos) => {
                if let Some(node_id) = self.sink_pw_ids.get(pos) {
                    for card in self.devices.values() {
                        for (nid, device) in &card.devices {
                            if node_id == nid {
                                self.active_sink = Some(pos);
                                pactl_set_default_sink(device.identifier.clone());
                                self.set_default_sink(device.identifier.clone());
                                return Task::none();
                            }
                        }
                    }
                }
            }
            Message::SourceChanged(pos) => {
                if let Some(node_id) = self.source_pw_ids.get(pos) {
                    for card in self.devices.values() {
                        for (nid, device) in &card.devices {
                            if node_id == nid {
                                self.active_source = Some(pos);
                                pactl_set_default_source(device.identifier.clone());
                                self.set_default_source(device.identifier.clone());
                                return Task::none();
                            }
                        }
                    }
                }
            }
            Message::SinkBalanceApply => {
                self.sink_balance_debounce = false;
                if let Some((balance, channels)) =
                    self.sink_balance.zip(self.sink_channels.as_mut())
                {
                    channels.set_balance(balance);
                }
            }
            Message::SinkVolumeApply(_) => {
                self.sink_volume_debounce = false;
                if let Some(channels) = self.sink_channels.as_mut() {
                    channels.set_volume(self.sink_volume as f32 / 100.);
                }
            }
            Message::SourceVolumeApply(node_id) => {
                self.source_volume_debounce = false;
                wpctl_set_volume(node_id, self.source_volume);
            }
            Message::SinkMuteToggle => {
                self.sink_mute = !self.sink_mute;
                if let Some(&node_id) = self.sink_pw_ids.get(self.active_sink.unwrap_or(0)) {
                    wpctl_set_mute(node_id, self.sink_mute);
                }
            }
            Message::SourceMuteToggle => {
                self.source_mute = !self.source_mute;
                if let Some(&node_id) = self.source_pw_ids.get(self.active_source.unwrap_or(0)) {
                    wpctl_set_mute(node_id, self.source_mute);
                }
            }
            Message::SinkProfileChanged(profile) => {
                self.active_sink_profile = Some(profile);

                if let Some(profile) = self.sink_profile_names.get(profile).cloned() {
                    if let Some(device_id) = self.active_sink_device.clone() {
                        if let Some(name) = self.card_names.get(&device_id).cloned() {
                            self.active_profiles
                                .insert(device_id.clone(), Some(profile.clone()));

                            self.changing_sink_profile = true;
                            return cosmic::Task::future(async move {
                                pactl_set_card_profile(name, profile).await;
                                Message::SinkProfileSelect(device_id).into()
                            });
                        }
                    }
                }
            }
            Message::SinkProfileSelect(device_id) => {
                self.changing_sink_profile = false;
                let sink_pos = self.active_sink.unwrap_or(0);

                if let Some(card) = self.devices.get(&device_id) {
                    if let Some((_, device)) = card.devices.get_index(sink_pos) {
                        pactl_set_default_sink(device.identifier.clone());
                        self.set_default_sink(device.identifier.clone());
                    }
                }
            }
            Message::SourceProfileChanged(profile) => {
                self.active_source_profile = Some(profile);
                if let Some(profile) = self.source_profile_names.get(profile).cloned() {
                    if let Some(device_id) = self.active_source_device.clone() {
                        if let Some(name) = self.card_names.get(&device_id).cloned() {
                            self.active_profiles
                                .insert(device_id.clone(), Some(profile.clone()));

                            self.changing_source_profile = true;
                            return cosmic::Task::future(async move {
                                pactl_set_card_profile(name, profile).await;
                                Message::SourceProfileSelect(device_id).into()
                            });
                        }
                    }
                }
            }
            Message::SourceProfileSelect(device_id) => {
                self.changing_source_profile = false;
                let source_pos = self.active_source.unwrap_or(0);

                if let Some(card) = self.devices.get(&device_id) {
                    if let Some((_, device)) = card.devices.get_index(source_pos) {
                        pactl_set_default_source(device.identifier.clone());
                        self.set_default_source(device.identifier.clone());
                    }
                }
            }
            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }

            Message::SubHandle(handle) => {
                if let Some(handle) = Arc::into_inner(handle) {
                    self.subscription_handle = Some(handle);
                }
            }
        }
        Task::none()
    }
}

fn input() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let volume = descriptions.insert(fl!("sound-input", "volume"));
    let device = descriptions.insert(fl!("sound-input", "device"));
    let _level = descriptions.insert(fl!("sound-input", "level"));
    let profile = descriptions.insert(fl!("profile"));

    Section::default()
        .title(fl!("sound-input"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            if page.sources.is_empty() {
                return widget::row().into();
            }

            let volume_control = widget::row::with_capacity(4)
                .align_y(Alignment::Center)
                .push(
                    widget::button::icon(widget::icon::from_name(if page.source_mute {
                        "microphone-sensitivity-muted-symbolic"
                    } else {
                        "audio-input-microphone-symbolic"
                    }))
                    .on_press(Message::SourceMuteToggle.into()),
                )
                .push(
                    widget::text::body(&page.source_volume_text)
                        .width(Length::Fixed(22.0))
                        .align_x(Alignment::Center),
                )
                .push(widget::horizontal_space().width(8))
                .push(
                    widget::slider(0..=150, page.source_volume, |change| {
                        Message::SourceVolumeChanged(change).into()
                    })
                    .breakpoints(&[100]),
                );
            let devices = widget::dropdown::popup_dropdown(
                &page.sources,
                Some(page.active_source.unwrap_or(0)),
                Message::SourceChanged,
                window::Id::RESERVED,
                Message::Surface,
                |a| crate::Message::from(a),
            )
            .apply(Element::from)
            .map(crate::pages::Message::from);

            let mut controls = settings::section()
                .title(&section.title)
                .add(settings::flex_item(
                    &*section.descriptions[volume],
                    volume_control,
                ))
                .add(settings::item(&*section.descriptions[device], devices));

            if !page.source_profiles.is_empty() {
                let dropdown = widget::dropdown::popup_dropdown(
                    &page.source_profiles,
                    page.active_source_profile,
                    Message::SourceProfileChanged,
                    window::Id::RESERVED,
                    Message::Surface,
                    |a| crate::Message::from(a),
                )
                .apply(Element::from)
                .map(crate::pages::Message::from);

                controls = controls.add(settings::item(&*section.descriptions[profile], dropdown));
            }

            Element::from(controls)
        })
}

fn output() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let volume = descriptions.insert(fl!("sound-output", "volume"));
    let device = descriptions.insert(fl!("sound-output", "device"));
    let _level = descriptions.insert(fl!("sound-output", "level"));
    let profile = descriptions.insert(fl!("profile"));
    let balance = descriptions.insert(fl!("sound-output", "balance"));
    let left = descriptions.insert(fl!("sound-output", "left"));
    let right = descriptions.insert(fl!("sound-output", "right"));
    // let balance = descriptions.insert(fl!("sound-output", "balance"));

    Section::default()
        .title(fl!("sound-output"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let volume_control = widget::row::with_capacity(4)
                .align_y(Alignment::Center)
                .push(
                    widget::button::icon(if page.sink_mute {
                        widget::icon::from_name("audio-volume-muted-symbolic")
                    } else {
                        widget::icon::from_name("audio-volume-high-symbolic")
                    })
                    .on_press(Message::SinkMuteToggle.into()),
                )
                .push(
                    widget::text::body(&page.sink_volume_text)
                        .width(Length::Fixed(22.0))
                        .align_x(Alignment::Center),
                )
                .push(widget::horizontal_space().width(8))
                .push(
                    widget::slider(0..=150, page.sink_volume, |change| {
                        Message::SinkVolumeChanged(change).into()
                    })
                    .breakpoints(&[100]),
                );

            let devices = widget::dropdown::popup_dropdown(
                &page.sinks,
                Some(page.active_sink.unwrap_or(0)),
                Message::SinkChanged,
                window::Id::RESERVED,
                Message::Surface,
                |a| crate::Message::from(a),
            )
            .apply(Element::from)
            .map(crate::pages::Message::from);

            let mut controls = settings::section()
                .title(&section.title)
                .add(settings::flex_item(
                    &*section.descriptions[volume],
                    volume_control,
                ))
                .add(settings::item(&*section.descriptions[device], devices));

            if !page.sink_profiles.is_empty() {
                let dropdown = widget::dropdown::popup_dropdown(
                    &page.sink_profiles,
                    page.active_sink_profile,
                    Message::SinkProfileChanged,
                    window::Id::RESERVED,
                    Message::Surface,
                    |a| crate::Message::from(a),
                )
                .apply(Element::from)
                .map(crate::pages::Message::from);

                controls = controls.add(settings::item(&*section.descriptions[profile], dropdown));
            }
            if let Some(sink_balance) = page.sink_balance {
                controls = controls.add(settings::item(
                    &*section.descriptions[balance],
                    widget::row::with_capacity(4)
                        .align_y(Alignment::Center)
                        .push(
                            widget::text::body(&*section.descriptions[left])
                                .width(Length::Fixed(22.0))
                                .align_x(Alignment::Center),
                        )
                        .push(widget::horizontal_space().width(8))
                        .push(
                            widget::slider(
                                0..=200,
                                ((sink_balance + 1.).max(0.) * 100.).round() as u32,
                                |change| Message::SinkBalanceChanged(change).into(),
                            )
                            .breakpoints(&[100]),
                        )
                        .push(widget::horizontal_space().width(8))
                        .push(
                            widget::text::body(&*section.descriptions[right])
                                .width(Length::Fixed(22.0))
                                .align_x(Alignment::Center),
                        ),
                ));
            }

            Element::from(controls)
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

fn sort_pulse_devices(descriptions: &mut Vec<String>, node_ids: &mut Vec<NodeId>) {
    let mut tmp: Vec<(String, NodeId)> = std::mem::take(descriptions)
        .into_iter()
        .zip(std::mem::take(node_ids))
        .collect();

    tmp.sort_unstable_by(|(ak, _), (bk, _)| ak.cmp(bk));

    (*descriptions, *node_ids) = tmp.into_iter().collect();
}

async fn pactl_set_card_profile(id: String, profile: String) {
    _ = tokio::process::Command::new("pactl")
        .args(["set-card-profile", id.as_str(), profile.as_str()])
        .status()
        .await
}

fn pactl_set_default_sink(id: String) {
    tokio::task::spawn(async move {
        _ = tokio::process::Command::new("pactl")
            .args(["set-default-sink", id.as_str()])
            .status()
            .await;
    });
}

fn pactl_set_default_source(id: String) {
    tokio::task::spawn(async move {
        _ = tokio::process::Command::new("pactl")
            .args(["set-default-source", id.as_str()])
            .status()
            .await;
    });
}

fn wpctl_set_mute(id: u32, mute: bool) {
    tokio::task::spawn(async move {
        let default = id.to_string();
        _ = tokio::process::Command::new("wpctl")
            .args(["set-mute", default.as_str(), if mute { "1" } else { "0" }])
            .status()
            .await;
    });
}

fn wpctl_set_volume(id: u32, volume: u32) {
    tokio::task::spawn(async move {
        let id = id.to_string();
        let volume = format!("{}.{:02}", volume / 100, volume % 100);
        _ = tokio::process::Command::new("wpctl")
            .args(["set-volume", id.as_str(), volume.as_str()])
            .status()
            .await;
    });
}
