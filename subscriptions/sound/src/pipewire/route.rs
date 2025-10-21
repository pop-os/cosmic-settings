// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Currently unusued

use crate::pipewire::{Availability, string_from_pod};
use libspa::pod::Pod;

#[derive(Clone, Debug)]
pub struct Route {
    pub index: i32,
    pub priority: i32,
    pub available: Availability,
    pub name: String,
    pub description: String,
}

impl Route {
    pub fn from_pod(pod: &Pod) -> Option<Self> {
        let mut index = 0;
        let mut priority = 0;
        let mut available = Availability::Unknown;
        let mut name = String::new();
        let mut description = String::new();

        let profile = pod.as_object().ok()?;

        for prop in profile.props() {
            match prop.key().0 {
                libspa_sys::SPA_PARAM_ROUTE_index => index = prop.value().get_int().ok()?,
                libspa_sys::SPA_PARAM_ROUTE_priority => priority = prop.value().get_int().ok()?,
                libspa_sys::SPA_PARAM_ROUTE_available => {
                    available = match prop.value().get_id().unwrap().0 {
                        libspa_sys::SPA_PARAM_AVAILABILITY_no => Availability::No,
                        libspa_sys::SPA_PARAM_AVAILABILITY_yes => Availability::Yes,
                        _ => Availability::Unknown,
                    };
                }
                libspa_sys::SPA_PARAM_ROUTE_name => name = string_from_pod(prop.value())?,
                libspa_sys::SPA_PARAM_ROUTE_description => {
                    description = string_from_pod(prop.value())?;
                }
                _ => (),
            }
        }

        Some(Self {
            index,
            priority,
            available,
            name,
            description,
        })
    }
}
