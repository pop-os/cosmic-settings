// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    borrow::Cow,
    collections::HashMap,
    path::{Path, PathBuf},
    time::Instant,
};

use apply::Apply;
use cosmic::{
    iced::alignment::Horizontal,
    iced::widget::{column, row},
    iced::Length,
    iced_core::{gradient::Linear, Background, Color, Degrees},
    iced_runtime::core::image::Handle as ImageHandle,
    iced_widget::core::BorderRadius,
    widget::{
        list_column,
        segmented_button::{self, SingleSelectModel},
        settings, toggler,
    },
    Element,
};
use cosmic_settings_desktop::wallpaper::{self, Entry, ScalingMode};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::{DefaultKey, SecondaryMap, SlotMap};

const FIT: usize = 0;
const STRETCH: usize = 1;
const ZOOM: usize = 2;

const MINUTES_5: usize = 0;
const MINUTES_10: usize = 1;
const MINUTES_15: usize = 2;
const MINUTES_30: usize = 3;
const HOUR_1: usize = 4;
const HOUR_2: usize = 5;

const COLORS: &[wallpaper::Color] = &[
    wallpaper::Color::Single([0.580, 0.922, 0.922]),
    wallpaper::Color::Single([0.000, 0.286, 0.427]),
    wallpaper::Color::Single([1.000, 0.678, 0.000]),
    wallpaper::Color::Single([0.282, 0.725, 0.78]),
    wallpaper::Color::Single([0.333, 0.278, 0.259]),
    wallpaper::Color::Single([0.969, 0.878, 0.384]),
    wallpaper::Color::Single([0.063, 0.165, 0.298]),
    wallpaper::Color::Single([1.000, 0.843, 0.631]),
    wallpaper::Color::Single([0.976, 0.227, 0.514]),
    wallpaper::Color::Single([1.000, 0.612, 0.867]),
    wallpaper::Color::Single([0.812, 0.49, 1.000]),
    wallpaper::Color::Single([0.835, 0.549, 1.000]),
    wallpaper::Color::Single([0.243, 0.533, 1.000]),
    wallpaper::Color::Single([0.584, 0.769, 0.988]),
    wallpaper::Color::Gradient(wallpaper::Gradient {
        colors: Cow::Borrowed(&[[1.000, 0.678, 0.000], [0.282, 0.725, 0.78]]),
        radius: 270.0,
    }),
    wallpaper::Color::Gradient(wallpaper::Gradient {
        colors: Cow::Borrowed(&[[1.000, 0.843, 0.631], [0.58, 0.922, 0.922]]),
        radius: 270.0,
    }),
    wallpaper::Color::Gradient(wallpaper::Gradient {
        colors: Cow::Borrowed(&[[1.000, 0.612, 0.867], [0.976, 0.29, 0.514]]),
        radius: 270.0,
    }),
    wallpaper::Color::Gradient(wallpaper::Gradient {
        colors: Cow::Borrowed(&[[0.584, 0.769, 0.988], [0.063, 0.165, 0.298]]),
        radius: 270.0,
    }),
];

#[derive(Clone, Debug)]
pub enum Message {
    ChangeCategory(String),
    ColorSelect(wallpaper::Color),
    Fit(String),
    Output(segmented_button::Entity),
    RotationFrequency(String),
    SameBackground(bool),
    Select(DefaultKey),
    Slideshow(bool),
    Update(Box<(wallpaper::Config, HashMap<String, String>, Context)>),
}

pub enum Category {
    SystemBackgrounds,
    Colors,
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            Self::SystemBackgrounds => "SystemBackgrounds".into(),
            Self::Colors => "Colors".into(),
        }
    }
}

pub struct Page {
    pub active_output: Option<String>,
    pub active_category: usize,
    pub categories: Vec<String>,
    pub config: wallpaper::Config,
    pub current_directory: PathBuf,
    pub fit_options: Vec<String>,
    pub outputs: SingleSelectModel,
    pub rotation_frequency: u64,
    pub rotation_options: Vec<String>,
    pub selected_fit: usize,
    pub selected_rotation: usize,
    pub selection: Context,
    pub slideshow: bool,
}

