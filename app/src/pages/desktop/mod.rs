// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod appearance;
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
    pub top_left_hot_corner: bool,
    pub show_workspaces_button: bool,
    pub show_applications_button: bool,
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
        page.sub_page::<options::Page>()
            .sub_page::<wallpaper::Page>()
            .sub_page::<appearance::Page>()
            .sub_page::<workspaces::Page>()
            .sub_page::<notifications::Page>()
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    ShowWorkspacesButton(bool),
    ShowApplicationsButton(bool),
    ShowMinimizeButton(bool),
    ShowMaximizeButton(bool),
    TopLeftHotCorner(bool),
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ShowApplicationsButton(value) => self.show_applications_button = value,
            Message::ShowMaximizeButton(value) => self.show_maximize_button = value,
            Message::ShowMinimizeButton(value) => self.show_minimize_button = value,
            Message::ShowWorkspacesButton(value) => self.show_workspaces_button = value,
            Message::TopLeftHotCorner(value) => self.top_left_hot_corner = value,
        }
    }
}

// impl From<page::Info> for Message {
//     fn from(page: page::Info) -> Message {
//         Message::page::Info(page)
//     }
// }

// pub enum Output {
//     page::Info(page::Info),
// }

// impl Subpage::Info for Desktoppage::Info {
//     //TODO: translate
//     fn title(&self) -> &'static str {
//         use Desktoppage::Info::*;
//         match self {
//             Workspaces => "Workspaces",
//             Notifications => "Notifications",
//         }
//     }

//     //TODO: translate
//     fn description(&self) -> &'static str {
//         use Desktoppage::Info::*;
//         match self {
//             Workspaces => "Set workspace number, behavior, and placement.",
//             Notifications => {
//                 "Do Not Disturb, lockscreen notifications, and per-application settings."
//             }
//         }
//     }

//     fn icon_name(&self) -> &'static str {
//         use Desktoppage::Info::*;
//         match self {
//             Workspaces => "preferences-pop-desktop-workspaces-symbolic",
//             Notifications => "preferences-system-notifications-symbolic",
//         }
//     }

//     fn parent_page(&self) -> page::Info {
//         page::Info::Desktop(None)
//     }

//     fn into_page(self) -> page::Info {
//         page::Info::Desktop(Some(self))
//     }
// }
