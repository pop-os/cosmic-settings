use cosmic::iced::Alignment;
use cosmic::widget::{self, row, settings, text};
use cosmic::{Apply, Element};
use cosmic_comp_config::input::{AccelProfile, ClickMethod, ScrollMethod};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slab::Slab;
use slotmap::SlotMap;

use super::Message;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(touchpad()),
            sections.insert(click_behavior()),
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
    let mut descriptions = Slab::new();

    let primary_button = descriptions.insert(fl!("primary-button"));
    let touchpad_speed = descriptions.insert(fl!("touchpad", "speed"));
    let acceleration = descriptions.insert(fl!("touchpad", "acceleration"));
    let acceleration_desc = descriptions.insert(fl!("acceleration-desc"));
    let disable_while_typing = descriptions.insert(fl!("disable-while-typing"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |binder, _page, section| {
            let descriptions = &section.descriptions;
            let input = binder.page::<super::Page>().expect("input page not found");
            let theme = cosmic::theme::active();

            settings::view_section(&section.title)
                .add(settings::flex_item(
                    &descriptions[primary_button],
                    cosmic::widget::segmented_control::horizontal(&input.touchpad_primary_button)
                        .minimum_button_width(0)
                        .on_activate(|x| Message::PrimaryButtonSelected(x, true)),
                ))
                .add(
                    settings::item::builder(&descriptions[touchpad_speed]).flex_control({
                        let value = (input
                            .input_touchpad
                            .acceleration
                            .as_ref()
                            .map_or(0.0, |x| x.speed)
                            + 0.81)
                            * 70.71;

                        let slider = widget::slider(1.0..=100.0, value, |value| {
                            Message::SetMouseSpeed((value / 70.71) - 0.81, true)
                        })
                        .width(250.0)
                        .breakpoints(&[50.0]);

                        row::with_capacity(2)
                            .align_items(Alignment::Center)
                            .spacing(theme.cosmic().space_s())
                            .push(text(format!("{:.0}", value.round())))
                            .push(slider)
                    }),
                )
                .add(
                    settings::item::builder(&descriptions[acceleration])
                        .description(&descriptions[acceleration_desc])
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
                    settings::item::builder(&descriptions[disable_while_typing]).toggler(
                        input.input_touchpad.disable_while_typing.unwrap_or(false),
                        |enabled| Message::DisableWhileTyping(enabled, true),
                    ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn click_behavior() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let click_finger = descriptions.insert(fl!("click-behavior", "click-finger"));
    let button_areas = descriptions.insert(fl!("click-behavior", "button-areas"));
    let tap_to_click = descriptions.insert(fl!("tap-to-click"));
    let _tap_to_click_desc = descriptions.insert(fl!("tap-to-click", "desc"));

    Section::default()
        .title(fl!("click-behavior"))
        .descriptions(descriptions)
        .view::<Page>(move |binder, _page, section| {
            let descriptions = &section.descriptions;
            let page = binder
                .page::<super::Page>()
                .expect("input devices page not found");

            settings::view_section(&*section.title)
                // Secondary click via two fingers, and middle-click via three fingers
                .add(settings::item_row(vec![widget::radio(
                    &descriptions[click_finger],
                    ClickMethod::Clickfinger,
                    page.input_touchpad.click_method,
                    |option| Message::SetSecondaryClickBehavior(Some(option), true),
                )
                .into()]))
                // Secondary and middle-click via button areas.
                .add(settings::item_row(vec![widget::radio(
                    &descriptions[button_areas],
                    ClickMethod::ButtonAreas,
                    page.input_touchpad.click_method,
                    |option| Message::SetSecondaryClickBehavior(Some(option), true),
                )
                .into()]))
                .add(
                    settings::item::builder(&descriptions[tap_to_click]).toggler(
                        page.input_touchpad
                            .tap_config
                            .as_ref()
                            .map_or(false, |x| x.enabled),
                        Message::TapToClick,
                    ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn scrolling() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let edge = descriptions.insert(fl!("scrolling", "edge"));
    let natural = descriptions.insert(fl!("scrolling", "natural"));
    let natural_desc = descriptions.insert(fl!("scrolling", "natural-desc"));
    let scroll_speed = descriptions.insert(fl!("scrolling", "speed"));
    let two_finger = descriptions.insert(fl!("scrolling", "two-finger"));

    Section::default()
        .title(fl!("scrolling"))
        .descriptions(descriptions)
        .view::<Page>(move |binder, _page, section| {
            let descriptions = &section.descriptions;
            let page = binder
                .page::<super::Page>()
                .expect("input devices page not found");
            let theme = cosmic::theme::active();

            settings::view_section(&section.title)
                // Two-finger scrolling toggle
                .add(settings::item_row(vec![widget::radio(
                    &descriptions[two_finger],
                    ScrollMethod::TwoFinger,
                    page.input_touchpad
                        .scroll_config
                        .as_ref()
                        .and_then(|x| x.method),
                    |option| Message::SetScrollMethod(Some(option), true),
                )
                .into()]))
                // Edge scrolling toggle
                .add(settings::item_row(vec![widget::radio(
                    &descriptions[edge],
                    ScrollMethod::Edge,
                    page.input_touchpad
                        .scroll_config
                        .as_ref()
                        .and_then(|x| x.method),
                    |option| Message::SetScrollMethod(Some(option), true),
                )
                .into()]))
                // Scroll speed slider
                .add(settings::item(&descriptions[scroll_speed], {
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
                    settings::item::builder(&descriptions[natural])
                        .description(&descriptions[natural_desc])
                        .toggler(
                            page.input_touchpad
                                .scroll_config
                                .as_ref()
                                .map_or(false, |conf| conf.natural_scroll.unwrap_or(false)),
                            |enabled| Message::SetNaturalScroll(enabled, true),
                        ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn swiping() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let four_finger_down = descriptions.insert(fl!("gestures", "four-finger-down"));
    // let four_finger_left = descriptions.insert(fl!("gestures", "four-finger-left"));
    // let four_finger_right = descriptions.insert(fl!("gestures", "four-finger-right"));
    let four_finger_up = descriptions.insert(fl!("gestures", "four-finger-up"));
    // let three_finger_any = descriptions.insert(fl!("gestures", "three-finger-any"));

    // let open_application_library = descriptions.insert(fl!("open-application-library"));
    // let open_workspaces_view = descriptions.insert(fl!("open-workspaces-view"));
    // let switch_between_windows = descriptions.insert(fl!("switch-between-windows"));
    let switch_to_next_workspace = descriptions.insert(fl!("switch-to-next-workspace"));
    let switch_to_prev_workspace = descriptions.insert(fl!("switch-to-prev-workspace"));

    Section::default()
        .title(fl!("gestures"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&*section.title)
                // .add(
                //     settings::item::builder(&descriptions[three_finger_any])
                //         .flex_control(text(&descriptions[switch_between_windows])),
                // )
                .add(
                    settings::item::builder(&descriptions[four_finger_up])
                        .flex_control(text(&descriptions[switch_to_prev_workspace])),
                )
                .add(
                    settings::item::builder(&descriptions[four_finger_down])
                        .flex_control(text(&descriptions[switch_to_next_workspace])),
                )
                // .add(
                //     settings::item::builder(&descriptions[four_finger_left])
                //         .flex_control(text(&descriptions[open_workspaces_view])),
                // )
                // .add(
                //     settings::item::builder(&descriptions[four_finger_right])
                //         .flex_control(text(&descriptions[open_application_library])),
                // )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}
