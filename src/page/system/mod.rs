// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod about;
pub mod firmware;
pub mod users;

use crate::page;

pub struct Page;

impl page::Page for Page {
    type Model = Model;

    const PERSISTENT_ID: &'static str = "system";

    fn page() -> page::Meta {
        page::Meta::default()
            .title(fl!("system"))
            .icon_name("system-users-symbolic")
    }

    fn sub_pages(page: page::Insert) -> page::Insert {
        page.sub_page::<users::Page>()
            .sub_page::<about::Page>()
            .sub_page::<firmware::Page>()
    }
}

#[derive(Default)]
pub struct Model {}
