// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    process::ExitStatus,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use cosmic::{
    Apply, Element, Task,
    cosmic_config::{self, ConfigGet, ConfigSet},
    iced::Length,
    surface,
    widget::{self, dropdown, text},
};
use cosmic_comp_config::{EavesdroppingKeyboardMode, XwaylandDescaling, XwaylandEavesdropping};
use cosmic_randr_shell::List;
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use futures::SinkExt;
use slab::Slab;
use slotmap::SlotMap;
use tokio::sync::oneshot;
use tracing::error;

#[derive(Clone, Debug)]
pub enum Message {
    RandrUpdate(Arc<Result<List, cosmic_randr_shell::Error>>),
    RandrResult(Arc<std::io::Result<ExitStatus>>),
    SetXwaylandDescaling(XwaylandDescaling),
    SetXwaylandKeyboardMode(EavesdroppingKeyboardMode),
    SetXwaylandMouseButtonMode(bool),
    SetXwaylandPrimaryOutput(usize),
    Surface(surface::Action),
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::LegacyApplications(message)
    }
}

pub struct Page {
    refresh_pending: Arc<AtomicBool>,
    randr_handle: Option<(oneshot::Sender<()>, cosmic::iced::task::Handle)>,
    output_options: Vec<String>,
    output_options_selected: usize,
    comp_config: cosmic_config::Config,
    comp_config_descale_xwayland: XwaylandDescaling,
    comp_config_xwayland_eavesdropping: XwaylandEavesdropping,
}

impl Default for Page {
    fn default() -> Self {
        let comp_config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let comp_config_descale_xwayland =
            comp_config.get("descale_xwayland").unwrap_or_else(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'descale_xwayland'");
                }

                XwaylandDescaling::Disabled
            });
        let comp_config_xwayland_eavesdropping = comp_config
            .get("xwayland_eavesdropping")
            .unwrap_or_else(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'xwayland_eavesdropping'");
                }

                Default::default()
            });

        let no_display = fl!("legacy-app-scaling", "no-display");
        Self {
            refresh_pending: Arc::new(AtomicBool::new(false)),
            randr_handle: None,
            output_options: vec![no_display],
            output_options_selected: 0,
            comp_config,
            comp_config_descale_xwayland,
            comp_config_xwayland_eavesdropping,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(legacy_application_global_shortcuts()),
            sections.insert(legacy_application_scaling()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new(
            "legacy-applications",
            "preferences-X11-applications-symbolic",
        )
        .title(fl!("legacy-applications"))
        .description(fl!("legacy-applications", "desc"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        let mut tasks = Vec::new();

        tasks.push(cosmic::task::future(on_enter()));

        let refresh_pending = self.refresh_pending.clone();
        let (tx, rx) = cosmic_randr::channel();
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
        let (randr_task, randr_handle) = Task::stream(cosmic::iced_futures::stream::channel(
            1,
            |mut sender| async move {
                while let Some(message) = rx.recv().await {
                    if let cosmic_randr::Message::ManagerDone = message
                        && !refresh_pending.swap(true, Ordering::SeqCst)
                    {
                        _ = sender.send(on_enter().await).await;
                    }
                }
            },
        ))
        .abortable();

        tasks.push(randr_task);
        self.randr_handle = Some((canceller, randr_handle));

        cosmic::task::batch(tasks)
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some((canceller, handle)) = self.randr_handle.take() {
            _ = canceller.send(());
            handle.abort();
        }

        Task::none()
    }
}

pub async fn on_enter() -> crate::pages::Message {
    let randr_fut = cosmic_randr_shell::list();

    crate::pages::Message::LegacyApplications(Message::RandrUpdate(Arc::new(randr_fut.await)))
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::RandrUpdate(randr) => {
                match Arc::into_inner(randr) {
                    Some(Ok(outputs)) => {
                        let output_options_selected = outputs
                            .outputs
                            .values()
                            .position(|o| o.xwayland_primary.is_some_and(std::convert::identity))
                            .map(|x| x + 1)
                            .unwrap_or(0);
                        let mut output_options = vec![fl!("legacy-app-scaling", "no-display")];
                        output_options.extend(
                            outputs
                                .outputs
                                .values()
                                .flat_map(|o| o.xwayland_primary.is_some().then(|| o.name.clone())),
                        );

                        self.output_options_selected = output_options_selected;
                        self.output_options = output_options;
                    }

                    Some(Err(why)) => {
                        tracing::error!(why = why.to_string(), "error fetching displays");
                    }

                    None => (),
                }

                self.refresh_pending.store(false, Ordering::SeqCst);
            }
            Message::RandrResult(result) => {
                if let Some(Err(why)) = Arc::into_inner(result) {
                    tracing::error!(why = why.to_string(), "cosmic-randr error");
                }
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
            Message::SetXwaylandKeyboardMode(mode) => {
                self.comp_config_xwayland_eavesdropping.keyboard = mode;
                if let Err(err) = self.comp_config.set(
                    "xwayland_eavesdropping",
                    self.comp_config_xwayland_eavesdropping,
                ) {
                    error!(?err, "Failed to set config 'xwayland_eavesdropping'");
                }
            }
            Message::SetXwaylandMouseButtonMode(mode) => {
                self.comp_config_xwayland_eavesdropping.pointer = mode;
                if let Err(err) = self.comp_config.set(
                    "xwayland_eavesdropping",
                    self.comp_config_xwayland_eavesdropping,
                ) {
                    error!(?err, "Failed to set config 'xwayland_eavesdropping'");
                }
            }
            Message::SetXwaylandPrimaryOutput(idx) => {
                let mut task = tokio::process::Command::new("cosmic-randr");
                task.arg("xwayland");
                if idx == 0 {
                    task.arg("--no-primary");
                } else {
                    task.arg("--primary").arg(&self.output_options[idx]);
                }

                return cosmic::task::future(async move {
                    tracing::debug!(?task, "executing");
                    crate::app::Message::PageMessage(crate::pages::Message::LegacyApplications(
                        Message::RandrResult(Arc::new(task.status().await)),
                    ))
                });
            }
            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }
        }

        Task::none()
    }
}

