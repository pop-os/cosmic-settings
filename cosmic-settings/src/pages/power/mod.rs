mod backend;
use self::backend::{GetCurrentPowerProfile, SetPowerProfile};
use backend::PowerProfile;

use cosmic::widget;
use cosmic::{widget::settings, Apply};
use cosmic_settings_page::{self as page, section, Section};
use slab::Slab;
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("power", "preferences-power-and-battery-symbolic")
            .title(fl!("power"))
            .description(fl!("power", "desc"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(profiles())])
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    PowerProfileChange(PowerProfile),
}

impl Page {
    pub fn update(&mut self, message: Message) {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let backend = runtime.block_on(backend::get_backend());

        match message {
            Message::PowerProfileChange(p) => {
                if let Some(b) = backend {
                    runtime.block_on(b.set_power_profile(p));
                }
            }
        };
    }
}

fn profiles() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let _power_desc = descriptions.insert(fl!("power", "desc"));

    Section::default()
        .title(fl!("power-mode"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            let mut section = settings::view_section(&section.title);

            let runtime = tokio::runtime::Runtime::new().unwrap();

            let backend = runtime.block_on(backend::get_backend());

            if let Some(b) = backend {
                let profiles = backend::get_power_profiles();

                let current_profile = runtime.block_on(b.get_current_power_profile());

                section = profiles
                    .into_iter()
                    .map(|profile| {
                        let selected = if current_profile == profile {
                            Some(true)
                        } else {
                            None
                        };

                        let widget = widget::Radio::new("", true, selected, |_| {
                            Message::PowerProfileChange(profile.clone())
                        });

                        settings::item::builder(profile.title())
                            .description(profile.description())
                            .control(widget)
                    })
                    .fold(section, settings::Section::add);
            } else {
                let item = widget::text::body(fl!("power-mode", "nobackend"));
                section = section.add(item);
            }

            section
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Power)
        })
}

impl page::AutoBind<crate::pages::Message> for Page {}
