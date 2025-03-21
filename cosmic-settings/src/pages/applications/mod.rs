// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(feature = "page-default-apps")]
pub mod default_apps;

pub mod startup_apps;

pub mod legacy_applications;

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
        page::Info::new("applications", "preferences-applications-symbolic")
            .title(fl!("applications"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: page::Insert<crate::pages::Message>,
    ) -> page::Insert<crate::pages::Message> {
        #[cfg(feature = "page-default-apps")]
        {
            page = page.sub_page::<default_apps::Page>();
        }

        page = page.sub_page::<startup_apps::Page>();

        page = page.sub_page::<legacy_applications::Page>();

        page
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Message {}

impl Page {
    pub fn update(&mut self, _message: Message) {}
}
