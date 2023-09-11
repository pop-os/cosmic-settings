use std::collections::HashMap;

use cosmic::cosmic_config::CosmicConfigEntry;
use cosmic_panel_config::{CosmicPanelConfig, CosmicPanelContainerConfig};
use cosmic_settings_page::{self as page, section, Section};
use slotmap::SlotMap;

use crate::pages::desktop::panel::inner::{add_panel, behavior_and_position, configuration, style};

use self::inner::{PageInner, PanelPage};

pub mod applets_inner;
pub mod inner;

pub struct Page {
    inner: PageInner,
}

#[derive(Clone, Debug)]
pub struct Message(pub inner::Message);

impl Page {
    pub fn update(&mut self, message: Message) {
        self.inner.update(message.0);
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<applets_inner::Page>()
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
        fl!("panel-behavior-and-position", "autohide")
    }

    fn gap_label(&self) -> String {
        fl!("panel-style", "anchor-gap")
    }

    fn extend_label(&self) -> String {
        fl!("panel-style", "extend")
    }

    fn configure_applets_label(&self) -> String {
        fl!("panel-applets", "desc")
    }

    fn applets_page_id(&self) -> &'static str {
        "panel_applets"
    }
}

impl Default for Page {
    fn default() -> Self {
        let config_helper = CosmicPanelConfig::cosmic_config("Panel").ok();
        let panel_config = config_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            // If the config is not present, it will be created with the default values and the name will not match
            (panel_config.name == "Panel").then_some(panel_config)
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

// TODO cleanup
impl page::Page<crate::pages::Message> for Page {
    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(if self.inner.panel_config.is_some() {
            vec![
                sections.insert(behavior_and_position::<Page, _>(self, |m| {
                    crate::pages::Message::Panel(Message(m))
                })),
                sections.insert(style::<Page, _>(self, |m| {
                    crate::pages::Message::Panel(Message(m))
                })),
                sections.insert(configuration::<Page>(self)),
            ]
        } else {
            vec![sections.insert(add_panel::<Page, _>(|m| {
                crate::pages::Message::Panel(Message(m))
            }))]
        })
    }

    fn info(&self) -> page::Info {
        page::Info::new("panel", "preferences-pop-desktop-dock-symbolic")
            .title(fl!("panel"))
            .description(fl!("panel", "desc"))
    }
}
