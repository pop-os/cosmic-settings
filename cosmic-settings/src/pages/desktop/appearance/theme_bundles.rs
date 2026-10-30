use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::time::Duration;

use cosmic::cosmic_config::{self, Config, ConfigGet, CosmicConfigEntry};
use cosmic::cosmic_theme::Theme;
use cosmic::cosmic_theme::palette::Srgba;
use cosmic::iced::Radius;
use cosmic::iced::alignment::Vertical;
use cosmic::iced::core::{Alignment, Background, Border, Length};
use cosmic::iced::widget::Stack;
use cosmic::widget::text_input::{focus, select_all};
use cosmic::widget::{self, button, column, container, icon, row, space, text};
use cosmic::{Apply, Element, Task};
use cosmic_settings_page::Section;
use cosmic_settings_wallpaper as wallpaper;
use cosmic_theme_bundle_config::{Bundle, Mode, Part, Store};

use super::{Message, Page};
use crate::pages::desktop::wallpaper::widgets::color_image;

pub(crate) static SAVE_AS_INPUT: LazyLock<widget::Id> =
    LazyLock::new(|| widget::Id::new("theme-bundle-save-as-name"));
pub(crate) static RENAME_INPUT: LazyLock<widget::Id> =
    LazyLock::new(|| widget::Id::new("theme-bundle-rename-name"));

const CARD_WIDTH: f32 = 218.0;
const CARD_HEIGHT: f32 = 152.0;

const BAND_HEIGHT: f32 = 44.0;

const LINE_WIDTH: u16 = 80;
const LINE_HEIGHT: u16 = 4;
const LINE_RADIUS: f32 = 2.0;
const LINE_SPACING: u16 = 4;

const CIRCLE_SIZE: u16 = 20;

#[derive(Debug, Clone)]
pub enum Preview {
    Image(cosmic::widget::image::Handle),
    Wallpaper(PathBuf, wallpaper::Color),
    Color(wallpaper::Color),
}

#[derive(Debug, Clone)]
pub struct Card {
    pub slug: String,
    pub name: String,
    pub system: bool,
    pub background: cosmic::iced::Color,
    pub on_background: cosmic::iced::Color,
    pub accent: cosmic::iced::Color,
    pub preview: Preview,
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub mode: Mode,
    pub live_name: String,
    pub live: Option<Card>,
    pub cards: Vec<Card>,
}

impl Deck {
    fn removable(&self) -> Option<&Card> {
        if self.cards.len() <= 1 {
            return None;
        }
        self.cards
            .iter()
            .find(|card| !card.system && card.name == self.live_name)
    }

    fn active(&self) -> Option<usize> {
        self.cards
            .iter()
            .position(|card| card.name == self.live_name)
    }

    fn active_card(&self) -> Option<&Card> {
        self.cards
            .iter()
            .find(|card| !card.system && !card.slug.is_empty() && card.name == self.live_name)
    }

    fn row(&self) -> Vec<(&Card, bool)> {
        let active = self.active();
        let mut row: Vec<(&Card, bool)> = self
            .cards
            .iter()
            .enumerate()
            .map(|(index, card)| (card, active == Some(index)))
            .collect();

        if active.is_none()
            && let Some(live) = self.live.as_ref()
        {
            let at = row
                .iter()
                .position(|(card, _)| card.name > live.name)
                .unwrap_or(row.len());
            row.insert(at, (live, true));
        }

        row
    }
}

pub fn load() -> Vec<Deck> {
    match Store::new() {
        Ok(store) => {
            if let Err(error) = store.migrate_toolkit() {
                tracing::warn!(
                    ?error,
                    "failed to copy the toolkit settings into the themes"
                );
            }
            if let Err(error) = store.migrate_names() {
                tracing::warn!(?error, "failed to rename the built-in themes");
            }
            if let Err(error) = store.normalize_names() {
                tracing::warn!(?error, "failed to make theme bundle names unique");
            }
            decks(&store)
        }
        Err(error) => {
            tracing::warn!(?error, "no theme bundle store");
            Vec::new()
        }
    }
}

