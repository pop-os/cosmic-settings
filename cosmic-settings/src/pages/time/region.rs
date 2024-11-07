// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::collections::BTreeMap;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;

use cosmic::iced::{Border, Color, Length};
use cosmic::iced_core::text::Wrapping;
use cosmic::widget::{self, button, container};
use cosmic::{theme, Apply, Element};
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use eyre::Context;
use fixed_decimal::FixedDecimal;
use icu::calendar::DateTime;
use icu::datetime::options::components::{self, Bag};
use icu::datetime::options::preferences;
use icu::datetime::DateTimeFormatter;
use icu::decimal::options::FixedDecimalFormatterOptions;
use icu::decimal::FixedDecimalFormatter;
use lichen_system::locale;
use slotmap::{DefaultKey, SlotMap};
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub enum Message {
    AddLanguage(DefaultKey),
    AddLanguageContext,
    AddLanguageSearch(String),
    SystemLocales(SlotMap<DefaultKey, SystemLocale>),
    ExpandLanguagePopover(Option<usize>),
    InstallAdditionalLanguages,
    SelectRegion(DefaultKey),
    SourceContext(SourceContext),
    Refresh(Arc<eyre::Result<PageRefresh>>),
    RegionContext,
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Region(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Region(message)
    }
}

enum ContextView {
    AddLanguage,
    Region,
}

#[derive(Clone, Debug)]
pub enum SourceContext {
    MoveDown(usize),
    MoveUp(usize),
    Remove(usize),
}

#[derive(Clone, Debug)]
struct SystemLocale {
    lang_code: String,
    display_name: String,
    region_name: String,
}

