// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

// #![deny(missing_docs)]

pub mod device;
pub use device::Device;

pub mod node;
use intmap::IntMap;
pub use node::{MediaClass, Node, NodeProps};

mod profile;
pub use profile::Profile;

mod route;
#[cfg(feature = "route-port-type")]
pub use route::PortType;
pub use route::{Route, RouteProps};

mod spa_utils;
pub use spa_utils::Channel;

use libspa::{
    param::{ParamType, format::FormatProperties},
    pod::{self, Pod, serialize::PodSerializer},
    utils::SpaTypes,
};
use pipewire::{
    device::{DeviceChangeMask, DeviceListener},
    main_loop::MainLoopWeak,
    metadata::MetadataListener,
    node::NodeListener,
    proxy::{ProxyListener, ProxyT},
    types::ObjectType,
};
use std::{cell::RefCell, rc::Rc, u32};

pub type NodeId = u32;
pub type RouteId = u32;
pub type DeviceId = u32;
pub type ProfileId = i32;
pub type PipewireId = u32;

pub fn run(on_event: impl FnMut(Event) + Send + 'static) -> Sender {
    let (request_tx, request_rx) = pipewire::channel::channel();

    std::thread::spawn(move || {
        let on_event = Box::new(on_event);
        if let Err(why) = run_service(request_rx, on_event) {
            tracing::error!(?why, "failed to run pipewire thread");
        }
    });

    Sender(request_tx)
}

