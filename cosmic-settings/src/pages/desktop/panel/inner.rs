use cosmic::{
    Element, Task,
    cctk::sctk::reexports::client::{Proxy, backend::ObjectId, protocol::wl_output::WlOutput},
    cosmic_config::{self, CosmicConfigEntry},
    iced::{Alignment, Length},
    surface, theme,
    widget::{
        button, container, dropdown, horizontal_space, icon, row, settings, slider, text, toggler,
    },
};

use cosmic::Apply;
use cosmic_config::ConfigSet;
use cosmic_panel_config::{
    AutoHide, CosmicPanelBackground, CosmicPanelConfig, CosmicPanelContainerConfig,
    CosmicPanelOuput, PanelAnchor, PanelSize,
};
use cosmic_settings_page::{self as page, Section};
use slab::Slab;
use std::{collections::HashMap, time::Duration};

pub struct PageInner {
    pub(crate) config_helper: Option<cosmic_config::Config>,
    pub(crate) panel_config: Option<CosmicPanelConfig>,
    pub opacity: f32,
    pub opacity_changing: bool,
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
            opacity: 0.0,
            opacity_changing: false,
            outputs: vec![fl!("all-displays")],
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
                        for err in errs.into_iter().filter(cosmic_config::Error::is_err) {
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
    T: Fn(Message) -> crate::pages::Message + Copy + Send + Sync + 'static,
>(
    p: &P,
    msg_map: T,
) -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let autohide_label = descriptions.insert(p.autohide_label());
    let position = descriptions.insert(fl!("panel-behavior-and-position", "position"));
    let display = descriptions.insert(fl!("panel-behavior-and-position", "display"));

    Section::default()
        .title(fl!("panel-behavior-and-position"))
        .descriptions(descriptions)
        .view::<P>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let page = page.inner();
            let Some(panel_config) = page.panel_config.as_ref() else {
                return Element::from(text::body(fl!("unknown")));
            };
            settings::section()
                .title(&section.title)
                .add(settings::item(
                    &descriptions[autohide_label],
                    toggler(panel_config.autohide.is_some()).on_toggle(Message::AutoHidePanel),
                ))
                .add(settings::item(
                    &descriptions[position],
                    dropdown::popup_dropdown(
                        page.anchors.as_slice(),
                        Some(panel_config.anchor as usize),
                        Message::PanelAnchor,
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        move |a| crate::app::Message::PageMessage(msg_map(a)),
                    ),
                ))
                .add(settings::item(
                    &descriptions[display],
                    dropdown::popup_dropdown(
                        page.outputs.as_slice(),
                        match &panel_config.output {
                            CosmicPanelOuput::All => Some(0),
                            CosmicPanelOuput::Active => None,
                            CosmicPanelOuput::Name(n) => page.outputs.iter().position(|o| o == n),
                        },
                        Message::Output,
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        move |a| crate::app::Message::PageMessage(msg_map(a)),
                    ),
                ))
                .apply(Element::from)
                .map(msg_map)
        })
}

pub(crate) fn style<
    P: page::Page<crate::pages::Message> + PanelPage,
    T: Fn(Message) -> crate::pages::Message + Copy + Send + Sync + 'static,
