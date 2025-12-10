// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use std::ffi::{c_float, c_int};

use crate::{
    Availability, Channel, Direction,
    spa_utils::{array_from_pod, string_from_pod},
};
use libspa::{pod::Pod, utils::Id};

#[derive(Clone, Debug, Default)]
pub struct Route {
    pub index: i32,
    pub priority: i32,
    pub device: i32,
    #[cfg(feature = "route-port-type")]
    pub port_type: PortType,
    pub available: Availability,
    pub direction: Direction,
    pub name: String,
    pub description: String,
    pub devices: Vec<i32>,
    pub props: Option<RouteProps>,
}

#[derive(Clone, Debug, Default)]
pub struct RouteProps {
    pub mute: Option<bool>,
    pub monitor_mute: Option<bool>,
    pub channel_map: Option<Vec<Channel>>,
    pub channel_volumes: Option<Vec<f32>>,
}

impl Route {
    pub fn from_pod(pod: &Pod) -> Option<Self> {
        let mut this = Route::default();

        let route = pod.as_object().ok()?;

        for prop in route.props() {
            match prop.key().0 {
                libspa_sys::SPA_PARAM_ROUTE_index => this.index = prop.value().get_int().ok()?,
                libspa_sys::SPA_PARAM_ROUTE_priority => {
                    this.priority = prop.value().get_int().ok()?
                }
                libspa_sys::SPA_PARAM_ROUTE_device => this.device = prop.value().get_int().ok()?,
                libspa_sys::SPA_PARAM_ROUTE_available => {
                    this.available = match prop.value().get_id().unwrap().0 {
                        libspa_sys::SPA_PARAM_AVAILABILITY_no => Availability::No,
                        libspa_sys::SPA_PARAM_AVAILABILITY_yes => Availability::Yes,
                        _ => Availability::Unknown,
                    };
                }
                libspa_sys::SPA_PARAM_ROUTE_name => this.name = string_from_pod(prop.value())?,
                libspa_sys::SPA_PARAM_ROUTE_description => {
                    this.description = string_from_pod(prop.value())?;
                }
                libspa_sys::SPA_PARAM_ROUTE_direction => {
                    this.direction = match prop.value().get_id().unwrap().0 {
                        libspa_sys::SPA_DIRECTION_OUTPUT => Direction::Output,
                        _ => Direction::Input,
                    }
                }
                libspa_sys::SPA_PARAM_ROUTE_devices => {
                    if let Some(data) = unsafe { array_from_pod::<c_int>(prop.value()) } {
                        this.devices = data;
                    }
                }
                libspa_sys::SPA_PARAM_ROUTE_props => {
                    let props = prop.value().as_object().ok()?;

                    this.props = Some(RouteProps {
                        mute: props
                            .find_prop(Id(libspa_sys::SPA_PROP_mute))
                            .and_then(|prop| prop.value().get_bool().ok()),
                        monitor_mute: props
                            .find_prop(Id(libspa_sys::SPA_PROP_monitorMute))
                            .and_then(|prop| prop.value().get_bool().ok()),
                        channel_map: props
                            .find_prop(Id(libspa_sys::SPA_PROP_channelMap))
                            .and_then(|prop| unsafe { array_from_pod::<Channel>(prop.value()) }),
                        channel_volumes: props
                            .find_prop(Id(libspa_sys::SPA_PROP_channelVolumes))
                            .and_then(|prop| unsafe { array_from_pod::<c_float>(prop.value()) }),
                    })
                }
                #[cfg(feature = "route-port-type")]
                libspa_sys::SPA_PARAM_ROUTE_info => {
                    if let Ok(prop) = prop.value().as_struct() {
                        let mut fields = prop.fields().skip(1);
                        while let Some((key, value)) = fields.next().zip(fields.next()) {
                            if let Some("port.type") = string_from_pod(key).as_deref() {
                                if let Some(value) = string_from_pod(value) {
                                    this.port_type = PortType::from(value.as_str());
                                }
                                break;
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        Some(this)
    }
}

#[cfg(feature = "route-port-type")]
#[derive(Clone, Debug, Default)]
pub enum PortType {
    #[default]
    Unknown = 0,
    Aux = 1,
    Speaker = 2,
    Headphones = 3,
    Line = 4,
    Mic = 5,
    Headset = 6,
    Handset = 7,
    Earpiece = 8,
    SPDIF = 9,
    HDMI = 10,
    TV = 11,
    Radio = 12,
    Video = 13,
    USB = 14,
    Bluetooth = 15,
    Portable = 16,
    Handsfree = 17,
    Car = 18,
    HiFi = 19,
    Phone = 20,
    Network = 21,
    Analog = 22,
}

#[cfg(feature = "route-port-type")]
impl From<&str> for PortType {
    fn from(value: &str) -> Self {
        match value {
            "analog" => Self::Analog,
            "aux" => Self::Aux,
            "speaker" => Self::Speaker,
            "headphones" => Self::Headphones,
            "line" => Self::Line,
            "mic" => Self::Mic,
            "headset" => Self::Headset,
            "handset" => Self::Handset,
            "earpiece" => Self::Earpiece,
            "spidf" => Self::SPDIF,
            "hdmi" => Self::HDMI,
            "tv" => Self::TV,
            "radio" => Self::Radio,
            "video" => Self::Video,
            "usb" => Self::USB,
            "bluetooth" => Self::Bluetooth,
            "portable" => Self::Portable,
            "handsfree" => Self::Handsfree,
            "car" => Self::Car,
            "hifi" => Self::HiFi,
            "phone" => Self::Phone,
            "network" => Self::Network,
            _ => Self::Unknown,
        }
    }
}
