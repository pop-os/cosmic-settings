// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod getent;

use crate::pages;
use cosmic::{
    Apply, Element,
    dialog::file_chooser,
    iced::{Alignment, Length},
    widget::{self, Space, column, icon, row, settings, text},
};
use cosmic_settings_page::{self as page, Section, section};
use image::GenericImageView;
use pwhash::{bcrypt, md5_crypt, sha256_crypt, sha512_crypt};
use regex::Regex;
use slab::Slab;
use slotmap::SlotMap;
use std::{
    collections::HashMap,
    fs::File,
    future::Future,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    sync::Arc,
};
use url::Url;
use zbus_polkit::policykit1::CheckAuthorizationFlags;

const DEFAULT_ICON_FILE: &str = "/usr/share/pixmaps/faces/pop-robot.png";
const USERS_ADMIN_POLKIT_POLICY_ID: &str = "com.system76.CosmicSettings.Users.Admin";

// AccountsService has a hard limit of 1MB for icon files
// https://gitlab.freedesktop.org/accountsservice/accountsservice/-/blob/main/src/user.c#L3131
const MAX_ICON_SIZE_BYTES: u64 = 1_048_576;
// Use a smaller threshold to ensure compressed images stay under the limit
const ICON_SIZE_THRESHOLD: u64 = 900_000; // 900KB
// Target dimensions for resized profile icons
const TARGET_ICON_SIZE: u32 = 512;

#[derive(Clone, Debug, Default)]
pub struct User {
    id: u64,
    profile_icon: Option<icon::Handle>,
    full_name: String,
    username: String,
    password: String,
    password_confirm: String,
    full_name_edit: bool,
    username_edit: bool,
    is_admin: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorField {
    FullName,
    Username,
}

#[derive(Clone, Debug)]
pub enum Dialog {
    AddNewUser(User),
    UpdatePassword(User),
}

#[derive(Clone, Debug)]
pub struct Page {
    on_enter_handle: Option<cosmic::iced::task::Handle>,
    current_user_id: u64,
    entity: page::Entity,
    users: Vec<User>,
    selected_user_idx: Option<usize>,
    dialog: Option<Dialog>,
    default_icon: icon::Handle,
    password_label: String,
    password_confirm_label: String,
    username_label: String,
    fullname_label: String,
    password_hidden: bool,
    password_confirm_hidden: bool,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            on_enter_handle: None,
            current_user_id: 0,
            entity: page::Entity::default(),
            users: Vec::default(),
            selected_user_idx: None,
            dialog: None,
            default_icon: icon::from_path(PathBuf::from(DEFAULT_ICON_FILE)),
            password_label: crate::fl!("password"),
            password_confirm_label: crate::fl!("password-confirm"),
            username_label: crate::fl!("username"),
            fullname_label: crate::fl!("full-name"),
            password_hidden: true,
            password_confirm_hidden: true,
        }
    }
}
#[derive(Clone, Debug)]
pub enum Message {
    ApplyEdit(usize, EditorField),
    ChangedAccountType(u64, bool),
    DeletedUser(u64),
    Dialog(Option<Dialog>),
    Edit(usize, EditorField, String),
    LoadedIcon(u64, icon::Handle),
    LoadPage(u64, Vec<User>),
    NewUser(String, String, String, bool),
    None,
    SelectProfileImage(u64),
    SelectedProfileImage(u64, Arc<Result<Url, file_chooser::Error>>),
    SelectUser(usize),
    SelectedUserDelete(u64),
    SelectedUserSetAdmin(u64, bool),
    ToggleEdit(usize, EditorField),
    TogglePasswordVisibility,
    TogglePasswordConfirmVisibility,
    SaveNewPassword(User),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::User(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::User(message)
    }
}

