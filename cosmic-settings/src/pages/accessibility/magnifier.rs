use std::collections::HashSet;

use cosmic::{
    iced::{Element, Length},
    iced_core::text::Wrapping,
    widget::{self, icon, settings, svg, text},
    Apply,
};
use cosmic_comp_config::{ZoomConfig, ZoomMovement};
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_page::{
    self as page,
    section::{self, Section},
    Entity,
};
use slotmap::SlotMap;
use tracing::error;

use super::{wayland, AccessibilityEvent, AccessibilityRequest};

#[derive(Debug)]
pub struct Page {
    entity: Entity,

    accessibility_config: cosmic_config::Config,
    zoom_config: ZoomConfig,
    zoom_increment: u32,

    wayland_thread: Option<wayland::Sender>,
    magnifier_state: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(wayland::AccessibilityEvent),
    ProtocolUnavailable,
    SetMagnifier(bool),
    SetIncrement(u32),
    SetSignin(bool),
    SetMovement(ZoomMovement),
}

impl Default for Page {
    fn default() -> Self {
        let comp_config = cosmic_config::Config::new("com.system76.CosmicComp", 1).unwrap();
        let zoom_config: ZoomConfig = comp_config
            .get("accessibility_zoom")
            .inspect_err(|err| {
                if err.is_err() {
                    error!(?err, "Failed to read config 'accessibility_zoom'");
                }
            })
            .unwrap_or_default();

        Page {
            entity: Entity::default(),

            accessibility_config: comp_config,
            zoom_increment: zoom_config.increment,
            zoom_config,

            wayland_thread: None,
            magnifier_state: false,
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: Entity) {
        self.entity = entity;
    }

    fn info(&self) -> page::Info {
        page::Info::new(
            "accessibility_magnifier",
            "preferences-desktop-accessibility",
        )
        .title(fl!("magnifier"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, page::Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(magnifier()),
            sections.insert(tip()),
            sections.insert(view_movement()),
        ])
    }

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        if self.wayland_thread.is_none() {
            match wayland::spawn_wayland_connection() {
                Ok((tx, mut rx)) => {
                    self.wayland_thread = Some(tx);

                    return cosmic::Task::stream(async_fn_stream::fn_stream(
                        |emitter| async move {
                            while let Some(event) = rx.recv().await {
                                let _ = emitter
                                    .emit(crate::pages::Message::AccessibilityMagnifier(
                                        Message::Event(event),
                                    ))
                                    .await;
                            }

                            let _ = emitter
                                .emit(crate::pages::Message::AccessibilityMagnifier(
                                    Message::ProtocolUnavailable,
                                ))
                                .await;
                        },
                    ));
                }
                Err(err) => {
                    tracing::warn!(
                        "Failed to spawn wayland connection for magnifier page: {}",
                        err
                    );
                    return cosmic::Task::done(crate::pages::Message::Accessibility(
                        super::Message::Return,
                    ));
                }
            }
        }

        cosmic::Task::none()
    }

    fn on_leave(&mut self) -> cosmic::Task<crate::pages::Message> {
        let _ = self.wayland_thread.take();

        cosmic::Task::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

pub fn magnifier() -> section::Section<crate::pages::Message> {
    crate::slab!(descriptions {
        magnifier = fl!("magnifier");
        controls = fl!("magnifier", "controls");
        increment = fl!("magnifier", "increment");
        signin = fl!("magnifier", "signin");
    });

    Section::default()
        .title(&descriptions[magnifier])
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(
                    settings::item::builder(&descriptions[magnifier])
                        .description(&descriptions[controls])
                        .control(
                            widget::toggler(page.magnifier_state).on_toggle(Message::SetMagnifier),
                        ),
                )
                .add(settings::item::builder(&descriptions[increment])
                    .flex_control({
                        let slider = widget::slider(25..=200, page.zoom_increment as i32, |value| {
                            // Round to nearest multiple of 5
                            let rounded_value = ((value + 2) / 5) * 5;
                            Message::SetIncrement(rounded_value as u32)
                        })
                        .step(5)
                        .width(Length::Fill)
                        .apply(widget::container)
                        .max_width(250);

                        widget::row::with_capacity(2)
                            .align_y(cosmic::iced::Alignment::Center)
                            .spacing(8)
                            .push(
                                widget::text::body(format!("{}%", page.zoom_increment))
                                    .width(Length::Fixed(48.0))
                                    .align_x(cosmic::iced::Alignment::Center),
                            )
                            .push(slider)
                    }),
                )
                .add(settings::item(
                    &descriptions[signin],
                    widget::toggler(page.zoom_config.start_on_login).on_toggle(Message::SetSignin),
                ))
                .apply(Element::from)
                .map(crate::pages::Message::AccessibilityMagnifier)
        })
}

