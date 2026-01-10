// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod vpn;
pub mod wifi;
pub mod wired;

use std::{ffi::OsStr, process::Stdio, sync::Arc};

use anyhow::Context;
use cosmic::{Apply, Element, Task, widget};
use cosmic_dbus_networkmanager::{
    interface::enums::{DeviceState, DeviceType},
    nm::NetworkManager,
};
use cosmic_settings_network_manager_subscription as network_manager;
use cosmic_settings_page::{self as page, Section, section};
use futures::{SinkExt, StreamExt};
use secure_string::SecureString;
use slotmap::SlotMap;

pub type SecretSender = Arc<tokio::sync::Mutex<Option<tokio::sync::oneshot::Sender<SecureString>>>>;

static NM_CONNECTION_EDITOR: &str = "nm-connection-editor";

#[derive(Debug, Default)]
pub struct Page {
    entity: page::Entity,
    nm_task: Option<tokio::sync::oneshot::Sender<()>>,
    devices: Vec<Arc<network_manager::devices::DeviceInfo>>,
    vpn: page::Entity,
    wifi: page::Entity,
    wired: page::Entity,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// An error occurred.
    Error(String),
    /// Successfully connected to the system dbus.
    NetworkManagerConnect(zbus::Connection),
    /// Open the wifi settings page with the selected device.
    OpenPage {
        page: page::Entity,
        device: Option<DeviceVariant>,
    },
    /// Update the devices lists
    UpdateDevices(Vec<Arc<network_manager::devices::DeviceInfo>>),
}

