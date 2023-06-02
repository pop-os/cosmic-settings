use cosmic::{
    cosmic_config::{self, CosmicConfigEntry},
    iced::widget::{button, container, horizontal_space, pick_list, row},
    iced::Length,
    iced_widget::slider,
    sctk::reexports::client::{backend::ObjectId, protocol::wl_output::WlOutput, Proxy},
    theme,
    widget::{icon, list, settings, text, toggler},
    Element,
};

use apply::Apply;
use cosmic_panel_config::{
    AutoHide, CosmicPanelBackground, CosmicPanelConfig, CosmicPanelOuput, PanelAnchor, PanelSize,
};
use cosmic_settings_page::{self as page, section, Section};
use slotmap::SlotMap;
use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

pub mod applets;

pub struct Page {
    config_helper: Option<cosmic_config::Config>,
    panel_config: Option<CosmicPanelConfig>,
    // TODO move these into panel config
    pub outputs: HashMap<ObjectId, (String, WlOutput)>,
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
            outputs: HashMap::new(),
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
            let Some(panel_config) = page.panel_config.as_ref() else {
                return Element::from(text(fl!("unknown")));
            };
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
                .add(settings::item(
                    &descriptions[2],
                    pick_list(
                        Cow::from(
                            Some(fl!("all"))
                                .into_iter()
                                .chain(page.outputs.values().map(|(name, _)| name.clone()))
                                .collect::<Vec<_>>(),
                        ),
                        match &panel_config.output {
                            CosmicPanelOuput::All => Some(fl!("all")),
                            CosmicPanelOuput::Active => None,
                            CosmicPanelOuput::Name(ref name) => Some(name.clone()),
                        },
                        Message::Output,
                    ),
                ))
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
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;
            let Some(panel_config) = page.panel_config.as_ref() else {
                return Element::from(text(fl!("unknown")));
            };
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
                        panel_config.background.clone().try_into().ok(),
                        Message::Appearance,
                    ),
                ))
                .add(settings::item(
                    &descriptions[3],
                    // TODO custom discrete slider variant
                    row![
                        text(fl!("small")),
                        slider(
                            0..=4,
                            match panel_config.size {
                                PanelSize::XS => 0,
                                PanelSize::S => 1,
                                PanelSize::M => 2,
                                PanelSize::L => 3,
                                PanelSize::XL => 4,
                            },
                            |v| {
                                if v == 0 {
                                    Message::PanelSize(PanelSize::XS)
                                } else if v == 1 {
                                    Message::PanelSize(PanelSize::S)
                                } else if v == 2 {
                                    Message::PanelSize(PanelSize::M)
                                } else if v == 3 {
                                    Message::PanelSize(PanelSize::L)
                                } else {
                                    Message::PanelSize(PanelSize::XL)
                                }
                            },
                        ),
                        text(fl!("large"))
                    ]
                    .spacing(12),
                ))
                .add(settings::item(
                    &descriptions[4],
                    row![
                        text(fl!("number", HashMap::from_iter(vec![("number", 0)]))),
                        slider(0..=100, (panel_config.opacity * 100.0) as i32, |v| {
                            Message::Opacity(v as f32 / 100.0)
                        },),
                        text(fl!("number", HashMap::from_iter(vec![("number", 100)]))),
                    ]
                    .spacing(12),
                ))
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

#[allow(clippy::module_name_repetitions)]
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

#[allow(clippy::module_name_repetitions)]
pub fn add_panel() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("panel-missing"))
        .descriptions(vec![
            fl!("panel-missing", "desc"),
            fl!("panel-missing", "fix"),
        ])
        .view::<Page>(|_binder, _page, section| {
            // _descriptions = &section.descriptions;
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

impl TryFrom<CosmicPanelBackground> for Appearance {
    type Error = ();
    fn try_from(value: CosmicPanelBackground) -> Result<Self, Self::Error> {
        match value {
            CosmicPanelBackground::ThemeDefault => Ok(Appearance::Match),
            CosmicPanelBackground::Light => Ok(Appearance::Light),
            CosmicPanelBackground::Dark => Ok(Appearance::Dark),
            _ => Err(()),
        }
    }
}

impl From<Appearance> for CosmicPanelBackground {
    fn from(appearance: Appearance) -> Self {
        match appearance {
            Appearance::Match => CosmicPanelBackground::ThemeDefault,
            Appearance::Light => CosmicPanelBackground::Light,
            Appearance::Dark => CosmicPanelBackground::Dark,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    // panel messages
    AutoHidePanel(bool),
    PanelAnchor(PanelAnchor),
    Output(String),
    AnchorGap(bool),
    PanelSize(PanelSize),
    Appearance(Appearance),
    ExtendToEdge(bool),
    Opacity(f32),
    Applets,
    OutputAdded(String, WlOutput),
    OutputRemoved(WlOutput),
    PanelConfig(CosmicPanelConfig),
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::AutoHidePanel(enabled) => {
                let helper = self.config_helper.as_ref().unwrap();
                let mut panel_config = self.panel_config.as_mut().unwrap();

                panel_config.autohide = enabled.then_some(AutoHide {
                    wait_time: 1000,
                    transition_time: 200,
                    handle_size: 4,
                });

                _ = panel_config.write_entry(helper);
            }
            Message::PanelAnchor(anchor) => {
                let helper = self.config_helper.as_ref().unwrap();
                let mut panel_config = self.panel_config.as_mut().unwrap();

                panel_config.anchor = anchor;

                _ = panel_config.write_entry(helper);
            }
            Message::Output(name) => {
                let helper = self.config_helper.as_ref().unwrap();
                let mut panel_config = self.panel_config.as_mut().unwrap();

                panel_config.output = match name {
                    s if s == fl!("all") => CosmicPanelOuput::All,
                    _ => CosmicPanelOuput::Name(name),
                };

                _ = panel_config.write_entry(helper);
            }
            Message::AnchorGap(enabled) => {
                let helper = self.config_helper.as_ref().unwrap();
                let mut panel_config = self.panel_config.as_mut().unwrap();

                panel_config.anchor_gap = enabled;

                _ = panel_config.write_entry(helper);
            }
            Message::PanelSize(size) => {
                let helper = self.config_helper.as_ref().unwrap();
                let mut panel_config = self.panel_config.as_mut().unwrap();

                panel_config.size = size;

                _ = panel_config.write_entry(helper);
            }
            Message::Appearance(a) => {
                let helper = self.config_helper.as_ref().unwrap();
                let mut panel_config = self.panel_config.as_mut().unwrap();

                panel_config.background = a.into();

                _ = panel_config.write_entry(helper);
            }
            Message::ExtendToEdge(enabled) => {
                let helper = self.config_helper.as_ref().unwrap();
                let mut panel_config = self.panel_config.as_mut().unwrap();

                panel_config.expand_to_edges = enabled;

                _ = panel_config.write_entry(helper);
            }
            Message::Opacity(opacity) => {
                let helper = self.config_helper.as_ref().unwrap();
                let mut panel_config = self.panel_config.as_mut().unwrap();

                panel_config.opacity = opacity;

                _ = panel_config.write_entry(helper);
            }
            Message::Applets => todo!(),

            Message::OutputAdded(name, output) => {
                self.outputs.insert(output.id(), (name, output));
            }
            Message::OutputRemoved(output) => {
                self.outputs.remove(&output.id());
            }
            Message::PanelConfig(c) => {
                self.panel_config = Some(c);
            }
        }
    }
}
