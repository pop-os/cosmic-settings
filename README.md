# cosmic-settings

This is the settings app for Pop!_OS's new COSMIC desktop. It is currently WIP and not ready for use, although if you want to run it and play around with the GUI, feel free!

## Building

COSMIC Settings is built using Rust. It targets the latest stable version of Rust, and requires that GTK 4 be installed.

### Install Pre-Requisites

```sh
# Install build tools and GTK-4
$ sudo apt install -y build-essential libgtk-4-dev
# Install Rust toolchain manager and Rust
# Note: you should always use rustup, as your package manager likely packages an older version!
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Make the Rust toolchain available in the current environment
$ source $HOME/.cargo/env
```

### Build

For a quick build, use `cargo build` (or `cargo run` to go straight to running it).

For an optimized release build, use `cargo build --release` (or `cargo run --release` to go straight to running it).

## App Styling

Currently, the settings app is styled using SCSS files, as seen in the [scss](scss) directory.

All .scss files in the directory (excluding those that start with `_`) are combined and compiled using the [grass](https://github.com/connorskees/grass) crate at build-time, and the resulting CSS is used to style the app.

## License

COSMIC Settings is free software: you can redistribute it and/or modify it under the terms of version 3 of the GNU General Public License as published by the Free Software Foundation.

COSMIC Settings is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU General Public License](LICENSE.md) for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
