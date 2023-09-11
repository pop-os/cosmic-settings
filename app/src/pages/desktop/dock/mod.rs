use std::collections::HashMap;

use apply::Apply;
use cosmic::{
    cosmic_config::CosmicConfigEntry,
    widget::{settings, text, toggler},
    Element,
};
use cosmic_panel_config::{CosmicPanelConfig, CosmicPanelContainerConfig};
use cosmic_settings_page::{self as page, section, Section};
use slotmap::SlotMap;

use crate::pages::desktop::panel::inner::{add_panel, behavior_and_position, configuration, style};

use super::panel::inner::{self, PageInner, PanelPage};

pub mod applets;

pub struct Page {
    inner: PageInner,
}

#[derive(Clone, Debug)]
pub enum Message {
    EnableDock(bool),
    Inner(inner::Message),
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::EnableDock(enabled) => {
                let Some(container_config) = self.inner.container_config.as_mut() else {
                    return;
                };
                let Some(panel_config) = self.inner.panel_config.as_ref() else {
                    return;
                };
                if enabled {
                    container_config.config_list.push(panel_config.clone());
                } else {
                    container_config
                        .config_list
                        .retain(|c| c.name.as_str() != "Dock");
                }
                _ = container_config.write_entries();
            }
            Message::Inner(inner) => {
                self.inner.update(inner);
            }
        };
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
        let container_config = CosmicPanelContainerConfig::load().ok();
        Self {
            inner: PageInner {
                config_helper,
                panel_config,
                container_config,
                outputs: HashMap::new(),
            },
        }
    }
}

pub(crate) fn enable() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![fl!("dock")])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;
            let Some(container_config) = page.inner.container_config.as_ref() else {
                return Element::from(text(fl!("unknown")));
            };
            settings::view_section(&section.title)
                .add(settings::item(
                    &descriptions[0],
                    toggler(
                        None,
                        container_config
                            .config_list
                            .iter()
                            .any(|e| e.name.as_str() == "Dock"),
                        Message::EnableDock,
                    ),
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
                sections.insert(configuration::<Page>(self)),
            ]
        } else {
            vec![sections.insert(add_panel::<Page, _>(|m| {
                crate::pages::Message::Dock(Message::Inner(m))
            }))]
        })
    }

    fn info(&self) -> page::Info {
        page::Info::new("dock", "preferences-pop-desktop-dock-symbolic")
            .title(fl!("dock"))
            .description(fl!("dock", "desc"))
    }
}
