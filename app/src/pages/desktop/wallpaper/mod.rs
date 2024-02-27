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

use apply::Apply;
use cosmic::{command, Command};
use cosmic::{
    dialog::file_chooser,
    widget::{
        button, dropdown, list_column, row,
        segmented_button::{self, SingleSelectModel},
        settings, tab_bar, text, toggler,
    },
};
use cosmic::{
    iced::{wayland::actions::window::SctkWindowSettings, window, Color, Length},
    prelude::CollectionWidget,
};
use cosmic::{
    iced_core::Alignment,
    iced_sctk::commands::window::{close_window, get_window},
    widget::icon,
};
use cosmic::{
    iced_core::{alignment, layout},
    iced_runtime::core::image::Handle as ImageHandle,
};
use cosmic::{
    widget::{color_picker::ColorPickerUpdate, ColorPickerModel},
    Element,
};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use cosmic_settings_wallpaper::{self as wallpaper, Entry, ScalingMode};
use image::imageops::FilterType::Lanczos3;
use image::{ImageBuffer, Rgba};
use slotmap::{DefaultKey, SecondaryMap, SlotMap};

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
    selection: Context,
}

/// Messages for the wallpaper view.
#[derive(Clone, Debug)]
pub enum Message {
    /// Adds a new wallpaper folder.
    AddFolder(Arc<Result<Url, file_chooser::Error>>),
    /// Adds a new image file the system wallpaper folder.
    AddFile(Arc<Result<Url, file_chooser::Error>>),
    /// Selects an option in the category dropdown menu.
    ChangeCategory(Category),
    /// Changes the displayed images in the wallpaper view.
    ChangeFolder(Context),
    /// Creates a color dialog
    ColorAddDialog,
    /// Handles messages from the color dialog.
    ColorDialogUpdate(ColorPickerUpdate),
    /// Removes a custom color from the color view.
    ColorRemove(wallpaper::Color),
    /// Selects a color in the color view.
    ColorSelect(wallpaper::Color),
    /// Handles the drag message in the color dialog.
    DragColorDialog,
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

/// The page struct for the wallpaper view.
pub struct Page {
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

    /// The window ID of the color dialog.
    pub color_dialog: window::Id,

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