>(
    p: &P,
    msg_map: T,
) -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let gap_label = descriptions.insert(p.gap_label());
    let extend_label = descriptions.insert(p.extend_label());
    let appearance = descriptions.insert(fl!("panel-style", "appearance"));
    let background_opacity = descriptions.insert(fl!("panel-style", "background-opacity"));
    let size = descriptions.insert(fl!("panel-style", "size"));

    Section::default()
        .title(fl!("panel-style"))
        .descriptions(descriptions)
        .view::<P>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let inner = page.inner();
            let Some(panel_config) = inner.panel_config.as_ref() else {
                return Element::from(text::body(fl!("unknown")));
            };
            settings::section()
                .title(&section.title)
                .add(settings::item(
                    &descriptions[gap_label],
                    toggler(panel_config.anchor_gap).on_toggle(Message::AnchorGap),
                ))
                .add(settings::item(
                    &descriptions[extend_label],
                    toggler(panel_config.expand_to_edges).on_toggle(Message::ExtendToEdge),
                ))
                .add(settings::item(
                    &descriptions[appearance],
                    dropdown::popup_dropdown(
                        inner.backgrounds.as_slice(),
                        match panel_config.background {
                            CosmicPanelBackground::ThemeDefault => Some(0),
                            CosmicPanelBackground::Light => Some(1),
                            CosmicPanelBackground::Dark => Some(2),
                            CosmicPanelBackground::Color(_) => None,
                        },
                        Message::Appearance,
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        move |a| crate::app::Message::PageMessage(msg_map(a)),
                    ),
                ))
                .add(settings::flex_item(
                    &descriptions[size],
                    // TODO custom discrete slider variant
                    row::with_children(vec![
                        text::body(fl!("small")).into(),
                        slider(
                            0..=4,
                            match panel_config.size {
                                PanelSize::XS => 0,
                                PanelSize::S => 1,
                                PanelSize::M => 2,
                                PanelSize::L => 3,
                                PanelSize::XL => 4,
                                PanelSize::Custom(_) => 2,
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
                        text::body(fl!("large")).into(),
                    ])
                    .align_y(Alignment::Center)
                    .spacing(8),
                ))
                .add(settings::flex_item(
                    &descriptions[background_opacity],
                    row::with_capacity(2)
                        .align_y(Alignment::Center)
                        .spacing(8)
                        .push(
                            text::body(fl!(
                                "number",
                                HashMap::from_iter(vec![(
                                    "number",
                                    (panel_config.opacity * 100.0) as i32
                                )])
                            ))
                            .width(Length::Fixed(22.0))
                            .align_x(Alignment::Center),
                        )
                        .push(
                            slider(0..=100, (panel_config.opacity * 100.0) as i32, |v| {
                                Message::OpacityRequest(v as f32 / 100.0)
                            })
                            .breakpoints(&[50]),
                        ),
                ))
                .apply(Element::from)
                .map(msg_map)
        })
}

pub(crate) fn configuration<P: page::Page<crate::pages::Message> + PanelPage>(
    p: &P,
) -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let applets_label = descriptions.insert(p.configure_applets_label());

    Section::default()
        .title(fl!("panel-applets"))
        .descriptions(descriptions)
        .view::<P>(move |binder, page, section| {
            let mut settings = settings::section().title(&section.title);
            let descriptions = &section.descriptions;
            settings = if let Some((panel_applets_entity, _panel_applets_info)) = binder
                .info
                .iter()
                .find(|(_, v)| v.id == page.applets_page_id())
            {
                let control = row::with_children(vec![
                    horizontal_space().into(),
                    icon::from_name("go-next-symbolic").size(16).into(),
                ]);

                settings.add(
                    settings::item::builder(&*descriptions[applets_label])
                        .control(control)
                        .spacing(16)
                        .apply(container)
                        .class(theme::Container::List)
                        .apply(button::custom)
                        .class(theme::Button::Transparent)
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
    let mut descriptions = Slab::new();

    let reset_to_default = descriptions.insert(fl!("reset-to-default"));

    Section::default()
        .title(fl!("panel-missing"))
        .descriptions(descriptions)
        .view::<P>(move |_binder, _page, section| {
            let descriptions = &section.descriptions;
            button::standard(&descriptions[reset_to_default])
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
    let mut descriptions = Slab::new();

    let reset_to_default = descriptions.insert(fl!("reset-to-default"));

    Section::default()
        .descriptions(descriptions)
        .view::<P>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let inner = page.inner();
            if inner.system_default == inner.panel_config {
                Element::from(horizontal_space().width(1))
            } else {
                button::standard(&descriptions[reset_to_default])
                    .on_press(Message::ResetPanel)
                    .into()
            }
            .map(msg_map)
        })
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Anchor(PanelAnchor);

impl std::fmt::Display for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                PanelAnchor::Top => fl!("panel-top"),
                PanelAnchor::Bottom => fl!("panel-bottom"),
                PanelAnchor::Left => fl!("panel-left"),
                PanelAnchor::Right => fl!("panel-right"),
            }
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Appearance {
    Match,
    Light,
    Dark,
}

impl std::fmt::Display for Appearance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Appearance::Match => fl!("panel-appearance", "match"),
                Appearance::Light => fl!("panel-appearance", "light"),
                Appearance::Dark => fl!("panel-appearance", "dark"),
            }
        )
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
    OpacityRequest(f32),
    OpacityApply,
    OutputAdded(String, WlOutput),
    OutputRemoved(WlOutput),
    PanelConfig(CosmicPanelConfig),
    ResetPanel,
    FullReset,
    Surface(surface::Action),
}

