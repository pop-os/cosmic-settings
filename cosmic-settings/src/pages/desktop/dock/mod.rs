use std::collections::HashMap;

use cosmic::Apply;
use cosmic::{
    Element, Task,
    cosmic_config::{ConfigGet, ConfigSet, CosmicConfigEntry},
    iced::Alignment,
    widget::{row, settings, slider, text, toggler},
};
use cosmic_panel_config::{CosmicPanelConfig, CosmicPanelContainerConfig};
use cosmic_settings_page::{self as page, Section, section};
use slab::Slab;
use slotmap::SlotMap;
use tracing::error;

use crate::pages::desktop::panel::inner::{
    add_panel, behavior_and_position, configuration, reset_button, style,
};

use super::panel::inner::{self, PageInner, PanelPage};

pub mod applets;

const DOCK_LENGTH_PERCENT_MIN: i32 = 0;
const DOCK_LENGTH_PERCENT_MAX: i32 = 100;
const DOCK_POSITION_PERCENT_MIN: i32 = 0;
const DOCK_POSITION_PERCENT_MAX: i32 = 100;
const DOCK_CORNER_RADIUS_MIN: i32 = 0;
const DOCK_CORNER_RADIUS_MAX: i32 = 160;

pub struct Page {
    inner: PageInner,
}

#[derive(Clone, Debug)]
pub enum Message {
    EnableDock(bool),
    DockLengthPercent(u16),
    DockPositionPercent(u16),
    DockCornerRadius(u16),
    Inner(inner::Message),
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::EnableDock(enabled) => {
                let Some(container_config) = self.inner.container_config.as_mut() else {
                    return Task::none();
                };

                let Some(panel_config) = self.inner.panel_config.as_ref() else {
                    return Task::none();
                };

                let Ok(helper) = CosmicPanelContainerConfig::cosmic_config() else {
                    return Task::none();
                };

                if enabled {
                    container_config.config_list.push(panel_config.clone());
                } else {
                    container_config
                        .config_list
                        .retain(|c| c.name.as_str() != "Dock");
                }

                let entry_names = container_config
                    .config_list
                    .iter()
                    .map(|c| c.name.clone())
                    .collect::<Vec<_>>();

                if let Err(err) = helper.set("entries", entry_names) {
                    error!("{:?}", err);
                }

                Task::none()
            }
            Message::DockLengthPercent(value) => {
                if self.inner.panel_config.is_none() {
                    return Task::none();
                }
                let Some(helper) = self.inner.config_helper.as_ref() else {
                    return Task::none();
                };
                let value =
                    value.clamp(DOCK_LENGTH_PERCENT_MIN as u16, DOCK_LENGTH_PERCENT_MAX as u16);
                let value = if value == 0 { None } else { Some(value) };
                if let Err(err) = helper.set("dock_length_percent", value) {
                    error!(?err, "Failed to update dock length percent.");
                }
                Task::none()
            }
            Message::DockPositionPercent(value) => {
                if self.inner.panel_config.is_none() {
                    return Task::none();
                }
                let Some(helper) = self.inner.config_helper.as_ref() else {
                    return Task::none();
                };
                let value = value
                    .clamp(DOCK_POSITION_PERCENT_MIN as u16, DOCK_POSITION_PERCENT_MAX as u16);
                if let Err(err) = helper.set("dock_position_percent", Some(value)) {
                    error!(?err, "Failed to update dock position percent.");
                }
                Task::none()
            }
            Message::DockCornerRadius(value) => {
                if self.inner.panel_config.is_none() {
                    return Task::none();
                }
                let Some(helper) = self.inner.config_helper.as_ref() else {
                    return Task::none();
                };
                let value =
                    value.clamp(DOCK_CORNER_RADIUS_MIN as u16, DOCK_CORNER_RADIUS_MAX as u16);
                if let Err(err) = helper.set("dock_corner_radius", Some(value)) {
                    error!(?err, "Failed to update dock corner radius.");
                }
                Task::none()
            }
            Message::Inner(inner) => {
                if let inner::Message::Surface(a) = inner {
                    cosmic::task::message(crate::app::Message::Surface(a))
                } else {
                    self.inner
                        .update(inner)
                        .map(Message::Inner)
                        .map(crate::pages::Message::Dock)
                        .map(crate::app::Message::PageMessage)
                }
            }
        }
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<applets::Page>()
    }
}

impl PanelPage for Page {
    fn inner(&self) -> &PageInner {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut PageInner {
        &mut self.inner
    }

    fn autohide_label(&self) -> String {
        fl!("panel-behavior-and-position", "dock-autohide")
    }

    fn gap_label(&self) -> String {
        fl!("panel-style", "dock-anchor-gap")
    }

    fn extend_label(&self) -> String {
        fl!("panel-style", "dock-extend")
    }

    fn configure_applets_label(&self) -> String {
        fl!("panel-applets", "dock-desc")
    }

    fn applets_page_id(&self) -> &'static str {
        "dock_applets"
    }
}

impl Default for Page {
    fn default() -> Self {
        // TODO CosmicPanelConfig should return its own version
        let config_helper = CosmicPanelConfig::cosmic_config("Dock").ok();

        let panel_config = config_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            // If the config is not present, it will be created with the default values and the name will not match
            (panel_config.name == "Dock").then_some(panel_config)
        });

        let system_default = cosmic::cosmic_config::Config::system(
            &format!("{}.Dock", cosmic_panel_config::NAME),
            CosmicPanelConfig::VERSION,
        )
        .map(|c| match CosmicPanelConfig::get_entry(&c) {
            Ok(c) => c,
            Err((errs, c)) => {
                for err in errs.into_iter().filter(cosmic_config::Error::is_err) {
                    tracing::error!(?err, "Failed to load Dock system config.");
                }
                c
            }
        })
        .ok();

        let container_config = CosmicPanelContainerConfig::load().ok();
        Self {
            inner: PageInner {
                config_helper,
                panel_config,
                container_config,
                outputs_map: HashMap::new(),
                system_default,
                ..Default::default()
            },
        }
    }
}

