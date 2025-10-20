// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use pipewire::node::{NodeInfoRef, NodeState};

/// Node information
#[must_use]
#[derive(Clone, Debug)]
pub struct Node {
    pub object_id: u32,
    pub audio_channels: u32,
    pub audio_position: String,
    pub card_profile_device: Option<u32>,
    pub description: String,
    pub device_id: Option<u32>,
    pub device_profile_description: String,
    pub device_profile_pro: bool,
    pub icon_name: String,
    pub media_class: MediaClass,
    pub node_name: String,
    pub state: State,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
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

impl Node {
    /// Attains process info from a pipewire info node.
    #[must_use]
    pub fn from_node(info: &NodeInfoRef) -> Option<Self> {
        let props = info.props()?;

        let mut audio_channels = 1;
        let mut audio_position = String::new();
        let mut card_profile_device = None;
        let mut device_id = None;
        let mut device_profile_description: &str = "";
        let mut device_profile_pro = false;
        let mut icon_name = String::new();
        let mut media_class = None;
        let mut node_description: &str = "";
        let mut node_name = String::new();
        let mut object_id = None;

        for (entry, value) in props.iter() {
            match entry {
                "device.id" => device_id = value.parse::<u32>().ok(),
                "object.id" => object_id = Some(value.parse::<u32>().ok()?),

                // 2
                "audio.channels" => audio_channels = value.parse::<u32>().unwrap_or(1),

                // FL,FR
                "audio.position" => audio_position = value.to_owned(),

                // 0
                "card.profile.device" => card_profile_device = Some(value.parse::<u32>().ok()?),

                // Analog Stereo (ALSA only)
                "device.profile.description" => {
                    device_profile_description = value;
                }

                // false
                "device.profile.pro" => device_profile_pro = value == "true",

                // audio-card-analog
                "device.icon-name" => icon_name = value.to_owned(),

                "media.class" => {
                    media_class = Some(match value {
                        "Audio/Sink" => MediaClass::Sink,
                        "Audio/Source" => MediaClass::Source,
                        _ => return None,
                    })
                }

                // alsa_input.pci-0000_66_00.6.analog-stereo
                "node.name" => node_name = value.to_owned(),

                // Family 17h/19h HD Audio Controller Analog Stereo
                "node.description" => node_description = value,

                _ => (),
            }
        }

        let device = Node {
            object_id: object_id?,
            device_id,
            card_profile_device,
            media_class: media_class?,
            description: if device_profile_description.is_empty() {
                node_description.to_owned()
            } else {
                let device_name = node_description
                    .strip_suffix(device_profile_description)
                    .unwrap_or(node_description)
                    .trim_ascii_end();
                device_name.to_owned()
            },
            device_profile_description: device_profile_description.to_owned(),
            device_profile_pro,
            icon_name,
            audio_channels,
            audio_position,
            node_name,
            state: match info.state() {
                NodeState::Idle => State::Idle,
                NodeState::Running => State::Running,
                NodeState::Creating => State::Creating,
                NodeState::Suspended => State::Suspended,
                NodeState::Error(why) => State::Error(why.to_owned()),
            },
        };

        Some(device)
    }
}
