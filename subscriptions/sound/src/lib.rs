// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use cosmic::Task;
use cosmic::iced_futures::MaybeSend;
use cosmic_pipewire as pipewire;
use futures::{SinkExt, Stream};
use intmap::IntMap;
use pipewire::Availability;
use std::{
    process::Stdio,
    sync::{Arc, Mutex},
    time::Duration,
};

pub type DeviceId = u32;
pub type NodeId = u32;
pub type ProfileId = i32;
pub type RouteId = u32;

pub fn watch() -> impl Stream<Item = Message> + MaybeSend + 'static {
    cosmic::iced_futures::stream::channel(1, |mut emitter| async move {
        loop {
            let (cancel_tx, cancel_rx) = futures::channel::oneshot::channel::<()>();
            let sender = Arc::new((Mutex::new(Vec::new()), tokio::sync::Notify::const_new()));
            let receiver = sender.clone();

            _ = emitter
                .send(Message::SubHandle(Arc::new(SubscriptionHandle {
                    cancel_tx,
                    pipewire: pipewire::run(move |event| {
                        sender.0.lock().unwrap().push(event);
                        sender.1.notify_one();
                    }),
                })))
                .await;

            let forwarder = Box::pin(async {
                loop {
                    _ = receiver.1.notified().await;
                    let events = std::mem::take(&mut *receiver.0.lock().unwrap());
                    if !events.is_empty() {
                        _ = emitter.send(Message::Server(Arc::from(events))).await;
                        tokio::time::sleep(Duration::from_millis(64)).await;
                    }
                }
            });

            futures::future::select(cancel_rx, forwarder).await;
        }
    })
}

#[derive(Default)]
pub struct Model {
    subscription_handle: Option<SubscriptionHandle>,

    pub device_profile_dropdowns: Vec<(DeviceId, String, Option<usize>, Vec<u32>, Vec<String>)>,

    // Translated text
    pub unplugged_text: String,
    pub hd_audio_text: String,
    pub usb_audio_text: String,

    device_ids: IntMap<NodeId, DeviceId>,
    node_names: IntMap<NodeId, String>,
    card_profile_devices: IntMap<NodeId, u32>,
    node_route_indexes: IntMap<NodeId, i32>,

    device_names: IntMap<DeviceId, String>,
    device_profiles: IntMap<DeviceId, Vec<pipewire::Profile>>,
    active_profiles: IntMap<DeviceId, pipewire::Profile>,
    device_routes: IntMap<DeviceId, Vec<pipewire::Route>>,

    /** Sink devices */

    /// Description of a sink device and its port
    sinks: Vec<String>,
    /// Node IDs for sinks
    sink_node_ids: Vec<NodeId>,
    /// Index of active sink device.
    active_sink: Option<usize>,
    /// Node ID of active sink device.
    active_sink_node: Option<NodeId>,
    /// Device ID of active sink device.
    active_sink_device: Option<DeviceId>,
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
    active_source_node: Option<NodeId>,
    /// Device ID of active source device.
    active_source_device: Option<DeviceId>,
    /// Node identifier of the default source.
    active_source_node_name: String,

    changing_sink_device: Option<DeviceId>,
    changing_source_device: Option<DeviceId>,

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

    /// Send a message to the pipewire-rs thread.
    pub fn pipewire_send(&self, request: pipewire::Request) {
        if let Some(handle) = self.subscription_handle.as_ref() {
            _ = handle.pipewire.send(request);
        }
    }

    /// Sets and applies a profile to a device with wpctl.
    ///
    /// Requires using the device ID rather than a node ID.
    pub fn set_profile(&mut self, device_id: DeviceId, index: u32, save: bool) {
        if save {
            self.changing_sink_device = self
                .device_ids
                .iter()
                .find(|(node_id, _device)| self.active_sink_node == Some(*node_id))
                .and_then(|(_node_id, &device)| {
                    if device == device_id {
                        Some(device_id)
                    } else {
                        None
                    }
                });

            self.changing_source_device = self
                .device_ids
                .iter()
                .find(|(node_id, _device)| self.active_source_node == Some(*node_id))
                .and_then(|(_node_id, &device)| {
                    if device == device_id {
                        Some(device_id)
                    } else {
                        None
                    }
                });
        }

        let mut update = false;

        if let Some(profiles) = self.device_profiles.get(device_id) {
            for profile in profiles {
                if profile.index as u32 == index {
                    self.active_profiles.insert(device_id, profile.clone());
                    self.pipewire_send(pipewire::Request::SetProfile(device_id, index, save));
                    update = true;
                }
            }

            if update {
                self.update_ui_profiles();
            }

            // Use pw-cli as a fallback in case it wasn't set correctly.
            tokio::spawn(async move {
                set_profile(device_id, index, save).await;
            });
        }
    }