fn decks(store: &Store) -> Vec<Deck> {
    let mut bundles = store.bundles().unwrap_or_else(|error| {
        tracing::warn!(?error, "failed to list theme bundles");
        Vec::new()
    });

    let live_looks: Vec<Bundle> = [Mode::Dark, Mode::Light]
        .into_iter()
        .map(|mode| Bundle {
            slug: String::new(),
            name: live_name(store, mode),
            mode,
            package: None,
        })
        .collect();

    let installed = store.system_bundles().unwrap_or_else(|error| {
        tracing::warn!(?error, "failed to list system theme bundles");
        Vec::new()
    });
    for bundle in installed.iter() {
        if shadowed(bundle, &bundles) || shadowed(bundle, &live_looks) {
            continue;
        }
        bundles.push(bundle.clone());
    }

    bundles.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.slug.cmp(&b.slug)));

    [Mode::Dark, Mode::Light]
        .into_iter()
        .map(|mode| {
            let live_name = live_name(store, mode);

            Deck {
                mode,
                live: Some(live_card(store, mode)),
                cards: bundles
                    .iter()
                    .filter(|bundle| bundle.mode == mode)
                    .map(|bundle| {
                        let on_screen = bundle.name == live_name;
                        card(store, bundle, shadowed(bundle, &installed), on_screen, mode)
                    })
                    .collect(),
                live_name,
            }
        })
        .collect()
}

fn shadowed(installed: &Bundle, stored: &[Bundle]) -> bool {
    stored.iter().any(|stored| {
        stored.slug == installed.slug
            || (stored.mode == installed.mode && stored.name == installed.name)
    })
}

fn live_card(store: &Store, mode: Mode) -> Card {
    let theme = store
        .live_config(Part::Theme, mode)
        .map(|config| read_theme(&config))
        .unwrap_or_default();

    Card {
        slug: String::new(),
        name: theme.name.clone(),
        system: false,
        background: theme.bg_color().into(),
        on_background: theme.on_bg_color().into(),
        accent: theme.accent_color().into(),
        preview: preview(
            store.live_config(Part::Background, mode).ok(),
            theme.bg_color(),
        ),
    }
}

fn live_name(store: &Store, mode: Mode) -> String {
    store
        .live_config(Part::Theme, mode)
        .map(|config| read_theme(&config).name)
        .unwrap_or_default()
}

pub fn on_screen_name(mode: Mode) -> Option<String> {
    let store = Store::new().ok()?;
    Some(live_name(&store, mode))
}

pub fn export_live(path: &Path, mode: Mode) -> Result<String, String> {
    let store = Store::new().map_err(|error| error.to_string())?;
    store
        .export_live_archive(mode, path)
        .map(|manifest| manifest.name)
        .map_err(|error| error.to_string())
}

pub fn reset_to_stored(mode: Mode) -> Result<bool, String> {
    let store = Store::new().map_err(|error| error.to_string())?;
    store.reset_live(mode).map_err(|error| error.to_string())
}

pub fn reset_to_installed(mode: Mode) -> Result<(), String> {
    let store = Store::new().map_err(|error| error.to_string())?;
    store
        .reset_live_to_installed(mode)
        .map_err(|error| error.to_string())
}

pub fn on_screen_mode() -> Mode {
    Store::new().map_or(Mode::Dark, |store| store.current_mode())
}

pub fn sync_active() -> Result<(), String> {
    sync_active_mode(on_screen_mode())
}

