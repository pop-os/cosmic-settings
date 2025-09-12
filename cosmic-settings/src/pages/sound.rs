// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    Apply, Element, Task,
    iced::{Alignment, Length, window},
    surface,
    widget::{self, settings},
};
use cosmic_config::{Config, ConfigGet, ConfigSet};
use cosmic_settings_page::{self as page, Section, section};
use slab::Slab;
use slotmap::SlotMap;

use cosmic_settings_subscriptions::sound as subscription;

const AUDIO_CONFIG: &str = "com.system76.CosmicAudio";
const AMPLIFICATION_SINK: &str = "amplification_sink";
const AMPLIFICATION_SOURCE: &str = "amplification_source";

#[derive(Clone, Debug)]
pub enum Message {
    /// Change the balance of the active sink.
    SinkBalanceChanged(u32),
    /// Change the default output.
    SinkChanged(usize),
    /// Toggle the mute status of the output.
    SinkMuteToggle,
    /// Change the active profile for an output.
    SinkProfileChanged(usize),
    /// Request to change the default output volume.
    SinkVolumeChanged(u32),
    /// Toggle amplification for sink
    ToggleOverAmplificationSink(bool),
    /// Change the default input output.
    SourceChanged(usize),
    /// Toggle the mute status of the input output.
    SourceMuteToggle,
    /// Change the active profile for an output.
    SourceProfileChanged(usize),
    /// Request to change the input volume.
    SourceVolumeChanged(u32),
    /// Toggle amplification for sink
    ToggleOverAmplificationSource(bool),
    /// Messages handled by the sound module in cosmic-settings-subscriptions
    Subscription(subscription::Message),
    /// Surface Action
    Surface(surface::Action),
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Sound(message)
    }
}

impl From<Message> for crate::Message {
    fn from(message: Message) -> Self {
        crate::Message::PageMessage(message.into())
    }
}

impl Into<Message> for subscription::Message {
    fn into(self) -> Message {
        Message::Subscription(self)
    }
}

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
    model: subscription::Model,
    sound_config: Option<Config>,
    amplification_sink: bool,
    amplification_source: bool,
}

