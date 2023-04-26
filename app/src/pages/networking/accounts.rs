// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic_settings_page as page;

pub fn info() -> page::Info {
    page::Info::new("online-accounts", "goa-panel-symbolic")
        .title(fl!("online-accounts"))
        .description(fl!("online-accounts", "desc"))
}