pub fn sync_active_mode(mode: Mode) -> Result<(), String> {
    let store = Store::new().map_err(|error| error.to_string())?;
    store
        .sync_active(mode)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

pub fn sync_active_background() -> Result<(), String> {
    sync_active_background_mode(on_screen_mode())
}

pub fn sync_active_background_mode(mode: Mode) -> Result<(), String> {
    let store = Store::new().map_err(|error| error.to_string())?;
    store
        .sync_active_background(mode)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

pub fn display_background(mode: Mode) -> Option<cosmic_bg_config::Context> {
    Store::new().ok()?.display_background(mode).ok()
}

pub fn activate(slug: &str) -> Result<(), String> {
    let store = Store::new().map_err(|error| error.to_string())?;
    activate_in(&store, slug)
}

fn activate_in(store: &Store, slug: &str) -> Result<(), String> {
    if store.bundle(slug).is_err() {
        store.adopt(slug).map_err(|error| error.to_string())?;
    }

    store
        .activate(slug)
        .map(|_bundle| ())
        .map_err(|error| error.to_string())
}

pub fn delete(slug: &str) -> Result<(), String> {
    let store = Store::new().map_err(|error| error.to_string())?;
    store.delete(slug).map_err(|error| error.to_string())
}

#[derive(Debug, Default, Clone)]
pub struct Rename {
    pub slug: String,
    pub name: String,
}

#[derive(Debug, Default, Clone)]
pub struct Import {
    pub name: String,
    pub path: Option<PathBuf>,
}

impl Import {
    pub fn ready(&self) -> bool {
        self.path.is_some()
    }
}

pub fn import_archive(path: &Path, name: Option<&str>) -> Result<String, String> {
    let store = Store::new().map_err(|error| error.to_string())?;
    store
        .import_archive(path, name)
        .map(|bundle| bundle.name)
        .map_err(|error| error.to_string())
}

pub fn rename(slug: &str, name: &str) -> Result<String, String> {
    let store = Store::new().map_err(|error| error.to_string())?;
    store
        .rename(slug, name)
        .map(|bundle| bundle.name)
        .map_err(|error| error.to_string())
}

pub fn rename_dialog(rename: &Rename) -> Element<'_, Message> {
    let input = widget::text_input("", rename.name.as_str())
        .id(RENAME_INPUT.clone())
        .label(fl!("theme-bundles-import-input-label"))
        .on_input(Message::RenameNameChanged)
        .on_submit_maybe((!rename.name.trim().is_empty()).then_some(|_| Message::RenameConfirm));

    let primary = button::suggested(fl!("rename"))
        .on_press_maybe((!rename.name.trim().is_empty()).then_some(Message::RenameConfirm));
    let secondary = button::standard(fl!("cancel")).on_press(Message::RenameCancel);

    widget::dialog()
        .title(fl!("theme-bundles-rename-title"))
        .control(input)
        .primary_action(primary)
        .secondary_action(secondary)
        .apply(Element::from)
}

pub fn save_live_as(mode: Mode, name: &str) -> Result<String, String> {
    let store = Store::new().map_err(|error| error.to_string())?;
    store
        .save_live_as(mode, name)
        .map(|bundle| bundle.name)
        .map_err(|error| error.to_string())
}

pub fn save_as_dialog(name: &str) -> Element<'_, Message> {
    let input = widget::text_input("", name)
        .id(SAVE_AS_INPUT.clone())
        .label(fl!("theme-bundles-import-input-label"))
        .on_input(Message::SaveAsNameChanged)
        .on_submit_maybe((!name.trim().is_empty()).then_some(|_| Message::SaveAsConfirm));

    let primary = button::suggested(fl!("save"))
        .on_press_maybe((!name.trim().is_empty()).then_some(Message::SaveAsConfirm));
    let secondary = button::standard(fl!("cancel")).on_press(Message::SaveAsCancel);

    widget::dialog()
        .title(fl!("theme-bundles-save-as-title"))
        .control(input)
        .primary_action(primary)
        .secondary_action(secondary)
        .apply(Element::from)
}

pub(crate) fn focus_input(id: widget::Id, select: bool) -> Task<crate::app::Message> {
    // Give the dialog a frame to enter the widget tree before asking for focus.
    cosmic::task::future(async {
        tokio::time::sleep(Duration::from_millis(10)).await;
    })
    .then(move |()| {
        let focus = focus::<Message>(id.clone());
        if select {
            focus.chain(select_all::<Message>(id.clone()))
        } else {
            focus
        }
    })
    .map(crate::app::Message::from)
}