fn prepare_icon_file(path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let metadata = std::fs::metadata(path)?;
    let file_size = metadata.len();

    tracing::debug!("Icon file size: {} bytes", file_size);

    if file_size <= ICON_SIZE_THRESHOLD {
        tracing::debug!("File size is acceptable, using original file");
        return Ok(path.to_path_buf());
    }

    tracing::info!(
        "Icon file is {} bytes, resizing to fit under 1MB limit",
        file_size
    );

    let img = image::open(path)?;
    let (width, height) = img.dimensions();

    tracing::debug!("Original image dimensions: {}x{}", width, height);

    let (new_width, new_height) = if width > height {
        let ratio = TARGET_ICON_SIZE as f32 / width as f32;
        (TARGET_ICON_SIZE, (height as f32 * ratio) as u32)
    } else {
        let ratio = TARGET_ICON_SIZE as f32 / height as f32;
        ((width as f32 * ratio) as u32, TARGET_ICON_SIZE)
    };

    tracing::debug!("Resizing to {}x{}", new_width, new_height);

    let resized = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);

    // Create a temporary file for the resized icon
    let temp_dir = std::env::temp_dir();
    let temp_filename = format!("cosmic-settings-icon-{}.png", std::process::id());
    let temp_path = temp_dir.join(temp_filename);

    tracing::debug!("Saving resized icon to: {:?}", temp_path);

    resized.save(&temp_path)?;

    let new_size = std::fs::metadata(&temp_path)?.len();
    tracing::info!("Resized icon file size: {} bytes", new_size);

    if new_size > MAX_ICON_SIZE_BYTES {
        tracing::warn!("Resized file is still too large, but attempting anyway");
    }

    Ok(temp_path)
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(user_list())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("users", "system-users-symbolic")
            .title(fl!("users"))
            .description(fl!("users", "desc"))
    }

    fn dialog(&self) -> Option<Element<'_, pages::Message>> {
        let dialog = self.dialog.as_ref()?;

        let dialog_element = match dialog {
            Dialog::AddNewUser(user) => {
                let full_name_input = widget::container(
                    widget::text_input("", &user.full_name)
                        .label(&self.fullname_label)
                        .on_input(|value| {
                            Message::Dialog(Some(Dialog::AddNewUser(User {
                                full_name: value,
                                ..user.clone()
                            })))
                        }),
                );

                let username_input = widget::container(
                    widget::text_input("", &user.username)
                        .label(&self.username_label)
                        .on_input(|value| {
                            Message::Dialog(Some(Dialog::AddNewUser(User {
                                username: value,
                                ..user.clone()
                            })))
                        }),
                );

                let password_input = widget::container(
                    widget::secure_input(
                        "",
                        &user.password,
                        Some(Message::TogglePasswordVisibility),
                        self.password_hidden,
                    )
                    .label(&self.password_label)
                    .on_input(|value| {
                        Message::Dialog(Some(Dialog::AddNewUser(User {
                            password: value,
                            ..user.clone()
                        })))
                    }),
                );

                let password_confirm_input = widget::container(
                    widget::secure_input(
                        "",
                        &user.password_confirm,
                        Some(Message::TogglePasswordConfirmVisibility),
                        self.password_confirm_hidden,
                    )
                    .label(&self.password_confirm_label)
                    .on_input(|value| {
                        Message::Dialog(Some(Dialog::AddNewUser(User {
                            password_confirm: value,
                            ..user.clone()
                        })))
                    }),
                );

                let admin_toggler = widget::toggler(user.is_admin).on_toggle(|value| {
                    Message::Dialog(Some(Dialog::AddNewUser(User {
                        is_admin: value,
                        ..user.clone()
                    })))
                });

                // validation
                let mut validation_msg = String::new();
                let username_regex = Regex::new("^[a-z][a-z0-9-]{0,30}$").unwrap();
                let username_valid = username_regex.is_match(&user.username);
                let complete_maybe = if !username_valid && !user.username.is_empty() {
                    validation_msg = fl!("invalid-username");
                    None
                } else if user.password != user.password_confirm
                    && !user.password.is_empty()
                    && !user.password_confirm.is_empty()
                {
                    validation_msg = fl!("password-mismatch");
                    None
                } else if user.full_name.is_empty()
                    || user.username.is_empty()
                    || user.password.is_empty()
                    || user.password_confirm.is_empty()
                {
                    None
                } else {
                    Some(Message::NewUser(
                        user.username.clone(),
                        user.full_name.clone(),
                        user.password.clone(),
                        user.is_admin,
                    ))
                };

                let add_user_button = widget::button::suggested(fl!("add-user"))
                    .on_press_maybe(complete_maybe)
                    .apply(Element::from);

                let cancel_button =
                    widget::button::standard(fl!("cancel")).on_press(Message::Dialog(None));

                widget::dialog()
                    .title(fl!("add-user"))
                    .control(
                        widget::ListColumn::default()
                            .add(full_name_input)
                            .add(username_input)
                            .add(password_input)
                            .add(password_confirm_input)
                            .add(
                                row::with_capacity(3)
                                    .push(
                                        column::with_capacity(2)
                                            .push(text::body(crate::fl!("administrator")))
                                            .push(text::caption(crate::fl!(
                                                "administrator",
                                                "desc"
                                            )))
                                            .width(Length::Fill),
                                    )
                                    .push(Space::new(5, 0))
                                    .push(admin_toggler)
                                    .align_y(Alignment::Center),
                            ),
                    )
                    .primary_action(add_user_button)
                    .secondary_action(cancel_button)
                    .tertiary_action(widget::text::body(validation_msg))
                    .apply(Element::from)
            }

            Dialog::UpdatePassword(user) => {
                let password_input = widget::container(
                    widget::secure_input(
                        "",
                        &user.password,
                        Some(Message::TogglePasswordVisibility),
                        self.password_hidden,
                    )
                    .label(&self.password_label)
                    .on_input(|value| {
                        Message::Dialog(Some(Dialog::UpdatePassword(User {
                            password: value,
                            ..user.clone()
                        })))
                    }),
                );

                let password_confirm_input = widget::container(
                    widget::secure_input(
                        "",
                        &user.password_confirm,
                        Some(Message::TogglePasswordConfirmVisibility),
                        self.password_confirm_hidden,
                    )
                    .label(&self.password_confirm_label)
                    .on_input(|value| {
                        Message::Dialog(Some(Dialog::UpdatePassword(User {
                            password_confirm: value,
                            ..user.clone()
                        })))
                    }),
                );

                // validation
                let mut validation_msg = String::new();
                let complete_maybe = if user.password != user.password_confirm
                    && !user.password.is_empty()
                    && !user.password_confirm.is_empty()
                {
                    validation_msg = fl!("password-mismatch");
                    None
                } else if user.password.is_empty() || user.password_confirm.is_empty() {
                    None
                } else {
                    Some(Message::SaveNewPassword(user.clone()))
                };

                let save_button = widget::button::suggested(fl!("save"))
                    .on_press_maybe(complete_maybe)
                    .apply(Element::from);

                let cancel_button =
                    widget::button::standard(fl!("cancel")).on_press(Message::Dialog(None));

                widget::dialog()
                    .title(fl!("change-password"))
                    .control(
                        widget::ListColumn::default()
                            .add(password_input)
                            .add(password_confirm_input),
                    )
                    .primary_action(save_button)
                    .secondary_action(cancel_button)
                    .tertiary_action(widget::text::body(validation_msg))
                    .apply(Element::from)
            }
        };

        dialog_element.map(crate::pages::Message::User).into()
    }

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }

        let (task, handle) = cosmic::task::future(async { Self::reload().await }).abortable();
        self.on_enter_handle = Some(handle);
        task
    }

    fn on_leave(&mut self) -> cosmic::Task<crate::pages::Message> {
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }

        cosmic::Task::none()
    }
}

