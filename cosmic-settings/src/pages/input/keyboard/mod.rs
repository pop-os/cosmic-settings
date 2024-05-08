use cosmic::{
    cosmic_config::{self, ConfigSet},
    iced::{self, Length},
    iced_core::Border,
    iced_style, theme,
    widget::{self, button, container, icon, radio, settings},
    Apply, Command, Element,
};
use cosmic_comp_config::XkbConfig;
use cosmic_settings_page::{self as page, section, Section};
use itertools::Itertools;
use slotmap::{DefaultKey, SlotMap};

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
    ("Right Alt", "lv3:alt_switch"),
    ("Left Super", "lv3:lwin_switch"),
    ("Right Super", "lv3:win_switch"),
    ("Menu key", "lv3:menu_switch"),
    // ("Right Ctrl", "lv3:"), XXX
    ("Caps Lock", "lv3:caps_switch"),
    // ("Scroll Lock", "lv3:"), XXX
    // ("Print Screen", "lv3"), XXX
];

#[derive(Clone, Debug)]
pub enum Message {
    ExpandInputSourcePopover(Option<DefaultKey>),
    InputSourceSearch(String),
    OpenSpecialCharacterContext(SpecialKey),
    ShowInputSourcesContext,
    SourceAdd(DefaultKey),
    SourceContext(SourceContext),
    SpecialCharacterSelect(Option<&'static str>),
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

pub struct Page {
    config: cosmic_config::Config,
    context: Option<Context>,
    input_source_search: String,
    xkb: XkbConfig,
    keyboard_layouts: SlotMap<DefaultKey, (Locale, Variant, Description)>,
    active_layouts: Vec<DefaultKey>,
    expanded_source_popover: Option<DefaultKey>,
}

impl Default for Page {
    fn default() -> Self {
        let config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();

        Self {
            context: None,
            expanded_source_popover: None,
            keyboard_layouts: SlotMap::new(),
            active_layouts: Vec::new(),
            xkb: XkbConfig::default(),
            input_source_search: String::new(),
            config,
        }
    }
}

enum Context {
    ShowInputSourcesContext,
    SpecialCharacter(SpecialKey),
}

#[derive(Copy, Clone, Debug)]
pub enum SpecialKey {
    AlternateCharacters,
    Compose,
}

impl SpecialKey {
    pub fn title(self) -> String {
        match self {
            Self::Compose => "Compose".to_string(),
            Self::AlternateCharacters => "Alternate Characters".to_string(),
        }
    }

    pub fn prefix(self) -> &'static str {
        match self {
            Self::Compose => "compose:",
            Self::AlternateCharacters => "lv3:",
        }
    }
}

fn popover_menu_row(
    id: DefaultKey,
    label: String,
    message: impl Fn(DefaultKey) -> SourceContext + 'static,
) -> cosmic::Element<'static, Message> {
    widget::text(label)
        .apply(widget::container)
        .style(cosmic::theme::Container::custom(|theme| {
            iced_style::container::Appearance {
                background: None,
                ..container::StyleSheet::appearance(theme, &cosmic::theme::Container::List)
            }
        }))
        .apply(button)
        .on_press(())
        .style(theme::Button::Transparent)
        .apply(Element::from)
        .map(move |()| Message::SourceContext(message(id)))
}

fn popover_menu(id: DefaultKey) -> cosmic::Element<'static, Message> {
    widget::column::with_children(vec![
        popover_menu_row(
            id,
            fl!("keyboard-sources", "move-up"),
            SourceContext::MoveUp,
        )
        .into(),
        popover_menu_row(
            id,
            fl!("keyboard-sources", "move-down"),
            SourceContext::MoveDown,
        )
        .into(),
        cosmic::widget::divider::horizontal::light().into(),
        popover_menu_row(
            id,
            fl!("keyboard-sources", "settings"),
            SourceContext::Settings,
        )
        .into(),
        popover_menu_row(
            id,
            fl!("keyboard-sources", "view-layout"),
            SourceContext::ViewLayout,
        )
        .into(),
        popover_menu_row(id, fl!("keyboard-sources", "remove"), SourceContext::Remove).into(),
    ])
    .width(Length::Shrink)
    .height(Length::Shrink)
    .apply(cosmic::widget::container)
    .style(cosmic::theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        container::Appearance {
            icon_color: Some(theme.cosmic().background.on.into()),
            text_color: Some(theme.cosmic().background.on.into()),
            background: Some(iced::Color::from(theme.cosmic().background.base).into()),
            border: Border {
                radius: cosmic.corner_radii.radius_m.into(),
                ..Default::default()
            },
            shadow: Default::default(),
        }
    }))
    .into()
}

