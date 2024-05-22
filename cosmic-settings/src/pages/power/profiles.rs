use cosmic::iced::widget;
use cosmic::{widget::settings, Apply};

use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

use super::backend::{self, PowerProfile};

#[derive(Default)]
pub struct Page;

impl Page {
    pub fn update(&mut self, message: Message) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        match message {
            Message::PowerProfileChange(p) => runtime.block_on(backend::set_power_profile(p)),
        };
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(profiles())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("power-profiles", "battery-symbolic")
            .title(fl!("power-profiles"))
            .description(fl!("power-profiles", "desc"))
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    PowerProfileChange(PowerProfile),
}

fn profiles() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("power-profiles"))
        .descriptions(vec![fl!("power", "desc").into()])
        .view::<Page>(|_binder, page, section| {
            let runtime = tokio::runtime::Runtime::new().unwrap();

            let profiles = backend::get_power_profiles();

            let current_profile = runtime.block_on(backend::get_current_power_profile());

            let mut section = settings::view_section(&section.title);

            let mut widgets = Vec::new();

            for profile in profiles {
                let selected = if current_profile == profile {
                    Some(true)
                } else {
                    None
                };

                let widget = widget::Radio::new("", true, selected, |_| {
                    Message::PowerProfileChange(profile.clone())
                });
                let item = settings::item::builder(profile.title())
                    .description(profile.description())
                    .control(widget);
                widgets.push(item);
            }

            for item in widgets {
                section = section.add(item);
            }

            section
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::PowerProfile)
        })
}

impl page::AutoBind<crate::pages::Message> for Page {}
