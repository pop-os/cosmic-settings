use cosmic::{
    app,
    cosmic_theme::{CosmicPalette, ThemeBuilder},
    iced_core::text::Wrapping,
    theme::{self, CosmicTheme},
    widget::{button, container, horizontal_space, icon, settings, text},
    Apply, Task,
};
pub use cosmic_comp_config::ZoomMovement;
use cosmic_config::CosmicConfigEntry;
use cosmic_settings_page::{
    self as page,
    section::{self, Section},
    Insert,
};
use slotmap::SlotMap;

pub mod magnifier;
mod wayland;
use tokio::task::spawn_blocking;
pub use wayland::{AccessibilityEvent, AccessibilityRequest};

#[derive(Debug, Default)]
pub struct Page {
    entity: page::Entity,
    magnifier_state: bool,

    wayland_available: bool,
    wayland_thread: Option<wayland::Sender>,
    theme: Box<cosmic::cosmic_theme::Theme>,
    high_contrast: Option<bool>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(wayland::AccessibilityEvent),
    ProtocolUnavailable,
    Return,
    HighContrast(bool),
    SystemTheme(Box<cosmic::cosmic_theme::Theme>),
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

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        if self.wayland_thread.is_none() {
            match wayland::spawn_wayland_connection() {
                Ok((tx, mut rx)) => {
                    self.wayland_available = true;
                    self.wayland_thread = Some(tx);

                    return cosmic::Task::stream(async_fn_stream::fn_stream(
                        |emitter| async move {
                            while let Some(event) = rx.recv().await {
                                let _ = emitter
                                    .emit(crate::pages::Message::Accessibility(Message::Event(
                                        event,
                                    )))
                                    .await;
                            }

                            let _ = emitter
                                .emit(crate::pages::Message::Accessibility(
                                    Message::ProtocolUnavailable,
                                ))
                                .await;
                        },
                    ));
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
        high_contrast = fl!("accessibility", "high-contrast");
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
                .add(
                    cosmic::Element::from(
                        settings::item::builder(&descriptions[high_contrast])
                            .toggler(page.theme.is_high_contrast, Message::HighContrast),
                    )
                    .map(crate::pages::Message::Accessibility),
                )
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
            Message::SystemTheme(theme) => {
                self.theme = theme;
            }
            Message::HighContrast(enabled) => {
                if self.theme.is_high_contrast == enabled
                    || self.high_contrast.is_some_and(|hc| hc == enabled)
                {
                    return Task::none();
                }
                self.high_contrast = Some(enabled);

                _ = std::thread::spawn(move || {
                    let set_hc = |is_dark: bool| {
                        let builder_config = if is_dark {
                            ThemeBuilder::dark_config()?
                        } else {
                            ThemeBuilder::light_config()?
                        };
                        let mut builder = match ThemeBuilder::get_entry(&builder_config) {
                            Ok(b) => b,
                            Err((errs, b)) => {
                                tracing::warn!("{errs:?}");
                                b
                            }
                        };

                        builder.palette = if is_dark {
                            if enabled {
                                CosmicPalette::HighContrastDark(builder.palette.inner())
                            } else {
                                CosmicPalette::Dark(builder.palette.inner())
                            }
                        } else if enabled {
                            CosmicPalette::HighContrastLight(builder.palette.inner())
                        } else {
                            CosmicPalette::Light(builder.palette.inner())
                        };
                        builder.write_entry(&builder_config)?;

                        let new_theme = builder.build();

                        let theme_config = if is_dark {
                            CosmicTheme::dark_config()?
                        } else {
                            CosmicTheme::light_config()?
                        };

                        new_theme.write_entry(&theme_config)?;

                        Result::<(), cosmic_config::Error>::Ok(())
                    };
                    if let Err(err) = set_hc(true) {
                        tracing::warn!("{err:?}");
                    }
                    if let Err(err) = set_hc(false) {
                        tracing::warn!("{err:?}");
                    }
                });
            }
        }

        cosmic::iced::Task::none()
    }
}