/// Monitor pipewire activity and
fn run_service(
    rx: pipewire::channel::Receiver<Request>,
    on_event: Box<dyn FnMut(Event)>,
) -> Result<(), pipewire::Error> {
    let main_loop = pipewire::main_loop::MainLoopRc::new(None)?;
    let context = pipewire::context::ContextRc::new(&main_loop, None)?;
    let core = context.connect_rc(None)?;
    let registry = core.get_registry_rc()?;

    let state = Rc::new(RefCell::new(State {
        nodes: IntMap::new(),
        proxies: Proxies {
            devices: IntMap::new(),
            metadata: IntMap::new(),
            nodes: IntMap::new(),
        },
        routes: IntMap::new(),
        node_devices: IntMap::new(),
        node_card_profile_device: IntMap::new(),
        node_props: IntMap::new(),
        main_loop: main_loop.downgrade(),
        on_event,
    }));

    let _request_handler = rx.attach(main_loop.loop_(), {
        let state = Rc::downgrade(&state);
        move |request| match request {
            Request::EnumerateDevice(id) => {
                if let Some(state) = state.upgrade() {
                    state.borrow_mut().enumerate_device(id);
                }
            }

            Request::SetRoute(id, card_profile_device, route) => {
                if let Some(state) = state.upgrade() {
                    state
                        .borrow_mut()
                        .set_route(id, card_profile_device as i32, route as i32);
                }
            }

            Request::SetNodeVolume(id, volume, balance) => {
                if let Some(state) = state.upgrade() {
                    state.borrow_mut().set_node_volume(id, volume, balance);
                }
            }

            Request::SetNodeMute(id, mute) => {
                if let Some(state) = state.upgrade() {
                    state.borrow_mut().set_mute_node(id, mute);
                }
            }

            Request::SetProfile(id, index, save) => {
                if let Some(state) = state.upgrade() {
                    state.borrow_mut().set_profile(id, index, save);
                }
            }

            Request::Quit => {
                if let Some(state) = state.upgrade() {
                    state.borrow_mut().quit();
                }
            }
        }
    });

    let registry_weak = registry.downgrade();

    let _registry_listener = registry
        .add_listener_local()
        .global(move |obj| {
            let Some(registry) = registry_weak.upgrade() else {
                return;
            };

            match obj.type_ {
                ObjectType::Device => {
                    let Ok(device) = registry.bind::<pipewire::device::Device, _>(obj) else {
                        return;
                    };

                    device.subscribe_params(&[
                        ParamType::EnumProfile,
                        ParamType::EnumRoute,
                        ParamType::Profile,
                        ParamType::Route,
                    ]);

                    let pw_id = device.upcast_ref().id();

                    let listener = device
                        .add_listener_local()
                        .info({
                            let state = Rc::downgrade(&state);
                            move |info| {
                                let change_mask = info.change_mask();
                                if change_mask == DeviceChangeMask::PARAMS {
                                    if let Some(state) = state.upgrade() {
                                        let state = state.borrow();
                                        let Some((_device_id, device, ..)) =
                                            state.proxies.devices.get(pw_id)
                                        else {
                                            return;
                                        };

                                        device.enum_params(
                                            0,
                                            Some(ParamType::EnumProfile),
                                            0,
                                            u32::MAX,
                                        );
                                        device.enum_params(
                                            0,
                                            Some(ParamType::Profile),
                                            0,
                                            u32::MAX,
                                        );
                                        device.enum_params(1, Some(ParamType::Route), 0, u32::MAX);
                                    }

                                    return;
                                }

                                if let Some(device) = Device::from_device(info) {
                                    if let Some(state) = state.upgrade() {
                                        state.borrow_mut().add_device(pw_id, device);
                                    }
                                }
                            }
                        })
                        .param({
                            let state = Rc::downgrade(&state);
                            move |_seq, param_type, index, _next, param| {
                                let Some(pod) = param else {
                                    return;
                                };

                                let Some(state) = state.upgrade() else {
                                    return;
                                };

                                let Some(&(device_id, ..)) =
                                    state.borrow().proxies.devices.get(pw_id)
                                else {
                                    return;
                                };

                                match param_type {
                                    ParamType::EnumProfile => {
                                        if let Some(profile) = Profile::from_pod(pod) {
                                            state.borrow_mut().add_profile(device_id, profile);
                                        }
                                    }

                                    ParamType::EnumRoute => {
                                        if let Some(route) = Route::from_pod(pod) {
                                            state.borrow_mut().add_route(device_id, index, route);
                                        }
                                    }

                                    ParamType::Profile => {
                                        if let Some(profile) = Profile::from_pod(pod) {
                                            state.borrow_mut().active_profile(device_id, profile);
                                        }
                                    }

                                    ParamType::Route => {
                                        if let Some(route) = Route::from_pod(pod) {
                                            state
                                                .borrow_mut()
                                                .active_route(device_id, index, route);
                                        }
                                    }

                                    _ => (),
                                }
                            }
                        })
                        .register();

                    let proxy = device.upcast_ref();

                    let remove_listener = proxy
                        .add_listener_local()
                        .removed({
                            let state = Rc::downgrade(&state);
                            move || {
                                if let Some(state) = state.upgrade() {
                                    let Some((id, ..)) =
                                        state.borrow_mut().proxies.devices.remove(pw_id)
                                    else {
                                        return;
                                    };

                                    state.borrow_mut().remove_device(id);
                                }
                            }
                        })
                        .register();

                    state
                        .borrow_mut()
                        .proxies
                        .devices
                        .insert(pw_id, (0, device, listener, remove_listener));
                }

                ObjectType::Node => {
                    let Ok(node) = registry.bind::<pipewire::node::Node, _>(obj) else {
                        return;
                    };

                    node.subscribe_params(&[ParamType::Props]);

                    let id = node.upcast_ref().id();

                    let listener = node
                        .add_listener_local()
                        .info({
                            let state = Rc::downgrade(&state);
                            move |info| {
                                if let Some(node) = Node::from_node(info) {
                                    if let Some(state) = state.upgrade() {
                                        state.borrow_mut().add_node(id, node);
                                    }
                                }
                            }
                        })
                        .param({
                            let state = Rc::downgrade(&state);
                            move |_seq, param_type, _index, _next, param| {
                                let Some(pod) = param else {
                                    return;
                                };

                                let Some(state) = state.upgrade() else {
                                    return;
                                };

                                let Some(&(node_id, ..)) = state.borrow().proxies.nodes.get(id)
                                else {
                                    return;
                                };

                                match param_type {
                                    ParamType::Props => {
                                        if let Some(props) = NodeProps::from_pod(pod) {
                                            state.borrow_mut().set_node_props(node_id, props);
                                        }
                                    }

                                    _ => (),
                                }
                            }
                        })
                        .register();

                    let remove_listener = node
                        .upcast_ref()
                        .add_listener_local()
                        .removed({
                            let state = Rc::downgrade(&state);
                            move || {
                                if let Some(state) = state.upgrade() {
                                    state.borrow_mut().remove_node(id);
                                }
                            }
                        })
                        .register();

                    state
                        .borrow_mut()
                        .proxies
                        .nodes
                        .insert(id, (0, node, listener, remove_listener));
                }

                ObjectType::Metadata => {
                    let Ok(metadata) = registry.bind::<pipewire::metadata::Metadata, _>(obj) else {
                        return;
                    };

                    let id = metadata.upcast_ref().id();

                    let listener = metadata
                        .add_listener_local()
                        .property({
                            let state = Rc::downgrade(&state);
                            move |_subject, key, _type, value| {
                                let Some((key, value)) = key.zip(value) else {
                                    return 0;
                                };

                                match key {
                                    "default.audio.sink" => {
                                        if let Ok(value) =
                                            serde_json::de::from_str::<DefaultAudio>(value)
                                        {
                                            if let Some(state) = state.upgrade() {
                                                state
                                                    .borrow_mut()
                                                    .default_sink(value.name.to_owned())
                                            }
                                        }
                                    }

                                    "default.audio.source" => {
                                        if let Ok(value) =
                                            serde_json::de::from_str::<DefaultAudio>(value)
                                        {
                                            if let Some(state) = state.upgrade() {
                                                state
                                                    .borrow_mut()
                                                    .default_source(value.name.to_owned())
                                            }
                                        }
                                    }

                                    _ => (),
                                }

                                0
                            }
                        })
                        .register();

                    let remove_listener = metadata
                        .upcast_ref()
                        .add_listener_local()
                        .removed({
                            let state = Rc::downgrade(&state);
                            move || {
                                if let Some(state) = state.upgrade() {
                                    state.borrow_mut().remove_metadata(id);
                                }
                            }
                        })
                        .register();

                    state
                        .borrow_mut()
                        .proxies
                        .metadata
                        .insert(id, (metadata, listener, remove_listener));
                }
                _ => {}
            };
        })
        .register();

    main_loop.run();
    Ok(())
}

