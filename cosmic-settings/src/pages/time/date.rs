// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::str::FromStr;

use chrono::{Datelike, Timelike};
use cosmic::{
    cosmic_config::{self, ConfigGet, ConfigSet},
    iced::Length,
    iced_core::text::Wrap,
    widget::{self, dropdown, settings},
    Apply, Command, Element,
};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use icu::{
    calendar::{DateTime, Iso},
    datetime::DateTimeFormatter,
    locid::Locale,
};
use slab::Slab;
use slotmap::SlotMap;
pub use timedate_zbus::TimeDateProxy;
use tracing::error;

crate::cache_dynamic_lazy! {
    static WEEKDAYS: [String; 4] = [fl!("time-format", "friday"), fl!("time-format", "saturday"), fl!("time-format", "sunday"), fl!("time-format", "monday")];
}

#[derive(Debug, Clone)]
pub struct Info {
    pub ntp_enabled: bool,
    pub timezone_id: Option<usize>,
    pub timezone_list: Vec<String>,
}

pub struct Page {
    cosmic_applet_config: cosmic_config::Config,
    first_day_of_week: usize,
    military_time: bool,
    show_seconds: bool,
    ntp_enabled: bool,
    show_date_in_top_panel: bool,
    timezone_context: bool,
    local_time: Option<DateTime<Iso>>,
    timezone: Option<usize>,
    timezone_list: Vec<String>,
    timezone_search: String,
    formatted_date: String,
}

impl Default for Page {
    fn default() -> Self {
        let cosmic_applet_config =
            cosmic_config::Config::new("com.system76.CosmicAppletTime", 1).unwrap();

        let military_time = cosmic_applet_config
            .get("military_time")
            .unwrap_or_else(|err| {
                if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                    error!(?err, "Failed to read config 'military_time'");
                }

                false
            });

