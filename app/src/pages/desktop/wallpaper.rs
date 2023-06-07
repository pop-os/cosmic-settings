// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::HashMap, path::PathBuf, time::Instant};

use apply::Apply;
use cosmic::{
    iced::alignment::Horizontal,
    iced::widget::{column, row},
    iced::Length,
    iced_runtime::core::image::Handle as ImageHandle,
    widget::{
        list_column,
        segmented_button::{self, SingleSelectModel},
        settings, toggler,
    },
    Element,
};
use cosmic_settings_desktop::wallpaper::{self, Entry, Output, ScalingMode};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::{DefaultKey, SecondaryMap, SlotMap};

#[derive(Clone, Debug)]
pub enum Message {
    Fit(String),
    Output(segmented_button::Entity),
    RotationFrequency(String),
    SameBackground(bool),
    Select(DefaultKey),
    Slideshow(bool),
    Update((wallpaper::Config, HashMap<String, String>, Context)),
}

pub struct Page {
    pub active_output: Option<String>,
    pub config: wallpaper::Config,
    pub current_directory: PathBuf,
    pub fit_options: Vec<String>,
    pub outputs: SingleSelectModel,
    pub rotation_frequency: u64,
    pub rotation_options: Vec<String>,
    pub same_background: bool,
    pub selected_fit: usize,
    pub selected_rotation: usize,
    pub selection: Context,
    pub slideshow: bool,
}

const FIT: usize = 0;
const STRETCH: usize = 1;
const ZOOM: usize = 2;

const MINUTES_5: usize = 0;
const MINUTES_10: usize = 1;
const MINUTES_15: usize = 2;
const MINUTES_30: usize = 3;
const HOUR_1: usize = 4;
const HOUR_2: usize = 5;

impl Default for Page {
    fn default() -> Self {
        Page {
            active_output: None,
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
            same_background: true,
            selected_fit: 0,
            selected_rotation: 0,
            selection: Context::default(),
            slideshow: false,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Context {
    active: DefaultKey,
    handles: SlotMap<DefaultKey, ImageHandle>,
    paths: SecondaryMap<DefaultKey, PathBuf>,
}

impl Page {
    /// Applies the current settings to cosmic-bg.
    pub fn apply(&mut self) {
        let path = if self.slideshow {
            &self.current_directory
        } else if let Some(path) = self.selection.paths.get(self.selection.active) {
            path
        } else {
            return;
        };

        let output = if self.same_background {
            Output::All
        } else if let Some(name) = self.outputs.active_data::<String>() {
            Output::Name(name.clone())
        } else {
            return;
        };

        let scaling_mode = match self.selected_fit {
            FIT => ScalingMode::Fit([0.0, 0.0, 0.0]),
            STRETCH => ScalingMode::Stretch,
            ZOOM => ScalingMode::Zoom,
            _ => return,
        };

        let entry = Entry::new(output.clone(), path.clone())
            .scaling_mode(scaling_mode)
            .rotation_frequency(self.rotation_frequency);

        if output != Output::All {
            self.config.backgrounds.clear();
            self.config.outputs.clear();
        }

        wallpaper::set(&mut self.config, entry);
    }

    pub fn update(&mut self, message: Message) {
        match message {
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
                self.same_background = value;
            }

            Message::Select(id) => {
                self.selection.active = id;
            }

            Message::Slideshow(value) => {
                self.slideshow = value;
            }

            Message::Update((config, outputs, selection)) => {
                self.config = config;
                self.selection = selection;
                self.outputs.clear();

                {
                    let mut first = None;
                    for (name, model) in outputs {
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
                }

                if let Some(entry) = self
                    .config
                    .backgrounds
                    .iter()
                    .find(|b| b.output == Output::All)
                {
                    self.same_background = true;
                    for (entity, path) in self.selection.paths.iter() {
                        if path == &entry.source {
                            self.selection.active = entity;

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
                    }
                }
            }
        }

        self.apply();
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

            crate::pages::Message::DesktopWallpaper(Message::Update((config, outputs, update)))
        }))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

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

            let mut children = Vec::with_capacity(3);

            if let Some(image) = page.selection.handles.get(page.selection.active) {
                children.push(crate::widget::display_container(image.clone(), 300.0));
            }

            children.push(if page.same_background {
                cosmic::widget::text("All Displays")
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill)
                    .apply(cosmic::iced::widget::container)
                    .width(Length::Fill)
                    .padding([0, 0, 16, 0])
                    .into()
            } else {
                cosmic::widget::horiontal_view_switcher(&page.outputs)
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
                        toggler(None, page.same_background, Message::SameBackground),
                    ))
                    .add(settings::item(&descriptions[1], background_fit))
                    .add(settings::item(
                        &descriptions[2],
                        toggler(None, page.slideshow, Message::Slideshow),
                    ));

                if page.slideshow {
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

            children.push(column(image_column).spacing(12).padding(0).into());

            cosmic::iced::widget::column(children)
                .spacing(22)
                .padding(0)
                .max_width(683)
                .apply(Element::from)
                .map(crate::pages::Message::DesktopWallpaper)
        })
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
