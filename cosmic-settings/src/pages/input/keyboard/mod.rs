// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod shortcuts;

use std::cmp;

use cosmic::{
    Apply, Element, Task,
    app::{ContextDrawer, context_drawer},
    cosmic_config::{self, ConfigSet},
    iced::{Alignment, Length},
    theme,
    widget::{self, ListColumn, button, container, icon, radio, row, settings},
};
use cosmic_comp_config::{KeyboardConfig, NumlockState, XkbConfig};
use cosmic_settings_page::{self as page, Section, section};
use itertools::Itertools;
use slab::Slab;
use slotmap::{DefaultKey, Key, SlotMap};

static COMPOSE_OPTIONS: &[(&str, &str)] = &[
    // ("Left Alt", "compose:lalt"), XXX?
    ("Right Alt", "compose:ralt"),
    ("Left Super", "compose:lwin"),
    ("Right Super", "compose:rwin"),
    ("Menu key", "compose:menu"),
    ("Right Ctrl", "compose:rctrl"),
    ("Caps Lock", "compose:caps"),
    ("Scroll Lock", "compose:sclk"),
    ("Print Screen", "compose:prsc"),
];

static ALTERNATE_CHARACTER_OPTIONS: &[(&str, &str)] = &[
    ("Left Alt", "lv3:lalt_switch"),
    ("Right Alt", "lv3:ralt_switch"),
    ("Left Super", "lv3:lwin_switch"),
    ("Right Super", "lv3:rwin_switch"),
    ("Menu key", "lv3:menu_switch"),
    // ("Right Ctrl", "lv3:"), XXX
    ("Caps Lock", "lv3:caps_switch"),
    // ("Scroll Lock", "lv3:"), XXX
    // ("Print Screen", "lv3"), XXX
];

static CAPS_LOCK_OPTIONS: &[(&str, &str)] = &[
    ("Escape", "caps:escape"),
    ("Swap with Escape", "caps:swapescape"),
    ("Backspace", "caps:backspace"),
    ("Super", "caps:super"),
    ("Control", "caps:ctrl_modifier"),
];

#[derive(Clone, Debug)]
pub enum Message {
    ExpandInputSourcePopover(Option<DefaultKey>),
    InputSourceSearch(String),
    OpenSpecialCharacterContext(SpecialKey),
    OpenNumlockContext,
    ShowInputSourcesContext,
    SourceAdd(DefaultKey),
    SourceContext(SourceContext),
    SpecialCharacterSelect(Option<&'static str>),
    SetRepeatKeysDelay(u32),
    SetRepeatKeysRate(u32),
    SetShowExtendedInputSources(bool),
    SetNumlockState(NumlockState),
}

#[derive(Clone, Debug)]
pub enum SourceContext {
    MoveDown(DefaultKey),
    MoveUp(DefaultKey),
    Remove(DefaultKey),
    Settings(DefaultKey),
    ViewLayout(DefaultKey),
}

pub type Locale = String;
pub type Variant = String;
pub type Description = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LayoutSource {
    Base,
    Extra,
}

const KB_REPEAT_DELAY_DEFAULT: u32 = 600;
const KB_REPEAT_RATE_DEFAULT: u32 = 25;
const KB_REPEAT_DELAY_MAX: u32 = 1000;
const KB_REPEAT_DELAY_MIN: u32 = 200;
const KB_REPEAT_RATE_MAX: u32 = 45;
const KB_REPEAT_RATE_MIN: u32 = 5;

const COSMIC_COMP_CONFIG: &str = "com.system76.CosmicComp";
const COSMIC_COMP_CONFIG_VERSION: u64 = 1;

pub struct Page {
    entity: page::Entity,
    config: cosmic_config::Config,
    context: Option<Context>,
    input_source_search: String,
    xkb: XkbConfig,
    keyboard_config: KeyboardConfig,
    keyboard_layouts: SlotMap<DefaultKey, (Locale, Variant, Description, LayoutSource)>,
    active_layouts: Vec<DefaultKey>,
    expanded_source_popover: Option<DefaultKey>,
    show_extended_input_sources: bool,
}

impl Default for Page {
    fn default() -> Self {
        let config =
            cosmic_config::Config::new(COSMIC_COMP_CONFIG, COSMIC_COMP_CONFIG_VERSION).unwrap();

        Self {
            entity: page::Entity::null(),
            context: None,
            expanded_source_popover: None,
            keyboard_layouts: SlotMap::new(),
            active_layouts: Vec::new(),
            xkb: XkbConfig::default(),
            keyboard_config: KeyboardConfig::default(),
            input_source_search: String::new(),
            show_extended_input_sources: false,
            config,
        }
    }
}

enum Context {
    ShowInputSourcesContext,
    SpecialCharacter(SpecialKey),
    NumlockState,
}

#[derive(Copy, Clone, Debug)]
pub enum SpecialKey {
    AlternateCharacters,
    Compose,
    CapsLock,
}

impl SpecialKey {
    pub fn title(self) -> String {
        match self {
            Self::Compose => "Compose".to_string(),
            Self::AlternateCharacters => "Alternate Characters".to_string(),
            Self::CapsLock => "Caps Lock".to_string(),
        }
    }

