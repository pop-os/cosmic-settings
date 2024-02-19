use apply::Apply;
use cosmic::widget::{self, settings};
use cosmic::Element;
use cosmic_comp_config::input::AccelProfile;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;
use std::borrow::Cow;

use super::Message;

crate::cache_dynamic_lazy! {
    static MOUSE_SCROLL_SPEED: String = fl!("mouse-scrolling", "speed");
    static MOUSE_SCROLL_NATURAL: String = fl!("mouse-scrolling", "natural");
    static MOUSE_SCROLL_NATURAL_DESC: String = fl!("mouse-scrolling", "natural-desc");
    static TOUCHPAD_PRIMARY_BUTTON: String = fl!("touchpad", "primary-button");
    static TOUCHPAD_SPEED: String = fl!("touchpad", "speed");
    static TOUCHPAD_ACCELERAION: String = fl!("touchpad", "acceleration");
    static TOUCHPAD_ACCELERAION_DESC: String = fl!("touchpad", "acceleration-desc");
    static TOUCHPAD_DOUBLE_CLICK_SPEED: String = fl!("touchpad", "double-click-speed");
    static TOUCHPAD_DOUBLE_CLICK_SPEED_DESC: String = fl!("touchpad", "double-click-speed-desc");
}

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(touchpad()),
            sections.insert(scrolling()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("touchpad", "input-touchpad-symbolic")
            .title(fl!("touchpad"))
            .description(fl!("touchpad", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn touchpad() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![
            TOUCHPAD_PRIMARY_BUTTON.as_str().into(),
            TOUCHPAD_SPEED.as_str().into(),
            TOUCHPAD_ACCELERAION.as_str().into(),
            TOUCHPAD_ACCELERAION_DESC.as_str().into(),
            TOUCHPAD_DOUBLE_CLICK_SPEED.as_str().into(),
            TOUCHPAD_DOUBLE_CLICK_SPEED_DESC.as_str().into(),
        ])
        .view::<Page>(|binder, _page, section| {
            let input = binder.page::<super::Page>().expect("input page not found");

            settings::view_section(&section.title)
                .add(settings::item(
                    &*TOUCHPAD_PRIMARY_BUTTON,
                    cosmic::widget::segmented_selection::horizontal(&input.touchpad_primary_button)
                        .on_activate(|x| Message::PrimaryButtonSelected(x, true)),
                ))
                .add(
                    settings::item::builder(&*TOUCHPAD_SPEED).control(widget::slider(
                        0.0..=100.0,
                        (input
                            .input_touchpad
                            .acceleration
                            .as_ref()
                            .map_or(0.0, |x| x.speed)
                            + 1.0)
                            * 50.0,
                        |value| Message::SetMouseSpeed((value / 50.0) - 1.0, true),
                    )),
                )
                .add(
                    settings::item::builder(&*TOUCHPAD_ACCELERAION)
                        .description(&*TOUCHPAD_ACCELERAION_DESC)
                        .toggler(
                            input
                                .input_touchpad
                                .acceleration
                                .as_ref()
                                .map_or(true, |x| x.profile == Some(AccelProfile::Adaptive)),
                            |x| Message::SetAcceleration(x, true),
                        ),
                )
                // TODO disable while typing
                .add(
                    settings::item::builder(&*TOUCHPAD_DOUBLE_CLICK_SPEED)
                        .description(&*TOUCHPAD_DOUBLE_CLICK_SPEED_DESC)
                        .control(widget::slider(0..=100, 0, |x| {
                            Message::SetDoubleClickSpeed(x, true)
                        })),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn scrolling() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("mouse-scrolling"))
        .descriptions(vec![
            MOUSE_SCROLL_SPEED.as_str().into(),
            MOUSE_SCROLL_NATURAL.as_str().into(),
            MOUSE_SCROLL_NATURAL_DESC.as_str().into(),
        ])
        .view::<Page>(|binder, _page, section| {
            let input = binder.page::<super::Page>().expect("input page not found");

            settings::view_section(&section.title)
                .add(settings::item(
                    &*MOUSE_SCROLL_SPEED,
                    // TODO show numeric value
                    // TODO desired range?
                    widget::slider(
                        1.0..=100.0,
                        input
                            .input_touchpad
                            .scroll_config
                            .as_ref()
                            .and_then(|x| x.scroll_factor)
                            .unwrap_or(1.)
                            .log(2.)
                            * 10.0
                            + 50.0,
                        |value| Message::SetScrollFactor(2f64.powf((value - 50.0) / 10.0), true),
                    ),
                ))
                .add(
                    settings::item::builder(&*MOUSE_SCROLL_NATURAL)
                        .description(&*MOUSE_SCROLL_NATURAL_DESC)
                        .toggler(
                            input
                                .input_touchpad
                                .scroll_config
                                .as_ref()
                                .and_then(|x| x.natural_scroll)
                                .unwrap_or(false),
                            |x| Message::SetNaturalScroll(x, true),
                        ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}
