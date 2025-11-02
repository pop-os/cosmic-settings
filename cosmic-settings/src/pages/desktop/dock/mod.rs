use std::collections::HashMap;

use cosmic::{Task, cosmic_config::CosmicConfigEntry};
use cosmic_panel_config::{CosmicPanelConfig, CosmicPanelContainerConfig};
use cosmic_settings_page::{self as page, Section, section};
use slotmap::SlotMap;

use crate::pages::desktop::panel::inner::{
    add_panel, behavior_and_position, configuration, enable, reset_button, style,
};

use super::panel::inner::{self, PageInner, PanelPage};

pub mod applets;

pub struct Page {
    inner: PageInner,
}

#[derive(Clone, Debug)]
pub struct Message(pub inner::Message);

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        if let inner::Message::Surface(a) = message.0 {
            cosmic::task::message(crate::app::Message::Surface(a))
        } else {
            self.inner
                .update(message.0)
                .map(Message)
                .map(crate::pages::Message::Dock)
                .map(crate::app::Message::PageMessage)
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

    fn enable_label(&self) -> String {
        fl!("dock")
    }

    fn page_id(&self) -> &'static str {
        "Dock"
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

// TODO cleanup
impl page::Page<crate::pages::Message> for Page {
    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(if self.inner.panel_config.is_some() {
            vec![
                sections.insert(enable::<Page, _>(self, |m| {
                    crate::pages::Message::Dock(Message(m))
                })),
                sections.insert(behavior_and_position::<Page, _>(self, |m| {
                    crate::pages::Message::Dock(Message(m))
                })),
                sections.insert(style::<Page, _>(self, |m| {
                    crate::pages::Message::Dock(Message(m))
                })),
                sections.insert(configuration::<Page>(self)),
                sections.insert(reset_button::<Page, _>(|m| {
                    crate::pages::Message::Dock(Message(m))
                })),
            ]
        } else {
            vec![sections.insert(add_panel::<Page, _>(|m| {
                crate::pages::Message::Dock(Message(m))
            }))]
        })
    }

    fn info(&self) -> page::Info {
        page::Info::new("dock", "preferences-dock-symbolic")
            .title(fl!("dock"))
            .description(fl!("dock", "desc"))
    }
}
