use crate::pages::desktop::wallpaper::widgets::color_image;
use cosmic::cosmic_theme::Spacing;
use cosmic::cosmic_theme::palette::Srgba;
use cosmic::iced::ContentFit;
use cosmic::iced_core::{Alignment, Length};
use cosmic::widget::icon::{from_name, icon};
use cosmic::widget::{self, button, container, settings, text};
use cosmic::{Apply, Element};
use cosmic_settings_page::Section;
use cosmic_settings_wallpaper as wallpaper;
use std::collections::HashMap;

use super::{ContextView, Message, Page};

#[allow(clippy::too_many_lines)]
pub fn section() -> Section<crate::pages::Message> {
    let (descriptions, label_keys) = i18n();

    Section::default()
        .title(fl!("mode-and-colors"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let label_keys = label_keys.clone();
            let descriptions = &section.descriptions;
            let theme_manager = &page.theme_manager;

            let mut section = settings::section()
                .title(&section.title)
                .add(theme_mode(page, section, &label_keys))
                .add(auto_switch(page, section, &label_keys))
                .add(accent_color_palette(page, section, &label_keys))
                .add(application_background(page, section, &label_keys))
                .add(container_background(page, section, &label_keys))
                .add(interface_text(page, section, &label_keys))
                .add(control_tint(page, section, &label_keys))
                .add(
                    settings::item::builder(&descriptions[label_keys["window_hint_toggle"]])
                        .toggler(
                            theme_manager.custom_window_hint().is_none(),
                            Message::UseDefaultWindowHint,
                        ),
                );
            if theme_manager.custom_window_hint().is_some() {
                section = section.add(
                    settings::item::builder(&descriptions[label_keys["window_hint"]]).control(
                        page.drawer
                            .accent_window_hint
                            .picker_button(
                                |_| Message::DrawerOpen(ContextView::AccentWindowHint),
                                Some(24),
                            )
                            .width(Length::Fixed(48.0))
                            .height(Length::Fixed(24.0)),
                    ),
                );
            }
            section
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

fn container_background<'a>(
    page: &Page,
    section: &'a Section<crate::pages::Message>,
    labels: &HashMap<String, usize>,
) -> impl Into<Element<'a, Message>> {
    let descriptions = &section.descriptions;
    let go_next_icon = from_name("go-next-symbolic").handle();

    settings::item::builder(&descriptions[labels["container_bg"]])
        .description(&descriptions[labels["container_bg_desc"]])
        .control(
            if page
                .drawer
                .container_background
                .get_applied_color()
                .is_some()
            {
                Element::from(
                    page.drawer
                        .container_background
                        .picker_button(
                            |_| Message::DrawerOpen(ContextView::ContainerBackground),
                            Some(24),
                        )
                        .width(Length::Fixed(48.0))
                        .height(Length::Fixed(24.0)),
                )
            } else {
                container(
                    button::text(&descriptions[labels["auto"]])
                        .trailing_icon(go_next_icon.clone())
                        .on_press(Message::DrawerOpen(ContextView::ContainerBackground)),
                )
                .into()
            },
        )
}

fn application_background<'a>(
    page: &Page,
    section: &'a Section<crate::pages::Message>,
    labels: &HashMap<String, usize>,
) -> impl Into<Element<'a, Message>> {
    let descriptions = &section.descriptions;

    settings::item::builder(&descriptions[labels["app_bg"]]).control(
        page.drawer
            .application_background
            .picker_button(
                |_| Message::DrawerOpen(ContextView::ApplicationBackground),
                Some(24),
            )
            .width(Length::Fixed(48.0))
            .height(Length::Fixed(24.0)),
    )
}

fn control_tint<'a>(
    page: &Page,
    section: &'a Section<crate::pages::Message>,
    labels: &HashMap<String, usize>,
) -> impl Into<Element<'a, Message>> {
    let descriptions = &section.descriptions;

    settings::item::builder(&descriptions[labels["control_tint"]])
        .description(&descriptions[labels["control_tint_desc"]])
        .control(
            page.drawer
                .control_component
                .picker_button(
                    |_| Message::DrawerOpen(ContextView::ControlComponent),
                    Some(24),
                )
                .width(Length::Fixed(48.0))
                .height(Length::Fixed(24.0)),
        )
}