fn popover_button(id: DefaultKey, expanded: bool) -> cosmic::Element<'static, Message> {
    let on_press = Message::ExpandInputSourcePopover(if expanded { None } else { Some(id) });

    let button = button::icon(icon::from_name("open-menu-symbolic"))
        .extra_small()
        .on_press(on_press);

    if expanded {
        cosmic::widget::popover(button)
            .popup(popover_menu(id))
            .into()
    } else {
        button.into()
    }
}

fn input_source(
    id: DefaultKey,
    description: &str,
    expanded_source_popover: Option<DefaultKey>,
) -> cosmic::Element<Message> {
    let expanded = expanded_source_popover.is_some_and(|expanded_id| expanded_id == id);

    settings::item(description, popover_button(id, expanded)).into()
}

pub mod shortcuts;

fn special_char_radio_row<'a>(
    desc: &'a str,
    value: Option<&'static str>,
    current_value: Option<&'a str>,
) -> cosmic::Element<'a, Message> {
    settings::item_row(vec![radio(desc, value, Some(current_value), |_| {
        Message::SpecialCharacterSelect(value)
    })
    .into()])
    .into()
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(input_sources()),
            sections.insert(special_character_entry()),
            sections.insert(keyboard_shortcuts()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("keyboard", "input-keyboard-symbolic")
            .title(fl!("keyboard"))
            .description(fl!("keyboard", "desc"))
    }

    fn context_drawer(&self) -> Option<Element<'_, crate::pages::Message>> {
        match self.context {
            Some(Context::ShowInputSourcesContext) => Some(self.add_input_source_view()),
            Some(Context::SpecialCharacter(special_key)) => self
                .special_character_key_view(special_key)
                .map(crate::pages::Message::Keyboard)
                .apply(Some),

            None => None,
        }
    }

    fn on_enter(
        &mut self,
        _page: page::Entity,
        sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> Command<crate::pages::Message> {
        self.xkb = super::get_config(&self.config, "xkb_config");
        match xkb_data::keyboard_layouts() {
            Ok(keyboard_layouts) => {
                self.active_layouts.clear();
                self.keyboard_layouts.clear();

                for layout in keyboard_layouts.layouts() {
                    self.keyboard_layouts.insert((
                        layout.name().to_owned(),
                        String::new(),
                        layout.description().to_owned(),
                    ));

                    if let Some(variants) = layout.variants() {
                        for variant in variants {
                            self.keyboard_layouts.insert((
                                layout.name().to_owned(),
                                variant.name().to_owned(),
                                variant.description().to_owned(),
                            ));
                        }
                    }
                }

                // Xkb layouts currently enabled.
                let layouts = self.xkb.layout.split_terminator(',');

                // Xkb variants for each layout. Repeat empty strings in case there's more layouts than variants.
                let variants = self
                    .xkb
                    .variant
                    .split_terminator(',')
                    .chain(std::iter::repeat(""));

                for (layout, variant) in layouts.zip(variants) {
                    for (id, (xkb_layout, xkb_variant, _desc)) in &self.keyboard_layouts {
                        if layout == xkb_layout && variant == xkb_variant {
                            self.active_layouts.push(id);
                        }
                    }
                }
            }

            Err(why) => {
                tracing::error!(?why, "failed to get keyboard layouts");
            }
        }

        Command::none()
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
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
                        {
                            if pos + 1 < self.active_layouts.len() {
                                self.active_layouts.swap(pos, pos + 1);
                                self.update_xkb_config();
                            }
                        }
                    }

                    SourceContext::MoveUp(id) => {
                        if let Some(pos) =
                            self.active_layouts.iter().position(|&active| active == id)
                        {
                            if pos > 0 {
                                self.active_layouts.swap(pos, pos - 1);
                                self.update_xkb_config();
                            }
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

                    SourceContext::Settings(id) => {
                        eprintln!("settings not implemented");
                    }

                    SourceContext::ViewLayout(id) => {
                        eprintln!("view layout not implemented");
                    }
                }
            }

            Message::ShowInputSourcesContext => {
                self.context = Some(Context::ShowInputSourcesContext);
                return cosmic::command::message(crate::app::Message::OpenContextDrawer(
                    fl!("keyboard-sources", "add").into(),
                ));
            }

            Message::ExpandInputSourcePopover(value) => {
                self.expanded_source_popover = value;
            }

            Message::OpenSpecialCharacterContext(key) => {
                self.context = Some(Context::SpecialCharacter(key));
                return cosmic::command::message(crate::app::Message::OpenContextDrawer(
                    key.title().into(),
                ));
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
        }

        Command::none()
    }

    pub fn add_input_source_view(&self) -> Element<'_, crate::pages::Message> {
        let search = widget::search_input(fl!("type-to-search"), &self.input_source_search)
            .on_input(Message::InputSourceSearch)
            .on_clear(Message::InputSourceSearch(String::new()));

        let mut list = widget::list_column();

        let search_input = &self.input_source_search.trim().to_lowercase();

        for (id, (_locale, variant, description)) in &self.keyboard_layouts {
            if search_input.is_empty() || description.to_lowercase().contains(search_input) {
                list = list.add(self.input_source_item(id, description, !variant.is_empty()));
            }
        }

        widget::column()
            .spacing(32)
            .push(search)
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
        let options = match special_key {
            SpecialKey::Compose => COMPOSE_OPTIONS,
            SpecialKey::AlternateCharacters => ALTERNATE_CHARACTER_OPTIONS,
        };
        let prefix = special_key.prefix();
        let current = self
            .xkb
            .options
            .iter()
            .flat_map(|x| x.split(','))
            .find(|x| x.starts_with(prefix));

        // TODO description, layout default

        let mut list = cosmic::widget::list_column();
        list = list.add(special_char_radio_row("None", None, current));
        for (desc, id) in options {
            list = list.add(special_char_radio_row(desc, Some(id), current));
        }

        cosmic::widget::scrollable(cosmic::widget::container(list).padding(24))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update_xkb_config(&mut self) {
        let mut new_layout = String::new();
        let mut new_variant = String::new();

        for id in &self.active_layouts {
            if let Some((locale, variant, _description)) = self.keyboard_layouts.get(*id) {
                new_layout.push_str(locale);
                new_layout.push(',');
                new_variant.push_str(variant);
                new_variant.push(',');
            }
        }

        let _excess_comma = new_layout.pop();
        let _excess_comma = new_variant.pop();

        self.xkb.layout = new_layout;
        self.xkb.variant = new_variant;

        if let Err(err) = self.config.set("xkb_config", &self.xkb) {
            tracing::error!(?err, "Failed to set config 'xkb_config'");
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
        .view::<Page>(|_binder, page, section| {
            // TODO Need something more custom, with drag and drop
            let mut section = settings::view_section(&section.title);

            for id in &page.active_layouts {
                if let Some((_locale, _variant, description)) = page.keyboard_layouts.get(*id) {
                    section =
                        section.add(input_source(*id, description, page.expanded_source_popover));
                }
            }

            let add_input_source = widget::button::standard(fl!("keyboard-sources", "add"))
                .on_press(Message::ShowInputSourcesContext);

            widget::column::with_capacity(2)
                .spacing(cosmic::theme::active().cosmic().space_xxs())
                .push(section)
                .push(
                    widget::container(add_input_source)
                        .width(Length::Fill)
                        .align_x(iced::alignment::Horizontal::Right),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Keyboard)
        })
}

fn special_character_entry() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("keyboard-special-char"))
        .descriptions(vec![
            fl!("keyboard-special-char", "alternate").into(),
            fl!("keyboard-special-char", "compose").into(),
        ])
        .view::<Page>(|_binder, _page, section| {
            let descriptions = &section.descriptions;

            settings::view_section(&section.title)
                .add(go_next_item(
                    &*descriptions[0],
                    Message::OpenSpecialCharacterContext(SpecialKey::AlternateCharacters),
                ))
                .add(go_next_item(
                    &*descriptions[1],
                    Message::OpenSpecialCharacterContext(SpecialKey::Compose),
                ))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Keyboard)
        })
}

fn keyboard_shortcuts() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("keyboard-shortcuts"))
        .descriptions(vec![fl!("keyboard-shortcuts", "desc").into()])
        .view::<Page>(|binder, _page, section| {
            let descriptions = &section.descriptions;

            let mut section = settings::view_section(&section.title);
            if let Some((shortcuts_entity, _)) = binder
                .info
                .iter()
                .find(|(_, v)| v.id == "keyboard-shortcuts")
            {
                section = section.add(go_next_item(
                    &*descriptions[0],
                    crate::pages::Message::Page(shortcuts_entity),
                ));
            }
            section.apply(cosmic::Element::from)
        })
}

fn go_next_control<Msg: Clone + 'static>() -> cosmic::Element<'static, Msg> {
    widget::row::with_children(vec![
        widget::horizontal_space(Length::Fill).into(),
        icon::from_name("go-next-symbolic").size(16).icon().into(),
    ])
    .into()
}

fn go_next_item<Msg: Clone + 'static>(description: &str, msg: Msg) -> cosmic::Element<'_, Msg> {
    settings::item(description, go_next_control())
        .apply(widget::container)
        .style(cosmic::theme::Container::List)
        .apply(button)
        .style(theme::Button::Transparent)
        .on_press(msg)
        .into()
}