impl Default for Page {
    fn default() -> Self {
        Page {
            active_output: None,
            active_category: 0,
            categories: vec![fl!("system-backgrounds"), fl!("colors")],
            config: wallpaper::Config::default(),
            current_directory: PathBuf::from("/usr/share/backgrounds/pop/"),
            fit_options: vec![fl!("fit-to-screen"), fl!("stretch"), fl!("zoom")],
            outputs: SingleSelectModel::default(),
            rotation_frequency: 300,
            rotation_options: vec![
                fl!("x-minutes", number = 5),
                fl!("x-minutes", number = 10),
                fl!("x-minutes", number = 15),
                fl!("x-minutes", number = 30),
                fl!("x-hours", number = 1),
                fl!("x-hours", number = 2),
            ],
            selected_fit: 0,
            selected_rotation: 0,
            selection: Context::default(),
            slideshow: false,
        }
    }
}

#[derive(Clone, Debug)]
enum Choice {
    Background(DefaultKey),
    Color(wallpaper::Color),
}

impl Default for Choice {
    fn default() -> Self {
        Self::Background(DefaultKey::default())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Context {
    active: Choice,
    handles: SlotMap<DefaultKey, ImageHandle>,
    paths: SecondaryMap<DefaultKey, PathBuf>,
}

impl Page {
    /// Applies the current settings to cosmic-bg.
    pub fn apply(&mut self) {
        let output = if self.config.same_on_all {
            self.config.outputs.clear();
            String::from("all")
        } else if let Some(name) = self.outputs.active_data::<String>() {
            name.clone()
        } else {
            return;
        };

        let entry = match self.selection.active {
            Choice::Background(key) => {
                let path = if self.slideshow {
                    &self.current_directory
                } else if let Some(path) = self.selection.paths.get(key) {
                    path
                } else {
                    return;
                };

                let scaling_mode = match self.selected_fit {
                    FIT => ScalingMode::Fit([0.0, 0.0, 0.0]),
                    STRETCH => ScalingMode::Stretch,
                    ZOOM => ScalingMode::Zoom,
                    _ => return,
                };

                Entry::new(output.clone(), wallpaper::Source::Path(path.clone()))
                    .scaling_mode(scaling_mode)
                    .rotation_frequency(self.rotation_frequency)
            }

            Choice::Color(ref color) => {
                Entry::new(output.clone(), wallpaper::Source::Color(color.clone()))
            }
        };

        if output != "all" {
            self.config.backgrounds.clear();
            self.config.outputs.clear();
        }

        wallpaper::set(&mut self.config, entry);
    }

    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeCategory(category) => {
                if let Some(pos) = self.categories.iter().position(|c| c == &category) {
                    self.active_category = pos;
                }
            }
            Message::ColorSelect(color) => {
                self.selection.active = Choice::Color(color);
            }
            Message::Fit(option) => {
                self.selected_fit = self
                    .fit_options
                    .iter()
                    .enumerate()
                    .find(|(_, key)| **key == option)
                    .map_or(0, |(indice, _)| indice);
            }

            Message::Output(id) => {
                self.outputs.activate(id);
                if let Some(name) = self.outputs.data::<String>(id) {
                    self.active_output = Some(name.clone());
                }
            }

            Message::RotationFrequency(option) => {
                self.selected_rotation = self
                    .fit_options
                    .iter()
                    .enumerate()
                    .find(|(_, key)| **key == option)
                    .map_or(0, |(indice, _)| indice);

                self.rotation_frequency = match self.selected_rotation {
                    MINUTES_5 => 300,
                    MINUTES_10 => 600,
                    MINUTES_15 => 900,
                    MINUTES_30 => 1800,
                    HOUR_1 => 3600,
                    HOUR_2 => 7200,
                    _ => 10800,
                };
            }

            Message::SameBackground(value) => {
                self.config.same_on_all = value;
                self.config.backgrounds.clear();
            }

            Message::Select(id) => {
                self.selection.active = Choice::Background(id);
            }

            Message::Slideshow(value) => {
                self.slideshow = value;
            }

            Message::Update(update) => {
                self.config = update.0;
                self.selection = update.2;
                self.outputs.clear();

                {
                    let mut first = None;
                    for (name, model) in update.1 {
                        let entity = self
                            .outputs
                            .insert()
                            .text(format!("{model} ({name})"))
                            .data(name);

                        if first.is_none() {
                            first = Some(entity.id());
                        }
                    }

                    if let Some(id) = first {
                        self.outputs.activate(id);
                    }

                    if self.config.same_on_all || self.config.backgrounds.is_empty() {
                        self.apply_background(&self.config.default_background.clone());
                    } else if let Some(data) = self.outputs.active_data::<String>() {
                        for background in &self.config.backgrounds {
                            if &background.output == data {
                                self.active_output = Some(data.clone());
                                self.apply_background(&background.clone());

                                break;
                            }
                        }
                    }
                }
            }
        }

        self.apply();
    }

