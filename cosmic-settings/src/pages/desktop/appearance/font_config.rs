use cosmic::{
    config::{CosmicTk, FontConfig},
    iced::Length,
    widget::{self, radio, settings, text},
    Command, Element,
};
use cosmic_config::ConfigSet;
use ustr::Ustr;

const INTERFACE_FONT: &str = "interface_font";
const MONOSPACE_FONT: &str = "monospace_font";

pub fn interface_font_view(page: &super::Page) -> Element<crate::pages::Message> {
    let search = widget::search_input(
        fl!("type-to-search"),
        &page.font_config.interface_font_search,
    )
    .on_input(|s| {
        crate::pages::Message::Appearance(super::Message::FontConfig(Message::InterfaceFontSearch(
            s,
        )))
    })
    .on_clear(crate::pages::Message::Appearance(
        super::Message::FontConfig(Message::InterfaceFontSearch(String::new())),
    ));

    let mut list = widget::list_column();

    let search_input = page.font_config.interface_font_search.trim().to_lowercase();

    for (index, name) in page.font_config.interface_font_families.iter().enumerate() {
        if search_input.is_empty() || name.to_lowercase().contains(&search_input) {
            let radio = radio(
                text::body(name),
                index,
                page.font_config.interface_font_family,
                |id| {
                    crate::pages::Message::Appearance(super::Message::FontConfig(
                        Message::InterfaceFontFamily(id),
                    ))
                },
            )
            .width(Length::Fill);

            list = list.add(settings::item_row(vec![radio.into()]));
        }
    }

    widget::column()
        .padding([2, 0])
        .spacing(32)
        .push(search)
        .push(list)
        .into()
}

pub fn monospace_font_view(page: &super::Page) -> Element<crate::pages::Message> {
    let search = widget::search_input(
        fl!("type-to-search"),
        &page.font_config.monospace_font_search,
    )
    .on_input(|s| {
        crate::pages::Message::Appearance(super::Message::FontConfig(Message::MonospaceFontSearch(
            s,
        )))
    })
    .on_clear(crate::pages::Message::Appearance(
        super::Message::FontConfig(Message::MonospaceFontSearch(String::new())),
    ));

    let mut list = widget::list_column();

    let search_input = page.font_config.monospace_font_search.trim().to_lowercase();

    for (index, name) in page.font_config.monospace_font_families.iter().enumerate() {
        if search_input.is_empty() || name.to_lowercase().contains(&search_input) {
            let radio = radio(
                text::body(name),
                index,
                page.font_config.monospace_font_family,
                |id| {
                    crate::pages::Message::Appearance(super::Message::FontConfig(
                        Message::MonospaceFontFamily(id),
                    ))
                },
            )
            .width(Length::Fill);

            list = list.add(settings::item_row(vec![radio.into()]));
        }
    }

    widget::column()
        .padding([2, 0])
        .spacing(32)
        .push(search)
        .push(list)
        .into()
}

#[derive(Debug, Clone)]
pub enum Message {
    InterfaceFontFamily(usize),
    InterfaceFontStretch(usize),
    InterfaceFontStyle(usize),
    InterfaceFontWeight(usize),
    InterfaceFontSearch(String),
    LoadedFonts(Vec<String>, Vec<String>),
    MonospaceFontFamily(usize),
    MonospaceFontWeight(usize),
    MonospaceFontSearch(String),
}

#[derive(Debug, Default)]
pub struct Model {
    pub font_weights: Vec<String>,
    pub font_stretches: Vec<String>,
    pub font_styles: Vec<String>,
    pub interface_font_families: Vec<String>,
    pub interface_font_family: Option<usize>,
    pub interface_font_weight: Option<usize>,
    pub interface_font_stretch: Option<usize>,
    pub interface_font_style: Option<usize>,
    pub interface_font_search: String,
    pub monospace_font_families: Vec<String>,
    pub monospace_font_family: Option<usize>,
    pub monospace_font_weight: Option<usize>,
    pub monospace_font_search: String,
}

