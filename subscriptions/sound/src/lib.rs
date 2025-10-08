// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

pub mod pipewire;
pub mod pulse;

use cosmic::Task;
use cosmic::iced_futures::MaybeSend;
use futures::{Stream, StreamExt};
use indexmap::IndexMap;
use std::{collections::BTreeMap, sync::Arc, time::Duration};

pub type NodeId = u32;
pub type ProfileId = u32;

pub fn watch() -> impl Stream<Item = Message> + MaybeSend + 'static {
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

        let mut pulse_channels = None;
        let mut balance = None;
        let mut source_volume = None;
        let mut sink_volume = None;
        let mut events = Vec::new();
        let mut timer = tokio::time::interval(Duration::from_millis(64));

        loop {
            tokio::select! {
                event = pulse_rx.next() => {
                    let Some(event) = event else {
                        break;
                    };

                    match event {
                        pulse::Event::Channels(channels) => pulse_channels = Some(channels),
                        pulse::Event::SinkVolume(volume) => sink_volume = Some(volume),
                        pulse::Event::SourceVolume(volume) => source_volume = Some(volume),
                        pulse::Event::Balance(value) => balance = Some(value),
                        _ => {
                            events.push(Server::Pulse(event));
                            timer.reset();
                        }
                    }
                }

                event = pw_rx.next() => {
                    let Some(event) = event else {
                        break;
                    };

                    timer.reset();
                    events.push(Server::Pipewire(event));
                }

                _ = timer.tick() => {
                    if let Some(channels) = pulse_channels.take() {
                        events.push(Server::Pulse(pulse::Event::Channels(channels)));
                    }

                    if let Some(volume) = sink_volume.take() {
                        events.push(Server::Pulse(pulse::Event::SinkVolume(volume)));
                    }

                    if let Some(volume) = source_volume.take() {
                        events.push(Server::Pulse(pulse::Event::SourceVolume(volume)));
                    }

                    if let Some(balance) = balance.take() {
                        events.push(Server::Pulse(pulse::Event::Balance(balance)));
                    }

                    if !events.is_empty() {
                        emitter
                            .emit(Message::Server(Arc::from(std::mem::take(&mut events))))
                            .await;
                    }
                }

                _ = &mut cancel_rx => break,
            }
        }

        drop(pulse_rx);
        drop(pw_rx);

        futures::future::pending::<Message>().await;
    })
}

#[derive(Default)]
pub struct Model {
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

    pub sink_volume_text: String,
    pub source_volume_text: String,

    pub sink_balance_text: Option<String>,
    pub sink_balance: Option<f32>,

    pub sink_volume: u32,
    pub source_volume: u32,

    pub sink_mute: bool,
    sink_volume_debounce: bool,
    sink_balance_debounce: bool,
    pub source_mute: bool,
    source_volume_debounce: bool,

    changing_sink_profile: Option<DeviceId>,
    changing_source_profile: Option<DeviceId>,
}

impl Model {
    pub fn active_sink(&self) -> Option<usize> {
        self.active_sink
    }

    pub fn active_sink_profile(&self) -> Option<usize> {
        self.active_sink_profile
    }

    pub fn active_source(&self) -> Option<usize> {
        self.active_source
    }

    pub fn active_source_profile(&self) -> Option<usize> {
        self.active_source_profile
    }

    pub fn sinks(&self) -> &[String] {
        &self.sinks
    }

    pub fn sink_profiles(&self) -> &[String] {
        &self.sink_profiles
    }

    pub fn sources(&self) -> &[String] {
        &self.sources
    }

    pub fn source_profiles(&self) -> &[String] {
        &self.source_profiles
    }

    pub fn clear(&mut self) {
        if let Some(handle) = self.subscription_handle.take() {
            _ = handle.cancel_tx.send(());
            _ = handle.pipewire.send(());
        }

        if let Some(channel) = self.sink_channels.take() {
            channel.quit();
        }
    }

    pub fn sink_balance_changed(&mut self, balance: u32) -> Task<Message> {
        self.sink_balance = Some((balance as f32 - 100.) / 100.);
        self.sink_balance_text = Some(format!("{balance:.2}"));
        if self.sink_balance_debounce {
            return Task::none();
        }

        if !self
            .sink_pw_ids
            .get(self.active_sink.unwrap_or(0))
            .is_none()
        {
            self.sink_balance_debounce = true;
            return cosmic::Task::future(async move {
                tokio::time::sleep(Duration::from_millis(64)).await;
                Message::SinkBalanceApply.into()
            });
        }

        Task::none()
    }

