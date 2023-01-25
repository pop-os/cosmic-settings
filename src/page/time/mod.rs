// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::page;

pub mod date;
pub mod region;

pub struct Page;

impl page::Page for Page {
    type Model = ();

    const PERSISTENT_ID: &'static str = "time";

    fn page() -> page::Meta {
        page::Meta::default()
            .title(fl!("time"))
            .description(fl!("time", "desc"))
            .icon_name("preferences-system-time-symbolic")
    }

    fn sub_pages(page: page::Insert) -> page::Insert {
        page.sub_page::<date::Page>().sub_page::<region::Page>()
    }
}
