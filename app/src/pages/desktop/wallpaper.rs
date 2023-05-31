// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::HashMap, path::PathBuf, time::Instant};

use apply::Apply;
use cosmic::{
    iced::alignment::Horizontal,
    iced::widget::{column, row},
    iced::Length,
    iced_runtime::core::image::Handle as ImageHandle,
    widget::{list_column, settings, toggler},
    Element,
};
use cosmic_settings_desktop::wallpaper::{self, Entry, Output, ScalingMode};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::{DefaultKey, SecondaryMap, SlotMap};

#[derive(Clone, Debug)]
pub enum Message {
    Fit(String),
    SameBackground(bool),
    Select(DefaultKey),
    Slideshow(bool),
    Update((wallpaper::Config, HashMap<String, String>, Context)),
}

pub struct Page {
    pub config: wallpaper::Config,
    pub fit_options: Vec<String>,
    pub outputs: HashMap<String, String>,
    pub same_background: bool,
    pub selected_fit: u32,
    pub selection: Context,
    pub slideshow: bool,
}

const FIT: u32 = 0;
const STRETCH: u32 = 1;
const ZOOM: u32 = 2;

impl Default for Page {
    fn default() -> Self {
        Page {
            config: wallpaper::Config::default(),
            fit_options: vec!["Fit to Screen".into(), "Stretch".into(), "Zoom".into()],
            outputs: HashMap::new(),
            same_background: true,
            selected_fit: 0,
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
    pub fn apply(&mut self) {
        let Some(path) = self.selection.paths.get(self.selection.active) else {
            return
        };

        let mut entry = Entry::new(Output::All, path.clone());

        match self.selected_fit {
            FIT => entry.scaling_mode = ScalingMode::Fit([0.0, 0.0, 0.0]),
            STRETCH => entry.scaling_mode = ScalingMode::Stretch,
            ZOOM => entry.scaling_mode = ScalingMode::Zoom,
            _ => (),
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
                    .map_or(0, |(indice, _)| indice as u32);

                self.apply();
            }

            Message::SameBackground(value) => {
                self.same_background = value;
            }

            Message::Select(id) => {
                self.selection.active = id;
                self.apply();
            }

            Message::Slideshow(value) => self.slideshow = value,

            Message::Update((config, outputs, selection)) => {
                self.config = config;
                self.selection = selection;
                self.outputs = outputs;

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
                        }
                    }
                }
            }
        }
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

            children.push(
                cosmic::widget::text("All Displays")
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill)
                    .apply(cosmic::iced::widget::container)
                    .width(Length::Fill)
                    .padding([0, 0, 16, 0])
                    .into(),
            );

            let background_fit = cosmic::iced::widget::pick_list(
                &page.fit_options,
                page.fit_options.get(page.selected_fit as usize).cloned(),
                Message::Fit,
            );

            children.push(
                list_column()
                    .add(settings::item(
                        &descriptions[0],
                        toggler(None, page.same_background, Message::SameBackground),
                    ))
                    .add(settings::item(&descriptions[1], background_fit))
                    .add(settings::item(
                        &descriptions[2],
                        toggler(None, page.slideshow, Message::Slideshow),
                    ))
                    .into(),
            );

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
