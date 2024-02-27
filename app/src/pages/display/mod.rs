// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod arrangement;
pub mod graphics;
pub mod text;

use crate::{app, pages};
use apply::Apply;
use arrangement::Arrangement;
use cosmic::iced::Length;
use cosmic::iced_widget::scrollable::{Direction, Properties, RelativeOffset};
use cosmic::widget::{
    column, container, dropdown, list_column, segmented_button, tab_bar, toggler,
};
use cosmic::{command, Command, Element};
use cosmic_randr_shell::{List, Output, OutputKey, Transform};
use cosmic_settings_page::{self as page, section, Section};
use slotmap::{Key, SlotMap};
use std::collections::BTreeMap;
use std::{process::ExitStatus, sync::Arc};

/// Display color depth options
#[derive(Clone, Copy, Debug)]
pub struct ColorDepth(usize);

/// Identifies the content to display in the context drawer
pub enum ContextDrawer {
    GraphicsMode,
    NightLight,
}

/// Display mirroring options
#[derive(Clone, Copy, Debug)]
pub enum Mirroring {
    Disable,
    ProjectToAll,
    Project(OutputKey),
    Mirror(OutputKey),
}

/// Night light preferences
#[derive(Clone, Copy, Debug)]
pub enum NightLight {
    /// Toggles night light's automatic scheduling.
    AutoSchedule(bool),
    /// Sets the night light schedule.
    ManualSchedule,
    /// Changes the preferred night light temperature.
    Temperature(f32),
    /// Toggles night light mode
    Toggle(bool),
}

#[derive(Clone, Debug)]
pub enum Message {
    /// Change placement of display
    Position(OutputKey, i32, i32),
    /// Changes the active display being configured.
    Display(segmented_button::Entity),
    /// Set the color depth of a display.
    ColorDepth(ColorDepth),
    /// Set the color profile of a display.
    ColorProfile(usize),
    /// Toggles display on or off.
    DisplayToggle(bool),
    /// Changes the hybrid graphics mode.
    GraphicsMode(graphics::Mode),
    /// Shows the graphics mode context drawer.
    GraphicsModeContext,
    /// Status of an applied graphics mode change
    GraphicsModeResult(Arc<std::io::Result<ExitStatus>>),
    /// Configures mirroring status of a display.
    Mirroring(Mirroring),
    /// Handle night light preferences.
    NightLight(NightLight),
    /// Show the night light mode context drawer.
    NightLightContext,
    /// Set the orientation of a display.
    Orientation(Transform),
    /// Status of an applied display change.
    RandrResult(Arc<std::io::Result<ExitStatus>>),
    /// Set the refresh rate of a display.
    RefreshRate(usize),
    /// Set the resolution of a display.
    Resolution(usize),
    /// Set the preferred scale for a display.
    Scale(usize),
    /// Refreshes display outputs.
    Update {
        /// The current graphics mode
        graphics: Option<Arc<std::io::Result<graphics::Mode>>>,

        /// Available outputs from cosmic-randr.
        randr: Arc<Result<List, cosmic_randr_shell::Error>>,
    },
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        let page_message = crate::pages::Message::Displays(message);
        app::Message::PageMessage(page_message)
    }
}

#[derive(Clone, Copy)]
enum Randr {
    Position(i32, i32),
    RefreshRate(u32),
    Resolution(u32, u32),
    Scale(u32),
    Transform(Transform),
    Toggle(bool),
}

/// The page struct for the display settings page.
pub struct Page {
    list: List,
    display_tabs: segmented_button::SingleSelectModel,
    active_display: OutputKey,
    config: Config,
    cache: ViewCache,
    context: Option<ContextDrawer>,
    display_arrangement_scrollable: cosmic::widget::Id,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            list: List::default(),
            display_tabs: segmented_button::SingleSelectModel::default(),
            active_display: OutputKey::default(),
            config: Config::default(),
            cache: ViewCache::default(),
            context: None,
            display_arrangement_scrollable: cosmic::widget::Id::unique(),
        }
    }
}

