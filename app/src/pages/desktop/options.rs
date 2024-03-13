// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::Message;
use apply::Apply;
use cosmic::{
    iced::Length,
    theme,
    widget::{button, container, horizontal_space, icon, row, settings, toggler},
    Element,
};

use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(super_key_action()),
            sections.insert(window_controls()),
            sections.insert(panel_dock_links()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("desktop-panel-options", "video-display-symbolic")
            .title(fl!("desktop-panel-options"))
            .description(fl!("desktop-panel-options", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<super::panel::Page>()
            .sub_page::<super::dock::Page>()
    }
}

pub fn super_key_action() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("super-key-action"))
        .descriptions(vec![
            fl!("super-key-action", "launcher").into(),
            fl!("super-key-action", "workspaces").into(),
            fl!("super-key-action", "applications").into(),
        ])
        .view::<Page>(|_binder, _page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(settings::item(
                    &*descriptions[0],
                    horizontal_space(Length::Fill),
                ))
                .add(settings::item(
                    &*descriptions[1],
                    horizontal_space(Length::Fill),
                ))
                .add(settings::item(
                    &*descriptions[2],
                    horizontal_space(Length::Fill),
                ))
                .into()
        })
}

pub fn window_controls() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("window-controls"))
        .descriptions(vec![
            fl!("window-controls", "minimize").into(),
            fl!("window-controls", "maximize").into(),
        ])
        .view::<Page>(|binder, _page, section| {
            let desktop = binder
                .page::<super::Page>()
                .expect("desktop page not found");
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(settings::item(
                    &*descriptions[0],
                    toggler(
                        None,
                        desktop.cosmic_tk.show_minimize,
                        Message::ShowMinimizeButton,
                    ),
                ))
                .add(settings::item(
                    &*descriptions[1],
                    toggler(
                        None,
                        desktop.cosmic_tk.show_maximize,
                        Message::ShowMaximizeButton,
                    ),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::Desktop)
        })
}

pub fn panel_dock_links() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("desktop-panels-and-applets"))
        .view::<Page>(|binder, _page, section| {
            // TODO probably a way of getting the entity and its info
            let mut settings = settings::view_section(&section.title);
            settings = if let Some((panel_entity, panel_info)) =
                binder.info.iter().find(|(_, v)| v.id == "panel")
            {
                let control = row::with_children(vec![
                    horizontal_space(Length::Fill).into(),
                    icon::from_name("go-next-symbolic").size(16).into(),
                ]);

                settings.add(
                    settings::item::builder(panel_info.title.clone())
                        .description(panel_info.description.clone())
                        .control(control)
                        .spacing(16)
                        .apply(container)
                        .style(theme::Container::List)
                        .apply(button)
                        .style(theme::Button::Transparent)
                        .on_press(crate::pages::Message::Page(panel_entity)),
                )
            } else {
                settings
            };

            settings = if let Some((dock_entity, dock_info)) =
                binder.info.iter().find(|(_, v)| v.id == "dock")
            {
                let control = row::with_children(vec![
                    horizontal_space(Length::Fill).into(),
                    icon::from_name("go-next-symbolic").size(16).into(),
                ]);

                settings.add(
                    settings::item::builder(dock_info.title.clone())
                        .description(dock_info.description.clone())
                        .control(control)
                        .spacing(16)
                        .apply(container)
                        .style(theme::Container::List)
                        .apply(button)
                        .style(theme::Button::Transparent)
                        .on_press(crate::pages::Message::Page(dock_entity)),
                )
            } else {
                settings
            };

            Element::from(settings)
        })
}
