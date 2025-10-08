// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

// #![deny(missing_docs)]

pub use pipewire::channel::Sender;

use cosmic::iced_futures::{self, Subscription, stream};
use futures::{SinkExt, executor::block_on};
use pipewire::{
    context::Context as PwContext,
    main_loop::MainLoop as PwMainLoop,
    node::{Node, NodeInfoRef, NodeState},
    proxy::{Listener, ProxyT},
    types::ObjectType,
};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    rc::Rc,
    thread::JoinHandle,
};

pub fn subscription() -> iced_futures::Subscription<DeviceEvent> {
    Subscription::run_with_id(
        "pipewire",
        stream::channel(20, |sender| async {
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

/// Node event`
#[derive(Debug)]
pub enum NodeEvent<'a> {
    /// Node info
    NodeInfo(u32, &'a NodeInfoRef),
    /// Node removal
    Remove(u32),
}

/// Device event
#[derive(Clone, Debug)]
pub enum DeviceEvent {
    /// A new device was detected.
    Add(Device),
    /// A device with the given object_id was removed.
    Remove(u32),
}

/// Device information
#[must_use]
#[derive(Clone, Debug)]
pub struct Device {
    pub object_id: u32,
    pub variant: DeviceVariant,
    pub media_class: MediaClass,
    pub product_name: String,
    pub node_name: String,
    pub state: DeviceState,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum DeviceVariant {
    Alsa { alsa_card: u32 },
    Bluez5 { address: String },
    Unknown {},
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeviceState {
    Idle,
    Running,
    Creating,
    Suspended,
    Error(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MediaClass {
    Source,
    Sink,
}

impl Device {
    /// Attains process info from a pipewire info node.
    #[must_use]
    pub fn from_node(info: &NodeInfoRef) -> Option<Self> {
        let props = info.props()?;

        let (variant, product_name) = if let Some(alsa_card) =
            props.get("alsa.card").and_then(|v| v.parse::<u32>().ok())
        {
            let device_profile_description = props.get("device.profile.description")?.to_owned();

            let description = props.get("node.description")?;

            let description = description
                .strip_suffix(&device_profile_description)
                .map(str::trim_end)
                .unwrap_or(description)
                .replace("High Definition Audio", "HD Audio");

            (DeviceVariant::Alsa { alsa_card }, description)
        } else if let Some(address) = props
            .get("api.bluez5.address")
            .and_then(|v| v.parse::<String>().ok())
        {
            (
                DeviceVariant::Bluez5 {
                    address: address.to_owned(),
                },
                props.get("node.description")?.to_owned(),
            )
        } else {
            (
                DeviceVariant::Unknown {},
                props.get("node.description")?.to_owned(),
            )
        };

        Some(Device {
            object_id: props.get("object.id")?.parse::<u32>().ok()?,
            variant,
            media_class: match props.get("media.class")? {
                "Audio/Sink" => MediaClass::Sink,
                "Audio/Source" => MediaClass::Source,
                _ => return None,
            },
            product_name,
            node_name: props.get("node.name")?.to_owned(),
            state: match info.state() {
                NodeState::Idle => DeviceState::Idle,
                NodeState::Running => DeviceState::Running,
                NodeState::Creating => DeviceState::Creating,
                NodeState::Suspended => DeviceState::Suspended,
                NodeState::Error(why) => DeviceState::Error(why.to_owned()),
            },
        })
    }
}

/// Monitors the devices from a given ``PipeWire`` socket.
///
/// ``PipeWire`` sockets are found in `/run/user/{{UID}}/pipewire-0`.
pub fn devices_from_socket(
    pw_cancel: pipewire::channel::Receiver<()>,
    mut on_event: futures::channel::mpsc::Sender<DeviceEvent>,
) {
    let mut managed = BTreeMap::new();

    let _res = nodes_from_socket(pw_cancel, move |main_loop, event| match event {
        NodeEvent::NodeInfo(pw_id, info) => {
            if let Some(device) = Device::from_node(info) {
                if managed.insert(pw_id, device.object_id).is_none() {
                    if block_on(on_event.send(DeviceEvent::Add(device))).is_err() {
                        main_loop.quit();
                    }
                }
            }
        }

        NodeEvent::Remove(pw_id) => {
            if let Some(object_id) = managed.remove(&pw_id) {
                if block_on(on_event.send(DeviceEvent::Remove(object_id))).is_err() {
                    main_loop.quit();
                }
            }
        }
    });
}

/// Listens to information about nodes, passing that info into a callback.
///
/// # Errors
///
/// Errors if the pipewire connection fails
pub fn nodes_from_socket(
    pw_cancel: pipewire::channel::Receiver<()>,
    on_event: impl FnMut(&PwMainLoop, NodeEvent) + 'static,
) -> Result<(), Box<dyn std::error::Error>> {
    let main_loop = PwMainLoop::new(None)?;
    let context = PwContext::new(&main_loop)?;
    let core = context.connect(None)?;

    // Exit main loop on receivering terminate message.
    let _cancel_rx = pw_cancel.attach(main_loop.loop_(), {
        let main_loop = main_loop.clone();
        move |_| main_loop.quit()
    });

    let registry = Rc::new(core.get_registry()?);
    let registry_weak = Rc::downgrade(&registry);

    let proxies = Rc::new(RefCell::new(HashMap::new()));
    let on_event = Rc::new(RefCell::new(on_event));

    let main_loop_clone = main_loop.clone();

    let _registry_listener = registry
        .add_listener_local()
        .global(move |obj| {
            let Some(registry) = registry_weak.upgrade() else {
                return;
            };

            let attached_proxy: Option<(Box<dyn ProxyT>, Box<dyn Listener>)> = match obj.type_ {
                ObjectType::Node => {
                    let Ok(node): Result<Node, _> = registry.bind(obj) else {
                        return;
                    };

                    let on_event_weak = Rc::downgrade(&on_event);
                    let main_loop = main_loop_clone.clone();
                    let id = node.upcast_ref().id();

                    let listener = node
                        .add_listener_local()
                        .info(move |info| {
                            if let Some(on_event) = on_event_weak.upgrade() {
                                on_event.borrow_mut()(&main_loop, NodeEvent::NodeInfo(id, info));
                            }
                        })
                        .register();

                    Some((Box::new(node), Box::new(listener)))
                }

                _ => None,
            };

            if let Some((proxy_spe, listener)) = attached_proxy {
                let proxy = proxy_spe.upcast_ref();
                let id = proxy.id();
                let (object_type, _object_version) = proxy.get_type();

                let proxies_weak = Rc::downgrade(&proxies);
                let on_event_weak = Rc::downgrade(&on_event);
                let main_loop = main_loop_clone.clone();

                let remove_listener = proxy
                    .add_listener_local()
                    .removed(move || {
                        if object_type == ObjectType::Node {
                            if let Some(on_event) = on_event_weak.upgrade() {
                                on_event.borrow_mut()(&main_loop, NodeEvent::Remove(id));
                            }
                        }

                        if let Some(proxies) = proxies_weak.upgrade() {
                            proxies.borrow_mut().remove(&id);
                        }
                    })
                    .register();

                proxies
                    .borrow_mut()
                    .insert(id, (proxy_spe, listener, remove_listener));
            }
        })
        .register();

    main_loop.run();
    Ok(())
}
