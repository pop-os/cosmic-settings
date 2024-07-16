// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod arrangement;
// pub mod night_light;

use crate::{app, pages};
use arrangement::Arrangement;
use cosmic::iced::{time, Alignment, Length};
use cosmic::iced_widget::scrollable::{Direction, Properties, RelativeOffset};
use cosmic::prelude::CollectionWidget;
use cosmic::widget::{
    self, column, container, dropdown, list_column, segmented_button, tab_bar, toggler,
};
use cosmic::{command, Apply, Command, Element};
use cosmic_randr_shell::{List, Output, OutputKey, Transform};
use cosmic_settings_page::{self as page, section, Section};
use slab::Slab;
use slotmap::{Key, SecondaryMap, SlotMap};
use std::{collections::BTreeMap, process::ExitStatus, sync::Arc};

/// Display color depth options
#[derive(Clone, Copy, Debug)]
pub struct ColorDepth(usize);

/// Identifies the content to display in the context drawer
// pub enum ContextDrawer {
//     NightLight,
// }

/// Display mirroring options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mirroring {
    Disable,
    // ProjectToAll,
    Project(OutputKey),
    Mirror(OutputKey),
}

/// Night light preferences
// #[derive(Clone, Copy, Debug)]
// pub enum NightLight {
/// Toggles night light's automatic scheduling.
//     AutoSchedule(bool),
/// Sets the night light schedule.
//     ManualSchedule,
/// Changes the preferred night light temperature.
//     Temperature(f32),
/// Toggles night light mode
//     Toggle(bool),
// }

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
    /// The dialog was cancelled, and will revert settings.
    DialogCancel,
    /// The dialog was completed.
    DialogComplete,
    /// How long until the dialog automatically cancelles, in seconds.
    DialogCountdown,
    /// Toggles display on or off.
    DisplayToggle(bool),
    /// Configures mirroring status of a display.
    Mirroring(Mirroring),
    /// Handle night light preferences.
    //  NightLight(NightLight),
    /// Show the night light mode context drawer.
    //  NightLightContext,
    /// Set the orientation of a display.
    Orientation(Transform),
    /// Status of an applied display change.
    RandrResult(Arc<std::io::Result<ExitStatus>>),
    /// Request to reload the page.
    Refresh,
    /// Set the refresh rate of a display.
    RefreshRate(usize),
    /// Set the resolution of a display.
    Resolution(usize),
    /// Set the preferred scale for a display.
    Scale(usize),
    /// Refreshes display outputs.
    Update {
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Randr {
    Mirror(OutputKey),
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
    mirror_map: SecondaryMap<OutputKey, OutputKey>,
    mirror_menu: widget::dropdown::multi::Model<String, Mirroring>,
    active_display: OutputKey,
    background_service: Option<tokio::task::JoinHandle<()>>,
    config: Config,
    cache: ViewCache,
    //  context: Option<ContextDrawer>,
    display_arrangement_scrollable: widget::Id,
    /// The setting to revert to if the next dialog page is cancelled.
    dialog: Option<Randr>,
    /// the instant the setting was changed.
    dialog_countdown: usize,
    show_display_options: bool,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            list: List::default(),
            display_tabs: segmented_button::SingleSelectModel::default(),
            mirror_map: SecondaryMap::new(),
            mirror_menu: widget::dropdown::multi::model(),
            active_display: OutputKey::default(),
            background_service: None,
            config: Config::default(),
            cache: ViewCache::default(),
            //          context: None,
            display_arrangement_scrollable: widget::Id::unique(),
            dialog: None,
            dialog_countdown: 0,
            show_display_options: true,
        }
    }
}

#[derive(Default)]
struct Config {
    /// Whether night light is enabled.
    //  night_light_enabled: bool,
    refresh_rate: Option<u32>,
    resolution: Option<(u32, u32)>,
    scale: u32,
}

