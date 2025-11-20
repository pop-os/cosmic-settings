// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic_settings_page::{self as page, Section, section};

use super::info::Info;
use cosmic::widget::{editable_input, list_column, settings, text};
use cosmic::{Apply, Task};
use slab::Slab;
use slotmap::SlotMap;

#[derive(Clone, Debug)]
pub enum Message {
    Error(String),
    HostnameEdit(bool),
    HostnameInput(String),
    HostnameSubmit,
    HostnameSuccess(String),
    Info(Box<Info>),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::About(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::About(message)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    entity: page::Entity,
    editing_device_name: bool,
    hostname_input: String,
    info: Info,
    on_enter_handle: Option<cosmic::iced::task::Handle>,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

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

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        let (task, handle) = Task::future(async move {
            crate::pages::Message::About(Message::Info(Box::new(Info::load())))
        })
        .abortable();

        self.on_enter_handle = Some(handle);
        task
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }

        Task::none()
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::app::Task<crate::Message> {
        match message {
            Message::HostnameEdit(editing) => {
                self.editing_device_name = editing;
            }

            Message::HostnameInput(hostname) => {
                self.hostname_input = hostname;
            }

            Message::HostnameSubmit => return self.hostname_submit(),

            Message::Info(info) => {
                self.info = *info;
                self.hostname_input = self.info.device_name.clone();
            }

            Message::Error(_why) => {
                self.hostname_input = self.info.device_name.clone();
                // TODO: display errors
            }

            Message::HostnameSuccess(name) => {
                self.info.device_name = name;
            }
        }

        Task::none()
    }

    fn hostname_submit(&mut self) -> cosmic::app::Task<crate::app::Message> {
        if self.hostname_input == self.info.device_name {
            return Task::none();
        }

        if !hostname_validator::is_valid(&self.hostname_input) {
            return Task::none();
        }

        self.editing_device_name = false;
        let hostname = self.hostname_input.clone();

        cosmic::Task::future(async move { set_hostname(hostname).await })
            .map(crate::app::Message::from)
            .map(Into::into)
    }
}

/// Sets the system hostname via D-Bus.
async fn set_hostname(hostname: String) -> Message {
    match set_hostname_impl(&hostname).await {
        Ok(()) => Message::HostnameSuccess(hostname),
        Err(err) => {
            tracing::error!("failed to set hostname: {}", err);
            Message::Error(err)
        }
    }
}

/// Implementation of hostname setting that uses Result for cleaner error handling.
async fn set_hostname_impl(hostname: &str) -> Result<(), String> {
    let connection = zbus::Connection::system()
        .await
        .map_err(|e| format!("failed to establish connection to dbus: {}", e))?;

    let hostname1 = hostname1_zbus::Hostname1Proxy::new(&connection)
        .await
        .map_err(|e| format!("failed to connect to org.freedesktop.hostname1: {}", e))?;

    hostname1
        .set_static_hostname(hostname, false)
        .await
        .map_err(|e| format!("failed to set static hostname: {}", e))?;

    Ok(())
}

fn device() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let device = descriptions.insert(fl!("about-device"));
    let device_desc = descriptions.insert(fl!("about-device", "desc"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let desc = &section.descriptions;

            let hostname_input = editable_input(
                "",
                &page.hostname_input,
                page.editing_device_name,
                Message::HostnameEdit,
            )
            .width(250)
            .on_input(Message::HostnameInput)
            .on_unfocus(Message::HostnameSubmit)
            .on_submit(|_| Message::HostnameSubmit);

            let device_name = settings::item::builder(&*desc[device])
                .description(&*desc[device_desc])
                .flex_control(hostname_input);

            list_column()
                .add(device_name)
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::About)
        })
}

fn hardware() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let model = descriptions.insert(fl!("about-hardware", "model"));
    let memory = descriptions.insert(fl!("about-hardware", "memory"));
    let processor = descriptions.insert(fl!("about-hardware", "processor"));
    let graphics = descriptions.insert(fl!("about-hardware", "graphics"));
    let disk_capacity = descriptions.insert(fl!("about-hardware", "disk-capacity"));

    Section::default()
        .title(fl!("about-hardware"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let desc = &section.descriptions;

            let mut section_builder = settings::section()
                .title(&section.title)
                .add(settings::flex_item(
                    &*desc[model],
                    text::body(&page.info.hardware_model),
                ))
                .add(settings::flex_item(
                    &*desc[memory],
                    text::body(&page.info.memory),
                ))
                .add(settings::flex_item(
                    &*desc[processor],
                    text::body(&page.info.processor),
                ));

            for card in &page.info.graphics {
                section_builder = section_builder.add(settings::flex_item(
                    &*desc[graphics],
                    text::body(card.as_str()),
                ));
            }

            section_builder
                .add(settings::flex_item(
                    &*desc[disk_capacity],
                    text::body(&page.info.disk_capacity),
                ))
                .into()
        })
}

fn os() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let os = descriptions.insert(fl!("about-os", "os"));
    let os_arch = descriptions.insert(fl!("about-os", "os-architecture"));
    let kernel = descriptions.insert(fl!("about-os", "kernel"));
    let desktop = descriptions.insert(fl!("about-os", "desktop-environment"));
    let windowing_system = descriptions.insert(fl!("about-os", "windowing-system"));

    Section::default()
        .title(fl!("about-os"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let desc = &section.descriptions;
            settings::section()
                .title(&section.title)
                .add(settings::flex_item(
                    &*desc[os],
                    text::body(&page.info.operating_system),
                ))
                .add(settings::flex_item(
                    &*desc[os_arch],
                    text::body(&page.info.os_architecture),
                ))
                .add(settings::flex_item(
                    &*desc[kernel],
                    text::body(&page.info.kernel_version),
                ))
                .add(settings::flex_item(
                    &*desc[desktop],
                    text::body(&page.info.desktop_environment),
                ))
                .add(settings::flex_item(
                    &*desc[windowing_system],
                    text::body(&page.info.windowing_system),
                ))
                .into()
        })
}

// Related settings: for 2nd COSMIC release
// fn related() -> Section<crate::pages::Message> {
//     Section::default()
//         .title(fl!("about-related"))
//         .descriptions(vec![fl!("about-related", "support").into()])
//         .view::<Page>(move |_binder, _page, section| {
//             settings::section().title(&section.title)
//                 .add(settings::item(&*section.descriptions[0], text::body("TODO")))
//                 .into()
//         })
// }

// fn page(app: &crate::SettingsApp) -> &Page {
//     app.pages
//         .resource::<Page>()
//         .expect("missing system->about page")
// }
