// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::Apply;
use std::process::Stdio;

pub async fn set_username(connection_name: &str, username: &str) -> Result<(), String> {
    tokio::process::Command::new("nmcli")
        .args(&["con", "mod", connection_name, "vpn.user-name", username])
        .stderr(Stdio::piped())
        .output()
        .await
        .apply(crate::utils::map_stderr_output)
}

pub async fn set_password_flags_none(connection_name: &str) -> Result<(), String> {
    tokio::process::Command::new("nmcli")
        .args([
            "con",
            "mod",
            connection_name,
            "+vpn.data",
            "password-flags=0",
        ])
        .stderr(Stdio::piped())
        .output()
        .await
        .apply(crate::utils::map_stderr_output)
}

pub async fn add_fallback(connection_name: &str) -> Result<(), String> {
    tokio::process::Command::new("nmcli")
        .args([
            "con",
            "mod",
            connection_name,
            "+vpn.data",
            "data-ciphers=AES-256-GCM:AES-128-GCM:CHACHA20-POLY1305:AES-256-CBC:AES-128-CBC",
        ])
        .stderr(Stdio::piped())
        .output()
        .await
        .apply(crate::utils::map_stderr_output)
}

pub async fn set_password(connection_name: &str, password: &str) -> Result<(), String> {
    tokio::process::Command::new("nmcli")
        .args([
            "con",
            "mod",
            connection_name,
            "vpn.secrets",
            &format!("password={password}"),
        ])
        .stderr(Stdio::piped())
        .output()
        .await
        .apply(crate::utils::map_stderr_output)
}

pub async fn connect(connection_name: &str) -> Result<(), String> {
    tokio::process::Command::new("nmcli")
        .args(&["con", "up", &connection_name])
        .stderr(Stdio::piped())
        .output()
        .await
        .apply(crate::utils::map_stderr_output)
}