pub fn import_dialog(import: &Import) -> Element<'_, Message> {
    let spacing = cosmic::theme::spacing();

    let name = widget::text_input("", import.name.as_str())
        .label(fl!("theme-bundles-import-input-label"))
        .on_input(Message::ImportNameChanged)
        .on_submit(|_| Message::ImportConfirm);

    let chosen = import.path.as_ref().map(|path| {
        path.file_name().map_or_else(
            || text(path.display().to_string()),
            |name| text(name.to_string_lossy().into_owned()),
        )
    });

    let choose = button::text(fl!("theme-bundles-import-file"))
        .trailing_icon(icon::from_name("window-pop-out-symbolic"))
        .class(button::ButtonClass::Link)
        .on_press(Message::ImportChooseFile)
        .apply(container)
        .width(Length::Fill);

    let control = column::with_capacity(2)
        .push(name)
        .push(
            row::with_capacity(4)
                .push_maybe(chosen)
                .push(space::horizontal().width(spacing.space_s))
                .push(choose)
                .align_y(Vertical::Center)
                .spacing(spacing.space_xxxs),
        )
        .spacing(spacing.space_s)
        .width(Length::Fill);

    let primary = button::suggested(fl!("import"))
        .on_press_maybe(import.ready().then_some(Message::ImportConfirm));
    let secondary = button::standard(fl!("cancel")).on_press(Message::ImportCancel);

    widget::dialog()
        .title(fl!("theme-bundles-import-title"))
        .control(control)
        .primary_action(primary)
        .secondary_action(secondary)
        .apply(Element::from)
}

fn card(store: &Store, bundle: &Bundle, system: bool, on_screen: bool, mode: Mode) -> Card {
    let theme = if on_screen {
        store
            .live_config(Part::Theme, mode)
            .map(|config| read_theme(&config))
            .unwrap_or_default()
    } else {
        store.bundle_theme(&bundle.slug).unwrap_or_else(|error| {
            tracing::warn!(?error, slug = %bundle.slug, "error reading theme bundle");
            Theme::default()
        })
    };

    let background = if on_screen && mode == store.current_mode() {
        store.live_config(Part::Background, mode).ok()
    } else {
        store.bundle_config(&bundle.slug, Part::Background).ok()
    };

    Card {
        slug: bundle.slug.clone(),
        name: bundle.name.clone(),
        system,
        background: theme.bg_color().into(),
        on_background: theme.on_bg_color().into(),
        accent: theme.accent_color().into(),
        preview: preview(background, theme.bg_color()),
    }
}

fn read_theme(config: &Config) -> Theme {
    match Theme::get_entry(config) {
        Ok(theme) => theme,
        Err((errors, theme)) => {
            if let Some(error) = errors.into_iter().find(cosmic_config::Error::is_err) {
                tracing::warn!(?error, "error reading theme bundle");
            }
            theme
        }
    }
}

fn preview(config: Option<Config>, fallback: Srgba) -> Preview {
    let source = config
        .and_then(|config| {
            config
                .get::<cosmic_bg_config::Entry>(cosmic_bg_config::DEFAULT_BACKGROUND)
                .ok()
        })
        .map(|entry| entry.source);

    match source {
        Some(cosmic_bg_config::Source::Path(path)) if path.exists() => {
            Preview::Wallpaper(path, colored(fallback.into()))
        }
        Some(cosmic_bg_config::Source::Color(color)) => Preview::Color(color),
        Some(cosmic_bg_config::Source::Path(_)) | None => Preview::Color(colored(fallback.into())),
    }
}

