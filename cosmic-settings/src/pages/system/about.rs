// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::Length;
use cosmic_settings_page::{self as page, section, Section};

use cosmic::widget::{self, editable_input, list_column, settings, text};
use cosmic::{command, Apply, Command};
use cosmic_settings_system::about::Info;
use slotmap::SlotMap;

#[derive(Clone, Debug)]
pub enum Message {
    HostnameEdit(bool),
    HostnameInput(String),
    HostnameSubmit,
    Info(Box<Info>),
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    editing_device_name: bool,
    info: Info,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(device()),
            sections.insert(hardware()),
            sections.insert(os()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("about", "help-about-symbolic")
            .title(fl!("about"))
            .description(fl!("about", "desc"))
    }

    fn on_enter(
        &mut self,
        _page: page::Entity,
        sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    ) -> Command<crate::pages::Message> {
        command::future(async move {
            crate::pages::Message::About(Message::Info(Box::new(Info::load())))
        })
    }
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::HostnameEdit(editing) => {
                self.editing_device_name = editing;
            }

            Message::HostnameInput(hostname) => {
                self.info.device_name = hostname;
            }

            Message::HostnameSubmit => {
                let hostname = &self.info.device_name;
                if hostname_validator::is_valid(hostname) {
                    // TODO: display errors
                    self.editing_device_name = false;
                    let hostname = hostname.clone();
                    tokio::task::spawn(async move {
                        let connection = match zbus::Connection::system().await {
                            Ok(conn) => conn,
                            Err(why) => {
                                tracing::error!(?why, "failed to establish connection to dbus");
                                return;
                            }
                        };

                        let hostname1 = match hostname1_zbus::Hostname1Proxy::new(&connection).await
                        {
                            Ok(proxy) => proxy,
                            Err(why) => {
                                tracing::error!(
                                    ?why,
                                    "failed to connect to org.freedesktop.hostname1"
                                );
                                return;
                            }
                        };

                        if let Err(why) = hostname1.set_static_hostname(&hostname, false).await {
                            tracing::error!(?why, "failed to set static hostname");
                        }
                    });
                }
            }

            Message::Info(info) => self.info = *info,
        }
    }
}

fn device() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![
            fl!("about-device").into(),
            fl!("about-device", "desc").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            let desc = &section.descriptions;

            let hostname_input = editable_input(
                "",
                &page.info.device_name,
                page.editing_device_name,
                Message::HostnameEdit,
            )
            .on_input(Message::HostnameInput)
            .on_submit(Message::HostnameSubmit);

            let hostname_row = widget::row::with_capacity(2)
                .push(widget::horizontal_space(Length::Fill))
                .push(hostname_input);

            let device_name = settings::item::builder(&*desc[0])
                .description(&*desc[1])
                .flex_control(hostname_row);

            list_column()
                .add(device_name)
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::About)
        })
}

fn hardware() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("about-hardware"))
        .descriptions(vec![
            fl!("about-hardware", "model").into(),
            fl!("about-hardware", "memory").into(),
            fl!("about-hardware", "processor").into(),
            fl!("about-hardware", "graphics").into(),
            fl!("about-hardware", "disk-capacity").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            let desc = &section.descriptions;

            let mut sections = settings::view_section(&section.title)
                .add(settings::flex_item(
                    &*desc[0],
                    text(&page.info.hardware_model),
                ))
                .add(settings::flex_item(&*desc[1], text(&page.info.memory)))
                .add(settings::flex_item(&*desc[2], text(&page.info.processor)));

            for card in &page.info.graphics {
                sections = sections.add(settings::flex_item(&*desc[3], text(card.as_str())));
            }

            sections
                .add(settings::flex_item(
                    &*desc[4],
                    text(&page.info.disk_capacity),
                ))
                .into()
        })
}

fn os() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("about-os"))
        .descriptions(vec![
            fl!("about-os", "os").into(),
            fl!("about-os", "os-architecture").into(),
            fl!("about-os", "desktop-environment").into(),
            fl!("about-os", "windowing-system").into(),
        ])
        .view::<Page>(|_binder, page, section| {
            let desc = &section.descriptions;
            settings::view_section(&section.title)
                .add(settings::flex_item(
                    &*desc[0],
                    text(&page.info.operating_system),
                ))
                .add(settings::flex_item(
                    &*desc[1],
                    text(&page.info.os_architecture),
                ))
                .add(settings::flex_item(
                    &*desc[2],
                    text(&page.info.desktop_environment),
                ))
                .add(settings::flex_item(
                    &*desc[3],
                    text(&page.info.windowing_system),
                ))
                .into()
        })
}

// Related settings: for 2nd COSMIC release
// fn related() -> Section<crate::pages::Message> {
//     Section::default()
//         .title(fl!("about-related"))
//         .descriptions(vec![fl!("about-related", "support").into()])
//         .view::<Page>(|_binder, _page, section| {
//             settings::view_section(&section.title)
//                 .add(settings::item(&*section.descriptions[0], text("TODO")))
//                 .into()
//         })
// }

// fn page(app: &crate::SettingsApp) -> &Page {
//     app.pages
//         .resource::<Page>()
//         .expect("missing system->about page")
// }
