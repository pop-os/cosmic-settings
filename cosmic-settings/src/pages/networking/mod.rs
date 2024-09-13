// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod vpn;
pub mod wifi;
pub mod wired;

use std::{ffi::OsStr, io, process::ExitStatus};

use cosmic_settings_page as page;

static NM_CONNECTION_EDITOR: &str = "nm-connection-editor";

#[derive(Debug, Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> cosmic_settings_page::Info {
        page::Info::new(
            "network-and-wireless",
            "preferences-network-and-wireless-symbolic",
        )
        .title(fl!("network-and-wireless"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        page: cosmic_settings_page::Insert<crate::pages::Message>,
    ) -> cosmic_settings_page::Insert<crate::pages::Message> {
        page.sub_page::<wired::Page>()
            .sub_page::<wifi::Page>()
            .sub_page::<vpn::Page>()
    }
}

async fn nm_add_vpn_file<P: AsRef<OsStr>>(type_: &str, path: P) -> io::Result<ExitStatus> {
    tokio::process::Command::new("nmcli")
        .args(["connection", "import", "type", type_, "file"])
        .arg(path)
        .status()
        .await
}

async fn nm_add_wired() -> io::Result<ExitStatus> {
    nm_connection_editor(&["--type=802-3-ethernet", "-c"]).await
}

async fn nm_add_wifi() -> io::Result<ExitStatus> {
    nm_connection_editor(&["--type=802-11-wireless", "-c"]).await
}

async fn nm_edit_connection(uuid: &str) -> io::Result<ExitStatus> {
    nm_connection_editor(&[&["--edit=", uuid].concat()]).await
}

async fn nm_connection_editor(args: &[&str]) -> io::Result<ExitStatus> {
    tokio::process::Command::new(NM_CONNECTION_EDITOR)
        .args(args)
        .status()
        .await
}
