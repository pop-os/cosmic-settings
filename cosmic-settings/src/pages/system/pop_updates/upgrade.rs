// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::Page;
use cosmic::{Apply, Element, Task, widget};
use cosmic_settings_page::{self as page, Section};
use pop_upgrade_client::ClientProxy;

pub fn is_development() -> bool {
    std::env::var("S76_TEST").is_ok_and(|v| v == "1")
}

pub fn section() -> page::Section<crate::pages::Message> {
    crate::slab!(descriptions {
        header_text = fl!("os-upgrade");
    });

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let desc = &section.descriptions;

            let mut section = widget::settings::section().title(&desc[header_text]);

            if let Some(ref release_info) = page.pop_upgrade.release_info {
                if release_info.build >= 0 {
                    let label = fl!("os-upgrade", "available", version = (&*release_info.next));
                    let button = widget::button::text(fl!("update-button"))
                        .on_press(super::Message::PopUpgrade(Message::Upgrade));
                    section = section.add(widget::settings::item::builder(label).control(button));
                } else {
                    let label = fl!("os-upgrade", "current", os = "Pop!_OS");
                    section = section
                        .add(widget::settings::item::builder(label).control(widget::row([])));
                }
            }

            section
                .apply(Element::from)
                .map(crate::pages::Message::SystemUpdater)
        })
}

pub async fn client() -> Message {
    let connection = match zbus::Connection::system().await {
        Ok(conn) => conn,
        Err(why) => {
            return Message::Error(ErrorKind::DBusConnect, format!("{why:?}"));
        }
    };

    let client = match pop_upgrade_client::ClientProxy::new(&connection).await {
        Ok(client) => client,
        Err(why) => {
            return Message::Error(ErrorKind::DBusProxy, format!("{why:?}"));
        }
    };

    let release_info = match client.release_check(is_development()).await {
        Ok((current, next, build, urgent, is_lts)) => ReleaseInfo {
            current,
            next,
            build,
            urgent: if urgent > -1 {
                Some(urgent as u16)
            } else {
                None
            },
            is_lts,
        },
        Err(why) => return Message::Error(ErrorKind::ReleaseCheck, format!("{why:?}")),
    };

    let recovery_info = match client.recovery_version().await {
        Ok((version, build)) => Some(RecoveryInfo { version, build }),
        Err(why) => None,
    };

    Message::Connect {
        client,
        release_info,
        recovery_info,
    }
}

#[derive(Default)]
pub struct Model {
    client: Option<ClientProxy<'static>>,
    release_info: Option<ReleaseInfo>,
    recovery_info: Option<RecoveryInfo>,
}

impl Model {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::Connect {
                client,
                release_info,
                recovery_info,
            } => {
                self.client = Some(client);
                self.release_info = Some(release_info);
                self.recovery_info = recovery_info;
            }

            Message::Upgrade => {}

            Message::Error(kind, why) => {
                eprintln!("error {kind:?}: {why}");
            }
        }

        Task::none()
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Connect {
        client: ClientProxy<'static>,
        release_info: ReleaseInfo,
        recovery_info: Option<RecoveryInfo>,
    },
    Error(ErrorKind, String),
    Upgrade,
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::SystemUpdater(super::Message::PopUpgrade(message)).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::SystemUpdater(super::Message::PopUpgrade(message))
    }
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    DBusConnect,
    DBusProxy,
    ReleaseCheck,
}

/// The version of the recovery partition's image.
#[derive(Clone, Debug)]
pub struct RecoveryInfo {
    pub version: Box<str>,
    pub build: i16,
}

/// Information about the current and next release.
///
/// The build is set to `-1` if the next release is
/// not available.
#[derive(Clone, Debug)]
pub struct ReleaseInfo {
    pub current: Box<str>,
    pub next: Box<str>,
    pub build: i16,
    pub urgent: Option<u16>,
    pub is_lts: bool,
}