#[derive(Debug, Clone)]
pub enum DeviceVariant {
    Wired(Arc<network_manager::devices::DeviceInfo>),
    WiFi(Arc<network_manager::devices::DeviceInfo>),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Networking(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Networking(message)
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> cosmic_settings_page::Info {
        page::Info::new(
            "network-and-wireless",
            "preferences-network-and-wireless-symbolic",
        )
        .title(fl!("network-and-wireless"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        crate::slab!(descriptions {
            wifi_desc = fl!("connections-and-profiles", variant = "wifi");
            wired_desc = fl!("connections-and-profiles", variant = "wired");
            vpn_desc = fl!("connections-and-profiles", variant = "vpn");
        });

        let device_list = Section::default().descriptions(descriptions).view::<Self>(
            move |_binder, page, section| {
                let descs = &section.descriptions;

                let multiple_wifi_adapters = page
                    .devices
                    .iter()
                    .filter(|device| device.device_type == DeviceType::Wifi)
                    .count()
                    > 1;
                let multiple_wired_adapters = page
                    .devices
                    .iter()
                    .filter(|device| device.device_type == DeviceType::Ethernet)
                    .count()
                    > 1;

                let wifi_devices = page
                    .devices
                    .iter()
                    .filter(|device| device.device_type == DeviceType::Wifi)
                    .map(|device| {
                        crate::widget::page_list_item(
                            if multiple_wifi_adapters {
                                fl!("wifi", "adapter", id = device.interface.as_str())
                            } else {
                                fl!("wifi")
                            },
                            if multiple_wifi_adapters {
                                ""
                            } else {
                                &descs[wifi_desc]
                            },
                            match device.state {
                                DeviceState::Activated => fl!("network-device-state", "activated"),
                                DeviceState::Config => fl!("network-device-state", "config"),
                                DeviceState::Deactivating => {
                                    fl!("network-device-state", "deactivating")
                                }
                                DeviceState::Disconnected => {
                                    fl!("network-device-state", "disconnected")
                                }
                                DeviceState::Failed => fl!("network-device-state", "failed"),
                                DeviceState::IpCheck => fl!("network-device-state", "ip-check"),
                                DeviceState::IpConfig => fl!("network-device-state", "ip-config"),
                                DeviceState::NeedAuth => fl!("network-device-state", "need-auth"),
                                DeviceState::Prepare => fl!("network-device-state", "prepare"),
                                DeviceState::Secondaries => {
                                    fl!("network-device-state", "secondaries")
                                }
                                DeviceState::Unavailable => {
                                    fl!("network-device-state", "unavailable")
                                }
                                DeviceState::Unknown => fl!("network-device-state", "unknown"),
                                DeviceState::Unmanaged => fl!("network-device-state", "unmanaged"),
                            },
                            "preferences-wireless-symbolic",
                            Message::OpenPage {
                                page: page.wifi,
                                device: Some(DeviceVariant::WiFi(device.clone())),
                            },
                        )
                    });

                let wired_devices = page
                    .devices
                    .iter()
                    .filter(|device| device.device_type == DeviceType::Ethernet)
                    .map(|device| {
                        crate::widget::page_list_item(
                            if multiple_wired_adapters {
                                fl!("wired", "adapter", id = device.interface.as_str())
                            } else {
                                fl!("wired")
                            },
                            if multiple_wired_adapters {
                                ""
                            } else {
                                &descs[wired_desc]
                            },
                            match device.state {
                                DeviceState::Activated => fl!("network-device-state", "activated"),
                                DeviceState::Config => fl!("network-device-state", "config"),
                                DeviceState::Deactivating => {
                                    fl!("network-device-state", "deactivating")
                                }
                                DeviceState::Disconnected => {
                                    fl!("network-device-state", "disconnected")
                                }
                                DeviceState::Failed => fl!("network-device-state", "failed"),
                                DeviceState::IpCheck => fl!("network-device-state", "ip-check"),
                                DeviceState::IpConfig => fl!("network-device-state", "ip-config"),
                                DeviceState::NeedAuth => fl!("network-device-state", "need-auth"),
                                DeviceState::Prepare => fl!("network-device-state", "prepare"),
                                DeviceState::Secondaries => {
                                    fl!("network-device-state", "secondaries")
                                }
                                DeviceState::Unavailable => {
                                    fl!("network-device-state", "unplugged")
                                }
                                DeviceState::Unknown => fl!("network-device-state", "unknown"),
                                DeviceState::Unmanaged => fl!("network-device-state", "unmanaged"),
                            },
                            "preferences-wired-symbolic",
                            Message::OpenPage {
                                page: page.wired,
                                device: Some(DeviceVariant::Wired(device.clone())),
                            },
                        )
                    });

                let device_list = wifi_devices
                    .chain(wired_devices)
                    .fold(widget::column(), |column, device| column.push(device))
                    .push(crate::widget::page_list_item(
                        fl!("vpn"),
                        &descs[vpn_desc],
                        "",
                        "preferences-vpn-symbolic",
                        Message::OpenPage {
                            page: page.vpn,
                            device: None,
                        },
                    ))
                    .spacing(cosmic::theme::active().cosmic().spacing.space_s);

                Element::from(device_list).map(crate::pages::Message::Networking)
            },
        );

        Some(vec![sections.insert(device_list)])
    }

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        if self.nm_task.is_none() {
            return cosmic::Task::future(async move {
                zbus::Connection::system()
                    .await
                    .context("failed to create system dbus connection")
                    .map_or_else(
                        |why| Message::Error(why.to_string()),
                        Message::NetworkManagerConnect,
                    )
                    .apply(crate::pages::Message::Networking)
            });
        }

        Task::none()
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        self.devices = Vec::new();

        if let Some(cancel) = self.nm_task.take() {
            _ = cancel.send(());
        }

        Task::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: cosmic_settings_page::Insert<crate::pages::Message>,
    ) -> cosmic_settings_page::Insert<crate::pages::Message> {
        let vpn = page.sub_page_with_id::<vpn::Page>();
        let wifi = page.sub_page_with_id::<wifi::Page>();
        let wired = page.sub_page_with_id::<wired::Page>();

        let model = page.model.page_mut::<Self>().unwrap();
        model.vpn = vpn;
        model.wifi = wifi;
        model.wired = wired;

        page
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        let span = tracing::span!(tracing::Level::INFO, "networking::update");
        let _span = span.enter();

        match message {
            Message::NetworkManagerConnect(conn) => {
                return self.connect(conn.clone());
            }

            Message::Error(why) => {
                tracing::error!(why);
            }

            Message::OpenPage { page, device } => {
                let mut tasks = Vec::<Task<crate::app::Message>>::new();

                tasks.push(cosmic::task::message(crate::app::Message::Page(page)));

                if let Some(device) = device {
                    tasks.push(cosmic::task::message(crate::app::Message::PageMessage(
                        match device {
                            DeviceVariant::WiFi(device) => {
                                crate::pages::Message::WiFi(wifi::Message::SelectDevice(device))
                            }
                            DeviceVariant::Wired(device) => {
                                crate::pages::Message::Wired(wired::Message::SelectDevice(device))
                            }
                        },
                    )));
                }

                return cosmic::Task::batch(tasks);
            }

            Message::UpdateDevices(devices) => {
                self.devices = devices;
            }
        }

        Task::none()
    }

    fn connect(&mut self, conn: zbus::Connection) -> Task<crate::app::Message> {
        if self.nm_task.is_none() {
            let (canceller, task) =
                crate::utils::forward_event_loop(move |mut sender| async move {
                    let network_manager = match NetworkManager::new(&conn).await {
                        Ok(n) => n,
                        Err(why) => {
                            tracing::error!(
                                why = why.to_string(),
                                "failed to connect to network_manager"
                            );

                            return futures::future::pending().await;
                        }
                    };

                    let mut devices_changed =
                        std::pin::pin!(network_manager.receive_devices_changed().await.then(
                            |_| async {
                                match network_manager::devices::list(&conn, |_| true).await {
                                    Ok(devices) => Message::UpdateDevices(
                                        devices.into_iter().map(Arc::new).collect(),
                                    ),
                                    Err(why) => Message::Error(why.to_string()),
                                }
                            }
                        ));

                    while let Some(message) = devices_changed.next().await {
                        _ = sender
                            .send(crate::pages::Message::Networking(message))
                            .await;
                    }
                });

            self.nm_task = Some(canceller);
            return task.map(crate::app::Message::from);
        }

        Task::none()
    }
}

async fn nm_add_vpn_file<P: AsRef<OsStr>>(type_: &str, path: P) -> Result<(), String> {
    tokio::process::Command::new("nmcli")
        .args(["connection", "import", "type", type_, "file"])
        .arg(path)
        .stderr(Stdio::piped())
        .output()
        .await
        .apply(crate::utils::map_stderr_output)
}

async fn nm_add_wired() -> Result<(), String> {
    nm_connection_editor(&["--type=802-3-ethernet", "-c"]).await
}

async fn nm_add_wifi() -> Result<(), String> {
    nm_connection_editor(&["--type=802-11-wireless", "-c"]).await
}

async fn nm_edit_connection(uuid: &str) -> Result<(), String> {
    nm_connection_editor(&[&["--edit=", uuid].concat()]).await
}

async fn nm_connection_editor(args: &[&str]) -> Result<(), String> {
    tokio::process::Command::new(NM_CONNECTION_EDITOR)
        .args(args)
        .stderr(Stdio::piped())
        .output()
        .await
        .apply(crate::utils::map_stderr_output)
}