#[derive(Debug)]
pub struct PageRefresh {
    config: Option<(cosmic_config::Config, Vec<String>)>,
    registry: Registry,
    language: Option<SystemLocale>,
    region: Option<SystemLocale>,
    available_languages: SlotMap<DefaultKey, SystemLocale>,
    system_locales: BTreeMap<String, SystemLocale>,
}

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
    config: Option<(cosmic_config::Config, Vec<String>)>,
    context: Option<ContextView>,
    language: Option<SystemLocale>,
    region: Option<SystemLocale>,
    available_languages: SlotMap<DefaultKey, SystemLocale>,
    system_locales: BTreeMap<String, SystemLocale>,
    registry: Option<locale::Registry>,
    expanded_source_popover: Option<usize>,
    add_language_search: String,
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
            sections.insert(preferred_languages::section()),
            sections.insert(formatting::section()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("time-region", "preferences-region-and-language-symbolic")
            .title(fl!("time-region"))
            .description(fl!("time-region", "desc"))
    }

    fn on_enter(
        &mut self,
        sender: mpsc::Sender<crate::pages::Message>,
    ) -> cosmic::Task<crate::pages::Message> {
        cosmic::command::future(async move { Message::Refresh(Arc::new(page_reload().await)) })
    }

    fn context_drawer(&self) -> Option<Element<'_, crate::pages::Message>> {
        Some(match self.context.as_ref()? {
            ContextView::AddLanguage => self.add_language_view(),
            ContextView::Region => self.region_view(),
        })
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::app::Message> {
        match message {
            Message::AddLanguage(id) => {
                if let Some(language) = self.available_languages.get(id) {
                    if let Some((config, locales)) = self.config.as_mut() {
                        if !locales.contains(&language.lang_code) {
                            locales.push(language.lang_code.clone());
                            _ = config.set("system_locales", &locales);
                        }
                    }
                }

                return cosmic::command::message(crate::app::Message::CloseContextDrawer);
            }

            Message::SelectRegion(id) => {
                let mut commands = Vec::with_capacity(2);

                if let Some((region, language)) =
                    self.available_languages.get(id).zip(self.language.as_ref())
                {
                    self.region = Some(region.clone());

                    let lang = language.lang_code.clone();
                    let region = region.lang_code.clone();

                    commands.push(cosmic::command::future(async move {
                        _ = set_locale(lang, region).await;
                        Message::Refresh(Arc::new(page_reload().await))
                    }));
                }

                commands.push(cosmic::Task::done(crate::app::Message::CloseContextDrawer));
                return cosmic::Task::batch(commands);
            }

            Message::AddLanguageContext => {
                self.context = Some(ContextView::AddLanguage);
                return cosmic::Task::done(crate::app::Message::OpenContextDrawer(
                    self.entity,
                    fl!("add-language", "context").into(),
                ));
            }

            Message::AddLanguageSearch(search) => {
                self.add_language_search = search;
            }

            Message::SystemLocales(languages) => {
                self.available_languages = languages;
            }

            Message::ExpandLanguagePopover(id) => {
                self.expanded_source_popover = id;
            }

            Message::InstallAdditionalLanguages => {
                return cosmic::command::future(async move {
                    _ = tokio::process::Command::new("gnome-language-selector")
                        .status()
                        .await;

                    Message::Refresh(Arc::new(page_reload().await))
                })
            }

            Message::Refresh(result) => match Arc::into_inner(result).unwrap() {
                Ok(page_refresh) => {
                    self.config = page_refresh.config;
                    self.available_languages = page_refresh.available_languages;
                    self.system_locales = page_refresh.system_locales;
                    self.language = page_refresh.language;
                    self.region = page_refresh.region;
                    self.registry = Some(page_refresh.registry.0);
                }

                Err(why) => {
                    tracing::error!(?why, "failed to get locales from the system");
                }
            },

            Message::RegionContext => {
                self.context = Some(ContextView::Region);
                return cosmic::Task::done(crate::app::Message::OpenContextDrawer(
                    self.entity,
                    fl!("region").into(),
                ));
            }

            Message::SourceContext(context_message) => {
                self.expanded_source_popover = None;

                if let Some((config, locales)) = self.config.as_mut() {
                    match context_message {
                        SourceContext::MoveDown(id) => {
                            if id + 1 < locales.len() {
                                locales.swap(id, id + 1);
                            }
                        }

                        SourceContext::MoveUp(id) => {
                            if id > 0 {
                                locales.swap(id, id - 1);
                            }
                        }

                        SourceContext::Remove(id) => {
                            let _removed = locales.remove(id);
                        }
                    }

                    _ = config.set("system_locales", &locales);

                    if let Some(language_code) = locales.get(0) {
                        if let Some(language) = self
                            .available_languages
                            .values()
                            .find(|lang| &lang.lang_code == language_code)
                        {
                            let language = language.clone();
                            self.language = Some(language.clone());
                            let region = self.region.clone();

                            tokio::spawn(async move {
                                _ = set_locale(
                                    language.lang_code.clone(),
                                    region.unwrap_or(language).lang_code.clone(),
                                )
                                .await;
                            });
                        }
                    }
                }
            }
        }

        cosmic::Task::none()
    }

    fn add_language_view(&self) -> cosmic::Element<'_, crate::pages::Message> {
        let space_l = theme::active().cosmic().spacing.space_l;

        let search = widget::search_input(fl!("type-to-search"), &self.add_language_search)
            .on_input(Message::AddLanguageSearch)
            .on_clear(Message::AddLanguageSearch(String::new()));

        let mut list = widget::list_column();

        let search_input = &self.add_language_search.trim().to_lowercase();

        let svg_accent = Rc::new(|theme: &cosmic::Theme| {
            let color = theme.cosmic().accent_color().into();
            cosmic::widget::svg::Style { color: Some(color) }
        });

        for (id, available_language) in &self.available_languages {
            if search_input.is_empty()
                || available_language
                    .display_name
                    .to_lowercase()
                    .contains(search_input)
            {
                let is_installed = self.config.as_ref().map_or(false, |(_, locales)| {
                    locales.contains(&available_language.lang_code)
                });

                let button = widget::settings::item_row(vec![
                    widget::text::body(&available_language.display_name)
                        .class(if is_installed {
                            cosmic::theme::Text::Accent
                        } else {
                            cosmic::theme::Text::Default
                        })
                        .wrapping(Wrapping::Word)
                        .into(),
                    widget::horizontal_space().width(Length::Fill).into(),
                    if is_installed {
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
                .on_press_maybe(if is_installed {
                    None
                } else {
                    Some(Message::AddLanguage(id))
                });

                list = list.add(button)
            }
        }

        let install_additional_button =
            widget::button::standard(fl!("install-additional-languages"))
                .on_press(Message::InstallAdditionalLanguages);

        widget::column()
            .padding([2, 0])
            .spacing(space_l)
            .push(search)
            .push(list)
            .push(install_additional_button)
            .apply(Element::from)
            .map(crate::pages::Message::Region)
    }

    fn formatted_date(&self) -> String {
        let time_locale = self
            .system_locales
            .get("LC_TIME")
            .or_else(|| self.system_locales.get("LANG"))
            .map_or("en_US", |locale| &locale.lang_code)
            .split('.')
            .next()
            .unwrap_or("en_US");

        let Ok(locale) = icu::locid::Locale::from_str(time_locale) else {
            return String::new();
        };

        let mut bag = Bag::empty();
        bag.day = Some(components::Day::TwoDigitDayOfMonth);
        bag.month = Some(components::Month::TwoDigit);
        bag.year = Some(components::Year::Numeric);

        let options = icu::datetime::DateTimeFormatterOptions::Components(bag);

        let dtf = DateTimeFormatter::try_new_experimental(&locale.into(), options).unwrap();

        let datetime = DateTime::try_new_gregorian_datetime(2024, 1, 1, 12, 0, 0)
            .unwrap()
            .to_iso()
            .to_any();

        dtf.format(&datetime)
            .expect("can't format value")
            .to_string()
    }

    fn formatted_dates_and_times(&self) -> String {
        let time_locale = self
            .system_locales
            .get("LC_TIME")
            .or_else(|| self.system_locales.get("LANG"))
            .map_or("en_US", |locale| &locale.lang_code)
            .split('.')
            .next()
            .unwrap_or("en_US");

        let Ok(locale) = icu::locid::Locale::from_str(time_locale) else {
            return String::new();
        };

        let mut bag = Bag::empty();
        bag.hour = Some(components::Numeric::Numeric);
        bag.minute = Some(components::Numeric::Numeric);
        bag.second = Some(components::Numeric::Numeric);
        bag.preferences = Some(preferences::Bag::from_hour_cycle(
            preferences::HourCycle::H12,
        ));
        // bag.time_zone_name = Some(components::TimeZoneName::ShortSpecific);
        bag.day = Some(components::Day::TwoDigitDayOfMonth);
        bag.month = Some(components::Month::Short);
        bag.year = Some(components::Year::Numeric);

        let options = icu::datetime::DateTimeFormatterOptions::Components(bag);

        let dtf = DateTimeFormatter::try_new_experimental(&locale.into(), options).unwrap();

        let datetime = DateTime::try_new_gregorian_datetime(2024, 1, 1, 12, 0, 0)
            .unwrap()
            .to_iso()
            .to_any();

        dtf.format(&datetime)
            .expect("can't format value")
            .to_string()
    }

    fn formatted_time(&self) -> String {
        let time_locale = self
            .system_locales
            .get("LC_TIME")
            .or_else(|| self.system_locales.get("LANG"))
            .map_or("en_US", |locale| &locale.lang_code)
            .split('.')
            .next()
            .unwrap_or("en_US");

        let Ok(locale) = icu::locid::Locale::from_str(time_locale) else {
            return String::new();
        };

        let mut bag = Bag::empty();
        bag.hour = Some(components::Numeric::Numeric);
        bag.minute = Some(components::Numeric::Numeric);
        bag.second = Some(components::Numeric::Numeric);
        bag.preferences = Some(preferences::Bag::from_hour_cycle(
            preferences::HourCycle::H12,
        ));

        let options = icu::datetime::DateTimeFormatterOptions::Components(bag);

        let dtf = DateTimeFormatter::try_new_experimental(&locale.into(), options).unwrap();

        let datetime = DateTime::try_new_gregorian_datetime(2024, 1, 1, 12, 0, 0)
            .unwrap()
            .to_iso()
            .to_any();

        dtf.format(&datetime)
            .expect("can't format value")
            .to_string()
    }

    fn formatted_numbers(&self) -> String {
        let numerical_locale = self
            .system_locales
            .get("LC_NUMERIC")
            .or_else(|| self.system_locales.get("LANG"))
            .map_or("en_US", |locale| &locale.lang_code)
            .split('.')
            .next()
            .unwrap_or("en_US");

        let Ok(locale) = icu::locid::Locale::from_str(numerical_locale) else {
            return String::new();
        };

        let options = FixedDecimalFormatterOptions::default();
        let formatter = FixedDecimalFormatter::try_new(&locale.into(), options).unwrap();
        let mut value = FixedDecimal::from(123456789);
        value.multiply_pow10(-2);

        formatter.format(&value).to_string()
    }

    fn region_view(&self) -> cosmic::Element<'_, crate::pages::Message> {
        let space_l = theme::active().cosmic().spacing.space_l;

        let search = widget::search_input(fl!("type-to-search"), &self.add_language_search)
            .on_input(Message::AddLanguageSearch)
            .on_clear(Message::AddLanguageSearch(String::new()));

        let mut list = widget::list_column();

        let search_input = &self.add_language_search.trim().to_lowercase();

        for (id, locale) in &self.available_languages {
            if search_input.is_empty() || locale.display_name.to_lowercase().contains(search_input)
            {
                let is_selected = self
                    .region
                    .as_ref()
                    .map_or(false, |l| l.lang_code == locale.lang_code);

                let button =
                    widget::settings::item_row(vec![widget::text::body(&locale.region_name)
                        .class(if is_selected {
                            cosmic::theme::Text::Accent
                        } else {
                            cosmic::theme::Text::Default
                        })
                        .wrapping(Wrapping::Word)
                        .into()])
                    .apply(widget::container)
                    .class(cosmic::theme::Container::List)
                    .apply(widget::button::custom)
                    .class(cosmic::theme::Button::Transparent)
                    .on_press_maybe(if is_selected {
                        None
                    } else {
                        Some(Message::SelectRegion(id))
                    });

                list = list.add(button)
            }
        }

        widget::column()
            .padding([2, 0])
            .spacing(space_l)
            .push(search)
            .push(list)
            .apply(Element::from)
            .map(crate::pages::Message::Region)
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

mod preferred_languages {
    use super::Message;
    use cosmic::{
        iced::{Alignment, Length},
        widget, Apply,
    };
    use cosmic_settings_page::Section;

    pub fn section() -> Section<crate::pages::Message> {
        crate::slab!(descriptions {
            pref_lang_desc = fl!("preferred-languages", "desc");
            add_lang_txt = fl!("add-language");
        });

        Section::default()
            .title(fl!("preferred-languages"))
            .descriptions(descriptions)
            .view::<super::Page>(move |_binder, page, section| {
                let title = widget::text::body(&section.title).font(cosmic::font::bold());

                let description = widget::text::body(&section.descriptions[pref_lang_desc]);

                let mut content = widget::settings::section();

                if let Some(((_config, locales), registry)) =
                    page.config.as_ref().zip(page.registry.as_ref())
                {
                    for (id, locale) in locales.iter().enumerate() {
                        if let Some(locale) = registry.locale(locale) {
                            content = content.add(super::language_element(
                                id,
                                locale.display_name.clone(),
                                page.expanded_source_popover,
                            ));
                        }
                    }
                }

                let add_language_button =
                    widget::button::standard(&section.descriptions[add_lang_txt])
                        .on_press(Message::AddLanguageContext)
                        .apply(widget::container)
                        .width(Length::Fill)
                        .align_x(Alignment::End);

                widget::column::with_capacity(5)
                    .push(title)
                    .push(description)
                    .push(content)
                    .push(
                        widget::vertical_space()
                            .height(cosmic::theme::active().cosmic().spacing.space_xxs),
                    )
                    .push(add_language_button)
                    .spacing(cosmic::theme::active().cosmic().spacing.space_xxs)
                    .apply(cosmic::Element::from)
                    .map(Into::into)
            })
    }
}

mod formatting {
    use super::Message;
    use cosmic::{widget, Apply};
    use cosmic_settings_page::Section;

    pub fn section() -> Section<crate::pages::Message> {
        crate::slab!(descriptions {
            formatting_txt = fl!("formatting");
            dates_txt = fl!("formatting", "dates");
            time_txt = fl!("formatting", "time");
            date_and_time_txt = fl!("formatting", "date-and-time");
            numbers_txt = fl!("formatting", "numbers");
            // measurement_txt = fl!("formatting", "measurement");
            // paper_txt = fl!("formatting", "paper");
            region_txt = fl!("region");
        });

        let dates_label = [&descriptions[dates_txt], ":"].concat();
        let time_label = [&descriptions[time_txt], ":"].concat();
        let date_and_time_label = [&descriptions[date_and_time_txt], ":"].concat();
        let numbers_label = [&descriptions[numbers_txt], ":"].concat();
        // let measurement_label = [&descriptions[measurement_txt], ":"].concat();
        // let paper_label = [&descriptions[paper_txt], ":"].concat();

        Section::default()
            .title(fl!("formatting"))
            .descriptions(descriptions)
            .view::<super::Page>(move |_binder, page, section| {
                let desc = &section.descriptions;

                let dates = widget::row::with_capacity(2)
                    .push(widget::text::body(dates_label.clone()))
                    .push(widget::text::body(page.formatted_date()).font(cosmic::font::bold()))
                    .spacing(4);

                let time = widget::row::with_capacity(2)
                    .push(widget::text::body(time_label.clone()))
                    .push(widget::text::body(page.formatted_time()).font(cosmic::font::bold()))
                    .spacing(4);

                let dates_and_times = widget::row::with_capacity(2)
                    .push(widget::text::body(date_and_time_label.clone()))
                    .push(
                        widget::text::body(page.formatted_dates_and_times())
                            .font(cosmic::font::bold()),
                    )
                    .spacing(4);

                let numbers = widget::row::with_capacity(2)
                    .push(widget::text::body(numbers_label.clone()))
                    .push(widget::text::body(page.formatted_numbers()).font(cosmic::font::bold()))
                    .spacing(4);

                // TODO: Display measurement and paper demos

                // let measurement = widget::row::with_capacity(2)
                //     .push(widget::text::body(measurement_label.clone()))
                //     .push(widget::text::body("").font(cosmic::font::bold()))
                //     .spacing(4);

                // let paper = widget::row::with_capacity(2)
                //     .push(widget::text::body(paper_label.clone()))
                //     .push(widget::text::body("").font(cosmic::font::bold()))
                //     .spacing(4);

                let formatted_demo = widget::column::with_capacity(6)
                    .push(dates)
                    .push(time)
                    .push(dates_and_times)
                    .push(numbers)
                    // .push(measurement)
                    // .push(paper)
                    .spacing(4)
                    .padding(5.0)
                    .apply(|column| widget::settings::item_row(vec![column.into()]));

                let region = page
                    .region
                    .as_ref()
                    .map(|locale| locale.region_name.as_str())
                    .unwrap_or("");

                let select_region = crate::widget::go_next_with_item(
                    &desc[region_txt],
                    widget::text::body(region),
                    Message::RegionContext,
                );

                widget::settings::section()
                    .title(&desc[formatting_txt])
                    .add(formatted_demo)
                    .add(select_region)
                    .apply(cosmic::Element::from)
                    .map(Into::into)
            })
    }
}

struct Registry(locale::Registry);

impl std::fmt::Debug for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registry").finish()
    }
}