impl Page {
    pub async fn reload() -> Message {
        let passwd_users = getent::passwd(uid_range());
        let mut users = Vec::with_capacity(passwd_users.len());
        let groups = getent::group();

        let uid = rustix::process::getuid().as_raw() as u64;

        let admin_group = groups.iter().find(|g| &*g.name == "sudo");

        let Ok(conn) = zbus::Connection::system().await else {
            tracing::error!("unable to access dbus system service");
            return Message::LoadPage(uid, Vec::new());
        };

        for user in passwd_users {
            let Ok(user_proxy) = accounts_zbus::UserProxy::from_uid(&conn, user.uid).await else {
                continue;
            };

            users.push(User {
                id: user.uid,
                profile_icon: user_proxy
                    .icon_file()
                    .await
                    .ok()
                    .map(|path| icon::from_path(PathBuf::from(path))),
                is_admin: match user_proxy.account_type().await {
                    Ok(1) => true,
                    Ok(_) => false,
                    Err(_) => admin_group.is_some_and(|group| group.users.contains(&user.username)),
                },
                username: String::from(user.username),
                full_name: String::from(user.full_name),
                password: String::new(),
                password_confirm: String::new(),
                full_name_edit: false,
                username_edit: false,
            });
        }

        Message::LoadPage(uid, users)
    }