    pub fn sink_changed(&mut self, pos: usize) -> Task<Message> {
        if let Some(&node_id) = self.sink_pw_ids.get(pos) {
            for card in self.devices.values() {
                for (&nid, port) in &card.ports {
                    if node_id == nid {
                        self.active_sink = Some(pos);
                        let identifier = port.identifier.clone();
                        return cosmic::Task::future(async move {
                            wpctl_set_default(nid).await;
                            Message::SetDefaultSink(identifier).into()
                        });
                    }
                }
            }
        }

        Task::none()
    }

    pub fn sink_mute_toggle(&mut self) {
        self.sink_mute = !self.sink_mute;
        if let Some(&node_id) = self.sink_pw_ids.get(self.active_sink.unwrap_or(0)) {
            wpctl_set_mute(node_id, self.sink_mute);
        }
    }

    pub fn sink_profile_changed(&mut self, profile: usize) -> Task<Message> {
        self.active_sink_profile = Some(profile);

        if let Some(profile) = self.sink_profile_names.get(profile).cloned() {
            if let Some(device_id) = self.active_sink_device.clone() {
                if let Some(name) = self.card_names.get(&device_id).cloned() {
                    self.active_profiles
                        .insert(device_id.clone(), Some(profile.clone()));

                    self.changing_sink_profile = Some(device_id);
                    return cosmic::Task::future(async move {
                        pactl_set_card_profile(name, profile).await;
                    })
                    .discard();
                }
            }
        }

        Task::none()
    }

    pub fn sink_volume_changed(&mut self, volume: u32) -> Task<Message> {
        self.sink_volume = volume;
        self.sink_volume_text = volume.to_string();
        if self.sink_volume_debounce {
            return Task::none();
        }

        if let Some(&node_id) = self.sink_pw_ids.get(self.active_sink.unwrap_or(0)) {
            self.sink_volume_debounce = true;
            return cosmic::Task::future(async move {
                tokio::time::sleep(Duration::from_millis(64)).await;
                Message::SinkVolumeApply(node_id).into()
            });
        }

        Task::none()
    }

    pub fn source_changed(&mut self, pos: usize) -> Task<Message> {
        if let Some(&node_id) = self.source_pw_ids.get(pos) {
            for card in self.devices.values() {
                for (&nid, port) in &card.ports {
                    if node_id == nid {
                        self.active_source = Some(pos);
                        let identifier = port.identifier.clone();
                        return cosmic::Task::future(async move {
                            wpctl_set_default(nid).await;
                            Message::SetDefaultSource(identifier).into()
                        });
                    }
                }
            }
        }

        Task::none()
    }

    pub fn source_mute_toggle(&mut self) {
        self.source_mute = !self.source_mute;
        if let Some(&node_id) = self.source_pw_ids.get(self.active_source.unwrap_or(0)) {
            wpctl_set_mute(node_id, self.source_mute);
        }
    }

    pub fn source_profile_changed(&mut self, profile: usize) -> Task<Message> {
        self.active_source_profile = Some(profile);
        if let Some(profile) = self.source_profile_names.get(profile).cloned() {
            if let Some(device_id) = self.active_source_device.clone() {
                if let Some(name) = self.card_names.get(&device_id).cloned() {
                    self.active_profiles
                        .insert(device_id.clone(), Some(profile.clone()));

                    self.changing_source_profile = Some(device_id.clone());
                    return cosmic::Task::future(async move {
                        pactl_set_card_profile(name, profile).await;
                    })
                    .discard();
                }
            }
        }

        Task::none()
    }

