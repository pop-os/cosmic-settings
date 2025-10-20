// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Currently unusued

use crate::pipewire::Direction;
use pipewire::port::PortInfoRef;

#[must_use]
#[derive(Clone, Debug)]
pub struct Port {
    pub node_id: u32,
    pub object_id: u32,
    pub port_id: u32,
    pub audio_channel: String,
    pub format_dsp: String,
    pub object_path: String,
    pub port_direction: Direction,
    pub port_group: String,
    pub port_name: String,
    pub port_alias: String,
    pub port_physical: bool,
    pub port_terminal: bool,
    pub port_monitor: bool,
}

impl Port {
    /// Attains process info from a pipewire info port.
    #[must_use]
    pub fn from_port(info: &PortInfoRef) -> Option<Self> {
        let props = info.props()?;
        let object_id = info.id();
        let port_direction = match info.direction() {
            libspa::utils::Direction::Input => Direction::Input,
            libspa::utils::Direction::Output => Direction::Output,
            _ => return None,
        };

        let mut node_id = 0;
        let mut port_id = 0;
        let mut port_monitor = false;
        let mut port_physical = false;
        let mut port_terminal = false;

        let mut audio_channel = String::new();
        let mut format_dsp = String::new();
        let mut object_path = String::new();
        let mut port_alias = String::new();
        let mut port_group = String::new();
        let mut port_name = String::new();

        for (entry, value) in props.iter() {
            match entry {
                // 32 bit float mono audio
                "format.dsp" => format_dsp = value.to_owned(),
                // FR
                "audio.channel" => audio_channel = value.to_owned(),
                // playback
                "port.group" => port_group = value.to_owned(),
                // 1
                "port.id" => port_id = value.parse::<u32>().ok()?,
                // false
                "port.monitor" => port_monitor = value == "true",
                // true
                "port.physical" => port_physical = value == "true",
                // true
                "port.terminal" => port_terminal = value == "true",
                // alsa:acp:Device:3:playback:playback_1
                "object.path" => object_path = value.to_owned(),
                // playback_FR
                "port.name" => port_name = value.to_owned(),
                // MosArt USB Audio Device:playback_FR
                "port.alias" => port_alias = value.to_owned(),
                // 59
                "node.id" => node_id = value.parse::<u32>().ok()?,
                _ => (),
            }
        }

        let port = Port {
            format_dsp,
            audio_channel,
            port_id,
            port_direction,
            object_path,
            port_name,
            port_alias,
            port_group,
            port_monitor,
            port_physical,
            port_terminal,
            node_id,
            object_id,
        };

        Some(port)
    }
}