#[allow(clippy::too_many_lines)]
pub fn section(mode: Mode) -> Section<crate::pages::Message> {
    let title = match mode {
        Mode::Dark => fl!("theme-bundles-dark"),
        Mode::Light => fl!("theme-bundles-light"),
    };

    Section::default()
        .title(title)
        .view::<Page>(move |_binder, page, section| {
            let spacing = cosmic::theme::spacing();
            let deck = page.decks.iter().find(|deck| deck.mode == mode);

            let body: Element<'_, crate::pages::Message> = match deck {
                Some(deck) => {
                    let activating = page.activating.is_some();
                    let cards = deck
                        .row()
                        .into_iter()
                        .map(|(card, active)| card_button(card, active, activating))
                        .collect();

                    container(widget::flex_row(cards).spacing(spacing.space_s))
                        .center_x(Length::Fill)
                        .apply(Element::from)
                }
                None => container(text::body(fl!("theme-bundles-empty")))
                    .apply(Element::from)
                    .map(crate::pages::Message::Appearance),
            };

            let body = if mode == Mode::Light {
                column::with_capacity(3)
                    .push(body)
                    .push(space::vertical().height(spacing.space_m))
                    .push(
                        button::standard(fl!("theme-bundles-import"))
                            .on_press(crate::pages::Message::Appearance(Message::BundleImport)),
                    )
                    .apply(Element::from)
            } else {
                body
            };

            let mut header = column::with_capacity(4)
                .push(title_row(section).map(crate::pages::Message::Appearance));

            if let Some(deck) = deck {
                header = header
                    .push(space::vertical().height(spacing.space_xxs))
                    .push(widget::divider::horizontal::default())
                    .push(space::vertical().height(spacing.space_xxxs))
                    .push(active_row(deck, mode))
                    .push(space::vertical().height(spacing.space_xxxs))
                    .push(widget::divider::horizontal::default());
            }

            column::with_capacity(2)
                .push(header)
                .push(space::vertical().height(spacing.space_xxs))
                .push(body)
                .apply(Element::from)
        })
}

fn title_row<'a>(section: &'a Section<crate::pages::Message>) -> Element<'a, Message> {
    row::with_capacity(1)
        .push(text::title4(section.title.as_str()))
        .align_y(Alignment::Center)
        .apply(Element::from)
}

fn active_row<'a>(deck: &'a Deck, mode: Mode) -> Element<'a, crate::pages::Message> {
    let spacing = cosmic::theme::spacing();

    let name = if deck.live_name.is_empty() {
        match deck.mode {
            Mode::Dark => fl!("dark"),
            Mode::Light => fl!("light"),
        }
    } else {
        deck.live_name.clone()
    };

    let rename = deck.active_card().map(|card| {
        button::icon(icon::from_name("edit-symbolic")).on_press(crate::pages::Message::Appearance(
            Message::RenameBundle(card.slug.clone(), card.name.clone()),
        ))
    });

    let remove = deck.removable().map(|card| {
        button::icon(icon::from_name("edit-delete-symbolic")).on_press(
            crate::pages::Message::Appearance(Message::DeleteBundle(card.slug.clone())),
        )
    });

    let edit = button::custom(
        row::with_capacity(2)
            .push(text::body(fl!("edit")))
            .push(icon::from_name("go-next-symbolic").size(16).icon())
            .align_y(Alignment::Center)
            .spacing(spacing.space_xxxs),
    )
    .class(button::ButtonClass::Link)
    .padding([0, spacing.space_s])
    .on_press(crate::pages::Message::AppearanceTheme(Message::Open(mode)));

    let gap = |width: u16| space::horizontal().width(Length::Fixed(f32::from(width)));

    let mut row = row::with_capacity(5)
        .push(text::body(name))
        .push(gap(spacing.space_s));

    if let Some(rename) = rename {
        row = row.push(rename).push(gap(spacing.space_xxs));
    }

    if let Some(remove) = remove {
        row = row.push(remove).push(gap(spacing.space_xs));
    }

    row.push(edit)
        .align_y(Alignment::Center)
        .apply(Element::from)
}

fn card_button<'a>(
    card: &'a Card,
    selected: bool,
    activating: bool,
) -> Element<'a, crate::pages::Message> {
    let preview = Stack::new()
        .width(Length::Fixed(CARD_WIDTH))
        .height(Length::Fixed(CARD_HEIGHT))
        .push(backdrop(card))
        .push(
            container(swatch(card))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_y(Alignment::End),
        );

    let press: Option<crate::pages::Message> = if activating {
        None
    } else {
        (!selected && !card.slug.is_empty())
            .then(|| crate::pages::Message::Appearance(Message::ActivateBundle(card.slug.clone())))
    };

    let image = button::custom_image_button(preview, None)
        .padding(0)
        .force_enabled(!activating)
        .width(Length::Fixed(CARD_WIDTH))
        .height(Length::Fixed(CARD_HEIGHT))
        .selected(selected)
        .class(button::ButtonClass::Image)
        .on_press_maybe(press);

    image.apply(Element::from)
}

