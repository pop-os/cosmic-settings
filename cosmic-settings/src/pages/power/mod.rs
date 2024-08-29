mod backend;

use self::backend::{GetCurrentPowerProfile, SetPowerProfile};
use backend::{Battery, PowerProfile};

use chrono::TimeDelta;
use cosmic::iced_widget::row;
use cosmic::widget::{self, column, text};
use cosmic::{widget::settings, Apply};
use cosmic_settings_page::{self as page, section, Section};
use slab::Slab;
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page {
    battery: Battery,
}

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
        Some(vec![
            sections.insert(battery_info()),
            sections.insert(profiles()),
        ])
    }

    fn on_enter(
        &mut self,
        _page: cosmic_settings_page::Entity,
        _sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> cosmic::Command<crate::pages::Message> {
        cosmic::command::future(async move {
            let battery = Battery::update_battery().await;
            Message::UpdateBattery(battery)
        })
        .map(crate::pages::Message::Power)
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    PowerProfileChange(PowerProfile),
    UpdateBattery(Battery),
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
            Message::UpdateBattery(battery) => self.battery = battery,
        };
    }
}

fn battery_info() -> Section<crate::pages::Message> {
    let descriptions = Slab::new();

    Section::default()
        .title(fl!("battery"))
        .descriptions(descriptions)
        .show_while::<Page>(|page| page.battery.is_present)
        .view::<Page>(move |_binder, page, section| {
            let battery_icon = widget::icon::from_name(page.battery.icon_name.clone());
            let battery_percent = widget::text(format!("{}%", page.battery.percent));

            let battery_time =
                widget::text(if page.battery.remaining_duration > TimeDelta::zero() {
                    &page.battery.remaining_time
                } else {
                    ""
                });

            column::with_capacity(2)
                .spacing(8)
                .push(text::heading(&section.title))
                .push(row!(battery_icon, battery_percent, battery_time).spacing(8))
                .into()
        })
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
