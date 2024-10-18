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
use cosmic_config::{Config, CosmicConfigEntry};
use cosmic_idle_config::CosmicIdleConfig;
use cosmic_settings_page::{self as page, section, Section};
use itertools::Itertools;
use slab::Slab;
use slotmap::SlotMap;
use std::iter;
use std::time::Duration;

static SCREEN_OFF_TIMES: &[Duration] = &[
    Duration::from_secs(2 * 60),
    Duration::from_secs(5 * 60),
    Duration::from_secs(10 * 60),
    Duration::from_secs(15 * 60),
    Duration::from_secs(30 * 60),
];

static SUSPEND_TIMES: &[Duration] = &[
    Duration::from_secs(15 * 60),
    Duration::from_secs(20 * 60),
    Duration::from_secs(25 * 60),
    Duration::from_secs(30 * 60),
    Duration::from_secs(45 * 60),
    Duration::from_secs(1 * 60 * 60),
    Duration::from_secs(80 * 60),
    Duration::from_secs(90 * 60),
    Duration::from_secs(100 * 60),
    Duration::from_secs(2 * 60 * 60),
];

fn format_time(duration: Duration) -> String {
    let m = duration.as_secs() / 60;
    if m % 60 == 0 {
        fl!("x-hours", number = (m / 60))
    } else {
        fl!("x-minutes", number = m)
    }
}

pub struct Page {
    battery: Battery,
    connected_devices: Vec<ConnectedDevice>,
    screen_off_labels: Vec<String>,
    suspend_labels: Vec<String>,
    screen_off_time: Option<Duration>,
    idle_config: Config,
    idle_conf: CosmicIdleConfig,
}

impl Default for Page {
    fn default() -> Self {
        let idle_config = Config::new("com.system76.CosmicIdle", 1).unwrap();
        let idle_conf = CosmicIdleConfig::get_entry(&idle_config).unwrap_or_else(|(_, conf)| conf);

        Self {
            battery: Default::default(),
            connected_devices: Vec::new(),
            screen_off_labels: SCREEN_OFF_TIMES
                .iter()
                .copied()
                .map(format_time)
                .chain(iter::once(fl!("never")))
                .collect(),
            suspend_labels: SUSPEND_TIMES
                .iter()
                .copied()
                .map(format_time)
                .chain(iter::once(fl!("never")))
                .collect(),
            screen_off_time: None,
            idle_config,
            idle_conf,
        }
    }
}

// TODO default: options

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
            sections.insert(power_saving()),
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
    ScreenOffTimeChange(Option<Duration>),
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
            Message::ScreenOffTimeChange(time) => {
                self.screen_off_time = time;
                // XXX
                let time = time.map(|x| x.as_millis() as u32).unwrap_or(u32::MAX);
                self.idle_conf.set_screen_off_time(&self.idle_config, time);
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
            let remaining_time = page.battery.remaining_time();
            let battery_label = text::body(if remaining_time.is_empty() {
                format!("{:.0}%", page.battery.percent)
            } else {
                format!("{:.0}% ({})", page.battery.percent, remaining_time)
            });

            widget::column::with_capacity(2)
                .push(text::heading(&section.title))
                .push(
                    row!(battery_icon, battery_label)
                        .align_items(Alignment::Center)
                        .spacing(cosmic::theme::active().cosmic().space_xxxs()),
                )
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
                                &connected_device.battery.remaining_time()
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
            let mut section = settings::section().title(&section.title);

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

fn power_saving_row<'a>(
    label: &'a str,
    labels: &'a [String],
    selected: Option<usize>,
) -> cosmic::Element<'a, Message> {
    settings::item_row(vec![
        widget::text(label).into(),
        widget::dropdown(labels, selected, |i| {
            Message::ScreenOffTimeChange(SCREEN_OFF_TIMES.get(i).copied())
        })
        .into(),
    ])
    .into()
}

fn power_saving() -> Section<crate::pages::Message> {
    Section::default()
        .title("Power Savings Options")
        .view::<Page>(move |_binder, page, section| {
            settings::section()
                .title(&section.title)
                .add(power_saving_row(
                    "Turn off the screen after",
                    &page.screen_off_labels,
                    Some(
                        SCREEN_OFF_TIMES
                            .iter()
                            .position(|x| Some(*x) == page.screen_off_time)
                            .unwrap_or(SCREEN_OFF_TIMES.len()),
                    ),
                ))
                .add(power_saving_row(
                    "Automatic suspend when plugged in",
                    &page.suspend_labels,
                    None,
                ))
                .add(power_saving_row(
                    "Automatic on battery power",
                    &page.suspend_labels,
                    None,
                ))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Power)
        })
}

impl page::AutoBind<crate::pages::Message> for Page {}
