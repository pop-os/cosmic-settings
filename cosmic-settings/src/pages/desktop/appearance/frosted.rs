use cosmic::iced::core::gradient::Linear;
use cosmic::iced::core::text::Wrapping;
use cosmic::iced::core::{Alignment, Background, Border, Color, Degrees, Length};
use cosmic::widget::icon::{from_name, icon};
use cosmic::widget::{button, container, row, settings, slider, text};
use cosmic::{Apply, Element};
use cosmic_settings_page::Section;

use super::{Message, Page};

#[derive(Debug, Default, Copy, Clone)]
pub enum Frosted {
    #[default]
    Frosted,
    Glassy,
    Custom,
}

fn gradient_container() -> cosmic::theme::Container<'static> {
    cosmic::theme::Container::custom(|theme| {
        let linear = Linear::new(Degrees(242.4))
            .add_stop(0.0, Color::from_rgb8(0x82, 0xA1, 0xCA))
            .add_stop(0.5, Color::from_rgb8(0xA0, 0x82, 0xB1))
            .add_stop(1.0, Color::from_rgb8(0x3E, 0xB0, 0xBE));

        container::Style {
            background: Some(Background::Gradient(cosmic::iced::core::Gradient::Linear(
                linear,
            ))),
            border: Border {
                radius: theme.cosmic().radius_s().into(),
                ..Default::default()
            },
            ..Default::default()
        }
    })
}

#[allow(clippy::too_many_lines)]
pub fn section() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        frosty = fl!("style", "frosty");
        glassy = fl!("style", "glassy");
        custom = fl!("style", "custom");
    });

    let dark_frosted_style = from_name("illustration-appearance-dark-frosty").handle();
    let light_frosted_style = from_name("illustration-appearance-light-frosty").handle();

    let dark_glassy_style = from_name("illustration-appearance-dark-glassy").handle();
    let light_glassy_style = from_name("illustration-appearance-light-glassy").handle();

    let dark_custom_style = from_name("illustration-appearance-dark-custom").handle();
    let light_custom_style = from_name("illustration-appearance-light-custom").handle();

    Section::default()
        .title(fl!("style"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(
                    container(
                        cosmic::iced::widget::row![
                            cosmic::iced::widget::column![
                                button::custom_image_button(
                                    icon(
                                        if page.theme_manager.mode().is_dark {
                                            &dark_frosted_style
                                        } else {
                                            &light_frosted_style
                                        }
                                        .clone()
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fixed(100.0)),
                                    None
                                )
                                .selected(matches!(page.frosted_mode, Frosted::Frosted))
                                .class(button::ButtonClass::Image)
                                .padding(0)
                                .on_press(Message::Frosted(Frosted::Frosted))
                                .width(Length::Fill)
                                .apply(container)
                                .width(Length::Fixed(191.0))
                                .class(gradient_container()),
                                text::body(&descriptions[frosty]).wrapping(Wrapping::None)
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_x(Alignment::Center),
                            cosmic::iced::widget::column![
                                button::custom_image_button(
                                    icon(
                                        if page.theme_manager.mode().is_dark {
                                            &dark_glassy_style
                                        } else {
                                            &light_glassy_style
                                        }
                                        .clone()
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fixed(100.0)),
                                    None
                                )
                                .selected(matches!(page.frosted_mode, Frosted::Glassy))
                                .class(button::ButtonClass::Image)
                                .padding(0)
                                .on_press(Message::Frosted(Frosted::Glassy))
                                .width(Length::Fill)
                                .apply(container)
                                .width(Length::Fixed(191.0))
                                .class(gradient_container()),
                                text::body(&descriptions[glassy]).wrapping(Wrapping::None)
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_x(Alignment::Center),
                            cosmic::iced::widget::column![
                                button::custom_image_button(
                                    icon(
                                        if page.theme_manager.mode().is_dark {
                                            &dark_custom_style
                                        } else {
                                            &light_custom_style
                                        }
                                        .clone()
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fixed(100.0)),
                                    None
                                )
                                .width(Length::FillPortion(1))
                                .selected(matches!(page.frosted_mode, Frosted::Custom))
                                .class(button::ButtonClass::Image)
                                .padding(0)
                                .on_press(Message::Frosted(Frosted::Custom))
                                .width(Length::Fill)
                                .apply(container)
                                .width(Length::Fixed(191.0))
                                .class(gradient_container()),
                                text::body(&descriptions[custom]).wrapping(Wrapping::None)
                            ]
                            .spacing(8)
                            .align_x(Alignment::Center)
                            .width(Length::FillPortion(1))
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                    )
                    .center_x(Length::Fill),
                )
                .add_maybe(matches!(page.frosted_mode, Frosted::Custom).then(|| {
                    settings::item::builder(fl!("style", "frosted-thickness")).flex_control({
                        row::with_children(vec![
                            text::body(fl!("style", "less")).into(),
                            slider(0..=13, page.frosted, Message::Blur)
                                .width(Length::Fill)
                                .apply(cosmic::widget::container)
                                .max_width(250)
                                .into(),
                            text::body(fl!("style", "more")).into(),
                        ])
                        .align_y(Alignment::Center)
                        .spacing(8)
                        .width(Length::Fill)
                    })
                }))
                .add_maybe(matches!(page.frosted_mode, Frosted::Custom).then(|| {
                    settings::item::builder(fl!("style", "glass-opacity")).flex_control({
                        row::with_children(vec![
                            text::body(fl!("style", "less")).into(),
                            slider(0.0..=1.0, page.glass, Message::Glass)
                                .step(0.05)
                                .width(Length::Fill)
                                .apply(cosmic::widget::container)
                                .max_width(250)
                                .into(),
                            text::body(fl!("style", "more")).into(),
                        ])
                        .align_y(Alignment::Center)
                        .spacing(8)
                        .width(Length::Fill)
                    })
                }))
                .add(
                    settings::item::builder(fl!("style", "frosted-panels"))
                        .description(fl!("style", "frosted-panels-desc"))
                        .toggler(page.frosted_panel, Message::FrostedPanel),
                )
                .add(
                    settings::item::builder(fl!("style", "frosted-applets"))
                        .description(fl!("style", "frosted-applets-desc"))
                        .toggler(page.frosted_applets, Message::FrostedApplets),
                )
                .add(
                    settings::item::builder(fl!("style", "frosted-system-interface"))
                        .description(fl!("style", "frosted-system-interface-desc"))
                        .toggler(
                            page.frosted_system_interface,
                            Message::FrostedSystemInterface,
                        ),
                )
                .add(
                    settings::item::builder(fl!("style", "frosted-windows"))
                        .description(fl!("style", "frosted-windows-desc"))
                        .toggler(page.frosted_windows, Message::FrostedWindows),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}