#[derive(Default)]
struct Config {
    /// Whether night light is enabled.
    night_light_enabled: bool,
    graphics_mode: Option<graphics::Mode>,
    refresh_rate: Option<u32>,
    resolution: Option<(u32, u32)>,
    scale: u32,
}

/// Cached view content for widgets.
#[derive(Default)]
struct ViewCache {
    modes: BTreeMap<(u32, u32), Vec<u32>>,
    orientations: [&'static str; 4],
    refresh_rates: Vec<String>,
    resolutions: Vec<String>,
    orientation_selected: Option<usize>,
    refresh_rate_selected: Option<usize>,
    resolution_selected: Option<usize>,
    scale_selected: Option<usize>,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            // Graphics switching and light mode
            sections.insert(
                Section::default()
                    .descriptions(vec![
                        text::GRAPHICS_MODE.as_str().into(),
                        text::GRAPHICS_MODE_COMPUTE_DESC.as_str().into(),
                        text::GRAPHICS_MODE_HYBRID_DESC.as_str().into(),
                        text::GRAPHICS_MODE_INTEGRATED_DESC.as_str().into(),
                        text::GRAPHICS_MODE_NVIDIA_DESC.as_str().into(),
                        text::NIGHT_LIGHT.as_str().into(),
                        text::NIGHT_LIGHT_AUTO.as_str().into(),
                        text::NIGHT_LIGHT_DESCRIPTION.as_str().into(),
                    ])
                    .view::<Page>(|_binder, page, _section| page.graphics_mode_view()),
            ),
            // Display arrangement
            sections.insert(
                Section::default()
                    .title(&*text::DISPLAY_ARRANGEMENT)
                    .descriptions(vec![
                        text::DISPLAY_ARRANGEMENT.as_str().into(),
                        text::DISPLAY_ARRANGEMENT_DESC.as_str().into(),
                    ])
                    // Show section when there is more than 1 display
                    .show_while::<Page>(|page| page.list.outputs.len() > 1)
                    .view::<Page>(|_binder, page, _section| page.display_arrangement_view()),
            ),
            // Display configuration
            sections.insert(
                Section::default()
                    .descriptions([
                        text::DISPLAY.as_str().into(),
                        text::DISPLAY_REFRESH_RATE.as_str().into(),
                        text::DISPLAY_SCALE.as_str().into(),
                        text::ORIENTATION.as_str().into(),
                        text::ORIENTATION_LANDSCAPE.as_str().into(),
                        text::ORIENTATION_PORTRAIT.as_str().into(),
                    ])
                    .view::<Page>(|_binder, page, _section| page.display_view()),
            ),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("display", "preferences-desktop-display-symbolic")
            .title(fl!("display"))
            .description(fl!("display", "desc"))
    }

    #[cfg(not(feature = "test"))]
    fn reload(&mut self, _page: page::Entity) -> Command<crate::pages::Message> {
        command::future(reload())
    }

    #[cfg(feature = "test")]
    fn reload(&mut self, _page: page::Entity) -> Command<crate::pages::Message> {
        command::future(async move {
            let mut randr = List::default();

            let test_mode = randr.modes.insert(cosmic_randr_shell::Mode {
                size: (1920, 1080),
                refresh_rate: 144_000,
                preferred: true,
            });

            randr.outputs.insert(cosmic_randr_shell::Output {
                name: "Dummy-1".into(),
                enabled: true,
                make: None,
                model: "Test 1".into(),
                physical: (1, 1),
                position: (0, 0),
                scale: 1.0,
                transform: Some(Transform::Normal),
                modes: vec![test_mode],
                current: Some(test_mode),
            });

            randr.outputs.insert(cosmic_randr_shell::Output {
                name: "Dummy-2".into(),
                enabled: true,
                make: None,
                model: "Test 1".into(),
                physical: (1, 1),
                position: (1920, 0),
                scale: 1.0,
                transform: Some(Transform::Normal),
                modes: vec![test_mode],
                current: Some(test_mode),
            });

            crate::pages::Message::Displays(Message::Update {
                graphics: graphics::fetch().await,
                randr: Arc::new(Ok(randr)),
            })
        })
    }

