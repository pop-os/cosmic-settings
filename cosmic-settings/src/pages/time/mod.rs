// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic_settings_page as page;

#[cfg(feature = "page-date")]
pub mod date;
pub mod region;

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> page::Info {
        page::Info::new("time", "preferences-time-and-language-symbolic")
            .title(fl!("time"))
            .description(fl!("time", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: page::Insert<crate::pages::Message>,
    ) -> page::Insert<crate::pages::Message> {
        #[cfg(feature = "page-date")]
        {
            page = page.sub_page::<date::Page>();
        }

        #[cfg(feature = "page-region")]
        {
            page = page.sub_page::<region::Page>();
        }

        page
    }
}