/// Response from pipewire
#[derive(Clone, Debug)]
pub enum Event {
    /// Set the active profile for a device
    ActiveProfile(DeviceId, Profile),
    /// Set the active route for a device
    ActiveRoute(DeviceId, u32, Route),
    /// A new device was detected.
    AddDevice(Device),
    /// A new node was detected.
    AddNode(Node),
    /// A profile was enumerated
    AddProfile(DeviceId, Profile),
    /// A route was enumerated
    AddRoute(DeviceId, u32, Route),
    /// The default sink was changed.
    DefaultSink(String),
    /// The default source was changed.
    DefaultSource(String),
    /// Emitted when the properties of a node has changed.
    NodeProperties(NodeId, NodeProps),
    /// A device with the given device_id was removed.
    RemoveDevice(DeviceId),
    /// A node with the given object_id was removed.
    RemoveNode(NodeId),
}

#[derive(Clone, Debug)]
pub enum Request {
    /// Request a device's routes, profiles, active routes, and active profile.
    EnumerateDevice(DeviceId),
    /// Mute a node ID
    SetNodeMute(NodeId, bool),
    /// Set a device profile by profile index.
    SetProfile(DeviceId, u32, bool),
    /// Set a new volume
    SetNodeVolume(DeviceId, f32, Option<f32>),
    /// Change route of a device
    SetRoute(DeviceId, u32, u32),
    /// Stop the main loop and exit the thread.
    Quit,
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum Availability {
    #[default]
    Unknown,
    No,
    Yes,
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum Direction {
    Input,
    #[default]
    Output,
}

#[derive(serde::Deserialize)]
pub struct DefaultAudio<'a> {
    name: &'a str,
}

struct Proxies {
    devices: IntMap<
        PipewireId,
        (
            DeviceId,
            pipewire::device::Device,
            DeviceListener,
            ProxyListener,
        ),
    >,
    nodes: IntMap<PipewireId, (NodeId, pipewire::node::Node, NodeListener, ProxyListener)>,
    metadata: IntMap<
        PipewireId,
        (
            pipewire::metadata::Metadata,
            MetadataListener,
            ProxyListener,
        ),
    >,
}

struct State {
    nodes: IntMap<PipewireId, (NodeId, Option<DeviceId>)>,
    pub(self) proxies: Proxies,
    routes: IntMap<DeviceId, Vec<Route>>,
    node_devices: IntMap<NodeId, DeviceId>,
    node_props: IntMap<NodeId, NodeProps>,
    node_card_profile_device: IntMap<NodeId, u32>,
    main_loop: MainLoopWeak,
    /// Handle events and exit the loop when `true` is returned.
    on_event: Box<dyn FnMut(Event)>,
}

impl State {
    fn active_profile(&mut self, id: DeviceId, profile: Profile) {
        self.on_event(Event::ActiveProfile(id, profile));
    }

