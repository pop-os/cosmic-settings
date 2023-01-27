// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod appearance;
pub mod dock;
pub mod notifications;
pub mod options;
pub mod wallpaper;
pub mod workspaces;

use crate::page;

pub struct Page;

impl page::Page for Page {
    type Model = super::Model;

    fn page() -> page::Meta {
        page::Meta::new("desktop", "video-display-symbolic").title(fl!("desktop"))
    }

    fn sub_pages(page: page::Insert) -> page::Insert {
        page.sub_page::<options::Page>()
            .sub_page::<wallpaper::Page>()
            .sub_page::<appearance::Page>()
            .sub_page::<dock::Page>()
            .sub_page::<workspaces::Page>()
            .sub_page::<notifications::Page>()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Slideshow(bool),
    SameBackground(bool),
    ShowWorkspacesButton(bool),
    ShowApplicationsButton(bool),
    ShowMinimizeButton(bool),
    ShowMaximizeButton(bool),
    TopLeftHotCorner(bool),
}

#[derive(Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Model {
    pub top_left_hot_corner: bool,
    pub show_workspaces_button: bool,
    pub show_applications_button: bool,
    pub show_minimize_button: bool,
    pub show_maximize_button: bool,
    pub slideshow: bool,
    pub same_background: bool,
}

impl Model {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SameBackground(value) => self.same_background = value,
            Message::ShowApplicationsButton(value) => self.show_applications_button = value,
            Message::ShowMaximizeButton(value) => self.show_maximize_button = value,
            Message::ShowMinimizeButton(value) => self.show_minimize_button = value,
            Message::ShowWorkspacesButton(value) => self.show_workspaces_button = value,
            Message::Slideshow(value) => self.slideshow = value,
            Message::TopLeftHotCorner(value) => self.top_left_hot_corner = value,
        }
    }
}

// impl From<Page> for Message {
//     fn from(page: Page) -> Message {
//         Message::Page(page)
//     }
// }

// pub enum Output {
//     Page(Page),
// }

// impl SubPage for DesktopPage {
//     //TODO: translate
//     fn title(&self) -> &'static str {
//         use DesktopPage::*;
//         match self {
//             Workspaces => "Workspaces",
//             Notifications => "Notifications",
//         }
//     }

//     //TODO: translate
//     fn description(&self) -> &'static str {
//         use DesktopPage::*;
//         match self {
//             Workspaces => "Set workspace number, behavior, and placement.",
//             Notifications => {
//                 "Do Not Disturb, lockscreen notifications, and per-application settings."
//             }
//         }
//     }

//     fn icon_name(&self) -> &'static str {
//         use DesktopPage::*;
//         match self {
//             Workspaces => "preferences-pop-desktop-workspaces-symbolic",
//             Notifications => "preferences-system-notifications-symbolic",
//         }
//     }

//     fn parent_page(&self) -> Page {
//         Page::Desktop(None)
//     }

//     fn into_page(self) -> Page {
//         Page::Desktop(Some(self))
//     }
// }