    pub fn prefix(self) -> &'static str {
        match self {
            Self::Compose => "compose:",
            Self::AlternateCharacters => "lv3:",
            Self::CapsLock => "caps:",
        }
    }
}

fn popover_menu_row(
    id: DefaultKey,
    label: String,
    message: impl Fn(DefaultKey) -> SourceContext + 'static,
) -> cosmic::Element<'static, Message> {
    let spacing = theme::spacing();
    widget::text::body(label)
        .align_y(Alignment::Center)
        .apply(button::custom)
        .padding([spacing.space_xxxs, spacing.space_xs])
        .width(Length::Fill)
        .class(theme::Button::MenuItem)
        .on_press(Message::SourceContext(message(id)))
        .apply(Element::from)
}

fn popover_menu(id: DefaultKey) -> cosmic::Element<'static, Message> {
    widget::column::with_children([
        popover_menu_row(
            id,
            fl!("keyboard-sources", "move-up"),
            SourceContext::MoveUp,
        ),
        popover_menu_row(
            id,
            fl!("keyboard-sources", "move-down"),
            SourceContext::MoveDown,
        ),
        widget::divider::horizontal::default()
            .apply(widget::container)
            .padding(8)
            .into(),
        popover_menu_row(
            id,
            fl!("keyboard-sources", "settings"),
            SourceContext::Settings,
        ),
        popover_menu_row(
            id,
            fl!("keyboard-sources", "view-layout"),
            SourceContext::ViewLayout,
        ),
        popover_menu_row(id, fl!("keyboard-sources", "remove"), SourceContext::Remove),
    ])
    .width(Length::Fixed(200.0))
    .apply(widget::container)
    .padding(theme::spacing().space_xxs)
    .class(theme::Container::Dropdown)
    .into()
}

fn popover_button(id: DefaultKey, expanded: bool) -> cosmic::Element<'static, Message> {
    let on_press = Message::ExpandInputSourcePopover(if expanded { None } else { Some(id) });

    let button = button::icon(icon::from_name("view-more-symbolic"))
        .extra_small()
        .on_press(on_press);

    if expanded {
        widget::popover(button)
            .position(widget::popover::Position::Bottom)
            .popup(popover_menu(id))
            .on_close(Message::ExpandInputSourcePopover(None))
            .into()
    } else {
        button.into()
    }
}

fn input_source(
    id: DefaultKey,
    description: &str,
    expanded_source_popover: Option<DefaultKey>,
) -> cosmic::Element<'_, Message> {
    let expanded = expanded_source_popover.is_some_and(|expanded_id| expanded_id == id);

    settings::item(description, popover_button(id, expanded)).into()
}

