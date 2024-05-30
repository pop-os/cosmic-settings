use cosmic::{
    cctk::sctk::reexports::client::{backend::ObjectId, protocol::wl_output::WlOutput, Proxy},
    cosmic_config::{self, CosmicConfigEntry},
    iced::Length,
    theme,
    widget::{
        button, container, dropdown, horizontal_space, icon, row, settings, slider, text, toggler,
    },
    Element,
};

use cosmic::Apply;
use cosmic_panel_config::{
    AutoHide, CosmicPanelBackground, CosmicPanelConfig, CosmicPanelContainerConfig,
    CosmicPanelOuput, PanelAnchor, PanelSize,
};
use cosmic_settings_page::{self as page, Section};
use std::collections::HashMap;

pub struct PageInner {
    pub(crate) config_helper: Option<cosmic_config::Config>,
    pub(crate) panel_config: Option<CosmicPanelConfig>,
    pub outputs: Vec<String>,
    pub anchors: Vec<String>,
    pub backgrounds: Vec<String>,
    pub(crate) container_config: Option<CosmicPanelContainerConfig>,
    // TODO move these into panel config
    pub(crate) outputs_map: HashMap<ObjectId, (String, WlOutput)>,
    pub(crate) system_default: Option<CosmicPanelConfig>,
    pub(crate) system_container: Option<CosmicPanelContainerConfig>,
}

