use cosmic::{
    Task,
    cosmic_theme::{CosmicPalette, ThemeBuilder},
    iced_core::text::Wrapping,
    surface,
    theme::CosmicTheme,
    widget::{dropdown, settings, text, toggler},
};
pub use cosmic_comp_config::ZoomMovement;
use cosmic_config::CosmicConfigEntry;
use cosmic_settings_daemon_config::CosmicSettingsDaemonConfig;
use cosmic_settings_page::{
    self as page, Insert,
    section::{self, Section},
};
use cosmic_settings_subscriptions::accessibility::{self, DBusRequest, DBusUpdate};
use cosmic_settings_subscriptions::cosmic_a11y_manager;
use num_traits::FromPrimitive;
use slotmap::SlotMap;

pub mod magnifier;
pub use cosmic_a11y_manager::{AccessibilityEvent, AccessibilityRequest, ColorFilter};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub struct Page {
    entity: page::Entity,
    magnifier_state: bool,
    screen_inverted: bool,
    screen_filter_active: bool,
    screen_filter_selection: ColorFilter,
    screen_filter_selections: Vec<String>,

    wayland_available: Option<u32>,
    wayland_thread: Option<cosmic_a11y_manager::Sender>,
    theme: Box<cosmic::cosmic_theme::Theme>,
    high_contrast: Option<bool>,
    daemon_config: CosmicSettingsDaemonConfig,
    daemon_helper: cosmic_config::Config,
    dbus_sender: Option<UnboundedSender<DBusRequest>>,
    reader_enabled: bool,
}

