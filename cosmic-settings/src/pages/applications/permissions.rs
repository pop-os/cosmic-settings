// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::{self, Alignment, Length, stream};
use cosmic::widget::{self, button, icon, settings, text};
use cosmic::{Element, Task};
use cosmic_settings_page::section::Entity;
use cosmic_settings_page::{self as page, Content, Info, Section};
use futures::{SinkExt, StreamExt};
use slotmap::SlotMap;
use std::collections::HashMap;
use tracing::error;
use zbus::proxy;

/// Permission store tables holding persistent grants made through the
/// ScreenCast and RemoteDesktop portals ("remember this selection").
const TABLES: &[&str] = &["screencast", "remote-desktop"];

#[proxy(
    interface = "org.freedesktop.impl.portal.PermissionStore",
    default_service = "org.freedesktop.impl.portal.PermissionStore",
    default_path = "/org/freedesktop/impl/portal/PermissionStore"
)]
trait PermissionStore {
    fn list(&self, table: &str) -> zbus::Result<Vec<String>>;

    fn lookup(
        &self,
        table: &str,
        id: &str,
    ) -> zbus::Result<(HashMap<String, Vec<String>>, zbus::zvariant::OwnedValue)>;

    fn delete_permission(&self, table: &str, id: &str, app: &str) -> zbus::Result<()>;

    #[zbus(signal)]
    fn changed(
        &self,
        table: &str,
        id: &str,
        deleted: bool,
        data: zbus::zvariant::Value<'_>,
        permissions: HashMap<String, Vec<String>>,
    ) -> zbus::Result<()>;
}

/// A single (table, id, app) entry from the permission store.
#[derive(Clone, Debug)]
pub struct Grant {
    table: &'static str,
    id: String,
    app: String,
    name: Option<String>,
    icon: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Loaded(Vec<Grant>),
    Refresh,
    Revoke(usize),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Permissions(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Permissions(message)
    }
}

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
    on_enter_handle: Option<cosmic::iced::task::Handle>,
    grants: Vec<Grant>,
    loaded: bool,
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> Info {
        page::Info::new("app-permissions", "preferences-system-privacy-symbolic")
            .title(fl!("app-permissions"))
            .description(fl!("app-permissions", "desc"))
    }

    fn content(
        &self,
        sections: &mut SlotMap<Entity, Section<crate::pages::Message>>,
    ) -> Option<Content> {
        Some(vec![sections.insert(grants())])
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        let (task, on_enter_handle) =
            Task::future(async move { Message::Loaded(load_grants().await).into() }).abortable();

        self.on_enter_handle = Some(on_enter_handle);

        task
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }

        Task::none()
    }

    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn subscription(&self, _core: &cosmic::Core) -> iced::Subscription<crate::pages::Message> {
        // Refresh the page whenever a grant in one of the displayed tables is
        // added, modified, or revoked elsewhere (portal dialogs, CLI, etc.).
        iced::Subscription::run(|| {
            stream::channel(
                1,
                |mut sender: futures::channel::mpsc::Sender<crate::pages::Message>| async move {
                    let changes = async {
                        let connection = zbus::Connection::session().await?;
                        let store = PermissionStoreProxy::new(&connection).await?;
                        store.receive_changed().await
                    }
                    .await;

                    let mut changes = match changes {
                        Ok(changes) => changes,
                        Err(why) => {
                            error!(?why, "failed to watch the XDG permission store");
                            return;
                        }
                    };

                    while let Some(signal) = changes.next().await {
                        let relevant = signal
                            .args()
                            .is_ok_and(|args| TABLES.contains(args.table()));

                        if relevant {
                            _ = sender.send(Message::Refresh.into()).await;
                        }
                    }
                },
            )
        })
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::Message> {
        match message {
            Message::Loaded(grants) => {
                self.grants = grants;
                self.loaded = true;
            }

            Message::Refresh => {
                return Task::future(async move { Message::Loaded(load_grants().await).into() });
            }

            Message::Revoke(idx) => {
                let Some(grant) = self.grants.get(idx).cloned() else {
                    return Task::none();
                };

                return Task::future(async move {
                    let result = async {
                        let connection = zbus::Connection::session().await?;
                        let store = PermissionStoreProxy::new(&connection).await?;
                        store
                            .delete_permission(grant.table, &grant.id, &grant.app)
                            .await
                    }
                    .await;

                    if let Err(why) = result {
                        error!(?why, "failed to revoke permission");
                    }

                    Message::Loaded(load_grants().await).into()
                });
            }
        }

        Task::none()
    }
}

