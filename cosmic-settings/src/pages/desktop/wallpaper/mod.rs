// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod config;
pub mod widgets;

pub use config::Config;
use url::Url;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

#[cfg(feature = "xdg-portal")]
use cosmic::dialog::file_chooser;
use cosmic::{
    Apply, Element, Task,
    widget::{ColorPickerModel, color_picker::ColorPickerUpdate, icon},
};
use cosmic::{app::ContextDrawer, iced_runtime::core::image::Handle as ImageHandle};
use cosmic::{
    iced::Subscription,
    widget::{
        button, dropdown, list_column, row,
        segmented_button::{self, SingleSelectModel},
        settings, tab_bar, text, toggler,
    },
};
use cosmic::{
    iced::{Alignment, Color, Length, window},
    surface,
};
use cosmic_bg_config::Source;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use cosmic_settings_wallpaper::{self as wallpaper, Entry, ScalingMode};
use image::imageops::FilterType::Lanczos3;
use image::{ImageBuffer, Rgba};
use slab::Slab;
use slotmap::{DefaultKey, Key, SecondaryMap, SlotMap};

use crate::subscription::WallpaperEvent;

const ZOOM: usize = 0;
const FIT: usize = 1;

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
struct OutputName(String);

#[derive(Clone, Debug)]
pub struct InitUpdate {
    service_config: wallpaper::Config,
    displays: HashMap<String, (String, (u32, u32))>,
}

/// Messages for the wallpaper view.
#[derive(Clone, Debug)]
pub enum Message {
    /// Adds a new wallpaper folder.
    #[cfg(feature = "xdg-portal")]
    AddFolder(Arc<Result<Url, file_chooser::Error>>),
    /// Adds a new image file the system wallpaper folder.
    #[cfg(feature = "xdg-portal")]
    AddFile(Arc<Result<Url, file_chooser::Error>>),
    /// Cache currently displayed image.
    CacheDisplayImage,
    /// Selects an option in the category dropdown menu.
    ChangeCategory(Category),
    /// Changes the displayed images in the wallpaper view.
    ChangeFolder(Context),
    /// Emits a wallpaper event.
    Event(WallpaperEvent),
    /// Handles messages from the color dialog.
    ColorAdd(ColorPickerUpdate),
    /// Creates a color context drawer
    ColorAddContext,
    /// Removes a custom color from the color view.
    ColorRemove(wallpaper::Color),
    /// Selects a color in the color view.
    ColorSelect(wallpaper::Color),
    /// Sets the wallpaper fit parameter.
    Fit(usize),
    /// Adds a new custom image to the wallpaper view.
    ImageAdd(Option<Arc<(PathBuf, Image, Image)>>),
    /// Creates an image dialog.
    ImageAddDialog,
    /// Removes a custom image from the wallpaper view.
    ImageRemove(DefaultKey),
    /// Initializes the view.
    Init(Box<InitUpdate>),
    /// Changes the active output display that is to be configured.
    Output(segmented_button::Entity),
    /// Changes the rotation frequency of wallpaper images in slideshow mode.
    RotationFrequency(usize),
    /// If set, all outputs will use the same wallpaper.
    SameWallpaper(bool),
    /// Selects an background option from the list of selections in the view.
    Select(DefaultKey),
    /// Changes the slideshow parameter.
    Slideshow(bool),
    /// State change from cosmic-bg
    UpdateState(cosmic_bg_config::state::State),
    Surface(surface::Action),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        let page_message = crate::pages::Message::DesktopWallpaper(message);
        crate::app::Message::PageMessage(page_message)
    }
}

/// Messages defined for the category dropdown menu.
#[derive(Clone, Debug, PartialEq)]
pub enum Category {
    /// Opens a dialog for adding a folder
    AddFolder,
    /// Changes the view to the color view.
    Colors,
    /// Changes the view to an added folder.
    RecentFolder(usize),
    /// Changes the view to the system wallpaper view.
    Wallpapers,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ContextView {
    AddColor,
}

/// The page struct for the wallpaper view.
pub struct Page {
    /// Internal ID of this page in the application.
    entity: page::Entity,

    /// Abort handle to the on_enter task.
    on_enter_handle: Option<cosmic::iced::task::Handle>,

    /// Whether to show a context drawer.
    context_view: Option<ContextView>,

    /// Whether to show the tab_bar or not.
    show_tab_bar: bool,

    /// The display that is currently being configured.
    ///
    /// If set to `None`, all displays will have the same wallpaper.
    active_output: Option<String>,

