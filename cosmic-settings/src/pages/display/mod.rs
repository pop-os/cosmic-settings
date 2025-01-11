// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod arrangement;
// pub mod night_light;

use crate::{app, pages};
use arrangement::Arrangement;
use cosmic::iced::{time, Alignment, Length};
use cosmic::iced_widget::scrollable::{Direction, RelativeOffset, Scrollbar};
use cosmic::widget::{
    self, column, container, dropdown, list_column, segmented_button, tab_bar, text, toggler,
};
use cosmic::{Apply, Element, Task};
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_randr_shell::{
    AdaptiveSyncAvailability, AdaptiveSyncState, List, Output, OutputKey, Transform,
};
use cosmic_settings_page::{self as page, section, Section};
use once_cell::sync::Lazy;
use slab::Slab;
use slotmap::{Key, SecondaryMap, SlotMap};
use std::{collections::BTreeMap, process::ExitStatus, sync::Arc};
use tokio::sync::oneshot;
use tracing::error;

static DPI_SCALES: &[u32] = &[50, 75, 100, 125, 150, 175, 200, 225, 250, 275, 300];
static DPI_SCALE_LABELS: Lazy<Vec<String>> =
    Lazy::new(|| DPI_SCALES.iter().map(|scale| format!("{scale}%")).collect());

/// Display color depth options
#[allow(dead_code)]
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
    // NightLight(NightLight),
    /// Show the night light mode context drawer.
    // NightLightContext,
    /// Set the orientation of a display.
    Orientation(Transform),
    /// Pan the displays view
    Pan(arrangement::Pan),
    /// Status of an applied display change.
    RandrResult(Arc<std::io::Result<ExitStatus>>),
    /// Request to reload the page.
    Refresh,
    /// Set the refresh rate of a display.
    RefreshRate(usize),
    /// Set the VRR mode of a display.
    VariableRefreshRate(usize),
    /// Set the resolution of a display.
    Resolution(usize),
    /// Set the preferred scale for a display.
    Scale(usize),
    /// Refreshes display outputs.
    Update {
        /// Available outputs from cosmic-randr.
        randr: Arc<Result<List, cosmic_randr_shell::Error>>,
    },
    SetXwaylandDescaling(bool),
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
    VariableRefreshRate(AdaptiveSyncState),
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
    background_service_cancel: Option<oneshot::Sender<()>>,
    config: Config,
    cache: ViewCache,
    // context: Option<ContextDrawer>,
    display_arrangement_scrollable: widget::Id,
    /// Tracks the last pan status.
    last_pan: f32,
    /// The setting to revert to if the next dialog page is cancelled.
    dialog: Option<Randr>,
    /// the instant the setting was changed.
    dialog_countdown: usize,
    show_display_options: bool,
    comp_config: cosmic_config::Config,
    comp_config_descale_xwayland: bool,
}

impl Default for Page {
    fn default() -> Self {
        let comp_config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let comp_config_descale_xwayland =
            comp_config.get("descale_xwayland").unwrap_or_else(|err| {
                if !matches!(err, cosmic_config::Error::NoConfigDirectory) {
                    error!(?err, "Failed to read config 'descale_xwayland'");
                }

                false
            });

        Self {
            list: List::default(),
            display_tabs: segmented_button::SingleSelectModel::default(),
            mirror_map: SecondaryMap::new(),
            mirror_menu: widget::dropdown::multi::model(),
            active_display: OutputKey::default(),
            background_service_cancel: None,
            config: Config::default(),
            cache: ViewCache::default(),
            // context: None,
            display_arrangement_scrollable: widget::Id::unique(),
            last_pan: 0.5,
            dialog: None,
            dialog_countdown: 0,
            show_display_options: true,
            comp_config,
            comp_config_descale_xwayland,
        }
    }
}