pub(crate) fn enable() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let dock = descriptions.insert(fl!("dock"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let Some(container_config) = page.inner.container_config.as_ref() else {
                return Element::from(text::body(fl!("unknown")));
            };
            settings::section()
                .title(&section.title)
                .add(settings::item(
                    &descriptions[dock],
                    toggler(
                        container_config
                            .config_list
                            .iter()
                            .any(|e| e.name.as_str() == "Dock"),
                    )
                    .on_toggle(Message::EnableDock),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::Dock)
        })
}

pub(crate) fn dock_overrides() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();
    let dock_length_label = descriptions.insert(format!("{} Width", fl!("dock")));
    let dock_position_label = descriptions.insert(format!("{} Position", fl!("dock")));
    let corner_radius_label = descriptions.insert(format!("{} Corner radius", fl!("dock")));

    Section::default()
        .title(fl!("dock"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let Some(panel_config) = page.inner.panel_config.as_ref() else {
                return Element::from(text::body(fl!("unknown")));
            };

            let dock_length_override = page
                .inner
                .config_helper
                .as_ref()
                .and_then(|helper| helper.get::<Option<u16>>("dock_length_percent").ok())
                .flatten();
            let dock_length = dock_length_override
                .map(|value| value.clamp(0, 100))
                .unwrap_or(0)
                .clamp(DOCK_LENGTH_PERCENT_MIN as u16, DOCK_LENGTH_PERCENT_MAX as u16)
                as i32;
            let dock_position_override = page
                .inner
                .config_helper
                .as_ref()
                .and_then(|helper| helper.get::<Option<u16>>("dock_position_percent").ok())
                .flatten();
            let dock_position = dock_position_override
                .map(|value| value.clamp(0, 100))
                .unwrap_or(50)
                .clamp(DOCK_POSITION_PERCENT_MIN as u16, DOCK_POSITION_PERCENT_MAX as u16)
                as i32;
            let corner_radius_override = page
                .inner
                .config_helper
                .as_ref()
                .and_then(|helper| helper.get::<Option<u16>>("dock_corner_radius").ok())
                .flatten();
            let corner_radius = corner_radius_override
                .map(u32::from)
                .unwrap_or(panel_config.border_radius)
                .clamp(DOCK_CORNER_RADIUS_MIN as u32, DOCK_CORNER_RADIUS_MAX as u32)
                as i32;

            settings::section()
                .title(&section.title)
                .add(settings::flex_item(
                    &descriptions[dock_length_label],
                    row::with_children(vec![
                        text::body(fl!("auto")).into(),
                        slider(
                            DOCK_LENGTH_PERCENT_MIN..=DOCK_LENGTH_PERCENT_MAX,
                            dock_length,
                            |v| Message::DockLengthPercent(v as u16),
                        )
                        .into(),
                        text::body("100%").into(),
                    ])
                    .align_y(Alignment::Center)
                    .spacing(8),
                ))
                .add(settings::flex_item(
                    &descriptions[dock_position_label],
                    row::with_children(vec![
                        text::body(fl!("panel-left")).into(),
                        slider(
                            DOCK_POSITION_PERCENT_MIN..=DOCK_POSITION_PERCENT_MAX,
                            dock_position,
                            |v| Message::DockPositionPercent(v as u16),
                        )
                        .into(),
                        text::body(fl!("panel-right")).into(),
                    ])
                    .align_y(Alignment::Center)
                    .spacing(8),
                ))
                .add(settings::flex_item(
                    &descriptions[corner_radius_label],
                    row::with_children(vec![
                        text::body(fl!("style", "square")).into(),
                        slider(
                            DOCK_CORNER_RADIUS_MIN..=DOCK_CORNER_RADIUS_MAX,
                            corner_radius,
                            |v| Message::DockCornerRadius(v as u16),
                        )
                        .into(),
                        text::body(fl!("style", "round")).into(),
                    ])
                    .align_y(Alignment::Center)
                    .spacing(8),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::Dock)
        })
}
// TODO cleanup
impl page::Page<crate::pages::Message> for Page {
    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(if self.inner.panel_config.is_some() {
            vec![
                sections.insert(enable()),
                sections.insert(behavior_and_position::<Page, _>(self, |m| {
                    crate::pages::Message::Dock(Message::Inner(m))
                })),
                sections.insert(style::<Page, _>(self, |m| {
                    crate::pages::Message::Dock(Message::Inner(m))
                })),
                sections.insert(dock_overrides()),
                sections.insert(configuration::<Page>(self)),
                sections.insert(reset_button::<Page, _>(|m| {
                    crate::pages::Message::Dock(Message::Inner(m))
                })),
            ]
        } else {
            vec![sections.insert(add_panel::<Page, _>(|m| {
                crate::pages::Message::Dock(Message::Inner(m))
            }))]
        })
    }

    fn info(&self) -> page::Info {
        page::Info::new("dock", "preferences-dock-symbolic")
            .title(fl!("dock"))
            .description(fl!("dock", "desc"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        self.inner.update_defaults();

        Task::none()
    }
}
