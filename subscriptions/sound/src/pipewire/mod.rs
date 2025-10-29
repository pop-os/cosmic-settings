// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

// #![deny(missing_docs)]

pub mod device;
pub use device::Device;

pub mod node;
use intmap::IntMap;
pub use node::{MediaClass, Node};

mod profile;
pub use profile::Profile;

mod route;
pub use route::Route;

use libspa::{param::ParamType, pod::Pod};
pub use pipewire::channel::Sender;

use cosmic::iced_futures::{self, Subscription, stream};
use futures::{SinkExt, executor::block_on};
pub use pipewire::channel::channel;
use pipewire::{
    device::DeviceListener,
    main_loop::MainLoopWeak,
    node::NodeListener,
    proxy::{ProxyListener, ProxyT},
    types::ObjectType,
};
use std::{any::TypeId, cell::RefCell, ffi::CStr, rc::Rc, u32};

use crate::{DeviceId, NodeId};
pub type PipewireId = u32;

pub fn subscription() -> iced_futures::Subscription<Event> {
    Subscription::run_with_id(
        TypeId::of::<Event>(),
        stream::channel(1, |sender| async {
            let (_tx, rx) = channel();
            std::thread::spawn(move || run(rx, sender));

            futures::future::pending().await
        }),
    )
}

pub fn run(
    rx: pipewire::channel::Receiver<Request>,
    on_event: futures::channel::mpsc::Sender<Event>,
) -> Result<(), pipewire::Error> {
    let main_loop = pipewire::main_loop::MainLoopRc::new(None)?;
    let context = pipewire::context::ContextRc::new(&main_loop, None)?;
    let core = context.connect_rc(None)?;
    let registry = core.get_registry_rc()?;

    let state = Rc::new(RefCell::new(State {
        nodes: IntMap::new(),
        proxies: Proxies {
            devices: IntMap::new(),
            nodes: IntMap::new(),
        },
        main_loop: main_loop.downgrade(),
        on_event,
    }));

    let _request_handler = rx.attach(main_loop.loop_(), {
        let state = Rc::downgrade(&state);
        move |request| {
            match request {
                // Receives device object IDs for enumerating its profiles.
                Request::EnumerateDevice(id) => {
                    if let Some(state) = state.upgrade() {
                        state.borrow_mut().enumerate_device(id);
                    }
                }

                // Exit main loop on receivering terminate message.
                Request::Quit => {
                    if let Some(state) = state.upgrade() {
                        state.borrow_mut().quit();
                    }
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

                    let pw_id = device.upcast_ref().id();

                    let listener = device
                        .add_listener_local()
                        .info({
                            let state = Rc::downgrade(&state);
                            move |info| {
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
                        .insert(id, (node, listener, remove_listener));
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
    /// A device with the given device_id was removed.
    RemoveDevice(DeviceId),
    /// A node with the given object_id was removed.
    RemoveNode(NodeId),
}

#[derive(Clone, Debug)]
pub enum Request {
    EnumerateDevice(DeviceId),
    Quit,
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum Availability {
    #[default]
    Unknown,
    No,
    Yes,
}

#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum Direction {
    Input,
    #[default]
    Output,
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
    nodes: IntMap<PipewireId, (pipewire::node::Node, NodeListener, ProxyListener)>,
}

struct State {
    nodes: IntMap<PipewireId, (NodeId, Option<DeviceId>)>,
    pub(self) proxies: Proxies,
    main_loop: MainLoopWeak,
    on_event: futures::channel::mpsc::Sender<Event>,
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
        self.enumerate_device(device_id);
    }

    fn add_node(&mut self, id: PipewireId, node: Node) {
        self.nodes.insert(id, (node.object_id, node.device_id));
        self.on_event(Event::AddNode(node));
    }

    fn add_profile(&mut self, id: DeviceId, profile: Profile) {
        self.on_event(Event::AddProfile(id, profile));
    }

    fn add_route(&mut self, id: DeviceId, index: u32, route: Route) {
        self.on_event(Event::AddRoute(id, index, route));
    }

    fn enumerate_device(&mut self, id: DeviceId) {
        if let Some((_, device, _, _)) = self
            .proxies
            .devices
            .values()
            .find(|(device_id, ..)| id == *device_id)
        {
            device.enum_params(0, Some(ParamType::EnumProfile), 0, u32::MAX);
            device.enum_params(1, Some(ParamType::EnumRoute), 0, u32::MAX);
            device.enum_params(2, Some(ParamType::Profile), 0, u32::MAX);
            device.enum_params(3, Some(ParamType::Route), 0, u32::MAX);
        }
    }

    fn on_event(&mut self, event: Event) {
        if block_on(self.on_event.send(event)).is_err() {
            if let Some(main_loop) = self.main_loop.upgrade() {
                main_loop.quit();
            }
        }
    }

    fn quit(&mut self) {
        if let Some(main_loop) = self.main_loop.upgrade() {
            main_loop.quit();
        }
    }

    fn remove_device(&mut self, id: PipewireId) {
        if let Some((device_id, ..)) = self.proxies.devices.remove(id) {
            self.on_event(Event::RemoveDevice(device_id));
        }
    }

    fn remove_node(&mut self, id: PipewireId) {
        if let Some((node_id, _)) = self.nodes.remove(id) {
            self.on_event(Event::RemoveNode(node_id));
        }

        self.proxies.nodes.remove(id);
    }
}

fn string_from_pod(pod: &Pod) -> Option<String> {
    if !pod.is_string() {
        return None;
    }

    let mut cstr = std::ptr::null();

    unsafe {
        // SAFETY: Pod is checked to be a string beforehand
        if libspa_sys::spa_pod_get_string(pod.as_raw_ptr(), &mut cstr) == 0 {
            if !cstr.is_null() {
                return Some(String::from_utf8_lossy(CStr::from_ptr(cstr).to_bytes()).into_owned());
            }
        }
    }

    None
}

/// SAFETY: Must be absolutely certain that the array is an integer array.
unsafe fn int_array_from_pod(pod: &Pod) -> Option<Vec<i32>> {
    if !pod.is_array() {
        return None;
    }

    let mut len = 0;

    unsafe {
        let array: *mut std::ffi::c_int =
            libspa_sys::spa_pod_get_array(pod.as_raw_ptr(), &mut len).cast();

        if array.is_null() {
            return None;
        }

        Some(std::slice::from_raw_parts(array, len as usize).to_owned())
    }
}