#[derive(Default)]
struct Config {
    /// Whether night light is enabled.
    // night_light_enabled: bool,
    refresh_rate: Option<u32>,
    vrr: Option<AdaptiveSyncState>,
    resolution: Option<(u32, u32)>,
    scale: u32,
}

/// Cached view content for widgets.
#[derive(Default)]
struct ViewCache {
    modes: BTreeMap<(u32, u32), Vec<u32>>,
    orientations: [String; 4],
    vrr_modes: Vec<String>,
    refresh_rates: Vec<String>,
    resolutions: Vec<String>,
    orientation_selected: Option<usize>,
    refresh_rate_selected: Option<usize>,
    vrr_selected: Option<usize>,
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
            // sections.insert(
            //     Section::default()
            //         .descriptions(vec![
            //             text::NIGHT_LIGHT.as_str().into(),
            //             text::NIGHT_LIGHT_AUTO.as_str().into(),
            //             text::NIGHT_LIGHT_DESCRIPTION.as_str().into(),
            //         ])
            //         .view::<Page>(move |_binder, page, _section| page.night_light_view()),
            // ),
            // Display arrangement
            sections.insert(display_arrangement()),
            // Display configuration
            sections.insert(display_configuration()),
            // Xwayland scaling options
            sections.insert(legacy_applications()),
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
        sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> Task<crate::pages::Message> {
        use std::time::Duration;

        use futures::pin_mut;

        if let Some(canceller) = self.background_service_cancel.take() {
            _ = canceller.send(());
        }

        #[cfg(feature = "wayland")]
        {
            let (tx, mut rx) = tachyonix::channel(4);
            let (canceller, cancelled) = oneshot::channel();
            let runtime = tokio::runtime::Handle::current();

            // Spawns a background service to monitor for display state changes.
            // This must be spawned onto its own thread because `*mut wayland_sys::client::wl_display` is not Send-able.
            tokio::task::spawn_blocking(move || {
                let dispatcher = async move {
                    let Ok((mut context, mut event_queue)) = cosmic_randr::connect(tx) else {
                        return;
                    };

                    loop {
                        if context.dispatch(&mut event_queue).await.is_err() {
                            return;
                        }
                    }
                };

                pin_mut!(dispatcher);
                runtime.block_on(futures::future::select(cancelled, dispatcher));
            });

            // Forward messages from another thread to prevent the monitoring thread from blocking.
            tokio::task::spawn(async move {
                while let Ok(message) = rx.recv().await {
                    if sender.is_closed() {
                        return;
                    }

                    if let cosmic_randr::Message::ManagerDone = message {
                        if matches!(
                            tokio::time::timeout(
                                Duration::from_secs(1),
                                sender.send(pages::Message::Displays(Message::Refresh))
                            )
                            .await,
                            Err(_) | Ok(Err(_))
                        ) {
                            return;
                        }
                    }
                }
            });

            self.background_service_cancel = Some(canceller);
        }

        cosmic::task::future(on_enter())
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some(canceller) = self.background_service_cancel.take() {
            _ = canceller.send(());
        }

        Task::none()
    }

