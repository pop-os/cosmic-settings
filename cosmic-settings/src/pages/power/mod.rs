use cosmic_settings_page as page;

pub mod profiles;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("power", "battery-symbolic")
            .title(fl!("power"))
            .description(fl!("power", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<profiles::Page>()
    }
}