    pub fn source_volume_changed(&mut self, volume: u32) -> Task<Message> {
        self.source_volume = volume;
        self.source_volume_text = volume.to_string();
        if self.source_volume_debounce {
            return Task::none();
        }

        if let Some(&node_id) = self.source_pw_ids.get(self.active_source.unwrap_or(0)) {
            self.source_volume_debounce = true;
            return cosmic::Task::future(async move {
                tokio::time::sleep(Duration::from_millis(64)).await;
                Message::SourceVolumeApply(node_id).into()
            });
        }

        Task::none()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Server(events) => {
                for event in Arc::into_inner(events).into_iter().flatten() {
                    match event {
                        Server::Pulse(event) => match event {
                            pulse::Event::SourceVolume(volume) => {
                                if self.sink_volume_debounce {
                                    return Task::none();
                                }

                                self.source_volume = volume;
                                self.source_volume_text = volume.to_string();
                            }

                            pulse::Event::SinkVolume(volume) => {
                                if self.sink_volume_debounce {
                                    return Task::none();
                                }

                                self.sink_volume = volume;
                                self.sink_volume_text = volume.to_string();
                            }

                            pulse::Event::CardInfo(card) => {
                                let device_id = match card.variant {
                                    pulse::DeviceVariant::Alsa { alsa_card, .. } => {
                                        DeviceId::Alsa(alsa_card)
                                    }
                                    pulse::DeviceVariant::Bluez5 { address, .. } => {
                                        DeviceId::Bluez5(address)
                                    }
                                };

                                eprintln!(
                                    "inserting card {:?}: name={}, active_profile={:?}, profiles={:?}",
                                    device_id,
                                    card.name,
                                    card.active_profile.as_ref().map(|p| p.name.as_str()),
                                    card.profiles
                                );

                                self.card_names.insert(device_id.clone(), card.name);
                                self.card_profiles.insert(device_id.clone(), card.profiles);
                                self.active_profiles
                                    .insert(device_id, card.active_profile.map(|p| p.name));
                            }

                            pulse::Event::DefaultSink(sink) => {
                                if !self.changing_sink_profile.is_some() {
                                    self.set_default_sink(sink);
                                }
                            }
                            pulse::Event::DefaultSource(source) => {
                                if !self.changing_source_profile.is_some() {
                                    self.set_default_source(source);
                                }
                            }
                            pulse::Event::SinkMute(mute) => {
                                self.sink_mute = mute;
                            }
                            pulse::Event::SourceMute(mute) => {
                                self.source_mute = mute;
                            }
                            pulse::Event::Balance(balance) => {
                                self.sink_balance = balance;
                                self.sink_balance_text = balance.map(|b| format!("{b:.2}"));
                            }
                            pulse::Event::Channels(channels) => {
                                self.sink_channels = Some(channels);
                            }
                        },

                        Server::Pipewire(event) => match event {
                            pipewire::DeviceEvent::Add(device) => {
                                let device_id = match device.variant {
                                    pipewire::DeviceVariant::Alsa { alsa_card, .. } => {
                                        DeviceId::Alsa(alsa_card)
                                    }
                                    pipewire::DeviceVariant::Bluez5 { address, .. } => {
                                        DeviceId::Bluez5(address)
                                    }
                                    pipewire::DeviceVariant::Unknown {} => DeviceId::Unknown {},
                                };

                                match device.media_class {
                                    pipewire::MediaClass::Sink => {
                                        self.sinks.push(device.product_name.clone());
                                        self.sink_pw_ids.push(device.object_id);

                                        sort_pulse_devices(&mut self.sinks, &mut self.sink_pw_ids);

                                        if self.default_sink == device.node_name {
                                            self.active_sink_device = Some(device_id.clone());
                                            self.active_sink = self
                                                .sinks
                                                .iter()
                                                .position(|s| *s == device.product_name);
                                            self.set_sink_profiles(&device_id);
                                        }
                                    }

                                    pipewire::MediaClass::Source => {
                                        self.sources.push(device.product_name.clone());
                                        self.source_pw_ids.push(device.object_id);

                                        sort_pulse_devices(
                                            &mut self.sources,
                                            &mut self.source_pw_ids,
                                        );

                                        if self.default_source == device.node_name {
                                            self.active_source = self
                                                .sources
                                                .iter()
                                                .position(|s| *s == device.product_name);
                                            self.active_source_device = Some(device_id.clone());
                                            self.set_source_profiles(&device_id);
                                        }
                                    }
                                }

                                let card = self.devices.entry(device_id).or_insert_with(|| Card {
                                    ports: IndexMap::new(),
                                });

                                card.ports.insert(
                                    device.object_id,
                                    CardPort {
                                        class: device.media_class,
                                        identifier: device.node_name,
                                        description: device.product_name,
                                    },
                                );

                                card.ports.sort_unstable_by(|_, av, _, bv| {
                                    av.description.cmp(&bv.description)
                                });
                            }

                            pipewire::DeviceEvent::Remove(node_id) => {
                                let mut remove = None;
                                for (card_id, card) in &mut self.devices {
                                    if card.ports.shift_remove(&node_id).is_some() {
                                        if card.ports.is_empty() {
                                            remove = Some(card_id.clone());
                                        }
                                        break;
                                    }
                                }

                                if let Some(card_id) = remove {
                                    _ = self.devices.remove(&card_id);
                                }

                                if let Some(pos) =
                                    self.sink_pw_ids.iter().position(|&id| id == node_id)
                                {
                                    _ = self.sink_pw_ids.remove(pos);
                                    _ = self.sinks.remove(pos);
                                    if self.active_sink == Some(pos) {
                                        self.active_sink = None;
                                        self.active_sink_device = None;
                                        self.active_sink_profile = None;
                                    } else {
                                        self.active_sink = self.active_sink.map(|active_pos| {
                                            if active_pos > pos {
                                                active_pos - 1
                                            } else {
                                                active_pos
                                            }
                                        });
                                    }
                                } else if let Some(pos) =
                                    self.source_pw_ids.iter().position(|&id| id == node_id)
                                {
                                    _ = self.source_pw_ids.remove(pos);
                                    _ = self.sources.remove(pos);
                                    if self.active_source == Some(pos) {
                                        self.active_source = None;
                                        self.active_source_device = None;
                                        self.active_source_profile = None;
                                    }
                                }
                            }
                        },
                    }
                }

                let mut tasks = Task::none();

                if let Some(device_id) = self.changing_sink_profile.take() {
                    tasks = tasks.chain(self.sink_profile_select(device_id));
                }

                if let Some(device_id) = self.changing_source_profile.take() {
                    tasks = tasks.chain(self.source_profile_select(device_id));
                }

                return tasks;
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

            Message::SetDefaultSink(identifier) => self.set_default_sink(identifier),

            Message::SetDefaultSource(identifier) => self.set_default_source(identifier),

            Message::SubHandle(handle) => {
                if let Some(handle) = Arc::into_inner(handle) {
                    self.subscription_handle = Some(handle);
                }
            }
        }

        Task::none()
    }

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