/// Cached view content for widgets.
#[derive(Default)]
struct ViewCache {
    modes: BTreeMap<(u32, u32), Vec<u32>>,
    orientations: [String; 4],
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
            // Night light
            //            sections.insert(
            //                Section::default()
            //                    .descriptions(vec![
            //                        text::NIGHT_LIGHT.as_str().into(),
            //                        text::NIGHT_LIGHT_AUTO.as_str().into(),
            //                        text::NIGHT_LIGHT_DESCRIPTION.as_str().into(),
            //                    ])
            //                    .view::<Page>(move |_binder, page, _section| page.night_light_view()),
            //            ),
            // Display arrangement
            sections.insert(display_arrangement()),
            // Display configuration
            sections.insert(display_configuration()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("display", "preferences-desktop-display-symbolic")
            .title(fl!("display"))
            .description(fl!("display", "desc"))
    }

    #[cfg(not(feature = "test"))]
    fn on_enter(
        &mut self,
        _page: page::Entity,
        sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> Command<crate::pages::Message> {
        if let Some(task) = self.background_service.take() {
            task.abort();
        }

        // Spawns a background service to monitor for display state changes.
        // This must be spawned onto its own thread because `*mut wayland_sys::client::wl_display` is not Send-able.
        let runtime = tokio::runtime::Handle::current();
        self.background_service = Some(tokio::task::spawn_blocking(move || {
            runtime.block_on(async move {
                let (tx, mut rx) = tachyonix::channel(5);
                let Ok((mut context, mut event_queue)) = cosmic_randr::connect(tx) else {
                    return;
                };

                while context.dispatch(&mut event_queue).await.is_ok() {
                    while let Ok(message) = rx.try_recv() {
                        if let cosmic_randr::Message::ManagerDone = message {
                            let _ = sender
                                .send(pages::Message::Displays(Message::Refresh))
                                .await;
                        }
                    }
                }
            });
        }));

        command::future(on_enter())
    }

    fn on_leave(&mut self) -> Command<crate::pages::Message> {
        if let Some(task) = self.background_service.take() {
            task.abort();
        }

        Command::none()
    }

    #[cfg(feature = "test")]
    fn on_enter(
        &mut self,
        _page: page::Entity,
        sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> Command<crate::pages::Message> {
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
                mirroring: None,
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
                mirroring: None,
                physical: (1, 1),
                position: (1920, 0),
                scale: 1.0,
                transform: Some(Transform::Normal),
                modes: vec![test_mode],
                current: Some(test_mode),
            });

            crate::pages::Message::Displays(Message::Update {
                randr: Arc::new(Ok(randr)),
            })
        })
    }

    //    fn context_drawer(&self) -> Option<Element<pages::Message>> {
    //        Some(match self.context {

    //            Some(ContextDrawer::NightLight) => self.night_light_context_view(),

    //            None => return None,
    //        })
    //    }

    /// Opens a dialog to confirm the display settings.
    ///
    /// This dialog has a 10 (arbitrary) second counter which will
    /// automatically revert to the original display settings when depleted.
    ///
    /// To make a setting activate this dialog. Call the `set_dialog` method with
    /// the Randr enum value which undos the current change. Makde sure the
    /// return value is returned with the `exec_value` return value within a batch
    /// command.
    fn dialog(&self) -> Option<Element<pages::Message>> {
        self.dialog?;
        let element = widget::dialog(fl!("dialog", "title"))
            .body(fl!("dialog", "change-prompt", time = self.dialog_countdown))
            .primary_action(
                widget::button::suggested(fl!("dialog", "keep-changes"))
                    .on_press(pages::Message::Displays(Message::DialogComplete)),
            )
            .secondary_action(
                widget::button::standard(fl!("dialog", "revert-settings"))
                    .on_press(pages::Message::Displays(Message::DialogCancel)),
            )
            .into();
        Some(element)
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        match message {
            Message::RandrResult(result) => {
                if let Some(Err(why)) = Arc::into_inner(result) {
                    tracing::error!(?why, "cosmic-randr error");
                } else {
                    // Reload display info
                    return cosmic::command::future(async move {
                        crate::Message::PageMessage(on_enter().await)
                    });
                }
            }

            Message::DialogCancel => {
                let Some(request) = self.dialog else {
                    return Command::none();
                };
                let Some(output) = self.list.outputs.get(self.active_display) else {
                    return Command::none();
                };
                self.dialog = None;
                self.dialog_countdown = 0;
                return self.exec_randr(output, request);
            }

            Message::DialogComplete => {
                self.dialog = None;
                self.dialog_countdown = 0;
            }

            Message::DialogCountdown => {
                if self.dialog_countdown == 0 {
                    if self.dialog.is_some() {
                        return command::message(app::Message::from(Message::DialogCancel));
                    }
                } else {
                    self.dialog_countdown -= 1;
                    return command::future(async move {
                        tokio::time::sleep(time::Duration::from_secs(1)).await;
                        Message::DialogCountdown
                    });
                }
            }

            Message::Display(display) => self.set_display(display),

            Message::ColorDepth(color_depth) => return self.set_color_depth(color_depth),

            Message::ColorProfile(profile) => return self.set_color_profile(profile),

            Message::DisplayToggle(enable) => return self.toggle_display(enable),

            Message::Mirroring(mirroring) => match mirroring {
                Mirroring::Disable => return self.toggle_display(true),

                Mirroring::Mirror(from_display) => {
                    let Some(output) = self.list.outputs.get(self.active_display) else {
                        return Command::none();
                    };

                    return self.exec_randr(output, Randr::Mirror(from_display));
                }

                Mirroring::Project(to_display) => {
                    let Some(output) = self.list.outputs.get(to_display) else {
                        return Command::none();
                    };

                    return self.exec_randr(output, Randr::Mirror(self.active_display));
                } // Mirroring::ProjectToAll => (),
            },

            //            Message::NightLight(night_light) => {}
            //
            //            Message::NightLightContext => {
            //                self.context = Some(ContextDrawer::NightLight);
            //                return cosmic::command::message(app::Message::OpenContextDrawer(
            //                    text::NIGHT_LIGHT.clone().into(),
            //                ));
            //            }
            Message::Orientation(orientation) => return self.set_orientation(orientation),

            Message::Position(display, x, y) => return self.set_position(display, x, y),

            Message::Refresh => {
                return cosmic::command::future(async move {
                    crate::Message::PageMessage(on_enter().await)
                });
            }

            Message::RefreshRate(rate) => return self.set_refresh_rate(rate),

            Message::Resolution(option) => return self.set_resolution(option),

            Message::Scale(scale) => return self.set_scale(scale),

            Message::Update { randr } => {
                match Arc::into_inner(randr) {
                    Some(Ok(outputs)) => self.update_displays(outputs),

                    Some(Err(why)) => {
                        tracing::error!(?why, "error fetching displays");
                    }

                    None => (),
                }

                self.cache.orientations = [
                    fl!("orientation", "standard"),
                    fl!("orientation", "rotate-90"),
                    fl!("orientation", "rotate-180"),
                    fl!("orientation", "rotate-270"),
                ];
            }
        }

        cosmic::iced::widget::scrollable::snap_to(
            self.display_arrangement_scrollable.clone(),
            RelativeOffset { x: 0.5, y: 0.5 },
        )
    }

    /// Displays the night light context drawer.
    //    pub fn night_light_context_view(&self) -> Element<pages::Message> {
    //        column().into()
    //    }

    /// Reloads the display list, and all information relevant to the active display.
    pub fn update_displays(&mut self, list: List) {
        let active_display_name = self
            .display_tabs
            .text_remove(self.display_tabs.active())
            .unwrap_or_default();
        let mut active_tab_pos: u16 = 0;

        self.active_display = OutputKey::null();
        self.display_tabs.clear();
        self.mirror_map.clear();
        self.list = list;

        let sorted_outputs = self
            .list
            .outputs
            .iter()
            .map(|(key, output)| (&*output.name, key))
            .collect::<BTreeMap<_, _>>();

        for (pos, (_name, id)) in sorted_outputs.into_iter().enumerate() {
            let Some(output) = self.list.outputs.get(id) else {
                continue;
            };

            if let Some(mirroring_from) = output.mirroring.as_deref() {
                for (other_id, other_output) in &self.list.outputs {
                    if other_output.name == mirroring_from {
                        self.mirror_map.insert(id, other_id);
                        break;
                    }
                }
            }

            let text = crate::utils::display_name(&output.name, output.physical);

            if text == active_display_name {
                active_tab_pos = pos as u16;
            }

            self.display_tabs.insert().text(text).data::<OutputKey>(id);
        }

        self.display_tabs.activate_position(active_tab_pos);

        // Retrieve data for the first, activated display.
        self.set_display(self.display_tabs.active());
    }

    /// Sets the dialog to be shown to the user. Will not show a dialog if the
    /// current request does not change anything.
    fn set_dialog(
        &mut self,
        revert_request: Randr,
        current_request: &Randr,
    ) -> Command<app::Message> {
        if revert_request == *current_request {
            return Command::none();
        }
        self.dialog = Some(revert_request);
        self.dialog_countdown = 10;
        command::future(async {
            tokio::time::sleep(time::Duration::from_secs(1)).await;
            app::Message::from(Message::DialogCountdown)
        })
    }

    /// Changes the color depth of the active display.
    pub fn set_color_depth(&mut self, _depth: ColorDepth) -> Command<app::Message> {
        unimplemented!()
    }

    /// Changes the color profile of the active display.
    pub fn set_color_profile(&mut self, _profile: usize) -> Command<app::Message> {
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
            Some(Transform::Rotate180) => Some(2),
            Some(Transform::Rotate270) => Some(3),
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

        self.mirror_menu.clear();

        self.mirror_menu.insert(widget::dropdown::multi::list(
            None,
            vec![(fl!("mirroring", "dont"), Mirroring::Disable)],
        ));

        self.mirror_menu.insert(widget::dropdown::multi::list(
            None,
            self.list
                .outputs
                .iter()
                .filter(|&(other_id, _)| other_id != output_id)
                .map(|(other_id, other_output)| {
                    (
                        fl!("mirroring", "project", display = other_output.name.as_str()),
                        Mirroring::Project(other_id),
                    )
                })
                .collect::<Vec<_>>(),
        ));

        self.mirror_menu.insert(widget::dropdown::multi::list(
            None,
            self.list
                .outputs
                .iter()
                .filter(|&(other_id, _)| other_id != output_id)
                .map(|(other_id, other_output)| {
                    (
                        fl!("mirroring", "mirror", display = other_output.name.as_str()),
                        Mirroring::Mirror(other_id),
                    )
                })
                .collect::<Vec<_>>(),
        ));

        self.mirror_menu.selected = self
            .mirror_map
            .get(output_id)
            .map(|id| Mirroring::Mirror(*id));

        self.show_display_options = self.mirror_menu.selected.is_none();

        if self.mirror_menu.selected.is_none() {
            self.mirror_menu.selected = self
                .mirror_map
                .iter()
                .find(|&(_, mirrored_id)| *mirrored_id == output_id)
                .map(|(projected_id, _)| Mirroring::Project(projected_id));
        }

        if self.mirror_menu.selected.is_none() {
            // If mirror menu is not set, set it to don't mirror.
            self.mirror_menu.selected = Some(Mirroring::Disable);
        }
    }

    /// Change display orientation.
    pub fn set_orientation(&mut self, transform: Transform) -> Command<app::Message> {
        let request = Randr::Transform(transform);

        let mut commands = Vec::with_capacity(2);
        commands.push(match self.cache.orientation_selected {
            Some(orientation) => self.set_dialog(
                Randr::Transform(match orientation {
                    1 => Transform::Rotate90,
                    2 => Transform::Flipped180,
                    3 => Transform::Flipped270,
                    _ => Transform::Normal,
                }),
                &request,
            ),
            None => Command::none(),
        });

        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Command::none();
        };

        self.cache.orientation_selected = match transform {
            Transform::Normal => Some(0),
            Transform::Rotate90 => Some(1),
            Transform::Rotate180 => Some(2),
            _ => Some(3),
        };

        commands.push(self.exec_randr(output, Randr::Transform(transform)));

        Command::batch(commands)
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
        let mut commands = Vec::with_capacity(2);

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

        let request = Randr::Resolution(resolution.0, resolution.1);
        let mut revert_request = request;
        if let Some(resolution) = self.config.resolution {
            revert_request = Randr::Resolution(resolution.0, resolution.1);
        }

        self.config.refresh_rate = Some(rate);
        self.config.resolution = Some(resolution);
        self.cache.refresh_rate_selected = Some(0);
        self.cache.resolution_selected = Some(option);
        commands.push(self.exec_randr(output, Randr::Resolution(resolution.0, resolution.1)));
        commands.push(self.set_dialog(revert_request, &request));

        Command::batch(commands)
    }

    /// Set the scale of the active display.
    pub fn set_scale(&mut self, option: usize) -> Command<app::Message> {
        let mut commands = Vec::with_capacity(2);

        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Command::none();
        };

        let scale = (option * 25 + 50) as u32;

        let request = Randr::Scale(scale);
        let revert_request = Randr::Scale(self.config.scale);

        self.cache.scale_selected = Some(option);
        self.config.scale = scale;
        commands.push(self.exec_randr(output, Randr::Scale(scale)));
        commands.push(self.set_dialog(revert_request, &request));
        Command::batch(commands)
    }

    /// Enables or disables the active display.
    pub fn toggle_display(&mut self, enable: bool) -> Command<app::Message> {
        let mut commands = Vec::with_capacity(2);
        let request = Randr::Toggle(enable);

        let Some(output) = self.list.outputs.get_mut(self.active_display) else {
            return Command::none();
        };

        let revert_request = Randr::Toggle(output.enabled);
        let current_request = request;

        output.enabled = enable;

        let output = &self.list.outputs[self.active_display];
        commands.push(self.exec_randr(output, request));
        commands.push(self.set_dialog(revert_request, &current_request));
        Command::batch(commands)
    }

    /// Applies a display configuration via `cosmic-randr`.
    fn exec_randr(&self, output: &Output, request: Randr) -> Command<app::Message> {
        let mut commands = Vec::with_capacity(2);

        // Removes the dialog if no change is being made
        if Some(request) == self.dialog {
            commands.push(command::message(app::Message::from(
                Message::DialogComplete,
            )));
        }

        let name = &*output.name;
        let mut command = tokio::process::Command::new("cosmic-randr");

        match request {
            Randr::Mirror(from_id) => {
                let Some(from_output) = self.list.outputs.get(from_id) else {
                    return Command::none();
                };

                command
                    .arg("mirror")
                    .arg(&output.name)
                    .arg(&from_output.name);
            }

            Randr::Position(x, y) => {
                let Some(current) = output.current.and_then(|id| self.list.modes.get(id)) else {
                    return Command::none();
                };

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
                let Some(current) = output.current.and_then(|id| self.list.modes.get(id)) else {
                    return Command::none();
                };

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
                let Some(current) = output.current.and_then(|id| self.list.modes.get(id)) else {
                    return Command::none();
                };

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
                let Some(current) = output.current.and_then(|id| self.list.modes.get(id)) else {
                    return Command::none();
                };

                command
                    .arg("mode")
                    .arg("--transform")
                    .arg(&*format!("{transform}"))
                    .arg(name)
                    .arg(itoa::Buffer::new().format(current.size.0))
                    .arg(itoa::Buffer::new().format(current.size.1));
            }
        }

        commands.push(cosmic::command::future(async move {
            tracing::debug!(?command, "executing");
            app::Message::from(Message::RandrResult(Arc::new(command.status().await)))
        }));
        Command::batch(commands)
    }
}

