// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use numtoa::BaseN;
use std::process::Stdio;

pub async fn set_default(id: u32) {
    let id = numtoa::BaseN::<10>::u32(id);
    _ = tokio::process::Command::new("wpctl")
        .args(["set-default", id.as_str()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;
}

pub async fn set_profile(id: u32, index: u32) {
    let id = BaseN::<10>::u32(id);
    let index = BaseN::<10>::u32(index);
    let value = ["{ index: ", index.as_str(), ", save: true }"].concat();
    _ = tokio::process::Command::new("pw-cli")
        .args(["s", id.as_str(), "Profile", &value])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;
}

pub async fn set_mute(id: u32, mute: bool) {
    let default = numtoa::BaseN::<10>::u32(id);
    _ = tokio::process::Command::new("wpctl")
        .args(["set-mute", default.as_str(), if mute { "1" } else { "0" }])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;
}

pub async fn set_volume(id: u32, volume: u32) {
    let id = numtoa::BaseN::<10>::u32(id);
    let volume = format!("{}.{:02}", volume / 100, volume % 100);
    _ = tokio::process::Command::new("wpctl")
        .args(["set-volume", id.as_str(), volume.as_str()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;
}
