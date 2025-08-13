use cosmic::iced_core::{Alignment, Length, text::Wrapping};
use cosmic::widget::icon::{from_name, icon};
use cosmic::widget::{button, container, settings, text};
use cosmic::{Apply, Element};
use cosmic_settings_page::Section;
use slab::Slab;

use super::{Message, Page, Roundness};

#[allow(clippy::too_many_lines)]
pub fn section() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let round = descriptions.insert(fl!("style", "round"));
    let slightly_round = descriptions.insert(fl!("style", "slightly-round"));
    let square = descriptions.insert(fl!("style", "square"));

    let dark_round_style = from_name("illustration-appearance-dark-style-round").handle();
    let light_round_style = from_name("illustration-appearance-light-style-round").handle();

    let dark_slightly_round_style =
        from_name("illustration-appearance-dark-style-slightly-round").handle();
    let light_slightly_round_style =
        from_name("illustration-appearance-light-style-slightly-round").handle();

    let dark_square_style = from_name("illustration-appearance-dark-style-square").handle();
    let light_square_style = from_name("illustration-appearance-light-style-square").handle();

    fn style_container() -> cosmic::theme::Container<'static> {
        cosmic::theme::Container::custom(|theme| {
            let mut background = theme.cosmic().palette.neutral_9;
            background.alpha = 0.1;
            container::Style {
                background: Some(cosmic::iced::Background::Color(background.into())),
                border: cosmic::iced::Border {
                    radius: theme.cosmic().radius_s().into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
    }

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
                                            &dark_round_style
                                        } else {
                                            &light_round_style
                                        }
                                        .clone()
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fixed(100.0)),
                                    None
                                )
                                .selected(matches!(page.roundness, Roundness::Round))
                                .class(button::ButtonClass::Image)
                                .padding(0)
                                .on_press(Message::Roundness(Roundness::Round))
                                .apply(container)
                                .width(Length::Fixed(191.0))
                                .class(style_container()),
                                text::body(&descriptions[round]).wrapping(Wrapping::None)
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_x(Alignment::Center),
                            cosmic::iced::widget::column![
                                button::custom_image_button(
                                    icon(
                                        if page.theme_manager.mode().is_dark {
                                            &dark_slightly_round_style
                                        } else {
                                            &light_slightly_round_style
                                        }
                                        .clone()
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fixed(100.0)),
                                    None
                                )
                                .selected(matches!(page.roundness, Roundness::SlightlyRound))
                                .class(button::ButtonClass::Image)
                                .padding(0)
                                .on_press(Message::Roundness(Roundness::SlightlyRound))
                                .apply(container)
                                .width(Length::Fixed(191.0))
                                .class(style_container()),
                                text::body(&descriptions[slightly_round]).wrapping(Wrapping::None)
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_x(Alignment::Center),
                            cosmic::iced::widget::column![
                                button::custom_image_button(
                                    icon(
                                        if page.theme_manager.mode().is_dark {
                                            &dark_square_style
                                        } else {
                                            &light_square_style
                                        }
                                        .clone()
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fixed(100.0)),
                                    None
                                )
                                .width(Length::FillPortion(1))
                                .selected(matches!(page.roundness, Roundness::Square))
                                .class(button::ButtonClass::Image)
                                .padding(0)
                                .on_press(Message::Roundness(Roundness::Square))
                                .apply(container)
                                .width(Length::Fixed(191.0))
                                .class(style_container()),
                                text::body(&descriptions[square]).wrapping(Wrapping::None)
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
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}