        let show_seconds = cosmic_applet_config
            .get("show_seconds")
            .unwrap_or_else(|err| {
                if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                    error!(?err, "Failed to read config 'show_seconds'");
                }

                false
            });

        let first_day_of_week = cosmic_applet_config
            .get("first_day_of_week")
            .unwrap_or_else(|err| {
                if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                    error!(?err, "Failed to read config 'first_day_of_week'");
                }

                6
            });

        let show_date_in_top_panel = cosmic_applet_config
            .get("show_date_in_top_panel")
            .unwrap_or_else(|err| {
                if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                    error!(?err, "Failed to read config 'show_date_in_top_panel'");
                }

                true
            });

        Self {
            cosmic_applet_config,
            first_day_of_week,
            formatted_date: String::new(),
            local_time: None,
            military_time,
            show_seconds,
            ntp_enabled: false,
            show_date_in_top_panel,
            timezone: None,
            timezone_context: false,
            timezone_list: Vec::new(),
            timezone_search: String::new(),
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(date()),
            sections.insert(timezone()),
            sections.insert(format()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("time-date", "preferences-system-time-symbolic")
            .title(fl!("time-date"))
            .description(fl!("time-date", "desc"))
    }

    fn on_enter(
        &mut self,
        _page: cosmic_settings_page::Entity,
        _sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> Command<crate::pages::Message> {
        cosmic::command::future(async move {
            let client = match zbus::Connection::system().await {
                Ok(client) => client,
                Err(why) => {
                    return Message::Error(why.to_string());
                }
            };

            let timedate_proxy = match TimeDateProxy::new(&client).await {
                Ok(timedate_proxy) => timedate_proxy,
                Err(why) => {
                    return Message::Error(why.to_string());
                }
            };

            let can_ntp = timedate_proxy.can_ntp().await.unwrap_or_default();
            let ntp_enabled = can_ntp && timedate_proxy.ntp().await.unwrap_or_default();
            let timezone_list = timedate_proxy.list_timezones().await.unwrap_or_default();

            let timezone = timedate_proxy.timezone().await.unwrap_or_default();

            Message::Refresh(Info {
                ntp_enabled,
                timezone_id: timezone_list.iter().position(|tz| tz == &timezone),
                timezone_list,
            })
        })
        .map(crate::pages::Message::DateAndTime)
    }

    fn context_drawer(&self) -> Option<Element<'_, crate::pages::Message>> {
        if self.timezone_context {
            return Some(self.timezone_context_view());
        }

        None
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Command<crate::Message> {
        match message {
            Message::TimezoneContext => {
                self.timezone_search.clear();
                self.timezone_context = true;
                return cosmic::command::message(crate::app::Message::OpenContextDrawer(
                    fl!("time-zone").into(),
                ));
            }

            Message::MilitaryTime(enable) => {
                self.military_time = enable;
                self.update_local_time();

                if let Err(err) = self.cosmic_applet_config.set("military_time", enable) {
                    error!(?err, "Failed to set config 'military_time'");
                }
            }

            Message::ShowSeconds(enable) => {
                self.show_seconds = enable;
                self.update_local_time();

                if let Err(err) = self.cosmic_applet_config.set("show_seconds", enable) {
                    error!(?err, "Failed to set config 'show_seconds'");
                }
            }

            Message::FirstDayOfWeek(weekday) => {
                self.first_day_of_week = weekday;

                if let Err(err) = self.cosmic_applet_config.set("first_day_of_week", weekday) {
                    error!(?err, "Failed to set config 'first_day_of_week'");
                }
            }

            Message::ShowDate(enable) => {
                self.show_date_in_top_panel = enable;

                if let Err(err) = self
                    .cosmic_applet_config
                    .set("show_date_in_top_panel", enable)
                {
                    error!(?err, "Failed to set config 'show_date_in_top_panel'");
                }
            }

            Message::TimezoneSearch(text) => {
                self.timezone_search = text;
            }

            Message::Timezone(timezone_id) => {
                self.timezone = Some(timezone_id);

                if let Some(timezone) = self.timezone_list.get(timezone_id).cloned() {
                    return cosmic::command::future(async move {
                        let client = match zbus::Connection::system().await {
                            Ok(client) => client,
                            Err(why) => {
                                return Message::Error(why.to_string());
                            }
                        };

                        let timedate_proxy = match TimeDateProxy::new(&client).await {
                            Ok(timedate_proxy) => timedate_proxy,
                            Err(why) => {
                                return Message::Error(why.to_string());
                            }
                        };

                        match timedate_proxy.set_timezone(&timezone, true).await {
                            Ok(_) => Message::UpdateTime,
                            Err(why) => Message::Error(why.to_string()),
                        }
                    })
                    .map(crate::pages::Message::DateAndTime)
                    .map(crate::Message::PageMessage);
                }
            }

            Message::Error(why) => {
                tracing::error!(why, "failed to set timezone");
                self.timezone_context = false;
                return cosmic::command::message(crate::Message::CloseContextDrawer);
            }

            Message::UpdateTime => {
                self.set_ntp(true);
                self.update_local_time();
                self.timezone_context = false;
                return cosmic::command::message(crate::Message::CloseContextDrawer);
            }

            Message::Refresh(info) => {
                self.ntp_enabled = info.ntp_enabled;
                self.timezone_list = info.timezone_list;
                self.timezone = info.timezone_id;

                self.update_local_time();
            }

            Message::None => (),
        }

        Command::none()
    }

    fn set_ntp(&mut self, enable: bool) {
        self.ntp_enabled = enable;

        tokio::task::spawn(async move {
            let client = match zbus::Connection::system().await {
                Ok(client) => client,
                Err(why) => {
                    tracing::error!(?why, "zbus client error");
                    return;
                }
            };

            let timedate_proxy = match TimeDateProxy::new(&client).await {
                Ok(timedate_proxy) => timedate_proxy,
                Err(why) => {
                    tracing::error!(?why, "zbus client error");
                    return;
                }
            };

            _ = timedate_proxy.set_ntp(enable, true).await;
        });
    }

    fn timezone_context_view(&self) -> Element<'_, crate::pages::Message> {
        let search = widget::search_input(fl!("type-to-search"), &self.timezone_search)
            .on_input(Message::TimezoneSearch)
            .on_clear(Message::TimezoneSearch(String::new()));

        let mut list = widget::list_column();

        let search_input = &self.timezone_search.trim().to_lowercase();

        for (id, timezone) in self.timezone_list.iter().enumerate() {
            if search_input.is_empty() || timezone.to_lowercase().contains(search_input) {
                list = list.add(self.timezone_context_item(id, timezone));
            }
        }

        widget::column()
            .padding([2, 0])
            .spacing(32)
            .push(search)
            .push(list)
            .apply(Element::from)
            .map(crate::pages::Message::DateAndTime)
    }

    fn timezone_context_item<'a>(&self, id: usize, timezone: &'a str) -> Element<'a, Message> {
        widget::button(widget::settings::item_row(vec![
            widget::text::body(timezone).wrap(Wrap::Word).into(),
            widget::horizontal_space(Length::Fill).into(),
        ]))
        .on_press(Message::Timezone(id))
        .style(cosmic::theme::Button::Icon)
        .into()
    }

    fn update_local_time(&mut self) {
        self.local_time = Some(update_local_time());

        self.formatted_date = match self.local_time {
            Some(ref time) => format_date(time, self.military_time, self.show_seconds),
            None => fl!("unknown"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Error(String),
    MilitaryTime(bool),
    ShowSeconds(bool),
    None,
    FirstDayOfWeek(usize),
    Refresh(Info),
    ShowDate(bool),
    Timezone(usize),
    TimezoneContext,
    TimezoneSearch(String),
    UpdateTime,
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn date() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let title = descriptions.insert(fl!("time-date"));

    Section::default()
        .title(fl!("time-date"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            settings::view_section(&section.title)
                .add(
                    settings::item::builder(&*section.descriptions[title])
                        .description(fl!("time-date", "auto-ntp"))
                        .control(widget::text::body(&page.formatted_date)),
                )
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::DateAndTime)
        })
}

fn format() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let military = descriptions.insert(fl!("time-format", "twenty-four"));
    let show_seconds = descriptions.insert(fl!("time-format", "show-seconds"));
    let first = descriptions.insert(fl!("time-format", "first"));
    let show_date = descriptions.insert(fl!("time-format", "show-date"));

    Section::default()
        .title(fl!("time-format"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            settings::view_section(&section.title)
                // 24-hour toggle
                .add(
                    settings::item::builder(&section.descriptions[military])
                        .toggler(page.military_time, Message::MilitaryTime),
                )
                // Show seconds in time format
                .add(
                    settings::item::builder(&section.descriptions[show_seconds])
                        .toggler(page.show_seconds, Message::ShowSeconds),
                )
                // First day of week
                .add(
                    settings::item::builder(&section.descriptions[first]).control(dropdown(
                        &*WEEKDAYS,
                        match page.first_day_of_week {
                            4 => Some(0), // friday
                            5 => Some(1), // saturday
                            0 => Some(3), // monday
                            _ => Some(2), // sunday
                        },
                        |v| {
                            match v {
                                0 => Message::FirstDayOfWeek(4), // friday
                                1 => Message::FirstDayOfWeek(5), // saturday
                                3 => Message::FirstDayOfWeek(0), // monday
                                _ => Message::FirstDayOfWeek(6), // sunday
                            }
                        },
                    )),
                )
                // Date on top panel toggle
                .add(
                    settings::item::builder(&section.descriptions[show_date])
                        .toggler(page.show_date_in_top_panel, Message::ShowDate),
                )
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::DateAndTime)
        })
}

