// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic_settings_page as page;

pub mod date;
pub mod region;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("time", "preferences-time-and-language-symbolic")
            .title(fl!("time"))
            .description(fl!("time", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<date::Page>().sub_page::<region::Page>()
    }
}