    fn active_route(&mut self, id: DeviceId, index: u32, route: Route) {
        self.on_event(Event::ActiveRoute(id, index, route));
    }

    fn add_device(&mut self, id: PipewireId, device: Device) {
        // Map the device's pipewire ID to its device ID
        if let Some(entry) = self.proxies.devices.get_mut(id) {
            entry.0 = device.id;
        };

        let device_id = device.id;
        self.on_event(Event::AddDevice(device));

        // Request the device's profiles and properties now that we've registered it.
        self.enumerate_device(device_id);
    }

    fn add_node(&mut self, id: PipewireId, node: Node) {
        // Map the device's pipewire ID to its device ID
        if let Some(entry) = self.proxies.nodes.get_mut(id) {
            entry.0 = node.object_id;
            // Request properties for this node now that we've registered it.
            entry.1.enum_params(0, Some(ParamType::Props), 0, u32::MAX);
        };

        // Track the node's node ID and device ID by its pipewire ID.
        self.nodes.insert(id, (node.object_id, node.device_id));

        // And the associated route device that the node is derived from.
        if let Some(card_profile_device) = node.card_profile_device {
            self.node_card_profile_device
                .insert(node.object_id, card_profile_device);
        }

        // Track the node's device ID by its node ID.
        if let Some(device_id) = node.device_id {
            self.node_devices.insert(node.object_id, device_id);
        }

        self.on_event(Event::AddNode(node));
    }

    fn add_profile(&mut self, id: DeviceId, profile: Profile) {
        self.on_event(Event::AddProfile(id, profile));
    }

    fn add_route(&mut self, id: DeviceId, index: u32, route: Route) {
        // Keep a record of routes attached to a device for setting properties.
        // This will overwrite routes on updates to
        let routes = self.routes.entry(id).or_default();
        if routes.len() < index as usize + 1 {
            let additional = (index as usize + 1) - routes.capacity();
            routes.reserve_exact(additional);
            routes.extend(std::iter::repeat(Route::default()).take(additional));
        }
        routes[index as usize] = route.clone();

        self.on_event(Event::AddRoute(id, index, route));
    }

    /// Request a device's profiles and routes.
    fn enumerate_device(&mut self, id: DeviceId) {
        let Some(device) = self.device(id) else {
            return;
        };

        device.enum_params(0, Some(ParamType::EnumProfile), 0, u32::MAX);
        device.enum_params(1, Some(ParamType::EnumRoute), 0, u32::MAX);
        device.enum_params(2, Some(ParamType::Profile), 0, u32::MAX);
        device.enum_params(3, Some(ParamType::Route), 0, u32::MAX);
    }

    fn default_sink(&mut self, name: String) {
        self.on_event(Event::DefaultSink(name));
    }

    fn default_source(&mut self, name: String) {
        self.on_event(Event::DefaultSource(name));
    }

    fn node_route(&self, device_id: DeviceId, route_device: i32) -> Option<&Route> {
        self.routes
            .get(device_id)?
            .iter()
            .find(|r| r.devices.contains(&route_device))
    }

    fn on_event(&mut self, event: Event) {
        (self.on_event)(event);
    }

    fn quit(&mut self) {
        if let Some(main_loop) = self.main_loop.upgrade() {
            main_loop.quit();
        }
    }

    fn remove_device(&mut self, id: PipewireId) {
        if let Some((device_id, ..)) = self.proxies.devices.remove(id) {
            self.routes.remove(device_id);
            self.on_event(Event::RemoveDevice(device_id));
        }
    }

