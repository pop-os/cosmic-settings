// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic_settings_page::Entity;

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
    Panel(desktop::panel::Message),
    External { id: String, message: Vec<u8> },
    Page(Entity),
}