pub async fn page_reload() -> eyre::Result<PageRefresh> {
    let conn = zbus::Connection::system()
        .await
        .wrap_err("zbus system connection error")?;

    let registry = locale::Registry::new().wrap_err("failed to get locale registry")?;

    let system_locales: BTreeMap<String, SystemLocale> = locale1::locale1Proxy::new(&conn)
        .await
        .wrap_err("locale1 proxy connect error")?
        .locale()
        .await
        .wrap_err("could not get locale from locale1")?
        .into_iter()
        .filter_map(|expression| {
            let mut fields = expression.split('=');
            let var = fields.next()?;
            let lang_code = fields.next()?;
            let locale = registry.locale(lang_code);
            Some((
                var.to_owned(),
                SystemLocale {
                    lang_code: lang_code.to_owned(),
                    display_name: registry
                        .locale(lang_code)
                        .map_or(String::from(""), |locale| locale.display_name.clone()),
                    region_name: locale.map_or(String::from(""), |locale| {
                        format!(
                            "{} ({})",
                            locale.territory.display_name, locale.language.display_name
                        )
                    }),
                },
            ))
        })
        .collect();

    let config = cosmic_config::Config::new("com.system76.CosmicSettings", 1)
        .ok()
        .map(|context| {
            let locales = context
                .get::<Vec<String>>("system_locales")
                .ok()
                .unwrap_or_else(|| {
                    let current = system_locales
                        .get("LANG")
                        .map_or("en_US.UTF-8", |l| l.lang_code.as_str())
                        .to_owned();

                    vec![current]
                });

            (context, locales)
        });

    let language = system_locales
        .get("LC_ALL")
        .or_else(|| system_locales.get("LANG"))
        .cloned();

    let region = system_locales
        .get("LC_TIME")
        .or_else(|| system_locales.get("LANG"))
        .cloned();

    let mut available_languages = SlotMap::new();

    let output = tokio::process::Command::new("localectl")
        .arg("list-locales")
        .output()
        .await
        .expect("Failed to run localectl");

    let output = String::from_utf8(output.stdout).unwrap_or_default();
    for line in output.lines() {
        if line == "C.UTF-8" {
            continue;
        }

        if let Some(locale) = registry.locale(line) {
            available_languages.insert(SystemLocale {
                lang_code: line.to_owned(),
                display_name: locale.display_name.clone(),
                region_name: format!(
                    "{} ({})",
                    locale.territory.display_name, locale.language.display_name
                ),
            });
        }
    }

    Ok(PageRefresh {
        config,
        registry: Registry(registry),
        language,
        region,
        available_languages,
        system_locales,
    })
}

