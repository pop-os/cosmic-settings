use cosmic::{
    iced::widget::{button, container, horizontal_space, pick_list, row},
    iced::Length,
    theme,
    widget::{icon, list, settings, text, toggler},
    Element,
};

use apply::Apply;
use cosmic_config::CosmicConfigEntry;
use cosmic_panel_config::{AutoHide, CosmicPanelConfig, CosmicPanelOuput, PanelAnchor, PanelSize};
use cosmic_settings_page::{self as page, section, Section};
use slotmap::SlotMap;
use std::borrow::Cow;

mod applets;

pub struct Page {
    config_helper: Option<cosmic_config::Config>,
    panel_config: Option<CosmicPanelConfig>,
    // TODO move these into panel config
    appearance: Appearance,
}

impl Default for Page {
    fn default() -> Self {
        // TODO CosmicPanelConfig should return its own version
        let config_helper = cosmic_config::Config::new("com.system76.CosmicPanel.panel", 1).ok();
        let panel_config = config_helper.as_ref().and_then(|config_helper| {
            // TODO error handling...
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            // If the config is not present, it will be created with the default values and the name will not match
            (panel_config.name == "panel").then_some(panel_config)
        });

        Self {
            config_helper,
            panel_config,
            appearance: Appearance::Dark,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(if self.panel_config.is_some() {
            vec![
                sections.insert(behavior_and_position()),
                sections.insert(style()),
                sections.insert(configuration()),
            ]
        } else {
            vec![sections.insert(add_panel())]
        })
    }

    fn info(&self) -> page::Info {
        page::Info::new("panel", "preferences-pop-desktop-dock-symbolic")
            .title(fl!("panel"))
            .description(fl!("panel", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<applets::Page>()
    }
}

pub fn behavior_and_position() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("panel-behavior-and-position"))
        .descriptions(vec![
            fl!("panel-behavior-and-position", "autohide"),
            fl!("panel-behavior-and-position", "position"),
            fl!("panel-behavior-and-position", "display"),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;
            let panel_config = page.panel_config.as_ref().unwrap();
            settings::view_section(&section.title)
                .add(settings::item(
                    &descriptions[0],
                    toggler(None, panel_config.autohide.is_some(), |value| {
                        Message::AutoHidePanel(value)
                    }),
                ))
                .add(settings::item(
                    &descriptions[1],
                    pick_list(
                        Cow::from(vec![
                            Anchor(PanelAnchor::Top),
                            Anchor(PanelAnchor::Left),
                            Anchor(PanelAnchor::Right),
                            Anchor(PanelAnchor::Bottom),
                        ]),
                        Some(Anchor(panel_config.anchor)),
                        |a| Message::PanelAnchor(a.0),
                    ),
                ))
                .add(settings::item(&descriptions[2], text("todo")))
                .apply(Element::from)
                .map(crate::pages::Message::Panel)
        })
}

pub fn style() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("panel-style"))
        .descriptions(vec![
            fl!("panel-style", "anchor-gap"),
            fl!("panel-style", "extend"),
            fl!("panel-style", "appearance"),
            fl!("panel-style", "size"),
            fl!("panel-style", "background-opacity"),
        ])
        .view::<Page>(|binder, page, section| {
            let descriptions = &section.descriptions;
            let panel_config = page.panel_config.as_ref().unwrap();
            settings::view_section(&section.title)
                .add(settings::item(
                    &descriptions[0],
                    toggler(None, panel_config.anchor_gap, |value| {
                        Message::AnchorGap(value)
                    }),
                ))
                .add(settings::item(
                    &descriptions[1],
                    toggler(None, panel_config.expand_to_edges, |value| {
                        Message::ExtendToEdge(value)
                    }),
                ))
                .add(settings::item(
                    &descriptions[2],
                    pick_list(
                        Cow::from(vec![Appearance::Match, Appearance::Light, Appearance::Dark]),
                        Some(page.appearance),
                        |a| Message::Appearance(a),
                    ),
                ))
                .add(settings::item(&descriptions[3], text("todo")))
                .add(settings::item(&descriptions[4], text("todo")))
                .apply(Element::from)
                .map(crate::pages::Message::Panel)
        })
}

