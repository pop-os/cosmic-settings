mod backend;

use self::backend::{GetCurrentPowerProfile, SetPowerProfile};
use backend::{Battery, ConnectedDevice, PowerProfile};

use chrono::TimeDelta;
use cosmic::iced::{Alignment, Length};
use cosmic::iced_widget::{column, row};
use cosmic::widget::{self, radio, settings, text};
use cosmic::Task;
use cosmic::{surface, Apply};
use cosmic_config::{Config, CosmicConfigEntry};
use cosmic_idle_config::CosmicIdleConfig;
use cosmic_settings_page::{self as page, section, Section};
use futures::StreamExt;
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
    entity: page::Entity,
    battery: Battery,
    connected_devices: Vec<ConnectedDevice>,
    on_enter_handle: Option<cosmic::iced::task::Handle>,
    screen_off_labels: Vec<String>,
    suspend_labels: Vec<String>,
    idle_config: Config,
    idle_conf: CosmicIdleConfig,
}

impl Default for Page {
    fn default() -> Self {
        let idle_config = Config::new("com.system76.CosmicIdle", 1).unwrap();
        let idle_conf = CosmicIdleConfig::get_entry(&idle_config).unwrap_or_else(|(_, conf)| conf);

        Self {
            entity: Default::default(),
            battery: Default::default(),
            connected_devices: Vec::new(),
            on_enter_handle: None,
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
            idle_config,
            idle_conf,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

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

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        let futures: Vec<Task<Message>> = vec![
            cosmic::Task::future(async move {
                let battery = Battery::update_battery().await;
                Message::UpdateBattery(battery)
            }),
            cosmic::Task::future(async move {
                let devices = ConnectedDevice::update_connected_devices().await;
                Message::UpdateConnectedDevices(devices)
            }),
            cosmic::Task::run(
                async_fn_stream::fn_stream(|emitter| async move {
                    let span = tracing::span!(tracing::Level::INFO, "power::device_stream task");
                    let _span_handle = span.enter();

                    let Ok(connection) = zbus::Connection::system().await else {
                        tracing::error!("could not established zbus connection to system");
                        return;
                    };

                    let added_stream = ConnectedDevice::device_added_stream(&connection).await;
                    let removed_stream = ConnectedDevice::device_removed_stream(&connection).await;

                    let added_future = std::pin::pin!(async {
                        match added_stream {
                            Ok(stream) => {
                                let mut stream = std::pin::pin!(stream);
                                while let Some(device) = stream.next().await {
                                    tracing::info!(device = device.model, "device added");
                                    emitter.emit(Message::DeviceConnect(device)).await;
                                }
                            }
                            Err(err) => tracing::error!(?err, "cannot establish added stream"),
                        }
                    });

                    let removed_future = std::pin::pin!(async {
                        match removed_stream {
                            Ok(stream) => {
                                let mut stream = std::pin::pin!(stream);
                                while let Some(device_path) = stream.next().await {
                                    tracing::info!(device_path, "device removed");
                                    emitter.emit(Message::DeviceDisconnect(device_path)).await;
                                }
                            }
                            Err(err) => tracing::error!(?err, "cannot establish removed stream"),
                        }
                    });

                    futures::future::select(added_future, removed_future).await;
                }),
                |msg| msg,
            ),
        ];

        let (task, handle) = cosmic::Task::batch(futures)
            .map(crate::pages::Message::Power)
            .abortable();

        self.on_enter_handle = Some(handle);
        task
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }

        Task::none()
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    PowerProfileChange(PowerProfile),
    UpdateBattery(Battery),
    UpdateConnectedDevices(Vec<ConnectedDevice>),
    DeviceDisconnect(String),
    DeviceConnect(ConnectedDevice),
    ScreenOffTimeChange(Option<Duration>),
    SuspendOnAcTimeChange(Option<Duration>),
    SuspendOnBatteryTimeChange(Option<Duration>),
    Surface(surface::Action),
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
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
                let time = time.map(|x| x.as_millis() as u32);
                if let Err(err) = self.idle_conf.set_screen_off_time(&self.idle_config, time) {
                    tracing::error!("failed to set screen off time: {}", err)
                }
            }
            Message::SuspendOnAcTimeChange(time) => {
                let time = time.map(|x| x.as_millis() as u32);
                if let Err(err) = self
                    .idle_conf
                    .set_suspend_on_ac_time(&self.idle_config, time)
                {
                    tracing::error!("failed to set suspend on ac time: {}", err)
                }
            }
            Message::SuspendOnBatteryTimeChange(time) => {
                let time = time.map(|x| x.as_millis() as u32);
                if let Err(err) = self
                    .idle_conf
                    .set_suspend_on_battery_time(&self.idle_config, time)
                {
                    tracing::error!("failed to set suspend on battery time: {}", err)
                }
            }
            Message::DeviceDisconnect(device_path) => self
                .connected_devices
                .retain(|device| device.device_path != device_path),
            Message::DeviceConnect(connected_device) => {
                self.connected_devices.push(connected_device)
            }
            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }
        };
        Task::none()
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
                        .align_y(Alignment::Center)
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
                                text::heading(&connected_device.model).height(20.0),
                                row!(battery_icon, battery_percent_and_time)
                                    .spacing(4)
                                    .align_y(Alignment::Center),
                            )
                            .height(Length::Shrink)
                        )
                        .align_y(Alignment::Center)
                        .spacing(16)
                        .padding([8, 16])
                        .width(Length::Fill)
                        .height(Length::Fill),
                    )
                    .height(64)
                    .class(cosmic::theme::Container::List)
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
                                    cosmic::Element::from(
                                        row!(
                                            device_row.next().unwrap_or(
                                                widget::horizontal_space()
                                                    .width(Length::Fill)
                                                    .into()
                                            ),
                                            device_row.next().unwrap_or(
                                                widget::horizontal_space()
                                                    .width(Length::Fill)
                                                    .into()
                                            ),
                                        )
                                        .spacing(8),
                                    )
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
                            profile,
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
    selected_time: Option<Duration>,
    times: &'static [Duration],
    on_select: fn(Option<Duration>) -> Message,
) -> cosmic::Element<'a, Message> {
    let selected = if let Some(time) = selected_time {
        times.iter().position(|x| *x == time)
    } else {
        // "Never"
        Some(times.len())
    };

    settings::item(
        label,
        widget::dropdown::popup_dropdown(
            labels,
            selected,
            move |i| on_select(times.get(i).copied()),
            cosmic::iced::window::Id::RESERVED,
            Message::Surface,
            |a| crate::app::Message::PageMessage(crate::pages::Message::Power(a)),
        ),
    )
    .into()
}

