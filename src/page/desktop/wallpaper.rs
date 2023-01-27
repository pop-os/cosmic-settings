// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::Message;
use apply::Apply;
use cosmic::{
    iced::widget::{column, container, horizontal_space, image, row, svg, text},
    iced::Length,
    theme,
    widget::{list_column, settings, toggler},
    Element,
};
use slotmap::SlotMap;

use crate::page::{self, section, Content, Section};

pub struct Page;

impl page::Page for Page {
    type Model = super::Model;

    fn page() -> page::Meta {
        page::Meta::new("wallpaper", "preferences-desktop-wallpaper-symbolic")
            .title(fl!("wallpaper"))
            .description(fl!("wallpaper", "desc"))
    }

    fn content(sections: &mut SlotMap<section::Entity, Section>) -> Option<Content> {
        Some(vec![sections.insert(settings())])
    }
}

pub fn settings() -> Section {
    Section::new()
        .descriptions(vec![
            fl!("wallpaper", "same"),
            fl!("wallpaper", "fit"),
            fl!("wallpaper", "slide"),
            fl!("wallpaper", "change"),
        ])
        .view_fn(|app, section| {
            let descriptions = &section.descriptions;
            let desktop = app
                .pages
                .resource::<super::Model>()
                .expect("desktop model is missing");
            let image_paths: Vec<std::path::PathBuf> = Vec::new();

            let mut image_column = Vec::with_capacity(image_paths.len() / 4);
            for chunk in image_paths.chunks(4) {
                let mut image_row = Vec::with_capacity(chunk.len());
                for image_path in chunk.iter() {
                    image_row.push(if image_path.ends_with(".svg") {
                        svg(svg::Handle::from_path(image_path))
                            .width(Length::Units(150))
                            .into()
                    } else {
                        image(image_path).width(Length::Units(150)).into()
                    });
                }
                image_column.push(row(image_row).spacing(16).into());
            }

            let children = vec![
                row!(
                    horizontal_space(Length::Fill),
                    container(
                        image("/usr/share/backgrounds/pop/kate-hazen-COSMIC-desktop-wallpaper.png")
                            .width(Length::Units(300))
                    )
                    .padding(4)
                    .style(theme::Container::Box),
                    horizontal_space(Length::Fill),
                )
                .into(),
                list_column()
                    .add(settings::item(
                        &descriptions[0],
                        toggler(None, desktop.same_background, Message::SameBackground),
                    ))
                    .add(settings::item(&descriptions[1], text("TODO")))
                    .add(settings::item(
                        &descriptions[2],
                        toggler(None, desktop.slideshow, Message::Slideshow),
                    ))
                    .into(),
                column(image_column).spacing(16).into(),
            ];

            settings::view_column(children)
                .padding(0)
                .apply(Element::from)
                .map(crate::Message::Desktop)
        })
}
