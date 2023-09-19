// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod widgets;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Instant,
};

use apply::Apply;
use cosmic::widget::{
    list_column,
    segmented_button::{self, SingleSelectModel},
    segmented_selection, settings, text, toggler,
};
use cosmic::{iced::Length, Element};
use cosmic::{iced_core::alignment, iced_runtime::core::image::Handle as ImageHandle};
use cosmic_settings_desktop::wallpaper::{self, Entry, ScalingMode};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use image::imageops::FilterType::Lanczos3;
use slotmap::{DefaultKey, SecondaryMap, SlotMap};

const SYSTEM_WALLPAPER_DIR: &str = "/usr/share/backgrounds/pop/";

const CATEGORY_SYSTEM_WALLPAPERS: usize = 0;
const CATEGORY_COLOR: usize = 1;

const FIT: usize = 0;
const STRETCH: usize = 1;
const ZOOM: usize = 2;

const SIMULATED_WIDTH: u16 = 300;
const SIMULATED_HEIGHT: u16 = 169;

const MINUTES_5: usize = 0;
const MINUTES_10: usize = 1;
const MINUTES_15: usize = 2;
const MINUTES_30: usize = 3;
const HOUR_1: usize = 4;
const HOUR_2: usize = 5;

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

pub struct Page {
    pub active_output: Option<String>,
    pub active_category: usize,
    pub cached_display_handle: Option<ImageHandle>,
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
}

