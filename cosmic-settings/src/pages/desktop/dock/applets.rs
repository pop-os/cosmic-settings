use cosmic::{
    cosmic_config::CosmicConfigEntry,
    iced::{alignment, Length},
    widget::{button, container, row},
    Apply, Element, Task,
};
use cosmic_panel_config::CosmicPanelConfig;
use cosmic_settings_page::{self as page, section, Section};
use slotmap::SlotMap;
use std::borrow::Cow;

use crate::{
    app,
    pages::{
        self,
        desktop::panel::applets_inner::{self, lists, AppletsPage, ContextDrawer},
    },
};

pub(crate) struct Page {
    inner: applets_inner::Page,
}

impl Default for Page {
    fn default() -> Self {
        let config_helper = CosmicPanelConfig::cosmic_config("Dock").ok();
        let current_config = config_helper.as_ref().and_then(|config_helper| {
            let panel_config = CosmicPanelConfig::get_entry(config_helper).ok()?;
            // If the config is not present, it will be created with the default values and the name will not match
            (panel_config.name == "Dock").then_some(panel_config)
        });
        Self {
            inner: applets_inner::Page {
                available_entries: freedesktop_desktop_entry::Iter::new(
                    freedesktop_desktop_entry::default_paths(),
                )
                .filter_map(|p| applets_inner::Applet::try_from(Cow::from(p)).ok())
                .collect(),
                config_helper,
                current_config,
                reorder_widget_state: None,
                search: String::new(),
                context: None,
            },
        }
    }
}

impl AppletsPage for Page {
    fn inner(&self) -> &applets_inner::Page {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut applets_inner::Page {
        &mut self.inner
    }
}

#[derive(Debug, Clone)]
pub struct Message(pub applets_inner::Message);

impl Page {
    pub fn update(&mut self, message: Message) -> Task<app::Message> {
        self.inner.update(message.0)
    }
}

impl page::Page<crate::pages::Message> for Page {
    #[allow(clippy::too_many_lines)]
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(lists::<Page, _>(|msg| {
            pages::Message::DockApplet(Message(msg))
        }))])
    }

    fn info(&self) -> page::Info {
        page::Info::new("dock_applets", "preferences-dock-symbolic").title(fl!("applets"))
    }

    fn header_view(&self) -> Option<Element<'_, crate::pages::Message>> {
        let theme = cosmic::theme::active();
        let spacing = theme.cosmic().spacing;
        let content = row::with_capacity(2)
            .spacing(spacing.space_xxs)
            .push(
                button::standard(fl!("add-applet"))
                    .on_press(Message(applets_inner::Message::AddAppletDrawer)),
            )
            .apply(container)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .apply(Element::from)
            .map(crate::pages::Message::DockApplet);

        Some(content)
    }

    fn context_drawer(&self) -> Option<Element<crate::pages::Message>> {
        Some(match self.inner.context {
            Some(ContextDrawer::AddApplet) => self
                .inner
                .add_applet_view(|msg| crate::pages::Message::DockApplet(Message(msg))),

            None => return None,
        })
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}
