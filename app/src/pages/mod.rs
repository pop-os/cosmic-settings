// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic_settings_page::Entity;

pub mod desktop;
pub mod input;
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
    Dock(desktop::dock::Message),
    DesktopWallpaper(desktop::wallpaper::Message),
    PanelApplet(desktop::panel::applets_inner::Message),
    DockApplet(desktop::dock::applets::Message),
    Appearance(desktop::appearance::Message),
    Input(input::Message),
    External { id: String, message: Vec<u8> },
    Page(Entity),
}
