// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic_settings_page::Entity;

#[cfg(feature = "page-accessibility")]
pub mod accessibility;
pub mod applications;
#[cfg(feature = "page-bluetooth")]
pub mod bluetooth;
pub mod desktop;
#[cfg(feature = "page-display")]
pub mod display;
#[cfg(feature = "page-input")]
pub mod input;
#[cfg(feature = "page-networking")]
pub mod networking;
#[cfg(feature = "page-power")]
pub mod power;
#[cfg(feature = "page-sound")]
pub mod sound;
pub mod system;
pub mod time;

pub type Element<'a> = cosmic::Element<'a, Message>;

#[derive(Clone, Debug)]
pub enum Message {
    // Page-specific messages
    #[cfg(feature = "page-accessibility")]
    Accessibility(accessibility::Message),
    #[cfg(feature = "page-accessibility")]
    AccessibilityMagnifier(accessibility::magnifier::Message),
    #[cfg(feature = "page-about")]
    About(system::about::Message),
    Appearance(desktop::appearance::Message),
    Applications(applications::Message),
    #[cfg(feature = "page-bluetooth")]
    Bluetooth(bluetooth::Message),
    #[cfg(feature = "page-input")]
    CustomShortcuts(input::keyboard::shortcuts::custom::Message),
    #[cfg(feature = "page-date")]
    DateAndTime(time::date::Message),
    #[cfg(feature = "page-default-apps")]
    DefaultApps(applications::default_apps::Message),
    Desktop(desktop::Message),
    DesktopWallpaper(desktop::wallpaper::Message),
    #[cfg(feature = "page-workspaces")]
    DesktopWorkspaces(desktop::workspaces::Message),
    #[cfg(feature = "page-display")]
    Displays(display::Message),
    #[cfg(feature = "wayland")]
    Dock(desktop::dock::Message),
    #[cfg(feature = "wayland")]
    DockApplet(desktop::dock::applets::Message),
    External {
        id: String,
        message: Vec<u8>,
    },
    #[cfg(feature = "page-input")]
    Input(input::Message),
    #[cfg(feature = "page-input")]
    Keyboard(input::keyboard::Message),
    #[cfg(feature = "page-input")]
    KeyboardShortcuts(input::keyboard::shortcuts::Message),
    #[cfg(feature = "page-legacy-applications")]
    LegacyApplications(applications::legacy_applications::Message),
    #[cfg(feature = "page-input")]
    ManageWindowShortcuts(input::keyboard::shortcuts::ShortcutMessage),
    #[cfg(feature = "page-input")]
    MoveWindowShortcuts(input::keyboard::shortcuts::ShortcutMessage),
    #[cfg(feature = "page-input")]
    NavShortcuts(input::keyboard::shortcuts::ShortcutMessage),
    #[cfg(feature = "page-networking")]
    Networking(networking::Message),
    Page(Entity),
    #[cfg(feature = "wayland")]
    Panel(desktop::panel::Message),
    #[cfg(feature = "wayland")]
    PanelApplet(desktop::panel::applets_inner::Message),
    #[cfg(feature = "page-power")]
    Power(power::Message),
    #[cfg(feature = "page-region")]
    Region(time::region::Message),
    #[cfg(feature = "page-sound")]
    Sound(sound::Message),
    StartupApps(applications::startup_apps::Message),
    #[cfg(feature = "page-users")]
    User(system::users::Message),
    #[cfg(feature = "page-input")]
    SystemShortcuts(input::keyboard::shortcuts::ShortcutMessage),
    #[cfg(feature = "page-input")]
    TilingShortcuts(input::keyboard::shortcuts::ShortcutMessage),
    #[cfg(feature = "page-networking")]
    Vpn(networking::vpn::Message),
    #[cfg(feature = "page-networking")]
    WiFi(networking::wifi::Message),
    #[cfg(feature = "page-window-management")]
    WindowManagement(desktop::window_management::Message),
    #[cfg(feature = "page-networking")]
    Wired(networking::wired::Message),

    // Common page functionality
    CloseContextDrawer,
}

impl From<Message> for crate::Message {
    fn from(message: Message) -> Self {
        crate::Message::PageMessage(message)
    }
}