fn interface_text<'a>(
    page: &Page,
    section: &'a Section<crate::pages::Message>,
    labels: &HashMap<String, usize>,
) -> impl Into<Element<'a, Message>> {
    let descriptions = &section.descriptions;

    settings::item::builder(&descriptions[labels["text_tint"]])
        .description(&descriptions[labels["text_tint_desc"]])
        .control(
            page.drawer
                .interface_text
                .picker_button(
                    |_| Message::DrawerOpen(ContextView::InterfaceText),
                    Some(24),
                )
                .width(Length::Fixed(48.0))
                .height(Length::Fixed(24.0)),
        )
}
fn auto_switch<'a>(
    page: &Page,
    section: &'a Section<crate::pages::Message>,
    labels: &HashMap<String, usize>,
) -> impl Into<Element<'a, Message>> {
    let descriptions = &section.descriptions;

    settings::item::builder(&descriptions[labels["auto_switch"]])
        .description(
            if !page.day_time && page.theme_manager.mode().is_dark {
                &descriptions[labels["auto_switch_desc/sunrise"]]
            } else if page.day_time && !page.theme_manager.mode().is_dark {
                &descriptions[labels["auto_switch_desc/sunset"]]
            } else if page.day_time && page.theme_manager.mode().is_dark {
                &descriptions[labels["auto_switch_desc/next-sunrise"]]
            } else {
                &descriptions[labels["auto_switch_desc/next-sunset"]]
            }
            .clone(),
        )
        .toggler(page.theme_manager.mode().auto_switch, Message::Autoswitch)
}

fn accent_color_palette<'a>(
    page: &Page,
    section: &'a Section<crate::pages::Message>,
    labels: &HashMap<String, usize>,
) -> impl Into<Element<'a, Message>> {
    let Spacing { space_xxs, .. } = cosmic::theme::spacing();
    let descriptions = &section.descriptions;
    let accent = page.theme_manager.accent_palette().as_ref().unwrap();
    let cur_accent = page.theme_manager.builder().accent.map_or_else(
        || page.theme_manager.builder().palette.as_ref().accent_blue,
        Srgba::from,
    );
    let mut accent_palette_row = Vec::with_capacity(accent.len());

    for &color in accent {
        accent_palette_row.push(color_button(
            Some(Message::PaletteAccent(color.into())),
            color.into(),
            cur_accent == color,
            48,
            48,
        ));
    }

    accent_palette_row.push(
        if let Some(c) = page.drawer.custom_accent.get_applied_color() {
            container(color_button(
                Some(Message::DrawerOpen(ContextView::CustomAccent)),
                c,
                cosmic::iced::Color::from(cur_accent) == c,
                48,
                48,
            ))
        } else {
            container(
                page.drawer
                    .custom_accent
                    .picker_button(|_| Message::DrawerOpen(ContextView::CustomAccent), Some(24))
                    .width(Length::Fixed(48.0))
                    .height(Length::Fixed(48.0)),
            )
        }
        .into(),
    );

    cosmic::iced::widget::column![
        text::body(&descriptions[labels["accent_color"]]),
        widget::flex_row(accent_palette_row).spacing(16)
    ]
    .spacing(space_xxs)
}