    fn context_drawer(&self) -> Option<Element<pages::Message>> {
        Some(match self.context {
            Some(ContextDrawer::GraphicsMode) => self.graphics_mode_context_view(),

            Some(ContextDrawer::NightLight) => self.night_light_context_view(),

            None => return None,
        })
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        match message {
            Message::RandrResult(result) => {
                if let Some(Err(why)) = Arc::into_inner(result) {
                    tracing::error!(?why, "cosmic-randr error");
                }

                return cosmic::command::future(async {
                    crate::Message::PageMessage(reload().await)
                });
            }

            Message::Display(display) => self.set_display(display),

            Message::ColorDepth(color_depth) => return self.set_color_depth(color_depth),

            Message::ColorProfile(profile) => return self.set_color_profile(profile),

            Message::DisplayToggle(enable) => return self.toggle_display(enable),

            Message::GraphicsMode(mode) => return self.set_graphics_mode(mode),

            Message::GraphicsModeContext => {
                self.context = Some(ContextDrawer::GraphicsMode);
                return cosmic::command::message(app::Message::OpenContextDrawer(
                    text::GRAPHICS_MODE.clone().into(),
                ));
            }

            Message::GraphicsModeResult(result) => {
                if let Some(Err(why)) = Arc::into_inner(result) {
                    tracing::error!(?why, "system76-power error");
                }
            }

            Message::Mirroring(mirroring) => match mirroring {
                Mirroring::Disable => (),
                Mirroring::Mirror(target_display) => (),
                Mirroring::Project(target_display) => (),
                Mirroring::ProjectToAll => (),
            },

            Message::NightLight(night_light) => {}

            Message::NightLightContext => {
                self.context = Some(ContextDrawer::NightLight);
                return cosmic::command::message(app::Message::OpenContextDrawer(
                    text::NIGHT_LIGHT.clone().into(),
                ));
            }

            Message::Orientation(orientation) => return self.set_orientation(orientation),

            Message::Position(display, x, y) => return self.set_position(display, x, y),

            Message::RefreshRate(rate) => return self.set_refresh_rate(rate),

            Message::Resolution(option) => return self.set_resolution(option),

            Message::Scale(scale) => return self.set_scale(scale),

            Message::Update { graphics, randr } => {
                match graphics.and_then(Arc::into_inner) {
                    Some(Ok(mode)) => {
                        self.config.graphics_mode = Some(mode);
                    }

                    Some(Err(why)) => {
                        tracing::error!(?why, "error fetching graphics switching mode");
                    }

                    None => (),
                }

                match Arc::into_inner(randr) {
                    Some(Ok(outputs)) => self.update_displays(outputs),

                    Some(Err(why)) => {
                        tracing::error!(?why, "error fetching displays");
                    }

                    None => (),
                }

                self.cache.orientations = [
                    text::ORIENTATION_LANDSCAPE.as_str(),
                    text::ORIENTATION_PORTRAIT.as_str(),
                    text::ORIENTATION_LANDSCAPE_FLIPPED.as_str(),
                    text::ORIENTATION_PORTRAIT_FLIPPED.as_str(),
                ];
            }
        }

        cosmic::iced::widget::scrollable::snap_to(
            self.display_arrangement_scrollable.clone(),
            RelativeOffset { x: 0.5, y: 0.5 },
        )
    }

    /// View for the display arrangement section.
    pub fn display_arrangement_view(&self) -> Element<pages::Message> {
        let theme = cosmic::theme::active();

        column()
            .padding(cosmic::iced::Padding::from([
                theme.cosmic().space_s(),
                theme.cosmic().space_m(),
            ]))
            .spacing(theme.cosmic().space_xs())
            .push(cosmic::widget::text::body(&*text::DISPLAY_ARRANGEMENT_DESC))
            .push({
                Arrangement::new(&self.list, &self.display_tabs)
                    .on_select(|id| pages::Message::Displays(Message::Display(id)))
                    .on_placement(|id, x, y| pages::Message::Displays(Message::Position(id, x, y)))
                    .apply(cosmic::widget::scrollable)
                    .id(self.display_arrangement_scrollable.clone())
                    .width(Length::Shrink)
                    .direction(Direction::Horizontal(Properties::new()))
                    .apply(container)
                    .center_x()
                    .width(Length::Fill)
            })
            .apply(cosmic::widget::list::container)
            .into()
    }