    fn remove_metadata(&mut self, id: PipewireId) {
        self.proxies.metadata.remove(id);
    }

    fn remove_node(&mut self, id: PipewireId) {
        if let Some((node_id, _)) = self.nodes.remove(id) {
            self.node_card_profile_device.remove(node_id);
            self.node_devices.remove(node_id);
            self.node_props.remove(node_id);
            self.on_event(Event::RemoveNode(node_id));
        }

        self.proxies.nodes.remove(id);
    }

    fn set_mute(&self, id: DeviceId, route_device: i32, route: &Route, mute: bool) {
        let Some(device) = self.device(id) else {
            return;
        };

        let route_props = pod::object!(
            SpaTypes::ObjectParamProps,
            ParamType::Props,
            pod::property!(FormatProperties(libspa_sys::SPA_PROP_mute), Bool, mute),
        );

        let buffer = std::io::Cursor::new(Vec::new());
        let Ok(serialized) = PodSerializer::serialize(
            buffer,
            &pod::Value::Object(pod::object!(
                SpaTypes::ObjectParamRoute,
                ParamType::Route,
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PARAM_ROUTE_index),
                    Int,
                    route.index
                ),
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PARAM_ROUTE_device),
                    Int,
                    route_device
                ),
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PARAM_ROUTE_props),
                    Object,
                    route_props
                ),
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PARAM_ROUTE_save),
                    Bool,
                    true
                )
            )),
        )
        .map(|(cursor, _)| cursor.into_inner()) else {
            return;
        };

        if let Some(param) = Pod::from_bytes(&serialized) {
            device.set_param(ParamType::Route, 0, param);
        }
    }

    fn set_mute_node(&self, id: NodeId, mute: bool) {
        // Prefer to mute the device instead of the node.
        // Muting a node will not emit a notification.
        if let Some((&device_id, &route_device)) = self
            .node_devices
            .get(id)
            .zip(self.node_card_profile_device.get(id))
        {
            let route_device = route_device as i32;
            if let Some(route) = self.node_route(device_id, route_device) {
                self.set_mute(device_id, route_device, route, mute);
                return;
            };
        }

        let Some(node) = self.node(id) else {
            return;
        };

        let buffer = std::io::Cursor::new(Vec::new());
        let Ok(serialized) = PodSerializer::serialize(
            buffer,
            &pod::Value::Object(pod::object!(
                SpaTypes::ObjectParamProps,
                ParamType::Props,
                pod::property!(FormatProperties(libspa_sys::SPA_PROP_mute), Bool, mute),
            )),
        )
        .map(|(cursor, _)| cursor.into_inner()) else {
            return;
        };

        if let Some(param) = Pod::from_bytes(&serialized) {
            node.set_param(ParamType::Props, 0, param);
        }
    }

    fn set_route(&self, device_id: DeviceId, card_profile_device: i32, route_index: i32) {
        let Some(device) = self.device(device_id) else {
            return;
        };

        tracing::debug!(target: "sound", "set_route device_id {device_id}, route_index {route_index}");

        let buffer = std::io::Cursor::new(Vec::new());
        let Ok(serialized) = PodSerializer::serialize(
            buffer,
            &pod::Value::Object(pod::object!(
                SpaTypes::ObjectParamRoute,
                ParamType::Route,
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PARAM_ROUTE_index),
                    Int,
                    route_index
                ),
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PARAM_ROUTE_device),
                    Int,
                    card_profile_device
                ),
            )),
        )
        .map(|(cursor, _)| cursor.into_inner()) else {
            return;
        };

        if let Some(param) = Pod::from_bytes(&serialized) {
            device.set_param(ParamType::Route, 0, param);
        }
        return;
    }

    fn set_node_props(&mut self, id: NodeId, props: NodeProps) {
        self.on_event(Event::NodeProperties(id, props.clone()));
        *self.node_props.entry(id).or_default() = props;
    }

    fn set_node_volume(&self, id: NodeId, volume: f32, balance: Option<f32>) {
        let Some(props) = self.node_props.get(id) else {
            return;
        };

        // Prefer to change the volume of the device instead of the node.
        if let Some((&device_id, &route_device)) = self
            .node_devices
            .get(id)
            .zip(self.node_card_profile_device.get(id))
        {
            let route_device = route_device as i32;
            if let Some(route) = self.node_route(device_id, route_device) {
                self.set_volume(device_id, props, route_device, route, volume, balance);
                return;
            };
        }

        let Some(node) = self.node(id) else {
            return;
        };

        let buffer = std::io::Cursor::new(Vec::new());
        let Ok(serialized) = PodSerializer::serialize(
            buffer,
            &pod::Value::Object(pod::object!(
                SpaTypes::ObjectParamProps,
                ParamType::Props,
                pod::property!(FormatProperties(libspa_sys::SPA_PROP_mute), Bool, false),
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PROP_channelVolumes),
                    ValueArray,
                    pod::ValueArray::Float(volume::to_channel_volumes(
                        &props.channel_map.as_deref().unwrap_or_default(),
                        volume,
                        balance,
                    ))
                )
            )),
        )
        .map(|(cursor, _)| cursor.into_inner()) else {
            return;
        };

        if let Some(param) = Pod::from_bytes(&serialized) {
            node.set_param(ParamType::Props, 0, param);
        }
    }

    fn set_profile(&mut self, id: DeviceId, index: u32, save: bool) {
        let Some(device) = self.device(id) else {
            return;
        };

        let buffer = std::io::Cursor::new(Vec::new());
        let Ok(serialized) = PodSerializer::serialize(
            buffer,
            &pod::Value::Object(pod::object!(
                SpaTypes::ObjectParamProfile,
                ParamType::Profile,
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PARAM_PROFILE_index),
                    Int,
                    index as i32
                ),
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PARAM_PROFILE_save),
                    Bool,
                    save
                )
            )),
        )
        .map(|(cursor, _)| cursor.into_inner()) else {
            return;
        };

        if let Some(param) = Pod::from_bytes(&serialized) {
            device.set_param(ParamType::Profile, 0, param);
        }
    }

    fn set_volume(
        &self,
        id: DeviceId,
        props: &NodeProps,
        route_device: i32,
        route: &Route,
        volume: f32,
        balance: Option<f32>,
    ) {
        let Some(device) = self.device(id) else {
            return;
        };

        let route_props = pod::object!(
            SpaTypes::ObjectParamProps,
            ParamType::Props,
            pod::property!(FormatProperties(libspa_sys::SPA_PROP_mute), Bool, false),
            pod::property!(
                FormatProperties(libspa_sys::SPA_PROP_channelVolumes),
                ValueArray,
                pod::ValueArray::Float(if matches!(route.direction, Direction::Output) {
                    volume::to_channel_volumes(
                        &props.channel_map.as_deref().unwrap_or_default(),
                        volume,
                        balance,
                    )
                } else {
                    vec![volume * volume * volume]
                })
            )
        );

        let buffer = std::io::Cursor::new(Vec::new());
        let Ok(serialized) = PodSerializer::serialize(
            buffer,
            &pod::Value::Object(pod::object!(
                SpaTypes::ObjectParamRoute,
                ParamType::Route,
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PARAM_ROUTE_index),
                    Int,
                    route.index
                ),
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PARAM_ROUTE_device),
                    Int,
                    route_device
                ),
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PARAM_ROUTE_props),
                    Object,
                    route_props
                ),
                pod::property!(
                    FormatProperties(libspa_sys::SPA_PARAM_ROUTE_save),
                    Bool,
                    true
                )
            )),
        )
        .map(|(cursor, _)| cursor.into_inner()) else {
            return;
        };

        if let Some(param) = Pod::from_bytes(&serialized) {
            device.set_param(ParamType::Route, 0, param);
        }
    }

    fn device(&self, id: DeviceId) -> Option<&pipewire::device::Device> {
        self.proxies
            .devices
            .values()
            .find(|(device_id, ..)| id == *device_id)
            .map(|(_, device, ..)| device)
    }

    fn node(&self, id: NodeId) -> Option<&pipewire::node::Node> {
        self.proxies
            .nodes
            .values()
            .find(|(node_id, ..)| id == *node_id)
            .map(|(_, node, ..)| node)
    }
}