    /// Configuration parameters used by the cosmic-bg service.
    wallpaper_service_config: wallpaper::Config,

    /// Cache for storing the image used by the display preview.
    cached_display_handle: Option<ImageHandle>,

    /// Model for the category dropdown, which has categories and recent folders.
    categories: dropdown::multi::Model<String, Category>,

    /// The color model updated by the color dialog.
    color_model: ColorPickerModel,

    /// Settings for this page, stored by cosmic-config.
    config: Config,

    /// Model containing available wallpaper fit options.
    fit_options: Vec<String>,

    /// Model for selecting between display outputs.
    outputs: SingleSelectModel,

    /// Current value of the slideshow rotation frequency.
    rotation_frequency: u64,

    /// Model for available options for rotation frequencies.
    rotation_options: Vec<String>,

    /// The ID of the currently-selected wallpaper fit.
    selected_fit: usize,

    /// The ID of the currently-selected slideshow rotation.
    selected_rotation: usize,

    /// Stores custom colors, custom images, and all image data for every wallpaper.
    selection: Context,

    /// When set, applys a config update after images are loaded.
    update_config: Option<(usize, HashMap<String, (String, (u32, u32))>)>,
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

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

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        // Check if the page is already being loaded.
        if self.on_enter_handle.is_some() {
            return Task::none();
        }

        let (task, on_enter_handle) = Task::future(async move {
            let (service_config, displays) = wallpaper::config().await;

            crate::pages::Message::DesktopWallpaper(Message::Init(Box::new(InitUpdate {
                service_config,
                displays,
            })))
        })
        .abortable();

        self.on_enter_handle = Some(on_enter_handle);
        task
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        // Reclaim memory
        self.cached_display_handle = None;
        self.selection = Context::default();
        self.outputs = SingleSelectModel::default();

        // Cancel the on_enter task if it was running.
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }

        Task::none()
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        self.context_view.map(|view| {
            cosmic::app::context_drawer(
                match view {
                    ContextView::AddColor => crate::widget::color_picker_context_view(
                        None,
                        fl!("reset-to-default").into(),
                        Message::ColorAdd,
                        &self.color_model,
                    )
                    .map(crate::pages::Message::DesktopWallpaper),
                },
                crate::pages::Message::CloseContextDrawer,
            )
            .title(fl!("color-picker"))
        })
    }

    /// Watch for state changes from the cosmic-bg session service.
    fn subscription(
        &self,
        core: &cosmic::Core,
    ) -> cosmic::iced::Subscription<crate::pages::Message> {
        let subscriptions = vec![
            core.watch_state::<cosmic_bg_config::state::State>(cosmic_bg_config::NAME)
                .map(|update| {
                    crate::pages::Message::DesktopWallpaper(Message::UpdateState(update.config))
                }),
            crate::subscription::wallpapers(self.config.current_folder())
                .map(|event| crate::pages::Message::DesktopWallpaper(Message::Event(event))),
        ];
        Subscription::batch(subscriptions)
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl Default for Page {
    fn default() -> Self {
        let mut page = Page {
            entity: page::Entity::null(),
            on_enter_handle: None,
            context_view: None,
            show_tab_bar: false,
            active_output: None,
            cached_display_handle: None,
            categories: {
                let mut categories = dropdown::multi::model();

                categories.insert(dropdown::multi::list(
                    None,
                    vec![(fl!("wallpaper", "plural"), Category::Wallpapers)],
                ));

                categories.insert(dropdown::multi::list(
                    None,
                    vec![(fl!("colors"), Category::Colors)],
                ));

                categories.insert(dropdown::multi::list(
                    None,
                    vec![(fl!("open-new-folder"), Category::AddFolder)],
                ));

                categories.insert(dropdown::multi::list(
                    Some(fl!("recent-folders")),
                    Vec::with_capacity(5),
                ));

                categories.selected = Some(Category::Wallpapers);

                categories
            },
            wallpaper_service_config: wallpaper::Config::default(),
            color_model: ColorPickerModel::new(fl!("hex"), fl!("rgb"), None, Some(Color::WHITE)),
            config: Config::new(),
            fit_options: vec![fl!("fill"), fl!("fit-to-screen")],
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
            update_config: None,
        };

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
        let recent_list = &mut self.categories.lists[3];
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
        self.cached_display_handle = None;

        let choice = match self.selection.active {
            Choice::Wallpaper(id) => self.selection.display_images.get(id),

            Choice::Slideshow => self
                .config_output()
                .and_then(|output| match self.config.current_image(output)? {
                    Source::Path(path) => {
                        let id = self.wallpaper_id_from_path(&path)?;
                        Some(&self.selection.display_images[id])
                    }

                    Source::Color(_color) => None,
                })
                .or(self.selection.display_images.values().next()),

            Choice::Color(_) => None,
        };

        let Some(image) = choice else {
            return;
        };

        let temp_image;

        let image = match self.selected_fit {
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

            FIT => image,

            _ => return,
        };

        self.cached_display_handle = Some(ImageHandle::from_rgba(
            image.width(),
            image.height(),
            image.to_vec(),
        ));
    }

    fn config_output(&self) -> Option<&str> {
        if self.wallpaper_service_config.same_on_all {
            Some("all")
        } else {
            self.outputs
                .active_data::<OutputName>()
                .map(|name| name.0.as_str())
        }
    }

    /// Applies the current settings to cosmic-bg.
    pub fn config_apply(&mut self) {
        let Some(output) = self.config_output().map(String::from) else {
            return;
        };

        if self.wallpaper_service_config.same_on_all {
            self.wallpaper_service_config.backgrounds.clear();
            self.wallpaper_service_config.outputs.clear();
        } else if let Some(pos) = self
            .wallpaper_service_config
            .backgrounds
            .iter()
            .position(|entry| entry.output == output)
        {
            let _removed = self.wallpaper_service_config.backgrounds.swap_remove(pos);
        }

        let entry = match self.selection.active {
            Choice::Slideshow => {
                match self.config_wallpaper_entry(output, self.config.current_folder()) {
                    Some(entry) => entry,
                    None => return,
                }
            }

            Choice::Wallpaper(key) => {
                if let Some(path) = self.selection.paths.get(key) {
                    match self.config_wallpaper_entry(output, path.clone()) {
                        Some(entry) => entry,
                        None => return,
                    }
                } else {
                    return;
                }
            }

            Choice::Color(ref color) => Entry::new(output, wallpaper::Source::Color(color.clone())),
        };

        wallpaper::set(&mut self.wallpaper_service_config, entry);
    }

    /// Locate the ID of a wallpaper that's already stored in memory
    fn wallpaper_id_from_path(&self, path: &Path) -> Option<DefaultKey> {
        self.selection
            .paths
            .iter()
            .find(|(_id, wallpaper)| *wallpaper == path)
            .map(|(id, _)| id)
    }

    /// Updates configuration from the wallpaper service.
    fn wallpaper_service_config_update(&mut self, displays: HashMap<String, (String, (u32, u32))>) {
        let mut first = None;
        for (name, (_model, physical)) in displays {
            let is_internal = "eDP-1" == name;

            let entity = self
                .outputs
                .insert()
                .text(crate::utils::display_name(&name, physical))
                .data(OutputName(name));

            if is_internal || first.is_none() {
                first = Some(entity.id());
            }
        }

        if let Some(id) = first {
            self.outputs.activate(id);
        }

        self.apply_active_selection();
    }

    /// Apply the selection for the active output.
    fn apply_active_selection(&mut self) {
        if self.wallpaper_service_config.same_on_all
            || self.wallpaper_service_config.backgrounds.is_empty()
        {
            let entry = self.wallpaper_service_config.default_background.clone();
            self.select_wallpaper_entry(&entry);
        } else if let Some(OutputName(output)) = self.outputs.active_data() {
            let mut wallpapers = Vec::new();

            std::mem::swap(
                &mut self.wallpaper_service_config.backgrounds,
                &mut wallpapers,
            );

            for wallpaper in &wallpapers {
                if wallpaper.output == *output {
                    self.active_output = Some(output.clone());
                    self.select_wallpaper_entry(wallpaper);

                    break;
                }
            }

            std::mem::swap(
                &mut self.wallpaper_service_config.backgrounds,
                &mut wallpapers,
            );
        }
    }

    /// Changes the selection category, such as wallpaper select or color select.
    fn change_category(&mut self, category: Category) -> Task<crate::app::Message> {
        match category {
            Category::Wallpapers => {
                if self.config.current_folder.is_some() {
                    let _ = self.config.set_current_folder(None);
                } else {
                    self.select_first_wallpaper();
                }
            }

            Category::Colors => {
                self.selection.active = Choice::Color(wallpaper::DEFAULT_COLORS[0].clone());
                self.cache_display_image();
            }

            Category::RecentFolder(id) => {
                if let Some(path) = self.config.recent_folders().get(id).cloned() {
                    if let Err(why) = self.config.set_current_folder(Some(path.clone())) {
                        tracing::error!(?path, ?why, "failed to set current folder");
                    }
                }
            }

            Category::AddFolder => {
                #[cfg(feature = "xdg-portal")]
                return cosmic::task::future(async {
                    let dialog_result = file_chooser::open::Dialog::new()
                        .title(fl!("wallpaper", "folder-dialog"))
                        .accept_label(fl!("dialog-add"))
                        .modal(false)
                        .open_folder()
                        .await
                        .map(|response| response.url().to_owned());

                    let message = Message::AddFolder(Arc::new(dialog_result));
                    let page_message = crate::pages::Message::DesktopWallpaper(message);
                    crate::Message::PageMessage(page_message)
                });
            }
        }

        self.categories.selected = Some(category);
        Task::none()
    }

    /// Changes the output being configured
    pub fn change_output(&mut self, entity: segmented_button::Entity) {
        self.outputs.activate(entity);

        if let Some(name) = self.outputs.data::<OutputName>(entity) {
            self.active_output = Some(name.0.clone());
        }

        self.apply_active_selection();
        self.cache_display_image();
    }

    /// Changes the slideshow wallpaper rotation frequency
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

    /// Updates configuration for wallpaper image.
    fn config_wallpaper_entry(&self, output: String, path: PathBuf) -> Option<Entry> {
        let scaling_mode = match self.selected_fit {
            ZOOM => ScalingMode::Zoom,
            FIT => ScalingMode::Fit([0.0, 0.0, 0.0]),
            _ => return None,
        };
        let old_entry = if output == "all" {
            Some(&self.wallpaper_service_config.default_background)
        } else {
            self.wallpaper_service_config
                .backgrounds
                .iter()
                .find(|entry| entry.output == output)
        };

        let entry = Entry::new(output, wallpaper::Source::Path(path))
            .scaling_mode(scaling_mode)
            .rotation_frequency(self.rotation_frequency);

        if let Some(old_entry) = old_entry {
            entry
                .sampling_method(old_entry.sampling_method)
                .filter_method(old_entry.filter_method.clone())
                .filter_by_theme(old_entry.filter_by_theme)
        } else {
            entry
        }
        .apply(Some)
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
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::UpdateState(_state) => {
                if let Choice::Slideshow = self.selection.active {
                    self.cache_display_image();
                }
            }

            Message::CacheDisplayImage => self.cache_display_image(),

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
                        self.selection.add_custom_image(path, display, selection);
                    }
                }

                self.select_first_wallpaper();
            }

            Message::ColorAdd(message) => {
                match message {
                    ColorPickerUpdate::ActionFinished => {
                        if let Some(color) = self.color_model.get_applied_color() {
                            let ret = self
                                .color_model
                                .update::<crate::app::Message>(ColorPickerUpdate::ActionFinished);
                            let color = wallpaper::Color::Single([color.r, color.g, color.b]);

                            if let Choice::Color(c) = self.selection.active.clone() {
                                if let Err(why) = self.config.remove_custom_color(&c) {
                                    tracing::error!(?why, "could not set custom color");
                                }
                                self.selection.remove_custom_color(&c);
                            }
                            if let Err(why) = self.config.add_custom_color(color.clone()) {
                                tracing::error!(?why, "could not set custom color");
                            }

                            self.cached_display_handle = None;
                            self.selection.replace_active_custom(color.clone());
                            self.config_apply();

                            return ret;
                        }
                    }
                    m => {
                        return self.color_model.update::<crate::app::Message>(m);
                    }
                };
            }

            Message::ColorAddContext => {
                self.context_view = Some(ContextView::AddColor);
                self.selection.active = Choice::Color(wallpaper::Color::Single([0., 0., 0.]));
                return cosmic::task::message(crate::app::Message::OpenContextDrawer(self.entity));
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
                    return Task::none();
                };

                if let Err(why) = self.config.add_custom_image(path.clone()) {
                    tracing::error!(?path, ?why, "could add custom image to config");
                }

                self.selection.add_custom_image(
                    path,
                    display,
                    ImageHandle::from_rgba(
                        selection.width(),
                        selection.height(),
                        selection.into_vec(),
                    ),
                );

                // If an update was queued, apply it after all custom images have been added.
                if let Some((mut remaining, displays)) = self.update_config.take() {
                    remaining -= 1;
                    if remaining == 0 {
                        self.wallpaper_service_config_update(displays);
                        self.config_apply();
                    } else {
                        self.update_config = Some((remaining, displays));
                    }
                }
            }

            Message::ImageAddDialog => {
                #[cfg(feature = "xdg-portal")]
                return cosmic::Task::future(async {
                    let dialog_result = file_chooser::open::Dialog::new()
                        .title(fl!("wallpaper", "image-dialog"))
                        .accept_label(fl!("dialog-add"))
                        .modal(false)
                        .open_file()
                        .await
                        .map(|response| response.url().to_owned());

                    let message = Message::AddFile(Arc::new(dialog_result));
                    let page_message = crate::pages::Message::DesktopWallpaper(message);
                    crate::Message::PageMessage(page_message)
                });
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

            Message::Output(id) => {
                self.change_output(id);
                return Task::none();
            }

            Message::RotationFrequency(pos) => self.change_rotation_frequency(pos),

            Message::SameWallpaper(value) => {
                self.wallpaper_service_config.same_on_all = value;
                self.wallpaper_service_config.backgrounds.clear();
            }

            Message::Select(id) => {
                self.selection.active = Choice::Wallpaper(id);
                self.cache_display_image();
            }

            Message::Slideshow(enable) => {
                if enable {
                    self.selection.active = Choice::Slideshow;
                    self.cache_display_image();
                } else {
                    if let Some(output) = self.config_output() {
                        if let Some(Source::Path(path)) = self.config.current_image(output) {
                            if let Some(entity) = self.wallpaper_id_from_path(&path) {
                                if let Some(entry) =
                                    self.config_wallpaper_entry(output.to_owned(), path)
                                {
                                    self.select_wallpaper(&entry, entity, false);
                                    self.config_apply();
                                    return Task::none();
                                }
                            }
                        }
                    }

                    self.select_first_wallpaper();
                }
            }

            #[cfg(feature = "xdg-portal")]
            Message::AddFolder(result) => {
                let path = match dialog_response(result) {
                    DialogResponse::Path(path) => path,
                    DialogResponse::Error(why) => {
                        tracing::error!(why, "dialog response error");
                        return Task::none();
                    }
                };

                if path.is_dir() {
                    tracing::info!(?path, "opening new folder");

                    let _res = self.config.set_current_folder(Some(path.clone()));

                    // Add the selected folder to the recent folders list.
                    self.add_recent_folder(path.clone());

                    // Select that folder in the recent folders list.
                    for (id, recent) in self.config.recent_folders().iter().enumerate() {
                        if &path == recent {
                            self.categories.selected = Some(Category::RecentFolder(id));
                        }
                    }
                }
            }

            #[cfg(feature = "xdg-portal")]
            Message::AddFile(result) => {
                let path = match dialog_response(result) {
                    DialogResponse::Path(path) => path,
                    DialogResponse::Error(why) => {
                        tracing::error!(why, "dialog response error");
                        return Task::none();
                    }
                };

                if path.is_file() {
                    tracing::info!(?path, "opening custom image");

                    // Loads a single custom image and its thumbnail for display in the backgrounds view.
                    return cosmic::Task::future(async move {
                        let result = wallpaper::load_image_with_thumbnail(path);

                        let message = Message::ImageAdd(result.map(Arc::new));
                        let page_message = crate::pages::Message::DesktopWallpaper(message);
                        crate::Message::PageMessage(page_message)
                    });
                }
            }

            Message::Event(event) => match event {
                WallpaperEvent::Loading => {
                    self.selection = Context::default();
                }
                WallpaperEvent::Load {
                    path,
                    display,
                    selection,
                } => {
                    let key = self.selection.paths.insert(path);
                    self.selection.display_images.insert(key, display);
                    self.selection.selection_handles.insert(
                        key,
                        ImageHandle::from_rgba(
                            selection.width(),
                            selection.height(),
                            selection.into_vec(),
                        ),
                    );

                    // `selection.active` is usually empty because `change_folder` creates a fresh context.
                    // This leads to blank previews in certain conditions when the program is restarted.
                    let fix_active = match self.selection.active {
                        Choice::Wallpaper(key) if !self.selection.paths.contains_key(key) => true,
                        Choice::Color(ref color)
                            if !self.selection.custom_colors.contains(color) =>
                        {
                            true
                        }
                        _ => false,
                    };
                    if fix_active {
                        self.selection.active =
                            match self.wallpaper_service_config.default_background.source {
                                Source::Path(ref path) if !path.is_dir() => self
                                    .selection
                                    .paths
                                    .iter()
                                    .find(|(_key, valid_path)| path == valid_path.as_path())
                                    .map(|(key, _)| Choice::Wallpaper(key))
                                    .unwrap_or_default(),
                                Source::Path(_) => Choice::Slideshow,
                                Source::Color(ref color) => {
                                    self.selection.add_custom_color(color.clone());
                                    Choice::Color(color.clone())
                                }
                            }
                    }
                }
                WallpaperEvent::Loaded => self.cache_display_image(),
                WallpaperEvent::Error(error) => {
                    tracing::error!("Failed to load wallpaper: {}", error);
                }
            },

            Message::Init(update) => {
                self.outputs.clear();
                self.wallpaper_service_config = update.service_config;
                self.show_tab_bar = update.displays.len() > 1;

                // Sync custom colors from config.
                for color in self.config.custom_colors() {
                    self.selection.add_custom_color(color.clone());
                }

                // Set the default selection if an image was selected.
                if let Choice::Wallpaper(_) | Choice::Slideshow = self.selection.active {
                    let folder = self.config.current_folder();
                    for (id, recent) in self.config.recent_folders().iter().enumerate() {
                        if &folder == recent {
                            self.categories.selected = Some(Category::RecentFolder(id));
                        }
                    }
                }

                // These will need to be loaded before applying the service config.
                let custom_images = self.config.custom_images().len();

                if custom_images == 0 {
                    self.wallpaper_service_config_update(update.displays);
                    self.config_apply();
                } else {
                    // Make note of how many images are to be loaded, with the display update for the service config.
                    self.update_config = Some((custom_images, update.displays));
                }

                // Load preview images concurrently for each custom image stored in the on-disk config.
                return cosmic::task::batch(
                    self.config
                        .custom_images()
                        .iter()
                        .cloned()
                        .map(|path| {
                            cosmic::task::future(async move {
                                let result = wallpaper::load_image_with_thumbnail(path);

                                Message::ImageAdd(result.map(Arc::new))
                            })
                        })
                        // Cache wallpaper preview early to prevent blank previews on reload
                        .chain(std::iter::once(cosmic::task::message::<
                            _,
                            crate::app::Message,
                        >(
                            Message::CacheDisplayImage
                        ))),
                );
            }
            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }
        }

        self.config_apply();
        Task::none()
    }

    /// Selects the given wallpaper entry.
    fn select_wallpaper_entry(&mut self, entry: &wallpaper::Entry) {
        match entry.source {
            wallpaper::Source::Path(ref path) => {
                if path.is_dir() {
                    self.selection.active = Choice::Slideshow;
                    self.cache_display_image();
                } else if let Some(entity) = self.wallpaper_id_from_path(path) {
                    self.select_wallpaper(entry, entity, path.is_dir());
                }
            }

            wallpaper::Source::Color(ref color) => {
                self.selection.active = Choice::Color(color.clone());
                self.categories.selected = Some(Category::Colors);
                self.cache_display_image();
            }
        }
    }

    /// Selects the first wallpaper from the wallpaper select options.
    fn select_first_wallpaper(&mut self) {
        let (entity, path) = if let Some(Category::Wallpapers) = self.categories.selected {
            match self.selection.custom_images.last() {
                Some(entity) => (*entity, &self.selection.paths[*entity]),
                None => match self.selection.paths.iter().next() {
                    Some(value) => value,
                    None => return,
                },
            }
        } else {
            match self.selection.paths.iter().next() {
                Some(value) => value,
                None => return,
            }
        };

        if let Some(output) = self.config_output() {
            let is_slideshow = path.is_dir();
            if let Some(entry) = self.config_wallpaper_entry(output.to_owned(), path.clone()) {
                self.select_wallpaper(&entry, entity, is_slideshow);
            }
        }
    }

    /// Selects the given wallpaper
    fn select_wallpaper(
        &mut self,
        entry: &wallpaper::Entry,
        entity: DefaultKey,
        is_slideshow: bool,
    ) {
        self.selection.active = if is_slideshow {
            Choice::Slideshow
        } else {
            Choice::Wallpaper(entity)
        };

        match entry.scaling_mode {
            ScalingMode::Zoom | ScalingMode::Stretch => self.selected_fit = ZOOM,
            ScalingMode::Fit(_) => self.selected_fit = FIT,
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
}

#[derive(Clone, Debug, PartialEq)]
enum Choice {
    Wallpaper(DefaultKey),
    Color(wallpaper::Color),
    Slideshow,
}

impl Default for Choice {
    fn default() -> Self {
        Self::Wallpaper(DefaultKey::default())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Context {
    active: Choice,
    custom_images: Vec<DefaultKey>,
    custom_colors: Vec<wallpaper::Color>,
    paths: SlotMap<DefaultKey, PathBuf>,
    is_custom: SecondaryMap<DefaultKey, ()>,
    display_images: SecondaryMap<DefaultKey, image::RgbaImage>,
    selection_handles: SecondaryMap<DefaultKey, ImageHandle>,
}

impl Context {
    fn add_custom_color(&mut self, color: wallpaper::Color) {
        if !self.custom_colors.contains(&color) {
            self.custom_colors.push(color);
        }
    }

    fn replace_active_custom(&mut self, color: wallpaper::Color) {
        if let Choice::Color(active) = &self.active {
            self.custom_colors.retain(|c| c != active);
        }
        if !self.custom_colors.contains(&color) {
            self.custom_colors.push(color.clone());
            self.active = Choice::Color(color);
        }
    }

    fn add_custom_image(&mut self, path: PathBuf, display: Image, selection: ImageHandle) {
        let key = self.paths.insert(path);
        self.is_custom.insert(key, ());
        self.display_images.insert(key, display);
        self.custom_images.push(key);
        self.selection_handles.insert(key, selection);
    }

    fn remove_custom_color(&mut self, color: &wallpaper::Color) {
        if let Some(id) = self.custom_colors.iter().position(|c| c == color) {
            self.custom_colors.remove(id);
        }
    }

    fn remove_custom_image(&mut self, image: DefaultKey) -> Option<PathBuf> {
        if self.is_custom.contains_key(image) {
            if let Some(id) = self.custom_images.iter().position(|i| i == &image) {
                self.custom_images.remove(id);
            }

            self.display_images.remove(image);
            self.selection_handles.remove(image);
            self.is_custom.remove(image);
            return self.paths.remove(image);
        }

        None
    }
}

#[allow(clippy::too_many_lines)]
pub fn settings() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let same_label = descriptions.insert(fl!("wallpaper", "same"));
    let fit_label = descriptions.insert(fl!("wallpaper", "fit"));
    let slide_label = descriptions.insert(fl!("wallpaper", "slide"));
    let change_label = descriptions.insert(fl!("wallpaper", "change"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let mut children = Vec::with_capacity(3);

            let mut show_slideshow_toggle = true;
            // Slideshow is enabled if the background path from cosmic-bg is a directory
            let mut slideshow_enabled = page
                .config_output()
                .and_then(|output| page.wallpaper_service_config.entry(output))
                .map_or(false, |entry| {
                    if let Source::Path(path) = &entry.source {
                        path.is_dir()
                    } else {
                        false
                    }
                });

            children.push(crate::widget::display_container(
                match page.selection.active {
                    // Shows wallpaper options, with the slideshow toggle enabled
                    Choice::Slideshow => {
                        slideshow_enabled = true;
                        page.display_image_view()
                    }

                    // Shows wallpaper options, with the slideshow toggle visible
                    Choice::Wallpaper(_) => page.display_image_view(),

                    // Displays color options, and hides the slideshow toggle
                    Choice::Color(ref color) => {
                        show_slideshow_toggle = false;
                        widgets::color_image(
                            color.clone(),
                            SIMULATED_WIDTH,
                            SIMULATED_HEIGHT,
                            Some(0.0),
                        )
                    }
                },
            ));

            if page.wallpaper_service_config.same_on_all {
                let element = text::heading(fl!("all-displays"))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .apply(cosmic::widget::container)
                    .width(Length::Fill)
                    .height(Length::Fixed(32.0));

                children.push(element.into());
            } else if page.show_tab_bar {
                let element = tab_bar::horizontal(&page.outputs)
                    .button_alignment(Alignment::Center)
                    .on_activate(Message::Output);

                children.push(element.into());
            }

            let wallpaper_fit = cosmic::widget::dropdown::popup_dropdown(
                &page.fit_options,
                Some(page.selected_fit),
                Message::Fit,
                window::Id::RESERVED,
                Message::Surface,
                |a| crate::app::Message::PageMessage(crate::pages::Message::DesktopWallpaper(a)),
            );

            children.push({
                let mut column = list_column()
                    .add(settings::item(
                        &descriptions[same_label],
                        toggler(page.wallpaper_service_config.same_on_all)
                            .on_toggle(Message::SameWallpaper),
                    ))
                    .add(settings::item(&descriptions[fit_label], wallpaper_fit));

                if show_slideshow_toggle {
                    column = column.add(settings::item(
                        &descriptions[slide_label],
                        toggler(slideshow_enabled).on_toggle(Message::Slideshow),
                    ));
                }

                // The rotation frequency dropdown should only be shown when the slideshow is enabled.
                if slideshow_enabled {
                    column
                        .add(settings::item(&descriptions[change_label], {
                            let dropdown = dropdown(
                                &page.rotation_options,
                                Some(page.selected_rotation),
                                Message::RotationFrequency,
                            );
                            #[cfg(feature = "wayland")]
                            let dropdown = {
                                dropdown.with_popup(
                                    cosmic::iced::window::Id::RESERVED,
                                    Message::Surface,
                                    |a| {
                                        crate::app::Message::PageMessage(
                                            crate::pages::Message::DesktopWallpaper(a),
                                        )
                                    },
                                )
                            };
                            dropdown
                        }))
                        .into()
                } else {
                    column.into()
                }
            });

            let category_selection =
                dropdown::multi::dropdown(&page.categories, Message::ChangeCategory);

            // Show the add button only on the Colors and Wallpapers pages
            let add_button =
                if let Some(Category::Colors | Category::Wallpapers) = page.categories.selected {
                    let (text, message) = if Some(Category::Colors) == page.categories.selected {
                        (fl!("add-color"), Message::ColorAddContext)
                    } else {
                        (fl!("add-image"), Message::ImageAddDialog)
                    };

                    let button = button::link(text).trailing_icon(true).on_press(message);

                    Some(button)
                } else {
                    None
                };

            children.push(
                row::with_capacity(2)
                    .align_y(Alignment::Center)
                    // Show a folder icon if the active category is a custom folder.
                    .push_maybe(
                        if let Some(Category::RecentFolder(_)) = page.categories.selected {
                            Some(icon::from_name("folder-symbolic").size(16).icon())
                        } else {
                            None
                        },
                    )
                    .push(category_selection)
                    .push(cosmic::widget::horizontal_space())
                    .push_maybe(add_button)
                    .into(),
            );

            match page.categories.selected {
                // Displays system wallpapers that are available to select from
                Some(Category::Wallpapers | Category::RecentFolder(_)) => {
                    children.push(widgets::wallpaper_select_options(
                        page,
                        if let Choice::Wallpaper(selection) = page.selection.active {
                            Some(selection)
                        } else {
                            None
                        },
                        matches!(page.categories.selected, Some(Category::Wallpapers)),
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

                _ => (),
            }

            cosmic::widget::column::with_children(children)
                .spacing(22)
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWallpaper)
        })
}

// // TODO: Reuse with the appearance page
// pub fn color_picker_view<Message: Clone + 'static>(
//     model: &ColorPickerModel,
//     on_drag: Message,
//     on_message: fn(ColorPickerUpdate) -> Message,
// ) -> Element<Message> {
//     let header = cosmic::widget::header_bar()
//         .title(fl!("color-picker"))
//         .on_close(on_message(ColorPickerUpdate::AppliedColor))
//         .on_drag(on_drag);

//     let content = cosmic::widget::container(
//         model
//             .builder(on_message)
//             .width(Length::Fixed(254.0))
//             .height(Length::Fixed(174.0))
//             .reset_label(fl!("reset-to-default"))
//             .build(
//                 fl!("recent-colors"),
//                 fl!("copy-to-clipboard"),
//                 fl!("copied-to-clipboard"),
//             ),
//     )
//     .width(Length::Fill)
//     .height(Length::Fill)
//     .center_x()
//     .class(cosmic::theme::style::Container::Background);

//     cosmic::widget::column::with_capacity(2)
//         .push(header)
//         .push(content)
//         .width(Length::Fill)
//         .height(Length::Fill)
//         .align_y(Alignment::Center)
//         .apply(Element::from)
// }

enum DialogResponse {
    Error(String),
    Path(PathBuf),
}

#[cfg(feature = "xdg-portal")]
fn dialog_response(result: Arc<Result<Url, file_chooser::Error>>) -> DialogResponse {
    let Some(result) = Arc::into_inner(result) else {
        return DialogResponse::Error(String::from("Arc::into_inner returned None"));
    };

    let selection = match result {
        Ok(response) => response,
        Err(why) => {
            let mut source: &dyn std::error::Error = &why;
            let mut string = format!("open dialog subscription errored\n    cause: {source}");

            while let Some(new_source) = source.source() {
                string.push_str(&format!("\n    cause: {new_source}"));
                source = new_source;
            }

            return DialogResponse::Error(string);
        }
    };

    let Ok(path) = selection.to_file_path() else {
        return DialogResponse::Error(format!("not a valid file path: {}", selection.path()));
    };

    DialogResponse::Path(path)
}