    /// View for the display configuration section.
    pub fn display_view(&self) -> Element<pages::Message> {
        let theme = cosmic::theme::active();

        let Some(&active_id) = self.display_tabs.active_data::<OutputKey>() else {
            return column().into();
        };

        let active_output = &self.list.outputs[active_id];

        let display_options = list_column()
            .add(cosmic::widget::settings::item(
                &*text::DISPLAY_RESOLUTION,
                dropdown(
                    &self.cache.resolutions,
                    self.cache.resolution_selected,
                    Message::Resolution,
                ),
            ))
            .add(cosmic::widget::settings::item(
                &*text::DISPLAY_REFRESH_RATE,
                dropdown(
                    &self.cache.refresh_rates,
                    self.cache.refresh_rate_selected,
                    Message::RefreshRate,
                ),
            ))
            .add(cosmic::widget::settings::item(
                &*text::DISPLAY_SCALE,
                dropdown(
                    &["50%", "75%", "100%", "125%", "150%", "175%", "200%"],
                    self.cache.scale_selected,
                    Message::Scale,
                ),
            ))
            .add(cosmic::widget::settings::item(
                &*text::ORIENTATION,
                dropdown(
                    &self.cache.orientations,
                    self.cache.orientation_selected,
                    |id| {
                        Message::Orientation(match id {
                            0 => Transform::Normal,
                            1 => Transform::Rotate90,
                            2 => Transform::Flipped,
                            _ => Transform::Flipped90,
                        })
                    },
                ),
            ));

        let mut content = column().spacing(theme.cosmic().space_m());

        if self.list.outputs.len() > 1 {
            let display_switcher =
                tab_bar::horizontal(&self.display_tabs).on_activate(Message::Display);

            let display_enable = list_column().add(cosmic::widget::settings::item(
                &*text::DISPLAY_ENABLE,
                toggler(None, active_output.enabled, Message::DisplayToggle),
            ));

            content = content.push(display_switcher).push(display_enable);
        }

        content
            .push(cosmic::widget::text::heading(&*text::DISPLAY_OPTIONS))
            .push(display_options)
            .apply(Element::from)
            .map(pages::Message::Displays)
    }

    /// Displays the night light context drawer.
    pub fn night_light_context_view(&self) -> Element<pages::Message> {
        column().into()
    }

    /// Reloads the display list, and all information relevant to the active display.
    pub fn update_displays(&mut self, list: List) {
        self.active_display = OutputKey::null();
        self.display_tabs.clear();
        self.list = list;

        let sorted_outputs = self
            .list
            .outputs
            .iter()
            .map(|(key, output)| (&*output.name, key))
            .collect::<BTreeMap<_, _>>();

        for (name, id) in sorted_outputs {
            let Some(output) = self.list.outputs.get(id) else {
                continue;
            };

            self.display_tabs
                .insert()
                .text(crate::utils::display_name(&output.name, output.physical))
                .data::<OutputKey>(id);
        }

        self.display_tabs.activate_position(0);

        // Retrieve data for the first, activated display.
        self.set_display(self.display_tabs.active());
    }

    /// Changes the color depth of the active display.
    pub fn set_color_depth(&mut self, depth: ColorDepth) -> Command<app::Message> {
        unimplemented!()
    }

    /// Changes the color profile of the active display.
    pub fn set_color_profile(&mut self, profile: usize) -> Command<app::Message> {
        unimplemented!()
    }