pub struct Sender(pipewire::channel::Sender<Request>);

impl Sender {
    pub fn send(&self, request: Request) -> Result<(), Request> {
        self.0.send(request)
    }
}

impl Drop for Sender {
    fn drop(&mut self) {
        _ = self.0.send(Request::Quit);
    }
}

pub mod volume {
    use crate::Channel;

    /// Get the configured volume and balance based on a provided channel volumes array.
    pub fn from_channel_volumes(channels: &[f32]) -> (f32, Option<f32>) {
        let left_volume = channels.first().cloned().unwrap_or_default();
        let right_volume = channels.last().cloned().unwrap_or_default();

        if (left_volume - right_volume).abs() < f32::EPSILON {
            return (left_volume.powf(1.0 / 3.0), None);
        }

        let (volume, balance) = if left_volume >= right_volume {
            (left_volume, right_volume / left_volume)
        } else {
            (right_volume, (2.0 - (left_volume / right_volume)))
        };

        (volume.powf(1.0 / 3.0), Some(balance))
    }

    /// Create a channel volumes array based on the provided volume, balance, and channel positions.
    pub fn to_channel_volumes(
        channel_map: &[Channel],
        volume: f32,
        balance: Option<f32>,
    ) -> Vec<f32> {
        let volume = volume * volume * volume;
        if let Some(balance) = balance {
            let (left_volume, right_volume) = if balance >= 1.0 {
                ((volume * (balance - 2.0).abs()), volume)
            } else {
                (volume, volume * balance)
            };

            let center_volume = (left_volume + right_volume) / 2.0;
            let mut channel_volumes = Vec::with_capacity(channel_map.len());

            // Use channel identifiers to apply volume balance
            for channel in channel_map {
                channel_volumes.push(match channel {
                    // Left channels
                    Channel::FL
                    | Channel::SL
                    | Channel::FLC
                    | Channel::RL
                    | Channel::TFL
                    | Channel::TFC
                    | Channel::TRL
                    | Channel::RLC
                    | Channel::FLW
                    | Channel::FLH
                    | Channel::TFLC
                    | Channel::TSL
                    | Channel::LLFE
                    | Channel::BLC => left_volume,
                    // Right channels
                    Channel::FR
                    | Channel::SR
                    | Channel::FRC
                    | Channel::RR
                    | Channel::TFR
                    | Channel::TRC
                    | Channel::TRR
                    | Channel::RRC
                    | Channel::FRW
                    | Channel::FRH
                    | Channel::TFRC
                    | Channel::TSR
                    | Channel::RLFE
                    | Channel::BRC => right_volume,
                    // Center/neutral channels
                    _ => center_volume,
                });
            }

            channel_volumes
        } else {
            vec![volume; channel_map.len()]
        }
    }

