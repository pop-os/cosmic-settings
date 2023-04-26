// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod desktop;
pub mod networking;
pub mod sound;
pub mod system;
pub mod time;

#[derive(Clone, Debug)]
pub enum Message {
    About(system::about::Message),
    DateAndTime(time::date::Message),
    Desktop(desktop::Message),
    External { id: String, message: Vec<u8> },
}