/// Reads all grants from the permission store tables shown by this page.
async fn load_grants() -> Vec<Grant> {
    match fetch_grants().await {
        Ok(grants) => grants,
        Err(why) => {
            error!(?why, "failed to read the XDG permission store");
            Vec::new()
        }
    }
}

async fn fetch_grants() -> zbus::Result<Vec<Grant>> {
    let connection = zbus::Connection::session().await?;
    let store = PermissionStoreProxy::new(&connection).await?;

    let mut grants = Vec::new();

    for &table in TABLES {
        // A table which has never been written to does not exist yet.
        let Ok(ids) = store.list(table).await else {
            continue;
        };

        for id in ids {
            let Ok((permissions, _data)) = store.lookup(table, &id).await else {
                continue;
            };

            for app in permissions.into_keys() {
                grants.push(Grant {
                    table,
                    id: id.clone(),
                    app,
                    name: None,
                    icon: None,
                });
            }
        }
    }

    if !grants.is_empty() {
        let locales = freedesktop_desktop_entry::get_languages_from_env();
        let entries =
            freedesktop_desktop_entry::Iter::new(freedesktop_desktop_entry::default_paths())
                .entries(Some(&locales));

        for entry in entries {
            let app_id = entry.flatpak().unwrap_or(entry.appid.as_ref());

            for grant in grants.iter_mut().filter(|grant| grant.app == app_id) {
                if grant.name.is_none() {
                    grant.name = entry.name(&locales).map(|name| name.into_owned());
                    grant.icon = entry.icon().map(String::from);
                }
            }
        }
    }

    grants.sort_by(|a, b| {
        a.table
            .cmp(b.table)
            .then_with(|| a.app.cmp(&b.app))
            .then_with(|| a.id.cmp(&b.id))
    });

    Ok(grants)
}

fn table_title(table: &str) -> String {
    match table {
        "screencast" => fl!("app-permissions", "screencast"),
        "remote-desktop" => fl!("app-permissions", "remote-desktop"),
        other => other.to_owned(),
    }
}

fn grants() -> Section<crate::pages::Message> {
    let cosmic::cosmic_theme::Spacing {
        space_xxs,
        space_xs,
        ..
    } = cosmic::theme::spacing();

    Section::default()
        .title(fl!("app-permissions"))
        .view::<Page>(move |_binder, page, _section| {
            let mut view = widget::column::with_capacity(TABLES.len()).spacing(space_xxs);

            if page.loaded && page.grants.is_empty() {
                view = view.push(text::body(fl!("app-permissions", "none")));
            }

            for &table in TABLES {
                let mut section = settings::section().title(table_title(table));
                let mut empty = true;

                for (idx, grant) in page
                    .grants
                    .iter()
                    .enumerate()
                    .filter(|(_, grant)| grant.table == table)
                {
                    empty = false;

                    let name: Element<crate::pages::Message> = match &grant.name {
                        Some(name) => widget::column::with_capacity(2)
                            .push(text::body(name))
                            .push(text::caption(&grant.app))
                            .into(),
                        None if grant.app.is_empty() => {
                            text::body(fl!("app-permissions", "unknown-app")).into()
                        }
                        None => text::body(&grant.app).into(),
                    };

                    let row = widget::row::with_capacity(3)
                        .spacing(space_xs)
                        .align_y(Alignment::Center)
                        .push(
                            icon::from_name(grant.icon.as_deref().unwrap_or("application-default"))
                                .size(32),
                        )
                        .push(widget::container(name).width(Length::Fill))
                        .push(
                            button::icon(icon::from_name("edit-delete-symbolic"))
                                .extra_small()
                                .on_press(Message::Revoke(idx).into()),
                        );

                    section = section.add(row);
                }

                if !empty {
                    view = view.push(section);
                }
            }

            view.into()
        })
}
