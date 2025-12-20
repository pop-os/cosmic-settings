// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod device_profiles;

use cosmic::{
    Apply, Element, Task,
    iced::{Alignment, Length, window},
    surface,
    widget::{self, settings},
};
use cosmic_config::{Config, ConfigGet, ConfigSet};
use cosmic_settings_page::{self as page, Section, section};
use cosmic_settings_sound_subscription as subscription;
use slab::Slab;
use slotmap::SlotMap;

const AUDIO_CONFIG: &str = "com.system76.CosmicAudio";
const AMPLIFICATION_SINK: &str = "amplification_sink";
const AMPLIFICATION_SOURCE: &str = "amplification_source";

#[derive(Clone, Debug)]
pub enum Message {
    /// Reload the model
    Reload,
    /// Change the default output.
    SetDefaultSink(usize),
    /// Change the default input output.
    SetDefaultSource(usize),
    /// Set the profile of a sound device.
    SetProfile(u32, u32),
    /// Change the balance of the active sink.
    SetSinkBalance(u32),
    /// Request to change the default output volume.
    SetSinkVolume(u32),
    /// Request to change the input volume.
    SetSourceVolume(u32),
    /// Messages handled by the sound module in cosmic-settings-subscriptions
    Subscription(subscription::Message),
    /// Surface Action
    Surface(surface::Action),
    /// Toggle the mute status of the output.
    ToggleSinkMute,
    /// Toggle the mute status of the input output.
    ToggleSourceMute,
    /// Toggle amplification for sink
    ToggleOverAmplificationSink(bool),
    /// Toggle amplification for sink
    ToggleOverAmplificationSource(bool),
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

impl From<subscription::Message> for Message {
    fn from(val: subscription::Message) -> Self {
        Message::Subscription(val)
    }
}

pub struct Page {
    entity: page::Entity,
    device_profiles: page::Entity,
    pub(self) model: subscription::Model,
    sound_config: Option<Config>,
    amplification_sink: bool,
    amplification_source: bool,
}

impl Default for Page {
    fn default() -> Self {
        let mut model = subscription::Model::default();
        model.unplugged_text = fl!("sound-device-port-unplugged");
        model.hd_audio_text = fl!("sound-hd-audio");
        model.usb_audio_text = fl!("sound-usb-audio");
        Self {
            entity: page::Entity::default(),
            device_profiles: page::Entity::default(),
            model,
            sound_config: None,
            amplification_sink: false,
            amplification_source: false,
        }
    }
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
        Some(vec![
            sections.insert(output()),
            sections.insert(input()),
            sections.insert(device_profiles()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("sound", "preferences-sound-symbolic")
            .title(fl!("sound"))
            .description(fl!("sound", "desc"))
    }

    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn subscription(
        &self,
        _core: &cosmic::Core,
    ) -> cosmic::iced::Subscription<crate::pages::Message> {
        cosmic::iced::Subscription::run(subscription::watch)
            .map(|message| Message::Subscription(message).into())
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        *self = Page {
            entity: self.entity,
            device_profiles: self.device_profiles,
            ..Page::default()
        };

        Task::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: page::Insert<crate::pages::Message>,
    ) -> page::Insert<crate::pages::Message> {
        let id = page.sub_page_with_id::<device_profiles::Page>();
        let model = page.model.page_mut::<Page>().unwrap();
        model.device_profiles = id;
        page
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::Surface(a) => return cosmic::task::message(crate::app::Message::Surface(a)),

            Message::Subscription(message) => {
                return self
                    .model
                    .update(message)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::SetSinkBalance(balance) => {
                return self
                    .model
                    .set_sink_balance(balance)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::SetDefaultSink(pos) => {
                return self
                    .model
                    .set_default_sink(pos)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::SetDefaultSource(pos) => {
                return self
                    .model
                    .set_default_source(pos)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::ToggleSinkMute => self.model.toggle_sink_mute(),

            Message::ToggleSourceMute => self.model.toggle_source_mute(),

            Message::SetSinkVolume(volume) => {
                return self
                    .model
                    .set_sink_volume(volume)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::SetSourceVolume(volume) => {
                return self
                    .model
                    .set_source_volume(volume)
                    .map(|message| Message::Subscription(message).into());
            }

            Message::ToggleOverAmplificationSink(enabled) => {
                self.amplification_sink = enabled;

                if let Some(config) = &self.sound_config
                    && let Err(why) = config.set(AMPLIFICATION_SINK, enabled)
                {
                    tracing::error!(?why, "Failed to save over amplification setting");
                }
            }

            Message::ToggleOverAmplificationSource(enabled) => {
                self.amplification_source = enabled;

                if let Some(config) = &self.sound_config
                    && let Err(why) = config.set(AMPLIFICATION_SOURCE, enabled)
                {
                    tracing::error!(?why, "Failed to save over amplification setting");
                }
            }

            Message::SetProfile(object_id, index) => {
                self.model.set_profile(object_id, index, true);
            }

            Message::Reload => {
                let mut model = subscription::Model::default();
                model.hd_audio_text = std::mem::take(&mut self.model.hd_audio_text);
                model.unplugged_text = std::mem::take(&mut self.model.unplugged_text);
                model.usb_audio_text = std::mem::take(&mut self.model.usb_audio_text);
                self.model = model;
            }
        }

        Task::none()
    }
}

fn input() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let volume = descriptions.insert(fl!("sound-input", "volume"));
    let device = descriptions.insert(fl!("sound-input", "device"));
    let _level = descriptions.insert(fl!("sound-input", "level"));
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
                    Message::SetSourceVolume(change).into()
                })
                .breakpoints(&[100])
            } else {
                widget::slider(0..=100, page.model.source_volume, |change| {
                    Message::SetSourceVolume(change).into()
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
                    .on_press(Message::ToggleSourceMute.into()),
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
                Message::SetDefaultSource,
                window::Id::RESERVED,
                Message::Surface,
                crate::Message::from,
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
                    Message::SetSinkVolume(change).into()
                })
                .breakpoints(&[100])
            } else {
                widget::slider(0..=100, page.model.sink_volume, |change| {
                    Message::SetSinkVolume(change).into()
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
                    .on_press(Message::ToggleSinkMute.into()),
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
                Message::SetDefaultSink,
                window::Id::RESERVED,
                Message::Surface,
                crate::Message::from,
            )
            .apply(Element::from)
            .map(crate::pages::Message::from);

            let mut controls = settings::section()
                .title(&section.title)
                .add(settings::flex_item(
                    &*section.descriptions[volume],
                    volume_control,
                ))
                .add(settings::item(&*section.descriptions[device], devices))
                .add(settings::item(
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
                                (page.model.sink_balance.unwrap_or(1.0).max(0.) * 100.).round()
                                    as u32,
                                |change| Message::SetSinkBalance(change).into(),
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

/// A section for opening the device profiles sub-page.
fn device_profiles() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        button_txt = fl!("sound-device-profiles");
    });

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;
            let button = widget::row::with_children(vec![
                widget::horizontal_space().into(),
                widget::icon::from_name("go-next-symbolic").size(16).into(),
            ]);

            let device_profiles = settings::item::builder(&*descriptions[button_txt])
                .control(button)
                .spacing(16)
                .apply(widget::container)
                .class(cosmic::theme::Container::List)
                .apply(widget::button::custom)
                .class(cosmic::theme::Button::Transparent)
                .on_press(crate::pages::Message::Page(page.device_profiles));

            settings::section().add(device_profiles).into()
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
