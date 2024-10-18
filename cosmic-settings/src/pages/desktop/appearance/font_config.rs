// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::rc::Rc;
use std::sync::Arc;

use cosmic::{
    config::{CosmicTk, FontConfig},
    iced::Length,
    iced_core::text::Wrapping,
    theme,
    widget::{self, settings, svg},
    Apply, Element, Task,
};
use cosmic_config::ConfigSet;
use ustr::Ustr;

const INTERFACE_FONT: &str = "interface_font";
const MONOSPACE_FONT: &str = "monospace_font";

pub fn load_font_families() -> (Vec<Arc<str>>, Vec<Arc<str>>) {
    let mut font_system = cosmic::iced::advanced::graphics::text::font_system()
        .write()
        .unwrap();

    let (mut interface, mut mono): (Vec<Arc<str>>, Vec<Arc<str>>) =
        font_system.raw().db().faces().fold(
            (Vec::new(), Vec::new()),
            |(mut interface, mut mono), face| {
                if face.stretch != fontdb::Stretch::Normal
                    || face.weight != fontdb::Weight::NORMAL
                    || face.style != fontdb::Style::Normal
                {
                    return (interface, mono);
                }

                let font_name = match face.families.first() {
                    Some(name) => &name.0,
                    None => return (interface, mono),
                };

                if face.monospaced {
                    if mono
                        .last()
                        .map_or(true, |name| &**name != font_name.as_str())
                    {
                        mono.push(Arc::from(font_name.as_str()));
                    }
                } else if interface
                    .last()
                    .map_or(true, |name| &**name != font_name.as_str())
                {
                    interface.push(Arc::from(font_name.as_str()));
                }

                (interface, mono)
            },
        );

    interface.sort_unstable();
    interface.dedup();
    mono.sort_unstable();
    mono.dedup();

    (interface, mono)
}

pub fn selection_context<'a>(
    families: &'a [Arc<str>],
    search: &'a str,
    current_font: &'a str,
    system: bool,
) -> Element<'a, super::Message> {
    let space_l = theme::active().cosmic().spacing.space_l;

    let svg_accent = Rc::new(|theme: &cosmic::Theme| {
        let color = theme.cosmic().accent_color().into();
        svg::Style { color: Some(color) }
    });

    let search_input = widget::search_input(fl!("type-to-search"), search)
        .on_input(super::Message::FontSearch)
        .on_clear(super::Message::FontSearch(String::new()));

    let list = families.iter().fold(widget::list_column(), |list, family| {
        let selected = &**family == current_font;
        list.add(
            settings::item_row(vec![
                widget::text::body(&**family)
                    .wrapping(Wrapping::Word)
                    .into(),
                widget::horizontal_space().width(Length::Fill).into(),
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
            .on_press(super::Message::FontSelect(system, family.clone())),
        )
    });

    widget::column()
        .padding([2, 0])
        .spacing(space_l)
        .push(search_input)
        .push(list)
        .into()
}

/// Set the preferred icon theme for GNOME/GTK applications.
pub async fn set_gnome_font_name(font_name: &str) {
    let _res = tokio::process::Command::new("gsettings")
        .args(["set", "org.gnome.desktop.interface", "font-name", font_name])
        .status()
        .await;
}

#[derive(Debug, Clone)]
pub enum Message {
    InterfaceFontFamily(usize),
    LoadedFonts(Vec<Arc<str>>, Vec<Arc<str>>),
    MonospaceFontFamily(usize),
}

#[derive(Debug, Default)]
pub struct Model {
    pub interface_font_families: Vec<Arc<str>>,
    pub interface_font_family: Option<usize>,
    pub monospace_font_families: Vec<Arc<str>>,
    pub monospace_font_family: Option<usize>,
}

impl Model {
    pub const fn new() -> Model {
        Model {
            interface_font_families: Vec::new(),
            interface_font_family: None,
            monospace_font_families: Vec::new(),
            monospace_font_family: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::InterfaceFontFamily(id) => {
                if let Some(family) = self.interface_font_families.get(id) {
                    let family = Ustr::from(family);

                    update_config(
                        INTERFACE_FONT,
                        FontConfig {
                            family,
                            weight: cosmic::iced::font::Weight::Normal,
                            style: cosmic::iced::font::Style::Normal,
                            stretch: cosmic::iced::font::Stretch::Normal,
                        },
                    );

                    self.interface_font_family = Some(id);

                    tokio::spawn(async move {
                        set_gnome_font_name(family.as_str()).await;
                    });
                }
            }

            Message::LoadedFonts(interface, mono) => {
                self.interface_font_families = interface;
                self.monospace_font_families = mono;

                let interface_font = cosmic::config::interface_font();
                let monospace_font = cosmic::config::monospace_font();

                self.interface_font_family =
                    font_family_to_pos(&self.interface_font_families, &interface_font.family);

                self.monospace_font_family =
                    font_family_to_pos(&self.monospace_font_families, &monospace_font.family);
            }

            Message::MonospaceFontFamily(id) => {
                if let Some(family) = self.monospace_font_families.get(id) {
                    update_config(
                        MONOSPACE_FONT,
                        FontConfig {
                            family: Ustr::from(family),
                            weight: cosmic::iced::font::Weight::Normal,
                            style: cosmic::iced::font::Style::Normal,
                            stretch: cosmic::iced::font::Stretch::Normal,
                        },
                    );

                    self.monospace_font_family = Some(id);
                }
            }
        }

        Task::none()
    }
}

fn font_family_to_pos(families: &[Arc<str>], family: &str) -> Option<usize> {
    families.iter().position(|f| &**f == family)
}

fn update_config(variant: &str, font: FontConfig) {
    if let Ok(config) = CosmicTk::config() {
        _ = config.set(variant, font);
    }
}