impl Default for Page {
    fn default() -> Self {
        let daemon_helper = CosmicSettingsDaemonConfig::config().unwrap();
        Page {
            dbus_sender: None,
            entity: page::Entity::default(),
            magnifier_state: false,
            screen_inverted: false,
            screen_filter_active: false,
            screen_filter_selection: ColorFilter::Greyscale,
            screen_filter_selections: vec![
                // This order has to match the representation of `ColorFilter`
                fl!("color-filter", "greyscale"),
                fl!("color-filter", "deuteranopia"),
                fl!("color-filter", "protanopia"),
                fl!("color-filter", "tritanopia"),
                fl!("color-filter", "unknown"),
            ],

            wayland_available: None,
            wayland_thread: None,
            theme: Box::default(),
            high_contrast: None,
            daemon_config: CosmicSettingsDaemonConfig::get_entry(&daemon_helper)
                .unwrap_or_default(),
            daemon_helper,
            reader_enabled: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(cosmic_a11y_manager::AccessibilityEvent),
    ProtocolUnavailable,
    Return,
    HighContrast(bool),
    SystemTheme(Box<cosmic::cosmic_theme::Theme>),
    SetScreenInverted(bool),
    SetScreenFilterActive(bool),
    SetScreenFilterSelection(ColorFilter),
    Surface(surface::Action),
    SetSoundMono(bool),
    DBusUpdate(DBusUpdate),
    ScreenReaderEnabled(bool),
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
        Some(vec![sections.insert(vision()), sections.insert(hearing())])
    }

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        if self.wayland_thread.is_none() {
            match cosmic_a11y_manager::spawn_wayland_connection(1) {
                Ok((tx, mut rx)) => {
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
                    self.wayland_available = None;
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
        screen_reader = fl!("accessibility", "screen-reader");
        magnifier = fl!("magnifier");
        vision = fl!("accessibility", "vision");
        on = fl!("accessibility", "on");
        off = fl!("accessibility", "off");
        unavailable = fl!("accessibility", "unavailable");
        high_contrast = fl!("accessibility", "high-contrast");
        invert_colors = fl!("accessibility", "invert-colors");
        color_filters = fl!("accessibility", "color-filters");
        color_filter_type = fl!("color-filter");
    });

    Section::default()
        .title(&descriptions[vision])
        .descriptions(descriptions)
        .view::<Page>(move |binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(
                    cosmic::Element::from(
                        settings::item::builder(&descriptions[screen_reader])
                            .toggler(page.reader_enabled, Message::ScreenReaderEnabled),
                    )
                    .map(crate::pages::Message::Accessibility),
                )
                .add({
                    let (magnifier_entity, _magnifier_info) = binder
                        .info
                        .iter()
                        .find(|(_, v)| v.id == "accessibility_magnifier")
                        .expect("magnifier page not found");

                    let status_text = if page.wayland_available.is_some() {
                        if page.magnifier_state {
                            &descriptions[on]
                        } else {
                            &descriptions[off]
                        }
                    } else {
                        &descriptions[unavailable]
                    };

                    crate::widget::go_next_with_item(
                        &descriptions[magnifier],
                        text::body(status_text).wrapping(Wrapping::Word),
                        page.wayland_available
                            .is_some()
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
                .add(
                    cosmic::Element::from(
                        settings::item::builder(&descriptions[invert_colors]).control(
                            toggler(page.screen_inverted).on_toggle_maybe(
                                page.wayland_available
                                    .is_some_and(|ver| ver >= 2)
                                    .then_some(Message::SetScreenInverted),
                            ),
                        ),
                    )
                    .map(crate::pages::Message::Accessibility),
                )
                .add({
                    cosmic::Element::from(
                        settings::item::builder(&descriptions[color_filters]).control(
                            toggler(page.screen_filter_active).on_toggle_maybe(
                                page.wayland_available
                                    .is_some_and(|ver| ver >= 2)
                                    .then_some(Message::SetScreenFilterActive),
                            ),
                        ),
                    )
                    .map(crate::pages::Message::Accessibility)
                })
                .add({
                    let selections = if page.screen_filter_selection == ColorFilter::Unknown {
                        &page.screen_filter_selections
                    } else {
                        &page.screen_filter_selections[0..4]
                    };
                    cosmic::Element::from(
                        settings::item::builder(&descriptions[color_filter_type]).control(
                            dropdown::popup_dropdown(
                                selections,
                                Some(page.screen_filter_selection as usize),
                                move |idx| {
                                    let filter = ColorFilter::from_usize(idx).unwrap_or_default();
                                    Message::SetScreenFilterSelection(filter)
                                },
                                cosmic::iced::window::Id::RESERVED,
                                Message::Surface,
                                |a| {
                                    crate::app::Message::PageMessage(
                                        crate::pages::Message::Accessibility(a),
                                    )
                                },
                            ),
                        ),
                    )
                    .map(crate::pages::Message::Accessibility)
                })
                .into()
        })
}

pub fn hearing() -> section::Section<crate::pages::Message> {
    crate::slab!(descriptions {
        hearing = fl!("hearing");
        mono = fl!("hearing", "mono");
    });

    Section::default()
        .title(&descriptions[hearing])
        .descriptions(descriptions)
        .view::<Page>(move |_, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(
                    cosmic::Element::from(
                        settings::item::builder(&descriptions[mono])
                            .toggler(page.daemon_config.mono_sound, Message::SetSoundMono),
                    )
                    .map(crate::pages::Message::Accessibility),
                )
                .into()
        })
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::iced::Task<crate::app::Message> {
        match message {
            Message::Event(AccessibilityEvent::Bound(version)) => {
                self.wayland_available = Some(version);
            }
            Message::Event(AccessibilityEvent::Magnifier(value)) => {
                self.magnifier_state = value;
            }
            Message::Event(AccessibilityEvent::ScreenFilter { inverted, filter }) => {
                self.screen_inverted = inverted;
                self.screen_filter_active = filter.is_some();
                if let Some(filter) = filter {
                    self.screen_filter_selection = filter;
                }
            }
            Message::Event(AccessibilityEvent::Closed) | Message::ProtocolUnavailable => {
                self.wayland_available = None;
                self.screen_filter_active = false;
            }
            Message::Return => {
                return cosmic::iced::Task::done(crate::app::Message::Page(self.entity));
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
            Message::SetScreenInverted(inverted) => {
                if let Some(sender) = self.wayland_thread.as_ref() {
                    let _ = sender.send(AccessibilityRequest::ScreenFilter {
                        inverted,
                        filter: Some(cosmic_a11y_manager::ColorFilter::Unknown),
                    });
                }
            }
            Message::SetScreenFilterActive(active) => {
                if let Some(sender) = self.wayland_thread.as_ref() {
                    let _ = sender.send(AccessibilityRequest::ScreenFilter {
                        inverted: self.screen_inverted,
                        filter: active.then_some(self.screen_filter_selection),
                    });
                }
            }
            Message::SetScreenFilterSelection(filter) => {
                if self.screen_filter_active && self.wayland_available.is_some_and(|ver| ver >= 2) {
                    if let Some(sender) = self.wayland_thread.as_ref() {
                        let _ = sender.send(AccessibilityRequest::ScreenFilter {
                            inverted: self.screen_inverted,
                            filter: Some(filter),
                        });
                    }
                } else {
                    self.screen_filter_selection = filter;
                }
            }
            Message::Surface(a) => {
                return cosmic::task::message(crate::app::Message::Surface(a));
            }
            Message::SetSoundMono(active) => {
                if let Err(err) = self
                    .daemon_config
                    .set_mono_sound(&self.daemon_helper, active)
                {
                    tracing::error!("{err:?}");
                }
            }
            Message::DBusUpdate(update) => match update {
                DBusUpdate::Error(err) => {
                    tracing::error!("{err}");
                    let _ = self.dbus_sender.take();
                    self.reader_enabled = false;
                }
                DBusUpdate::Status(enabled) => {
                    self.reader_enabled = enabled;
                }
                DBusUpdate::Init(enabled, tx) => {
                    self.reader_enabled = enabled;
                    self.dbus_sender = Some(tx);
                }
            },
            Message::ScreenReaderEnabled(enabled) => {
                if let Some(tx) = &self.dbus_sender {
                    self.reader_enabled = enabled;
                    let _ = tx.send(DBusRequest::Status(enabled));
                } else {
                    self.reader_enabled = false;
                }
            }
        }
        cosmic::iced::Task::none()
    }
}