    /// Update the state of the default sink and its profiles.
    fn set_default_sink(&mut self, sink: String) {
        if self.default_sink == sink {
            return;
        }

        self.default_sink = sink;

        for (device_id, card) in &self.devices {
            for (&node_id, card_port) in &card.ports {
                if let pipewire::MediaClass::Sink = card_port.class {
                    if &card_port.identifier == &self.default_sink {
                        let device_id = device_id.clone();
                        self.set_sink_profiles(&device_id);
                        self.active_sink = self.sink_pw_ids.iter().position(|&id| id == node_id);
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

        for (device_id, card) in &self.devices {
            for (&node_id, card_ports) in &card.ports {
                if let pipewire::MediaClass::Source = card_ports.class {
                    if card_ports.identifier == self.default_source {
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

    fn sink_profile_select(&mut self, device_id: DeviceId) -> Task<Message> {
        let sink_pos = self.active_sink.unwrap_or(0);
        if let Some(card) = self.devices.get(&device_id) {
            if let Some((&nid, port)) = card.ports.get_index(sink_pos) {
                let identifier = port.identifier.clone();
                return cosmic::Task::future(async move {
                    wpctl_set_default(nid).await;
                    Message::SetDefaultSink(identifier)
                });
            }
        }

        Task::none()
    }

    fn source_profile_select(&mut self, device_id: DeviceId) -> Task<Message> {
        self.changing_source_profile = None;
        let source_pos = self.active_source.unwrap_or(0);

        if let Some(card) = self.devices.get(&device_id) {
            if let Some((&nid, port)) = card.ports.get_index(source_pos) {
                let identifier = port.identifier.clone();
                return cosmic::Task::future(async move {
                    wpctl_set_default(nid).await;
                    Message::SetDefaultSource(identifier)
                });
            }
        }

        Task::none()
    }
}

#[derive(Debug)]
struct Card {
    ports: IndexMap<NodeId, CardPort>,
}

#[derive(Debug)]
struct CardPort {
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

#[derive(Clone, Debug)]
pub enum Message {
    /// Handle messages from the sound server.
    Server(Arc<Vec<Server>>),
    /// Set the default sink.
    SetDefaultSink(String),
    /// Set the default source.
    SetDefaultSource(String),
    /// Change the output volume.
    SinkVolumeApply(NodeId),
    /// Change the output balance.
    SinkBalanceApply,
    /// Change the input volume.
    SourceVolumeApply(NodeId),
    /// On init of the subscription, channels for closing background threads are given to the app.
    SubHandle(Arc<SubscriptionHandle>),
}

#[derive(Clone, Debug)]
pub enum Server {
    /// Get default sinks/sources and their volumes/mute status.
    Pulse(pulse::Event),
    /// Get ALSA cards and their profiles.
    Pipewire(pipewire::DeviceEvent),
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

fn sort_pulse_devices(descriptions: &mut Vec<String>, node_ids: &mut Vec<NodeId>) {
    let mut tmp: Vec<(String, NodeId)> = std::mem::take(descriptions)
        .into_iter()
        .zip(std::mem::take(node_ids))
        .collect();

    tmp.sort_unstable_by(|(ak, _), (bk, _)| ak.cmp(bk));

    (*descriptions, *node_ids) = tmp.into_iter().collect();
}

async fn pactl_set_card_profile(id: String, profile: String) {
    tracing::debug!("pactl set-card-profile {id} {profile}");
    _ = tokio::process::Command::new("pactl")
        .args(["set-card-profile", id.as_str(), profile.as_str()])
        .status()
        .await
}

async fn wpctl_set_default(id: u32) {
    tracing::debug!("wpctl set-default {id}");
    let id = id.to_string();
    _ = tokio::process::Command::new("wpctl")
        .args(["set-default", id.as_str()])
        .status()
        .await;
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
