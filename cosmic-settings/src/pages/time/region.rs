// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;
use std::sync::Arc;

use cosmic::app::{ContextDrawer, context_drawer};
use cosmic::iced::{Alignment, Length};
use cosmic::iced_core::text::Wrapping;
use cosmic::widget::{self, button};
use cosmic::{Apply, Element};
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use eyre::Context;
use icu::{
    calendar::{types::Weekday, week},
    datetime::{
        DateTimeFormatter, DateTimeFormatterPreferences, fieldsets,
        input::{Date, DateTime, Time},
    },
    decimal::{DecimalFormatter, input::Decimal},
    locale::Locale,
};
use locales_rs as locale;
use slotmap::{DefaultKey, SlotMap};

#[derive(Clone, Debug)]
pub enum Message {
    AddLanguage(DefaultKey),
    AddLanguageContext,
    AddLanguageSearch(String),
    ExpandLanguagePopover(Option<usize>),
    InstallAdditionalLanguages,
    SelectRegion(DefaultKey),
    SourceContext(SourceContext),
    Refresh(Arc<eyre::Result<PageRefresh>>),
    RegionContext,
    RemoveLanguage(DefaultKey),
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
pub struct SystemLocale {
    lang_code: String,
    display_name: String,
    region_name: String,
}

impl Eq for SystemLocale {}

impl Ord for SystemLocale {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.display_name.cmp(&other.display_name)
    }
}

impl PartialOrd for SystemLocale {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SystemLocale {
    fn eq(&self, other: &Self) -> bool {
        self.display_name == other.display_name
    }
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
    /// Cached LC_NUMERIC locale in icu locale format.
    numeric_locale: Option<Locale>,
    /// Cached LC_TIME locale in icu locale format.
    time_locale: Option<Locale>,
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

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        cosmic::task::future(async move { Message::Refresh(Arc::new(page_reload().await)) })
    }

