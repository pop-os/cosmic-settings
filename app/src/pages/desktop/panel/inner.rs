use cosmic::{
    cosmic_config::{self, CosmicConfigEntry},
    iced::Length,
    iced_widget::slider,
    sctk::reexports::client::{backend::ObjectId, protocol::wl_output::WlOutput, Proxy},
    theme,
    widget::{
        button, container, horizontal_space, icon, list, pick_list, row, settings, text, toggler,
    },
    Element,
};

use apply::Apply;
use cosmic_panel_config::{
    AutoHide, CosmicPanelBackground, CosmicPanelConfig, CosmicPanelContainerConfig,
    CosmicPanelOuput, PanelAnchor, PanelSize,
};
use cosmic_settings_page::{self as page, Section};
use std::{borrow::Cow, collections::HashMap};

pub struct PageInner {
    pub(crate) config_helper: Option<cosmic_config::Config>,
    pub(crate) panel_config: Option<CosmicPanelConfig>,
    pub(crate) container_config: Option<CosmicPanelContainerConfig>,
    // TODO move these into panel config
    pub(crate) outputs: HashMap<ObjectId, (String, WlOutput)>,
}

pub trait PanelPage {
    fn inner(&self) -> &PageInner;

    fn inner_mut(&mut self) -> &mut PageInner;

    fn autohide_label(&self) -> String;

    fn gap_label(&self) -> String;

    fn extend_label(&self) -> String;

    fn configure_applets_label(&self) -> String;

    fn applets_page_id(&self) -> &'static str;
}

pub(crate) fn behavior_and_position<
    P: page::Page<crate::pages::Message> + PanelPage,
    T: Fn(Message) -> crate::pages::Message + Copy + 'static,
>(
    p: &P,
    msg_map: T,
) -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("panel-behavior-and-position"))
        .descriptions(vec![
            p.autohide_label(),
            fl!("panel-behavior-and-position", "position"),
            fl!("panel-behavior-and-position", "display"),
        ])
        .view::<P>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let page = page.inner();
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
                .map(msg_map)
        })
}

pub(crate) fn style<
    P: page::Page<crate::pages::Message> + PanelPage,
    T: Fn(Message) -> crate::pages::Message + Copy + 'static,
>(
    p: &P,
    msg_map: T,
) -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("panel-style"))
        .descriptions(vec![
            p.gap_label(),
            p.extend_label(),
            fl!("panel-style", "appearance"),
            fl!("panel-style", "size"),
            fl!("panel-style", "background-opacity"),
        ])
        .view::<P>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let inner = page.inner();
            let Some(panel_config) = inner.panel_config.as_ref() else {
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
                    row::with_children(vec![
                        text(fl!("small")).into(),
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
                        )
                        .into(),
                        text(fl!("large")).into(),
                    ])
                    .spacing(12),
                ))
                .add(settings::item(
                    &descriptions[4],
                    row::with_children(vec![
                        text(fl!("number", HashMap::from_iter(vec![("number", 0)]))).into(),
                        slider(0..=100, (panel_config.opacity * 100.0) as i32, |v| {
                            Message::Opacity(v as f32 / 100.0)
                        })
                        .into(),
                        text(fl!("number", HashMap::from_iter(vec![("number", 100)]))).into(),
                    ])
                    .spacing(12),
                ))
                .apply(Element::from)
                .map(msg_map)
        })
}

pub(crate) fn configuration<P: page::Page<crate::pages::Message> + PanelPage>(
    p: &P,
) -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("panel-applets"))
        .descriptions(vec![p.configure_applets_label()])
        .view::<P>(move |binder, page, section| {
            let mut settings = settings::view_section(&section.title);
            let descriptions = &section.descriptions;
            settings = if let Some((panel_applets_entity, _panel_applets_info)) = binder
                .info
                .iter()
                .find(|(_, v)| v.id == page.applets_page_id())
            {
                let control = row::with_children(vec![
                    horizontal_space(Length::Fill).into(),
                    icon::from_name("go-next-symbolic").size(16).into(),
                ]);

                settings.add(
                    settings::item::builder(&descriptions[0])
                        .control(control)
                        .spacing(16)
                        .apply(container)
                        .style(theme::Container::custom(list::style))
                        .apply(button)
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
pub(crate) fn add_panel<
    P: page::Page<crate::pages::Message> + PanelPage,
    T: Fn(Message) -> crate::pages::Message + Copy + 'static,
>(
    msg_map: T,
) -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("panel-missing"))
        .descriptions(vec![
            fl!("panel-missing", "desc"),
            fl!("panel-missing", "fix"),
        ])
        .view::<P>(move |_binder, page, section| {
            // _descriptions = &section.descriptions;
            settings::view_section(&section.title)
                .apply(Element::from)
                .map(msg_map)
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
    OutputAdded(String, WlOutput),
    OutputRemoved(WlOutput),
    PanelConfig(CosmicPanelConfig),
}

impl PageInner {
    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) {
        let helper = self.config_helper.as_ref().unwrap();
        let Some(mut panel_config) = self.panel_config.as_mut() else {
            return
        };

        match message {
            Message::AutoHidePanel(enabled) => {
                if enabled {
                    panel_config.exclusive_zone = false;
                    panel_config.autohide = Some(AutoHide {
                        wait_time: 1000,
                        transition_time: 200,
                        handle_size: 4,
                    });
                } else {
                    panel_config.exclusive_zone = true;
                    panel_config.autohide = None;
                }
            }
            Message::PanelAnchor(anchor) => {
                panel_config.anchor = anchor;
            }
            Message::Output(name) => {
                panel_config.output = match name {
                    s if s == fl!("all") => CosmicPanelOuput::All,
                    _ => CosmicPanelOuput::Name(name),
                };
            }
            Message::AnchorGap(enabled) => {
                panel_config.anchor_gap = enabled;

                if enabled {
                    panel_config.margin = 4;
                } else {
                    panel_config.margin = 0;
                }
            }
            Message::PanelSize(size) => {
                panel_config.size = size;
            }
            Message::Appearance(a) => {
                panel_config.background = a.into();
            }
            Message::ExtendToEdge(enabled) => {
                panel_config.expand_to_edges = enabled;
            }
            Message::Opacity(opacity) => {
                panel_config.opacity = opacity;
            }
            Message::OutputAdded(name, output) => {
                self.outputs.insert(output.id(), (name, output));
            }
            Message::OutputRemoved(output) => {
                self.outputs.remove(&output.id());
            }
            Message::PanelConfig(c) => {
                self.panel_config = Some(c);
                return;
            }
        }

        if panel_config.anchor_gap || !panel_config.expand_to_edges {
            panel_config.border_radius = 8;
        } else {
            panel_config.border_radius = 0;
        }

        _ = panel_config.write_entry(helper);
    }
}
