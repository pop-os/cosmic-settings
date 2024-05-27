use backend::PowerProfile;
use cosmic::iced::widget;
use cosmic::{widget::settings, Apply};
use cosmic_settings_page::{self as page, section, Section};
use slotmap::SlotMap;

use self::backend::{GetCurrentPowerProfile, SetPowerProfile};

mod backend;

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("power", "battery-symbolic")
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
            },
        };
    }
}

fn profiles() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("power-profiles"))
        .descriptions(vec![fl!("power", "desc").into()])
        .view::<Page>(|_binder, page, section| {
            
            let mut section = settings::view_section(&section.title);

            let runtime = tokio::runtime::Runtime::new().unwrap();

            let backend = runtime.block_on(backend::get_backend());

           if let Some(b) = backend {
                    let profiles = backend::get_power_profiles();

                    let current_profile = runtime.block_on(b.get_current_power_profile());
        
        
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
            }
            else {
                let item = widget::Text::new(fl!("power-profiles", "nobackend"));
                section = section.add(item);
            }

            section
            .apply(cosmic::Element::from)
            .map(crate::pages::Message::Power)
        })
}


impl page::AutoBind<crate::pages::Message> for Page {}