impl PageInner {
    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let Some(helper) = self.config_helper.as_ref() else {
            return Task::none();
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
                    self.panel_config.clone_from(&self.system_default);
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
            return Task::none();
        };

        match message {
            Message::AutoHidePanel(enabled) => {
                if enabled {
                    _ = panel_config.set_exclusive_zone(helper, false);
                    _ = panel_config.set_autohide(
                        helper,
                        Some(AutoHide {
                            wait_time: 1000,
                            transition_time: 200,
                            handle_size: 4,
                        }),
                    );
                } else {
                    _ = panel_config.set_exclusive_zone(helper, true);
                    _ = panel_config.set_autohide(helper, None);
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
                    _ = panel_config.set_anchor(helper, *anchor);
                }
            }
            Message::Output(i) => {
                if i == 0 {
                    _ = panel_config.set_output(helper, CosmicPanelOuput::All);
                } else {
                    _ = panel_config
                        .set_output(helper, CosmicPanelOuput::Name(self.outputs[i].clone()));
                }
            }
            Message::AnchorGap(enabled) => {
                _ = panel_config.set_anchor_gap(helper, enabled);

                if enabled {
                    _ = panel_config.set_margin(helper, 4);
                } else {
                    _ = panel_config.set_margin(helper, 0);
                }
            }
            Message::PanelSize(size) => {
                _ = panel_config.set_size(helper, size);
                // Reset any size overrides the user might have set
                _ = panel_config.set_size_center(helper, None);
                _ = panel_config.set_size_wings(helper, None);
            }
            Message::Appearance(a) => {
                if let Some(b) = [Appearance::Match, Appearance::Light, Appearance::Dark]
                    .iter()
                    .find(|b| b.to_string() == self.backgrounds[a])
                {
                    _ = panel_config.set_background(helper, (*b).into());
                }
            }
            Message::ExtendToEdge(enabled) => {
                _ = panel_config.set_expand_to_edges(helper, enabled);
            }
            Message::OpacityRequest(opacity) => {
                panel_config.opacity = opacity;

                if self.opacity_changing {
                    return Task::none();
                }

                self.opacity_changing = true;
                return cosmic::Task::future(async move {
                    tokio::time::sleep(Duration::from_millis(125)).await;
                    Message::OpacityApply
                });
            }

            Message::OpacityApply => {
                self.opacity_changing = false;
                _ = helper.set("opacity", panel_config.opacity);
            }

            Message::OutputAdded(name, output) => {
                self.outputs.push(name.clone());
                self.outputs_map.insert(output.id(), (name, output));
                return Task::none();
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
                return Task::none();
            }
            Message::ResetPanel | Message::FullReset => {}
            Message::Surface(_) => {
                unimplemented!()
            }
        }

        if panel_config.anchor_gap || !panel_config.expand_to_edges {
            let radii = cosmic::theme::system_preference()
                .cosmic()
                .corner_radii
                .radius_xl[0] as u32;
            _ = panel_config.set_border_radius(helper, radii);
        } else {
            _ = panel_config.set_border_radius(helper, 0);
        }

        Task::none()
    }
}