    #[cfg(feature = "test")]
    fn on_enter(
        &mut self,
        _sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> Task<crate::pages::Message> {
        cosmic::task::future(async move {
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
                adaptive_sync: None,
                adaptive_sync_availability: None,
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
                adaptive_sync: Some(AdaptiveSyncState::Disabled),
                adaptive_sync_availability: Some(AdaptiveSyncAvailability::Supported),
            });

            crate::pages::Message::Displays(Message::Update {
                randr: Arc::new(Ok(randr)),
            })
        })
    }

    // fn context_drawer(&self) -> Option<Element<pages::Message>> {
    //     Some(match self.context {

    //         Some(ContextDrawer::NightLight) => self.night_light_context_view(),

    //         None => return None,
    //     })
    // }

    /// Opens a dialog to confirm the display settings.
    ///
    /// This dialog has a 10 (arbitrary) second counter which will
    /// automatically revert to the original display settings when depleted.
    ///
    /// To make a setting activate this dialog. Call the `set_dialog` method with
    /// the Randr enum value which undos the current change. Makde sure the
    /// return value is returned with the `exec_value` return value within a batch
    /// Task.
    fn dialog(&self) -> Option<Element<pages::Message>> {
        self.dialog?;
        let element = widget::dialog()
            .title(fl!("dialog", "title"))
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
    pub fn update(&mut self, message: Message) -> Task<app::Message> {
        match message {
            Message::RandrResult(result) => {
                if let Some(Err(why)) = Arc::into_inner(result) {
                    tracing::error!(?why, "cosmic-randr error");
                } else {
                    // Reload display info
                    return cosmic::task::future(async move {
                        crate::Message::PageMessage(on_enter().await)
                    });
                }
            }

            Message::DialogCancel => {
                let Some(request) = self.dialog else {
                    return Task::none();
                };
                let Some(output) = self.list.outputs.get(self.active_display) else {
                    return Task::none();
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
                        return cosmic::task::message(app::Message::from(Message::DialogCancel));
                    }
                } else {
                    self.dialog_countdown -= 1;
                    return cosmic::task::future(async move {
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
                        return Task::none();
                    };

                    return self.exec_randr(output, Randr::Mirror(from_display));
                }

                Mirroring::Project(to_display) => {
                    let Some(output) = self.list.outputs.get(to_display) else {
                        return Task::none();
                    };

                    return self.exec_randr(output, Randr::Mirror(self.active_display));
                } // Mirroring::ProjectToAll => (),
            },

            // Message::NightLight(night_light) => {}
            //
            // Message::NightLightContext => {
            //     self.context = Some(ContextDrawer::NightLight);
            //     return cosmic::task::message(app::Message::OpenContextDrawer(
            //         text::NIGHT_LIGHT.clone().into(),
            //     ));
            // }
            Message::Orientation(orientation) => return self.set_orientation(orientation),

            Message::Pan(pan) => {
                match pan {
                    arrangement::Pan::Left => self.last_pan = 0.0f32.max(self.last_pan - 0.01),
                    arrangement::Pan::Right => self.last_pan = 1.0f32.min(self.last_pan + 0.01),
                }

                return cosmic::iced::widget::scrollable::snap_to(
                    self.display_arrangement_scrollable.clone(),
                    RelativeOffset {
                        x: self.last_pan,
                        y: 0.0,
                    },
                );
            }

            Message::Position(display, x, y) => return self.set_position(display, x, y),

            Message::Refresh => {
                return cosmic::task::future(async move {
                    crate::Message::PageMessage(on_enter().await)
                });
            }

            Message::RefreshRate(rate) => return self.set_refresh_rate(rate),

            Message::VariableRefreshRate(mode) => return self.set_vrr(mode),

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

            Message::SetXwaylandDescaling(descale) => {
                self.comp_config_descale_xwayland = descale;
                if let Err(err) = self
                    .comp_config
                    .set("descale_xwayland", self.comp_config_descale_xwayland)
                {
                    error!(?err, "Failed to set config 'descale_xwayland'");
                }
            }
        }

        self.last_pan = 0.5;
        cosmic::iced::widget::scrollable::snap_to(
            self.display_arrangement_scrollable.clone(),
            RelativeOffset { x: 0.5, y: 0.5 },
        )
    }

    /// Displays the night light context drawer.
    // pub fn night_light_context_view(&self) -> Element<pages::Message> {
    //     column().into()
    // }

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
    fn set_dialog(&mut self, revert_request: Randr, current_request: &Randr) -> Task<app::Message> {
        if revert_request == *current_request {
            return Task::none();
        }
        self.dialog = Some(revert_request);
        self.dialog_countdown = 10;
        cosmic::task::future(async {
            tokio::time::sleep(time::Duration::from_secs(1)).await;
            app::Message::from(Message::DialogCountdown)
        })
    }

    /// Changes the color depth of the active display.
    pub fn set_color_depth(&mut self, _depth: ColorDepth) -> Task<app::Message> {
        unimplemented!()
    }

    /// Changes the color profile of the active display.
    pub fn set_color_profile(&mut self, _profile: usize) -> Task<app::Message> {
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
        self.config.vrr = output.adaptive_sync;
        self.config.scale = (output.scale * 100.0) as u32;

        self.cache.modes.clear();
        self.cache.refresh_rates.clear();
        self.cache.vrr_modes.clear();
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
        self.cache.vrr_selected = None;

        self.cache.scale_selected = Some(
            DPI_SCALES
                .iter()
                .position(|scale| self.config.scale <= *scale)
                .unwrap_or(DPI_SCALES.len() - 1),
        );

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

        if let Some(state) = output.adaptive_sync {
            match output.adaptive_sync_availability {
                Some(AdaptiveSyncAvailability::Supported) => {
                    self.cache.vrr_modes = vec![
                        fl!("vrr", "force"),
                        fl!("vrr", "auto"),
                        fl!("vrr", "disabled"),
                    ];
                    self.cache.vrr_selected = match state {
                        AdaptiveSyncState::Always => Some(0),
                        AdaptiveSyncState::Auto => Some(1),
                        AdaptiveSyncState::Disabled => Some(2),
                    };
                }
                Some(AdaptiveSyncAvailability::RequiresModeset) => {
                    self.cache.vrr_modes = vec![fl!("vrr", "enabled"), fl!("vrr", "disabled")];
                    self.cache.vrr_selected = match state {
                        AdaptiveSyncState::Always | AdaptiveSyncState::Auto => Some(0),
                        AdaptiveSyncState::Disabled => Some(1),
                    };
                }
                _ => {}
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
    pub fn set_orientation(&mut self, transform: Transform) -> Task<app::Message> {
        let request = Randr::Transform(transform);

        let mut tasks = Vec::with_capacity(2);
        tasks.push(match self.cache.orientation_selected {
            Some(orientation) => self.set_dialog(
                Randr::Transform(match orientation {
                    1 => Transform::Rotate90,
                    2 => Transform::Flipped180,
                    3 => Transform::Flipped270,
                    _ => Transform::Normal,
                }),
                &request,
            ),
            None => Task::none(),
        });

        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Task::none();
        };

        self.cache.orientation_selected = match transform {
            Transform::Normal => Some(0),
            Transform::Rotate90 => Some(1),
            Transform::Rotate180 => Some(2),
            _ => Some(3),
        };

        tasks.push(self.exec_randr(output, Randr::Transform(transform)));

        Task::batch(tasks)
    }

    /// Changes the position of the display.
    pub fn set_position(&mut self, display: OutputKey, x: i32, y: i32) -> Task<app::Message> {
        let Some(output) = self.list.outputs.get_mut(display) else {
            return Task::none();
        };

        output.position = (x, y);

        if cfg!(feature = "test") {
            tracing::debug!("set position {x},{y}");
            return Task::none();
        }

        let output = &self.list.outputs[display];
        self.exec_randr(output, Randr::Position(x, y))
    }

    /// Changes the refresh rate of the active display.
    pub fn set_refresh_rate(&mut self, option: usize) -> Task<app::Message> {
        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Task::none();
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

        Task::none()
    }

    /// Changes the variable refresh rate mode of the active display.
    pub fn set_vrr(&mut self, option: usize) -> Task<app::Message> {
        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Task::none();
        };

        let mode = match output.adaptive_sync_availability {
            Some(AdaptiveSyncAvailability::Supported) => match option {
                0 => AdaptiveSyncState::Always,
                1 => AdaptiveSyncState::Auto,
                2 => AdaptiveSyncState::Disabled,
                _ => return Task::none(),
            },
            Some(AdaptiveSyncAvailability::RequiresModeset) => match option {
                0 => AdaptiveSyncState::Always,
                1 => AdaptiveSyncState::Disabled,
                _ => return Task::none(),
            },
            _ => return Task::none(),
        };

        self.cache.vrr_selected = Some(option);
        self.config.vrr = Some(mode);
        self.exec_randr(output, Randr::VariableRefreshRate(mode))
    }

    /// Change the resolution of the active display.
    pub fn set_resolution(&mut self, option: usize) -> Task<app::Message> {
        let mut tasks = Vec::with_capacity(2);

        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Task::none();
        };

        let Some((&resolution, rates)) = self.cache.modes.iter().rev().nth(option) else {
            return Task::none();
        };

        self.cache.refresh_rates.clear();
        cache_rates(&mut self.cache.refresh_rates, rates);

        let Some(&rate) = rates.first() else {
            return Task::none();
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
        tasks.push(self.exec_randr(output, Randr::Resolution(resolution.0, resolution.1)));
        tasks.push(self.set_dialog(revert_request, &request));

        Task::batch(tasks)
    }

    /// Set the scale of the active display.
    pub fn set_scale(&mut self, option: usize) -> Task<app::Message> {
        let mut tasks = Vec::with_capacity(2);

        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Task::none();
        };

        let scale = (option * 25 + 50) as u32;

        let request = Randr::Scale(scale);
        let revert_request = Randr::Scale(self.config.scale);

        self.cache.scale_selected = Some(option);
        self.config.scale = scale;
        tasks.push(self.exec_randr(output, Randr::Scale(scale)));
        tasks.push(self.set_dialog(revert_request, &request));
        Task::batch(tasks)
    }

    /// Enables or disables the active display.
    pub fn toggle_display(&mut self, enable: bool) -> Task<app::Message> {
        let mut tasks = Vec::with_capacity(2);
        let request = Randr::Toggle(enable);

        let Some(output) = self.list.outputs.get_mut(self.active_display) else {
            return Task::none();
        };

        let revert_request = Randr::Toggle(output.enabled);
        let current_request = request;

        output.enabled = enable;

        let output = &self.list.outputs[self.active_display];
        tasks.push(self.exec_randr(output, request));
        tasks.push(self.set_dialog(revert_request, &current_request));
        Task::batch(tasks)
    }

    /// Applies a display configuration via `cosmic-randr`.
    fn exec_randr(&self, output: &Output, request: Randr) -> Task<app::Message> {
        let mut tasks = Vec::with_capacity(2);

        // Removes the dialog if no change is being made
        if Some(request) == self.dialog {
            tasks.push(cosmic::task::message(app::Message::from(
                Message::DialogComplete,
            )));
        }

        let name = &*output.name;
        let mut task = tokio::process::Command::new("cosmic-randr");

        match request {
            Randr::Mirror(from_id) => {
                let Some(from_output) = self.list.outputs.get(from_id) else {
                    return Task::none();
                };

                task.arg("mirror").arg(&output.name).arg(&from_output.name);
            }

            Randr::Position(x, y) => {
                let Some(current) = output.current.and_then(|id| self.list.modes.get(id)) else {
                    return Task::none();
                };

                task.arg("mode")
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
                    return Task::none();
                };

                task.arg("mode")
                    .arg("--refresh")
                    .arg(
                        [
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

            Randr::VariableRefreshRate(mode) => {
                let Some(current) = output.current.and_then(|id| self.list.modes.get(id)) else {
                    return Task::none();
                };

                task.arg("mode")
                    .arg("--adaptive-sync")
                    .arg(format!("{}", mode))
                    .arg(name)
                    .arg(itoa::Buffer::new().format(current.size.0))
                    .arg(itoa::Buffer::new().format(current.size.1));
            }

            Randr::Resolution(width, height) => {
                task.arg("mode")
                    .arg(name)
                    .arg(itoa::Buffer::new().format(width))
                    .arg(itoa::Buffer::new().format(height));
            }

            Randr::Scale(scale) => {
                let Some(current) = output.current.and_then(|id| self.list.modes.get(id)) else {
                    return Task::none();
                };

                task.arg("mode")
                    .arg("--scale")
                    .arg(
                        [
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
                task.arg(if enable { "enable" } else { "disable" })
                    .arg(name);
            }

            Randr::Transform(transform) => {
                let Some(current) = output.current.and_then(|id| self.list.modes.get(id)) else {
                    return Task::none();
                };

                task.arg("mode")
                    .arg("--transform")
                    .arg(&*format!("{transform}"))
                    .arg(name)
                    .arg(itoa::Buffer::new().format(current.size.0))
                    .arg(itoa::Buffer::new().format(current.size.1));
            }
        }

        tasks.push(cosmic::task::future(async move {
            tracing::debug!(?task, "executing");
            app::Message::from(Message::RandrResult(Arc::new(task.status().await)))
        }));
        Task::batch(tasks)
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
            let cosmic::cosmic_theme::Spacing {
                space_xxs, space_m, ..
            } = cosmic::theme::active().cosmic().spacing;

            column()
                .push(
                    text::body(&descriptions[display_arrangement_desc])
                        .apply(container)
                        .padding([space_xxs, space_m]),
                )
                .push({
                    Arrangement::new(&page.list, &page.display_tabs)
                        .on_select(|id| pages::Message::Displays(Message::Display(id)))
                        .on_pan(|pan| pages::Message::Displays(Message::Pan(pan)))
                        .on_placement(|id, x, y| {
                            pages::Message::Displays(Message::Position(id, x, y))
                        })
                        .apply(widget::scrollable)
                        .id(page.display_arrangement_scrollable.clone())
                        .width(Length::Shrink)
                        .direction(Direction::Horizontal(Scrollbar::new()))
                        .apply(container)
                        .center_x(Length::Fill)
                })
                .apply(container)
                .class(cosmic::theme::Container::List)
                .width(Length::Fill)
                .into()
        })
}

/// View for the display configuration section.
pub fn display_configuration() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let _display = descriptions.insert(fl!("display"));
    let refresh_rate = descriptions.insert(fl!("display", "refresh-rate"));
    let vrr = descriptions.insert(fl!("vrr"));
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
                let mut items = vec![
                    widget::settings::item(
                        &descriptions[resolution],
                        dropdown(
                            &page.cache.resolutions,
                            page.cache.resolution_selected,
                            Message::Resolution,
                        ),
                    ),
                    widget::settings::item(
                        &descriptions[refresh_rate],
                        dropdown(
                            &page.cache.refresh_rates,
                            page.cache.refresh_rate_selected,
                            Message::RefreshRate,
                        ),
                    ),
                ];

                if let Some(vrr_selected) = page.cache.vrr_selected {
                    items.push(widget::settings::item(
                        &descriptions[vrr],
                        dropdown(
                            &page.cache.vrr_modes,
                            Some(vrr_selected),
                            Message::VariableRefreshRate,
                        ),
                    ));
                }

                items.extend(vec![
                    widget::settings::item(
                        &descriptions[scale],
                        dropdown(&DPI_SCALE_LABELS, page.cache.scale_selected, Message::Scale),
                    ),
                    widget::settings::item(
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
                    ),
                ]);

                items
            });

            let mut content = column().spacing(theme.cosmic().space_xs());

            if page.list.outputs.len() > 1 {
                let display_switcher = tab_bar::horizontal(&page.display_tabs)
                    .button_alignment(Alignment::Center)
                    .on_activate(Message::Display);

                let mut display_enable = (page
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
                                toggler(active_output.enabled).on_toggle(Message::DisplayToggle),
                            ))
                            .add(widget::settings::item(
                                &descriptions[mirroring_label],
                                widget::dropdown::multi::dropdown(
                                    &page.mirror_menu,
                                    Message::Mirroring,
                                ),
                            ))
                    })
                    .unwrap_or_else(list_column);

                if let Some(items) = display_options {
                    for item in items {
                        display_enable = display_enable.add(item);
                    }
                }

                content = content.push(display_switcher).push(display_enable);
            } else {
                content = content
                    .push(text::heading(&descriptions[options_label]))
                    .push_maybe(display_options.map(|items| {
                        let mut column = list_column();
                        for item in items {
                            column = column.add(item);
                        }
                        column
                    }));
            }

            content.apply(Element::from).map(pages::Message::Displays)
        })
}

