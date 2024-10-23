// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod appearance;
#[cfg(feature = "wayland")]
pub mod dock;
#[cfg(feature = "wayland")]
pub mod panel;
pub mod wallpaper;
pub mod window_management;
pub mod workspaces;

use cosmic_settings_page as page;

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct Page {}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("desktop", "video-display-symbolic").title(fl!("desktop"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        let mut page = page
            .sub_page::<wallpaper::Page>()
            .sub_page::<appearance::Page>();

        #[cfg(feature = "wayland")]
        {
            page = page.sub_page::<panel::Page>();
            page = page.sub_page::<dock::Page>();
        }

        page.sub_page::<window_management::Page>()
            .sub_page::<workspaces::Page>()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Message {}

impl Page {
    pub fn update(&mut self, _message: Message) {}
}