fn power_saving() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let turn_off_screen_desc = descriptions.insert(fl!("power-saving", "turn-off-screen-after"));
    let auto_suspend_desc = descriptions.insert(fl!("power-saving", "auto-suspend"));
    let auto_suspend_ac_desc = descriptions.insert(fl!("power-saving", "auto-suspend-ac"));
    let auto_suspend_battery_desc =
        descriptions.insert(fl!("power-saving", "auto-suspend-battery"));

    Section::default()
        .title(fl!("power-saving"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let screen_off_time = page
                .idle_conf
                .screen_off_time
                .map(|t| Duration::from_millis(t.into()));
            let suspend_on_ac_time = page
                .idle_conf
                .suspend_on_ac_time
                .map(|t| Duration::from_millis(t.into()));
            let suspend_on_battery_time = page
                .idle_conf
                .suspend_on_battery_time
                .map(|t| Duration::from_millis(t.into()));
            let mut section = settings::section()
                .title(&section.title)
                .add(power_saving_row(
                    &descriptions[turn_off_screen_desc],
                    &page.screen_off_labels,
                    screen_off_time,
                    SCREEN_OFF_TIMES,
                    Message::ScreenOffTimeChange,
                ))
                .add(power_saving_row(
                    &descriptions[if page.battery.is_present {
                        auto_suspend_ac_desc
                    } else {
                        auto_suspend_desc
                    }],
                    &page.suspend_labels,
                    suspend_on_ac_time,
                    SUSPEND_TIMES,
                    Message::SuspendOnAcTimeChange,
                ));
            if page.battery.is_present {
                section = section.add(power_saving_row(
                    &descriptions[auto_suspend_battery_desc],
                    &page.suspend_labels,
                    suspend_on_battery_time,
                    SUSPEND_TIMES,
                    Message::SuspendOnBatteryTimeChange,
                ));
            }
            section
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Power)
        })
}

impl page::AutoBind<crate::pages::Message> for Page {}