    #[cfg(test)]
    mod test {
        use crate::Channel;

        #[test]
        fn volume_balance_to_channel_volumes() {
            // Test conversions to and from a channel
            let channel_map = &[Channel::FL, Channel::FR];
            let inputs = vec![
                ((0.77, Some(0.32)), &[0.45653298, 0.14609055]),
                ((0.77, Some(0.57)), &[0.45653298, 0.2602238]),
                ((0.77, Some(0.68)), &[0.45653298, 0.31044245]),
                ((0.77, Some(0.74)), &[0.45653298, 0.33783442]),
                ((0.77, Some(1.00)), &[0.45653298, 0.45653298]),
                ((0.77, Some(1.32)), &[0.31044242, 0.45653298]),
                ((0.77, Some(1.57)), &[0.19630916, 0.45653298]),
                ((0.77, Some(1.68)), &[0.14609058, 0.45653298]),
                ((0.77, Some(1.74)), &[0.118698575, 0.45653298]),
            ];

            for ((volume, balance), channel_volumes) in inputs {
                let out = super::to_channel_volumes(channel_map, volume, balance);
                assert_eq!(&out, channel_volumes);
                let res = super::from_channel_volumes(&out);
                assert!((volume - res.0).abs() < 0.01, "{} != {}", volume, res.0);
                assert!(
                    balance.map_or_else(
                        || res.1 == Some(1.0),
                        |b| res.1.map_or_else(|| b == 1.0, |r| (b - r).abs() < 0.01)
                    ),
                    "{:?} != {:?}",
                    balance,
                    res.1
                );
            }
        }
    }
}
