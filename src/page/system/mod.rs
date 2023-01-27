// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod about;
pub mod firmware;
pub mod users;

use crate::page;

pub struct Page;

impl page::Page for Page {
    type Model = Model;

    fn page() -> page::Meta {
        page::Meta::new("system", "system-users-symbolic").title(fl!("system"))
    }

    fn sub_pages(page: page::Insert) -> page::Insert {
        page.sub_page::<users::Page>()
            .sub_page::<about::Page>()
            .sub_page::<firmware::Page>()
    }
}

#[derive(Default)]
pub struct Model {}
