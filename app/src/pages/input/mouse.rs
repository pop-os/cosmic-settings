use apply::Apply;
use cosmic::iced::widget;
use cosmic::widget::settings;
use cosmic::Element;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

use super::Message;

// XXX
pub fn default_primary_button() -> cosmic::widget::segmented_button::SingleSelectModel {
    let mut model = cosmic::widget::segmented_button::SingleSelectModel::builder()
        .insert(|b| b.text(fl!("mouse", "primary-button-left")))
        .insert(|b| b.text(fl!("mouse", "primary-button-right")))
        .build();
    model.activate_position(0);
    model
}

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(mouse()), sections.insert(scrolling())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("mouse", "input-mouse-symbolic")
            .title(fl!("mouse"))
            .description(fl!("mouse", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn mouse() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![
            fl!("mouse", "primary-button"),
            fl!("mouse", "speed"),
            fl!("mouse", "acceleration"),
            fl!("mouse", "acceleration-desc"),
            fl!("mouse", "double-click-speed"),
            fl!("mouse", "double-click-speed-desc"),
        ])
        .view::<Page>(|binder, _page, section| {
            let descriptions = &section.descriptions;

            let input = binder.page::<super::Page>().expect("input page not found");

            // TODO need something more custom
            settings::view_section(&section.title)
                // TODO
                .add(settings::item(
                    &descriptions[0],
                    cosmic::widget::segmented_selection::horizontal(&input.primary_button)
                        .on_activate(Message::PrimaryButtonSelected),
                ))
                .add(
                    settings::item::builder(&descriptions[1]).control(widget::slider(
                        0..=100,
                        input.mouse_speed,
                        Message::SetMouseSpeed,
                    )),
                )
                .add(
                    settings::item::builder(&descriptions[2])
                        .description(&descriptions[3])
                        .toggler(input.acceleration, Message::SetAcceleration),
                )
                .add(
                    settings::item::builder(&descriptions[4])
                        .description(&descriptions[5])
                        .control(widget::slider(
                            0..=100,
                            input.double_click_speed,
                            Message::SetDoubleClickSpeed,
                        )),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn scrolling() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("mouse-scrolling"))
        .descriptions(vec![
            fl!("mouse-scrolling", "speed"),
            fl!("mouse-scrolling", "natural"),
            fl!("mouse-scrolling", "natural-desc"),
        ])
        .view::<Page>(|binder, _page, section| {
            let descriptions = &section.descriptions;

            let input = binder.page::<super::Page>().expect("input page not found");

            settings::view_section(&section.title)
                .add(settings::item(
                    &descriptions[0],
                    // TODO show numeric value
                    widget::slider(0..=100, input.scroll_speed, Message::SetScrollSpeed),
                ))
                .add(
                    settings::item::builder(&descriptions[1])
                        .description(&descriptions[2])
                        .toggler(input.natural_scroll, Message::SetNaturalScroll),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}