fn special_char_radio_row<'a>(
    desc: &'a str,
    value: Option<&'static str>,
    current_value: Option<&'a str>,
) -> cosmic::Element<'a, Message> {
    settings::item_row(vec![
        radio(desc, value, Some(current_value), |_| {
            Message::SpecialCharacterSelect(value)
        })
        .width(Length::Fill)
        .into(),
    ])
    .into()
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
            sections.insert(input_sources()),
            sections.insert(special_character_entry()),
            sections.insert(keyboard_shortcuts()),
            sections.insert(keyboard_typing_assist()),
            sections.insert(keyboard_num_lock()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("keyboard", "input-keyboard-symbolic")
            .title(fl!("keyboard"))
            .description(fl!("keyboard", "desc"))
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        self.context.as_ref().map(|context| match context {
            Context::ShowInputSourcesContext => {
                let search = widget::search_input("", &self.input_source_search)
                    .on_input(Message::InputSourceSearch)
                    .on_clear(Message::InputSourceSearch(String::new()))
                    .apply(Element::from)
                    .map(crate::pages::Message::Keyboard);

                context_drawer(
                    self.add_input_source_view(),
                    crate::pages::Message::CloseContextDrawer,
                )
                .title(fl!("keyboard-sources", "add"))
                .header(search)
            }
            Context::SpecialCharacter(special_key) => context_drawer(
                self.special_character_key_view(*special_key)
                    .map(crate::pages::Message::Keyboard),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(special_key.title()),
            Context::NumlockState => context_drawer(
                self.numlock_state_view()
                    .map(crate::pages::Message::Keyboard),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("keyboard-numlock-boot", "set")),
        })
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        self.xkb = super::get_config(&self.config, "xkb_config");
        self.keyboard_config = super::get_config(&self.config, "keyboard_config");
        match (
            xkb_data::keyboard_layouts(),
            xkb_data::extra_keyboard_layouts(),
        ) {
            (Ok(base_layouts), Ok(extra_layouts)) => {
                self.active_layouts.clear();
                self.keyboard_layouts.clear();

                let mut sorted_layouts = base_layouts
                    .layouts()
                    .iter()
                    .map(|layout| (layout, LayoutSource::Base))
                    .chain(
                        extra_layouts
                            .layouts()
                            .iter()
                            .map(|layout| (layout, LayoutSource::Extra)),
                    )
                    .collect::<Vec<_>>();

                sorted_layouts.sort_unstable_by(|(a, _), (b, _)| {
                    match (a.name(), b.name()) {
                        // Place US at the top of the list as it's the default
                        ("us", _) => cmp::Ordering::Less,
                        (_, "us") => cmp::Ordering::Greater,
                        // Place custom at the bottom
                        ("custom", _) => cmp::Ordering::Greater,
                        (_, "custom") => cmp::Ordering::Less,
                        // Compare everything else by description because it looks nicer (e.g. all
                        // English grouped together)
                        _ => a.description().cmp(b.description()),
                    }
                });

                for (layout, source) in sorted_layouts {
                    self.keyboard_layouts.insert((
                        layout.name().to_owned(),
                        String::new(),
                        gettextrs::dgettext("xkeyboard-config", layout.description()),
                        source.clone(),
                    ));

                    if let Some(variants) = layout.variants().map(|variants| {
                        variants.iter().map(|variant| {
                            (
                                layout.name().to_owned(),
                                variant.name().to_owned(),
                                gettextrs::dgettext("xkeyboard-config", variant.description()),
                                source.clone(),
                            )
                        })
                    }) {
                        let mut variants: Vec<_> = variants.collect();
                        variants.sort_unstable_by(|(_, _, desc_a, _), (_, _, desc_b, _)| {
                            desc_a.cmp(desc_b)
                        });

                        for (layout_name, name, description, source) in variants {
                            self.keyboard_layouts
                                .insert((layout_name, name, description, source));
                        }
                    }
                }

                // Xkb layouts currently enabled.
                let layouts = if self.xkb.layout.is_empty() {
                    "us"
                } else {
                    &self.xkb.layout
                }
                .split_terminator(',');

                // Xkb variants for each layout. Repeat empty strings in case there's more layouts than variants.
                let variants = self
                    .xkb
                    .variant
                    .split_terminator(',')
                    .chain(std::iter::repeat(""));

                for (layout, variant) in layouts.zip(variants) {
                    for (id, (xkb_layout, xkb_variant, _desc, _source)) in &self.keyboard_layouts {
                        if layout == xkb_layout && variant == xkb_variant {
                            self.active_layouts.push(id);
                            break;
                        }
                    }
                }
            }
            (Err(why), _) | (_, Err(why)) => {
                tracing::error!(?why, "failed to get keyboard layouts");
            }
        }

        Task::none()
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        self.active_layouts = Vec::new();
        self.keyboard_layouts = SlotMap::new();
        self.input_source_search = String::new();
        Task::none()
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::InputSourceSearch(search) => {
                self.input_source_search = search;
            }

            Message::SourceAdd(id) => {
                self.context = None;

                if !self.active_layouts.contains(&id) {
                    self.active_layouts.push(id);
                    self.update_xkb_config();
                }
            }

            Message::SourceContext(context_message) => {
                self.expanded_source_popover = None;

                match context_message {
                    SourceContext::MoveDown(id) => {
                        if let Some(pos) =
                            self.active_layouts.iter().position(|&active| active == id)
                            && pos + 1 < self.active_layouts.len()
                        {
                            self.active_layouts.swap(pos, pos + 1);
                            self.update_xkb_config();
                        }
                    }

                    SourceContext::MoveUp(id) => {
                        if let Some(pos) =
                            self.active_layouts.iter().position(|&active| active == id)
                            && pos > 0
                        {
                            self.active_layouts.swap(pos, pos - 1);
                            self.update_xkb_config();
                        }
                    }

                    SourceContext::Remove(id) => {
                        if let Some(pos) =
                            self.active_layouts.iter().position(|&active| active == id)
                        {
                            let _removed = self.active_layouts.remove(pos);
                            self.update_xkb_config();
                        }
                    }

                    SourceContext::Settings(_id) => {
                        eprintln!("settings not implemented");
                    }

                    SourceContext::ViewLayout(_id) => {
                        eprintln!("view layout not implemented");
                    }
                }
            }

            Message::ShowInputSourcesContext => {
                self.context = Some(Context::ShowInputSourcesContext);
                return cosmic::task::message(crate::app::Message::OpenContextDrawer(self.entity));
            }

            Message::ExpandInputSourcePopover(value) => {
                self.expanded_source_popover = value;
            }

            Message::OpenSpecialCharacterContext(key) => {
                self.context = Some(Context::SpecialCharacter(key));
                return cosmic::task::message(crate::app::Message::OpenContextDrawer(self.entity));
            }

            Message::OpenNumlockContext => {
                self.context = Some(Context::NumlockState);
                return cosmic::task::message(crate::app::Message::OpenContextDrawer(self.entity));
            }

            Message::SpecialCharacterSelect(id) => {
                if let Some(Context::SpecialCharacter(special_key)) = self.context {
                    let options = self.xkb.options.as_deref().unwrap_or_default();
                    let prefix = special_key.prefix();
                    let new_options = options
                        .split(',')
                        .filter(|x| !x.starts_with(prefix))
                        .chain(id)
                        .join(",");

                    self.xkb.options = Some(new_options).filter(|x| !x.is_empty());

                    if let Err(err) = self.config.set("xkb_config", &self.xkb) {
                        tracing::error!(?err, "Failed to set config 'xkb_config'");
                    }
                }
            }
            Message::SetRepeatKeysDelay(delay) => {
                self.xkb.repeat_delay = delay;
                self.update_xkb_config();
            }
            Message::SetRepeatKeysRate(rate) => {
                self.xkb.repeat_rate = rate;
                self.update_xkb_config();
            }
            Message::SetShowExtendedInputSources(value) => {
                self.show_extended_input_sources = value;
            }
            Message::SetNumlockState(numlock_state) => {
                self.keyboard_config.numlock_state = numlock_state;
                if let Err(err) = self.config.set("keyboard_config", &self.keyboard_config) {
                    tracing::error!(?err, "Failed to set config 'keyboard_config'");
                }
            }
        }

        Task::none()
    }

    pub fn add_input_source_view(&self) -> Element<'_, crate::pages::Message> {
        let space_l = theme::spacing().space_l;

        let toggler = settings::item::builder(fl!("show-extended-input-sources")).toggler(
            self.show_extended_input_sources,
            Message::SetShowExtendedInputSources,
        );

        let mut list = widget::list_column();

        let search_input = &self.input_source_search.trim().to_lowercase();

        for (id, (_locale, variant, description, source)) in &self.keyboard_layouts {
            if (search_input.is_empty() || description.to_lowercase().contains(search_input))
                && (source != &LayoutSource::Extra || self.show_extended_input_sources)
            {
                list = list.add(self.input_source_item(id, description, !variant.is_empty()));
            }
        }

        widget::column()
            .spacing(space_l)
            .push(toggler)
            .push(list)
            .apply(Element::from)
            .map(crate::pages::Message::Keyboard)
    }

    fn input_source_item<'a>(
        &self,
        id: DefaultKey,
        description: &'a str,
        indent: bool,
    ) -> Element<'a, Message> {
        let is_added = self.active_layouts.contains(&id);
        let button_text = if is_added { fl!("added") } else { fl!("add") };

        let add_button = widget::button::text(button_text).on_press_maybe(if is_added {
            None
        } else {
            Some(Message::SourceAdd(id))
        });

        let button = widget::settings::item::builder(description).control(add_button);

        if indent {
            container(button).padding([0, 0, 0, 16]).into()
        } else {
            button.into()
        }
    }

    fn special_character_key_view(&self, special_key: SpecialKey) -> cosmic::Element<'_, Message> {
        let (options, description) = match special_key {
            SpecialKey::Compose => (
                COMPOSE_OPTIONS,
                Some(fl!("keyboard-special-char", "compose-desc")),
            ),
            SpecialKey::AlternateCharacters => (ALTERNATE_CHARACTER_OPTIONS, None),
            SpecialKey::CapsLock => (CAPS_LOCK_OPTIONS, None),
        };
        let prefix = special_key.prefix();
        let current = self
            .xkb
            .options
            .iter()
            .flat_map(|x| x.split(','))
            .find(|x| x.starts_with(prefix));

        // TODO layout default

        let mut list = cosmic::widget::list_column();

        if matches!(special_key, SpecialKey::CapsLock) {
            list = list.add(special_char_radio_row("Caps Lock", None, current));
        } else {
            list = list.add(special_char_radio_row("None", None, current));
        }

        list = options
            .iter()
            .map(|(desc, id)| special_char_radio_row(desc, Some(id), current))
            .fold(list, ListColumn::add);

        widget::column::with_capacity(2)
            .spacing(theme::spacing().space_m)
            .push_maybe(description.map(widget::text::body))
            .push(list)
            .into()
    }

    fn numlock_state_view(&self) -> cosmic::Element<'_, Message> {
        let current = self.keyboard_config.numlock_state;
        let options = [
            (fl!("keyboard-numlock-boot", "off"), NumlockState::BootOff),
            (fl!("keyboard-numlock-boot", "on"), NumlockState::BootOn),
            (
                fl!("keyboard-numlock-boot", "last-boot"),
                NumlockState::LastBoot,
            ),
        ];

        let mut list = cosmic::widget::list_column();
        for (desc, state) in options {
            list = list.add(settings::item_row(vec![
                radio(
                    cosmic::widget::text(desc),
                    Some(state),
                    Some(Some(current)),
                    |_| Message::SetNumlockState(state),
                )
                .width(Length::Fill)
                .into(),
            ]));
        }

        list.into()
    }

    fn update_xkb_config(&mut self) {
        let result = update_xkb_config(
            &self.config,
            &mut self.xkb,
            &mut self
                .active_layouts
                .iter()
                .filter_map(|id| self.keyboard_layouts.get(*id))
                .map(|(locale, variant, _description, _source)| {
                    (locale.as_str(), variant.as_str())
                }),
        );

        if let Err(why) = result {
            tracing::error!(?why, "Failed to set config 'xkb_config'");
        }
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<shortcuts::Page>()
    }
}