pub fn legacy_application_global_shortcuts() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let desc = descriptions.insert(fl!("legacy-app-global-shortcuts", "desc"));
    let none = descriptions.insert(fl!("legacy-app-global-shortcuts", "none"));
    let modifiers = descriptions.insert(fl!("legacy-app-global-shortcuts", "modifiers"));
    let combination = descriptions.insert(fl!("legacy-app-global-shortcuts", "combination"));
    let all = descriptions.insert(fl!("legacy-app-global-shortcuts", "all"));
    let mouse = descriptions.insert(fl!("legacy-app-global-shortcuts", "mouse"));

    Section::default()
        .title(fl!("legacy-app-global-shortcuts"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let title = widget::text::body(&section.title).font(cosmic::font::bold());
            let description = widget::text::body(&section.descriptions[desc]);

            let content = widget::settings::section::<'_, crate::pages::Message>()
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        text::body(&section.descriptions[none]),
                        EavesdroppingKeyboardMode::None,
                        Some(page.comp_config_xwayland_eavesdropping.keyboard),
                        |t| Message::SetXwaylandKeyboardMode(t).into(),
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        text::body(&section.descriptions[modifiers]),
                        EavesdroppingKeyboardMode::Modifiers,
                        Some(page.comp_config_xwayland_eavesdropping.keyboard),
                        |t| Message::SetXwaylandKeyboardMode(t).into(),
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        text::body(&section.descriptions[combination]),
                        EavesdroppingKeyboardMode::Combinations,
                        Some(page.comp_config_xwayland_eavesdropping.keyboard),
                        |t| Message::SetXwaylandKeyboardMode(t).into(),
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        text::body(&section.descriptions[all]),
                        EavesdroppingKeyboardMode::All,
                        Some(page.comp_config_xwayland_eavesdropping.keyboard),
                        |t| Message::SetXwaylandKeyboardMode(t).into(),
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item(
                    &section.descriptions[mouse],
                    widget::toggler(page.comp_config_xwayland_eavesdropping.pointer)
                        .on_toggle(|t| Message::SetXwaylandMouseButtonMode(t).into()),
                ));

            widget::column::with_capacity(3)
                .push(title)
                .push(description)
                .push(content)
                .spacing(cosmic::theme::active().cosmic().spacing.space_xxs)
                .apply(cosmic::Element::from)
                .map(Into::into)
        })
}

pub fn legacy_application_scaling() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let gaming = descriptions.insert(fl!("legacy-app-scaling", "scaled-gaming"));
    let gaming_desc = descriptions.insert(fl!("legacy-app-scaling", "gaming-description"));
    let apps = descriptions.insert(fl!("legacy-app-scaling", "scaled-applications"));
    let apps_desc = descriptions.insert(fl!("legacy-app-scaling", "applications-description"));
    let compat = descriptions.insert(fl!("legacy-app-scaling", "scaled-compatibility"));
    let compat_desc = descriptions.insert(fl!("legacy-app-scaling", "compatibility-description"));
    let preferred_display = descriptions.insert(fl!("legacy-app-scaling", "preferred-display"));

    Section::default()
        .title(fl!("legacy-app-scaling"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            widget::settings::section()
                .title(&section.title)
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        widget::column()
                            .push(text::body(&descriptions[gaming]))
                            .push(text::caption(&descriptions[gaming_desc])),
                        XwaylandDescaling::Fractional,
                        Some(page.comp_config_descale_xwayland),
                        Message::SetXwaylandDescaling,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        widget::column()
                            .push(text::body(&descriptions[apps]))
                            .push(text::caption(&descriptions[apps_desc])),
                        XwaylandDescaling::Enabled,
                        Some(page.comp_config_descale_xwayland),
                        Message::SetXwaylandDescaling,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        widget::column()
                            .push(text::body(&descriptions[compat]))
                            .push(text::caption(&descriptions[compat_desc])),
                        XwaylandDescaling::Disabled,
                        Some(page.comp_config_descale_xwayland),
                        Message::SetXwaylandDescaling,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item(
                    &descriptions[preferred_display],
                    dropdown::popup_dropdown(
                        &page.output_options,
                        Some(page.output_options_selected),
                        Message::SetXwaylandPrimaryOutput,
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        |a| {
                            crate::app::Message::PageMessage(
                                crate::pages::Message::LegacyApplications(a),
                            )
                        },
                    ),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::LegacyApplications)
        })
}