pub fn legacy_applications() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let system = descriptions.insert(fl!("legacy-applications", "scaled-by-system"));
    let system_desc = descriptions.insert(fl!("legacy-applications", "system-description"));
    let native = descriptions.insert(fl!("legacy-applications", "scaled-natively"));
    let native_desc = descriptions.insert(fl!("legacy-applications", "native-description"));

    Section::default()
        .title(fl!("legacy-applications"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            widget::settings::section()
                .title(&section.title)
                .add(widget::settings::item_row(vec![widget::radio(
                    widget::column()
                        .push(text::body(&descriptions[system]))
                        .push(text::caption(&descriptions[system_desc])),
                    false,
                    Some(page.comp_config_descale_xwayland),
                    Message::SetXwaylandDescaling,
                )
                .width(Length::Fill)
                .into()]))
                .add(widget::settings::item_row(vec![widget::radio(
                    widget::column()
                        .push(text::body(&descriptions[native]))
                        .push(text::caption(&descriptions[native_desc])),
                    true,
                    Some(page.comp_config_descale_xwayland),
                    Message::SetXwaylandDescaling,
                )
                .width(Length::Fill)
                .into()]))
                .apply(Element::from)
                .map(crate::pages::Message::Displays)
        })
}

fn cache_rates(cached_rates: &mut Vec<String>, rates: &[u32]) {
    cached_rates.clear();

    #[inline]
    fn round(rate: &u32) -> u32 {
        (*rate as f32 / 1000.0).round() as u32
    }

    #[inline]
    fn format_dec(rate: &u32) -> String {
        format!("{:>3}.{:02} Hz", rate / 1000, rate % 1000)
    }

    for (i, rate) in rates.iter().enumerate() {
        let prev = match i.checked_sub(1) {
            Some(i) => rates.get(i),
            None => None,
        };

        match (prev, rates.get(i + 1)) {
            (None, None) => cached_rates.push(format!("{} Hz", round(rate))),
            (None, Some(next)) => {
                if round(rate) == round(next) {
                    cached_rates.push(format_dec(rate))
                } else {
                    cached_rates.push(format!("{} Hz", round(rate)))
                }
            }
            (Some(prev), None) => {
                if round(rate) == round(prev) {
                    cached_rates.push(format_dec(rate))
                } else {
                    cached_rates.push(format!("{} Hz", round(rate)))
                }
            }
            (Some(prev), Some(next)) => {
                if round(rate) == round(prev) || round(rate) == round(next) {
                    cached_rates.push(format_dec(rate))
                } else {
                    cached_rates.push(format!("{} Hz", round(rate)))
                }
            }
        }
    }
}

pub async fn on_enter() -> crate::pages::Message {
    let randr_fut = cosmic_randr_shell::list();

    crate::pages::Message::Displays(Message::Update {
        randr: Arc::new(randr_fut.await),
    })
}

#[test]
fn test_cache_rates() {
    let rates: &[u32] = &[10000, 11006, 100004, 100005];

    let mut cached = vec![];

    cache_rates(&mut cached, rates);

    assert_eq!(cached, vec!["10 Hz", "11 Hz", "100.04 Hz", "100.05 Hz"])
}