/// View for the display arrangement section.
pub fn display_arrangement() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    _ = descriptions.insert(fl!("display", "arrangement"));
    let display_arrangement_desc = descriptions.insert(fl!("display", "arrangement-desc"));

    Section::default()
        .title(fl!("display", "arrangement"))
        .descriptions(descriptions)
        // Show section when there is more than 1 display
        .show_while::<Page>(|page| page.list.outputs.len() > 1)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let theme = cosmic::theme::active();

            column()
                .padding(cosmic::iced::Padding::from([
                    theme.cosmic().space_s(),
                    theme.cosmic().space_m(),
                ]))
                .spacing(theme.cosmic().space_xs())
                .push(widget::text::body(&descriptions[display_arrangement_desc]))
                .push({
                    Arrangement::new(&page.list, &page.display_tabs)
                        .on_select(|id| pages::Message::Displays(Message::Display(id)))
                        .on_placement(|id, x, y| {
                            pages::Message::Displays(Message::Position(id, x, y))
                        })
                        .apply(widget::scrollable)
                        .id(page.display_arrangement_scrollable.clone())
                        .width(Length::Shrink)
                        .direction(Direction::Horizontal(Properties::new()))
                        .apply(container)
                        .center_x()
                        .width(Length::Fill)
                })
                .apply(widget::list::container)
                .into()
        })
}

