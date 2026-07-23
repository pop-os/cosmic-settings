// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod appearance;
#[cfg(feature = "wayland")]
pub mod dock;
pub mod notifications;
#[cfg(feature = "wayland")]
pub mod panel;
pub mod wallpaper;
#[cfg(feature = "page-window-management")]
pub mod window_management;
#[cfg(feature = "page-workspaces")]
pub mod workspaces;

use cosmic_settings_page as page;

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct Page {
    entity: page::Entity,
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> page::Info {
        page::Info::new("desktop", "video-display-symbolic").title(fl!("desktop"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: page::Insert<crate::pages::Message>,
    ) -> page::Insert<crate::pages::Message> {
        page = page.sub_page::<wallpaper::Page>();
        page = page.sub_page::<appearance::Page>();
        page = page.sub_page::<notifications::Page>();

        #[cfg(feature = "wayland")]
        {
            page = page.sub_page::<panel::Page>();
            page = page.sub_page::<dock::Page>();
        }

        #[cfg(feature = "page-window-management")]
        {
            page = page.sub_page::<window_management::Page>();
        }

        #[cfg(feature = "page-workspaces")]
        {
            page = page.sub_page::<workspaces::Page>();
        }

        page
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Message {}

impl Page {
    pub fn update(&mut self, _message: Message) {}
}