    pub fn update(&mut self, message: Message) -> cosmic::Task<crate::app::Message> {
        match message {
            Message::None => (),

            Message::ChangedAccountType(uid, is_admin) => {
                for user in &mut self.users {
                    if user.id == uid {
                        user.is_admin = is_admin;
                        return cosmic::Task::none();
                    }
                }
            }

            Message::LoadedIcon(uid, handle) => {
                for user in &mut self.users {
                    if user.id == uid {
                        user.profile_icon = Some(handle);
                        return cosmic::Task::none();
                    }
                }
            }

            Message::SelectProfileImage(uid) => {
                return cosmic::task::future(async move {
                    let dialog_result = file_chooser::open::Dialog::new()
                        .title(fl!("users", "profile-add"))
                        .accept_label(fl!("dialog-add"))
                        .modal(false)
                        .open_file()
                        .await
                        .map(|response| response.url().to_owned());

                    Message::SelectedProfileImage(uid, Arc::new(dialog_result))
                });
            }

            Message::SelectedProfileImage(uid, image_result) => {
                let url = match Arc::into_inner(image_result).unwrap() {
                    Ok(url) => url,
                    Err(why) => {
                        tracing::error!(?why, "failed to get image file");
                        return cosmic::Task::none();
                    }
                };

                return cosmic::task::future(async move {
                    let Ok(conn) = zbus::Connection::system().await else {
                        return Message::None;
                    };

                    let Ok(user) = accounts_zbus::UserProxy::from_uid(&conn, uid).await else {
                        return Message::None;
                    };

                    let Ok(path) = url.to_file_path() else {
                        tracing::error!("selected image is not a file path");
                        return Message::None;
                    };

                    // Prepare the icon file, resizing if necessary to fit within accountsservice's 1MB limit
                    let icon_path = match prepare_icon_file(&path) {
                        Ok(p) => p,
                        Err(why) => {
                            tracing::error!(?why, "failed to prepare icon file");
                            return Message::None;
                        }
                    };

                    let result = request_permission_on_denial(&conn, || {
                        user.set_icon_file(icon_path.to_str().unwrap())
                    })
                    .await;

                    if let Err(why) = result {
                        tracing::error!(?why, "failed to set profile icon");
                        return Message::None;
                    }

                    Message::LoadedIcon(uid, icon::from_path(path))
                });
            }

            Message::Edit(id, field, value) => {
                if let Some(user) = self.users.get_mut(id) {
                    match field {
                        EditorField::FullName => user.full_name = value,
                        EditorField::Username => user.username = value,
                    }
                }
            }

            Message::ToggleEdit(id, field) => {
                if let Some(user) = self.users.get_mut(id) {
                    match field {
                        EditorField::FullName => user.full_name_edit = !user.full_name_edit,
                        EditorField::Username => user.username_edit = !user.username_edit,
                    }
                }
            }

            Message::TogglePasswordVisibility => {
                self.password_hidden = !self.password_hidden;
            }
            Message::TogglePasswordConfirmVisibility => {
                self.password_confirm_hidden = !self.password_confirm_hidden;
            }

            Message::ApplyEdit(id, field) => {
                if let Some(user) = self.users.get_mut(id) {
                    let uid = user.id;

                    match field {
                        EditorField::FullName => {
                            if user.full_name_edit {
                                let full_name = user.full_name.clone();

                                return cosmic::Task::future(async move {
                                    let Ok(conn) = zbus::Connection::system().await else {
                                        return;
                                    };

                                    let Ok(user) =
                                        accounts_zbus::UserProxy::from_uid(&conn, uid).await
                                    else {
                                        return;
                                    };

                                    _ = request_permission_on_denial(&conn, || {
                                        user.set_real_name(&full_name)
                                    })
                                    .await;
                                })
                                .discard();
                            }
                        }

                        EditorField::Username => {
                            if user.username_edit {
                                let username = user.username.clone();

                                return cosmic::Task::future(async move {
                                    let Ok(conn) = zbus::Connection::system().await else {
                                        return;
                                    };

                                    let Ok(user) =
                                        accounts_zbus::UserProxy::from_uid(&conn, uid).await
                                    else {
                                        return;
                                    };

                                    _ = request_permission_on_denial(&conn, || {
                                        user.set_user_name(&username)
                                    })
                                    .await;
                                })
                                .discard();
                            }
                        }
                    }
                }
            }

            Message::SaveNewPassword(user) => {
                self.dialog = None;

                let uid = user.id;
                let password_hashed = hash_password(&user.password);

                return cosmic::Task::future(async move {
                    let Ok(conn) = zbus::Connection::system().await else {
                        return;
                    };

                    let Ok(user) = accounts_zbus::UserProxy::from_uid(&conn, uid).await else {
                        return;
                    };

                    if let Err(why) = request_permission_on_denial(&conn, || {
                        user.set_password(&password_hashed, "")
                    })
                    .await
                    {
                        tracing::error!(?why, "failed to set password");
                    }
                })
                .discard();
            }

            Message::LoadPage(uid, users) => {
                self.current_user_id = uid;
                self.users = users;
            }

            Message::SelectUser(user_idx) => {
                match self.selected_user_idx {
                    Some(currently_selected_idx) if currently_selected_idx == user_idx => {
                        self.selected_user_idx = None;
                    }
                    _ => {
                        self.selected_user_idx = Some(user_idx);
                    }
                };
            }

            Message::SelectedUserDelete(uid) => {
                return cosmic::task::future(async move {
                    let Ok(conn) = zbus::Connection::system().await else {
                        return Message::None;
                    };

                    let accounts = accounts_zbus::AccountsProxy::new(&conn).await.unwrap();

                    let result = request_permission_on_denial(&conn, || {
                        accounts.delete_user(uid as i64, false)
                    })
                    .await;

                    if let Err(why) = result {
                        tracing::error!(?why, "failed to delete user account");
                        return Message::None;
                    }

                    Message::DeletedUser(uid)
                });
            }

            Message::DeletedUser(uid) => {
                self.users.retain(|user| user.id != uid);
            }

            Message::Dialog(dialog) => {
                self.password_hidden = true;
                self.password_confirm_hidden = true;
                self.dialog = dialog;
            }

            Message::NewUser(username, full_name, password, is_admin) => {
                self.dialog = None;

                return cosmic::task::future(async move {
                    let Ok(conn) = zbus::Connection::system().await else {
                        return Message::None;
                    };

                    let accounts = accounts_zbus::AccountsProxy::new(&conn).await.unwrap();

                    let user_result = request_permission_on_denial(&conn, || {
                        accounts.create_user(&username, &full_name, if is_admin { 1 } else { 0 })
                    })
                    .await;

                    let user_object_path = match user_result {
                        Ok(path) => path,

                        Err(why) => {
                            tracing::error!(?why, "failed to create user account");
                            return Message::None;
                        }
                    };

                    let password_hashed = hash_password(&password);
                    match accounts_zbus::UserProxy::new(&conn, user_object_path).await {
                        Ok(user) => {
                            _ = user.set_password(&password_hashed, "").await;
                            _ = user.set_icon_file(DEFAULT_ICON_FILE).await
                        }

                        Err(why) => {
                            tracing::error!(?why, "failed to get user by object path");
                        }
                    }

                    Self::reload().await
                });
            }

            Message::SelectedUserSetAdmin(uid, is_admin) => {
                return cosmic::task::future(async move {
                    let Ok(conn) = zbus::Connection::system().await else {
                        return Message::None;
                    };

                    let Ok(user) = accounts_zbus::UserProxy::from_uid(&conn, uid).await else {
                        return Message::None;
                    };

                    let result = request_permission_on_denial(&conn, || async {
                        user.set_account_type(if is_admin { 1 } else { 0 }).await
                    })
                    .await;

                    if let Err(why) = result {
                        tracing::error!(?why, "failed to change account type of user");
                        return Message::None;
                    }

                    Message::ChangedAccountType(uid, is_admin)
                });
            }
        };

        cosmic::Task::none()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn user_list() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let user_type_standard = descriptions.insert(fl!("users", "standard"));
    let user_type_admin = descriptions.insert(fl!("users", "admin"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            let cosmic::cosmic_theme::Spacing {
                space_xxs, space_m, ..
            } = cosmic::theme::active().cosmic().spacing;

            let users_list = page
                .users
                .iter()
                .enumerate()
                .flat_map(|(idx, user)| {
                    let expanded =
                        matches!(page.selected_user_idx, Some(user_idx) if user_idx == idx);

                    let username =
                        widget::editable_input("", &user.username, user.username_edit, move |_| {
                            Message::ToggleEdit(idx, EditorField::Username)
                        })
                        .on_input(move |name| Message::Edit(idx, EditorField::Username, name))
                        .on_submit(move |_| Message::ApplyEdit(idx, EditorField::Username))
                        .on_unfocus(Message::ApplyEdit(idx, EditorField::Username));

                    let password = widget::button::standard(fl!("change-password"))
                        .on_press(Message::Dialog(Some(Dialog::UpdatePassword(user.clone()))))
                        .apply(Element::from);

                    let fullname = widget::editable_input(
                        "",
                        &user.full_name,
                        user.full_name_edit,
                        move |_| Message::ToggleEdit(idx, EditorField::FullName),
                    )
                    .on_input(move |name| Message::Edit(idx, EditorField::FullName, name))
                    .on_submit(move |_| Message::ApplyEdit(idx, EditorField::FullName))
                    .on_unfocus(Message::ApplyEdit(idx, EditorField::FullName));

                    let fullname_text = text::body(if !user.full_name.is_empty() {
                        &user.full_name
                    } else {
                        &user.username
                    });

                    let account_type = text::caption(if user.is_admin {
                        &descriptions[user_type_admin]
                    } else {
                        &descriptions[user_type_standard]
                    });

                    let expanded_details = expanded.then(|| {
                        let mut details_list = widget::list_column()
                            .add(settings::item(&page.fullname_label, fullname))
                            .add(settings::item(&page.username_label, username))
                            .add(settings::item(&page.password_label, password))
                            .add(settings::item_row(vec![
                                column::with_capacity(2)
                                    .push(text::body(crate::fl!("administrator")))
                                    .push(text::caption(crate::fl!("administrator", "desc")))
                                    .width(Length::Fill)
                                    .into(),
                                Space::new(5, 0).into(),
                                widget::toggler(user.is_admin)
                                    .on_toggle(|enabled| {
                                        Message::SelectedUserSetAdmin(user.id, enabled)
                                    })
                                    .into(),
                            ]));

                        if page.users.len() > 1 {
                            details_list = details_list.add(settings::item_row(vec![
                                widget::horizontal_space().width(Length::Fill).into(),
                                widget::button::destructive(crate::fl!("remove-user"))
                                    .on_press(Message::SelectedUserDelete(user.id))
                                    .into(),
                            ]));
                        }

                        details_list.apply(Element::from)
                    });

                    let profile_icon_handle = user
                        .profile_icon
                        .clone()
                        .unwrap_or_else(|| page.default_icon.clone());

                    let profile_icon = widget::button::icon(profile_icon_handle)
                        .large()
                        .padding(0)
                        .class(cosmic::theme::Button::Standard)
                        .on_press(Message::SelectProfileImage(user.id));

                    let account_details_content = settings::item_row(vec![
                        widget::row::with_capacity(2)
                            .push(profile_icon)
                            .push(
                                column::with_capacity(2)
                                    .push(fullname_text)
                                    .push(account_type),
                            )
                            .align_y(Alignment::Center)
                            .spacing(space_xxs)
                            .into(),
                        widget::horizontal_space().width(Length::Fill).into(),
                        icon::from_name(if expanded {
                            "go-up-symbolic"
                        } else {
                            "go-down-symbolic"
                        })
                        .icon()
                        .size(16)
                        .into(),
                    ]);

                    let account_details = Some(
                        widget::button::custom(account_details_content)
                            .padding([space_xxs, space_m])
                            .on_press(Message::SelectUser(idx))
                            .class(cosmic::theme::Button::ListItem)
                            .selected(expanded)
                            .apply(Element::from),
                    );

                    vec![account_details, expanded_details]
                })
                .flatten()
                .fold(
                    widget::list_column()
                        .spacing(0)
                        .padding([8, 0])
                        .divider_padding(0)
                        .list_item_padding(0),
                    widget::ListColumn::add,
                )
                .apply(|list| Element::from(settings::section::with_column(list)));

            let add_user = widget::button::standard(crate::fl!("add-user"))
                .on_press(Message::Dialog(Some(Dialog::AddNewUser(User::default()))))
                .apply(widget::container)
                .width(Length::Fill)
                .align_x(Alignment::End);

            widget::column::with_capacity(2)
                .push(users_list)
                .push(add_user)
                .spacing(space_m)
                .apply(Element::from)
                .map(crate::pages::Message::User)
        })
}

async fn check_authorization(conn: &zbus::Connection) -> anyhow::Result<()> {
    let proxy = zbus_polkit::policykit1::AuthorityProxy::new(conn).await?;
    let subject = zbus_polkit::policykit1::Subject::new_for_owner(std::process::id(), None, None)?;
    proxy
        .check_authorization(
            &subject,
            USERS_ADMIN_POLKIT_POLICY_ID,
            &HashMap::new(),
            CheckAuthorizationFlags::AllowUserInteraction.into(),
            "",
        )
        .await?;
    Ok(())
}

fn uid_range() -> (u64, u64) {
    let (mut min, mut max) = (1000, 60000);
    let Ok(file) = std::fs::File::open("/etc/login.defs") else {
        return (min, max);
    };

    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => break,
            _ => (),
        }