fn input_sources() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("keyboard-sources"))
        .view::<Page>(move |_binder, page, section| {
            // TODO Need something more custom, with drag and drop
            let mut section = settings::section().title(&section.title);

            for id in &page.active_layouts {
                if let Some((_locale, _variant, description, _source)) =
                    page.keyboard_layouts.get(*id)
                {
                    section =
                        section.add(input_source(*id, description, page.expanded_source_popover));
                }
            }

            let add_input_source = widget::button::standard(fl!("keyboard-sources", "add"))
                .on_press(Message::ShowInputSourcesContext);

            widget::column::with_capacity(2)
                .spacing(theme::spacing().space_xxs)
                .push(section)
                .push(
                    widget::container(add_input_source)
                        .width(Length::Fill)
                        .align_x(Alignment::End),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Keyboard)
        })
}

fn special_character_entry() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let alternate = descriptions.insert(fl!("keyboard-special-char", "alternate"));
    let compose = descriptions.insert(fl!("keyboard-special-char", "compose"));
    let caps = descriptions.insert(fl!("keyboard-special-char", "caps"));

    Section::default()
        .title(fl!("keyboard-special-char"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(crate::widget::go_next_item(
                    &descriptions[alternate],
                    Message::OpenSpecialCharacterContext(SpecialKey::AlternateCharacters),
                ))
                .add(crate::widget::go_next_item(
                    &descriptions[compose],
                    Message::OpenSpecialCharacterContext(SpecialKey::Compose),
                ))
                .add(crate::widget::go_next_item(
                    &descriptions[caps],
                    Message::OpenSpecialCharacterContext(SpecialKey::CapsLock),
                ))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Keyboard)
        })
}

