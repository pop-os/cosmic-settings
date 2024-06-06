// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    iced::Length,
    theme,
    widget::{self, button, container, horizontal_space, icon, row, settings, toggler},
    Apply, Element,
};

use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_config::{shortcuts, Action, Binding, Shortcuts};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slab::Slab;
use slotmap::SlotMap;

#[derive(Copy, Clone, Debug)]
pub enum Message {
    SuperKey(usize),
}

pub struct Page {
    pub super_key_selections: Vec<String>,
    pub super_key_active: Option<usize>,
}

impl Default for Page {
    fn default() -> Self {
        Page {
            super_key_selections: vec![
                fl!("super-key", "launcher"),
                fl!("super-key", "workspaces"),
                fl!("super-key", "applications"),
            ],
            super_key_active: super_key_active_config(),
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SuperKey(id) => {
                let action = match id {
                    0 => shortcuts::action::System::Launcher,
                    1 => shortcuts::action::System::WorkspaceOverview,
                    2 => shortcuts::action::System::AppLibrary,
                    _ => return,
                };

                self.super_key_active = Some(id);
                super_key_set(action);
            }
        }
    }
}

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
    let mut descriptions = Slab::new();

    let super_key = descriptions.insert(fl!("super-key"));
    let _launcher = descriptions.insert(fl!("super-key", "launcher"));
    let _workspaces = descriptions.insert(fl!("super-key", "workspaces"));
    let _applications = descriptions.insert(fl!("super-key", "applications"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(
                    settings::item::builder(&descriptions[super_key]).control(widget::dropdown(
                        &page.super_key_selections,
                        page.super_key_active,
                        Message::SuperKey,
                    )),
                )
                .apply(Element::from)
                .map(crate::pages::Message::DesktopOptions)
        })
}

pub fn window_controls() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let minimize = descriptions.insert(fl!("window-controls", "minimize"));
    let maximize = descriptions.insert(fl!("window-controls", "maximize"));

    Section::default()
        .title(fl!("window-controls"))
        .descriptions(descriptions)
        .view::<Page>(move |binder, _page, section| {
            let desktop = binder
                .page::<super::Page>()
                .expect("desktop page not found");
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(settings::flex_item(
                    &descriptions[minimize],
                    toggler(
                        None,
                        desktop.cosmic_tk.show_minimize,
                        super::Message::ShowMinimizeButton,
                    ),
                ))
                .add(settings::flex_item(
                    &descriptions[maximize],
                    toggler(
                        None,
                        desktop.cosmic_tk.show_maximize,
                        super::Message::ShowMaximizeButton,
                    ),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::Desktop)
        })
}

pub fn panel_dock_links() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("desktop-panels-and-applets"))
        .view::<Page>(move |binder, _page, section| {
            // TODO probably a way of getting the entity and its info
            let mut settings = settings::view_section(&section.title);

            if let Some((panel_entity, panel_info)) =
                binder.info.iter().find(|(_, v)| v.id == "panel")
            {
                let control = row::with_children(vec![
                    horizontal_space(Length::Fill).into(),
                    icon::from_name("go-next-symbolic").size(16).into(),
                ]);

                settings = settings.add(
                    settings::item::builder(panel_info.title.clone())
                        .description(panel_info.description.clone())
                        .control(control)
                        .spacing(16)
                        .apply(container)
                        .style(theme::Container::List)
                        .apply(button)
                        .style(theme::Button::Transparent)
                        .on_press(crate::pages::Message::Page(panel_entity)),
                );
            }

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

fn super_key_active_config() -> Option<usize> {
    let super_binding = Binding::new(shortcuts::Modifiers::new().logo(), None);

    let config = shortcuts::context().ok()?;
    let shortcuts = config.get::<Shortcuts>("shortcuts").ok()?;

    let new_id = shortcuts
        .iter()
        .find(|(binding, _action)| binding == &&super_binding)
        .and_then(|(_, action)| match action {
            Action::System(shortcuts::action::System::Launcher) => Some(0),
            Action::System(shortcuts::action::System::WorkspaceOverview) => Some(1),
            Action::System(shortcuts::action::System::AppLibrary) => Some(2),
            _ => None,
        });

    new_id
}

fn super_key_set(action: shortcuts::action::System) {
    let Ok(config) = shortcuts::context() else {
        return;
    };

    let Ok(mut shortcuts) = config.get::<Shortcuts>("shortcuts") else {
        return;
    };

    shortcuts.0.insert(
        Binding::new(shortcuts::Modifiers::new().logo(), None),
        Action::System(action),
    );

    _ = config.set("shortcuts", &shortcuts);
}