pub fn tip() -> section::Section<crate::pages::Message> {
    crate::slab!(descriptions {
        applet = fl!("magnifier", "applet");
    });
    let applet_illustration = icon::from_name("illustration-accessibility-magnifier-applet")
        .icon()
        .into_svg_handle();

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            let descriptions = &section.descriptions;

            let mut items = vec![text::body(&descriptions[applet])
                .wrapping(Wrapping::Word)
                .width(Length::Shrink)
                .into()];
            if let Some(illustration) = applet_illustration.clone() {
                items.push(svg(illustration).width(Length::Fill).into());
            }

            settings::section()
                .add(settings::flex_item_row(items))
                .into()
        })
}

pub fn view_movement() -> section::Section<crate::pages::Message> {
    crate::slab!(descriptions {
        movement = fl!("magnifier", "movement");
        continuous = fl!("magnifier", "continuous");
        onedge = fl!("magnifier", "onedge");
        centered = fl!("magnifier", "centered");
    });
    Section::default()
        .title(&descriptions[movement])
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(widget::settings::item_row(vec![widget::radio(
                    text::body(&descriptions[continuous]),
                    ZoomMovement::Continuously,
                    Some(page.zoom_config.view_moves),
                    Message::SetMovement,
                )
                .width(Length::Fill)
                .into()]))
                .add(widget::settings::item_row(vec![widget::radio(
                    text::body(&descriptions[onedge]),
                    ZoomMovement::OnEdge,
                    Some(page.zoom_config.view_moves),
                    Message::SetMovement,
                )
                .width(Length::Fill)
                .into()]))
                .add(widget::settings::item_row(vec![widget::radio(
                    text::body(&descriptions[centered]),
                    ZoomMovement::Centered,
                    Some(page.zoom_config.view_moves),
                    Message::SetMovement,
                )
                .width(Length::Fill)
                .into()]))
                .apply(Element::from)
                .map(crate::pages::Message::AccessibilityMagnifier)
        })
}

impl Page {
    pub fn update(
        &mut self,
        active_page: page::Entity,
        message: Message,
    ) -> cosmic::iced::Task<crate::app::Message> {
        match message {
            Message::Event(AccessibilityEvent::Magnifier(value)) => {
                self.magnifier_state = value;
            }
            Message::SetMagnifier(value) => {
                if let Some(sender) = self.wayland_thread.as_ref() {
                    let _ = sender.send(AccessibilityRequest::Magnifier(value));
                }
            }
            Message::SetIncrement(value) => {
                // Ensure the value is a multiple of 5
                let rounded_value = ((value + 2) / 5) * 5;
                self.zoom_increment = rounded_value;
                self.zoom_config.increment = rounded_value;

                if let Err(err) = self
                    .accessibility_config
                    .set("accessibility_zoom", self.zoom_config)
                {
                    error!(?err, "Failed to set config 'accessibility_zoom'");
                }
            }
            Message::SetSignin(value) => {
                self.zoom_config.start_on_login = value;

                if let Err(err) = self
                    .accessibility_config
                    .set("accessibility_zoom", self.zoom_config)
                {
                    error!(?err, "Failed to set config 'accessibility_zoom'");
                }
            }
            Message::SetMovement(zoom_movement) => {
                self.zoom_config.view_moves = zoom_movement;

                if let Err(err) = self
                    .accessibility_config
                    .set("accessibility_zoom", self.zoom_config)
                {
                    error!(?err, "Failed to set config 'accessibility_zoom'");
                }
            }
            // We shouldn't have gotten into this page in that case
            Message::Event(AccessibilityEvent::Closed) | Message::ProtocolUnavailable => {
                if active_page == self.entity {
                    return cosmic::iced::Task::done(crate::app::Message::PageMessage(
                        crate::pages::Message::Accessibility(super::Message::Return),
                    ));
                }
            }
        }

        cosmic::iced::Task::none()
    }
}
