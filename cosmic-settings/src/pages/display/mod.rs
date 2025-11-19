// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod arrangement;

use crate::{app, pages};
use arrangement::Arrangement;
use cosmic::iced::{Alignment, Length, time};
use cosmic::widget::{
    self, column, container, dropdown, list_column, segmented_button, tab_bar, text, toggler,
};
use cosmic::{Apply, Element, Task, surface};
use cosmic_randr_shell::{
    AdaptiveSyncAvailability, AdaptiveSyncState, List, Output, OutputKey, Transform,
};
use cosmic_settings_page::{self as page, Section, section};
use slab::Slab;
use slotmap::{Key, SecondaryMap, SlotMap};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock};
use std::{collections::BTreeMap, process::ExitStatus};
use tokio::sync::oneshot;

static DPI_SCALES: &[u32] = &[50, 75, 100, 125, 150, 175, 200, 225, 250, 275, 300];

static DPI_SCALE_LABELS: LazyLock<Vec<String>> =
    LazyLock::new(|| DPI_SCALES.iter().map(|scale| format!("{scale}%")).collect());

/// Display color depth options
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct ColorDepth(usize);

/// Display mirroring options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mirroring {
    Disable,
    // ProjectToAll,
    Project(OutputKey),
    Mirror(OutputKey),
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
    /// The dialog was cancelled, and will revert settings.
    DialogCancel,
    /// The dialog was completed.
    DialogComplete,
    /// How long until the dialog automatically cancels, in seconds.
    DialogCountdown,
    /// Toggles display on or off.
    DisplayToggle(bool),
    /// Configures mirroring status of a display.
    Mirroring(Mirroring),
    /// Set the orientation of a display.
    Orientation(Transform),
    /// Status of an applied display change.
    RandrResult(Arc<std::io::Result<ExitStatus>>),
    /// Set the refresh rate of a display.
    RefreshRate(usize),
    /// Set the VRR mode of a display.
    VariableRefreshRate(usize),
    /// Set the resolution of a display.
    Resolution(usize),
    /// Set the preferred scale for a display.
    Scale(usize),
    /// Adjust the display scale.
    AdjustScale(u32),
    /// Refreshes display outputs.
    Update {
        /// Available outputs from cosmic-randr.
        randr: Arc<Result<List, cosmic_randr_shell::Error>>,
    },
    Surface(surface::Action),
    /// Clear the invalid arrangement error if generation matches
    ClearInvalidArrangement(u64),
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
    /// Set when the page is being refreshed
    refreshing_page: Arc<AtomicBool>,
    list: List,
    display_tabs: segmented_button::SingleSelectModel,
    mirror_map: SecondaryMap<OutputKey, OutputKey>,
    mirror_menu: widget::dropdown::multi::Model<String, Mirroring>,
    active_display: OutputKey,
    /// Name of the active display (stable across updates, unlike OutputKey)
    active_display_name: Option<String>,
    randr_handle: Option<(oneshot::Sender<()>, cosmic::iced::task::Handle)>,
    hotplug_handle: Option<(oneshot::Sender<()>, cosmic::iced::task::Handle)>,
    config: Config,
    cache: ViewCache,
    /// The setting to revert to if the next dialog page is cancelled.
    dialog: Option<Randr>,
    /// the instant the setting was changed.
    dialog_countdown: usize,
    show_display_options: bool,
    adjusted_scale: u32,
    /// Set when an invalid display arrangement is attempted.
    /// Automatically cleared after 5 seconds or when a valid arrangement is applied.
    invalid_arrangement: bool,
    /// Generation counter for invalid arrangement errors.
    /// Incremented on each validation failure to prevent race conditions where
    /// an older timeout clears a newer error message.
    invalid_arrangement_generation: u64,
    /// Flag to force first display selection on next update.
    /// Set to true when entering the page, ensures first display is always selected
    /// even when multiple rapid updates occur.
    force_first_display: bool,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            refreshing_page: Arc::new(AtomicBool::new(false)),
            list: List::default(),
            display_tabs: segmented_button::SingleSelectModel::default(),
            mirror_map: SecondaryMap::new(),
            mirror_menu: widget::dropdown::multi::model(),
            active_display: OutputKey::null(),
            active_display_name: None,
            randr_handle: None,
            hotplug_handle: None,
            config: Config::default(),
            cache: ViewCache::default(),
            dialog: None,
            dialog_countdown: 0,
            show_display_options: true,
            adjusted_scale: 0,
            invalid_arrangement: false,
            invalid_arrangement_generation: 0,
            force_first_display: false,
        }
    }
}

