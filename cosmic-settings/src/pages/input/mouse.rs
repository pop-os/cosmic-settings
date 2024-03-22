use cosmic::iced::Alignment;
use cosmic::widget::{self, row, settings, text};
use cosmic::{Apply, Element};
use cosmic_comp_config::input::AccelProfile;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

use super::Message;

crate::cache_dynamic_lazy! {
    static MOUSE_ACCELERATION: String = fl!("mouse", "acceleration");
    static MOUSE_SPEED: String = fl!("mouse", "speed");
}

pub fn default_primary_button() -> cosmic::widget::segmented_button::SingleSelectModel {
    let mut model = cosmic::widget::segmented_button::SingleSelectModel::builder()
        .insert(|b| b.text(fl!("primary-button", "left")))
        .insert(|b| b.text(fl!("primary-button", "right")))
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
            super::PRIMARY_BUTTON.as_str().into(),
            MOUSE_SPEED.as_str().into(),
            MOUSE_ACCELERATION.as_str().into(),
            super::ACCELERATION_DESC.as_str().into(),
        ])
        .view::<Page>(|binder, _page, section| {
            let input = binder.page::<super::Page>().expect("input page not found");
            let theme = cosmic::theme::active();

            settings::view_section(&section.title)
                .add(settings::item(
                    &*super::PRIMARY_BUTTON,
                    cosmic::widget::segmented_control::horizontal(&input.primary_button)
                        .minimum_button_width(0)
                        .on_activate(|x| Message::PrimaryButtonSelected(x, false)),
                ))
                .add(settings::item::builder(&*MOUSE_SPEED).control({
                    let value = (input
                        .input_default
                        .acceleration
                        .as_ref()
                        .map_or(0.0, |x| x.speed)
                        + 1.0)
                        * 50.0;

                    let slider = widget::slider(10.0..=80.0, value, |value| {
                        Message::SetMouseSpeed((value / 50.0) - 1.0, false)
                    })
                    .width(250.0)
                    .breakpoints(&[45.0]);

                    row::with_capacity(2)
                        .align_items(Alignment::Center)
                        .spacing(theme.cosmic().space_s())
                        .push(text(format!("{:.0}", value.round())))
                        .push(slider)
                }))
                .add(
                    settings::item::builder(&*MOUSE_ACCELERATION)
                        .description(&*super::ACCELERATION_DESC)
                        .toggler(
                            input
                                .input_default
                                .acceleration
                                .as_ref()
                                .map_or(true, |x| x.profile == Some(AccelProfile::Adaptive)),
                            |x| Message::SetAcceleration(x, false),
                        ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn scrolling() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("scrolling"))
        .descriptions(vec![
            super::SCROLLING_SPEED.as_str().into(),
            super::SCROLLING_NATURAL.as_str().into(),
            super::SCROLLING_NATURAL_DESC.as_str().into(),
        ])
        .view::<Page>(|binder, _page, section| {
            let input = binder.page::<super::Page>().expect("input page not found");
            let theme = cosmic::theme::active();

            settings::view_section(&section.title)
                .add(settings::item(&*super::SCROLLING_SPEED, {
                    let value = input
                        .input_default
                        .scroll_config
                        .as_ref()
                        .and_then(|x| x.scroll_factor)
                        .unwrap_or(1.)
                        .log(2.)
                        * 10.0
                        + 50.0;

                    let slider = widget::slider(1.0..=100.0, value, |value| {
                        Message::SetScrollFactor(2f64.powf((value - 50.0) / 10.0), false)
                    })
                    .width(250.0)
                    .breakpoints(&[50.0]);

                    row::with_capacity(2)
                        .align_items(Alignment::Center)
                        .spacing(theme.cosmic().space_s())
                        .push(text(format!("{:.0}", value.round())))
                        .push(slider)
                }))
                .add(
                    settings::item::builder(&*super::SCROLLING_NATURAL)
                        .description(&*super::SCROLLING_NATURAL_DESC)
                        .toggler(
                            input
                                .input_default
                                .scroll_config
                                .as_ref()
                                .and_then(|x| x.natural_scroll)
                                .unwrap_or(false),
                            |x| Message::SetNaturalScroll(x, false),
                        ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}
