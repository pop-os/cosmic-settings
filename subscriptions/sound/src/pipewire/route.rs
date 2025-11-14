// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use std::ffi::{c_float, c_int};

use crate::pipewire::{
    Availability, Channel, Direction,
    spa_utils::{array_from_pod, string_from_pod},
};
use libspa::{pod::Pod, utils::Id};

#[derive(Clone, Debug, Default)]
pub struct Route {
    pub index: i32,
    pub priority: i32,
    pub device: i32,
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

        // let mut index = 0;
        // let mut priority = 0;
        // let mut device = 0;
        // let mut available = Availability::Unknown;
        // let mut direction = Direction::Output;
        // let mut name = String::new();
        // let mut description = String::new();
        // let mut devices = Vec::new();

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
                _ => (),
            }
        }

        Some(this)
    }
}

// #[derive(Clone, Debug, Default)]
// pub struct RouteProps {
//     pub index: Option<i32>,
//     pub availability: Option<Availability>,
//     pub direction: Option<Direction>,
//     pub device: Option<i32>,
//     pub mute: Option<bool>,
//     pub monitor_mute: Option<bool>,
//     pub channel_map: Option<Vec<Channel>>,
//     pub channel_volumes: Option<Vec<f32>>,
// }

// impl RouteProps {
//     pub fn from_pod(pod: &Pod) -> Option<Self> {
//         let route = pod.as_object().ok()?;

//         let mut this = RouteProps::default();

//         this.index = route
//             .find_prop(Id(libspa_sys::SPA_PARAM_ROUTE_index))
//             .and_then(|prop| prop.value().get_int().ok());

//         this.availability = route
//             .find_prop(Id(libspa_sys::SPA_PARAM_ROUTE_available))
//             .map(|prop| match prop.value().get_id().unwrap().0 {
//                 libspa_sys::SPA_PARAM_AVAILABILITY_no => Availability::No,
//                 libspa_sys::SPA_PARAM_AVAILABILITY_yes => Availability::Yes,
//                 _ => Availability::Unknown,
//             });

//         this.direction = route
//             .find_prop(Id(libspa_sys::SPA_PARAM_ROUTE_direction))
//             .map(|prop| match prop.value().get_id().unwrap().0 {
//                 libspa_sys::SPA_DIRECTION_OUTPUT => Direction::Output,
//                 _ => Direction::Input,
//             });

//         this.device = route
//             .find_prop(Id(libspa_sys::SPA_PARAM_ROUTE_device))
//             .and_then(|prop| prop.value().get_int().ok());

//         if let Some(props) = route.find_prop(Id(libspa_sys::SPA_PARAM_ROUTE_props)) {
//             let props = props.value().as_object().ok()?;

//             this.mute = props
//                 .find_prop(Id(libspa_sys::SPA_PROP_mute))
//                 .and_then(|prop| prop.value().get_bool().ok());
//             this.monitor_mute = props
//                 .find_prop(Id(libspa_sys::SPA_PROP_monitorMute))
//                 .and_then(|prop| prop.value().get_bool().ok());
//             this.channel_map = props
//                 .find_prop(Id(libspa_sys::SPA_PROP_channelMap))
//                 .and_then(|prop| unsafe { array_from_pod::<Channel>(prop.value()) });
//             this.channel_volumes = props
//                 .find_prop(Id(libspa_sys::SPA_PROP_channelVolumes))
//                 .and_then(|prop| unsafe { array_from_pod::<c_float>(prop.value()) });
//         }

//         if this.index.is_none()
//             && this.availability.is_none()
//             && this.device.is_none()
//             && this.mute.is_none()
//             && this.monitor_mute.is_none()
//             && this.channel_map.is_none()
//             && this.channel_volumes.is_none()
//         {
//             None
//         } else {
//             Some(this)
//         }
//     }

//     pub fn merge(&mut self, other: RouteProps) {
//         if other.index.is_some() {
//             self.index = other.index;
//         }

//         if other.availability.is_some() {
//             self.availability = other.availability;
//         }

//         if other.device.is_some() {
//             self.device = other.device;
//         }

//         if other.mute.is_some() {
//             self.mute = other.mute
//         }

//         if other.monitor_mute.is_some() {
//             self.monitor_mute = other.monitor_mute;
//         }

//         if other.channel_map.is_some() {
//             self.channel_map = other.channel_map;
//         }

//         if other.channel_volumes.is_some() {
//             self.channel_volumes = other.channel_volumes;
//         }
//     }
// }
