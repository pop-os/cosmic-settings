// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::widget::selection_context_item;
use cosmic::app::{ContextDrawer, context_drawer};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, button};
use cosmic::{Apply, Element};
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_page::{self as page, Section, section};
use eyre::Context;
use icu::calendar::types::Weekday;
use icu::calendar::week;
use icu::datetime::input::{Date, DateTime, Time};
use icu::datetime::{DateTimeFormatter, DateTimeFormatterPreferences, fieldsets};
use icu::decimal::DecimalFormatter;
use icu::decimal::input::Decimal;
use icu::locale::Locale;
use locales_rs as locale;
use regex::Regex;
use slotmap::{DefaultKey, SlotMap};

static GNOME_LANGUAGE_SELECTOR: &str = "gnome-language-selector";

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
    language_selector_available: bool,
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
    /// Whether gnome-language-selector is in the path.
    language_selector_available: bool,
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
            .description(fl!("xdg-entry-region-language-comment"))
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
                let search = widget::search_input("", &self.add_language_search)
                    .on_input(Message::AddLanguageSearch)
                    .on_clear(Message::AddLanguageSearch(String::new()))
                    .apply(Element::from)
                    .map(crate::pages::Message::from);
                let drawer = context_drawer(
                    self.add_language_view().map(crate::pages::Message::from),
                    crate::pages::Message::CloseContextDrawer,
                )
                .title(fl!("add-language", "context"))
                .header(search);

                if self.language_selector_available {
                    let install_additional_button =
                        widget::button::standard(fl!("install-additional-languages"))
                            .on_press(Message::InstallAdditionalLanguages)
                            .apply(widget::container)
                            .width(Length::Fill)
                            .align_x(Alignment::End)
                            .apply(Element::from)
                            .map(crate::pages::Message::from);

                    drawer.footer(install_additional_button)
                } else {
                    drawer
                }
            }
            ContextView::Region => {
                let search = widget::search_input("", &self.add_language_search)
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
                        if set_locale(lang, region.clone()).await.is_ok() {
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
                    _ = tokio::process::Command::new(GNOME_LANGUAGE_SELECTOR)
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
                    self.language_selector_available = page_refresh.language_selector_available;
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

                    // Build the LANGUAGE string for AccountsService (colon-separated locales)
                    let language_list = build_language_list(locales);

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

                            // Set the LANGUAGE variable via AccountsService
                            if let Err(why) = set_user_language(language_list).await {
                                tracing::error!(
                                    ?why,
                                    "failed to set user language via AccountsService"
                                );
                            }
                        });
                    }
                }
            }
        }

        cosmic::Task::none()
    }

    fn add_language_view(&self) -> cosmic::Element<'_, crate::pages::Message> {
        let mut list = widget::list_column::with_capacity(self.available_languages.len());
        let search_input = &self.add_language_search.trim().to_lowercase();

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

                list = list.add(selection_context_item(
                    &available_language.display_name,
                    is_installed,
                    if is_installed {
                        Message::RemoveLanguage(id)
                    } else {
                        Message::AddLanguage(id)
                    },
                ))
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
        let mut list = widget::list_column::with_capacity(self.available_languages.len());

        let search_input = &self.add_language_search.trim().to_lowercase();

        for (id, locale) in &self.available_languages {
            if search_input.is_empty() || locale.display_name.to_lowercase().contains(search_input)
            {
                let is_selected = self
                    .region
                    .as_ref()
                    .is_some_and(|l| l.lang_code == locale.lang_code);

                list = list.add(selection_context_item(
                    &locale.region_name,
                    is_selected,
                    if is_selected {
                        None
                    } else {
                        Some(Message::SelectRegion(id))
                    },
                ))
            }
        }

        list.apply(Element::from).map(crate::pages::Message::Region)
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

mod preferred_languages {
    use crate::pages::time::region::localized_iso_codes;

    use super::Message;
    use cosmic::iced::{Alignment, Length};
    use cosmic::{Apply, widget};
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
    let registry = locale::Registry::new().wrap_err("failed to get locale registry")?;