#[derive(Default)]
struct Config {
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
    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        use std::time::Duration;

        self.force_first_display = true;
        self.active_display = OutputKey::null();
        self.active_display_name = None;

        self.cache.orientations = [
            fl!("orientation", "standard"),
            fl!("orientation", "rotate-90"),
            fl!("orientation", "rotate-180"),
            fl!("orientation", "rotate-270"),
        ];

        let mut tasks = Vec::with_capacity(3);
        tasks.push(cosmic::task::future(on_enter()));

        if let Some((canceller, handle)) = self.randr_handle.take() {
            _ = canceller.send(());
            handle.abort();
        }

        if let Some((canceller, handle)) = self.hotplug_handle.take() {
            _ = canceller.send(());
            handle.abort();
        }

        self.refreshing_page.store(true, Ordering::SeqCst);

        #[cfg(feature = "wayland")]
        {
            let refreshing_page = self.refreshing_page.clone();
            let (tx, mut rx) = tachyonix::channel(4);
            let (canceller, cancelled) = oneshot::channel::<()>();
            let runtime = tokio::runtime::Handle::current();

            // Spawns a background service to monitor for display state changes.
            // This must be spawned onto its own thread because `*mut wayland_sys::client::wl_display` is not Send-able.
            tokio::task::spawn_blocking(move || {
                let dispatcher = std::pin::pin!(async move {
                    let Ok((mut context, mut event_queue)) = cosmic_randr::connect(tx) else {
                        return;
                    };

                    loop {
                        if context.dispatch(&mut event_queue).await.is_err() {
                            return;
                        }
                    }
                });

                runtime.block_on(futures::future::select(cancelled, dispatcher));
            });

            // Forward messages from another thread to prevent the monitoring thread from blocking.
            let (randr_task, randr_handle) =
                Task::stream(async_fn_stream::fn_stream(|emitter| async move {
                    while let Ok(message) = rx.recv().await {
                        if let cosmic_randr::Message::ManagerDone = message
                            && !refreshing_page.swap(true, Ordering::SeqCst)
                        {
                            _ = emitter.emit(on_enter().await).await;
                        }
                    }
                }))
                .abortable();

            tasks.push(randr_task);
            self.randr_handle = Some((canceller, randr_handle));
        }