pub fn configuration() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("panel-applets"))
        .descriptions(vec![fl!("panel-applets", "desc")])
        .view::<Page>(|binder, _page, section| {
            let mut settings = settings::view_section(&section.title);
            settings = if let Some((panel_applets_entity, _panel_applets_info)) =
                binder.info.iter().find(|(_, v)| v.id == "panel_applets")
            {
                settings.add(
                    settings::item::builder(fl!("panel-applets", "desc"))
                        .control(row!(
                            horizontal_space(Length::Fill),
                            icon("go-next-symbolic", 20).style(theme::Svg::Symbolic)
                        ))
                        .spacing(16)
                        .apply(container)
                        .style(theme::Container::custom(list::column::style))
                        .apply(button)
                        .padding(0)
                        .style(theme::Button::Transparent)
                        .on_press(crate::pages::Message::Page(panel_applets_entity)),
                )
            } else {
                settings
            };

            Element::from(settings)
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
                settings.add(
                    settings::item::builder(panel_info.title.clone())
                        .description(panel_info.description.clone())
                        .control(row!(
                            horizontal_space(Length::Fill),
                            icon("go-next-symbolic", 20).style(theme::Svg::Symbolic)
                        ))
                        .spacing(16)
                        .apply(container)
                        .style(theme::Container::custom(list::column::style))
                        .apply(button)
                        .padding(0)
                        .style(theme::Button::Transparent)
                        .on_press(crate::pages::Message::Page(panel_entity)),
                )
            } else {
                settings
            };

            settings = if let Some((dock_entity, dock_info)) =
                binder.info.iter().find(|(_, v)| v.id == "dock")
            {
                settings.add(
                    settings::item::builder(dock_info.title.clone())
                        .description(dock_info.description.clone())
                        .control(row!(
                            horizontal_space(Length::Fill),
                            icon("go-next-symbolic", 20).style(theme::Svg::Symbolic)
                        ))
                        .spacing(16)
                        .apply(container)
                        .style(theme::Container::custom(list::column::style))
                        .apply(button)
                        .padding(0)
                        .style(theme::Button::Transparent)
                        .on_press(crate::pages::Message::Page(dock_entity)),
                )
            } else {
                settings
            };

            Element::from(settings)
        })
}

pub fn add_panel() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("panel-missing"))
        .descriptions(vec![
            fl!("panel-missing", "desc"),
            fl!("panel-missing", "fix"),
        ])
        .view::<Page>(|binder, _page, section| {
            let desktop = binder
                .page::<super::Page>()
                .expect("desktop page not found");

            let descriptions = &section.descriptions;
            settings::view_section(&section.title)
                .apply(Element::from)
                .map(crate::pages::Message::Desktop)
        })
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Anchor(PanelAnchor);

impl ToString for Anchor {
    fn to_string(&self) -> String {
        match self.0 {
            PanelAnchor::Top => fl!("panel-top"),
            PanelAnchor::Bottom => fl!("panel-bottom"),
            PanelAnchor::Left => fl!("panel-left"),
            PanelAnchor::Right => fl!("panel-right"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Appearance {
    Match,
    Light,
    Dark,
}

impl ToString for Appearance {
    fn to_string(&self) -> String {
        match self {
            Appearance::Match => fl!("panel-appearance", "match"),
            Appearance::Light => fl!("panel-appearance", "light"),
            Appearance::Dark => fl!("panel-appearance", "dark"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    // panel messages
    AutoHidePanel(bool),
    PanelAnchor(PanelAnchor),
    Output(CosmicPanelOuput),
    AnchorGap(bool),
    PanelSize(PanelSize),
    Appearance(Appearance),
    ExtendToEdge(bool),
    Opacity(f64),
    Applets,
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::AutoHidePanel(enabled) => {
                let helper = self.config_helper.as_ref().unwrap();
                let panel_config = self.panel_config.as_mut().unwrap();

                panel_config.autohide = enabled.then_some(AutoHide {
                    wait_time: 1000,
                    transition_time: 200,
                    handle_size: 4,
                });

                let _ = panel_config.write_entry(helper);
            }
            Message::PanelAnchor(anchor) => {
                let helper = self.config_helper.as_ref().unwrap();
                let panel_config = self.panel_config.as_mut().unwrap();

                panel_config.anchor = anchor;

                let _ = panel_config.write_entry(helper);
            }
            Message::Output(_) => todo!(),
            Message::AnchorGap(enabled) => {
                let helper = self.config_helper.as_ref().unwrap();
                let panel_config = self.panel_config.as_mut().unwrap();

                panel_config.anchor_gap = enabled;

                let _ = panel_config.write_entry(helper);
            }
            Message::PanelSize(_) => todo!(),
            Message::Appearance(_) => {
                //TODO update panel config to support these kinds of configs
            }
            Message::ExtendToEdge(enabled) => {
                let helper = self.config_helper.as_ref().unwrap();
                let panel_config = self.panel_config.as_mut().unwrap();

                panel_config.expand_to_edges = enabled;

                let _ = panel_config.write_entry(helper);
            }
            Message::Opacity(_) => todo!(),
            Message::Applets => todo!(),
        }
    }
}
