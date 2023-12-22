// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod appearance;
pub mod display;
pub mod dock;
pub mod notifications;
pub mod options;
pub mod panel;
pub mod wallpaper;
pub mod workspaces;

use cosmic_settings_page as page;

#[derive(Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Page {
    pub show_minimize_button: bool,
    pub show_maximize_button: bool,
}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("desktop", "video-display-symbolic").title(fl!("desktop"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<display::Page>()
            .sub_page::<options::Page>()
            .sub_page::<wallpaper::Page>()
            .sub_page::<appearance::Page>()
            .sub_page::<workspaces::Page>()
            .sub_page::<notifications::Page>()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Message {
    ShowMinimizeButton(bool),
    ShowMaximizeButton(bool),
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ShowMaximizeButton(value) => self.show_maximize_button = value,
            Message::ShowMinimizeButton(value) => self.show_minimize_button = value,
        }
    }
}