fn timezone() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let time_zone = descriptions.insert(fl!("time-zone"));

    Section::default()
        .title(fl!("time-zone"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let timezone_context_button = widget::row::with_capacity(2)
                .spacing(12)
                .push(
                    widget::text::body(
                        page.timezone
                            .map(|id| &*page.timezone_list[id])
                            .unwrap_or_default(),
                    )
                    .wrap(Wrap::Word),
                )
                .push(widget::icon::from_name("go-next-symbolic").size(16).icon())
                .apply(widget::container)
                .style(cosmic::theme::Container::List)
                .apply(widget::button)
                .style(cosmic::theme::Button::Transparent)
                .on_press(Message::TimezoneContext);

            settings::view_section(&section.title)
                // Time zone select
                .add(
                    settings::item::builder(&*section.descriptions[time_zone])
                        .control(timezone_context_button),
                )
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::DateAndTime)
        })
}

fn locale() -> Result<Locale, Box<dyn std::error::Error>> {
    let locale = std::env::var("LC_TIME").or_else(|_| std::env::var("LANG"))?;
    let locale = locale
        .split('.')
        .next()
        .ok_or(format!("Can't split the locale {locale}"))?;

    let locale = Locale::from_str(locale).map_err(|e| format!("{e:?}"))?;
    Ok(locale)
}

fn format_date(date: &DateTime<Iso>, military: bool, show_seconds: bool) -> String {
    let Ok(locale) = locale() else {
        return String::new();
    };

    let mut bag = icu::datetime::options::components::Bag::empty();

    bag.year = Some(icu::datetime::options::components::Year::Numeric);
    bag.day = Some(icu::datetime::options::components::Day::NumericDayOfMonth);
    bag.month = Some(icu::datetime::options::components::Month::Long);
    bag.hour = Some(icu::datetime::options::components::Numeric::Numeric);
    bag.minute = Some(icu::datetime::options::components::Numeric::Numeric);
    bag.second = show_seconds.then_some(icu::datetime::options::components::Numeric::Numeric);
    bag.preferences = Some(icu::datetime::options::preferences::Bag::from_hour_cycle(
        if military {
            icu::datetime::options::preferences::HourCycle::H23
        } else {
            icu::datetime::options::preferences::HourCycle::H12
        },
    ));

    let dtf = DateTimeFormatter::try_new_experimental(&locale.into(), bag.into()).unwrap();

    dtf.format(&date.to_any())
        .expect("can't format value")
        .to_string()
}

fn update_local_time() -> DateTime<Iso> {
    let now = chrono::Local::now();

    DateTime::try_new_gregorian_datetime(
        now.year(),
        now.month() as u8,
        now.day() as u8,
        now.hour() as u8,
        now.minute() as u8,
        now.second() as u8,
    )
    .unwrap()
    .to_iso()
}