impl Default for PageInner {
    fn default() -> Self {
        Self {
            config_helper: Option::default(),
            panel_config: Option::default(),
            outputs: vec![fl!("all")],
            anchors: vec![
                Anchor(PanelAnchor::Left).to_string(),
                Anchor(PanelAnchor::Right).to_string(),
                Anchor(PanelAnchor::Top).to_string(),
                Anchor(PanelAnchor::Bottom).to_string(),
            ],
            backgrounds: vec![
                Appearance::Match.to_string(),
                Appearance::Light.to_string(),
                Appearance::Dark.to_string(),
            ],
            container_config: Option::default(),
            outputs_map: HashMap::default(),
            system_default: None,
            system_container: cosmic::cosmic_config::Config::system(
                cosmic_panel_config::NAME,
                CosmicPanelConfig::VERSION,
            )
            .map(
                |c| match CosmicPanelContainerConfig::load_from_config(&c, true) {
                    Ok(c) => c,
                    Err((errs, c)) => {
                        for err in errs {
                            tracing::error!(?err, "Error when loading Panel container config.");
                        }
                        c
                    }
                },
            )
            .ok(),
        }
    }
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
            p.autohide_label().into(),
            fl!("panel-behavior-and-position", "position").into(),
            fl!("panel-behavior-and-position", "display").into(),
        ])
        .view::<P>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let page = page.inner();
            let Some(panel_config) = page.panel_config.as_ref() else {
                return Element::from(text(fl!("unknown")));
            };
            settings::view_section(&section.title)
                .add(settings::flex_item(
                    &*descriptions[0],
                    toggler(None, panel_config.autohide.is_some(), |value| {
                        Message::AutoHidePanel(value)
                    }),
                ))
                .add(settings::flex_item(
                    &*descriptions[1],
                    dropdown(
                        page.anchors.as_slice(),
                        Some(panel_config.anchor as usize),
                        Message::PanelAnchor,
                    ),
                ))
                .add(settings::flex_item(
                    &*descriptions[2],
                    dropdown(
                        page.outputs.as_slice(),
                        match &panel_config.output {
                            CosmicPanelOuput::All => Some(0),
                            CosmicPanelOuput::Active => None,
                            CosmicPanelOuput::Name(n) => page.outputs.iter().position(|o| o == n),
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
            p.gap_label().into(),
            p.extend_label().into(),
            fl!("panel-style", "appearance").into(),
            fl!("panel-style", "size").into(),
            fl!("panel-style", "background-opacity").into(),
        ])
        .view::<P>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let inner = page.inner();
            let Some(panel_config) = inner.panel_config.as_ref() else {
                return Element::from(text(fl!("unknown")));
            };
            settings::view_section(&section.title)
                .add(settings::flex_item(
                    &*descriptions[0],
                    toggler(None, panel_config.anchor_gap, |value| {
                        Message::AnchorGap(value)
                    }),
                ))
                .add(settings::flex_item(
                    &*descriptions[1],
                    toggler(None, panel_config.expand_to_edges, |value| {
                        Message::ExtendToEdge(value)
                    }),
                ))
                .add(settings::flex_item(
                    &*descriptions[2],
                    dropdown(
                        inner.backgrounds.as_slice(),
                        match panel_config.background {
                            CosmicPanelBackground::ThemeDefault => Some(0),
                            CosmicPanelBackground::Light => Some(1),
                            CosmicPanelBackground::Dark => Some(2),
                            CosmicPanelBackground::Color(_) => None,
                        },
                        Message::Appearance,
                    ),
                ))
                .add(settings::flex_item(
                    &*descriptions[3],
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
                .add(settings::flex_item(
                    &*descriptions[4],
                    row::with_children(vec![
                        text(fl!("number", HashMap::from_iter(vec![("number", 0)]))).into(),
                        slider(0..=100, (panel_config.opacity * 100.0) as i32, |v| {
                            Message::Opacity(v as f32 / 100.0)
                        })
                        .breakpoints(&[50])
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
        .descriptions(vec![p.configure_applets_label().into()])
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
                    settings::item::builder(&*descriptions[0])
                        .control(control)
                        .spacing(16)
                        .apply(container)
                        .style(theme::Container::List)
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
        .descriptions(vec![fl!("reset-to-default").into()])
        .view::<P>(move |_binder, _page, section| {
            let descriptions = &section.descriptions;
            button::standard(&*descriptions[0])
                .on_press(Message::FullReset)
                .apply(Element::from)
                .map(msg_map)
        })
}

#[allow(clippy::too_many_lines)]
pub fn reset_button<
    P: page::Page<crate::pages::Message> + PanelPage,
    T: Fn(Message) -> crate::pages::Message + Copy + 'static,
>(
    msg_map: T,
) -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![fl!("reset-to-default").into()])
        .view::<P>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let inner = page.inner();
            if inner.system_default == inner.panel_config {
                Element::from(horizontal_space(1))
            } else {
                button::standard(&*descriptions[0])
                    .on_press(Message::ResetPanel)
                    .into()
            }
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
    PanelAnchor(usize),
    Output(usize),
    AnchorGap(bool),
    PanelSize(PanelSize),
    Appearance(usize),
    ExtendToEdge(bool),
    Opacity(f32),
    OutputAdded(String, WlOutput),
    OutputRemoved(WlOutput),
    PanelConfig(CosmicPanelConfig),
    ResetPanel,
    FullReset,
}

impl PageInner {
    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) {
        let Some(helper) = self.config_helper.as_ref() else {
            return;
        };
        match &message {
            Message::ResetPanel => {
                if let Some((default, config)) = self
                    .system_default
                    .as_mut()
                    .zip(self.config_helper.as_ref())
                {
                    if default.anchor_gap || !default.expand_to_edges {
                        let radii = cosmic::theme::system_preference()
                            .cosmic()
                            .corner_radii
                            .radius_xl[0] as u32;
                        default.border_radius = radii;
                    } else {
                        default.border_radius = 0;
                    }
                    if let Err(err) = default.write_entry(config) {
                        tracing::error!(?err, "Error resetting panel config.");
                    }
                    self.panel_config = self.system_default.clone();
                } else {
                    tracing::error!("Panel config default is missing.");
                }
            }
            Message::FullReset => {
                if let Some(container) = self.system_container.as_ref() {
                    if let Err(err) = container.write_entries() {
                        tracing::error!(?err, "Error fully resetting the panel config.");
                    }
                }
            }
            _ => {}
        };

        let Some(panel_config) = self.panel_config.as_mut() else {
            return;
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
            Message::PanelAnchor(i) => {
                if let Some(anchor) = [
                    PanelAnchor::Left,
                    PanelAnchor::Right,
                    PanelAnchor::Top,
                    PanelAnchor::Bottom,
                ]
                .iter()
                .find(|a| Anchor(**a).to_string() == self.anchors[i])
                {
                    panel_config.anchor = *anchor;
                }
            }
            Message::Output(i) => {
                if i == 0 {
                    panel_config.output = CosmicPanelOuput::All;
                } else {
                    panel_config.output = CosmicPanelOuput::Name(self.outputs[i].clone());
                }
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
                if let Some(b) = [Appearance::Match, Appearance::Light, Appearance::Dark]
                    .iter()
                    .find(|b| b.to_string() == self.backgrounds[a])
                {
                    panel_config.background = (*b).into();
                }
            }
            Message::ExtendToEdge(enabled) => {
                panel_config.expand_to_edges = enabled;
            }
            Message::Opacity(opacity) => {
                panel_config.opacity = opacity;
            }
            Message::OutputAdded(name, output) => {
                self.outputs.push(name.clone());
                self.outputs_map.insert(output.id(), (name, output));
                return;
            }
            Message::OutputRemoved(output) => {
                if let Some((name, _)) = self.outputs_map.remove(&output.id()) {
                    if let Some(pos) = self.outputs.iter().position(|o| o == &name) {
                        self.outputs.remove(pos);
                    }
                }
            }
            Message::PanelConfig(c) => {
                self.panel_config = Some(c);
                return;
            }
            Message::ResetPanel | Message::FullReset => {}
        }

        if panel_config.anchor_gap || !panel_config.expand_to_edges {
            let radii = cosmic::theme::system_preference()
                .cosmic()
                .corner_radii
                .radius_xl[0] as u32;
            panel_config.border_radius = radii;
        } else {
            panel_config.border_radius = 0;
        }

        _ = panel_config.write_entry(helper);
    }
}