    /// Apply settings for the active background entry.
    fn apply_background(&mut self, entry: &wallpaper::Entry) {
        match entry.source {
            wallpaper::Source::Path(ref path) => {
                if let Some(id) = self.background_id(path) {
                    self.selection.active = Choice::Background(id);
                    self.apply_background_by_path_source(id, entry, &path.clone());
                }
            }

            wallpaper::Source::Color(ref color) => {
                self.selection.active = Choice::Color(color.clone());
                self.active_category = 1;
            }
        }
    }

    /// Apply settings for a background entry defined by path.
    fn apply_background_by_path_source(
        &mut self,
        entity: DefaultKey,
        entry: &wallpaper::Entry,
        path: &Path,
    ) {
        self.selection.active = Choice::Background(entity);

        match entry.scaling_mode {
            ScalingMode::Fit(_) => self.selected_fit = FIT,
            ScalingMode::Stretch => self.selected_fit = STRETCH,
            ScalingMode::Zoom => self.selected_fit = ZOOM,
        }

        self.slideshow = path.is_dir();

        match entry.rotation_frequency {
            600 => self.selected_rotation = MINUTES_10,
            900 => self.selected_rotation = MINUTES_15,
            1800 => self.selected_rotation = MINUTES_30,
            3600 => self.selected_rotation = HOUR_1,
            7200 => self.selected_rotation = HOUR_2,
            _ => self.selected_rotation = MINUTES_5,
        }

        self.rotation_frequency = entry.rotation_frequency;
    }

    /// Locate the ID of a background that's already stored in memory
    fn background_id(&self, path: &Path) -> Option<DefaultKey> {
        self.selection
            .paths
            .iter()
            .find(|(_id, background)| *background == path)
            .map(|(id, _)| id)
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(settings())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("wallpaper", "preferences-desktop-wallpaper-symbolic")
            .title(fl!("wallpaper"))
            .description(fl!("wallpaper", "desc"))
    }

