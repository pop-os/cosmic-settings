mod backend;

use self::backend::{GetCurrentPowerProfile, SetPowerProfile};
use backend::{Battery, ConnectedDevice, PowerProfile};

use chrono::TimeDelta;
use cosmic::iced::{Alignment, Length};
use cosmic::iced_widget::{column, row};
use cosmic::prelude::CollectionWidget;
use cosmic::widget::{self, radio, settings, text};
use cosmic::Apply;
use cosmic::Command;
use cosmic_settings_page::{self as page, section, Section};
use itertools::Itertools;
use slab::Slab;
use slotmap::SlotMap;

#[derive(Default)]
pub struct Page {
    battery: Battery,
    connected_devices: Vec<ConnectedDevice>,
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
            sections.insert(connected_devices()),
            sections.insert(profiles()),
        ])
    }

    fn on_enter(
        &mut self,
        _page: cosmic_settings_page::Entity,
        _sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> cosmic::Command<crate::pages::Message> {
        let futures: Vec<Command<Message>> = vec![
            cosmic::command::future(async move {
                let battery = Battery::update_battery().await;
                Message::UpdateBattery(battery)
            }),
            cosmic::command::future(async move {
                let devices = ConnectedDevice::update_connected_devices().await;
                // let devices = vec![
                //     ConnectedDevice {
                //         model: "Test device 1".to_owned(),
                //         device_icon: "display-symbolic",
                //         battery: Battery::default(),
                //     },
                //     ConnectedDevice {
                //         model: "Test device 2".to_owned(),
                //         device_icon: "laptop-symbolic",
                //         battery: Battery::default()
                //     },
                //     ConnectedDevice {
                //         model: "Test device 3".to_owned(),
                //         device_icon: "network-wired-symbolic",
                //         battery: Battery::default()
                //     },
                // ];
                Message::UpdateConnectedDevices(devices)
            }),
        ];

        cosmic::command::batch(futures).map(crate::pages::Message::Power)
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    PowerProfileChange(PowerProfile),
    UpdateBattery(Battery),
    UpdateConnectedDevices(Vec<ConnectedDevice>),
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
            Message::UpdateConnectedDevices(connected_devices) => {
                self.connected_devices = connected_devices;
            }
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
            let battery_percent = text::body(format!("{}%", page.battery.percent));

            let battery_time = text::body(if page.battery.remaining_duration > TimeDelta::zero() {
                &page.battery.remaining_time
            } else {
                ""
            });

            widget::column::with_capacity(2)
                .spacing(8)
                .push(text::heading(&section.title))
                .push(row!(battery_icon, battery_percent, battery_time).spacing(8))
                .into()
        })
}

fn connected_devices() -> Section<crate::pages::Message> {
    let descriptions = Slab::new();

    Section::default()
        .title(fl!("connected-devices"))
        .descriptions(descriptions)
        .show_while::<Page>(|page| !page.connected_devices.is_empty())
        .view::<Page>(move |_binder, page, section| {
            let devices: Vec<cosmic::Element<'_, _>> = page
                .connected_devices
                .iter()
                .map(|connected_device| {
                    let battery_icon =
                        widget::icon::from_name(connected_device.battery.icon_name.clone());

                    let battery_percent_and_time = widget::text(
                        if connected_device.battery.remaining_duration > TimeDelta::zero() {
                            format!(
                                "{}% - {}",
                                connected_device.battery.percent,
                                &connected_device.battery.remaining_time
                            )
                        } else {
                            format!("{}%", connected_device.battery.percent)
                        },
                    );
                    widget::container(
                        row!(
                            widget::icon::from_name(connected_device.device_icon).size(48),
                            column!(
                                text::heading(&connected_device.model),
                                row!(battery_icon, battery_percent_and_time)
                                    .spacing(4)
                                    .align_items(Alignment::Center),
                            )
                            .height(Length::Shrink)
                        )
                        .align_items(Alignment::Center)
                        .spacing(16)
                        .padding([8, 16])
                        .width(Length::Fill)
                        .height(Length::Fill),
                    )
                    .height(64)
                    .style(cosmic::theme::Container::List)
                    .into()
                })
                .collect();

            widget::column::with_capacity(2)
                .spacing(8)
                .push(text::heading(&section.title))
                .push(
                    widget::column()
                        .extend(
                            devices
                                .into_iter()
                                .chunks(2)
                                .into_iter()
                                .map(|mut device_row| {
                                    row!(
                                        device_row.next().unwrap_or(
                                            widget::horizontal_space(Length::Fill).into()
                                        ),
                                        device_row.next().unwrap_or(
                                            widget::horizontal_space(Length::Fill).into()
                                        ),
                                    )
                                    .spacing(8)
                                }),
                        )
                        .spacing(8),
                )
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
                        settings::item_row(vec![radio(
                            widget::column::with_capacity(2)
                                .push(text::body(profile.title()))
                                .push(text::caption(profile.description())),
                            profile.clone(),
                            Some(current_profile),
                            Message::PowerProfileChange,
                        )
                        .width(Length::Fill)
                        .into()])
                    })
                    .fold(section, settings::Section::add);
            } else {
                let item = text::body(fl!("power-mode", "no-backend"));
                section = section.add(item);
            }

            section
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Power)
        })
}

impl page::AutoBind<crate::pages::Message> for Page {}
