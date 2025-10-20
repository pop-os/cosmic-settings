// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

// #![deny(missing_docs)]

pub mod device;
pub use device::{Device, MediaClass};

// mod port;
// pub use port::Port;

mod profile;
pub use profile::Profile;

// mod route;
// pub use route::Route;

use libspa::{param::ParamType, pod::Pod};
pub use pipewire::channel::Sender;

use cosmic::iced_futures::{self, Subscription, stream};
use futures::{SinkExt, executor::block_on};
use pipewire::{
    device::DeviceListener,
    main_loop::MainLoop as PwMainLoop,
    node::{Node, NodeInfoRef, NodeListener},
    proxy::{ProxyListener, ProxyT},
    types::ObjectType,
};
use std::{
    any::TypeId,
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    ffi::CStr,
    rc::Rc,
    thread::JoinHandle,
    u32,
};

/// Node event`
#[derive(Debug)]
pub enum NodeEvent<'a> {
    /// Sets the active profile for a device
    ActiveProfile(u32, Profile),
    /// Node info
    NodeInfo(u32, &'a NodeInfoRef),
    /// A profile parameter was enumerated
    Profile(u32, Profile),
    /// Node removal
    Remove(u32),
}

/// Device event
#[derive(Clone, Debug)]
pub enum DeviceEvent {
    /// Set the active profile for a device
    ActiveProfile(u32, Profile),
    /// A new device was detected.
    Add(Device),
    /// A profile was enumerated
    AddProfile(u32, Profile),
    /// A device with the given object_id was removed.
    Remove(u32),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Availability {
    Unknown,
    No,
    Yes,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Direction {
    Input,
    Output,
}

struct Proxies {
    devices: HashMap<u32, (u32, pipewire::device::Device, DeviceListener, ProxyListener)>,
    nodes: HashMap<u32, (pipewire::node::Node, NodeListener, ProxyListener)>,
}

pub fn subscription() -> iced_futures::Subscription<DeviceEvent> {
    Subscription::run_with_id(
        TypeId::of::<DeviceEvent>(),
        stream::channel(1, |sender| async {
            _ = thread(sender);

            futures::future::pending().await
        }),
    )
}

pub fn thread(
    on_event: futures::channel::mpsc::Sender<DeviceEvent>,
) -> (JoinHandle<()>, pipewire::channel::Sender<()>) {
    let (pw_tx, pw_rx) = pipewire::channel::channel();

    let handle = std::thread::spawn(move || {
        devices_from_socket(pw_rx, on_event);
    });

    (handle, pw_tx)
}

/// Monitors devices from the system's ``PipeWire`` socket.
///
/// ``PipeWire`` sockets are found in `/run/user/{{UID}}/pipewire-0`.
pub fn devices_from_socket(
    pw_cancel: pipewire::channel::Receiver<()>,
    mut on_event: futures::channel::mpsc::Sender<DeviceEvent>,
) {
    let mut managed = BTreeMap::new();
    let (device_enum_tx, device_enum_rx) = pipewire::channel::channel();

    let _res = nodes_from_socket(
        pw_cancel,
        device_enum_rx,
        move |main_loop, event| match event {
            NodeEvent::NodeInfo(pw_id, info) => {
                if let Some(device) = Device::from_node(info) {
                    _ = device_enum_tx.send(device.device_id).unwrap();
                    if managed
                        .insert(pw_id, (device.object_id, device.device_id))
                        .is_none()
                    {
                        if block_on(on_event.send(DeviceEvent::Add(device))).is_err() {
                            main_loop.quit();
                        }
                    }
                }
            }

            NodeEvent::Profile(device_id, profile) => {
                let Some(object_id) = managed
                    .values()
                    .find(|(_, dev_id)| device_id == *dev_id)
                    .map(|(obj_id, _)| *obj_id)
                else {
                    return;
                };

                if block_on(on_event.send(DeviceEvent::AddProfile(object_id, profile))).is_err() {
                    main_loop.quit();
                }
            }

            NodeEvent::ActiveProfile(device_id, profile) => {
                let Some(object_id) = managed
                    .values()
                    .find(|(_, dev_id)| device_id == *dev_id)
                    .map(|(obj_id, _)| *obj_id)
                else {
                    return;
                };

                if block_on(on_event.send(DeviceEvent::ActiveProfile(object_id, profile))).is_err()
                {
                    main_loop.quit();
                }
            }

            NodeEvent::Remove(pw_id) => {
                if let Some((object_id, _)) = managed.remove(&pw_id) {
                    if block_on(on_event.send(DeviceEvent::Remove(object_id))).is_err() {
                        main_loop.quit();
                    }
                }
            }
        },
    );
}

/// Listens to information about nodes, passing that info into a callback.
///
/// # Errors
///
/// Errors if the pipewire connection fails
pub fn nodes_from_socket(
    pw_cancel: pipewire::channel::Receiver<()>,
    pw_enum_device: pipewire::channel::Receiver<u32>,
    on_event: impl FnMut(&PwMainLoop, NodeEvent) + 'static,
) -> Result<(), Box<dyn std::error::Error>> {
    let main_loop = pipewire::main_loop::MainLoopRc::new(None)?;
    let context = pipewire::context::ContextRc::new(&main_loop, None)?;
    let core = context.connect_rc(None)?;
    let registry = core.get_registry_rc()?;
    let on_event = Rc::new(RefCell::new(on_event));

    let proxies = Rc::new(RefCell::new(Proxies {
        devices: HashMap::new(),
        nodes: HashMap::new(),
    }));

    // Receives device object IDs for enumerating its profiles.
    let _device_enum_rx = pw_enum_device.attach(main_loop.loop_(), {
        let proxies_weak = Rc::downgrade(&proxies);
        move |device_id| {
            let Some(proxies) = proxies_weak.upgrade() else {
                return;
            };

            if let Some((_, device, _, _)) = proxies
                .borrow()
                .devices
                .values()
                .find(|(id, ..)| *id == device_id)
            {
                device.enum_params(0, Some(ParamType::EnumProfile), 0, u32::MAX);
                device.enum_params(2, Some(ParamType::Profile), 0, u32::MAX);
            }
        }
    });

    let registry_weak = registry.downgrade();
    let main_loop_weak = main_loop.downgrade();

    // Exit main loop on receivering terminate message.
    let _cancel_rx = pw_cancel.attach(main_loop.loop_(), {
        let main_loop = main_loop.downgrade();
        move |_| {
            if let Some(main_loop) = main_loop.upgrade() {
                main_loop.quit();
            }
        }
    });

    let _registry_listener = registry
        .add_listener_local()
        .global(move |obj| {
            let Some(registry) = registry_weak.upgrade() else {
                return;
            };

            let Some(main_loop) = main_loop_weak.upgrade() else {
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
                            let proxies = Rc::downgrade(&proxies);
                            move |info| {
                                let Some(props) = info.props() else {
                                    return;
                                };

                                let Some(proxies) = proxies.upgrade() else {
                                    return;
                                };

                                // Map the device's pipewire ID to
                                if let Some(Ok(object_id)) =
                                    props.get("object.id").map(str::parse::<u32>)
                                {
                                    if let Some(entry) =
                                        proxies.borrow_mut().devices.get_mut(&pw_id)
                                    {
                                        entry.0 = object_id;
                                    }
                                }
                            }
                        })
                        .param({
                            let main_loop_weak = main_loop.downgrade();
                            let on_event_weak = Rc::downgrade(&on_event);
                            let proxies = Rc::downgrade(&proxies);

                            move |_seq, param_type, _index, _next, param| {
                                let Some(pod) = param else {
                                    return;
                                };

                                let Some(main_loop) = main_loop_weak.upgrade() else {
                                    return;
                                };

                                let Some(on_event) = on_event_weak.upgrade() else {
                                    return;
                                };

                                let Some(proxies) = proxies.upgrade() else {
                                    return;
                                };

                                let Some(&(device_id, ..)) = proxies.borrow().devices.get(&pw_id)
                                else {
                                    return;
                                };

                                match param_type {
                                    ParamType::EnumProfile => {
                                        if let Some(profile) = Profile::from_pod(pod) {
                                            on_event.borrow_mut()(
                                                &main_loop,
                                                NodeEvent::Profile(device_id, profile),
                                            );
                                        }
                                    }
                                    ParamType::Profile => {
                                        if let Some(profile) = Profile::from_pod(pod) {
                                            on_event.borrow_mut()(
                                                &main_loop,
                                                NodeEvent::ActiveProfile(device_id, profile),
                                            );
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
                            let proxies_weak = Rc::downgrade(&proxies);
                            move || {
                                if let Some(proxies) = proxies_weak.upgrade() {
                                    proxies.borrow_mut().devices.remove(&pw_id);
                                }
                            }
                        })
                        .register();

                    proxies
                        .borrow_mut()
                        .devices
                        .insert(pw_id, (0, device, listener, remove_listener));
                }

                ObjectType::Node => {
                    let Ok(node) = registry.bind::<Node, _>(obj) else {
                        return;
                    };

                    let id = node.upcast_ref().id();

                    let listener = node
                        .add_listener_local()
                        .info({
                            let main_loop_weak = main_loop.downgrade();
                            let on_event_weak = Rc::downgrade(&on_event);
                            move |info| {
                                let Some(main_loop) = main_loop_weak.upgrade() else {
                                    return;
                                };

                                if let Some(on_event) = on_event_weak.upgrade() {
                                    on_event.borrow_mut()(
                                        &main_loop,
                                        NodeEvent::NodeInfo(id, info),
                                    );
                                }
                            }
                        })
                        .register();

                    let remove_listener = node
                        .upcast_ref()
                        .add_listener_local()
                        .removed({
                            let main_loop_weak = main_loop.downgrade();
                            let proxies_weak = Rc::downgrade(&proxies);
                            let on_event_weak = Rc::downgrade(&on_event);
                            move || {
                                let Some(main_loop) = main_loop_weak.upgrade() else {
                                    return;
                                };

                                if let Some(on_event) = on_event_weak.upgrade() {
                                    on_event.borrow_mut()(&main_loop, NodeEvent::Remove(id));
                                }

                                if let Some(proxies) = proxies_weak.upgrade() {
                                    proxies.borrow_mut().nodes.remove(&id);
                                }
                            }
                        })
                        .register();

                    proxies
                        .borrow_mut()
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

fn string_from_pod(pod: &Pod) -> Option<String> {
    let mut cstr = std::ptr::null();

    unsafe {
        // SAFETY: The reference to Pod should be valid if pipewire-rs is implemented correctly.
        // libspa will set our null pointer to C string if the pod's value is a string.
        if libspa_sys::spa_pod_get_string(pod.as_raw_ptr(), &mut cstr) == 0 {
            if !cstr.is_null() {
                return Some(String::from_utf8_lossy(CStr::from_ptr(cstr).to_bytes()).into_owned());
            }
        }
    }

    None
}
