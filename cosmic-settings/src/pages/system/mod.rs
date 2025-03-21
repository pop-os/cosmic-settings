// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(feature = "page-about")]
pub mod about;
#[cfg(feature = "page-default-apps")]
pub mod default_apps;
pub mod firmware;
#[cfg(feature = "page-users")]
pub mod users;

pub mod startup_apps;

use cosmic_settings_page as page;

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> page::Info {
        page::Info::new("system", "system-users-symbolic").title(fl!("system"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: page::Insert<crate::pages::Message>,
    ) -> page::Insert<crate::pages::Message> {
        #[cfg(feature = "page-users")]
        {
            page = page.sub_page::<users::Page>();
        }

        #[cfg(feature = "page-about")]
        {
            page = page.sub_page::<about::Page>();
        }

        page = page.sub_page::<firmware::Page>();

        #[cfg(feature = "page-default-apps")]
        {
            page = page.sub_page::<default_apps::Page>();
        }

        page = page.sub_page::<startup_apps::Page>();

        page
    }
}
