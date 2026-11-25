// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(feature = "page-default-apps")]
pub mod default_apps;

pub mod startup_apps;

#[cfg(feature = "page-legacy-applications")]
pub mod legacy_applications;

use cosmic_settings_page as page;
use freedesktop_desktop_entry::DesktopEntry;
use std::path::Path;

fn desktop_entry_is_visible(entry: &DesktopEntry) -> bool {
    !entry.no_display() && !entry.hidden()
}

fn desktop_file_is_visible(path: &Path) -> bool {
    match DesktopEntry::from_path(path, None::<&[String]>) {
        Ok(entry) => desktop_entry_is_visible(&entry),
        Err(error) => {
            tracing::warn!(
                ?path,
                ?error,
                "Unable to parse desktop entry; keeping it visible"
            );
            true
        }
    }
}

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

        #[cfg(feature = "page-legacy-applications")]
        {
            page = page.sub_page::<legacy_applications::Page>();
        }

        page
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Message {}

impl Page {
    pub fn update(&mut self, _message: Message) {}
}
