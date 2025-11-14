// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

pub mod pipewire;

use crate::pipewire::Availability;
use cosmic::Task;
use cosmic::iced_futures::MaybeSend;
use futures::{FutureExt, SinkExt, Stream, StreamExt};
use intmap::IntMap;
use std::{process::Stdio, sync::Arc, time::Duration};

pub type DeviceId = u32;
pub type NodeId = u32;
pub type ProfileId = i32;
pub type RouteId = u32;

pub fn watch() -> impl Stream<Item = Message> + MaybeSend + 'static {
    cosmic::iced_futures::stream::channel(8, |mut emitter| async move {
        let (cancel_tx, mut cancel_rx) = futures::channel::oneshot::channel::<()>();
        let (tx, mut pw_rx) = futures::channel::mpsc::channel(1);
        let (request_tx, request_rx) = pipewire::channel();
        std::thread::spawn(move || {
            if let Err(why) = pipewire::run(request_rx, tx) {
                tracing::error!(?why, "failed to run pipewire thread");
            }
        });

        _ = emitter
            .send(
                Message::SubHandle(Arc::new(SubscriptionHandle {
                    cancel_tx,
                    pipewire: request_tx,
                }))
                .into(),
            )
            .await;

        let mut events = Vec::new();
        let mut timer = tokio::time::interval(Duration::from_millis(64));

        loop {
            futures::select! {
                event = pw_rx.next().fuse() => {
                    let Some(event) = event else {
                        break;
                    };

                    events.push(event);
                    timer.reset();
                }

                _ = timer.tick().fuse() => {
                    if !events.is_empty() {
                        _ = emitter
                            .send(Message::Server(Arc::from(std::mem::take(&mut events))))
                            .await;
                    }
                }

                _ = &mut cancel_rx => break,
            }
        }

        futures::future::pending::<Message>().await;
    })
}

#[derive(Default)]
pub struct Model {
    subscription_handle: Option<SubscriptionHandle>,

    // Translated text
    pub unplugged_text: String,
    pub hd_audio_text: String,
    pub usb_audio_text: String,

    device_ids: IntMap<NodeId, DeviceId>,
    pub node_names: IntMap<NodeId, String>,
    card_profile_devices: IntMap<NodeId, u32>,

    pub device_names: IntMap<DeviceId, String>,
    pub device_profiles: IntMap<DeviceId, Vec<pipewire::Profile>>,
    pub active_profiles: IntMap<DeviceId, pipewire::Profile>,
    pub device_routes: IntMap<DeviceId, Vec<pipewire::Route>>,

    prev_profile_node: Option<(DeviceId, NodeId)>,

    /** Sink devices */

    /// Description of a sink device and its port
    sinks: Vec<String>,
    /// Node IDs for sinks
    sink_node_ids: Vec<NodeId>,
    /// Index of active sink device.
    active_sink: Option<usize>,
    /// Device ID of active sink device.
    active_sink_node: Option<u32>,
    /// Device identifier of the default sink.
    active_sink_node_name: String,

    /** Source devices */

    /// Product names for source devices.
    sources: Vec<String>,
    /// Node IDs for sources
    source_node_ids: Vec<NodeId>,
    /// Index of active source device.
    active_source: Option<usize>,
    /// Node ID of active source device.
    active_source_node: Option<u32>,
    /// Node identifier of the default source.
    active_source_node_name: String,

    pub sink_volume_text: String,
    pub source_volume_text: String,
    pub sink_balance: Option<f32>,

    pub sink_volume: u32,
    pub source_volume: u32,

    pub sink_mute: bool,
    sink_volume_debounce: bool,
    pub source_mute: bool,
    source_volume_debounce: bool,
}

impl Model {
    pub fn active_sink(&self) -> Option<usize> {
        self.active_sink
    }