fn language_element(
    id: usize,
    description: String,
    expanded_source_popover: Option<usize>,
) -> cosmic::Element<'static, Message> {
    let expanded = expanded_source_popover.is_some_and(|expanded_id| expanded_id == id);

    widget::settings::flex_item(description, popover_button(id, expanded)).into()
}

fn popover_button(id: usize, expanded: bool) -> Element<'static, Message> {
    let on_press = Message::ExpandLanguagePopover(if expanded { None } else { Some(id) });

    let button = button::icon(widget::icon::from_name("view-more-symbolic"))
        .extra_small()
        .on_press(on_press);

    if expanded {
        widget::popover(button)
            .popup(popover_menu(id))
            .on_close(Message::ExpandLanguagePopover(None))
            .into()
    } else {
        button.into()
    }
}

fn popover_menu(id: usize) -> Element<'static, Message> {
    widget::column::with_children(vec![
        popover_menu_row(
            id,
            fl!("keyboard-sources", "move-up"),
            SourceContext::MoveUp,
        ),
        cosmic::widget::divider::horizontal::default().into(),
        popover_menu_row(
            id,
            fl!("keyboard-sources", "move-down"),
            SourceContext::MoveDown,
        ),
        cosmic::widget::divider::horizontal::default().into(),
        popover_menu_row(id, fl!("keyboard-sources", "remove"), SourceContext::Remove),
    ])
    .padding(8)
    .width(Length::Shrink)
    .height(Length::Shrink)
    .apply(cosmic::widget::container)
    .class(cosmic::theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        container::Style {
            icon_color: Some(theme.cosmic().background.on.into()),
            text_color: Some(theme.cosmic().background.on.into()),
            background: Some(Color::from(theme.cosmic().background.base).into()),
            border: Border {
                radius: cosmic.corner_radii.radius_m.into(),
                ..Default::default()
            },
            shadow: Default::default(),
        }
    }))
    .into()
}