    fn reload(&mut self, _page: page::Entity) -> Command<crate::pages::Message> {
        let current_folder = self.config.current_folder().to_owned();

        command::future(async move {
            let (service_config, displays) = wallpaper::config().await;

            let selection = change_folder(current_folder).await;

            crate::pages::Message::DesktopWallpaper(Message::Init(Box::new(InitUpdate {
                service_config,
                displays,
                selection,
            })))
        })
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
            color_dialog: window::Id::unique(),
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
                .and_then(|output| {
                    let path = wallpaper::current_image(output).ok()?;
                    let id = self.wallpaper_id_from_path(&path)?;
                    Some(&self.selection.display_images[id])
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

        self.cached_display_handle = Some(ImageHandle::from_pixels(
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
                match self
                    .config_wallpaper_entry(output, self.config.current_folder().to_path_buf())
                {
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
    fn change_category(&mut self, category: Category) -> Command<crate::app::Message> {
        let mut command = Command::none();

        match category {
            Category::Wallpapers => {
                if self.config.current_folder.is_some() {
                    let _ = self.config.set_current_folder(None);
                    command = cosmic::command::future(async move {
                        let folder = change_folder(Config::default_folder().to_owned()).await;
                        Message::ChangeFolder(folder).into()
                    });
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

                    command = cosmic::command::future(async move {
                        Message::ChangeFolder(change_folder(path).await).into()
                    });
                }
            }

            Category::AddFolder => {
                return cosmic::command::future(async {
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
        command
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

        Entry::new(output, wallpaper::Source::Path(path))
            .scaling_mode(scaling_mode)
            .rotation_frequency(self.rotation_frequency)
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
    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        match message {
            Message::DragColorDialog => {
                return cosmic::iced_sctk::commands::window::start_drag_window(self.color_dialog)
            }

            Message::ColorDialogUpdate(update) => {
                let cmd = match update {
                    ColorPickerUpdate::AppliedColor
                    | ColorPickerUpdate::Cancel
                    | ColorPickerUpdate::Reset => {
                        if let Some(color) = self.color_model.get_applied_color() {
                            let color = wallpaper::Color::Single([color.r, color.g, color.b]);

                            if let Err(why) = self.config.add_custom_color(color.clone()) {
                                tracing::error!(?why, "could not set custom color");
                            }

                            self.selection.add_custom_color(color);
                        }

                        close_window(self.color_dialog)
                    }

                    ColorPickerUpdate::ActionFinished => {
                        let _res = self
                            .color_model
                            .update::<crate::app::Message>(ColorPickerUpdate::AppliedColor);
                        Command::none()
                    }

                    _ => Command::none(),
                };

                return Command::batch(vec![
                    cmd,
                    self.color_model.update::<crate::app::Message>(update),
                ]);
            }

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

            Message::ColorAddDialog => {
                return get_window(color_picker_window_settings(self.color_dialog));
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

                self.selection.add_custom_image(
                    path,
                    display,
                    ImageHandle::from_pixels(
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
                return cosmic::command::future(async {
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
                return Command::none();
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
                        if let Ok(path) = wallpaper::current_image(output) {
                            if let Some(entity) = self.wallpaper_id_from_path(&path) {
                                if let Some(entry) =
                                    self.config_wallpaper_entry(output.to_owned(), path)
                                {
                                    self.select_wallpaper(&entry, entity, false);
                                    self.config_apply();
                                    return Command::none();
                                }
                            }
                        }
                    }

                    self.select_first_wallpaper();
                }
            }

            Message::AddFolder(result) => {
                let path = match dialog_response(result) {
                    DialogResponse::Path(path) => path,
                    DialogResponse::Error(why) => {
                        tracing::error!(why, "dialog response error");
                        return Command::none();
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

                    // Load the wallpapers from the selected folder into the view.
                    return cosmic::command::future(async move {
                        let message = Message::ChangeFolder(change_folder(path).await);
                        let page_message = crate::pages::Message::DesktopWallpaper(message);
                        crate::Message::PageMessage(page_message)
                    });
                }
            }

            Message::AddFile(result) => {
                let path = match dialog_response(result) {
                    DialogResponse::Path(path) => path,
                    DialogResponse::Error(why) => {
                        tracing::error!(why, "dialog response error");
                        return Command::none();
                    }
                };

                if path.is_file() {
                    tracing::info!(?path, "opening custom image");

                    // Loads a single custom image and its thumbnail for display in the backgrounds view.
                    return cosmic::command::future(async move {
                        let result =
                            wallpaper::load_image_with_thumbnail(&mut Vec::new(), path).await;

                        let message = Message::ImageAdd(result.map(Arc::new));
                        let page_message = crate::pages::Message::DesktopWallpaper(message);
                        crate::Message::PageMessage(page_message)
                    });
                }
            }

            Message::Init(update) => {
                self.outputs.clear();
                self.wallpaper_service_config = update.service_config;
                self.selection = update.selection;

                // Sync custom colors from config.
                for color in self.config.custom_colors() {
                    self.selection.add_custom_color(color.clone());
                }

                // Set the default selection if an image was selected.
                if let Choice::Wallpaper(_) | Choice::Slideshow = self.selection.active {
                    let folder = self.config.current_folder();
                    for (id, recent) in self.config.recent_folders().iter().enumerate() {
                        if recent == folder {
                            self.categories.selected = Some(Category::RecentFolder(id));
                        }
                    }
                }

                // These will need to be loaded before applying the service config.
                let custom_images = self.config.custom_images();

                // Make note of how many images are to be loaded, with the display update for the service config.
                self.update_config = Some((custom_images.len(), update.displays));

                // Load preview images concurrently for each custom image stored in the on-disk config.
                return cosmic::command::batch(custom_images.iter().cloned().map(|path| {
                    cosmic::command::future(async move {
                        let result =
                            wallpaper::load_image_with_thumbnail(&mut Vec::new(), path).await;

                        Message::ImageAdd(result.map(Arc::new)).into()
                    })
                }));
            }
        }

        self.config_apply();
        Command::none()
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

    pub fn show_color_dialog(&self) -> Element<crate::app::Message> {
        color_picker_view(
            &self.color_model,
            Message::DragColorDialog,
            Message::ColorDialogUpdate,
        )
        .map(|m| crate::app::Message::PageMessage(crate::pages::Message::DesktopWallpaper(m)))
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

pub async fn change_folder(current_folder: PathBuf) -> Context {
    let mut update = Context::default();
    let mut wallpapers = wallpaper::load_each_from_path(current_folder);

    while let Some((path, display_image, selection_image)) = wallpapers.recv().await {
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

crate::cache_dynamic_lazy! {
    static WALLPAPER_SAME: String = fl!("wallpaper", "same");
    static WALLPAPER_FIT: String = fl!("wallpaper", "fit");
    static WALLPAPER_SLIDE: String = fl!("wallpaper", "slide");
    static WALLPAPER_CHANGE: String = fl!("wallpaper", "change");
}

#[allow(clippy::too_many_lines)]
pub fn settings() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![
            WALLPAPER_SAME.as_str().into(),
            WALLPAPER_FIT.as_str().into(),
            WALLPAPER_SLIDE.as_str().into(),
            WALLPAPER_CHANGE.as_str().into(),
        ])
        .view::<Page>(|_binder, page, _section| {
            let mut children = Vec::with_capacity(3);

            let mut show_slideshow_toggle = true;
            let mut slideshow_enabled = false;

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

            children.push(if page.wallpaper_service_config.same_on_all {
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
                tab_bar::horizontal(&page.outputs)
                    .on_activate(Message::Output)
                    .into()
            });

            let wallpaper_fit =
                cosmic::widget::dropdown(&page.fit_options, Some(page.selected_fit), Message::Fit);

            children.push({
                let mut column = list_column()
                    .add(settings::item(
                        &*WALLPAPER_SAME,
                        toggler(
                            None,
                            page.wallpaper_service_config.same_on_all,
                            Message::SameWallpaper,
                        ),
                    ))
                    .add(settings::item(&*WALLPAPER_FIT, wallpaper_fit));

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

            // Show the add button only on the Colors and Wallpapers pages
            let add_button =
                if let Some(Category::Colors | Category::Wallpapers) = page.categories.selected {
                    let (text, message) = if Some(Category::Colors) == page.categories.selected {
                        (fl!("add-color"), Message::ColorAddDialog)
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
                    .align_items(Alignment::Center)
                    // Show a folder icon if the active category is a custom folder.
                    .push_maybe(
                        if let Some(Category::RecentFolder(_)) = page.categories.selected {
                            Some(icon::from_name("folder-symbolic").size(16).icon())
                        } else {
                            None
                        },
                    )
                    .push(category_selection)
                    .push(cosmic::widget::horizontal_space(Length::Fill))
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

fn color_picker_window_settings(window_id: window::Id) -> SctkWindowSettings {
    SctkWindowSettings {
        window_id,
        app_id: Some("com.system76.CosmicSettings".to_string()),
        title: Some(fl!("color-picker")),
        parent: Some(window::Id::MAIN),
        autosize: false,
        size_limits: layout::Limits::NONE
            .min_width(300.0)
            .max_width(800.0)
            .min_height(520.0)
            .max_height(520.0),
        size: (300, 520),
        resizable: Some(8.0),
        client_decorations: true,
        transparent: true,
        ..Default::default()
    }
}

// TODO: Reuse with the appearance page
pub fn color_picker_view<Message: Clone + 'static>(
    model: &ColorPickerModel,
    on_drag: Message,
    on_message: fn(ColorPickerUpdate) -> Message,
) -> Element<Message> {
    let header = cosmic::widget::header_bar()
        .title(fl!("color-picker"))
        .on_close(on_message(ColorPickerUpdate::AppliedColor))
        .on_drag(on_drag);

    let content = cosmic::widget::container(
        model
            .builder(on_message)
            .width(Length::Fixed(254.0))
            .height(Length::Fixed(174.0))
            .reset_label(fl!("reset-to-default"))
            .build(
                fl!("recent-colors"),
                fl!("copy-to-clipboard"),
                fl!("copied-to-clipboard"),
            ),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .style(cosmic::theme::style::Container::Background);

    cosmic::widget::column::with_capacity(2)
        .push(header)
        .push(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(cosmic::iced_core::Alignment::Center)
        .apply(Element::from)
}

enum DialogResponse {
    Error(String),
    Path(PathBuf),
}

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