    // Try locale1 first; fall back to config files and environment on systems
    // without systemd (s6/OpenRC).
    let mut system_locales: BTreeMap<String, SystemLocale> = BTreeMap::new();
    if let Ok(conn) = zbus::Connection::system().await {
        if let Ok(proxy) = locale1_zbus::locale1Proxy::new(&conn).await {
            if let Ok(locales) = proxy.locale().await {
                for expression in locales {
                    let mut fields = expression.split('=');
                    if let (Some(var), Some(lang_code)) = (fields.next(), fields.next()) {
                        if let Some(loc) = registry.locale(lang_code) {
                            system_locales.insert(
                                var.to_owned(),
                                localized_locale(&loc, lang_code.to_owned()),
                            );
                        }
                    }
                }
            }
        }
    }
    if system_locales.is_empty() {
        system_locales = fallback_system_locales(&registry);
    }

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

    // Use 'locale -a' instead of 'localectl list-locales' for OpenRC compatibility
    let output_result = tokio::process::Command::new("locale")
        .arg("-a")
        .output()
        .await;

    let locale_list = match output_result {
        Ok(output) => {
            let output_str = String::from_utf8(output.stdout).unwrap_or_default();
            parse_locale_output(&output_str)
        }
        Err(why) => {
            tracing::error!(?why, "failed to list available locales using 'locale -a'");
            Vec::new()
        }
    };

    for line in locale_list {
        if let Some(locale) = registry.locale(&line) {
            available_languages_set.insert(localized_locale(&locale, line));
        }
    }

    let mut available_languages = SlotMap::new();
    for language in available_languages_set {
        available_languages.insert(language);
    }

    let language_selector_available = which::which(GNOME_LANGUAGE_SELECTOR).is_ok();

    Ok(PageRefresh {
        config,
        registry: Registry(registry),
        language,
        region,
        available_languages,
        system_locales,
        language_selector_available,
    })
}

fn fallback_system_locales(registry: &locale::Registry) -> BTreeMap<String, SystemLocale> {
    let mut map = BTreeMap::new();
    let mut insert_expr = |expr: &str| {
        let expr = expr
            .trim()
            .trim_start_matches("export ")
            .trim_matches('"')
            .trim_matches('\'');
        let mut fields = expr.splitn(2, '=');
        if let (Some(var), Some(lang_code)) = (fields.next(), fields.next()) {
            let var = var.trim();
            let lang_code = lang_code.trim().trim_matches('"').trim_matches('\'');
            if !var.is_empty() && !lang_code.is_empty() && !map.contains_key(var) {
                if let Some(loc) = registry.locale(lang_code) {
                    map.insert(var.to_owned(), localized_locale(&loc, lang_code.to_owned()));
                }
            }
        }
    };

    for path in ["/etc/locale.conf", "/etc/default/locale"] {
        if let Ok(content) = std::fs::read_to_string(path) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                insert_expr(line);
            }
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        for path in [
            format!("{home}/.config/locale.conf"),
            format!("{home}/.config/environment.d/90-locale.conf"),
        ] {
            if let Ok(content) = std::fs::read_to_string(&path) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    insert_expr(line);
                }
            }
        }
    }

    for var in [
        "LC_ALL", "LANG", "LC_ADDRESS", "LC_IDENTIFICATION", "LC_MEASUREMENT", "LC_MONETARY",
        "LC_NAME", "LC_NUMERIC", "LC_PAPER", "LC_TELEPHONE", "LC_TIME",
    ] {
        if !map.contains_key(var) {
            if let Ok(val) = std::env::var(var) {
                if !val.is_empty() {
                    if let Some(loc) = registry.locale(&val) {
                        map.insert(var.to_owned(), localized_locale(&loc, val));
                    }
                }
            }
        }
    }

    map
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