fn theme_mode<'a>(
    page: &Page,
    section: &'a Section<crate::pages::Message>,
    labels: &HashMap<String, usize>,
) -> impl Into<Element<'a, Message>> {
    let descriptions = &section.descriptions;
    let dark_mode_illustration = from_name("illustration-appearance-mode-dark").handle();
    let light_mode_illustration = from_name("illustration-appearance-mode-light").handle();

    container(
        cosmic::iced::widget::row![
            cosmic::iced::widget::column![
                button::custom_image_button(
                    icon(dark_mode_illustration)
                        .content_fit(ContentFit::Contain)
                        .width(Length::Fill)
                        .height(Length::Fixed(100.0)),
                    None
                )
                .class(button::ButtonClass::Image)
                .selected(page.theme_manager.mode().is_dark)
                .on_press(super::Message::DarkMode(true))
                .padding(1)
                .apply(widget::container)
                .max_width(191),
                text::body(&descriptions[labels["dark"]])
            ]
            .spacing(8)
            .width(Length::FillPortion(1))
            .align_x(Alignment::Center),
            cosmic::iced::widget::column![
                button::custom_image_button(
                    icon(light_mode_illustration)
                        .content_fit(ContentFit::Contain)
                        .width(Length::Fill)
                        .height(Length::Fixed(100.0)),
                    None
                )
                .class(button::ButtonClass::Image)
                .selected(!page.theme_manager.mode().is_dark)
                .on_press(super::Message::DarkMode(false))
                .padding(1)
                .apply(widget::container)
                .max_width(191),
                text::body(&descriptions[labels["light"]])
            ]
            .spacing(8)
            .width(Length::FillPortion(1))
            .align_x(Alignment::Center)
        ]
        .spacing(8)
        .width(Length::Fixed(478.0))
        .align_y(Alignment::Center),
    )
    .center_x(Length::Fill)
}

/// A button for selecting a color or gradient.
pub fn color_button<'a, Message: 'a + Clone>(
    on_press: Option<Message>,
    color: cosmic::iced::Color,
    selected: bool,
    width: u16,
    height: u16,
) -> Element<'a, Message> {
    button::custom_image_button(
        color_image(
            wallpaper::Color::Single([color.r, color.g, color.b]),
            width,
            height,
            None,
        ),
        None,
    )
    .padding(0)
    .selected(selected)
    .class(button::ButtonClass::Image)
    .on_press_maybe(on_press)
    .width(Length::Fixed(f32::from(width)))
    .height(Length::Fixed(f32::from(height)))
    .into()
}

#[inline]
fn i18n() -> (slab::Slab<String>, HashMap<String, usize>) {
    let mut descriptions = slab::Slab::new();
    let keys: HashMap<String, usize> = HashMap::from([
        ("auto".into(), descriptions.insert(fl!("auto"))),
        (
            "auto_switch".into(),
            descriptions.insert(fl!("auto-switch")),
        ),
        (
            "auto_switch_desc/sunrise".into(),
            descriptions.insert(fl!("auto-switch", "sunrise")),
        ),
        (
            "auto_switch_desc/sunset".into(),
            descriptions.insert(fl!("auto-switch", "sunset")),
        ),
        (
            "auto_switch_desc/next-sunrise".into(),
            descriptions.insert(fl!("auto-switch", "next-sunrise")),
        ),
        (
            "auto_switch_desc/next-sunset".into(),
            descriptions.insert(fl!("auto-switch", "next-sunset")),
        ),
        (
            "accent_color".into(),
            descriptions.insert(fl!("accent-color")),
        ),
        ("app_bg".into(), descriptions.insert(fl!("app-background"))),
        (
            "container_bg".into(),
            descriptions.insert(fl!("container-background")),
        ),
        (
            "container_bg_desc".into(),
            descriptions.insert(fl!("container-background", "desc")),
        ),
        ("text_tint".into(), descriptions.insert(fl!("text-tint"))),
        (
            "text_tint_desc".into(),
            descriptions.insert(fl!("text-tint", "desc")),
        ),
        (
            "control_tint".into(),
            descriptions.insert(fl!("control-tint")),
        ),
        (
            "control_tint_desc".into(),
            descriptions.insert(fl!("control-tint", "desc")),
        ),
        (
            "window_hint_toggle".into(),
            descriptions.insert(fl!("window-hint-accent-toggle")),
        ),
        (
            "window_hint".into(),
            descriptions.insert(fl!("window-hint-accent")),
        ),
        ("dark".into(), descriptions.insert(fl!("dark"))),
        ("light".into(), descriptions.insert(fl!("light"))),
    ]);

    (descriptions, keys)
}