    fn on_leave(&mut self) -> cosmic::Task<crate::pages::Message> {
        self.add_language_search = String::new();
        self.available_languages = SlotMap::new();
        self.config = None;
        self.context = None;
        self.expanded_source_popover = None;
        self.language = None;
        self.region = None;
        self.registry = None;
        self.system_locales = BTreeMap::new();
        cosmic::Task::none()
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        Some(match self.context.as_ref()? {
            ContextView::AddLanguage => {
                let search = widget::search_input(fl!("type-to-search"), &self.add_language_search)
                    .on_input(Message::AddLanguageSearch)
                    .on_clear(Message::AddLanguageSearch(String::new()))
                    .apply(Element::from)
                    .map(crate::pages::Message::from);
                let install_additional_button =
                    widget::button::standard(fl!("install-additional-languages"))
                        .on_press(Message::InstallAdditionalLanguages)
                        .apply(widget::container)
                        .width(Length::Fill)
                        .align_x(Alignment::End)
                        .apply(Element::from)
                        .map(crate::pages::Message::from);

                context_drawer(
                    self.add_language_view().map(crate::pages::Message::from),
                    crate::pages::Message::CloseContextDrawer,
                )
                .title(fl!("add-language", "context"))
                .header(search)
                .footer(install_additional_button)
            }
            ContextView::Region => {
                let search = widget::search_input(fl!("type-to-search"), &self.add_language_search)
                    .on_input(Message::AddLanguageSearch)
                    .on_clear(Message::AddLanguageSearch(String::new()))
                    .apply(Element::from)
                    .map(crate::pages::Message::from);

                context_drawer(
                    self.region_view().map(crate::pages::Message::from),
                    crate::pages::Message::CloseContextDrawer,
                )
                .title(fl!("region"))
                .header(search)
            }
        })
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::app::Message> {
        match message {
            Message::AddLanguage(id) => {
                if let Some(language) = self.available_languages.get(id)
                    && let Some((config, locales)) = self.config.as_mut()
                    && !locales.contains(&language.lang_code)
                {
                    locales.push(language.lang_code.clone());
                    _ = config.set("system_locales", &locales);
                }
            }

            Message::RemoveLanguage(id) => {
                if let Some(language) = self.available_languages.remove(id)
                    && let Some((config, locales)) = self.config.as_mut()
                    && let Some(pos) = locales.iter().position(|l| l == &language.lang_code)
                {
                    locales.remove(pos);
                    _ = config.set("system_locales", &locales);
                }
            }

            Message::SelectRegion(id) => {
                if let Some((region, language)) =
                    self.available_languages.get(id).zip(self.language.as_ref())
                {
                    self.region = Some(region.clone());

                    let lang = language.lang_code.clone();
                    let region = region.lang_code.clone();

                    return cosmic::task::future(async move {
                        if let Ok(exit_status) = set_locale(lang, region.clone()).await
                            && exit_status.success()
                        {
                            update_time_settings_after_region_change(region);
                        }

                        Message::Refresh(Arc::new(page_reload().await))
                    });
                }
            }

            Message::AddLanguageContext => {
                self.context = Some(ContextView::AddLanguage);
                return cosmic::Task::done(crate::app::Message::OpenContextDrawer(self.entity));
            }

            Message::AddLanguageSearch(search) => {
                self.add_language_search = search;
            }

            Message::ExpandLanguagePopover(id) => {
                self.expanded_source_popover = id;
            }

            Message::InstallAdditionalLanguages => {
                return cosmic::task::future(async move {
                    _ = tokio::process::Command::new("gnome-language-selector")
                        .status()
                        .await;

                    Message::Refresh(Arc::new(page_reload().await))
                });
            }

            Message::Refresh(result) => match Arc::into_inner(result).unwrap() {
                Ok(page_refresh) => {
                    self.config = page_refresh.config;
                    self.available_languages = page_refresh.available_languages;
                    self.system_locales = page_refresh.system_locales;
                    self.language = page_refresh.language;
                    self.region = page_refresh.region;
                    self.registry = Some(page_refresh.registry.0);
                    self.numeric_locale = self.icu_locale_from_env("LC_NUMERIC");
                    self.time_locale = self.icu_locale_from_env("LC_TIME");
                }

                Err(why) => {
                    tracing::error!(?why, "failed to get locales from the system");
                }
            },

            Message::RegionContext => {
                self.context = Some(ContextView::Region);
                return cosmic::Task::done(crate::app::Message::OpenContextDrawer(self.entity));
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

                    if let Some(language_code) = locales.first()
                        && let Some(language) = self
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

        cosmic::Task::none()
    }

    fn add_language_view(&self) -> cosmic::Element<'_, crate::pages::Message> {
        let mut list = widget::list_column();

        let search_input = &self.add_language_search.trim().to_lowercase();

        let svg_accent = Rc::new(|theme: &cosmic::Theme| cosmic::widget::svg::Style {
            color: Some(theme.cosmic().accent_text_color().into()),
        });

        for (id, available_language) in &self.available_languages {
            if search_input.is_empty()
                || available_language
                    .display_name
                    .to_lowercase()
                    .contains(search_input)
            {
                let is_installed = self
                    .config
                    .as_ref()
                    .is_some_and(|(_, locales)| locales.contains(&available_language.lang_code));

                let button = widget::settings::item_row(vec![
                    widget::text::body(&available_language.display_name)
                        .class(if is_installed {
                            cosmic::theme::Text::Accent
                        } else {
                            cosmic::theme::Text::Default
                        })
                        .wrapping(Wrapping::Word)
                        .width(Length::Fill)
                        .into(),
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
                .on_press(if is_installed {
                    Message::RemoveLanguage(id)
                } else {
                    Message::AddLanguage(id)
                });

                list = list.add(button)
            }
        }

        list.apply(Element::from).map(crate::pages::Message::Region)
    }

    fn icu_locale_from_env(&self, key: &'static str) -> Option<Locale> {
        self.system_locales
            .get(key)
            .or_else(|| self.system_locales.get("LANG"))
            .map_or("en-US", |locale| &locale.lang_code)
            .split('.')
            .next()
            .unwrap_or("en-US")
            .replacen('_', "-", 1)
            .parse::<Locale>()
            .ok()
    }

    fn formatted_date(&self) -> String {
        let Some(locale) = self.time_locale.as_ref() else {
            return String::new();
        };

        let prefs = DateTimeFormatterPreferences::from(locale);
        let dtf = DateTimeFormatter::try_new(prefs, fieldsets::YMD::medium()).unwrap();

        let datetime = DateTime {
            date: Date::try_new_gregorian(1776, 7, 4).unwrap(),
            time: Time::try_new(12, 0, 0, 0).unwrap(),
        };

        dtf.format(&datetime).to_string()
    }

    fn formatted_dates_and_times(&self) -> String {
        let Some(locale) = self.time_locale.as_ref() else {
            return String::new();
        };

        let prefs = DateTimeFormatterPreferences::from(locale);
        let dtf = DateTimeFormatter::try_new(prefs, fieldsets::YMDT::long()).unwrap();

        let datetime = DateTime {
            date: Date::try_new_gregorian(1776, 7, 4).unwrap(),
            time: Time::try_new(13, 0, 0, 0).unwrap(),
        };

        dtf.format(&datetime).to_string()
    }

    fn formatted_time(&self) -> String {
        let Some(locale) = self.time_locale.as_ref() else {
            return String::new();
        };

        let prefs = DateTimeFormatterPreferences::from(locale);
        let dtf = DateTimeFormatter::try_new(prefs, fieldsets::T::medium()).unwrap();

        let datetime = DateTime {
            date: Date::try_new_gregorian(1776, 7, 4).unwrap(),
            time: Time::try_new(13, 0, 0, 0).unwrap(),
        };

        dtf.format(&datetime).to_string()
    }

    fn formatted_numbers(&self) -> String {
        let Some(locale) = self.numeric_locale.as_ref() else {
            return String::new();
        };

        let formatter = DecimalFormatter::try_new(locale.into(), Default::default()).unwrap();
        let mut value = Decimal::from(123456789);
        value.multiply_pow10(-2);

        formatter.format(&value).to_string()
    }

    fn region_view(&self) -> cosmic::Element<'_, crate::pages::Message> {
        let svg_accent = Rc::new(|theme: &cosmic::Theme| {
            let color = theme.cosmic().accent_text_color().into();
            cosmic::widget::svg::Style { color: Some(color) }
        });

        let mut list = widget::list_column();

        let search_input = &self.add_language_search.trim().to_lowercase();

        for (id, locale) in &self.available_languages {
            if search_input.is_empty() || locale.display_name.to_lowercase().contains(search_input)
            {
                let is_selected = self
                    .region
                    .as_ref()
                    .is_some_and(|l| l.lang_code == locale.lang_code);

                let button = widget::settings::item_row(vec![
                    widget::text::body(&locale.region_name)
                        .class(if is_selected {
                            cosmic::theme::Text::Accent
                        } else {
                            cosmic::theme::Text::Default
                        })
                        .wrapping(Wrapping::Word)
                        .width(Length::Fill)
                        .into(),
                    if is_selected {
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
                .on_press_maybe(if is_selected {
                    None
                } else {
                    Some(Message::SelectRegion(id))
                });

                list = list.add(button)
            }
        }

        list.apply(Element::from).map(crate::pages::Message::Region)
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

mod preferred_languages {
    use crate::pages::time::region::localized_iso_codes;

    use super::Message;
    use cosmic::{
        Apply,
        iced::{Alignment, Length},
        widget,
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
                            let (language, country) = localized_iso_codes(&locale);

                            content = content.add(super::language_element(
                                id,
                                format!("{} ({})", language, country),
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
                    .push(add_language_button)
                    .spacing(cosmic::theme::spacing().space_xxs)
                    .apply(cosmic::Element::from)
                    .map(Into::into)
            })
    }
}

mod formatting {
    use super::Message;
    use cosmic::{Apply, widget};
    use cosmic_settings_page::Section;

    pub fn section() -> Section<crate::pages::Message> {
        crate::slab!(descriptions {
            formatting_txt = fl!("formatting");
            dates_txt = [&fl!("formatting", "dates"), ":"].concat();
            time_txt = [&fl!("formatting", "time"), ":"].concat();
            date_and_time_txt = [&fl!("formatting", "date-and-time"), ":"].concat();
            numbers_txt = [&fl!("formatting", "numbers"), ":"].concat();
            region_txt = fl!("region");
        });

        Section::default()
            .title(fl!("formatting"))
            .descriptions(descriptions)
            .view::<super::Page>(move |_binder, page, section| {
                let desc = &section.descriptions;

                let dates = widget::row::with_capacity(2)
                    .push(widget::text::body(&desc[dates_txt]))
                    .push(widget::text::body(page.formatted_date()).font(cosmic::font::bold()))
                    .spacing(4);

                let time = widget::row::with_capacity(2)
                    .push(widget::text::body(&desc[time_txt]))
                    .push(widget::text::body(page.formatted_time()).font(cosmic::font::bold()))
                    .spacing(4);

                let dates_and_times = widget::row::with_capacity(2)
                    .push(widget::text::body(&desc[date_and_time_txt]))
                    .push(
                        widget::text::body(page.formatted_dates_and_times())
                            .font(cosmic::font::bold()),
                    )
                    .spacing(4);

                let numbers = widget::row::with_capacity(2)
                    .push(widget::text::body(&desc[numbers_txt]))
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
            let locale = registry.locale(lang_code)?;

            Some((
                var.to_owned(),
                localized_locale(&locale, lang_code.to_owned()),
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

    let mut available_languages_set = BTreeSet::new();

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
            available_languages_set.insert(localized_locale(&locale, line.to_owned()));
        }
    }

    let mut available_languages = SlotMap::new();
    for language in available_languages_set {
        available_languages.insert(language);
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

    widget::settings::item(description, popover_button(id, expanded)).into()
}

fn localized_iso_codes(locale: &locale::Locale) -> (String, String) {
    let mut language = gettextrs::dgettext("iso_639", &locale.language.display_name);
    let country = gettextrs::dgettext("iso_3166", &locale.territory.display_name);

    // Ensure language is title-cased.
    let mut chars = language.chars();
    if let Some(c) = chars.next() {
        language = c.to_uppercase().collect::<String>() + chars.as_str();
    }

    (language, country)
}

fn localized_locale(locale: &locale::Locale, lang_code: String) -> SystemLocale {
    let (language, country) = localized_iso_codes(locale);

    SystemLocale {
        lang_code,
        display_name: format!("{language} ({country})"),
        region_name: format!("{country} ({language})"),
    }
}

fn popover_button(id: usize, expanded: bool) -> Element<'static, Message> {
    let on_press = Message::ExpandLanguagePopover(if expanded { None } else { Some(id) });

    let button = button::icon(widget::icon::from_name("view-more-symbolic"))
        .extra_small()
        .on_press(on_press);

    if expanded {
        widget::popover(button)
            .position(widget::popover::Position::Bottom)
            .popup(popover_menu(id))
            .on_close(Message::ExpandLanguagePopover(None))
            .into()
    } else {
        button.into()
    }
}

fn popover_menu(id: usize) -> Element<'static, Message> {
    widget::column::with_children([
        popover_menu_row(
            id,
            fl!("keyboard-sources", "move-up"),
            SourceContext::MoveUp,
        ),
        widget::divider::horizontal::default()
            .apply(widget::container)
            .padding([0, 8])
            .into(),
        popover_menu_row(
            id,
            fl!("keyboard-sources", "move-down"),
            SourceContext::MoveDown,
        ),
        widget::divider::horizontal::default()
            .apply(widget::container)
            .padding([0, 8])
            .into(),
        popover_menu_row(id, fl!("keyboard-sources", "remove"), SourceContext::Remove),
    ])
    .width(Length::Fixed(200.0))
    .apply(widget::container)
    .padding(cosmic::theme::spacing().space_xxs)
    .class(cosmic::theme::Container::Dropdown)
    .into()
}

fn popover_menu_row(
    id: usize,
    label: String,
    message: impl Fn(usize) -> SourceContext + 'static,
) -> cosmic::Element<'static, Message> {
    let spacing = cosmic::theme::spacing();
    widget::text::body(label)
        .align_y(Alignment::Center)
        .apply(button::custom)
        .padding([spacing.space_xxxs, spacing.space_xs])
        .width(Length::Fill)
        .class(cosmic::theme::Button::MenuItem)
        .on_press(Message::SourceContext(message(id)))
        .apply(Element::from)
}

pub async fn set_locale(
    lang: String,
    region: String,
) -> Result<std::process::ExitStatus, std::io::Error> {
    eprintln!("setting locale lang={lang}, region={region}");
    tokio::process::Command::new("localectl")
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
        .await
}

fn parse_locale(locale: &str) -> Option<Locale> {
    locale
        .split('.')
        .next()?
        .replacen('_', "-", 1)
        .parse::<Locale>()
        .ok()
}

fn get_default_24h(locale: &str) -> bool {
    let Some(locale) = parse_locale(locale) else {
        return false;
    };

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

fn get_default_first_day(locale: &str) -> usize {
    let Some(locale) = parse_locale(locale) else {
        return 6;
    };
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

fn update_time_settings_after_region_change(region: String) {
    // Create the same config that date.rs uses
    let cosmic_applet_config = match cosmic_config::Config::new("com.system76.CosmicAppletTime", 1)
    {
        Ok(config) => config,
        Err(err) => {
            tracing::error!(
                ?err,
                "Failed to create cosmic applet config for time settings"
            );
            return;
        }
    };

    // Update military_time based on new locale
    let new_military_time = get_default_24h(&region);
    if let Err(why) = cosmic_applet_config.set("military_time", new_military_time) {
        tracing::error!(?why, "Failed to update military_time after region change");
    }

    // Update first_day_of_week based on new locale
    let new_first_day = get_default_first_day(&region);
    if let Err(why) = cosmic_applet_config.set("first_day_of_week", new_first_day) {
        tracing::error!(
            ?why,
            "Failed to update first_day_of_week after region change"
        );
    }
}