        let line = line.trim();

        let variable = if line.starts_with("UID_MIN ") {
            &mut min
        } else if line.starts_with("UID_MAX ") {
            &mut max
        } else {
            continue;
        };

        if let Some(value) = line
            .split_ascii_whitespace()
            .nth(1)
            .and_then(|value| value.parse::<u64>().ok())
        {
            *variable = value;
        }
    }

    (min, max)
}

async fn request_permission_on_denial<T, Fun, Fut>(
    conn: &zbus::Connection,
    action: Fun,
) -> zbus::Result<T>
where
    Fun: Fn() -> Fut,
    Fut: Future<Output = zbus::Result<T>>,
{
    match action().await {
        Ok(value) => Ok(value),
        Err(why) => {
            if permission_was_denied(&why) {
                _ = check_authorization(conn).await;
                action().await
            } else {
                Err(why)
            }
        }
    }
}

fn permission_was_denied(result: &zbus::Error) -> bool {
    matches!(result, zbus::Error::MethodError(name, _, _) if name.as_str() == "org.freedesktop.Accounts.Error.PermissionDenied")
}

// TODO: Should we allow deprecated methods?
fn hash_password(password_plain: &str) -> String {
    #[allow(deprecated)]
    match get_encrypt_method().as_str() {
        "SHA512" => sha512_crypt::hash(password_plain).unwrap(),
        "SHA256" => sha256_crypt::hash(password_plain).unwrap(),
        "MD5" => md5_crypt::hash(password_plain).unwrap(),
        _ => bcrypt::hash(password_plain).unwrap(),
    }
}

// TODO: In the future loading in the whole login.defs file into an object might be handy?
// For now, just grabbing what we need
fn get_encrypt_method() -> String {
    let mut value = String::new();
    let login_defs = if let Ok(file) = File::open("/etc/login.defs") {
        file
    } else {
        return value;
    };
    let reader = BufReader::new(login_defs);

    for line in reader.lines().map_while(Result::ok) {
        if !line.trim().is_empty()
            && let Some(index) = line.find(|c: char| c.is_whitespace())
        {
            let key = line[0..index].trim();
            if key == "ENCRYPT_METHOD" {
                value = line[(index + 1)..].trim().to_string();
            }
        }
    }
    value
}