fn backdrop<'a>(card: &'a Card) -> Element<'a, crate::pages::Message> {
    let radius = cosmic::theme::active().cosmic().corner_radii.radius_s;

    match &card.preview {
        Preview::Image(handle) => widget::image(handle.clone())
            .width(Length::Fixed(CARD_WIDTH))
            .height(Length::Fixed(CARD_HEIGHT))
            .content_fit(cosmic::iced::ContentFit::Cover)
            .border_radius(radius)
            .into(),
        Preview::Wallpaper(_, color) | Preview::Color(color) => {
            color_image(color.clone(), CARD_WIDTH as u16, CARD_HEIGHT as u16, None)
        }
    }
}

fn swatch<'a>(card: &'a Card) -> Element<'a, crate::pages::Message> {
    let spacing = cosmic::theme::spacing();

    let lines = column::with_capacity(3)
        .push(color_image(
            colored(card.on_background),
            LINE_WIDTH,
            LINE_HEIGHT,
            Some(LINE_RADIUS),
        ))
        .push(color_image(
            colored(card.on_background),
            LINE_WIDTH,
            LINE_HEIGHT,
            Some(LINE_RADIUS),
        ))
        .push(color_image(
            colored(card.on_background),
            LINE_WIDTH,
            LINE_HEIGHT,
            Some(LINE_RADIUS),
        ))
        .spacing(f32::from(LINE_SPACING));

    row::with_capacity(5)
        .push(space::horizontal())
        .push(lines)
        .push(space::horizontal().width(Length::Fixed(8.)))
        .push(color_image(
            colored(card.accent),
            CIRCLE_SIZE * 2,
            CIRCLE_SIZE,
            Some(f32::from(CIRCLE_SIZE) / 2.0),
        ))
        .push(space::horizontal())
        .align_y(Alignment::Center)
        .padding([spacing.space_xxs, spacing.space_xs])
        .width(Length::Fill)
        .height(Length::Fixed(BAND_HEIGHT))
        .apply(container)
        .class(cosmic::theme::Container::custom(move |theme| {
            let radius = theme.cosmic().corner_radii.radius_s;

            container::Style {
                snap: true,
                background: Some(Background::Color(card.background)),
                border: Border {
                    radius: Radius {
                        top_left: 0.0,
                        top_right: 0.0,
                        bottom_right: radius[2],
                        bottom_left: radius[3],
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        }))
        .into()
}

pub fn wallpapers(decks: &[Deck]) -> Vec<(Vec<(Mode, String)>, PathBuf)> {
    let mut wanted: BTreeMap<PathBuf, Vec<(Mode, String)>> = BTreeMap::new();

    for deck in decks {
        for card in deck.live.iter().chain(deck.cards.iter()) {
            if let Preview::Wallpaper(path, _) = &card.preview {
                wanted
                    .entry(path.clone())
                    .or_default()
                    .push((deck.mode, card.slug.clone()));
            }
        }
    }

    wanted
        .into_iter()
        .map(|(path, slugs)| (slugs, path))
        .collect()
}

pub async fn wallpaper_handle(path: PathBuf) -> Option<cosmic::widget::image::Handle> {
    let thumbnail = tokio::task::spawn_blocking(move || wallpaper::load_image_with_thumbnail(path))
        .await
        .ok()
        .flatten()?;
    let display = thumbnail.1;

    Some(cosmic::widget::image::Handle::from_rgba(
        display.width(),
        display.height(),
        display.into_raw(),
    ))
}

pub fn wallpaper_loaded(
    decks: &mut [Deck],
    cards: &[(Mode, String)],
    handle: cosmic::widget::image::Handle,
) {
    for deck in decks.iter_mut() {
        let mine = |slug: &str| {
            cards
                .iter()
                .any(|(mode, it)| *mode == deck.mode && it == slug)
        };

        if mine("")
            && let Some(live) = deck.live.as_mut()
        {
            live.preview = Preview::Image(handle.clone());
        }
        for card in deck.cards.iter_mut().filter(|card| mine(&card.slug)) {
            card.preview = Preview::Image(handle.clone());
        }
    }
}

fn colored(color: cosmic::iced::Color) -> wallpaper::Color {
    wallpaper::Color::Single([color.r, color.g, color.b])
}
