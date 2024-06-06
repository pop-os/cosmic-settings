use cosmic::widget::{column, settings};
use cosmic::{Apply, Element};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slab::Slab;
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page;

//crate::app::Message::Page

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(shortcuts())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("keyboard-shortcuts", "input-keyboard-symbolic")
            .title(fl!("keyboard-shortcuts"))
            .description(fl!("keyboard-shortcuts", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn shortcuts() -> Section<crate::pages::Message> {
    let descriptions = Slab::new();

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            // TODO need something more custom
            /*
            settings::view_section(&section.title)
                .apply(Element::from)
                .map(crate::pages::Message::Input)
            */
            column()
                .push(settings::view_section(&section.title))
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}