impl Default for Page {
    fn default() -> Self {
        Page {
            active_output: None,
            active_category: CATEGORY_SYSTEM_WALLPAPERS,
            cached_display_handle: None,
            categories: vec![fl!("system-backgrounds"), fl!("colors")],
            config: wallpaper::Config::default(),
            current_directory: PathBuf::from(SYSTEM_WALLPAPER_DIR),
            fit_options: vec![fl!("fit-to-screen"), fl!("stretch"), fl!("zoom")],
            outputs: SingleSelectModel::default(),
            rotation_frequency: 300,
            rotation_options: vec![
                // FIX: fluent is inserting extra unicode characters in formatting
                fl!("x-minutes", number = 5)
                    .replace('\u{2068}', "")
                    .replace('\u{2069}', ""),
                fl!("x-minutes", number = 10)
                    .replace('\u{2068}', "")
                    .replace('\u{2069}', ""),
                fl!("x-minutes", number = 15)
                    .replace('\u{2068}', "")
                    .replace('\u{2069}', ""),
                fl!("x-minutes", number = 30)
                    .replace('\u{2068}', "")
                    .replace('\u{2069}', ""),
                fl!("x-hours", number = 1)
                    .replace('\u{2068}', "")
                    .replace('\u{2069}', ""),
                fl!("x-hours", number = 2)
                    .replace('\u{2068}', "")
                    .replace('\u{2069}', ""),
            ],
            selected_fit: 0,
            selected_rotation: 0,
            selection: Context::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Choice {
    Background(DefaultKey),
    Color(wallpaper::Color),
    Slideshow,
}

impl Default for Choice {
    fn default() -> Self {
        Self::Background(DefaultKey::default())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Context {
    active: Choice,
    paths: SlotMap<DefaultKey, PathBuf>,
    display_images: SecondaryMap<DefaultKey, image::RgbaImage>,
    selection_handles: SecondaryMap<DefaultKey, ImageHandle>,
}

impl Page {
    fn cache_display_image(&mut self) {
        let choice = match self.selection.active {
            Choice::Background(id) => self.selection.display_images.get(id),

            Choice::Slideshow => self.selection.display_images.values().next(),

            Choice::Color(_) => None,
        };

        let Some(image) = choice else {
            return;
        };

        self.cached_display_handle = None;

        let temp_image;

        let image = match self.selected_fit {
            FIT => image,

            STRETCH => {
                temp_image = image::imageops::resize(
                    image,
                    SIMULATED_WIDTH as u32,
                    SIMULATED_HEIGHT as u32,
                    Lanczos3,
                );
                &temp_image
            }

            ZOOM => {
                let (w, h) = (image.width(), image.height());

                let ratio =
                    (SIMULATED_WIDTH as f64 / w as f64).max(SIMULATED_HEIGHT as f64 / h as f64);

                let (new_width, new_height) = (
                    (w as f64 * ratio).round() as u32,
                    (h as f64 * ratio).round() as u32,
                );

                let mut new_image = image::imageops::resize(image, new_width, new_height, Lanczos3);

                temp_image = image::imageops::crop(
                    &mut new_image,
                    (new_width - SIMULATED_WIDTH as u32) / 2,
                    (new_height - SIMULATED_HEIGHT as u32) / 2,
                    SIMULATED_WIDTH as u32,
                    SIMULATED_HEIGHT as u32,
                )
                .to_image();

                &temp_image
            }

            _ => return,
        };

        self.cached_display_handle = Some(ImageHandle::from_pixels(
            image.width(),
            image.height(),
            image.to_vec(),
        ));
    }

    fn config_output(&self) -> Option<String> {
        if self.config.same_on_all {
            Some(String::from("all"))
        } else {
            self.outputs.active_data::<String>().cloned()
        }
    }

    /// Applies the current settings to cosmic-bg.
    pub fn config_apply(&mut self) {
        let Some(output) = self.config_output() else {
            return;
        };

        if self.config.same_on_all {
            self.config.outputs.clear();
        }

        let entry = match self.selection.active {
            Choice::Slideshow => {
                match self.config_background_entry(output.clone(), self.current_directory.clone()) {
                    Some(entry) => entry,
                    None => return,
                }
            }
            Choice::Background(key) => {
                if let Some(path) = self.selection.paths.get(key) {
                    match self.config_background_entry(output.clone(), path.clone()) {
                        Some(entry) => entry,
                        None => return,
                    }
                } else {
                    return;
                }
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

    fn config_background_entry(&self, output: String, path: PathBuf) -> Option<Entry> {
        let scaling_mode = match self.selected_fit {
            FIT => ScalingMode::Fit([0.0, 0.0, 0.0]),
            STRETCH => ScalingMode::Stretch,
            ZOOM => ScalingMode::Zoom,
            _ => return None,
        };

        Entry::new(output, wallpaper::Source::Path(path))
            .scaling_mode(scaling_mode)
            .rotation_frequency(self.rotation_frequency)
            .apply(Some)
    }

    fn config_update(
        &mut self,
        config: wallpaper::Config,
        displays: HashMap<String, String>,
        selection: Context,
    ) {
        self.config = config;
        self.selection = selection;
        self.outputs.clear();

        let mut first = None;
        for (name, model) in displays {
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
            let entry = self.config.default_background.clone();
            self.select_background_entry(&entry);

            if let Some(current) = entry_directory(&entry) {
                self.current_directory = current;
            }
        } else if let Some(data) = self.outputs.active_data::<String>() {
            let mut backgrounds = Vec::new();
            std::mem::swap(&mut self.config.backgrounds, &mut backgrounds);

            for background in &backgrounds {
                if &background.output == data {
                    self.active_output = Some(data.clone());
                    self.select_background_entry(background);

                    if let Some(current) = entry_directory(background) {
                        self.current_directory = current;
                    }

                    break;
                }
            }

            std::mem::swap(&mut self.config.backgrounds, &mut backgrounds);
        }
    }

    /// Changes the selection category, such as wallpaper select or color select.
    fn change_category(&mut self, category: &str) {
        if let Some(pos) = self.categories.iter().position(|c| c == category) {
            self.active_category = pos;
            match pos {
                CATEGORY_SYSTEM_WALLPAPERS => {
                    self.select_first_background();
                }

                CATEGORY_COLOR => {
                    self.selection.active = Choice::Color(wallpaper::DEFAULT_COLORS[0].clone());
                    self.cache_display_image();
                }

                _ => (),
            }
        }
    }

    /// Changes the output being configured
    pub fn change_output(&mut self, entity: segmented_button::Entity) {
        self.outputs.activate(entity);
        if let Some(name) = self.outputs.data::<String>(entity) {
            self.active_output = Some(name.clone());
        }
    }

    // Changes the slideshow background rotation frequency
    pub fn change_rotation_frequency(&mut self, option: &str) {
        self.selected_rotation = self
            .rotation_options
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

    #[must_use]
    pub fn display_image_view(&self) -> cosmic::Element<Message> {
        match self.cached_display_handle {
            Some(ref handle) => cosmic::iced::widget::image(handle.clone())
                .width(Length::Fixed(SIMULATED_WIDTH as f32))
                .into(),

            None => cosmic::iced::widget::Space::new(SIMULATED_WIDTH, SIMULATED_HEIGHT).into(),
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeCategory(category) => self.change_category(&category),

            Message::ColorSelect(color) => {
                self.selection.active = Choice::Color(color);
                self.cached_display_handle = None;
            }

            Message::Fit(option) => {
                self.selected_fit = self
                    .fit_options
                    .iter()
                    .enumerate()
                    .find(|(_, key)| **key == option)
                    .map_or(0, |(indice, _)| indice);

                self.cache_display_image();
            }

            Message::Output(id) => self.change_output(id),

            Message::RotationFrequency(option) => self.change_rotation_frequency(&option),

            Message::SameBackground(value) => {
                self.config.same_on_all = value;
                self.config.backgrounds.clear();
            }

            Message::Select(id) => {
                self.selection.active = Choice::Background(id);
                self.cache_display_image();
            }

            Message::Slideshow(enable) => {
                if enable {
                    self.selection.active = Choice::Slideshow;
                    self.cache_display_image();
                } else {
                    self.select_first_background();
                }
            }

            Message::Update(update) => self.config_update(update.0, update.1, update.2),
        }

        self.config_apply();
    }

    /// Selects the given background entry.
    fn select_background_entry(&mut self, entry: &wallpaper::Entry) {
        match entry.source {
            wallpaper::Source::Path(ref path) => {
                if path.is_dir() {
                    self.selection.active = Choice::Slideshow;
                    self.cache_display_image();
                } else if let Some(entity) = self.background_id_from_path(path) {
                    self.select_background(entry, entity, path.is_dir());
                }
            }

            wallpaper::Source::Color(ref color) => {
                self.selection.active = Choice::Color(color.clone());
                self.active_category = CATEGORY_COLOR;
                self.cache_display_image();
            }
        }
    }

    /// Selects the first background from the wallpaper select options.
    fn select_first_background(&mut self) {
        if let Some((entity, path)) = self.selection.paths.iter().next() {
            if let Some(output) = self.config_output() {
                if let Some(entry) = self.config_background_entry(output, path.clone()) {
                    self.select_background(&entry, entity, path.is_dir());
                }
            }
        }
    }

    /// Selects the given background
    fn select_background(
        &mut self,
        entry: &wallpaper::Entry,
        entity: DefaultKey,
        is_slideshow: bool,
    ) {
        self.selection.active = if is_slideshow {
            Choice::Slideshow
        } else {
            Choice::Background(entity)
        };

        match entry.scaling_mode {
            ScalingMode::Fit(_) => self.selected_fit = FIT,
            ScalingMode::Stretch => self.selected_fit = STRETCH,
            ScalingMode::Zoom => self.selected_fit = ZOOM,
        }

        match entry.rotation_frequency {
            600 => self.selected_rotation = MINUTES_10,
            900 => self.selected_rotation = MINUTES_15,
            1800 => self.selected_rotation = MINUTES_30,
            3600 => self.selected_rotation = HOUR_1,
            7200 => self.selected_rotation = HOUR_2,
            _ => self.selected_rotation = MINUTES_5,
        }

        self.rotation_frequency = entry.rotation_frequency;

        self.cache_display_image();
    }

    /// Locate the ID of a background that's already stored in memory
    fn background_id_from_path(&self, path: &Path) -> Option<DefaultKey> {
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
            let start = Instant::now();

            let (config, outputs) = wallpaper::config();

            let mut backgrounds = wallpaper::load_each_from_path(SYSTEM_WALLPAPER_DIR.into());

            let mut update = Context::default();

            while let Some((path, display_image, selection_image)) = backgrounds.recv().await {
                let id = update.paths.insert(path);

                update.display_images.insert(id, display_image);

                let selection_handle = ImageHandle::from_pixels(
                    selection_image.width(),
                    selection_image.height(),
                    selection_image.into_vec(),
                );
                update.selection_handles.insert(id, selection_handle);
            }

            tracing::debug!(
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

            let mut show_slideshow_toggle = true;
            let mut slideshow_enabled = false;

            children.push(crate::widget::display_container(
                match page.selection.active {
                    // Shows background options, with the slideshow toggle enabled
                    Choice::Slideshow => {
                        slideshow_enabled = true;
                        page.display_image_view()
                    }

                    // Shows background options, with the slideshow toggle visible
                    Choice::Background(_) => page.display_image_view(),

                    // Displays color options, and hides the slideshow toggle
                    Choice::Color(ref color) => {
                        show_slideshow_toggle = false;
                        widgets::color_image(color.clone(), SIMULATED_WIDTH, SIMULATED_HEIGHT, 0.0)
                    }
                },
            ));

            children.push(if page.config.same_on_all {
                text(fl!("all-displays"))
                    .font(cosmic::font::FONT_SEMIBOLD)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .apply(cosmic::iced::widget::container)
                    .width(Length::Fill)
                    .height(Length::Fixed(32.0))
                    .into()
            } else {
                segmented_selection::horizontal(&page.outputs)
                    .on_activate(Message::Output)
                    .into()
            });

            let background_fit = cosmic::iced::widget::pick_list(
                &page.fit_options,
                page.fit_options.get(page.selected_fit).cloned(),
                Message::Fit,
            );

            children.push({
                let mut column = list_column()
                    .add(settings::item(
                        &descriptions[0],
                        toggler(None, page.config.same_on_all, Message::SameBackground),
                    ))
                    .add(settings::item(&descriptions[1], background_fit));

                if show_slideshow_toggle {
                    column = column.add(settings::item(
                        &descriptions[2],
                        toggler(None, slideshow_enabled, Message::Slideshow),
                    ));
                }

                // The rotation frequency pick list should only be shown when the slideshow is enabled.
                if slideshow_enabled {
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
                // Displays system wallpapers that are available to select from
                CATEGORY_SYSTEM_WALLPAPERS => {
                    children.push(widgets::wallpaper_select_options(page));
                }

                // Displays colors and gradients that are available to select from
                CATEGORY_COLOR => {
                    children.push(widgets::color_select_options());
                }

                _ => (),
            }

            cosmic::iced::widget::column(children)
                .spacing(22)
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWallpaper)
        })
}

/// Sets the current wallpaper directory.
fn entry_directory(entry: &wallpaper::Entry) -> Option<PathBuf> {
    Some(match entry.source {
        wallpaper::Source::Path(ref path) => {
            if path.is_dir() {
                path.clone()
            } else if let Some(path) = path.parent() {
                path.to_path_buf()
            } else {
                return None;
            }
        }

        wallpaper::Source::Color(_) => PathBuf::from(SYSTEM_WALLPAPER_DIR),
    })
}
