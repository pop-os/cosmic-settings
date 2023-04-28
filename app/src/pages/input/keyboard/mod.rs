use apply::Apply;
use cosmic::iced::{
    self,
    widget::{self, horizontal_space},
    Length,
};
use cosmic::iced_style;
use cosmic::widget::settings;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

use super::Message;

fn popover_menu_row(label: String) -> cosmic::Element<'static, Message> {
    widget::text(label)
        .apply(widget::container)
        .style(cosmic::theme::Container::custom(|theme| {
            iced_style::container::Appearance {
                background: None,
                ..cosmic::widget::list::column::style(theme)
            }
        }))
        .apply(widget::button)
        .style(cosmic::theme::Button::Transparent)
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
        iced_style::container::Appearance {
            text_color: Some(theme.cosmic().background.on.into()),
            background: Some(iced::Color::from(theme.cosmic().background.base).into()),
            border_radius: (12.0).into(),
            border_width: 0.0,
            border_color: iced::Color::TRANSPARENT,
        }
    }))
    .into()
}

fn popover_button(input_source: &InputSource, expanded: bool) -> cosmic::Element<'static, Message> {
    let style = if expanded {
        cosmic::theme::Svg::SymbolicActive
    } else {
        cosmic::theme::Svg::Symbolic
    };
    let on_press = Message::ExpandInputSourcePopover(if expanded {
        None
    } else {
        Some(input_source.id.clone())
    });
    let button = cosmic::widget::button(cosmic::theme::Button::Secondary)
        .icon(style, "open-menu-symbolic", 20)
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
                .add(settings::item(&descriptions[0], go_next_control()))
                .add(settings::item(&descriptions[1], go_next_control()))
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

            settings::view_section(&section.title)
                .add(
                    settings::item(&descriptions[0], go_next_control())
                        .apply(widget::container)
                        .style(cosmic::theme::Container::custom(
                            cosmic::widget::list::column::style,
                        ))
                        .apply(widget::button)
                        .style(cosmic::theme::Button::Transparent)
                        .padding(0)
                        .on_press(Message::OpenKeyboardShortcuts),
                )
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn go_next_control() -> cosmic::Element<'static, Message> {
    widget::row!(
        horizontal_space(Length::Fill),
        cosmic::widget::icon("go-next-symbolic", 20).style(cosmic::theme::Svg::Symbolic)
    )
    .into()
}