    /// Changes the active display, and regenerates available options for it.
    pub fn set_display(&mut self, display: segmented_button::Entity) {
        let Some(&output_id) = self.display_tabs.data::<OutputKey>(display) else {
            return;
        };

        let Some(output) = self.list.outputs.get_mut(output_id) else {
            return;
        };

        self.display_tabs.activate(display);
        self.active_display = output_id;
        self.config.refresh_rate = None;
        self.config.resolution = None;
        self.config.scale = (output.scale * 100.0) as u32;

        self.cache.modes.clear();
        self.cache.refresh_rates.clear();
        self.cache.resolutions.clear();
        self.cache.orientation_selected = match output.transform {
            Some(Transform::Normal) => Some(0),
            Some(Transform::Rotate90) => Some(1),
            Some(Transform::Flipped) => Some(2),
            Some(Transform::Flipped90) => Some(3),
            _ => None,
        };
        self.cache.resolution_selected = None;
        self.cache.refresh_rate_selected = None;

        self.cache.scale_selected = Some(if self.config.scale < 75 {
            0
        } else if self.config.scale < 100 {
            1
        } else if self.config.scale < 125 {
            2
        } else if self.config.scale < 150 {
            3
        } else if self.config.scale < 175 {
            4
        } else if self.config.scale < 200 {
            5
        } else {
            6
        });

        if let Some(current_mode_id) = output.current {
            for (mode_id, mode) in output
                .modes
                .iter()
                .filter_map(|&id| self.list.modes.get(id).map(|m| (id, m)))
            {
                let refresh_rates = self.cache.modes.entry(mode.size).or_default();

                refresh_rates.push(mode.refresh_rate);

                if current_mode_id == mode_id {
                    self.cache.refresh_rate_selected = Some(refresh_rates.len() - 1);
                    self.cache.resolution_selected = Some(self.cache.modes.len() - 1);
                    self.config.resolution = Some(mode.size);
                    self.config.refresh_rate = Some(mode.refresh_rate);
                }
            }
        }

        for (&resolution, rates) in self.cache.modes.iter().rev() {
            self.cache
                .resolutions
                .push(format!("{}x{}", resolution.0, resolution.1));
            if Some(resolution) == self.config.resolution {
                cache_rates(&mut self.cache.refresh_rates, rates);
            }
        }
    }

    /// Change display orientation.
    pub fn set_orientation(&mut self, transform: Transform) -> Command<app::Message> {
        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Command::none();
        };

        self.cache.orientation_selected = match transform {
            Transform::Normal => Some(0),
            Transform::Rotate90 => Some(1),
            Transform::Flipped => Some(2),
            _ => Some(3),
        };

