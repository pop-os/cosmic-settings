use cosmic_settings_page::{self as page, section, Section};
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(Section::default())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("panel_applets", "preferences-pop-desktop-dock-symbolic")
            .title(fl!("applets"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}
