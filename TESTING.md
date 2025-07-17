# Testing

This document provides a regression testing checklist for COSMIC Settings and the COSMIC Settings Daemon. The checklist provides a starting point for Quality Assurance reviews.

## Checklist

### Network & Wireless

#### Wi-Fi

- [ ] Able to disconnect from and connect to a network.

### Desktop

#### Wallpaper

- [ ] Clicking a different wallpaper switches to it immediately
- [ ] Leaving the page and coming back does not change the wallpaper

#### Appearance

- [ ] Open a COSMIC app and a GNOME app. When switching between Dark and Light mode, the COSMIC app switches immediately, and the GNOME app switches after restarting.
- [ ] Changing the accent color works.
- [ ] Changing the roundness style works and does not affect other aspects of the theme (e.g. color mode).
- [ ] Changing the interface density works.

#### Panel

- [ ] Enable tiling. "Automatically hide panel" takes effect immediately.
- [ ] Disable tiling; maximize a window. "Automatically hide panel" takes effect immediately.

#### Dock

- [ ] Enable tiling. "Automatically hide dock" takes effect immediately.
- [ ] Disable tiling; maximize a window. "Autommatically hide dock" takes effect immediately.
- [ ] Changing position on screen works.
- [ ] All Style settings take effect as expected.

### Displays

- [ ] Plug in a second display. Displays can be rearranged via dragging.

### Sound

TBD after devices/profiles are fixed.

### Power & Battery

- [ ] Changing power mode takes effect and is reflected by top panel applet.

### Input Devices

#### Touchpad

- [ ] All click behavior settings work as expected.
- [ ] All scrolling settings work as expected.

### Applications

#### Default Applications

- [ ] Changing the default terminal affects Super-T shortcut.

#### X11 Applications Compatibility

- [ ] Global Shortcuts in X11 Applications works (in e.g. Discord).

### Time & Language

#### Date & Time

- [ ] Changing time zome works.
- [ ] Toggling 24-hour time takes effect in Settings preview, on top panel, and on the lock screen.
- [ ] Toggling seconds takes effect in Settings preview and on top panel.
- [ ] Changing first day of week takes effect in Calendar applet.
- [ ] Turning date on/off for applet works.

### System & Accounts

#### About

- [ ] Changing hostname works (check terminal prompt).