/// Sets the system locale via locale1 D-Bus, with a config-file fallback for
/// systems without systemd (s6/OpenRC).
pub async fn set_locale(lang: String, region: String) -> eyre::Result<()> {
    tracing::debug!("setting locale lang={lang}, region={region}");

    let locale_settings = build_locale_settings(&lang, &region);

    if let Ok(conn) = zbus::Connection::system().await {
        if let Ok(proxy) = locale1_zbus::locale1Proxy::new(&conn).await {
            let locale_strs: Vec<&str> = locale_settings.iter().map(|s| s.as_str()).collect();
            if proxy.set_locale(&locale_strs, true).await.is_ok() {
                tracing::debug!("successfully set locale via D-Bus");
                return Ok(());
            }
        }
    }

    // Fallback: sources /etc/locale.conf from /etc/profile.d/locale.sh,
    // so keep the "export" prefix used there. Try sudo, then doas.
    let content = locale_settings
        .iter()
        .map(|s| format!("export {s}\n"))
        .collect::<String>();
    let escaped = shell_escape(&content);
    let cmd = format!(
        "if command -v sudo >/dev/null 2>&1; then printf %s {0} | sudo tee /etc/locale.conf >/dev/null; elif command -v doas >/dev/null 2>&1; then printf %s {0} | doas tee /etc/locale.conf >/dev/null; else exit 1; fi",
        escaped
    );
    let wrote_system = tokio::process::Command::new("/bin/sh")
        .arg("-c")
        .arg(&cmd)
        .status()
        .await
        .map(|s| s.success())
        .unwrap_or(false);

    if wrote_system {
        tracing::debug!("set_locale: wrote /etc/locale.conf");
        return Ok(());
    }

    if let Ok(home) = std::env::var("HOME") {
        let path = format!("{home}/.config/locale.conf");
        if let Some(parent) = std::path::Path::new(&path).parent() {
            let _ = tokio::fs::create_dir_all(parent).await;
        }
        if tokio::fs::write(&path, &content).await.is_ok() {
            tracing::debug!("set_locale: wrote {path}");
            return Ok(());
        }
    }

    tracing::warn!("set_locale: could not persist locale via D-Bus or config files");
    Ok(())
}

fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\"'\"'"))
}

/// Sets the user's preferred language list via AccountsService D-Bus.
/// This updates the LANGUAGE environment variable for gettext-based applications.
/// The language_list should be a colon-separated string like "de_DE:de:en_US:en".
pub async fn set_user_language(language_list: String) -> eyre::Result<()> {
    if let Ok(conn) = zbus::Connection::system().await {
        let uid = rustix::process::getuid().as_raw() as u64;
        if let Ok(user_proxy) = accounts_zbus::UserProxy::from_uid(&conn, uid).await {
            if user_proxy.set_language(&language_list).await.is_ok() {
                tracing::debug!("set user language via AccountsService: {language_list}");
                return Ok(());
            }
        }
    }

    // Fallback: persist LANGUAGE for the next login on non-systemd systems.
    if let Ok(home) = std::env::var("HOME") {
        let env_dir = format!("{home}/.config/environment.d");
        let _ = tokio::fs::create_dir_all(&env_dir).await;
        let path = format!("{env_dir}/90-locale.conf");
        match tokio::fs::write(&path, format!("LANGUAGE={language_list}\n")).await {
            Ok(_) => tracing::debug!("set user language fallback: wrote to {path}"),
            Err(e) => tracing::error!(?e, "set user language fallback failed"),
        }
    }
    Ok(())
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

/// Builds a colon-separated language list for the LANGUAGE environment variable.
/// Converts locales like ["de_DE.UTF-8", "en_US.UTF-8"] to "de_DE:de:en_US:en".
///
/// Important: The list stops at English locales since English is typically the
/// source language and doesn't need translation files. This prevents fallback
/// to other languages when English is selected.
fn build_language_list(locales: &[String]) -> String {
    let mut parts = Vec::new();

    for locale in locales {
        // Parse locale: language_TERRITORY[.CODESET][@MODIFIER]
        // We want to extract "language_TERRITORY" without codeset or modifier
        let base = strip_locale_suffix(locale);

        // Get the language-only code (e.g., "de" from "de_DE")
        let lang = base.split('_').next().unwrap_or(&base);

        // Add the full locale code (e.g., "de_DE")
        parts.push(base.clone());

        // Add the language-only code as fallback if different
        if lang != base {
            parts.push(lang.to_string());
        }

        // Stop after English - it's the source language and needs no translation
        // This matches gnome-language-selector's behavior
        if lang == "en" {
            break;
        }
    }

    parts.join(":")
}

/// Strips the codeset (.UTF-8) and modifier (@latin) from a locale string.
/// "de_DE.UTF-8" -> "de_DE"
/// "sr_RS@latin" -> "sr_RS"
/// "sr_RS.UTF-8@latin" -> "sr_RS"
fn strip_locale_suffix(locale: &str) -> String {
    // First strip the codeset (everything from '.' onwards)
    let without_codeset = locale.split('.').next().unwrap_or(locale);
    // Then strip the modifier (everything from '@' onwards)
    without_codeset
        .split('@')
        .next()
        .unwrap_or(without_codeset)
        .to_string()
}

/// Parses the output from `locale -a` command and returns a vector of locale strings.
/// Filters out pseudo-locales (C, POSIX) and accepts only allowed character encodings.
fn parse_locale_output(output: &str) -> Vec<String> {
    // Regex to match pseudo-locales: C or POSIX, optionally followed by .anything
    let pseudo_locale_re = Regex::new(r"^(C|POSIX)(\.|$)").unwrap();

    // Regex to match UTF-8 encoded locales (case-insensitive)
    // Supports optional modifiers after encoding (e.g., @euro, @valencia)
    let utf8_encoding_re = Regex::new(r"(?i)\.(utf-?8)(@.*)?$").unwrap();

    let mut locales = BTreeSet::new();

    for line in output.lines().map(str::trim) {
        if pseudo_locale_re.is_match(line) || !utf8_encoding_re.is_match(line) {
            continue;
        }
        // Normalize to the .utf8 suffix that locales-rs resolves, so entries
        // from `locale -a` and from the definitions below don't duplicate.
        let base = utf8_encoding_re.replace(line, "").to_string();
        locales.insert(format!("{base}.utf8"));
    }

    // `locale -a` alone would show just a couple of languages. Offer every
    // installed definition as a UTF-8 locale too.
    if let Ok(entries) = std::fs::read_dir("/usr/share/i18n/locales") {
        for entry in entries.flatten() {
            let Ok(name) = entry.file_name().into_string() else {
                continue;
            };
            if name == "C" || name == "POSIX" || name.starts_with("translit_") {
                continue;
            }
            if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                locales.insert(format!("{name}.utf8"));
            }
        }
    }

    locales.into_iter().collect()
}