fn keyboard_shortcuts() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let shortcuts_desc = descriptions.insert(fl!("keyboard-shortcuts", "desc"));

    Section::default()
        .title(fl!("keyboard-shortcuts"))
        .descriptions(descriptions)
        .view::<Page>(move |binder, _page, section| {
            let descriptions = &section.descriptions;

            let mut section = settings::section().title(&section.title);
            if let Some((shortcuts_entity, _)) = binder
                .info
                .iter()
                .find(|(_, v)| v.id == "keyboard-shortcuts")
            {
                section = section.add(crate::widget::go_next_item(
                    &descriptions[shortcuts_desc],
                    crate::pages::Message::Page(shortcuts_entity),
                ));
            }
            section.apply(cosmic::Element::from)
        })
}

fn keyboard_typing_assist() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let repeat_delay = descriptions.insert(fl!("keyboard-typing-assist", "repeat-delay"));
    let repeat_rate = descriptions.insert(fl!("keyboard-typing-assist", "repeat-rate"));
    let short = descriptions.insert(fl!("short"));
    let long = descriptions.insert(fl!("long"));
    let slow = descriptions.insert(fl!("slow"));
    let fast = descriptions.insert(fl!("fast"));

    Section::default()
        .title(fl!("keyboard-typing-assist"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(settings::flex_item(&descriptions[repeat_delay], {
                    // Delay
                    let delay_slider = cosmic::widget::slider(
                        KB_REPEAT_DELAY_MIN..=KB_REPEAT_DELAY_MAX,
                        page.xkb.repeat_delay,
                        Message::SetRepeatKeysDelay,
                    )
                    .width(Length::Fill)
                    .breakpoints(&[KB_REPEAT_DELAY_DEFAULT])
                    .step(50_u32)
                    .apply(widget::container)
                    .max_width(250);

                    row::with_capacity(3)
                        .align_y(Alignment::Center)
                        .spacing(theme::spacing().space_s)
                        .push(widget::text::body(&descriptions[short]))
                        .push(delay_slider)
                        .push(widget::text::body(&descriptions[long]))
                }))
                .add(settings::flex_item(&descriptions[repeat_rate], {
                    // Repeat rate
                    let rate_slider = cosmic::widget::slider(
                        KB_REPEAT_RATE_MIN..=KB_REPEAT_RATE_MAX,
                        page.xkb.repeat_rate,
                        Message::SetRepeatKeysRate,
                    )
                    .width(Length::Fill)
                    .breakpoints(&[KB_REPEAT_RATE_DEFAULT])
                    .step(5_u32)
                    .apply(widget::container)
                    .max_width(250);

                    row::with_capacity(3)
                        .align_y(Alignment::Center)
                        .spacing(theme::spacing().space_s)
                        .push(widget::text::body(&descriptions[slow]))
                        .push(rate_slider)
                        .push(widget::text::body(&descriptions[fast]))
                }))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Keyboard)
        })
}

fn keyboard_num_lock() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let boot_state = descriptions.insert(fl!("keyboard-numlock-boot", "boot-state"));

    Section::default()
        .title(fl!("keyboard-numlock-boot"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(crate::widget::go_next_item(
                    &descriptions[boot_state],
                    Message::OpenNumlockContext,
                ))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Keyboard)
        })
}

fn update_xkb_config(
    config: &cosmic_config::Config,
    xkb: &mut XkbConfig,
    active_layouts: &mut dyn Iterator<Item = (&str, &str)>,
) -> Result<(), cosmic_config::Error> {
    let mut new_layout = String::new();
    let mut new_variant = String::new();

    for (locale, variant) in active_layouts {
        new_layout.push_str(locale);
        new_layout.push(',');
        new_variant.push_str(variant);
        new_variant.push(',');
    }

    let _excess_comma = new_layout.pop();
    let _excess_comma = new_variant.pop();

    xkb.layout = new_layout;
    xkb.variant = new_variant;

    config.set("xkb_config", xkb)
}
