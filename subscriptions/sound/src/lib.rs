// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

pub mod pipewire;
pub mod pulse;
mod wpctl;

use cosmic::Task;
use cosmic::iced_futures::MaybeSend;
use futures::{Stream, StreamExt};
use intmap::IntMap;
use std::{sync::Arc, time::Duration};

pub type DeviceId = u32;
pub type NodeId = u32;
pub type ProfileId = u32;

pub fn watch() -> impl Stream<Item = Message> + MaybeSend + 'static {
    async_fn_stream::fn_stream(|emitter| async move {
        let (cancel_tx, mut cancel_rx) = futures::channel::oneshot::channel::<()>();

        let (tx, pulse_rx) = futures::channel::mpsc::channel(1);
        let _pulse_handle = std::thread::spawn(move || {
            pulse::thread(tx);
        });

        let (tx, pw_rx) = futures::channel::mpsc::channel(1);
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

        enum Variant {
            Pulse(pulse::Event),
            Pipewire(pipewire::Event),
        }

        let mut stream =
            futures::stream::select(pulse_rx.map(Variant::Pulse), pw_rx.map(Variant::Pipewire));

        loop {
            tokio::select! {
                event = stream.next() => {
                    let Some(event) = event else {
                        break;
                    };


                    match event {
                        Variant::Pulse(event) => match event {
                            pulse::Event::Channels(channels) => pulse_channels = Some(channels),
                            pulse::Event::SinkVolume(volume) => sink_volume = Some(volume),
                            pulse::Event::SourceVolume(volume) => source_volume = Some(volume),
                            pulse::Event::Balance(value) => balance = Some(value),
                            _ => {
                                events.push(Server::Pulse(event));
                                timer.reset();
                            }
                        }

                        Variant::Pipewire(event) => {
                            events.push(Server::Pipewire(event));
                            timer.reset();
                        }
                    }
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

        drop(stream);
        futures::future::pending::<Message>().await;
    })
}

#[derive(Default)]
pub struct Model {
    subscription_handle: Option<SubscriptionHandle>,
    sink_channels: Option<pulse::PulseChannels>,

    device_ids: IntMap<NodeId, DeviceId>,
    pub node_names: IntMap<NodeId, String>,
    pub node_descriptions: IntMap<NodeId, String>,
    pub device_names: IntMap<DeviceId, String>,
    pub device_profiles: IntMap<DeviceId, Vec<pipewire::Profile>>,
    pub active_profiles: IntMap<DeviceId, pipewire::Profile>,

    /** Sink devices */

    /// Product names for source sink devices.
    sinks: Vec<String>,
    /// Pipewire object IDs for sink devices.
    sink_pw_ids: Vec<NodeId>,
    /// Device ID of active sink device.
    active_sink_device: Option<u32>,
    /// Index of active sink device.
    active_sink: Option<usize>,
    /// Card profile index of active sink device.
    active_sink_profile: Option<usize>,

    /** Source devices */

    /// Product names for source devices.
    sources: Vec<String>,
    /// Pipewire object IDs for source devices.
    source_pw_ids: Vec<NodeId>,
    /// Device ID of active source device.
    active_source_device: Option<u32>,
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

    pub fn sources(&self) -> &[String] {
        &self.sources
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

    /// Sets and applies a profile to a device with wpctl.
    ///
    /// Requires using the device ID rather than a node ID.
    pub fn set_profile(&mut self, device_id: DeviceId, pos: usize) {
        if let Some(profiles) = self.device_profiles.get(device_id) {
            if let Some(profile) = profiles.get(pos) {
                let index = profile.index as u32;
                tokio::spawn(async move {
                    wpctl::set_profile(device_id, index).await;
                });

                self.active_profiles.insert(device_id, profile.clone());

                if self.active_sink_device == Some(device_id) {
                    self.active_sink_profile = Some(pos)
                } else if self.active_source_device == Some(device_id) {
                    self.active_source_profile = Some(pos);
                }
            }
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
        if let Some(&object_id) = self.sink_pw_ids.get(pos) {
            self.set_default_sink_id(object_id);
            tokio::task::spawn(async move {
                wpctl::set_default(object_id).await;
            });
        }

        Task::none()
    }

    pub fn sink_mute_toggle(&mut self) {
        self.sink_mute = !self.sink_mute;
        if let Some(&node_id) = self.sink_pw_ids.get(self.active_sink.unwrap_or(0)) {
            let mute = self.sink_mute;
            tokio::task::spawn(async move {
                wpctl::set_mute(node_id, mute).await;
            });
        }
    }

    pub fn sink_volume_changed(&mut self, volume: u32) -> Task<Message> {
        self.sink_volume = volume;
        self.sink_volume_text = numtoa::BaseN::<10>::u32(volume).as_str().to_owned();
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
        if let Some(&object_id) = self.source_pw_ids.get(pos) {
            self.set_default_source_id(object_id);
            tokio::task::spawn(async move {
                wpctl::set_default(object_id).await;
            });
        }

        Task::none()
    }

    pub fn source_mute_toggle(&mut self) {
        self.source_mute = !self.source_mute;
        if let Some(&node_id) = self.source_pw_ids.get(self.active_source.unwrap_or(0)) {
            let mute = self.source_mute;
            tokio::task::spawn(async move {
                wpctl::set_mute(node_id, mute).await;
            });
        }
    }

    pub fn source_volume_changed(&mut self, volume: u32) -> Task<Message> {
        self.source_volume = volume;
        self.source_volume_text = numtoa::BaseN::<10>::u32(volume).as_str().to_owned();
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
                'outer: for event in Arc::into_inner(events).into_iter().flatten() {
                    match event {
                        Server::Pulse(event) => match event {
                            pulse::Event::SourceVolume(volume) => {
                                if self.sink_volume_debounce {
                                    continue;
                                }

                                self.source_volume = volume;
                                self.source_volume_text =
                                    numtoa::BaseN::<10>::u32(volume).as_str().to_owned();
                            }

                            pulse::Event::SinkVolume(volume) => {
                                if self.sink_volume_debounce {
                                    continue;
                                }

                                self.sink_volume = volume;
                                self.sink_volume_text =
                                    numtoa::BaseN::<10>::u32(volume).as_str().to_owned();
                            }

                            pulse::Event::DefaultSink(node_name) => {
                                if self.default_sink == node_name {
                                    continue;
                                }

                                if let Some(id) = self.node_id_from_name(&node_name) {
                                    self.set_default_sink_id(id);
                                }

                                self.default_sink = node_name;
                            }
                            pulse::Event::DefaultSource(node_name) => {
                                if self.default_source == node_name {
                                    continue;
                                }

                                if let Some(id) = self.node_id_from_name(&node_name) {
                                    self.set_default_source_id(id);
                                }

                                self.default_source = node_name;
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
                            pipewire::Event::ActiveProfile(id, profile) => {
                                self.active_profiles.insert(id, profile);
                            }

                            pipewire::Event::AddProfile(id, profile) => {
                                let profiles = self.device_profiles.entry(id).or_default();
                                for p in profiles.iter_mut() {
                                    if p.index == profile.index {
                                        *p = profile;
                                        continue 'outer;
                                    }
                                }

                                profiles.push(profile);
                            }

                            pipewire::Event::AddDevice(device) => {
                                self.device_names.insert(device.id, device.name);
                            }

                            pipewire::Event::AddNode(node) => {
                                let object_id = node.object_id;
                                if !self.node_names.contains_key(object_id) {
                                    match node.media_class {
                                        pipewire::MediaClass::Sink => {
                                            self.sinks.push(node.description.clone());
                                            self.sink_pw_ids.push(node.object_id);

                                            sort_devices(&mut self.sinks, &mut self.sink_pw_ids);

                                            if self.default_sink == node.node_name {
                                                self.set_default_sink_id(node.object_id);
                                            }
                                        }

                                        pipewire::MediaClass::Source => {
                                            self.sources.push(node.description.clone());
                                            self.source_pw_ids.push(node.object_id);

                                            sort_devices(
                                                &mut self.sources,
                                                &mut self.source_pw_ids,
                                            );

                                            if self.default_source == node.node_name {
                                                self.set_default_source_id(object_id);
                                            }
                                        }
                                    }

                                    if let Some(device_id) = node.device_id {
                                        self.device_ids.insert(object_id, device_id);
                                    }

                                    self.node_names.insert(object_id, node.node_name);
                                    self.node_descriptions.insert(object_id, node.description);
                                }

                                if node.device_id.is_some() {
                                    if self.active_sink_device == node.device_id {
                                        self.set_default_sink_id(object_id);
                                        tokio::task::spawn(async move {
                                            wpctl::set_default(object_id).await;
                                        });
                                    } else if self.active_source_device == node.device_id {
                                        self.set_default_source_id(object_id);
                                        tokio::task::spawn(async move {
                                            wpctl::set_default(object_id).await;
                                        });
                                    }
                                }
                            }

                            pipewire::Event::RemoveDevice(id) => self.remove_device(id),
                            pipewire::Event::RemoveNode(id) => self.remove_node(id),
                        },
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

            Message::SinkVolumeApply(node_id) => {
                self.sink_volume_debounce = false;
                let volume = self.sink_volume;
                tokio::task::spawn(async move {
                    wpctl::set_volume(node_id, volume).await;
                });
            }

            Message::SourceVolumeApply(node_id) => {
                self.source_volume_debounce = false;
                let volume = self.source_volume;
                tokio::task::spawn(async move {
                    wpctl::set_volume(node_id, volume).await;
                });
            }

            Message::SubHandle(handle) => {
                if let Some(handle) = Arc::into_inner(handle) {
                    self.subscription_handle = Some(handle);
                }
            }
        }

        Task::none()
    }

    fn node_id_from_name(&self, name: &str) -> Option<u32> {
        self.node_names
            .iter()
            .find(|&(_, n)| *n == name)
            .map(|(id, _)| id)
    }

    fn remove_device(&mut self, id: DeviceId) {
        _ = self.device_names.remove(id);
        _ = self.device_profiles.remove(id);
        _ = self.active_profiles.remove(id);
    }

    fn remove_node(&mut self, id: NodeId) {
        if let Some(pos) = self.sink_pw_ids.iter().position(|&node_id| node_id == id) {
            self.sink_pw_ids.remove(pos);
            self.sinks.remove(pos);
            if self.active_sink_device == Some(id) {
                self.active_sink = None;
                self.active_sink_device = self.device_ids.get(id).cloned();
            } else if let Some(id) = self.active_sink_device {
                self.set_default_sink_id(id);
            }
        } else if let Some(pos) = self.source_pw_ids.iter().position(|&node_id| node_id == id) {
            self.source_pw_ids.remove(pos);
            self.sources.remove(pos);
            if self.active_source_device == Some(id) {
                self.active_source = None;
                self.active_source_device = self.device_ids.get(id).cloned();
            } else if let Some(id) = self.active_source_device {
                self.set_default_source_id(id);
            }
        }

        _ = self.device_ids.remove(id);
        _ = self.node_names.remove(id);
        _ = self.node_descriptions.remove(id);
    }

    /// Set the default sink device by its the node ID.
    fn set_default_sink_id(&mut self, object_id: u32) {
        self.active_sink = self.sink_pw_ids.iter().position(|&id| id == object_id);
        self.active_sink_device = Some(object_id);
        self.default_sink = self.node_names.get(object_id).cloned().unwrap_or_default();
    }

    /// Set the default source device by its the node ID.
    fn set_default_source_id(&mut self, object_id: u32) {
        self.active_source = self.source_pw_ids.iter().position(|&id| id == object_id);
        self.active_source_device = Some(object_id);
        self.default_source = self.node_names.get(object_id).cloned().unwrap_or_default();
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    /// Handle messages from the sound server.
    Server(Arc<Vec<Server>>),
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
    Pipewire(pipewire::Event),
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

fn sort_devices(descriptions: &mut Vec<String>, node_ids: &mut Vec<NodeId>) {
    let mut tmp: Vec<(String, NodeId)> = std::mem::take(descriptions)
        .into_iter()
        .zip(std::mem::take(node_ids))
        .collect();

    tmp.sort_unstable_by(|(ak, _), (bk, _)| ak.cmp(bk));

    (*descriptions, *node_ids) = tmp.into_iter().collect();
}