impl Model {
    pub fn new() -> Model {
        Model {
            font_weights: vec![
                fl!("font-weight", "thin"),
                fl!("font-weight", "extra-light"),
                fl!("font-weight", "light"),
                fl!("font-weight", "normal"),
                fl!("font-weight", "medium"),
                fl!("font-weight", "semibold"),
                fl!("font-weight", "bold"),
                fl!("font-weight", "extra-bold"),
                fl!("font-weight", "black"),
            ],

            font_stretches: vec![
                fl!("font-stretch", "condensed"),
                fl!("font-stretch", "normal"),
                fl!("font-stretch", "expanded"),
            ],

            font_styles: vec![
                fl!("font-style", "normal"),
                fl!("font-style", "italic"),
                fl!("font-style", "oblique"),
            ],

            interface_font_families: Vec::new(),
            interface_font_family: None,
            interface_font_stretch: None,
            interface_font_style: None,
            interface_font_weight: None,
            interface_font_search: String::new(),
            monospace_font_families: Vec::new(),
            monospace_font_family: None,
            monospace_font_weight: None,
            monospace_font_search: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        match message {
            Message::InterfaceFontFamily(id) => {
                if let Some(family) = self.interface_font_families.get(id) {
                    update_config(
                        INTERFACE_FONT,
                        FontConfig {
                            family: Ustr::from(family),
                            ..cosmic::config::interface_font()
                        },
                    );

                    self.interface_font_family = Some(id);
                }
            }

            Message::InterfaceFontStretch(id) => {
                update_config(
                    INTERFACE_FONT,
                    FontConfig {
                        stretch: font_stretch_by_id(id),
                        ..cosmic::config::interface_font()
                    },
                );

                self.interface_font_stretch = Some(id);
            }

            Message::InterfaceFontStyle(id) => {
                update_config(
                    INTERFACE_FONT,
                    FontConfig {
                        style: font_style_by_id(id),
                        ..cosmic::config::interface_font()
                    },
                );

                self.interface_font_style = Some(id);
            }

            Message::InterfaceFontWeight(id) => {
                update_config(
                    INTERFACE_FONT,
                    FontConfig {
                        weight: font_weight_by_id(id),
                        ..cosmic::config::interface_font()
                    },
                );

                self.interface_font_weight = Some(id);
            }

            Message::InterfaceFontSearch(search) => {
                self.interface_font_search = search;
            }

            Message::LoadedFonts(interface, mono) => {
                self.interface_font_families = interface;
                self.monospace_font_families = mono;

                let interface_font = cosmic::config::interface_font();
                let monospace_font = cosmic::config::monospace_font();

                self.interface_font_stretch = font_stretch_to_pos(interface_font.stretch);
                self.interface_font_style = font_style_to_pos(interface_font.style);
                self.interface_font_weight = font_weight_to_pos(interface_font.weight);
                self.interface_font_family =
                    font_family_to_pos(&self.interface_font_families, &interface_font.family);

                self.monospace_font_weight = font_weight_to_pos(monospace_font.weight);
                self.monospace_font_family =
                    font_family_to_pos(&self.monospace_font_families, &monospace_font.family);
            }

            Message::MonospaceFontFamily(id) => {
                if let Some(family) = self.monospace_font_families.get(id) {
                    update_config(
                        MONOSPACE_FONT,
                        FontConfig {
                            family: Ustr::from(family),
                            ..cosmic::config::monospace_font()
                        },
                    );

                    self.monospace_font_family = Some(id);
                }
            }

            Message::MonospaceFontWeight(id) => {
                update_config(
                    MONOSPACE_FONT,
                    FontConfig {
                        weight: font_weight_by_id(id),
                        ..cosmic::config::monospace_font()
                    },
                );

                self.monospace_font_weight = Some(id);
            }

            Message::MonospaceFontSearch(search) => {
                self.monospace_font_search = search;
            }
        }

        Command::none()
    }
}

fn font_family_to_pos(families: &[String], family: &str) -> Option<usize> {
    families.iter().position(|f| f.as_str() == family)
}

fn font_weight_by_id(id: usize) -> cosmic::iced::font::Weight {
    match id {
        0 => cosmic::iced::font::Weight::Thin,
        1 => cosmic::iced::font::Weight::ExtraLight,
        2 => cosmic::iced::font::Weight::Light,
        3 => cosmic::iced::font::Weight::Normal,
        4 => cosmic::iced::font::Weight::Medium,
        5 => cosmic::iced::font::Weight::Semibold,
        6 => cosmic::iced::font::Weight::Bold,
        7 => cosmic::iced::font::Weight::ExtraBold,
        8 => cosmic::iced::font::Weight::Black,
        _ => cosmic::iced::font::Weight::Normal,
    }
}
fn font_weight_to_pos(weight: cosmic::iced::font::Weight) -> Option<usize> {
    match weight {
        cosmic::iced::font::Weight::Thin => Some(0),
        cosmic::iced::font::Weight::Light => Some(1),
        cosmic::iced::font::Weight::ExtraLight => Some(2),
        cosmic::iced::font::Weight::Normal => Some(3),
        cosmic::iced::font::Weight::Medium => Some(4),
        cosmic::iced::font::Weight::Semibold => Some(5),
        cosmic::iced::font::Weight::Bold => Some(6),
        cosmic::iced::font::Weight::ExtraBold => Some(7),
        cosmic::iced::font::Weight::Black => Some(8),
    }
}

fn font_stretch_by_id(id: usize) -> cosmic::iced::font::Stretch {
    match id {
        0 => cosmic::iced::font::Stretch::Condensed,
        1 => cosmic::iced::font::Stretch::Normal,
        2 => cosmic::iced::font::Stretch::Expanded,
        _ => cosmic::iced::font::Stretch::Normal,
    }
}

fn font_stretch_to_pos(stretch: cosmic::iced::font::Stretch) -> Option<usize> {
    match stretch {
        cosmic::iced::font::Stretch::Condensed => Some(0),
        cosmic::iced::font::Stretch::Normal => Some(1),
        cosmic::iced::font::Stretch::Expanded => Some(2),
        _ => None,
    }
}

fn font_style_by_id(id: usize) -> cosmic::iced::font::Style {
    match id {
        0 => cosmic::iced::font::Style::Normal,
        1 => cosmic::iced::font::Style::Italic,
        2 => cosmic::iced::font::Style::Oblique,
        _ => cosmic::iced::font::Style::Normal,
    }
}

fn font_style_to_pos(style: cosmic::iced::font::Style) -> Option<usize> {
    match style {
        cosmic::iced::font::Style::Normal => Some(0),
        cosmic::iced::font::Style::Italic => Some(1),
        cosmic::iced::font::Style::Oblique => Some(2),
    }
}

fn update_config(variant: &str, font: FontConfig) {
    if let Ok(config) = CosmicTk::config() {
        _ = config.set(variant, font);
    }
}

pub fn load_font_families() -> (Vec<String>, Vec<String>) {
    let mut font_system = cosmic::iced::advanced::graphics::text::font_system()
        .write()
        .unwrap();

    let (mut interface, mut mono) = font_system.raw().db().faces().fold(
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
                if mono.last().map_or(true, |name| name != font_name) {
                    mono.push(font_name.clone());
                }
            } else if interface.last().map_or(true, |name| name != font_name) {
                interface.push(font_name.clone());
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