impl page::Page<crate::pages::Message> for Page {
    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        match Config::new(AUDIO_CONFIG, 1) {
            Ok(config) => {
                self.amplification_sink = config.get::<bool>(AMPLIFICATION_SINK).unwrap_or(true);
                self.amplification_source =
                    config.get::<bool>(AMPLIFICATION_SOURCE).unwrap_or(false);
                self.sound_config = Some(config);
            }
            Err(why) => {
                tracing::error!(?why, "Failed to load sound config");
                self.amplification_sink = true;
                self.amplification_source = false;
            }
        }
        Task::none()
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(output()), sections.insert(input())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("sound", "preferences-sound-symbolic")
            .title(fl!("sound"))
            .description(fl!("sound", "desc"))
    }

    fn subscription(
        &self,
        _core: &cosmic::Core,
    ) -> cosmic::iced::Subscription<crate::pages::Message> {
        cosmic::iced::Subscription::run(|| subscription::watch())
            .map(|message| Message::Subscription(message).into())
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        self.model.clear();

        *self = Page {
            entity: self.entity,
            ..Page::default()
        };

        Task::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::SinkBalanceChanged(balance) => {
                return self
                    .model
                    .sink_balance_changed(balance)
                    .map(|message| Message::Subscription(message).into());
            }
            Message::SinkChanged(pos) => {
                return self
                    .model
                    .sink_changed(pos)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::SinkMuteToggle => self.model.sink_mute_toggle(),

            Message::SinkProfileChanged(profile) => {
                return self
                    .model
                    .sink_profile_changed(profile)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::SinkVolumeChanged(volume) => {
                return self
                    .model
                    .sink_volume_changed(volume)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::ToggleOverAmplificationSink(enabled) => {
                self.amplification_sink = enabled;

                if let Some(config) = &self.sound_config {
                    if let Err(why) = config.set(AMPLIFICATION_SINK, enabled) {
                        tracing::error!(?why, "Failed to save over amplification setting");
                    }
                }
            }

            Message::SourceChanged(pos) => {
                return self
                    .model
                    .source_changed(pos)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::SourceMuteToggle => self.model.source_mute_toggle(),

            Message::SourceProfileChanged(profile) => {
                return self
                    .model
                    .source_profile_changed(profile)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::SourceVolumeChanged(volume) => {
                return self
                    .model
                    .source_volume_changed(volume)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::ToggleOverAmplificationSource(enabled) => {
                self.amplification_source = enabled;

                if let Some(config) = &self.sound_config {
                    if let Err(why) = config.set(AMPLIFICATION_SOURCE, enabled) {
                        tracing::error!(?why, "Failed to save over amplification setting");
                    }
                }
            }

            Message::Subscription(message) => {
                return self
                    .model
                    .update(message)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::Surface(a) => return cosmic::task::message(crate::app::Message::Surface(a)),
        }

        Task::none()
    }
}

fn input() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let volume = descriptions.insert(fl!("sound-input", "volume"));
    let device = descriptions.insert(fl!("sound-input", "device"));
    let _level = descriptions.insert(fl!("sound-input", "level"));
    let profile = descriptions.insert(fl!("profile"));
    let amplification = descriptions.insert(fl!("amplification"));
    let amplification_desc = descriptions.insert(fl!("amplification", "desc"));

    Section::default()
        .title(fl!("sound-input"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            if page.model.sources().is_empty() {
                return widget::row().into();
            }

            let slider = if page.amplification_source {
                widget::slider(0..=150, page.model.source_volume, |change| {
                    Message::SourceVolumeChanged(change).into()
                })
                .breakpoints(&[100])
            } else {
                widget::slider(0..=100, page.model.source_volume, |change| {
                    Message::SourceVolumeChanged(change).into()
                })
            };

            let volume_control = widget::row::with_capacity(4)
                .align_y(Alignment::Center)
                .push(
                    widget::button::icon(widget::icon::from_name(if page.model.source_mute {
                        "microphone-sensitivity-muted-symbolic"
                    } else {
                        "audio-input-microphone-symbolic"
                    }))
                    .on_press(Message::SourceMuteToggle.into()),
                )
                .push(
                    widget::text::body(&page.model.source_volume_text)
                        .width(Length::Fixed(22.0))
                        .align_x(Alignment::Center),
                )
                .push(widget::horizontal_space().width(8))
                .push(slider);
            let devices = widget::dropdown::popup_dropdown(
                page.model.sources(),
                Some(page.model.active_source().unwrap_or(0)),
                Message::SourceChanged,
                window::Id::RESERVED,
                Message::Surface,
                |a| crate::Message::from(a),
            )
            .apply(Element::from)
            .map(crate::pages::Message::from);

            let mut controls = settings::section()
                .title(&section.title)
                .add(settings::flex_item(
                    &*section.descriptions[volume],
                    volume_control,
                ))
                .add(settings::item(&*section.descriptions[device], devices));

            if !page.model.source_profiles().is_empty() {
                let dropdown = widget::dropdown::popup_dropdown(
                    page.model.source_profiles(),
                    page.model.active_source_profile(),
                    Message::SourceProfileChanged,
                    window::Id::RESERVED,
                    Message::Surface,
                    |a| crate::Message::from(a),
                )
                .apply(Element::from)
                .map(crate::pages::Message::from);

                controls = controls.add(settings::item(&*section.descriptions[profile], dropdown));
            }

            controls = controls.add(
                settings::item::builder(&*section.descriptions[amplification])
                    .description(&*section.descriptions[amplification_desc])
                    .control(
                        widget::toggler(page.amplification_source)
                            .on_toggle(|t| Message::ToggleOverAmplificationSource(t).into()),
                    ),
            );

            Element::from(controls)
        })
}

fn output() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let volume = descriptions.insert(fl!("sound-output", "volume"));
    let device = descriptions.insert(fl!("sound-output", "device"));
    let _level = descriptions.insert(fl!("sound-output", "level"));
    let profile = descriptions.insert(fl!("profile"));
    let balance = descriptions.insert(fl!("sound-output", "balance"));
    let left = descriptions.insert(fl!("sound-output", "left"));
    let right = descriptions.insert(fl!("sound-output", "right"));
    // let balance = descriptions.insert(fl!("sound-output", "balance"));
    let amplification = descriptions.insert(fl!("amplification"));
    let amplification_desc = descriptions.insert(fl!("amplification", "desc"));

    Section::default()
        .title(fl!("sound-output"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let slider = if page.amplification_sink {
                widget::slider(0..=150, page.model.sink_volume, |change| {
                    Message::SinkVolumeChanged(change).into()
                })
                .breakpoints(&[100])
            } else {
                widget::slider(0..=100, page.model.sink_volume, |change| {
                    Message::SinkVolumeChanged(change).into()
                })
            };

            let volume_control = widget::row::with_capacity(4)
                .align_y(Alignment::Center)
                .push(
                    widget::button::icon(if page.model.sink_mute {
                        widget::icon::from_name("audio-volume-muted-symbolic")
                    } else {
                        widget::icon::from_name("audio-volume-high-symbolic")
                    })
                    .on_press(Message::SinkMuteToggle.into()),
                )
                .push(
                    widget::text::body(&page.model.sink_volume_text)
                        .width(Length::Fixed(22.0))
                        .align_x(Alignment::Center),
                )
                .push(widget::horizontal_space().width(8))
                .push(slider);

            let devices = widget::dropdown::popup_dropdown(
                page.model.sinks(),
                Some(page.model.active_sink().unwrap_or(0)),
                Message::SinkChanged,
                window::Id::RESERVED,
                Message::Surface,
                |a| crate::Message::from(a),
            )
            .apply(Element::from)
            .map(crate::pages::Message::from);

            let mut controls = settings::section()
                .title(&section.title)
                .add(settings::flex_item(
                    &*section.descriptions[volume],
                    volume_control,
                ))
                .add(settings::item(&*section.descriptions[device], devices));

            if !page.model.sink_profiles().is_empty() {
                let dropdown = widget::dropdown::popup_dropdown(
                    page.model.sink_profiles(),
                    page.model.active_sink_profile(),
                    Message::SinkProfileChanged,
                    window::Id::RESERVED,
                    Message::Surface,
                    |a| crate::Message::from(a),
                )
                .apply(Element::from)
                .map(crate::pages::Message::from);

                controls = controls.add(settings::item(&*section.descriptions[profile], dropdown));
            }
            if let Some(sink_balance) = page.model.sink_balance {
                controls = controls.add(settings::item(
                    &*section.descriptions[balance],
                    widget::row::with_capacity(4)
                        .align_y(Alignment::Center)
                        .push(
                            widget::text::body(&*section.descriptions[left])
                                .width(Length::Fixed(22.0))
                                .align_x(Alignment::Center),
                        )
                        .push(widget::horizontal_space().width(8))
                        .push(
                            widget::slider(
                                0..=200,
                                ((sink_balance + 1.).max(0.) * 100.).round() as u32,
                                |change| Message::SinkBalanceChanged(change).into(),
                            )
                            .breakpoints(&[100]),
                        )
                        .push(widget::horizontal_space().width(8))
                        .push(
                            widget::text::body(&*section.descriptions[right])
                                .width(Length::Fixed(22.0))
                                .align_x(Alignment::Center),
                        ),
                ));
            }

            controls = controls.add(
                settings::item::builder(&*section.descriptions[amplification])
                    .description(&*section.descriptions[amplification_desc])
                    .control(
                        widget::toggler(page.amplification_sink)
                            .on_toggle(|t| Message::ToggleOverAmplificationSink(t).into()),
                    ),
            );

            Element::from(controls)
        })
}

// fn alerts() -> Section<crate::pages::Message> {
//     let mut descriptions = Slab::new();
//     let volume = descriptions.insert(fl!("sound-alerts", "volume"));
//     let sound = descriptions.insert(fl!("sound-alerts", "sound"));

//     Section::default()
//         .title(fl!("sound-alerts"))
//         .descriptions(descriptions)
//         .view::<Page>(move |_binder, _page, section| {
//             settings::section().title(&section.title)
//                 .add(settings::item(&section.descriptions[volume], text::body("TODO")))
//                 .add(settings::item(&section.descriptions[sound], text::body("TODO")))
//                 .into()
//         })
// }

// fn applications() -> Section<crate::pages::Message> {
//     let mut descriptions = Slab::new();

//     let applications = descriptions.insert(fl!("sound-applications", "desc"));

//     Section::default()
//         .title(fl!("sound-applications"))
//         .descriptions(descriptions)
//         .view::<Page>(move |_binder, _page, section| {
//             settings::section().title(&section.title)
//                 .add(settings::item(
//                     &*section.descriptions[applications],
//                     text::body("TODO"),
//                 ))
//                 .into()
//         })
// }