/// View for the display configuration section.
pub fn display_configuration() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let _display = descriptions.insert(fl!("display"));
    let refresh_rate = descriptions.insert(fl!("display", "refresh-rate"));
    let resolution = descriptions.insert(fl!("display", "resolution"));
    let scale = descriptions.insert(fl!("display", "scale"));
    let orientation = descriptions.insert(fl!("orientation"));
    let enable_label = descriptions.insert(fl!("display", "enable"));
    let options_label = descriptions.insert(fl!("display", "options"));
    let mirroring_label = descriptions.insert(fl!("mirroring"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let theme = cosmic::theme::active();

            let Some(&active_id) = page.display_tabs.active_data::<OutputKey>() else {
                return column().into();
            };

            let active_output = &page.list.outputs[active_id];

            let display_options = (page.show_display_options && active_output.enabled).then(|| {
                list_column()
                    .add(widget::settings::item(
                        &descriptions[resolution],
                        dropdown(
                            &page.cache.resolutions,
                            page.cache.resolution_selected,
                            Message::Resolution,
                        ),
                    ))
                    .add(widget::settings::item(
                        &descriptions[refresh_rate],
                        dropdown(
                            &page.cache.refresh_rates,
                            page.cache.refresh_rate_selected,
                            Message::RefreshRate,
                        ),
                    ))
                    .add(widget::settings::item(
                        &descriptions[scale],
                        dropdown(
                            &["50%", "75%", "100%", "125%", "150%", "175%", "200%"],
                            page.cache.scale_selected,
                            Message::Scale,
                        ),
                    ))
                    .add(widget::settings::item(
                        &descriptions[orientation],
                        dropdown(
                            &page.cache.orientations,
                            page.cache.orientation_selected,
                            |id| {
                                Message::Orientation(match id {
                                    0 => Transform::Normal,
                                    1 => Transform::Rotate90,
                                    2 => Transform::Rotate180,
                                    _ => Transform::Rotate270,
                                })
                            },
                        ),
                    ))
            });

            let mut content = column().spacing(theme.cosmic().space_m());

            if page.list.outputs.len() > 1 {
                let display_switcher = tab_bar::horizontal(&page.display_tabs)
                    .button_alignment(Alignment::Center)
                    .on_activate(Message::Display);

                let display_enable = (page
                    // Don't allow disabling display if it's the only active
                    .list
                    .outputs
                    .values()
                    .filter(|display| display.enabled)
                    .count()
                    > 1
                    || !active_output.enabled)
                    .then(|| {
                        list_column()
                            .add(widget::settings::item(
                                &descriptions[enable_label],
                                toggler(None, active_output.enabled, Message::DisplayToggle),
                            ))
                            .add(widget::settings::item(
                                &descriptions[mirroring_label],
                                widget::dropdown::multi::dropdown(
                                    &page.mirror_menu,
                                    Message::Mirroring,
                                ),
                            ))
                    });

                content = content.push(display_switcher).push_maybe(display_enable);
            }

            content
                .push(widget::text::heading(&descriptions[options_label]))
                .push_maybe(display_options)
                .apply(Element::from)
                .map(pages::Message::Displays)
        })
}

fn cache_rates(cached_rates: &mut Vec<String>, rates: &[u32]) {
    *cached_rates = rates
        .iter()
        .map(|&rate| format!("{:>3}.{:02} Hz", rate / 1000, rate % 1000))
        .collect();
}

pub async fn on_enter() -> crate::pages::Message {
    let randr_fut = cosmic_randr_shell::list();

    crate::pages::Message::Displays(Message::Update {
        randr: Arc::new(randr_fut.await),
    })
}