        // Channels for communicating messages from the DRM hotplug thread.
        let (hotplug_cancel_tx, hotplug_cancel_rx) = oneshot::channel::<()>();
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);

        // Spawn a background thread for asynchronously polling a udev monitor for DRM
        // hotplug events. As udev is not thread-safe, it needs its own dedicated thread.
        let tokio_handle = tokio::runtime::Handle::current();
        std::thread::spawn(move || {
            tokio_handle.block_on(async move {
                let Ok(builder) = udev::MonitorBuilder::new() else {
                    return;
                };

                let Ok(builder) = builder.match_subsystem("drm") else {
                    return;
                };

                let Ok(socket) = builder.listen() else {
                    return;
                };

                let Ok(mut async_fd) = tokio::io::unix::AsyncFd::new(socket) else {
                    return;
                };

                let emitter = std::pin::pin!(async move {
                    loop {
                        // If any DRM events occur, a message updating the display list will be sent.
                        if let Ok(mut guard) = async_fd.writable().await {
                            guard.clear_ready();

                            let drm_hotplug_occurred = &mut false;
                            async_fd.get_mut().iter().for_each(|_| {
                                *drm_hotplug_occurred = true;
                            });

                            if *drm_hotplug_occurred {
                                _ = tx.send(on_enter().await).await;
                            }
                        }

                        tokio::time::sleep(Duration::from_secs(3)).await;
                    }
                });

                let cancellation = std::pin::pin!(async move {
                    _ = hotplug_cancel_rx.await;
                });

                futures::future::select(cancellation, emitter).await;
            });
        });

        // Forward messages from the DRM hotplug thread.
        let (hotplug_task, hotplug_handle) =
            Task::stream(async_fn_stream::fn_stream(|emitter| async move {
                while let Some(message) = rx.recv().await {
                    _ = emitter.emit(message).await;
                }
            }))
            .abortable();

        tasks.push(hotplug_task);
        self.hotplug_handle = Some((hotplug_cancel_tx, hotplug_handle));

        cosmic::task::batch(tasks)
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some((canceller, handle)) = self.hotplug_handle.take() {
            _ = canceller.send(());
            handle.abort();
        }

        if let Some((canceller, handle)) = self.randr_handle.take() {
            _ = canceller.send(());
            handle.abort();
        }

        Task::none()
    }

    #[cfg(feature = "test")]
    fn on_enter(&mut self) -> Task<crate::pages::Message> {
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
                xwayland_primary: None,
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
                xwayland_primary: None,
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
    fn dialog(&self) -> Option<Element<'_, pages::Message>> {
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
                    tracing::error!(why = why.to_string(), "cosmic-randr error");
                    // Cancel the revert dialog if resolution or refresh rate did not change.
                    // RandR may revert those changes in certain circumstances so showing the
                    // dialog is superfluous and confusing.
                    if let Some(mode) =
                        self.list
                            .outputs
                            .get(self.active_display)
                            .and_then(|display| {
                                display.current.and_then(|key| self.list.modes.get(key))
                            })
                    {
                        tracing::debug!(old = ?self.dialog, new = ?mode, "Mode update");
                        match self.dialog {
                            Some(Randr::Resolution(width, height)) => {
                                if mode.size.0 == width && mode.size.1 == height {
                                    self.config.resolution = Some((width, height));
                                    return self.update(Message::DialogComplete);
                                }
                            }
                            Some(Randr::RefreshRate(rate)) => {
                                if mode.refresh_rate == rate {
                                    self.config.refresh_rate = Some(rate);
                                    return self.update(Message::DialogComplete);
                                }
                            }
                            _ => {}
                        }
                    }
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

            Message::Display(display_entity) => {
                self.set_display(display_entity);
            }

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
                }
            },

            Message::Orientation(orientation) => return self.set_orientation(orientation),

            Message::Position(output_key, x, y) => {
                return self.set_position(output_key, x, y);
            }

            Message::RefreshRate(rate) => return self.set_refresh_rate(rate),

            Message::VariableRefreshRate(mode) => return self.set_vrr(mode),

            Message::Resolution(option) => return self.set_resolution(option),

            Message::Scale(option) => {
                self.adjusted_scale = 0;
                return self.set_scale(option);
            }

            Message::AdjustScale(scale) => {
                if self.adjusted_scale != scale {
                    self.adjusted_scale = scale;

                    if let Some(option) = self.cache.scale_selected {
                        return self.set_scale(option);
                    }
                }
            }

            Message::Update { randr } => {
                match Arc::into_inner(randr) {
                    Some(Ok(outputs)) => {
                        self.update_displays(outputs);
                    }

                    Some(Err(why)) => {
                        tracing::error!(why = why.to_string(), "error fetching displays");
                    }

                    None => {}
                }

                self.refreshing_page.store(false, Ordering::SeqCst);
            }

            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }

            Message::ClearInvalidArrangement(generation) => {
                // Only clear if this is still the current error generation (prevents race conditions)
                if self.invalid_arrangement_generation == generation {
                    self.invalid_arrangement = false;
                    tracing::debug!(
                        "Cleared invalid arrangement error (generation {})",
                        generation
                    );
                }
            }
        }

        Task::none()
    }

    pub fn update_displays(&mut self, list: List) {
        let force_first = self.force_first_display;
        if force_first {
            self.force_first_display = false;
        }

        let previous_active_name = self.active_display_name.clone();

        let mut active_tab_pos: u16 = 0;
        let mut found_previous_active = false;

        self.active_display = OutputKey::null();
        self.active_display_name = None;
        self.display_tabs = Default::default();
        self.mirror_map = SecondaryMap::new();
        self.list = list;

        let sorted_outputs = self
            .list
            .outputs
            .iter()
            .map(|(key, output)| (&*output.name, key))
            .collect::<BTreeMap<_, _>>();

        let mut target_entity = None;

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

            let is_previous_active = previous_active_name
                .as_ref()
                .map(|prev_name| &output.name == prev_name)
                .unwrap_or(false);

            if is_previous_active {
                active_tab_pos = pos as u16;
                found_previous_active = true;
            }

            let entity = self
                .display_tabs
                .insert()
                .text(text)
                .data::<OutputKey>(id)
                .id();

            if is_previous_active {
                target_entity = Some((entity, id, output.name.clone()));
            }
        }

        if force_first {
            self.display_tabs.activate_position(0);
            let active_entity = self.display_tabs.active();
            self.set_display(active_entity);
        } else if !found_previous_active || previous_active_name.is_none() {
            self.display_tabs.activate_position(active_tab_pos);
            let active_entity = self.display_tabs.active();
            self.set_display(active_entity);
        } else if let Some((entity, output_key, display_name)) = target_entity {
            self.display_tabs.activate(entity);
            self.active_display = output_key;
            self.active_display_name = Some(display_name.clone());
        } else {
            self.display_tabs.activate_position(active_tab_pos);
            let active_entity = self.display_tabs.active();
            self.set_display(active_entity);
        }
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

    pub fn set_display(&mut self, display_entity: segmented_button::Entity) {
        let Some(&output_id) = self.display_tabs.data::<OutputKey>(display_entity) else {
            return;
        };

        let Some(output) = self.list.outputs.get_mut(output_id) else {
            return;
        };

        if self.display_tabs.active() != display_entity {
            self.display_tabs.activate(display_entity);
        }
        self.active_display = output_id;
        self.active_display_name = Some(output.name.clone());
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

        let selected_scale = DPI_SCALES
            .iter()
            .position(|scale| self.config.scale <= *scale)
            .unwrap_or(DPI_SCALES.len() - 1);

        self.adjusted_scale = ((self.config.scale % 25).min(20) as f32 / 5.0).round() as u32 * 5;
        self.cache.scale_selected = Some(if self.adjusted_scale != 0 && selected_scale > 0 {
            selected_scale - 1
        } else {
            selected_scale
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

    fn is_landscape(transform: Transform) -> bool {
        matches!(
            transform,
            Transform::Normal | Transform::Rotate180 | Transform::Flipped | Transform::Flipped180
        )
    }

    fn validate_arrangement(&self, output_key: OutputKey, new_x: i32, new_y: i32) -> bool {
        let Some(moving_output) = self.list.outputs.get(output_key) else {
            return false;
        };

        // Only validate enabled displays
        if !moving_output.enabled {
            return true;
        }

        let Some(mode) = moving_output
            .current
            .and_then(|key| self.list.modes.get(key))
        else {
            return false;
        };

        let (moving_width, moving_height) = if moving_output.transform.is_none_or(Self::is_landscape) {
            (mode.size.0, mode.size.1)
        } else {
            (mode.size.1, mode.size.0)
        };

        let moving_width = (moving_width as f64 / moving_output.scale) as u32;
        let moving_height = (moving_height as f64 / moving_output.scale) as u32;

        // Check for overlaps with other displays
        for (other_key, other_output) in &self.list.outputs {
            if other_key == output_key || !other_output.enabled {
                continue;
            }

            let Some(other_mode) = other_output
                .current
                .and_then(|key| self.list.modes.get(key))
            else {
                continue;
            };

            let (other_width, other_height) = if other_output.transform.is_none_or(Self::is_landscape) {
                (other_mode.size.0, other_mode.size.1)
            } else {
                (other_mode.size.1, other_mode.size.0)
            };

            let other_width = (other_width as f64 / other_output.scale) as u32;
            let other_height = (other_height as f64 / other_output.scale) as u32;

            // Check for rectangle overlap
            let x_overlap = new_x < other_output.position.0 + other_width as i32
                && new_x + moving_width as i32 > other_output.position.0;
            let y_overlap = new_y < other_output.position.1 + other_height as i32
                && new_y + moving_height as i32 > other_output.position.1;

            if x_overlap && y_overlap {
                return false;
            }
        }

        self.validate_connectivity(output_key, new_x, new_y)
    }

    /// Check if two display rectangles are adjacent (sharing an edge).
    fn are_displays_adjacent(
        rect1: &(OutputKey, i32, i32, u32, u32),
        rect2: &(OutputKey, i32, i32, u32, u32),
    ) -> bool {
        const EDGE_TOLERANCE: i32 = arrangement::EDGE_TOLERANCE as i32;

        let (_, x1, y1, w1, h1) = *rect1;
        let (_, x2, y2, w2, h2) = *rect2;

        let right1 = x1 + w1 as i32;
        let bottom1 = y1 + h1 as i32;
        let right2 = x2 + w2 as i32;
        let bottom2 = y2 + h2 as i32;

        // Vertical adjacency (left-right touching)
        let left_touches_right = (right1 - x2).abs() <= EDGE_TOLERANCE;
        let right_touches_left = (right2 - x1).abs() <= EDGE_TOLERANCE;
        let vertical_overlap = bottom1 > y2 && y1 < bottom2;

        if (left_touches_right || right_touches_left) && vertical_overlap {
            return true;
        }

        // Horizontal adjacency (top-bottom touching)
        let bottom_touches_top = (bottom1 - y2).abs() <= EDGE_TOLERANCE;
        let top_touches_bottom = (bottom2 - y1).abs() <= EDGE_TOLERANCE;
        let horizontal_overlap = right1 > x2 && x1 < right2;

        (bottom_touches_top || top_touches_bottom) && horizontal_overlap
    }

    fn validate_connectivity(
        &self,
        moved_output_key: OutputKey,
        moved_x: i32,
        moved_y: i32,
    ) -> bool {
        use std::collections::{HashMap, HashSet, VecDeque};

        // Collect all enabled display rectangles
        let display_rects: Vec<_> = self
            .list
            .outputs
            .iter()
            .filter(|(_, output)| output.enabled)
            .filter_map(|(key, output)| {
                let mode = self.list.modes.get(output.current?)?;
                let (width, height) = if output.transform.is_none_or(Self::is_landscape) {
                    (mode.size.0, mode.size.1)
                } else {
                    (mode.size.1, mode.size.0)
                };

                let scaled_width = (width as f64 / output.scale) as u32;
                let scaled_height = (height as f64 / output.scale) as u32;

                let (x, y) = if key == moved_output_key {
                    (moved_x, moved_y)
                } else {
                    output.position
                };

                Some((key, x, y, scaled_width, scaled_height))
            })
            .collect();

        // Single display is always connected
        if display_rects.len() <= 1 {
            return true;
        }

        // Build adjacency graph
        let mut adjacency: HashMap<OutputKey, Vec<OutputKey>> = display_rects
            .iter()
            .map(|(key, ..)| (*key, Vec::new()))
            .collect();

        for i in 0..display_rects.len() {
            for j in (i + 1)..display_rects.len() {
                if Self::are_displays_adjacent(&display_rects[i], &display_rects[j]) {
                    let key_i = display_rects[i].0;
                    let key_j = display_rects[j].0;
                    adjacency.get_mut(&key_i).unwrap().push(key_j);
                    adjacency.get_mut(&key_j).unwrap().push(key_i);
                }
            }
        }

        // BFS to check connectivity
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([display_rects[0].0]);
        visited.insert(display_rects[0].0);

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = adjacency.get(&current) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        let all_connected = visited.len() == display_rects.len();

        if !all_connected {
            tracing::debug!(
                ?moved_output_key,
                moved_x,
                moved_y,
                total_displays = display_rects.len(),
                connected_displays = visited.len(),
                "Display arrangement is disconnected"
            );
        }

        all_connected
    }

    pub fn set_position(&mut self, output_key: OutputKey, x: i32, y: i32) -> Task<app::Message> {
        if !self.list.outputs.contains_key(output_key) {
            return Task::none();
        }

        if !self.validate_arrangement(output_key, x, y) {
            self.invalid_arrangement = true;
            self.invalid_arrangement_generation =
                self.invalid_arrangement_generation.wrapping_add(1);
            let generation = self.invalid_arrangement_generation;

            return cosmic::task::future(async move {
                tokio::time::sleep(cosmic::iced::time::Duration::from_secs(5)).await;
                app::Message::from(Message::ClearInvalidArrangement(generation))
            });
        }

        self.invalid_arrangement = false;

        let Some(output) = self.list.outputs.get_mut(output_key) else {
            return Task::none();
        };

        output.position = (x, y);

        if cfg!(feature = "test") {
            tracing::debug!("set position {x},{y}");
            return Task::none();
        }

        let output = &self.list.outputs[output_key];
        self.exec_randr(output, Randr::Position(x, y))
    }

    /// Changes the refresh rate of the active display.
    pub fn set_refresh_rate(&mut self, option: usize) -> Task<app::Message> {
        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Task::none();
        };

        if let Some(ref resolution) = self.config.resolution
            && let Some(rates) = self.cache.modes.get(resolution)
            && let Some(&rate) = rates.get(option)
        {
            self.cache.refresh_rate_selected = Some(option);
            self.config.refresh_rate = Some(rate);
            return self.exec_randr(output, Randr::RefreshRate(rate));
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
        let Some(output) = self.list.outputs.get(self.active_display) else {
            return Task::none();
        };

        let mut tasks = Vec::with_capacity(2);

        let scale = (option * 25 + 50) as u32 + self.adjusted_scale.min(20);

        self.cache.scale_selected = Some(option);
        self.config.scale = scale;
        tasks.push(self.exec_randr(output, Randr::Scale(scale)));
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
                    .arg(format!("{}.{:03}", rate / 1000, rate % 1000))
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
                    .arg(<&'static str>::from(mode))
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

                let rate = current.refresh_rate;

                task.arg("mode")
                    .arg("--scale")
                    .arg(format!("{}.{:02}", scale / 100, scale % 100))
                    .arg("--refresh")
                    .arg(format!("{}.{:03}", rate / 1000, rate % 1000))
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
            } = cosmic::theme::spacing();

            column()
                .push(
                    // Header row with description on left and error message on right
                    cosmic::iced::widget::row![
                        text::body(&descriptions[display_arrangement_desc]),
                        cosmic::iced::widget::horizontal_space(),
                        if page.invalid_arrangement {
                            text::body(fl!("invalid-arrangement"))
                        } else {
                            text::body("")
                        }
                    ]
                    .align_y(Alignment::Center)
                    .apply(container)
                    .padding([space_xxs, space_m]),
                )
                .push(
                    Arrangement::new(&page.list, &page.display_tabs)
                        .on_placement(|id, x, y| {
                            pages::Message::Displays(Message::Position(id, x, y))
                        })
                        .on_select(|output_key| {
                            // Find the entity that corresponds to this OutputKey
                            let entity = page
                                .display_tabs
                                .iter()
                                .find(|&entity| {
                                    page.display_tabs
                                        .data::<OutputKey>(entity)
                                        .map(|&key| key == output_key)
                                        .unwrap_or(false)
                                })
                                .unwrap_or(page.display_tabs.active());
                            pages::Message::Displays(Message::Display(entity))
                        })
                        .view(),
                )
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
    let additional_scale_options = descriptions.insert(fl!("display", "additional-scale-options"));
    let orientation = descriptions.insert(fl!("orientation"));
    let enable_label = descriptions.insert(fl!("display", "enable"));
    let options_label = descriptions.insert(fl!("display", "options"));
    let mirroring_label = descriptions.insert(fl!("mirroring"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            let Some(&active_id) = page.display_tabs.active_data::<OutputKey>() else {
                return column().into();
            };

            let active_output = &page.list.outputs[active_id];

            let display_options = (page.show_display_options && active_output.enabled).then(|| {
                let mut items = vec![
                    widget::settings::item(
                        &descriptions[resolution],
                        dropdown::popup_dropdown(
                            &page.cache.resolutions,
                            page.cache.resolution_selected,
                            Message::Resolution,
                            cosmic::iced::window::Id::RESERVED,
                            Message::Surface,
                            |a| {
                                crate::app::Message::PageMessage(crate::pages::Message::Displays(a))
                            },
                        ),
                    ),
                    widget::settings::item(
                        &descriptions[refresh_rate],
                        dropdown::popup_dropdown(
                            &page.cache.refresh_rates,
                            page.cache.refresh_rate_selected,
                            Message::RefreshRate,
                            cosmic::iced::window::Id::RESERVED,
                            Message::Surface,
                            |a| {
                                crate::app::Message::PageMessage(crate::pages::Message::Displays(a))
                            },
                        ),
                    ),
                ];

                if let Some(vrr_selected) = page.cache.vrr_selected {
                    items.push(widget::settings::item(
                        &descriptions[vrr],
                        dropdown::popup_dropdown(
                            &page.cache.vrr_modes,
                            Some(vrr_selected),
                            Message::VariableRefreshRate,
                            cosmic::iced::window::Id::RESERVED,
                            Message::Surface,
                            |a| {
                                crate::app::Message::PageMessage(crate::pages::Message::Displays(a))
                            },
                        ),
                    ));
                }

                items.extend(vec![
                    widget::settings::item(
                        &descriptions[scale],
                        dropdown::popup_dropdown(
                            DPI_SCALE_LABELS.as_slice(),
                            page.cache.scale_selected,
                            Message::Scale,
                            cosmic::iced::window::Id::RESERVED,
                            Message::Surface,
                            |a| {
                                crate::app::Message::PageMessage(crate::pages::Message::Displays(a))
                            },
                        ),
                    ),
                    widget::settings::item(
                        &descriptions[additional_scale_options],
                        widget::spin_button(
                            format!("{}%", page.adjusted_scale),
                            page.adjusted_scale,
                            5,
                            0,
                            20,
                            Message::AdjustScale,
                        ),
                    ),
                    widget::settings::item(
                        &descriptions[orientation],
                        dropdown::popup_dropdown(
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
                            cosmic::iced::window::Id::RESERVED,
                            Message::Surface,
                            |a| {
                                crate::app::Message::PageMessage(crate::pages::Message::Displays(a))
                            },
                        ),
                    ),
                ]);

                items
            });

            let mut content = column().spacing(cosmic::theme::spacing().space_xs);

            if page.list.outputs.len() > 1 {
                let display_switcher = tab_bar::horizontal(&page.display_tabs)
                    .button_alignment(Alignment::Center)
                    .on_activate(Message::Display);

                let mut display_enable = if page
                    // Don't allow disabling display if it's the only active
                    .list
                    .outputs
                    .values()
                    .filter(|display| display.enabled)
                    .count()
                    > 1
                    || !active_output.enabled
                {
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
                } else {
                    list_column()
                };

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