    /// Change the balance of channel volumes on the sink device.
    pub fn set_sink_balance(&mut self, balance: u32) -> Task<Message> {
        self.sink_balance = (balance != 100).then(|| balance as f32 / 100.);
        if self.sink_volume_debounce {
            return Task::none();
        }

        if let Some(id) = self.active_sink_node {
            self.sink_volume_debounce = true;
            return cosmic::Task::future(async move {
                tokio::time::sleep(Duration::from_millis(128)).await;
                Message::SinkVolumeApply(id)
            });
        }

        Task::none()
    }

    /// Change the default sink device
    pub fn set_default_sink(&mut self, pos: usize) -> Task<Message> {
        if let Some(&node_id) = self.sink_node_ids.get(pos) {
            self.set_default_sink_node_id(node_id);
        }

        Task::none()
    }

    pub fn set_default_sink_node_id(&mut self, node_id: NodeId) {
        tracing::debug!(target: "sound", "set default sink node {node_id}");
        self.set_default_sink_id(node_id);

        // Use pactl if the node is not a device node.
        let virtual_sink_name: Option<String> =
            if let Some(device) = self.device_ids.get(node_id).cloned() {
                // Get route index of the selected node and apply it to the device.
                if let Some((card_profile_device, route_index)) = self
                    .card_profile_devices
                    .get(node_id)
                    .cloned()
                    .zip(self.node_route_indexes.get(node_id).cloned())
                {
                    self.pipewire_send(pipewire::Request::SetRoute(
                        device,
                        card_profile_device,
                        route_index as u32,
                    ));
                }

                None
            } else {
                self.node_names.get(node_id).map(|name| name.clone())
            };

        tokio::task::spawn(async move {
            if let Some(node_name) = virtual_sink_name {
                pactl_set_default_sink(&node_name).await
            } else {
                set_default(node_id).await
            }
        });
    }

