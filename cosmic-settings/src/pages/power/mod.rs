use cosmic_settings_page as page;

pub mod profiles;

#[derive(Default)]
pub struct Page;

// TO-DO implement localization after find out how it works 
impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("power", "preferences-system-time-symbolic")
            .title("Power & Sleep")
            .description("Settings for power and sleep")
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<profiles::Page>()
    }
}