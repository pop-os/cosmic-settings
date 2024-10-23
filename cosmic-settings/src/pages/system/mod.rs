// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(feature = "page-about")]
pub mod about;
pub mod firmware;
pub mod users;

use cosmic_settings_page as page;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("system", "system-users-symbolic").title(fl!("system"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: page::Insert<crate::pages::Message>,
    ) -> page::Insert<crate::pages::Message> {
        page = page.sub_page::<users::Page>();
        #[cfg(feature = "page-about")]
        {
            page = page.sub_page::<about::Page>();
        }
        page = page.sub_page::<firmware::Page>();
        page
    }
}