    /// Toggle the mute property of the sink device.
    pub fn toggle_sink_mute(&mut self) {
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

    /// Change the sink device's volume.
    pub fn set_sink_volume(&mut self, volume: u32) -> Task<Message> {
        self.sink_volume = volume;
        self.sink_volume_text = numtoa::BaseN::<10>::u32(volume).as_str().to_owned();
        if self.sink_volume_debounce {
            return Task::none();
        }

        // Wait for the debounce duration before applying the volume change.
        if let Some(node_id) = self.active_sink_node {
            self.sink_volume_debounce = true;
            return cosmic::Task::future(async move {
                tokio::time::sleep(Duration::from_millis(128)).await;
                Message::SinkVolumeApply(node_id)
            });
        }

        Task::none()
    }

    /// Change the default source device.
    pub fn set_default_source(&mut self, pos: usize) -> Task<Message> {
        if let Some(&node_id) = self.source_node_ids.get(pos) {
            self.set_default_source_node_id(node_id);
        }

        Task::none()
    }

    pub fn set_default_source_node_id(&mut self, node_id: NodeId) {
        tracing::debug!(target: "sound", "set default source node {node_id}");
        self.set_default_source_id(node_id);

        // Use pactl if the node is not a device node.
        let virtual_source_name: Option<String> =
            if let Some(device) = self.device_ids.get(node_id).cloned() {
                // Get route index of the selected node and apply it to the device.
                if let Some((card_profile_device, route_index)) = self
                    .card_profile_devices
                    .get(node_id)
                    .cloned()
                    .zip(self.node_route_indexes.get(node_id).cloned())
                {
                    self.pipewire_send(pipewire::Request::SetRoute(
                        device,
                        card_profile_device,
                        route_index as u32,
                    ));
                }

                None
            } else if let Some(name) = self.node_names.get(node_id) {
                Some(name.clone())
            } else {
                None
            };

        tokio::task::spawn(async move {
            if let Some(node_name) = virtual_source_name {
                pactl_set_default_source(&node_name).await
            } else {
                set_default(node_id).await
            }
        });
    }

    /// Toggle the mute property of the source device.
    pub fn toggle_source_mute(&mut self) {
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

    /// Change the source device's volume.
    pub fn set_source_volume(&mut self, volume: u32) -> Task<Message> {
        self.source_volume = volume;
        self.source_volume_text = numtoa::BaseN::<10>::u32(volume).as_str().to_owned();
        if self.source_volume_debounce {
            return Task::none();
        }

        // Wait for the debounce duration before applying the volume change.
        if let Some(node_id) = self.active_source_node {
            self.source_volume_debounce = true;
            return cosmic::Task::future(async move {
                tokio::time::sleep(Duration::from_millis(128)).await;
                Message::SourceVolumeApply(node_id)
            });
        }

        Task::none()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Server(events) => {
                Arc::into_inner(events)
                    .into_iter()
                    .flatten()
                    .for_each(|event| self.pipewire_update(event));
            }

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
                    if self.sink_volume_debounce {
                        return;
                    }

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
                    if self.source_volume_debounce {
                        return;
                    }

                    if let Some(mute) = props.mute {
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
                tracing::debug!(
                    target: "sound",
                    "Device {id} active profile changed to {}: {}",
                    profile.index,
                    profile.description
                );

                let prev = self.active_profiles.insert(id, profile.clone());
                self.update_ui_profiles();
                if let Some(prev) = prev {
                    if prev.index == profile.index {
                        return;
                    }

                    tracing::debug!(
                        target: "sound",
                        "Device {id} profile changed from {} to {}: {}",
                        prev.index, profile.index, profile.description
                    );
                } else {
                    #[cfg(feature = "auto-profile-init")]
                    if profile.index != 0 {
                        // Use pw-cli to re-set the profile in case wireplumber has invalid state.
                        // Profiles set by us do not need to use this. Only sets if profile is not `Off`.
                        tracing::debug!(
                            target: "sound",
                            "Device {id} initialized with profile {}: {}", profile.index, profile.description
                        );

                        self.set_profile(id, profile.index as u32, false);
                    }
                }
            }

            pipewire::Event::ActiveRoute(id, _index, route) => {
                tracing::debug!(
                    target: "sound",
                    "Device {id} active route changed to {}: {}",
                    route.index,
                    route.description
                );

                self.update_device_route_name(&route, id);

                let (active_device, node_ids, set_default_node): (
                    Option<DeviceId>,
                    &[NodeId],
                    fn(&mut Self, NodeId),
                ) = match route.direction {
                    pipewire::Direction::Output => (
                        self.active_sink_device,
                        &self.sink_node_ids,
                        Self::set_default_sink_id,
                    ),
                    pipewire::Direction::Input => (
                        self.active_source_device,
                        &self.source_node_ids,
                        Self::set_default_source_id,
                    ),
                };

                if active_device == Some(id) {
                    for (node_id, &device) in &self.device_ids {
                        if device == id && node_ids.contains(&node_id) {
                            set_default_node(self, node_id);
                            break;
                        }
                    }
                }
            }

            pipewire::Event::AddProfile(id, profile) => {
                if let Some(p) = self.active_profiles.get_mut(id) {
                    if p.index == profile.index {
                        *p = profile.clone();
                    }
                }

                let profiles = self.device_profiles.entry(id).or_default();
                for p in profiles.iter_mut() {
                    if p.index == profile.index {
                        *p = profile;

                        self.update_ui_profiles();
                        return;
                    }
                }

                profiles.push(profile);
                self.update_ui_profiles();
            }

            pipewire::Event::AddRoute(id, index, route) => self.add_route(id, index, route),

            pipewire::Event::AddDevice(device) => {
                tracing::debug!(target: "sound", "Device {} added: {}", device.id, device.name);
                self.device_names
                    .insert(device.id, self.translate_device_name(&device.name));
            }

            pipewire::Event::AddNode(node) => {
                tracing::debug!(target: "sound", "Node {} added: {}", node.object_id, node.node_name);
                // Device nodes will have device and card profile device IDs.
                // Virtual sinks/sources do not have these.
                if let Some(device_id) = node.device_id {
                    self.device_ids.insert(node.object_id, device_id);

                    // This is the device number of the route. This is used with the
                    // device ID to set properties for a route.
                    if let Some(card_profile_device) = node.card_profile_device {
                        self.card_profile_devices
                            .insert(node.object_id, card_profile_device);
                    }
                }

                let description = self.translate_device_name(&node.description);

                // The default sink/source is defined by a node's name. We use this when setting
                // virtual sink/source nodes with pactl; and when pipewire notifies us of a new
                // default sink/source.
                if self
                    .node_names
                    .insert(node.object_id, node.node_name.clone())
                    .is_none()
                {
                    // Use the device.profile.description as the route name by default for the UI.
                    let name = if node.device_profile_description.is_empty() {
                        description
                    } else {
                        [&node.device_profile_description, " - ", &description].concat()
                    };

                    // Check if the node is a sink or a source, and append it to the relevant collections.
                    match node.media_class {
                        pipewire::MediaClass::Sink => {
                            self.sinks.push(name);
                            self.sink_node_ids.push(node.object_id);

                            // Set the sink as the default if it matches the server.
                            if self.active_sink_node_name == node.node_name {
                                tracing::debug!(
                                    target: "sound",
                                    "Node {} ({}) was the default sink",
                                    node.object_id,
                                    node.node_name
                                );
                                self.set_default_sink_node_id(node.object_id);
                            } else if let Some(device_id) = self.changing_sink_device {
                                for (node_id, &device) in &self.device_ids {
                                    if device == device_id && self.sink_node_ids.contains(&node_id)
                                    {
                                        self.changing_sink_device = None;
                                        self.set_default_sink_node_id(node_id);
                                        return;
                                    }
                                }
                            }
                        }

                        pipewire::MediaClass::Source => {
                            self.sources.push(name);
                            self.source_node_ids.push(node.object_id);

                            // Set the source as the default if it matches the server.
                            if self.active_source_node_name == node.node_name {
                                tracing::debug!(
                                    target: "sound",
                                    "Node {} ({}) was the default source",
                                    node.object_id,
                                    node.node_name
                                );
                                self.set_default_source_node_id(node.object_id);
                            } else if let Some(device_id) = self.changing_source_device {
                                for (node_id, &device) in &self.device_ids {
                                    if device == device_id
                                        && self.source_node_ids.contains(&node_id)
                                    {
                                        self.changing_source_device = None;
                                        self.set_default_source_node_id(node_id);
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            pipewire::Event::DefaultSink(node_name) => {
                tracing::debug!(target: "sound", "default sink node changed to {node_name}");
                if self.active_sink_node_name == node_name {
                    return;
                }

                if let Some(id) = self.node_id_from_name(&node_name) {
                    self.set_default_sink_id(id);
                }

                self.active_sink_node_name = node_name;
            }

            pipewire::Event::DefaultSource(node_name) => {
                tracing::debug!(target: "sound", "default source node changed to {node_name}");
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
        self.update_device_route_name(&route, id);
        let routes = self.device_routes.entry(id).or_default();
        if routes.len() < index as usize + 1 {
            let additional = (index as usize + 1) - routes.capacity();
            routes.reserve_exact(additional);
            routes.extend(std::iter::repeat_n(pipewire::Route::default(), additional));
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
        tracing::debug!(target: "sound", "Device {id} removed");
        _ = self.device_names.remove(id);
        _ = self.device_profiles.remove(id);
        _ = self.active_profiles.remove(id);
        _ = self.device_routes.remove(id);
    }

    fn remove_node(&mut self, id: NodeId) {
        tracing::debug!(target: "sound", "Node {id} removed");
        if let Some(pos) = self.sink_node_ids.iter().position(|&node_id| node_id == id) {
            self.sink_node_ids.remove(pos);
            self.sinks.remove(pos);
            if let Some(node_id) = self.active_sink_node {
                if id == node_id {
                    self.active_sink = None;
                    self.active_sink_node = None;
                    self.active_sink_node_name.clear();
                }
            }
        } else if let Some(pos) = self
            .source_node_ids
            .iter()
            .position(|&node_id| node_id == id)
        {
            self.source_node_ids.remove(pos);
            self.sources.remove(pos);
            if let Some(node_id) = self.active_source_node {
                if id == node_id {
                    self.active_source = None;
                    self.active_source_node = None;
                    self.active_source_node_name.clear();
                }
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
        self.active_sink_device = self
            .device_ids
            .iter()
            .find_map(|(nid, did)| if nid == node_id { Some(*did) } else { None });
    }

    /// Set the default source device by its the node ID.
    fn set_default_source_id(&mut self, node_id: NodeId) {
        self.active_source = self.source_node_ids.iter().position(|&id| id == node_id);
        self.active_source_node = Some(node_id);
        self.active_source_node_name = self.node_names.get(node_id).cloned().unwrap_or_default();
        self.active_source_device = self
            .device_ids
            .iter()
            .find_map(|(nid, did)| if nid == node_id { Some(*did) } else { None });
    }

    fn update_device_route_name(&mut self, route: &pipewire::Route, id: DeviceId) {
        if matches!(route.available, Availability::No) {
            return;
        }

        let (devices, node_ids) = match route.direction {
            pipewire::Direction::Output => (&mut self.sinks, &self.sink_node_ids),
            pipewire::Direction::Input => (&mut self.sources, &self.source_node_ids),
        };

        for (pos, &node) in node_ids.iter().enumerate() {
            let Some(&device) = self.device_ids.get(node) else {
                continue;
            };

            if device != id {
                continue;
            }

            let Some(profile) = self.active_profiles.get(id) else {
                continue;
            };

            if !profile.name.starts_with("pro-audio") {
                let Some(&card_profile_device) = self.card_profile_devices.get(node) else {
                    continue;
                };

                if !route.devices.contains(&(card_profile_device as i32)) {
                    continue;
                }
            }

            let Some(device_name) = self.device_names.get(id) else {
                continue;
            };

            tracing::debug!(target: "sound", "matched route {} on {}: {}", route.index, id, route.description);
            devices[pos] = [&route.description, " - ", device_name].concat();
            self.node_route_indexes.insert(node, route.index);

            break;
        }
    }

    // Update the cached profiles for the UI.
    fn update_ui_profiles(&mut self) {
        self.device_profile_dropdowns = self
            .device_profiles
            .iter()
            .filter_map(|(device_id, profiles)| {
                let name = self.device_names.get(device_id)?.as_str();
                let (active_profile, indexes, descriptions) = self
                    .active_profiles
                    .get(device_id)
                    .map(|profile| {
                        let (indexes, descriptions): (Vec<_>, Vec<_>) = profiles
                            .iter()
                            .filter(|p| {
                                p.index == profile.index
                                    || !matches!(p.available, pipewire::Availability::No)
                            })
                            .map(|p| (p.index as u32, p.description.clone()))
                            .collect();

                        let pos = profiles
                            .iter()
                            .filter(|p| {
                                p.index == profile.index
                                    || !matches!(p.available, pipewire::Availability::No)
                            })
                            .enumerate()
                            .find(|(_, p)| p.index == profile.index)
                            .map(|(pos, _)| pos);

                        (pos, indexes, descriptions)
                    })
                    .unwrap_or_else(|| {
                        let (indexes, descriptions): (Vec<_>, Vec<_>) = profiles
                            .iter()
                            .filter(|p| !matches!(p.available, pipewire::Availability::No))
                            .map(|p| (p.index as u32, p.description.clone()))
                            .collect();

                        (None, indexes, descriptions)
                    });

                Some((
                    device_id,
                    name.to_owned(),
                    active_profile,
                    indexes,
                    descriptions,
                ))
            })
            .collect::<Vec<_>>();

        self.device_profile_dropdowns.sort_by(|a, b| a.1.cmp(&b.1));
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
    pipewire: pipewire::Sender,
}

impl std::fmt::Debug for SubscriptionHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SubscriptionHandle")
    }
}

// TODO: Use pipewire library
pub async fn set_default(id: u32) {
    tracing::debug!(target: "sound", "setting default node {id}");
    let id = numtoa::BaseN::<10>::u32(id);
    _ = tokio::process::Command::new("wpctl")
        .args(["set-default", id.as_str()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;
}

/// Use this to set a virtual sink as a default.
/// TODO: We should be able to set this with pipewire-rs somehow.
pub async fn pactl_set_default_sink(node_name: &str) {
    tracing::debug!(target: "sound", "setting default virtual node {node_name}");
    _ = tokio::process::Command::new("pactl")
        .args(["set-default-sink", node_name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;
}

/// Use this to set a virtual sink as a default.
/// TODO: We should be able to set this with pipewire-rs somehow.
pub async fn pactl_set_default_source(node_name: &str) {
    _ = tokio::process::Command::new("pactl")
        .args(["set-default-source", node_name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;
}

// TODO: Use pipewire library
pub async fn set_profile(id: u32, index: u32, save: bool) {
    let id = numtoa::BaseN::<10>::u32(id);
    let index = numtoa::BaseN::<10>::u32(index);
    let value = [
        "{ index: ",
        index.as_str(),
        if save {
            ", save: true }"
        } else {
            ", save: false }"
        },
    ]
    .concat();

    _ = tokio::process::Command::new("pw-cli")
        .args(["s", id.as_str(), "Profile", &value])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;
}
