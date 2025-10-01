// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::cosmic_config::ConfigGet;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, row, settings, text};
use cosmic::{Apply, Element};
use cosmic_comp_config::CosmicCompConfig;
use cosmic_comp_config::input::{AccelProfile, ClickMethod, ScrollMethod};
use cosmic_comp_config::workspace::{WorkspaceConfig, WorkspaceLayout};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slab::Slab;
use slotmap::SlotMap;
use tracing::error;

use super::Message;

pub struct Page {
    comp_workspace_config: WorkspaceConfig,
}

impl Default for Page {
    fn default() -> Self {
        let comp_config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let comp_workspace_config = comp_config.get("workspaces").unwrap_or_else(|err| {
            if err.is_err() {
                error!(?err, "Failed to read config 'workspaces'");
            }

            WorkspaceConfig::default()
        });
        Self {
            comp_workspace_config,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(touchpad()),
            sections.insert(click_behavior()),
            sections.insert(scrolling()),
            sections.insert(gestures()),
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
    let primary_button_desc = descriptions.insert(fl!("primary-button", "desc"));
    let touchpad_speed = descriptions.insert(fl!("touchpad", "speed"));
    let acceleration = descriptions.insert(fl!("touchpad", "acceleration"));
    let acceleration_desc = descriptions.insert(fl!("acceleration-desc"));
    let disable_while_typing = descriptions.insert(fl!("disable-while-typing"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |binder, _page, section| {
            let descriptions = &section.descriptions;
            let input = binder.page::<super::Page>().expect("input page not found");

            settings::section()
                .title(&section.title)
                .add(
                    settings::item::builder(&descriptions[primary_button])
                        .description(&descriptions[primary_button_desc])
                        .flex_control(
                            cosmic::widget::segmented_control::horizontal(
                                &input.touchpad_primary_button,
                            )
                            .minimum_button_width(0)
                            .on_activate(|x| Message::PrimaryButtonSelected(x, true)),
                        ),
                )
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
                        .width(Length::Fill)
                        .breakpoints(&[50.0])
                        .apply(widget::container)
                        .max_width(250);

                        row::with_capacity(2)
                            .align_y(Alignment::Center)
                            .spacing(8)
                            .push(
                                text::body(format!("{:.0}", value.round()))
                                    .width(Length::Fixed(22.0))
                                    .align_x(Alignment::Center),
                            )
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
                                .is_none_or(|x| x.profile == Some(AccelProfile::Adaptive)),
                            |x| Message::SetAcceleration(x, true),
                        ),
                )
                .add(
                    settings::item::builder(&descriptions[disable_while_typing]).toggler(
                        input
                            .input_touchpad
                            .disable_while_typing
                            .unwrap_or_else(|| {
                                CosmicCompConfig::default()
                                    .input_touchpad
                                    .disable_while_typing
                                    .unwrap_or(true)
                            }),
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

            settings::section()
                .title(&*section.title)
                // Secondary click via two fingers, and middle-click via three fingers
                .add(settings::item_row(vec![
                    widget::radio(
                        text::body(&descriptions[click_finger]),
                        ClickMethod::Clickfinger,
                        page.input_touchpad.click_method,
                        |option| Message::SetSecondaryClickBehavior(Some(option), true),
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                // Secondary and middle-click via button areas.
                .add(settings::item_row(vec![
                    widget::radio(
                        text::body(&descriptions[button_areas]),
                        ClickMethod::ButtonAreas,
                        page.input_touchpad.click_method,
                        |option| Message::SetSecondaryClickBehavior(Some(option), true),
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(
                    settings::item::builder(&descriptions[tap_to_click]).toggler(
                        page.input_touchpad
                            .tap_config
                            .as_ref()
                            .is_some_and(|x| x.enabled),
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

            settings::section()
                .title(&section.title)
                // Two-finger scrolling toggle
                .add(settings::item_row(vec![
                    widget::radio(
                        text::body(&descriptions[two_finger]),
                        ScrollMethod::TwoFinger,
                        page.input_touchpad
                            .scroll_config
                            .as_ref()
                            .and_then(|x| x.method),
                        |option| Message::SetScrollMethod(Some(option), true),
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                // Edge scrolling toggle
                .add(settings::item_row(vec![
                    widget::radio(
                        text::body(&descriptions[edge]),
                        ScrollMethod::Edge,
                        page.input_touchpad
                            .scroll_config
                            .as_ref()
                            .and_then(|x| x.method),
                        |option| Message::SetScrollMethod(Some(option), true),
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
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
                    .width(Length::Fill)
                    .breakpoints(&[50.0])
                    .apply(widget::container)
                    .max_width(250);

                    row::with_capacity(2)
                        .align_y(Alignment::Center)
                        .spacing(8)
                        .push(
                            text::body(format!("{:.0}", value.round()))
                                .width(Length::Fixed(22.0))
                                .align_x(Alignment::Center),
                        )
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
                                .is_some_and(|conf| conf.natural_scroll.unwrap_or(false)),
                            |enabled| Message::SetNaturalScroll(enabled, true),
                        ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn gestures() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    // let four_finger_down = descriptions.insert(fl!("gestures", "four-finger-down"));
    // let four_finger_left = descriptions.insert(fl!("gestures", "four-finger-left"));
    // let four_finger_right = descriptions.insert(fl!("gestures", "four-finger-right"));
    // let four_finger_up = descriptions.insert(fl!("gestures", "four-finger-up"));
    // let three_finger_any = descriptions.insert(fl!("gestures", "three-finger-any"));

    // let open_application_library = descriptions.insert(fl!("open-application-library"));
    // let open_workspaces_view = descriptions.insert(fl!("open-workspaces-view"));
    // let switch_between_windows = descriptions.insert(fl!("switch-between-windows"));

    let switch_workspaces = descriptions.insert(fl!("switch-workspaces"));
    let switch_workspaces_horizontal = descriptions.insert(fl!("switch-workspaces", "horizontal"));
    let switch_workspaces_vertical = descriptions.insert(fl!("switch-workspaces", "vertical"));

    Section::default()
        .title(fl!("gestures"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                // .add(
                //     settings::item::builder(&descriptions[three_finger_any])
                //         .flex_control(text::body(&descriptions[switch_between_windows])),
                // )
                .add(
                    settings::item::builder(
                        &descriptions[match page.comp_workspace_config.workspace_layout {
                            WorkspaceLayout::Horizontal => switch_workspaces_horizontal,
                            WorkspaceLayout::Vertical => switch_workspaces_vertical,
                        }],
                    )
                    .flex_control(text::body(&descriptions[switch_workspaces])),
                )
                // .add(
                //     settings::item::builder(
                //         &descriptions[match page.comp_workspace_config.workspace_layout {
                //             WorkspaceLayout::Horizontal => four_finger_down,
                //             WorkspaceLayout::Vertical => four_finger_right,
                //         }],
                //     )
                //     .flex_control(text::body(&descriptions[open_workspaces_view])),
                // )
                // .add(
                //     settings::item::builder(
                //         &descriptions[match page.comp_workspace_config.workspace_layout {
                //             WorkspaceLayout::Horizontal => four_finger_up,
                //             WorkspaceLayout::Vertical => four_finger_left,
                //         }],
                //     )
                //     .flex_control(text::body(&descriptions[open_application_library])),
                // )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}
