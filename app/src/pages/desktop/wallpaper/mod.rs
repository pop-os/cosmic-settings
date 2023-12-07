// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod config;
pub mod widgets;

pub use config::Config;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use apply::Apply;
use cosmic::{iced::Length, Element};
use cosmic::{iced_core::alignment, iced_runtime::core::image::Handle as ImageHandle};
use cosmic::{
    widget::{
        button, dropdown, list_column, row,
        segmented_button::{self, SingleSelectModel},
        segmented_selection, settings, text, toggler,
    },
    Command,
};
use cosmic_settings_desktop::wallpaper::{self, Entry, ScalingMode};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use image::imageops::FilterType::Lanczos3;
use image::{ImageBuffer, Rgba};
use slotmap::{DefaultKey, SecondaryMap, SlotMap};
use static_init::dynamic;

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

pub type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Clone, Debug)]
pub enum Message {
    ChangeFolder(Context),
    ColorAdd(wallpaper::Color),
    ColorAddDialog,
    ColorRemove(wallpaper::Color),
    ChangeCategory(Category),
    ColorSelect(wallpaper::Color),
    Fit(usize),
    ImageAdd(Option<Arc<(PathBuf, Image, Image)>>),
    ImageAddDialog,
    ImageRemove(DefaultKey),
    Output(segmented_button::Entity),
    RotationFrequency(usize),
    SameBackground(bool),
    Select(DefaultKey),
    Slideshow(bool),
    Init(Box<(wallpaper::Config, HashMap<String, String>, Context)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Category {
    Backgrounds,
    Colors,
    RecentFolder(usize),
}

pub struct Page {
    pub active_output: Option<String>,
    pub background_service_config: wallpaper::Config,
    pub cached_display_handle: Option<ImageHandle>,
    pub categories: dropdown::multi::Model<String, Category>,
    pub config: Config,
    pub fit_options: Vec<String>,
    pub outputs: SingleSelectModel,
    pub recent_folders: Vec<(PathBuf, String)>,
    pub rotation_frequency: u64,
    pub rotation_options: Vec<String>,
    pub selected_fit: usize,
    pub selected_rotation: usize,
    pub selection: Context,
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

    fn file_chooser(&mut self, selections: Vec<url::Url>) -> Command<crate::pages::Message> {
        if let Some(selection) = selections.first() {
            let path = PathBuf::from(selection.path());

            if path.is_dir() {
                self.add_recent_folder(path);
            } else {
                if let Some(parent) = path.parent() {
                    self.add_recent_folder(parent.to_owned());
                }

                return cosmic::command::future(async move {
                    let result = wallpaper::load_image_with_thumbnail(&mut Vec::new(), path).await;

                    crate::pages::Message::DesktopWallpaper(Message::ImageAdd(result.map(Arc::new)))
                });
            }
        }

        Command::none()
    }

    fn load(&self, _page: page::Entity) -> Option<page::Task<crate::pages::Message>> {
        let current_folder = self.config.current_folder().to_owned();

        Some(Box::pin(async move {
            let (background_service_config, outputs) = wallpaper::config();

            let update = change_folder(current_folder).await;

            crate::pages::Message::DesktopWallpaper(Message::Init(Box::new((
                background_service_config,
                outputs,
                update,
            ))))
        }))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl Default for Page {
    fn default() -> Self {
        let mut page = Page {
            active_output: None,
            cached_display_handle: None,
            categories: {
                let mut categories = dropdown::multi::model();

                categories.insert(dropdown::multi::list(
                    None,
                    vec![(fl!("system-backgrounds"), Category::Backgrounds)],
                ));

                categories.insert(dropdown::multi::list(
                    None,
                    vec![(fl!("colors"), Category::Colors)],
                ));

                categories.insert(dropdown::multi::list(
                    Some(fl!("recent-folders")),
                    Vec::with_capacity(5),
                ));

                categories.selected = Some(Category::Backgrounds);

                categories
            },
            background_service_config: wallpaper::Config::default(),
            config: Config::new(),
            fit_options: vec![fl!("fit-to-screen"), fl!("stretch"), fl!("zoom")],
            outputs: SingleSelectModel::default(),
            recent_folders: Vec::new(),
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
        };

        // Sync custom colors from config.
        for color in page.config.custom_colors() {
            page.selection.add_custom_color(color.clone());
        }

        page.assign_recent_folders();

        page
    }
}

impl Page {
    fn add_recent_folder(&mut self, folder: PathBuf) {
        if let Err(why) = self.config.add_recent_folder(folder) {
            tracing::error!(?why, "cannot add recent folder to config");
        }

        self.assign_recent_folders();
    }

    fn assign_recent_folders(&mut self) {
        let recent_list = &mut self.categories.lists[2];
        recent_list.options.clear();

        for (id, folder) in self.config.recent_folders().iter().enumerate() {
            if let Some(name) = folder.file_name() {
                let name = name.to_string_lossy();
                recent_list
                    .options
                    .push((name.to_string(), Category::RecentFolder(id)));
            }
        }
    }

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
        if self.background_service_config.same_on_all {
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

        if self.background_service_config.same_on_all {
            self.background_service_config.outputs.clear();
        }

        let entry = match self.selection.active {
            Choice::Slideshow => {
                match self.config_background_entry(
                    output.clone(),
                    self.config.current_folder().to_path_buf(),
                ) {
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
            self.background_service_config.backgrounds.clear();
            self.background_service_config.outputs.clear();
        }

        wallpaper::set(&mut self.background_service_config, entry);
    }

    /// Updates configuration for background image.
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

    /// Updates configuration from the background service.
    fn background_service_config_update(
        &mut self,
        background_service_config: wallpaper::Config,
        displays: HashMap<String, String>,
        selection: Context,
    ) {
        self.background_service_config = background_service_config;
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

        if self.background_service_config.same_on_all
            || self.background_service_config.backgrounds.is_empty()
        {
            let entry = self.background_service_config.default_background.clone();
            self.select_background_entry(&entry);

            if let Some(current) = entry_directory(self.config.current_folder(), &entry) {
                if let Err(why) = self.config.set_current_folder(current) {
                    tracing::error!(?why, "cannot set current folder");
                }
            }
        } else if let Some(data) = self.outputs.active_data::<String>() {
            let mut backgrounds = Vec::new();
            std::mem::swap(
                &mut self.background_service_config.backgrounds,
                &mut backgrounds,
            );

            for background in &backgrounds {
                if &background.output == data {
                    self.active_output = Some(data.clone());
                    self.select_background_entry(background);

                    if let Some(current) = entry_directory(self.config.current_folder(), background)
                    {
                        if let Err(why) = self.config.set_current_folder(current) {
                            tracing::error!(?why, "cannot set current folder");
                        }
                    }

                    break;
                }
            }

            std::mem::swap(
                &mut self.background_service_config.backgrounds,
                &mut backgrounds,
            );
        }
    }

    /// Changes the selection category, such as wallpaper select or color select.
    fn change_category(&mut self, category: Category) -> Command<crate::app::Message> {
        let mut command = Command::none();

        match category {
            Category::Backgrounds => {
                self.select_first_background();
            }

            Category::Colors => {
                self.selection.active = Choice::Color(wallpaper::DEFAULT_COLORS[0].clone());
                self.cache_display_image();
            }

            Category::RecentFolder(id) => {
                if let Some(path) = self.config.recent_folders().get(id).cloned() {
                    if let Err(why) = self.config.set_current_folder(path.clone()) {
                        tracing::error!(?path, ?why, "failed to set current folder");
                    }

                    command = cosmic::command::future(async move {
                        crate::app::Message::PageMessage(crate::pages::Message::DesktopWallpaper(
                            Message::ChangeFolder(change_folder(path).await),
                        ))
                    });
                }
            }
        }

        self.categories.selected = Some(category);
        command
    }

    /// Changes the output being configured
    pub fn change_output(&mut self, entity: segmented_button::Entity) {
        self.outputs.activate(entity);
        if let Some(name) = self.outputs.data::<String>(entity) {
            self.active_output = Some(name.clone());
        }
    }

    /// Changes the slideshow background rotation frequency
    pub fn change_rotation_frequency(&mut self, option: usize) {
        self.selected_rotation = option;

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
            Some(ref handle) => cosmic::widget::image(handle.clone())
                .width(Length::Fixed(SIMULATED_WIDTH as f32))
                .into(),

            None => cosmic::widget::Space::new(SIMULATED_WIDTH, SIMULATED_HEIGHT).into(),
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        match message {
            Message::ChangeFolder(mut context) => {
                // Reassign custom colors and images to the new context.
                std::mem::swap(&mut context, &mut self.selection);

                for color in context.custom_colors {
                    self.selection.add_custom_color(color);
                }

                for image in context.custom_images {
                    let path = context.paths.remove(image);
                    let display = context.display_images.remove(image);
                    let selection = context.selection_handles.remove(image);

                    if let Some(((display, selection), path)) = display.zip(selection).zip(path) {
                        let key = self.selection.paths.insert(path);
                        self.selection.display_images.insert(key, display);
                        self.selection.selection_handles.insert(key, selection);
                    }
                }

                self.select_first_background();
            }
            Message::ColorAdd(color) => {
                if let Err(why) = self.config.add_custom_color(color) {
                    tracing::error!(?why, "could not set custom color");
                }
            }

            Message::ColorAddDialog => {
                unimplemented!();
            }

            Message::ColorRemove(color) => {
                self.selection.remove_custom_color(&color);

                if let Err(why) = self.config.remove_custom_color(&color) {
                    tracing::error!(?why, "could not remove custom color from config");
                }
            }

            Message::ImageAdd(result) => {
                let result = result.and_then(Arc::into_inner);

                let Some((path, display, selection)) = result else {
                    tracing::warn!("image not found for provided wallpaper");
                    return Command::none();
                };

                if let Err(why) = self.config.add_custom_image(path.clone()) {
                    tracing::error!(?path, ?why, "could add custom image to config");
                }

                self.selection.add_custom_image(path, display, selection);
            }

            Message::ImageAddDialog => {
                return cosmic::command::message(crate::Message::FileChooser(
                    crate::app::FileChooser::Open {
                        title: fl!("wallpaper-dialog-image"),
                        accept_label: fl!("wallpaper-dialog-image", "accept"),
                        include_directories: false,
                        modal: false,
                        multiple_files: false,
                    },
                ));
            }

            Message::ImageRemove(image) => {
                if let Some(path) = self.selection.remove_custom_image(image) {
                    if let Err(why) = self.config.remove_custom_image(&path) {
                        tracing::error!(?why, "could not remove custom image from config");
                    }
                }
            }

            Message::ChangeCategory(category) => {
                return self.change_category(category);
            }

            Message::ColorSelect(color) => {
                self.selection.active = Choice::Color(color);
                self.cached_display_handle = None;
            }

            Message::Fit(selection) => {
                self.selected_fit = selection;
                self.cache_display_image();
            }

            Message::Output(id) => self.change_output(id),

            Message::RotationFrequency(pos) => self.change_rotation_frequency(pos),

            Message::SameBackground(value) => {
                self.background_service_config.same_on_all = value;
                self.background_service_config.backgrounds.clear();
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

            Message::Init(update) => {
                self.background_service_config_update(update.0, update.1, update.2);
                self.config_apply();

                // Load custom content
                return cosmic::command::batch(self.config.custom_images().iter().cloned().map(
                    |path| {
                        cosmic::command::future(async move {
                            let result =
                                wallpaper::load_image_with_thumbnail(&mut Vec::new(), path).await;

                            crate::app::Message::PageMessage(
                                crate::pages::Message::DesktopWallpaper(Message::ImageAdd(
                                    result.map(Arc::new),
                                )),
                            )
                        })
                    },
                ));
            }
        }

        self.config_apply();
        Command::none()
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
                self.categories.selected = Some(Category::Colors);
                self.cache_display_image();
            }
        }
    }

    /// Selects the first background from the wallpaper select options.
    fn select_first_background(&mut self) {
        let (entity, path) = match self.selection.custom_images.last() {
            Some(entity) => (*entity, &self.selection.paths[*entity]),
            None => match self.selection.paths.iter().next() {
                Some(value) => value,
                None => return,
            },
        };

        if let Some(output) = self.config_output() {
            if let Some(entry) = self.config_background_entry(output, path.clone()) {
                self.select_background(&entry, entity, path.is_dir());
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
    custom_images: Vec<DefaultKey>,
    custom_colors: Vec<wallpaper::Color>,
    paths: SlotMap<DefaultKey, PathBuf>,
    is_custom: SecondaryMap<DefaultKey, bool>,
    display_images: SecondaryMap<DefaultKey, image::RgbaImage>,
    selection_handles: SecondaryMap<DefaultKey, ImageHandle>,
}

impl Context {
    fn add_custom_color(&mut self, color: wallpaper::Color) {
        if !self.custom_colors.contains(&color) {
            self.add_custom_color(color);
        }
    }

    fn add_custom_image(&mut self, path: PathBuf, display: Image, selection: Image) {
        let key = self.paths.insert(path);
        self.is_custom.insert(key, true);
        self.display_images.insert(key, display);
        self.custom_images.push(key);
        self.selection_handles.insert(
            key,
            ImageHandle::from_pixels(selection.width(), selection.height(), selection.into_vec()),
        );
    }

    fn remove_custom_color(&mut self, color: &wallpaper::Color) {
        if let Some(id) = self.custom_colors.iter().position(|c| c == color) {
            self.custom_colors.remove(id);
        }
    }

    fn remove_custom_image(&mut self, image: DefaultKey) -> Option<PathBuf> {
        if let Some(true) = self.is_custom.remove(image) {
            if let Some(id) = self.custom_images.iter().position(|i| i == &image) {
                self.custom_images.remove(id);
            }

            self.display_images.remove(image);
            self.selection_handles.remove(image);
            return self.paths.remove(image);
        }

        None
    }
}

pub async fn change_folder(current_folder: PathBuf) -> Context {
    let mut update = Context::default();
    let mut backgrounds = wallpaper::load_each_from_path(current_folder);

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

    update
}

#[dynamic]
static WALLPAPER_SAME: String = fl!("wallpaper", "same");

#[dynamic]
static WALLPAPER_FIT: String = fl!("wallpaper", "fit");

#[dynamic]
static WALLPAPER_SLIDE: String = fl!("wallpaper", "slide");

#[dynamic]
static WALLPAPER_CHANGE: String = fl!("wallpaper", "change");

#[allow(clippy::too_many_lines)]
pub fn settings() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![
            WALLPAPER_SAME.clone(),
            WALLPAPER_FIT.clone(),
            WALLPAPER_SLIDE.clone(),
            WALLPAPER_CHANGE.clone(),
        ])
        .view::<Page>(|_binder, page, _section| {
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

            children.push(if page.background_service_config.same_on_all {
                text(fl!("all-displays"))
                    .font(cosmic::font::FONT_SEMIBOLD)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .apply(cosmic::widget::container)
                    .width(Length::Fill)
                    .height(Length::Fixed(32.0))
                    .into()
            } else {
                segmented_selection::horizontal(&page.outputs)
                    .on_activate(Message::Output)
                    .into()
            });

            let background_fit =
                cosmic::widget::dropdown(&page.fit_options, Some(page.selected_fit), Message::Fit);

            children.push({
                let mut column = list_column()
                    .add(settings::item(
                        &*WALLPAPER_SAME,
                        toggler(
                            None,
                            page.background_service_config.same_on_all,
                            Message::SameBackground,
                        ),
                    ))
                    .add(settings::item(&*WALLPAPER_FIT, background_fit));

                if show_slideshow_toggle {
                    column = column.add(settings::item(
                        &*WALLPAPER_SLIDE,
                        toggler(None, slideshow_enabled, Message::Slideshow),
                    ));
                }

                // The rotation frequency dropdown should only be shown when the slideshow is enabled.
                if slideshow_enabled {
                    column
                        .add(settings::item(
                            &*WALLPAPER_CHANGE,
                            dropdown(
                                &page.rotation_options,
                                Some(page.selected_rotation),
                                Message::RotationFrequency,
                            ),
                        ))
                        .into()
                } else {
                    column.into()
                }
            });

            let category_selection =
                dropdown::multi::dropdown(&page.categories, Message::ChangeCategory);

            let add_button = {
                let (text, message) = if Some(Category::Colors) == page.categories.selected {
                    (fl!("add-color"), Message::ColorAddDialog)
                } else {
                    (fl!("add-image"), Message::ImageAddDialog)
                };

                button::link(text).on_press(message)
            };

            children.push(
                row::with_capacity(2)
                    .push(category_selection)
                    .push(cosmic::widget::horizontal_space(Length::Fill))
                    .push(add_button)
                    .into(),
            );

            match page.categories.selected {
                // Displays system wallpapers that are available to select from
                Some(Category::Backgrounds | Category::RecentFolder(_)) => {
                    children.push(widgets::wallpaper_select_options(
                        page,
                        if let Choice::Background(selection) = page.selection.active {
                            Some(selection)
                        } else {
                            None
                        },
                    ));
                }

                // Displays colors and gradients that are available to select from
                Some(Category::Colors) => {
                    children.push(widgets::color_select_options(
                        &page.selection,
                        if let Choice::Color(ref color) = page.selection.active {
                            Some(color)
                        } else {
                            None
                        },
                    ));
                }

                None => (),
            }

            cosmic::widget::column::with_children(children)
                .spacing(22)
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWallpaper)
        })
}

/// Sets the current wallpaper directory.
fn entry_directory(current_folder: &Path, entry: &wallpaper::Entry) -> Option<PathBuf> {
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

        wallpaper::Source::Color(_) => PathBuf::from(current_folder),
    })
}
