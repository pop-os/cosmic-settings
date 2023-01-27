// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::page;

pub mod date;
pub mod region;

pub struct Page;

impl page::Page for Page {
    type Model = ();

    fn page() -> page::Meta {
        page::Meta::new("time", "preferences-system-time-symbolic")
            .title(fl!("time"))
            .description(fl!("time", "desc"))
    }

    fn sub_pages(page: page::Insert) -> page::Insert {
        page.sub_page::<date::Page>().sub_page::<region::Page>()
    }
}