    pub fn active_source(&self) -> Option<usize> {
        self.active_source
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
            _ = handle.pipewire.send(pipewire::Request::Quit);
        }
    }

    pub fn pipewire_send(&self, request: pipewire::Request) {
        if let Some(handle) = self.subscription_handle.as_ref() {
            _ = handle.pipewire.send(request);
        }
    }

    /// Sets and applies a profile to a device with wpctl.
    ///
    /// Requires using the device ID rather than a node ID.
    pub fn set_profile(&mut self, device_id: DeviceId, index: u32) {
        if let Some(profiles) = self.device_profiles.get(device_id) {
            for profile in profiles {
                if profile.index as u32 == index {
                    // Pipewire will change the default device if the profile on that device is changed.
                    // We can prevent this by re-setting the default after changing it.
                    self.prev_profile_node =
                        self.device_ids.iter().find_map(|(node_id, &dev_id)| {
                            if dev_id != device_id {
                                return None;
                            }

                            if Some(node_id) == self.active_source_node
                                || Some(node_id) == self.active_sink_node
                            {
                                Some((dev_id, node_id))
                            } else {
                                None
                            }
                        });

                    self.active_profiles.insert(device_id, profile.clone());
                    self.pipewire_send(pipewire::Request::SetProfile(device_id, index));
                }
            }
        }
    }

    pub fn sink_balance_changed(&mut self, balance: u32) -> Task<Message> {
        self.sink_balance = (balance != 100).then(|| balance as f32 / 100.);
        if self.sink_volume_debounce {
            return Task::none();
        }

        if let Some(id) = self.active_sink_node {
            self.sink_volume_debounce = true;
            return cosmic::Task::future(async move {
                tokio::time::sleep(Duration::from_millis(32)).await;
                Message::SinkVolumeApply(id).into()
            });
        }

        Task::none()
    }

    pub fn sink_changed(&mut self, pos: usize) -> Task<Message> {
        if let Some(&object_id) = self.sink_node_ids.get(pos) {
            self.set_default_sink_id(object_id);
            tokio::task::spawn(async move {
                set_default(object_id).await;
            });
        }

        Task::none()
    }

    pub fn sink_mute_toggle(&mut self) {
        self.sink_mute = !self.sink_mute;
        if let Some(node_id) = self.active_sink_node {
            let mute = self.sink_mute;
            if let Some(handle) = self.subscription_handle.as_mut() {
                _ = handle
                    .pipewire
                    .send(pipewire::Request::SetNodeMute(node_id, mute));
            }
        }
    }

    pub fn sink_volume_changed(&mut self, volume: u32) -> Task<Message> {
        self.sink_volume = volume;
        self.sink_volume_text = numtoa::BaseN::<10>::u32(volume).as_str().to_owned();
        if self.sink_volume_debounce {
            return Task::none();
        }

        if let Some(node_id) = self.active_sink_node {
            self.sink_volume_debounce = true;
            return cosmic::Task::future(async move {
                tokio::time::sleep(Duration::from_millis(32)).await;
                Message::SinkVolumeApply(node_id).into()
            });
        }

        Task::none()
    }

    pub fn source_changed(&mut self, pos: usize) -> Task<Message> {
        if let Some(&object_id) = self.source_node_ids.get(pos) {
            self.set_default_source_id(object_id);
            tokio::task::spawn(async move {
                set_default(object_id).await;
            });
        }

        Task::none()
    }

    pub fn source_mute_toggle(&mut self) {
        self.source_mute = !self.source_mute;
        if let Some(node_id) = self.active_source_node {
            let mute = self.source_mute;
            if let Some(handle) = self.subscription_handle.as_mut() {
                _ = handle
                    .pipewire
                    .send(pipewire::Request::SetNodeMute(node_id, mute));
            }
        }
    }

    pub fn source_volume_changed(&mut self, volume: u32) -> Task<Message> {
        self.source_volume = volume;
        self.source_volume_text = numtoa::BaseN::<10>::u32(volume).as_str().to_owned();
        if self.source_volume_debounce {
            return Task::none();
        }

        if let Some(node_id) = self.active_source_node {
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
            Message::Server(events) => Arc::into_inner(events)
                .into_iter()
                .flatten()
                .for_each(|event| self.pipewire_update(event)),

            Message::SinkVolumeApply(node_id) => {
                self.sink_volume_debounce = false;
                self.pipewire_send(pipewire::Request::SetNodeVolume(
                    node_id,
                    self.sink_volume as f32 / 100.0,
                    self.sink_balance,
                ));
            }

            Message::SourceVolumeApply(node_id) => {
                self.source_volume_debounce = false;
                self.pipewire_send(pipewire::Request::SetNodeVolume(
                    node_id,
                    self.source_volume as f32 / 100.0,
                    None,
                ));
            }

            Message::SubHandle(handle) => {
                if let Some(handle) = Arc::into_inner(handle) {
                    self.subscription_handle = Some(handle);
                }
            }
        }

        Task::none()
    }

    fn pipewire_update(&mut self, event: pipewire::Event) {
        match event {
            pipewire::Event::NodeProperties(id, props) => {
                if self.active_sink_node == Some(id) {
                    if let Some(mute) = props.mute {
                        self.sink_mute = mute;
                    }

                    if let Some(channel_volumes) = props.channel_volumes {
                        let (volume, balance) =
                            pipewire::volume::from_channel_volumes(&channel_volumes);

                        self.sink_balance = balance;
                        self.sink_volume = (volume * 100.0) as u32;
                        self.sink_volume_text = numtoa::BaseN::<10>::u32(self.sink_volume)
                            .as_str()
                            .to_owned();
                    }
                } else if self.active_source_node == Some(id) {
                    if let Some(mute) = props.monitor_mute {
                        self.source_mute = mute;
                    }

                    if let Some(channel_volumes) = props.channel_volumes {
                        let (volume, _balance) =
                            pipewire::volume::from_channel_volumes(&channel_volumes);
                        self.source_volume = (volume * 100.0) as u32;
                        self.source_volume_text = numtoa::BaseN::<10>::u32(self.source_volume)
                            .as_str()
                            .to_owned();
                    }
                }
            }

            pipewire::Event::ActiveProfile(id, profile) => {
                let index = profile.index as u32;
                self.active_profiles.insert(id, profile);
                self.pipewire_send(pipewire::Request::SetProfile(id, index));
            }

            pipewire::Event::ActiveRoute(id, index, route) => {
                self.add_route(id, index, route);
            }

            pipewire::Event::AddProfile(id, profile) => {
                let profiles = self.device_profiles.entry(id).or_default();
                for p in profiles.iter_mut() {
                    if p.index == profile.index {
                        *p = profile;
                        return;
                    }
                }

                profiles.push(profile);
            }

            pipewire::Event::AddRoute(id, index, route) => self.add_route(id, index, route),

            pipewire::Event::AddDevice(device) => {
                self.device_names
                    .insert(device.id, self.translate_device_name(&device.name));
            }

            pipewire::Event::AddNode(node) => {
                if let Some(device_id) = node.device_id {
                    self.device_ids.insert(node.object_id, device_id);

                    if let Some(card_profile_device) = node.card_profile_device {
                        self.card_profile_devices
                            .insert(node.object_id, card_profile_device);
                    }
                }

                let description = self.translate_device_name(&node.description);

                if self
                    .node_names
                    .insert(node.object_id, node.node_name.clone())
                    .is_none()
                {
                    let name = if node.device_profile_description.is_empty() {
                        description
                    } else {
                        [&node.device_profile_description, " - ", &description].concat()
                    };
                    match node.media_class {
                        pipewire::MediaClass::Sink => {
                            self.sinks.push(name);
                            self.sink_node_ids.push(node.object_id);

                            if self.active_sink_node_name == node.node_name {
                                self.set_default_sink_id(node.object_id);
                                tokio::task::spawn(async move {
                                    set_default(node.object_id).await;
                                });
                            }
                        }

                        pipewire::MediaClass::Source => {
                            self.sources.push(name);
                            self.source_node_ids.push(node.object_id);

                            if self.active_source_node_name == node.node_name {
                                self.set_default_source_id(node.object_id);
                                tokio::task::spawn(async move {
                                    set_default(node.object_id).await;
                                });
                            }
                        }
                    }
                }

                if let Some((device_id, node_id)) = self.prev_profile_node {
                    if Some(device_id) == node.device_id && node.object_id == node_id {
                        self.prev_profile_node = None;
                        tokio::task::spawn(async move {
                            set_default(node_id).await;
                        });
                    }
                }
            }

            pipewire::Event::DefaultSink(node_name) => {
                if self.active_sink_node_name == node_name {
                    return;
                }

                if let Some(id) = self.node_id_from_name(&node_name) {
                    self.set_default_sink_id(id);
                }

                self.active_sink_node_name = node_name;
            }

            pipewire::Event::DefaultSource(node_name) => {
                if self.active_source_node_name == node_name {
                    return;
                }

                if let Some(id) = self.node_id_from_name(&node_name) {
                    self.set_default_source_id(id);
                }

                self.active_source_node_name = node_name;
            }

            pipewire::Event::RemoveDevice(id) => self.remove_device(id),
            pipewire::Event::RemoveNode(id) => self.remove_node(id),
        }
    }

    fn add_route(&mut self, id: DeviceId, index: u32, route: pipewire::Route) {
        self.update_device_route(&route, id);
        let routes = self.device_routes.entry(id).or_default();
        if routes.len() < index as usize + 1 {
            let additional = (index as usize + 1) - routes.capacity();
            routes.reserve_exact(additional);
            routes.extend(std::iter::repeat(pipewire::Route::default()).take(additional));
        }
        routes[index as usize] = route;
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
        _ = self.device_routes.remove(id);
    }

    fn remove_node(&mut self, id: NodeId) {
        if let Some(pos) = self.sink_node_ids.iter().position(|&node_id| node_id == id) {
            self.sink_node_ids.remove(pos);
            self.sinks.remove(pos);
            if let Some(node_id) = self.active_sink_node {
                self.set_default_sink_id(node_id);
            }
        } else if let Some(pos) = self
            .source_node_ids
            .iter()
            .position(|&node_id| node_id == id)
        {
            self.source_node_ids.remove(pos);
            self.sources.remove(pos);
            if let Some(node_id) = self.active_source_node {
                self.set_default_source_id(node_id);
            }
        }

        _ = self.device_ids.remove(id);
        _ = self.node_names.remove(id);
        _ = self.card_profile_devices.remove(id);
    }

    /// Set the default sink device by its the node ID.
    fn set_default_sink_id(&mut self, node_id: NodeId) {
        self.active_sink = self.sink_node_ids.iter().position(|&id| id == node_id);
        self.active_sink_node = Some(node_id);
        self.active_sink_node_name = self.node_names.get(node_id).cloned().unwrap_or_default();
    }

    /// Set the default source device by its the node ID.
    fn set_default_source_id(&mut self, node_id: NodeId) {
        self.active_source = self.source_node_ids.iter().position(|&id| id == node_id);
        self.active_source_node = Some(node_id);
        self.active_source_node_name = self.node_names.get(node_id).cloned().unwrap_or_default();
    }

    /// Check if a node has had its route appended to the name, and return a name if we should update it.
    fn route_plug_check(
        &mut self,
        node: NodeId,
        device: DeviceId,
        route: &pipewire::Route,
    ) -> Option<String> {
        let profile = self.active_profiles.get(device)?;

        if !profile.name.starts_with("pro-audio") {
            let Some(&card_profile_device) = self.card_profile_devices.get(node) else {
                return None;
            };

            if !route.devices.contains(&(card_profile_device as i32)) {
                return None;
            }
        }

        self.route_name_get(&route.description, route.available, device)
    }

    fn route_name_get(
        &self,
        route_description: &str,
        route_available: Availability,
        device: DeviceId,
    ) -> Option<String> {
        if matches!(route_available, Availability::No) {
            return None;
        }

        let Some(device_name) = self.device_names.get(device) else {
            return None;
        };

        Some([&route_description, " - ", device_name].concat())
    }

    fn update_device_route(&mut self, route: &pipewire::Route, id: DeviceId) {
        if matches!(route.available, Availability::No) {
            return;
        }

        match route.direction {
            pipewire::Direction::Output => {
                for (pos, &node) in self.sink_node_ids.iter().enumerate() {
                    let Some(&device) = self.device_ids.get(node) else {
                        continue;
                    };

                    if device != id {
                        continue;
                    }

                    if let Some(node_name) = self.route_plug_check(node, device, &route) {
                        self.sinks[pos] = node_name;
                    }

                    break;
                }
            }

            pipewire::Direction::Input => {
                for (pos, &node) in self.source_node_ids.iter().enumerate() {
                    let Some(&device) = self.device_ids.get(node) else {
                        continue;
                    };

                    if device != id {
                        continue;
                    }

                    if let Some(node_name) = self.route_plug_check(node, device, &route) {
                        self.sources[pos] = node_name;
                    }

                    break;
                }
            }
        }
    }

    fn translate_device_name(&self, input: &str) -> String {
        input
            .replacen(" Controller", "", 1)
            .replacen("High Definition Audio", &self.hd_audio_text, 1)
            .replacen("HD Audio", &self.hd_audio_text, 1)
            .replacen("USB Audio Device", &self.usb_audio_text, 1)
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    /// Handle messages from the sound server.
    Server(Arc<Vec<pipewire::Event>>),
    /// Change the output volume.
    SinkVolumeApply(NodeId),
    /// Change the input volume.
    SourceVolumeApply(NodeId),
    /// On init of the subscription, channels for closing background threads are given to the app.
    SubHandle(Arc<SubscriptionHandle>),
}

pub struct SubscriptionHandle {
    cancel_tx: futures::channel::oneshot::Sender<()>,
    pipewire: pipewire::Sender<pipewire::Request>,
}

impl std::fmt::Debug for SubscriptionHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SubscriptionHandle")
    }
}

// TODO: Use pipewire library
pub async fn set_default(id: u32) {
    let id = numtoa::BaseN::<10>::u32(id);
    _ = tokio::process::Command::new("wpctl")
        .args(["set-default", id.as_str()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;
}
