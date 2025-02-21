use cosmic::{
    iced_core::text::Wrapping,
    theme,
    widget::{button, container, horizontal_space, icon, settings, text},
    Apply,
};
pub use cosmic_comp_config::ZoomMovement;
use cosmic_settings_page::{
    self as page,
    section::{self, Section},
    Insert,
};
use slotmap::SlotMap;
use tokio::sync::mpsc;

pub mod magnifier;
mod wayland;
pub use wayland::{AccessibilityEvent, AccessibilityRequest};

#[derive(Debug, Default)]
pub struct Page {
    entity: page::Entity,
    magnifier_state: bool,

    wayland_available: bool,
    wayland_thread: Option<wayland::Sender>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(wayland::AccessibilityEvent),
    ProtocolUnavailable,
    Return,
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> page::Info {
        page::Info::new(
            "accessibility",
            "preferences-desktop-accessibility-symbolic",
        )
        .title(fl!("accessibility"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, page::Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(vision())])
    }

    fn on_enter(
        &mut self,
        sender: mpsc::Sender<crate::pages::Message>,
    ) -> cosmic::Task<crate::pages::Message> {
        if self.wayland_thread.is_none() {
            match wayland::spawn_wayland_connection() {
                Ok((tx, mut rx)) => {
                    self.wayland_available = true;
                    self.wayland_thread = Some(tx);
                    tokio::task::spawn(async move {
                        while let Some(event) = rx.recv().await {
                            let _ = sender
                                .send(crate::pages::Message::Accessibility(Message::Event(event)))
                                .await;
                        }
                        let _ = sender
                            .send(crate::pages::Message::Accessibility(
                                Message::ProtocolUnavailable,
                            ))
                            .await;
                    });
                }
                Err(err) => {
                    tracing::warn!(
                        "Failed to spawn wayland connection for accessibility page: {}",
                        err
                    );
                    self.wayland_available = false;
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

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: Insert<crate::pages::Message>) -> Insert<crate::pages::Message> {
        page.sub_page::<magnifier::Page>()
    }
}

pub fn vision() -> section::Section<crate::pages::Message> {
    crate::slab!(descriptions {
        magnifier = fl!("magnifier");
        vision = fl!("accessibility", "vision");
        on = fl!("accessibility", "on");
        off = fl!("accessibility", "off");
        unavailable = fl!("accessibility", "unavailable");
    });

    Section::default()
        .title(&descriptions[vision])
        .descriptions(descriptions)
        .view::<Page>(move |binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add({
                    let (magnifier_entity, _magnifier_info) = binder
                        .info
                        .iter()
                        .find(|(_, v)| v.id == "accessibility_magnifier")
                        .expect("magnifier page not found");

                    let status_text = if page.wayland_available {
                        if page.magnifier_state {
                            &descriptions[on]
                        } else {
                            &descriptions[off]
                        }
                    } else {
                        &descriptions[unavailable]
                    };

                    settings::item_row(vec![
                        text::body(&descriptions[magnifier])
                            .wrapping(Wrapping::Word)
                            .into(),
                        horizontal_space().into(),
                        text::body(status_text).wrapping(Wrapping::Word).into(),
                        icon::from_name("go-next-symbolic").size(16).into(),
                    ])
                    .apply(container)
                    .class(cosmic::theme::Container::List)
                    .apply(button::custom)
                    .class(theme::Button::Transparent)
                    .on_press_maybe(
                        page.wayland_available
                            .then_some(crate::pages::Message::Page(magnifier_entity)),
                    )
                })
                .into()
        })
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::iced::Task<crate::app::Message> {
        match message {
            Message::Event(AccessibilityEvent::Magnifier(value)) => {
                self.magnifier_state = value;
            }
            Message::Event(AccessibilityEvent::Closed) | Message::ProtocolUnavailable => {
                self.wayland_available = false;
            }
            Message::Return => {
                return cosmic::iced::Task::done(crate::app::Message::Page(self.entity))
            }
        }

        cosmic::iced::Task::none()
    }
}