    fn load(&self, _page: page::Entity) -> Option<page::Task<crate::pages::Message>> {
        Some(Box::pin(async move {
            let (config, outputs) = wallpaper::config();

            let mut backgrounds =
                wallpaper::load_each_from_path("/usr/share/backgrounds/pop/".into());

            let mut update = Context::default();

            let start = Instant::now();

            while let Some((path, image)) = backgrounds.recv().await {
                let handle =
                    ImageHandle::from_pixels(image.width(), image.height(), image.into_vec());

                let id = update.handles.insert(handle);
                update.paths.insert(id, path);
            }

            tracing::info!(
                "loaded wallpapers in {:?}",
                Instant::now().duration_since(start)
            );

            crate::pages::Message::DesktopWallpaper(Message::Update(Box::new((
                config, outputs, update,
            ))))
        }))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

#[allow(clippy::too_many_lines)]
pub fn settings() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![
            fl!("wallpaper", "same"),
            fl!("wallpaper", "fit"),
            fl!("wallpaper", "slide"),
            fl!("wallpaper", "change"),
        ])
        .view::<Page>(|_binder, page, section| {
            let descriptions = &section.descriptions;

            let mut children = Vec::with_capacity(3);

            let mut is_color = false;

            let display_demo = match page.selection.active {
                Choice::Background(key) => page.selection.handles.get(key).map(|image| {
                    cosmic::iced::widget::image(image.clone())
                        .width(Length::Fixed(300.0))
                        .into()
                }),

                Choice::Color(ref color) => {
                    is_color = true;
                    Some(color_image(color.clone(), 300, 169, 0.0))
                }
            };

            if let Some(element) = display_demo {
                children.push(crate::widget::display_container(element));
            }

            children.push(if page.config.same_on_all {
                cosmic::widget::text(fl!("all-displays"))
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill)
                    .apply(cosmic::iced::widget::container)
                    .width(Length::Fill)
                    .padding([0, 0, 16, 0])
                    .into()
            } else {
                cosmic::widget::horizontal_segmented_selection(&page.outputs)
                    .on_activate(Message::Output)
                    .into()
            });

            let background_fit = cosmic::iced::widget::pick_list(
                &page.fit_options,
                page.fit_options.get(page.selected_fit).cloned(),
                Message::Fit,
            );

            children.push({
                let column = list_column()
                    .add(settings::item(
                        &descriptions[0],
                        toggler(None, page.config.same_on_all, Message::SameBackground),
                    ))
                    .add(settings::item(&descriptions[1], background_fit))
                    .add(settings::item(
                        &descriptions[2],
                        toggler(None, page.slideshow, Message::Slideshow),
                    ));

                if !is_color && page.slideshow {
                    column
                        .add(settings::item(
                            &descriptions[3],
                            cosmic::iced::widget::pick_list(
                                &page.rotation_options,
                                page.rotation_options.get(page.selected_rotation).cloned(),
                                Message::RotationFrequency,
                            ),
                        ))
                        .into()
                } else {
                    column.into()
                }
            });

            let category_selection = cosmic::iced::widget::pick_list(
                &page.categories,
                Some(page.categories[page.active_category].clone()),
                Message::ChangeCategory,
            );

            children.push(category_selection.into());

            match page.active_category {
                0 => {
                    let mut image_column = Vec::with_capacity(page.selection.handles.len() / 4);
                    let mut image_handles = page.selection.handles.iter();

                    while let Some((id, handle)) = image_handles.next() {
                        let mut image_row = Vec::with_capacity(4);

                        image_row.push(wallpaper_button(handle, id));

                        for (id, handle) in image_handles.by_ref().take(3) {
                            image_row.push(wallpaper_button(handle, id));
                        }

                        image_column.push(row(image_row).spacing(16).into());
                    }

                    children.push(column(image_column).spacing(12).padding(0).into());
                }

                1 => {
                    let mut color_column = Vec::with_capacity(COLORS.len() / 8);
                    let mut colors = COLORS.iter();

                    while let Some(color) = colors.next() {
                        let mut color_row = Vec::with_capacity(8);

                        color_row.push(color_button(color.clone()));

                        for color in colors.by_ref().take(7) {
                            color_row.push(color_button(color.clone()));
                        }

                        color_column.push(row(color_row).spacing(16).into());
                    }

                    children.push(column(color_column).spacing(12).padding(0).into());
                }

                _ => (),
            }

            cosmic::iced::widget::column(children)
                .spacing(22)
                .padding(0)
                .max_width(683)
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWallpaper)
        })
}

fn color_image(
    color: wallpaper::Color,
    width: u16,
    height: u16,
    border_radius: f32,
) -> Element<'static, Message> {
    cosmic::iced::widget::container(cosmic::iced::widget::space::Space::new(width, height))
        .style(cosmic::theme::Container::custom(move |_theme| {
            cosmic::iced::widget::container::Appearance {
                text_color: None,
                background: Some(match &color {
                    wallpaper::Color::Single([r, g, b]) => {
                        Background::Color(Color::from_rgb(*r, *g, *b))
                    }
                    wallpaper::Color::Gradient(wallpaper::Gradient { colors, radius }) => {
                        let stop_increment = 1.0 / (colors.len() - 1) as f32;
                        let mut stop = 0.0;

                        let mut linear = Linear::new(Degrees(*radius));

                        for &[r, g, b] in &**colors {
                            linear = linear.add_stop(stop, cosmic::iced::Color::from_rgb(r, g, b));
                            stop += stop_increment;
                        }

                        Background::Gradient(cosmic::iced_core::Gradient::Linear(linear))
                    }
                }),
                border_radius: BorderRadius::from(border_radius),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }))
        .into()
}

fn color_button(color: wallpaper::Color) -> Element<'static, Message> {
    cosmic::iced::widget::button(color_image(color.clone(), 70, 70, 8.0))
        .width(Length::Fixed(71.0))
        .height(Length::Fixed(71.0))
        .style(cosmic::theme::Button::Transparent)
        .on_press(Message::ColorSelect(color))
        .into()
}

fn wallpaper_button(handle: &ImageHandle, id: DefaultKey) -> Element<Message> {
    let image = cosmic::iced::widget::image(handle.clone()).apply(cosmic::iced::Element::from);

    cosmic::iced::widget::button(image)
        .width(Length::Fixed(158.0))
        .height(Length::Fixed(105.0))
        .style(cosmic::theme::Button::Transparent)
        .on_press(Message::Select(id))
        .into()
}
