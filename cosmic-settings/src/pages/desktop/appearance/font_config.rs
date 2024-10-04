use cosmic::{
    config::{CosmicTk, FontConfig},
    widget::{self, settings},
    Apply, Command, Element,
};
use cosmic_config::ConfigSet;
use cosmic_settings_page::Section;
use ustr::Ustr;

const INTERFACE_FONT: &str = "interface_font";
const MONOSPACE_FONT: &str = "monospace_font";

pub fn section() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        font_family_txt = fl!("font-family");
        font_weight_txt = fl!("font-weight");
        font_stretch_txt = fl!("font-stretch");
        font_style_txt = fl!("font-style");
        interface_font_txt = fl!("interface-font");
        monospace_font_txt = fl!("monospace-font");
    });

    Section::default()
        .title(fl!("font-config"))
        .descriptions(descriptions)
        .view::<super::Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            let interface_font_family = settings::item::builder(&descriptions[font_family_txt])
                .control(widget::dropdown(
                    &page.font_config.interface_font_families,
                    page.font_config.interface_font_family,
                    |id| super::Message::FontConfig(Message::InterfaceFontFamily(id)),
                ));

            let interface_font_weight = settings::item::builder(&descriptions[font_weight_txt])
                .control(widget::dropdown(
                    &page.font_config.font_weights,
                    page.font_config.interface_font_weight,
                    |id| super::Message::FontConfig(Message::InterfaceFontWeight(id)),
                ));

            let interface_font_stretch = settings::item::builder(&descriptions[font_stretch_txt])
                .control(widget::dropdown(
                    &page.font_config.font_stretches,
                    page.font_config.interface_font_stretch,
                    |id| super::Message::FontConfig(Message::InterfaceFontStretch(id)),
                ));

            let interface_font_style = settings::item::builder(&descriptions[font_style_txt])
                .control(widget::dropdown(
                    &page.font_config.font_styles,
                    page.font_config.interface_font_style,
                    |id| super::Message::FontConfig(Message::InterfaceFontStyle(id)),
                ));

            let monospace_font_family = settings::item::builder(&descriptions[font_family_txt])
                .control(widget::dropdown(
                    &page.font_config.monospace_font_families,
                    page.font_config.monospace_font_family,
                    |id| super::Message::FontConfig(Message::MonospaceFontFamily(id)),
                ));

            let monospace_font_weight = settings::item::builder(&descriptions[font_weight_txt])
                .control(widget::dropdown(
                    &page.font_config.font_weights,
                    page.font_config.monospace_font_weight,
                    |id| super::Message::FontConfig(Message::MonospaceFontWeight(id)),
                ));

            let interface_font = settings::section()
                .title(&descriptions[interface_font_txt])
                .add(interface_font_family)
                .add(interface_font_weight)
                .add(interface_font_stretch)
                .add(interface_font_style);

            let monospace_font = settings::section()
                .title(&descriptions[monospace_font_txt])
                .add(monospace_font_family)
                .add(monospace_font_weight);

            widget::column::with_capacity(2)
                .push(interface_font)
                .push(monospace_font)
                .spacing(8)
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

#[derive(Debug, Clone)]
pub enum Message {
    InterfaceFontFamily(usize),
    InterfaceFontStretch(usize),
    InterfaceFontStyle(usize),
    InterfaceFontWeight(usize),
    LoadedFonts(Vec<String>, Vec<String>),
    MonospaceFontFamily(usize),
    MonospaceFontWeight(usize),
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
    pub monospace_font_families: Vec<String>,
    pub monospace_font_family: Option<usize>,
    pub monospace_font_weight: Option<usize>,
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
            monospace_font_families: Vec::new(),
            monospace_font_family: None,
            monospace_font_weight: None,
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
