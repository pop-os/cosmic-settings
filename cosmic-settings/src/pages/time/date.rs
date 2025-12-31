// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use chrono::{Datelike, Timelike};
use cosmic::{
    Apply, Element, Task,
    app::ContextDrawer,
    cosmic_config::{self, ConfigGet, ConfigSet},
    iced_core::text::Wrapping,
    surface,
    widget::{self, dropdown, settings},
};
use cosmic_settings_page::{self as page, Section, section};
use icu::{
    calendar::{Gregorian, types::Weekday, week},
    datetime::{
        DateTimeFormatter, DateTimeFormatterPreferences, fieldsets,
        input::{Date, DateTime, Time},
        options::TimePrecision,
    },
    locale::{Locale, preferences::extensions::unicode::keywords::HourCycle},
};
use slab::Slab;
use slotmap::{Key, SlotMap};
use std::rc::Rc;
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
    entity: page::Entity,
    cosmic_applet_config: cosmic_config::Config,
    first_day_of_week: usize,
    military_time: bool,
    show_seconds: bool,
    ntp_enabled: bool,
    show_date_in_top_panel: bool,
    timezone_context: bool,
    local_time: Option<DateTime<Gregorian>>,
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
                if err.is_err() {
                    error!(?err, "Failed to read config 'military_time'");
                }

                let default = get_locale_default_24h();
                let _ = cosmic_applet_config.set("military_time", default);
                default
            });

        let show_seconds = cosmic_applet_config
            .get("show_seconds")
            .unwrap_or_else(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'show_seconds'");
                }

                false
            });

        let first_day_of_week = cosmic_applet_config
            .get("first_day_of_week")
            .unwrap_or_else(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'first_day_of_week'");
                }

                let default = get_locale_default_first_day();
                let _ = cosmic_applet_config.set("first_day_of_week", default);
                default
            });

        let show_date_in_top_panel = cosmic_applet_config
            .get("show_date_in_top_panel")
            .unwrap_or_else(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'show_date_in_top_panel'");
                }

                true
            });

        Self {
            entity: page::Entity::null(),
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
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

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

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        cosmic::Task::future(async move {
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

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        if self.timezone_context {
            let search = widget::search_input("", &self.timezone_search)
                .on_input(Message::TimezoneSearch)
                .on_clear(Message::TimezoneSearch(String::new()))
                .apply(Element::from)
                .map(crate::pages::Message::DateAndTime);

            return Some(
                cosmic::app::context_drawer(
                    self.timezone_context_view()
                        .map(crate::pages::Message::from),
                    crate::pages::Message::CloseContextDrawer,
                )
                .title(fl!("time-zone"))
                .header(search),
            );
        }

        None
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::Message> {
        match message {
            Message::TimezoneContext => {
                self.timezone_search.clear();
                self.timezone_context = true;
                return cosmic::task::message(crate::app::Message::OpenContextDrawer(self.entity));
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
                    return cosmic::Task::future(async move {
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
                return cosmic::task::message(crate::pages::Message::CloseContextDrawer);
            }

            Message::UpdateTime => {
                self.set_ntp(true);
                self.update_local_time();
                self.timezone_context = false;
                return cosmic::task::message(crate::pages::Message::CloseContextDrawer);
            }

            Message::Refresh(info) => {
                self.ntp_enabled = info.ntp_enabled;
                self.timezone_list = info.timezone_list;
                self.timezone = info.timezone_id;

                self.update_local_time();
            }

            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }

            Message::None => (),
        }

        Task::none()
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
        let mut list = widget::list_column();

        let search_input = &self.timezone_search.trim().to_lowercase();

        for (id, timezone) in self.timezone_list.iter().enumerate() {
            if search_input.is_empty() || timezone.to_lowercase().contains(search_input) {
                list = list.add(self.timezone_context_item(id, timezone));
            }
        }

        list.apply(Element::from)
            .map(crate::pages::Message::DateAndTime)
    }

    fn timezone_context_item<'a>(&self, id: usize, timezone: &'a str) -> Element<'a, Message> {
        let svg_accent = Rc::new(|theme: &cosmic::Theme| cosmic::widget::svg::Style {
            color: Some(theme.cosmic().accent_text_color().into()),
        });
        let selected = Some(id) == self.timezone;

        widget::settings::item_row(vec![
            widget::text::body(timezone)
                .class(if selected {
                    cosmic::theme::Text::Accent
                } else {
                    cosmic::theme::Text::Default
                })
                .wrapping(Wrapping::Word)
                .width(cosmic::iced::Length::Fill)
                .into(),
            if selected {
                widget::icon::from_name("object-select-symbolic")
                    .size(16)
                    .icon()
                    .class(cosmic::theme::Svg::Custom(svg_accent.clone()))
                    .into()
            } else {
                widget::horizontal_space().width(16).into()
            },
        ])
        .apply(widget::container)
        .class(cosmic::theme::Container::List)
        .apply(widget::button::custom)
        .class(cosmic::theme::Button::Transparent)
        .on_press(Message::Timezone(id))
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
    Surface(surface::Action),
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn date() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let title = descriptions.insert(fl!("time-date"));

    Section::default()
        .title(fl!("time-date"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            settings::section()
                .title(&section.title)
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
            settings::section()
                .title(&section.title)
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
                    settings::item::builder(&section.descriptions[first]).control(
                        dropdown::popup_dropdown(
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
                            cosmic::iced::window::Id::RESERVED,
                            Message::Surface,
                            |a| {
                                crate::app::Message::PageMessage(
                                    crate::pages::Message::DateAndTime(a),
                                )
                            },
                        ),
                    ),
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
                    .wrapping(Wrapping::Word),
                )
                .push(widget::icon::from_name("go-next-symbolic").size(16).icon())
                .apply(widget::container)
                .class(cosmic::theme::Container::List)
                .apply(widget::button::custom)
                .class(cosmic::theme::Button::Transparent)
                .on_press(Message::TimezoneContext);

            settings::section()
                .title(&section.title)
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
    let locale_env = std::env::var("LC_TIME").or_else(|_| std::env::var("LANG"))?;

    locale_env
        .split('.')
        .next()
        .ok_or(format!("Can't split the locale {locale_env}"))?
        .replacen("_", "-", 1)
        .parse::<Locale>()
        .map_err(|e| format!("{e:?}").into())
}

fn format_date(date: &DateTime<Gregorian>, military: bool, show_seconds: bool) -> String {
    let Ok(locale) = locale() else {
        return String::new();
    };

    let mut prefs = DateTimeFormatterPreferences::from(locale);
    prefs.hour_cycle = Some(if military {
        HourCycle::H23
    } else {
        HourCycle::H12
    });

    let mut fs = fieldsets::YMDT::long();
    if !show_seconds {
        fs = fs.with_time_precision(TimePrecision::Minute);
    }

    let dtf = DateTimeFormatter::try_new(prefs, fs).unwrap();

    dtf.format(date).to_string()
}

fn update_local_time() -> DateTime<Gregorian> {
    let now = chrono::Local::now();

    DateTime {
        date: Date::try_new_gregorian(now.year(), now.month() as u8, now.day() as u8).unwrap(),
        time: Time::try_new(now.hour() as u8, now.minute() as u8, now.second() as u8, 0).unwrap(),
    }
}

fn get_locale_default_24h() -> bool {
    let Ok(locale) = locale() else { return false };

    let test_time = DateTime {
        date: Date::try_new_gregorian(2024, 1, 1).unwrap(),
        time: Time::try_new(13, 0, 0, 0).unwrap(),
    };

    let prefs = DateTimeFormatterPreferences::from(locale);
    let Ok(dtf) = DateTimeFormatter::try_new(prefs, fieldsets::T::medium()) else {
        return false;
    };

    let formatted = dtf.format(&test_time).to_string();

    // If we see "13" in the output, it's 24-hour format
    // If we see "1" (but not "13"), it's 12-hour format
    formatted.contains("13")
}

fn get_locale_default_first_day() -> usize {
    let Ok(locale) = locale() else { return 6 };
    let Ok(week_info) = week::WeekInformation::try_new(week::WeekPreferences::from(&locale)) else {
        return 6;
    };

    match week_info.first_weekday {
        Weekday::Monday => 0,
        Weekday::Tuesday => 1,
        Weekday::Wednesday => 2,
        Weekday::Thursday => 3,
        Weekday::Friday => 4,
        Weekday::Saturday => 5,
        Weekday::Sunday => 6,
    }
}
