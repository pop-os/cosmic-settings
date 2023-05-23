use cosmic::{
    iced::widget::horizontal_space,
    iced::Length,
    widget::{settings, toggler},
    Element,
};

use apply::Apply;
use cosmic_config::CosmicConfigEntry;
use cosmic_panel_config::{AutoHide, CosmicPanelConfig, CosmicPanelOuput, PanelAnchor, PanelSize};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

pub struct Page {
    config_helper: Option<cosmic_config::Config>,
    panel_config: Option<CosmicPanelConfig>,
}

impl Default for Page {
    fn default() -> Self {
        // TODO CosmicPanelConfig should return its own version
        let config_helper = cosmic_config::Config::new("com.system76.CosmicPanel.panel", 1).ok();
        let panel_config = config_helper.as_ref().and_then(|config_helper| {
            // TODO error handling...
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            (panel_config.name == "panel").then_some(panel_config)
        });

        // If the config is not present, it will be created with the default values and the name will not match
        Self {
            config_helper,
            panel_config,
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

impl page::AutoBind<crate::pages::Message> for Page {}

pub fn behavior_and_position() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("panel-behavior-and-position"))
        .descriptions(vec![fl!("panel-behavior-and-position", "autohide")])
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
                .apply(Element::from)
                .map(crate::pages::Message::Panel)
        })
}

pub fn style() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("hot-corner"))
        .descriptions(vec![fl!("hot-corner", "top-left-corner")])
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

pub fn configuration() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("hot-corner"))
        .descriptions(vec![fl!("hot-corner", "top-left-corner")])
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

pub fn add_panel() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("hot-corner"))
        .descriptions(vec![fl!("hot-corner", "top-left-corner")])
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

#[derive(Clone, Debug)]
pub enum Message {
    // panel messages
    AutoHidePanel(bool),
    PanelAnchor(PanelAnchor),
    Output(CosmicPanelOuput),
    AnchorGap(bool),
    PanelSize(PanelSize),
    Appearance,
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
            Message::PanelAnchor(_) => todo!(),
            Message::Output(_) => todo!(),
            Message::AnchorGap(_) => todo!(),
            Message::PanelSize(_) => todo!(),
            Message::Appearance => todo!(),
            Message::ExtendToEdge(_) => todo!(),
            Message::Opacity(_) => todo!(),
            Message::Applets => todo!(),
        }
    }
}
