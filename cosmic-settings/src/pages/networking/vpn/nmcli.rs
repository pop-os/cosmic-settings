// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use as_result::IntoResult;
use std::io;

pub async fn set_username(connection_name: &str, username: &str) -> io::Result<()> {
    tokio::process::Command::new("nmcli")
        .args(&["con", "mod", connection_name, "vpn.user-name", username])
        .status()
        .await
        .and_then(IntoResult::into_result)
}

pub async fn set_password_flags_none(connection_name: &str) -> io::Result<()> {
    tokio::process::Command::new("nmcli")
        .args(&[
            "con",
            "mod",
            connection_name,
            "+vpn.data",
            "password-flags=0",
        ])
        .status()
        .await
        .and_then(IntoResult::into_result)
}

pub async fn set_password(connection_name: &str, password: &str) -> io::Result<()> {
    tokio::process::Command::new("nmcli")
        .args(&[
            "con",
            "mod",
            &connection_name,
            "vpn.secrets",
            &format!("password={password}"),
        ])
        .status()
        .await
        .and_then(IntoResult::into_result)
}

pub async fn connect(connection_name: &str) -> io::Result<()> {
    tokio::process::Command::new("nmcli")
        .args(&["con", "up", &connection_name])
        .status()
        .await
        .and_then(IntoResult::into_result)
}