/// Builds the locale settings array for D-Bus SetLocale call.
/// Sets LANG to the language parameter and all LC_* variables to the region parameter.
fn build_locale_settings(lang: &str, region: &str) -> Vec<String> {
    vec![
        format!("LANG={}", lang),
        format!("LC_ADDRESS={}", region),
        format!("LC_IDENTIFICATION={}", region),
        format!("LC_MEASUREMENT={}", region),
        format!("LC_MONETARY={}", region),
        format!("LC_NAME={}", region),
        format!("LC_NUMERIC={}", region),
        format!("LC_PAPER={}", region),
        format!("LC_TELEPHONE={}", region),
        format!("LC_TIME={}", region),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_locale_output_handles_empty_input() {
        let output = "";
        let result = parse_locale_output(output);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_locale_output_preserves_locale_strings() {
        let output = "en_US.utf8\nde_DE.utf8\nfr_FR.utf8\n";
        let result = parse_locale_output(output);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&"en_US.utf8".to_string()));
    }

    #[test]
    fn test_build_locale_settings_includes_all_lc_variables() {
        let lang = "en_US.UTF-8";
        let region = "de_DE.UTF-8";
        let settings = build_locale_settings(lang, region);

        assert_eq!(settings.len(), 10);
        assert!(settings.contains(&format!("LANG={}", lang)));
        assert!(settings.contains(&format!("LC_ADDRESS={}", region)));
        assert!(settings.contains(&format!("LC_IDENTIFICATION={}", region)));
        assert!(settings.contains(&format!("LC_MEASUREMENT={}", region)));
        assert!(settings.contains(&format!("LC_MONETARY={}", region)));
        assert!(settings.contains(&format!("LC_NAME={}", region)));
        assert!(settings.contains(&format!("LC_NUMERIC={}", region)));
        assert!(settings.contains(&format!("LC_PAPER={}", region)));
        assert!(settings.contains(&format!("LC_TELEPHONE={}", region)));
        assert!(settings.contains(&format!("LC_TIME={}", region)));
    }

    #[test]
    fn test_build_locale_settings_uses_correct_values() {
        let lang = "fr_FR.UTF-8";
        let region = "en_GB.UTF-8";
        let settings = build_locale_settings(lang, region);

        // LANG should use the lang parameter
        assert!(settings.iter().any(|s| s == "LANG=fr_FR.UTF-8"));
        // LC_* variables should use the region parameter
        assert!(settings.iter().any(|s| s == "LC_TIME=en_GB.UTF-8"));
    }

    #[test]
    fn test_parse_locale_output_filters_pseudo_locales() {
        let output = "C\nC.utf8\nC.UTF-8\nPOSIX\nen_US.utf8\nde_DE.UTF-8\n";
        let result = parse_locale_output(output);

        // Should filter out all C and POSIX variants
        assert!(!result.contains(&"C".to_string()));
        assert!(!result.contains(&"C.utf8".to_string()));
        assert!(!result.contains(&"C.UTF-8".to_string()));
        assert!(!result.contains(&"POSIX".to_string()));

        // Should keep actual locales
        assert!(result.contains(&"en_US.utf8".to_string()));
        assert!(result.contains(&"de_DE.UTF-8".to_string()));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_parse_locale_output_accepts_only_utf8_locales() {
        let output =
            "en_US\nen_US.utf8\nen_US.UTF-8\nar_IN\nar_IN.utf8\nde_DE.iso88591\nfr_FR.UTF-8\n";
        let result = parse_locale_output(output);

        // Should accept UTF-8 variants
        assert!(result.contains(&"en_US.utf8".to_string()));
        assert!(result.contains(&"en_US.UTF-8".to_string()));
        assert!(result.contains(&"ar_IN.utf8".to_string()));
        assert!(result.contains(&"fr_FR.UTF-8".to_string()));

        // Should filter out non-UTF-8 encoded locales
        assert!(!result.contains(&"en_US".to_string()));
        assert!(!result.contains(&"ar_IN".to_string()));
        assert!(!result.contains(&"de_DE.iso88591".to_string()));

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_parse_locale_output_filters_any_c_posix_variant() {
        let output = "C\nC.iso88591\nC.anything\nPOSIX\nPOSIX.utf8\nen_US.utf8\n";
        let result = parse_locale_output(output);

        // Should filter out any C or POSIX variant regardless of encoding
        assert!(!result.contains(&"C".to_string()));
        assert!(!result.contains(&"C.iso88591".to_string()));
        assert!(!result.contains(&"C.anything".to_string()));
        assert!(!result.contains(&"POSIX".to_string()));
        assert!(!result.contains(&"POSIX.utf8".to_string()));

        // Should keep actual locales
        assert!(result.contains(&"en_US.utf8".to_string()));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_parse_locale_output_handles_whitespace() {
        let output = "  en_US.utf8  \n\t de_DE.UTF-8\t\n   fr_FR.utf8   \n";
        let result = parse_locale_output(output);

        // Should handle leading/trailing whitespace
        assert!(result.contains(&"en_US.utf8".to_string()));
        assert!(result.contains(&"de_DE.UTF-8".to_string()));
        assert!(result.contains(&"fr_FR.utf8".to_string()));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_parse_locale_output_handles_empty_lines() {
        let output = "en_US.utf8\n\n\nde_DE.UTF-8\n\n";
        let result = parse_locale_output(output);

        // Should skip empty lines
        assert!(result.contains(&"en_US.utf8".to_string()));
        assert!(result.contains(&"de_DE.UTF-8".to_string()));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_parse_locale_output_catalan_not_filtered_as_pseudo() {
        let output = "C\nca_ES.UTF-8\nca_ES.utf8\ncs_CZ.UTF-8\nen_US.utf8\n";
        let result = parse_locale_output(output);

        // Should filter out C but not Catalan (ca_*) or Czech (cs_*)
        assert!(!result.contains(&"C".to_string()));
        assert!(result.contains(&"ca_ES.UTF-8".to_string()));
        assert!(result.contains(&"ca_ES.utf8".to_string()));
        assert!(result.contains(&"cs_CZ.UTF-8".to_string()));
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_parse_locale_output_handles_locale_modifiers() {
        let output = "en_US.UTF-8@euro\nca_ES.UTF-8@valencia\nde_DE.utf8\n";
        let result = parse_locale_output(output);

        // Locales with modifiers should be accepted
        assert!(result.contains(&"en_US.UTF-8@euro".to_string()));
        assert!(result.contains(&"ca_ES.UTF-8@valencia".to_string()));
        assert!(result.contains(&"de_DE.utf8".to_string()));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_parse_locale_output_case_variations() {
        let output = "en_US.UTF-8\nen_US.utf-8\nen_US.utf8\nen_US.UTF8\nde_DE.Utf8\n";
        let result = parse_locale_output(output);

        // All case variations should be accepted (case-insensitive regex)
        assert!(result.contains(&"en_US.UTF-8".to_string()));
        assert!(result.contains(&"en_US.utf-8".to_string()));
        assert!(result.contains(&"en_US.utf8".to_string()));
        assert!(result.contains(&"en_US.UTF8".to_string()));
        assert!(result.contains(&"de_DE.Utf8".to_string()));
        assert_eq!(result.len(), 5);
    }
}
