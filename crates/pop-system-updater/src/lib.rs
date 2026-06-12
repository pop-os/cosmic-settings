// Copyright 2021 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

pub mod client;
mod config;

pub use config::{Config, NotificationConfig};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zvariant::Type;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Type)]
#[repr(u32)]
pub enum Frequency {
    Weekly = 0,
    Daily = 1,
    Monthly = 2,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
pub struct Schedule {
    pub interval: Interval,
    pub hour: u8,
    pub minute: u8,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Deserialize_repr, Serialize_repr, Type)]
pub enum Interval {
    Monday = 1,
    Tuesday = 1 << 1,
    Wednesday = 1 << 2,
    Thursday = 1 << 3,
    Friday = 1 << 4,
    Saturday = 1 << 5,
    Sunday = 1 << 6,
    Weekdays = 1 << 7,
}
