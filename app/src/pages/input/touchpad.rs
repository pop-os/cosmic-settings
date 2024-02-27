use cosmic::iced::Alignment;
use cosmic::widget::{self, row, settings, text};
use cosmic::{Apply, Element};
use cosmic_comp_config::input::{AccelProfile, ScrollMethod};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

use super::Message;

crate::cache_dynamic_lazy! {
    static EDGE_SCROLLING_DESC: String = fl!("edge-scrolling", "desc");
    static EDGE_SCROLLING: String = fl!("edge-scrolling");
    static TWO_FINGER_SCROLLING: String = fl!("two-finger-scrolling");
    static PINCH_TO_ZOOM_DESC: String = fl!("pinch-to-zoom", "desc");
    static PINCH_TO_ZOOM: String = fl!("pinch-to-zoom");
    static TAP_TO_CLICK_DESC: String = fl!("tap-to-click", "desc");
    static TAP_TO_CLICK: String = fl!("tap-to-click");
    static TAPPING_AND_PINCHING: String = fl!("tapping-and-pinching");
    static TOUCHPAD_ACCELERAION: String = fl!("touchpad", "acceleration");
    static TOUCHPAD_SPEED: String = fl!("touchpad", "speed");

    static OPEN_APPLICATION_LIBRARY: String = fl!("open-application-library");
    static OPEN_WORKSPACES_VIEW: String = fl!("open-workspaces-view");
    static SWIPING_FOUR_FINGER_DOWN: String = fl!("swiping", "four-finger-down");
    static SWIPING_FOUR_FINGER_LEFT: String = fl!("swiping", "four-finger-left");
    static SWIPING_FOUR_FINGER_RIGHT: String = fl!("swiping", "four-finger-right");
    static SWIPING_FOUR_FINGER_UP: String = fl!("swiping", "four-finger-up");
    static SWIPING_THREE_FINGER_ANY: String = fl!("swiping", "three-finger-any");
    static SWITCH_BETWEEN_WINDOWS: String = fl!("switch-between-windows");
    static SWITCH_TO_NEXT_WORKSPACE: String = fl!("switch-to-next-workspace");
    static SWITCH_TO_PREV_WORKSPACE: String = fl!("switch-to-prev-workspace");
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
            sections.insert(tapping_and_pinching()),
            sections.insert(scrolling()),
            sections.insert(swiping()),
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
            super::PRIMARY_BUTTON.as_str().into(),
            TOUCHPAD_SPEED.as_str().into(),
            TOUCHPAD_ACCELERAION.as_str().into(),
            super::ACCELERATION_DESC.as_str().into(),
            super::DISABLE_WHILE_TYPING.as_str().into(),
        ])
        .view::<Page>(|binder, _page, section| {
            let input = binder.page::<super::Page>().expect("input page not found");
            let theme = cosmic::theme::active();

            settings::view_section(&section.title)
                .add(settings::item(
                    &*super::PRIMARY_BUTTON,
                    cosmic::widget::segmented_control::horizontal(&input.touchpad_primary_button)
                        .minimum_button_width(0)
                        .on_activate(|x| Message::PrimaryButtonSelected(x, true)),
                ))
                .add(settings::item::builder(&*TOUCHPAD_SPEED).control({
                    let value = (input
                        .input_touchpad
                        .acceleration
                        .as_ref()
                        .map_or(0.0, |x| x.speed)
                        + 1.0)
                        * 50.0;

                    let slider = widget::slider(10.0..=90.0, value, |value| {
                        Message::SetMouseSpeed((value / 50.0) - 1.0, true)
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
                    settings::item::builder(&*TOUCHPAD_ACCELERAION)
                        .description(&*super::ACCELERATION_DESC)
                        .toggler(
                            input
                                .input_touchpad
                                .acceleration
                                .as_ref()
                                .map_or(true, |x| x.profile == Some(AccelProfile::Adaptive)),
                            |x| Message::SetAcceleration(x, true),
                        ),
                )
                .add(
                    settings::item::builder(&*super::DISABLE_WHILE_TYPING).toggler(
                        input.input_touchpad.disable_while_typing.unwrap_or(false),
                        |enabled| Message::DisableWhileTyping(enabled, true),
                    ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn tapping_and_pinching() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("tapping-and-pinching"))
        .descriptions(vec![
            TAP_TO_CLICK.as_str().into(),
            TAP_TO_CLICK_DESC.as_str().into(),
            PINCH_TO_ZOOM.as_str().into(),
            PINCH_TO_ZOOM_DESC.as_str().into(),
        ])
        .view::<Page>(|binder, _page, _section| {
            let page = binder
                .page::<super::Page>()
                .expect("input devices page not found");

            settings::view_section(&*TAPPING_AND_PINCHING)
                .add(
                    settings::item::builder(&*TAP_TO_CLICK).toggler(
                        page.input_touchpad
                            .tap_config
                            .as_ref()
                            .map_or(false, |x| x.enabled),
                        Message::TapToClick,
                    ),
                )
                .add(settings::item::builder(&*PINCH_TO_ZOOM).toggler(false, Message::PinchToZoom))
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
            TWO_FINGER_SCROLLING.as_str().into(),
            EDGE_SCROLLING.as_str().into(),
            EDGE_SCROLLING_DESC.as_str().into(),
        ])
        .view::<Page>(|binder, _page, section| {
            let page = binder
                .page::<super::Page>()
                .expect("input devices page not found");
            let theme = cosmic::theme::active();

            settings::view_section(&section.title)
                // Scroll speed slider
                .add(settings::item(&*super::SCROLLING_SPEED, {
                    let value = page
                        .input_touchpad
                        .scroll_config
                        .as_ref()
                        .and_then(|x| x.scroll_factor)
                        .unwrap_or(1.)
                        .log(2.)
                        * 10.0
                        + 50.0;

                    let slider = widget::slider(1.0..=100.0, value, |value| {
                        Message::SetScrollFactor(2f64.powf((value - 50.0) / 10.0), true)
                    })
                    .width(250.0)
                    .breakpoints(&[50.0]);

                    row::with_capacity(2)
                        .align_items(Alignment::Center)
                        .spacing(theme.cosmic().space_s())
                        .push(text(format!("{:.0}", value.round())))
                        .push(slider)
                }))
                // Natural scrolling toggle
                .add(
                    settings::item::builder(&*super::SCROLLING_NATURAL)
                        .description(&*super::SCROLLING_NATURAL_DESC)
                        .toggler(
                            page.input_touchpad
                                .scroll_config
                                .as_ref()
                                .map_or(false, |conf| conf.natural_scroll.unwrap_or(false)),
                            |enabled| Message::SetNaturalScroll(enabled, true),
                        ),
                )
                // Two-finger scrolling toggle
                .add(
                    settings::item::builder(&*TWO_FINGER_SCROLLING).toggler(
                        page.input_touchpad
                            .scroll_config
                            .as_ref()
                            .map_or(false, |x| matches!(x.method, Some(ScrollMethod::TwoFinger))),
                        |enabled| {
                            Message::SetScrollMethod(
                                enabled.then_some(ScrollMethod::TwoFinger),
                                true,
                            )
                        },
                    ),
                )
                // Edge scrolling toggle
                .add(
                    settings::item::builder(&*EDGE_SCROLLING)
                        .description(&*EDGE_SCROLLING_DESC)
                        .toggler(
                            page.input_touchpad
                                .scroll_config
                                .as_ref()
                                .map_or(false, |x| matches!(x.method, Some(ScrollMethod::Edge))),
                            |enabled| {
                                Message::SetScrollMethod(
                                    enabled.then_some(ScrollMethod::Edge),
                                    true,
                                )
                            },
                        ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn swiping() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("swiping"))
        .descriptions(vec![
            SWIPING_FOUR_FINGER_DOWN.as_str().into(),
            SWIPING_FOUR_FINGER_LEFT.as_str().into(),
            SWIPING_FOUR_FINGER_RIGHT.as_str().into(),
            SWIPING_FOUR_FINGER_UP.as_str().into(),
            SWIPING_THREE_FINGER_ANY.as_str().into(),
        ])
        .view::<Page>(|_binder, _page, section| {
            settings::view_section(&*section.title)
                .add(
                    settings::item::builder(&*SWIPING_THREE_FINGER_ANY)
                        .control(text(&*SWITCH_BETWEEN_WINDOWS)),
                )
                .add(
                    settings::item::builder(&*SWIPING_FOUR_FINGER_UP)
                        .control(text(&*SWITCH_TO_PREV_WORKSPACE)),
                )
                .add(
                    settings::item::builder(&*SWIPING_FOUR_FINGER_DOWN)
                        .control(text(&*SWITCH_TO_NEXT_WORKSPACE)),
                )
                .add(
                    settings::item::builder(&*SWIPING_FOUR_FINGER_LEFT)
                        .control(text(&*OPEN_WORKSPACES_VIEW)),
                )
                .add(
                    settings::item::builder(&*SWIPING_FOUR_FINGER_RIGHT)
                        .control(text(&*OPEN_APPLICATION_LIBRARY)),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}
