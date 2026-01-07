// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::rc::Rc;
use std::sync::Arc;

use cosmic::{
    Apply, Element, Task,
    config::{CosmicTk, FontConfig},
    iced_core::text::Wrapping,
    widget::{self, settings, svg},
};
use cosmic_config::ConfigSet;

use crate::app;

use super::{ContextView, Message, drawer};

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
                if face.stretch != fontdb::Stretch::Normal || face.style != fontdb::Style::Normal {
                    return (interface, mono);
                }

                let font_name = match face.families.first() {
                    Some(name) => &name.0,
                    None => return (interface, mono),
                };

                if face.monospaced {
                    if mono.last().is_none_or(|name| &**name != font_name.as_str()) {
                        mono.push(Arc::from(font_name.as_str()));
                    }
                } else if interface
                    .last()
                    .is_none_or(|name| &**name != font_name.as_str())
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

#[derive(Debug)]
pub struct Model {
    font_search: String,
    font_filter: Vec<Arc<str>>,

    interface_font_families: Vec<Arc<str>>,
    pub interface_font: FontConfig,

    monospace_font_families: Vec<Arc<str>>,
    pub monospace_font: FontConfig,
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

impl Model {
    pub fn new() -> Model {
        Model {
            font_filter: Vec::new(),
            font_search: String::new(),
            interface_font_families: Vec::new(),
            interface_font: cosmic::config::interface_font(),
            monospace_font_families: Vec::new(),
            monospace_font: cosmic::config::monospace_font(),
        }
    }

    pub fn reset(&mut self) {
        self.font_search.clear();
        self.font_filter.clear();
    }

    pub fn font_loaded(
        &mut self,
        mono: Vec<Arc<str>>,
        interface: Vec<Arc<str>>,
    ) -> Task<crate::app::Message> {
        self.interface_font_families = interface;
        self.monospace_font_families = mono;

        Task::none()
    }

    pub fn select(
        &mut self,
        font: String,
        context_view: &ContextView,
    ) -> Option<Task<app::Message>> {
        match *context_view {
            ContextView::MonospaceFont => {
                self.monospace_font = FontConfig {
                    family: font.to_string(),
                    weight: cosmic::iced::font::Weight::Normal,
                    style: cosmic::iced::font::Style::Normal,
                    stretch: cosmic::iced::font::Stretch::Normal,
                };

                update_config(MONOSPACE_FONT, self.monospace_font.clone());
                None
            }
            ContextView::SystemFont => {
                self.interface_font = FontConfig {
                    family: font.to_string(),
                    weight: cosmic::iced::font::Weight::Normal,
                    style: cosmic::iced::font::Style::Normal,
                    stretch: cosmic::iced::font::Stretch::Normal,
                };
                update_config(INTERFACE_FONT, self.interface_font.clone());
                tokio::spawn(async move {
                    set_gnome_font_name(font.as_ref()).await;
                });
                None
            }
            _ => None,
        }
    }

    pub fn search(&mut self, input: String, context_view: &ContextView) -> Task<app::Message> {
        self.font_search = input.to_lowercase();
        self.font_filter.clear();

        let mut result: Option<Vec<Arc<str>>> = None;

        if let Some(fonts) = self.current_font_family(context_view) {
            result = Some(
                fonts
                    .iter()
                    .filter(|f| f.to_lowercase().contains(&self.font_search))
                    .cloned()
                    .collect(),
            );
        }

        if let Some(fonts) = result.as_mut() {
            self.font_filter.append(fonts);
        }
        Task::none()
    }

    pub fn search_input(&self) -> Element<'_, crate::pages::Message> {
        widget::search_input("", &self.font_search)
            .on_input(|input| Message::DrawerFont(drawer::FontMessage::Search(input)))
            .on_clear(Message::DrawerFont(drawer::FontMessage::Search(
                String::new(),
            )))
            .apply(Element::from)
            .map(crate::pages::Message::Appearance)
    }

    pub fn selection_context(
        &self,
        context_view: &ContextView,
        callback: impl Fn(Arc<str>) -> super::Message,
    ) -> Element<'_, super::Message> {
        let svg_accent = Rc::new(|theme: &cosmic::Theme| svg::Style {
            color: Some(theme.cosmic().accent_text_color().into()),
        });

        let (mut families, current_font) = match *context_view {
            ContextView::MonospaceFont => {
                (&self.monospace_font_families, &self.monospace_font.family)
            }
            ContextView::SystemFont => (&self.interface_font_families, &self.interface_font.family),
            _ => (&self.monospace_font_families, &self.monospace_font.family),
        };

        if !self.font_filter.is_empty() {
            families = &self.font_filter;
        }

        let list = families.iter().fold(widget::list_column(), |list, family| {
            let selected = &**family == current_font;
            list.add(
                settings::item_row(vec![
                    widget::text::body(&**family)
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
                .on_press(callback(family.clone())),
            )
        });
        list.into()
    }

    fn current_font_family(&self, context_view: &ContextView) -> Option<&Vec<Arc<str>>> {
        match *context_view {
            ContextView::SystemFont => Some(&self.interface_font_families),
            ContextView::MonospaceFont => Some(&self.monospace_font_families),
            _ => None,
        }
    }
}

fn update_config(variant: &str, font: FontConfig) {
    if let Ok(config) = CosmicTk::config() {
        _ = config.set(variant, font);
    }
}

/// Set the preferred icon theme for GNOME/GTK applications.
pub async fn set_gnome_font_name(font_name: &str) {
    let _res = tokio::process::Command::new("gsettings")
        .args(["set", "org.gnome.desktop.interface", "font-name", font_name])
        .status()
        .await;
}
