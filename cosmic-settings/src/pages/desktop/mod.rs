// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod appearance;
pub mod dock;
pub mod panel;
pub mod wallpaper;
pub mod window_management;
pub mod workspaces;

use cosmic_settings_page as page;

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct Page {}

impl Default for Page {
    fn default() -> Self {
        Self {}
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("desktop", "video-display-symbolic").title(fl!("desktop"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<wallpaper::Page>()
            .sub_page::<appearance::Page>()
            .sub_page::<panel::Page>()
            .sub_page::<dock::Page>()
            .sub_page::<window_management::Page>()
            .sub_page::<workspaces::Page>()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Message {
    // ShowMaximizeButton(bool),
    // ShowMinimizeButton(bool),
}

impl Page {
    pub fn update(&mut self, _message: Message) {}
}