        self.exec_randr(output, Randr::Transform(transform))
    }

    /// Changes the position of the display.
    pub fn set_position(&mut self, display: OutputKey, x: i32, y: i32) -> Command<app::Message> {
        let Some(output) = self.list.outputs.get_mut(display) else {
            return Command::none();
        };

        output.position = (x, y);

        if cfg!(feature = "test") {
            tracing::debug!("set position {x},{y}");
            return Command::none();
        }

        let output = &self.list.outputs[display];
        self.exec_randr(output, Randr::Position(x, y))
    }

    /// Changes the refresh rate of the active display.
    pub fn set_refresh_rate(&mut self, option: usize) -> Command<app::Message> {
        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Command::none();
        };

        if let Some(ref resolution) = self.config.resolution {
            if let Some(rates) = self.cache.modes.get(resolution) {
                if let Some(&rate) = rates.get(option) {
                    self.cache.refresh_rate_selected = Some(option);
                    self.config.refresh_rate = Some(rate);
                    return self.exec_randr(output, Randr::RefreshRate(rate));
                }
            }
        }

        Command::none()
    }

    /// Change the resolution of the active display.
    pub fn set_resolution(&mut self, option: usize) -> Command<app::Message> {
        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Command::none();
        };

        let Some((&resolution, rates)) = self.cache.modes.iter().rev().nth(option) else {
            return Command::none();
        };

        self.cache.refresh_rates.clear();
        cache_rates(&mut self.cache.refresh_rates, rates);

        let Some(&rate) = rates.first() else {
            return Command::none();
        };

        self.config.refresh_rate = Some(rate);
        self.config.resolution = Some(resolution);
        self.cache.refresh_rate_selected = Some(0);
        self.cache.resolution_selected = Some(option);
        self.exec_randr(output, Randr::Resolution(resolution.0, resolution.1))
    }

    /// Set the scale of the active display.
    pub fn set_scale(&mut self, option: usize) -> Command<app::Message> {
        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Command::none();
        };

        let scale = (option * 25 + 50) as u32;

        self.cache.scale_selected = Some(option);
        self.config.scale = scale;
        self.exec_randr(output, Randr::Scale(scale))
    }

    /// Enables or disables the active display.
    pub fn toggle_display(&mut self, enable: bool) -> Command<app::Message> {
        let Some(output) = self.list.outputs.get_mut(self.active_display) else {
            return Command::none();
        };

        output.enabled = enable;

        let output = &self.list.outputs[self.active_display];
        self.exec_randr(output, Randr::Toggle(output.enabled))
    }

    /// Applies a display configuration via `cosmic-randr`.
    fn exec_randr(&self, output: &Output, request: Randr) -> Command<app::Message> {
        let Some(current) = output.current.and_then(|id| self.list.modes.get(id)) else {
            return Command::none();
        };

        let name = &*output.name;

        let mut command = tokio::process::Command::new("cosmic-randr");

        match request {
            Randr::Position(x, y) => {
                command
                    .arg("mode")
                    .arg("--pos-x")
                    .arg(itoa::Buffer::new().format(x))
                    .arg("--pos-y")
                    .arg(itoa::Buffer::new().format(y))
                    .arg(name)
                    .arg(itoa::Buffer::new().format(current.size.0))
                    .arg(itoa::Buffer::new().format(current.size.1));
            }

            Randr::RefreshRate(rate) => {
                command
                    .arg("mode")
                    .arg("--refresh")
                    .arg(
                        &[
                            itoa::Buffer::new().format(rate / 1000),
                            ".",
                            itoa::Buffer::new().format(rate % 1000),
                        ]
                        .concat(),
                    )
                    .arg(name)
                    .arg(itoa::Buffer::new().format(current.size.0))
                    .arg(itoa::Buffer::new().format(current.size.1));
            }

            Randr::Resolution(width, height) => {
                command
                    .arg("mode")
                    .arg(name)
                    .arg(itoa::Buffer::new().format(width))
                    .arg(itoa::Buffer::new().format(height));
            }

            Randr::Scale(scale) => {
                command
                    .arg("mode")
                    .arg("--scale")
                    .arg(
                        &[
                            itoa::Buffer::new().format(scale / 100),
                            ".",
                            itoa::Buffer::new().format(scale % 100),
                        ]
                        .concat(),
                    )
                    .arg(name)
                    .arg(itoa::Buffer::new().format(current.size.0))
                    .arg(itoa::Buffer::new().format(current.size.1));
            }

            Randr::Toggle(enable) => {
                command
                    .arg(if enable { "enable" } else { "disable" })
                    .arg(name);
            }

            Randr::Transform(transform) => {
                command
                    .arg("mode")
                    .arg("--transform")
                    .arg(&*format!("{transform}"))
                    .arg(name)
                    .arg(itoa::Buffer::new().format(current.size.0))
                    .arg(itoa::Buffer::new().format(current.size.1));
            }
        }

        cosmic::command::future(async move {
            tracing::debug!(?command, "executing");
            app::Message::from(Message::RandrResult(Arc::new(command.status().await)))
        })
    }
}

fn cache_rates(cached_rates: &mut Vec<String>, rates: &[u32]) {
    *cached_rates = rates
        .iter()
        .map(|&rate| format!("{:>3}.{:02} Hz", rate / 1000, rate % 1000))
        .collect();
}

pub async fn reload() -> crate::pages::Message {
    let graphics_fut = graphics::fetch();
    let randr_fut = cosmic_randr_shell::list();
    let (graphics, randr) = futures::future::zip(graphics_fut, randr_fut).await;

    crate::pages::Message::Displays(Message::Update {
        graphics,
        randr: Arc::new(randr),
    })
}
