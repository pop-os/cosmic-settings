// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::page;

pub fn page() -> page::Meta {
    page::Meta::default()
        .title(fl!("wired"))
        .description(fl!("wired", "desc"))
        .icon_name("network-workgroup-symbolic")
}
