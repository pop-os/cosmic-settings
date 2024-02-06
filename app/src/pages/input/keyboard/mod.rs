use apply::Apply;
use cosmic::{
    iced::{
        self,
        widget::{self, horizontal_space},
        window, Length,
    },
    iced_core::Border,
    iced_style, theme,
    widget::{button, container, icon, radio, settings},
};
use cosmic_settings_page::{self as page, section, Section};
use once_cell::sync::Lazy;
use slotmap::SlotMap;

use super::Message;

pub static ADD_INPUT_SOURCE_DIALOGUE_ID: Lazy<window::Id> = Lazy::new(window::Id::unique);
pub static SPECIAL_CHARACTER_DIALOGUE_ID: Lazy<window::Id> = Lazy::new(window::Id::unique);

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

fn popover_menu_row(label: String) -> cosmic::Element<'static, Message> {
    widget::text(label)
        .apply(widget::container)
        .style(cosmic::theme::Container::custom(|theme| {
            iced_style::container::Appearance {
                background: None,
                ..cosmic::widget::list::style(theme)
            }
        }))
        .apply(button)
        .style(theme::Button::Transparent)
        .into()
}

// TODO for on press, would need to clone ID for each row?
fn popover_menu() -> cosmic::Element<'static, Message> {
    // XXX translate
    widget::column![
        popover_menu_row(fl!("keyboard-sources", "move-up")),
        popover_menu_row(fl!("keyboard-sources", "move-down")),
        //cosmic::widget::divider::horizontal::light(),
        cosmic::widget::divider::horizontal::light(),
        popover_menu_row(fl!("keyboard-sources", "settings")),
        popover_menu_row(fl!("keyboard-sources", "view-layout")),
        popover_menu_row(fl!("keyboard-sources", "remove")),
    ]
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

fn popover_button(input_source: &InputSource, expanded: bool) -> cosmic::Element<'static, Message> {
    let on_press = Message::ExpandInputSourcePopover(if expanded {
        None
    } else {
        Some(input_source.id.clone())
    });

    let button = button::icon(icon::from_name("open-menu-symbolic"))
        .extra_small()
        .padding(0)
        .on_press(on_press);

    if expanded {
        cosmic::widget::popover(button, popover_menu()).into()
    } else {
        button.into()
    }
}

fn input_source<'a>(
    input_source: &'a InputSource,
    expanded_source_popover: Option<&'a str>,
) -> cosmic::Element<'a, Message> {
    let expanded = expanded_source_popover == Some(input_source.id.as_str());
    settings::item(&input_source.label, popover_button(input_source, expanded)).into()
}

pub mod shortcuts;

pub struct InputSource {
    id: String,
    // TODO Translate?
    label: String,
}

impl super::Page {
    pub fn add_input_source_view(&self) -> cosmic::Element<'static, crate::app::Message> {
        widget::column![].into()
    }

    pub fn special_character_key_view(&self) -> cosmic::Element<'_, crate::app::Message> {
        let Some(special_key) = self.special_character_dialog else {
            return widget::text("").into();
        };

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
        widget::column![
            cosmic::widget::header_bar()
                .title(special_key.title())
                .on_close(Message::CloseSpecialCharacterDialog),
            cosmic::widget::container(
                cosmic::widget::scrollable(cosmic::widget::container(list).padding(24))
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
            .style(theme::Container::Background)
            .width(Length::Fill)
            .height(Length::Fill)
        ]
        .apply(cosmic::Element::from)
        .map(crate::pages::Message::Input)
        .map(crate::app::Message::PageMessage)
    }
}

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

#[derive(Default)]
pub struct Page;

// XXX
pub fn default_input_sources() -> Vec<InputSource> {
    vec![InputSource {
        id: "us".to_string(),
        label: "English (US)".to_string(),
    }]
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
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<shortcuts::Page>()
    }
}

fn input_sources() -> Section<crate::pages::Message> {
    // TODO desc
    Section::default()
        .title(fl!("keyboard-sources"))
        .view::<Page>(|binder, _page, section| {
            let input = binder.page::<super::Page>().expect("input page not found");

            // TODO Need something more custom, with drag and drop
            let mut section = settings::view_section(&section.title);

            let expanded_source = input.expanded_source_popover.as_deref();
            for source in &input.sources {
                section = section.add(input_source(source, expanded_source));
            }

            section
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn special_character_entry() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("keyboard-special-char"))
        .descriptions(vec![
            fl!("keyboard-special-char", "alternate"),
            fl!("keyboard-special-char", "compose"),
        ])
        .view::<Page>(|_binder, _page, section| {
            let descriptions = &section.descriptions;

            // TODO dialogs
            settings::view_section(&section.title)
                .add(go_next_item(
                    &descriptions[0],
                    Message::OpenSpecialCharacterDialog(SpecialKey::AlternateCharacters),
                ))
                .add(go_next_item(
                    &descriptions[1],
                    Message::OpenSpecialCharacterDialog(SpecialKey::Compose),
                ))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn keyboard_shortcuts() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("keyboard-shortcuts"))
        .descriptions(vec![fl!("keyboard-shortcuts", "desc")])
        .view::<Page>(|binder, _page, section| {
            let descriptions = &section.descriptions;

            let mut section = settings::view_section(&section.title);
            if let Some((shortcuts_entity, _)) = binder
                .info
                .iter()
                .find(|(_, v)| v.id == "keyboard-shortcuts")
            {
                section = section.add(go_next_item(
                    &descriptions[0],
                    crate::pages::Message::Page(shortcuts_entity),
                ));
            }
            section.apply(cosmic::Element::from)
        })
}

fn go_next_control<Msg: Clone + 'static>() -> cosmic::Element<'static, Msg> {
    widget::row!(
        horizontal_space(Length::Fill),
        icon::from_name("go-next-symbolic").size(16).icon(),
    )
    .into()
}

fn go_next_item<Msg: Clone + 'static>(description: &str, msg: Msg) -> cosmic::Element<'_, Msg> {
    settings::item(description, go_next_control())
        .apply(widget::container)
        .style(cosmic::theme::Container::custom(
            cosmic::widget::list::style,
        ))
        .apply(button)
        .style(theme::Button::Transparent)
        .on_press(msg)
        .into()
}