fn popover_menu_row(
    id: usize,
    label: String,
    message: impl Fn(usize) -> SourceContext + 'static,
) -> cosmic::Element<'static, Message> {
    widget::text::body(label)
        .apply(widget::container)
        .class(cosmic::theme::Container::custom(|theme| {
            widget::container::Style {
                background: None,
                ..widget::container::Catalog::style(theme, &cosmic::theme::Container::List)
            }
        }))
        .apply(button::custom)
        .on_press(())
        .class(theme::Button::Transparent)
        .apply(Element::from)
        .map(move |()| Message::SourceContext(message(id)))
}

pub async fn set_locale(lang: String, region: String) {
    eprintln!("setting locale lang={lang}, region={region}");
    _ = tokio::process::Command::new("localectl")
        .arg("set-locale")
        .args(&[
            ["LANG=", &lang].concat(),
            ["LC_ADDRESS=", &region].concat(),
            ["LC_IDENTIFICATION=", &region].concat(),
            ["LC_MEASUREMENT=", &region].concat(),
            ["LC_MONETARY=", &region].concat(),
            ["LC_NAME=", &region].concat(),
            ["LC_NUMERIC=", &region].concat(),
            ["LC_PAPER=", &region].concat(),
            ["LC_TELEPHONE=", &region].concat(),
            ["LC_TIME=", &region].concat(),
        ])
        .status()
        .await;
}
