use std::collections::HashSet;
use std::fmt::Write;

use cosmic::{
    Apply,
    iced::{Element, Length},
    iced_core::text::Wrapping,
    surface,
    widget::{self, icon, settings, svg, text},
};
use cosmic_comp_config::{ZoomConfig, ZoomMovement};
use cosmic_config::{ConfigGet, ConfigSet};
use cosmic_settings_config::{Action, Binding, shortcuts};
use cosmic_settings_page::{
    self as page, Entity,
    section::{self, Section},
};
use slotmap::SlotMap;
use tracing::error;

use super::{AccessibilityEvent, AccessibilityRequest, cosmic_a11y_manager as wayland};

#[derive(Debug)]
pub struct Page {
    entity: Entity,

    accessibility_config: cosmic_config::Config,
    zoom_config: ZoomConfig,
    increment_values: Vec<String>,
    increment_idx: Option<usize>,
    zoom_in_shortcuts: Vec<Binding>,
    zoom_out_shortcuts: Vec<Binding>,

    wayland_thread: Option<wayland::Sender>,
    magnifier_state: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(wayland::AccessibilityEvent),
    ProtocolUnavailable,
    SetMagnifier(bool),
    SetMouseShortcuts(bool),
    SetOverlay(bool),
    SetIncrement(usize),
    SetSignin(bool),
    SetMovement(ZoomMovement),
    Surface(surface::Action),
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

        let mut values = HashSet::<u32>::from_iter([25, 50, 100, 150, 200, zoom_config.increment])
            .into_iter()
            .collect::<Vec<_>>();
        values.sort_unstable();
        let increment_values = values
            .into_iter()
            .map(|val| {
                let mut option_string = String::new();
                write!(&mut option_string, "{}%", val).expect("");
                if val == ZoomConfig::default().increment {
                    write!(&mut option_string, " ({})", fl!("default")).expect("");
                }
                option_string
            })
            .collect::<Vec<_>>();
        let increment_idx = increment_values.iter().position(|s| {
            s.split('%').next().and_then(|val| str::parse(val).ok()) == Some(zoom_config.increment)
        });

        let (zoom_in_shortcuts, zoom_out_shortcuts) = zoom_shortcuts();

        Page {
            entity: Entity::default(),

            accessibility_config: comp_config,
            zoom_config,
            increment_values,
            increment_idx,
            zoom_in_shortcuts,
            zoom_out_shortcuts,

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
            sections.insert(magnifier(&self.zoom_in_shortcuts, &self.zoom_out_shortcuts)),
            sections.insert(tip()),
            sections.insert(view_movement()),
        ])
    }

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        if self.wayland_thread.is_none() {
            match wayland::spawn_wayland_connection(1) {
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

pub fn magnifier(
    zoom_in: &[Binding],
    zoom_out: &[Binding],
) -> section::Section<crate::pages::Message> {
    let zoom_in = if zoom_in.is_empty() {
        String::from("zero")
    } else {
        zoom_in.iter().fold(String::new(), |mut str, b| {
            if !str.is_empty() {
                str += ", ";
            }
            b.to_string_in_place(&mut str);
            str
        })
    };
    let zoom_out = if zoom_out.is_empty() {
        String::from("zero")
    } else {
        zoom_out.iter().fold(String::new(), |mut str, b| {
            if !str.is_empty() {
                str += ", ";
            }
            b.to_string_in_place(&mut str);
            str
        })
    };

    crate::slab!(descriptions {
        magnifier = fl!("magnifier");
        controls = fl!("magnifier", "controls", zoom_in = zoom_in, zoom_out = zoom_out);
        scroll_controls = fl!("magnifier", "scroll_controls");
        show_overlay = fl!("magnifier", "show_overlay");
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
                .add(settings::item(
                    &descriptions[scroll_controls],
                    widget::toggler(page.zoom_config.enable_mouse_zoom_shortcuts)
                        .on_toggle(Message::SetMouseShortcuts),
                ))
                .add(settings::item(
                    &descriptions[show_overlay],
                    widget::toggler(page.zoom_config.show_overlay).on_toggle(Message::SetOverlay),
                ))
                .add(settings::item(
                    &descriptions[increment],
                    widget::dropdown::popup_dropdown(
                        &page.increment_values,
                        page.increment_idx,
                        Message::SetIncrement,
                        cosmic::iced::window::Id::RESERVED,
                        Message::Surface,
                        |a| {
                            crate::app::Message::PageMessage(
                                crate::pages::Message::AccessibilityMagnifier(a),
                            )
                        },
                    ),
                ))
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

            let mut items = vec![
                text::body(&descriptions[applet])
                    .wrapping(Wrapping::Word)
                    .width(Length::Shrink)
                    .into(),
            ];
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
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        text::body(&descriptions[continuous]),
                        ZoomMovement::Continuously,
                        Some(page.zoom_config.view_moves),
                        Message::SetMovement,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        text::body(&descriptions[onedge]),
                        ZoomMovement::OnEdge,
                        Some(page.zoom_config.view_moves),
                        Message::SetMovement,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(widget::settings::item_row(vec![
                    widget::radio(
                        text::body(&descriptions[centered]),
                        ZoomMovement::Centered,
                        Some(page.zoom_config.view_moves),
                        Message::SetMovement,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
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
            Message::Event(
                AccessibilityEvent::Bound(_) | AccessibilityEvent::ScreenFilter { .. },
            ) => {}
            Message::SetMagnifier(value) => {
                if let Some(sender) = self.wayland_thread.as_ref() {
                    let _ = sender.send(AccessibilityRequest::Magnifier(value));
                }
            }
            Message::SetMouseShortcuts(value) => {
                self.zoom_config.enable_mouse_zoom_shortcuts = value;

                if let Err(err) = self
                    .accessibility_config
                    .set("accessibility_zoom", self.zoom_config)
                {
                    error!(?err, "Failed to set config 'accessibility_zoom'");
                }
            }
            Message::SetOverlay(value) => {
                self.zoom_config.show_overlay = value;

                if let Err(err) = self
                    .accessibility_config
                    .set("accessibility_zoom", self.zoom_config)
                {
                    error!(?err, "Failed to set config 'accessibility_zoom'");
                }
            }
            Message::SetIncrement(idx) => {
                self.increment_idx = Some(idx);
                let value = self.increment_values[idx]
                    .split("%")
                    .next()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                self.zoom_config.increment = value;

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
            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }
        }

        cosmic::iced::Task::none()
    }
}

fn zoom_shortcuts() -> (Vec<Binding>, Vec<Binding>) {
    let Some(config) = shortcuts::context().ok() else {
        return (Vec::new(), Vec::new());
    };
    let shortcuts = shortcuts::shortcuts(&config);

    let zoom_in = shortcuts
        .iter()
        .filter(|&(_binding, action)| *action == Action::ZoomIn)
        .map(|(binding, _action)| binding.clone())
        .collect();
    let zoom_out = shortcuts
        .iter()
        .filter(|&(_binding, action)| *action == Action::ZoomOut)
        .map(|(binding, _action)| binding.clone())
        .collect();

    (zoom_in, zoom_out)
}
