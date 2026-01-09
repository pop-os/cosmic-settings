use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::Arc,
};

use bitflags::bitflags;
use cosmic_dbus_networkmanager::interface::settings::connection::ConnectionSettingsProxy;
use futures::{SinkExt, Stream};
use secure_string::SecureString;
use tokio::sync::oneshot;
use zbus::{
    ObjectServer, fdo,
    zvariant::{OwnedValue, Str},
};

pub type SecretSender = Arc<tokio::sync::Mutex<Option<tokio::sync::oneshot::Sender<SecureString>>>>;

pub const SECRET_ID: &'static str = "com.system76.CosmicSettings.NetworkManager";
pub const DBUS_PATH: &str = "/org/freedesktop/NetworkManager/SecretAgent";

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GetSecretsFlags: u32 {
        /// No special behavior.
        /// By default no user interaction is allowed and secrets must come
        /// from persistent storage, otherwise an error is returned.
        const NONE = 0x0;

        /// Allows interaction with the user (eg. prompt via UI).
        const ALLOW_INTERACTION = 0x1;

        /// Explicitly request new secrets from the user.
        /// Implies ALLOW_INTERACTION.
        const REQUEST_NEW = 0x2;

        /// Request was initiated by a user action (via D-Bus).
        const USER_REQUESTED = 0x4;

        /// Internal flag, not part of the public D-Bus API.
        const ONLY_SYSTEM = 0x8000_0000;

        /// Internal flag, not part of the public D-Bus API.
        const NO_ERRORS = 0x4000_0000;
    }
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("zbus error")]
    Zbus(#[from] zbus::Error),
    #[error("listening for secret agent closed")]
    RecvError(#[from] oneshot::error::RecvError),
    #[error("secret service error")]
    SecretService(#[from] Arc<secret_service::Error>),
    #[error("no password found for identifier: {0}")]
    NoPasswordForIdentifier(String),
    #[error("utf8 error")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PasswordFlag {
    /// The system is responsible for providing and storing this secret.
    None = 0,
    /// A user-session secret agent is responsible for providing and storing
    /// this secret; when it is required, agents will be asked to provide it.
    AgentOwned = 1,
    /// This secret should not be saved but should be requested from the user
    /// each time it is required. This flag should be used for One-Time-Pad
    /// secrets, PIN codes from hardware tokens, or if the user simply does not
    /// want to save the secret.
    NotSaved = 2,
    /// in some situations it cannot be automatically determined that a secret is required or not. This flag hints that the secret is not required and should not be requested from the user.
    NotRequired = 4,
}

#[derive(Debug, Clone)]
pub struct SecretHint {
    pub key: String,
    pub message: Option<String>,
}

fn parse_hints(hints: Vec<String>) -> Vec<SecretHint> {
    hints
        .into_iter()
        // fold message hints into previous hints
        .fold(Vec::new(), |mut acc, hint| {
            if let Some((key, msg)) = hint.split_once(':') {
                if let Some(last) = acc.last_mut() {
                    last.message = Some(format!("{}: {}", key, msg));
                }
            } else {
                acc.push(SecretHint {
                    key: hint,
                    message: None,
                });
            }
            acc
        })
}

#[derive(Debug, Clone)]
pub enum Event {
    RequestSecret {
        uuid: String,
        name: String,
        description: Option<String>,
        previous: SecureString,
        tx: SecretSender,
    },
    CancelGetSecrets {
        uuid: String,
        name: String,
    },
    Failed(Error),
}

#[derive(Debug)]
pub enum Request {
    SetSecrets {
        setting_name: String,
        uuid: String,
        secrets: HashMap<String, SecureString>,
        applied_tx: oneshot::Sender<()>,
    },
    GetSecrets {
        setting_name: String,
        uuid: String,
        resp_tx: oneshot::Sender<HashMap<String, SecureString>>,
    },
}

pub fn secret_agent_stream(
    identifier: impl AsRef<str>,
    rx: tokio::sync::mpsc::Receiver<Request>,
) -> impl Stream<Item = Event> {
    iced_futures::stream::channel(4, move |mut msg_tx| async move {
        if let Err(e) = secret_agent_stream_impl(identifier.as_ref(), msg_tx.clone(), rx).await {
            let _ = msg_tx.send(Event::Failed(e)).await;
        }
    })
}

async fn secret_agent_stream_impl(
    identifier: &str,
    msg_tx: futures::channel::mpsc::Sender<Event>,
    mut rx: tokio::sync::mpsc::Receiver<Request>,
) -> Result<(), Error> {
    // register the secret agent with NetworkManager
    let proxy =
        nm_secret_agent_manager::AgentManagerProxy::builder(&zbus::Connection::system().await?)
            .path("/org/freedesktop/NetworkManager/AgentManager")?
            .build()
            .await?;

    let _ = ObjectServer::at(
        proxy.inner().connection().object_server(),
        DBUS_PATH,
        SettingsSecretAgent { tx: msg_tx },
    )
    .await?;

    proxy.register_with_capabilities(identifier, 1).await?;

    while let Some(request) = rx.recv().await {
        match request {
            Request::SetSecrets {
                setting_name,
                uuid,
                secrets,
                applied_tx,
            } => {
                let ss = secret_service::SecretService::connect(secret_service::EncryptionType::Dh)
                    .await
                    .map_err(|e| Arc::new(e))?;
                let collection = ss.get_default_collection().await.map_err(|e| Arc::new(e))?;

                if secrets.is_empty() {
                    let mut attributes = std::collections::HashMap::new();
                    attributes.insert("application", SECRET_ID);
                    attributes.insert("uuid", &uuid);
                    let search_items = collection
                        .search_items(attributes)
                        .await
                        .map_err(|e| Arc::new(e))?;

                    for item in &search_items {
                        item.delete().await.map_err(|e| Arc::new(e))?;
                    }
                    let _ = applied_tx.send(());

                    continue;
                }

                for (name, secret) in &secrets {
                    let mut attributes = std::collections::HashMap::new();
                    attributes.insert("application", SECRET_ID);
                    attributes.insert("uuid", &uuid);
                    attributes.insert("setting_name", &setting_name);
                    attributes.insert("name", name);
                    let _item = collection
                        .create_item(
                            "NetworkManager Secret",
                            attributes,
                            secret.unsecure().as_bytes(),
                            true,
                            "text/plain",
                        )
                        .await
                        .map_err(|e| Arc::new(e))?;
                }
                let _ = applied_tx.send(());
            }
            Request::GetSecrets {
                setting_name,
                uuid,
                resp_tx,
            } => {
                let ss = secret_service::SecretService::connect(secret_service::EncryptionType::Dh)
                    .await
                    .map_err(|e| Arc::new(e))?;
                let collection = ss.get_default_collection().await.map_err(|e| Arc::new(e))?;

                let mut attributes = std::collections::HashMap::new();
                attributes.insert("application", SECRET_ID);
                attributes.insert("uuid", &uuid);
                attributes.insert("setting_name", &setting_name);

                let search_items = collection
                    .search_items(attributes)
                    .await
                    .map_err(|e| Arc::new(e))?;

                let mut secrets = HashMap::new();
                for item in &search_items {
                    let name = item
                        .get_attributes()
                        .await
                        .map_err(|e| Arc::new(e))?
                        .get("name")
                        .cloned()
                        .unwrap_or_else(|| "unknown".to_string());
                    let secret = item.get_secret().await.map_err(|e| Arc::new(e))?;
                    let secret: String = String::from_utf8(secret)?.into();
                    secrets.insert(name, SecureString::from(secret));
                }
                let _ = resp_tx.send(secrets);
            }
        }
    }
    Ok(())
}

fn parse_secret_flag(value: &str) -> PasswordFlag {
    match value {
        "0" => PasswordFlag::None,
        "1" => PasswordFlag::AgentOwned,
        "2" => PasswordFlag::NotSaved,
        "4" => PasswordFlag::NotRequired,
        _ => PasswordFlag::AgentOwned,
    }
}

fn setting_has_always_ask(setting: zbus::zvariant::Dict) -> bool {
    for (key, value) in setting.iter() {
        let Ok(key) = key.downcast_ref::<zbus::zvariant::Str>() else {
            continue;
        };
        let Ok(value) = value.downcast_ref::<zbus::zvariant::Str>() else {
            continue;
        };
        // we only care about "<secret>-flags"
        if !key.ends_with("-flags") {
            continue;
        }

        if parse_secret_flag(value.as_str()) == PasswordFlag::NotSaved {
            return true;
        }
    }

    false
}

fn has_always_ask(setting: Option<zbus::zvariant::Dict>) -> bool {
    setting.map(setting_has_always_ask).unwrap_or(false)
}

fn is_connection_always_ask(connection: &HashMap<String, HashMap<String, OwnedValue>>) -> bool {
    let conn_setting = match connection.get("connection") {
        Some(s) => s,
        None => return false,
    };

    let conn_type = match conn_setting
        .get("type")
        .and_then(|v| v.downcast_ref::<String>().ok())
    {
        Some(t) => t,
        None => return false,
    };

    // Primary setting (vpn, wifi, ethernet, etc)
    if has_always_ask(
        connection
            .get(&conn_type)
            .and_then(|d| d.get("data"))
            .and_then(|data| data.downcast_ref::<zbus::zvariant::Dict>().ok()),
    ) {
        return true;
    }

    match conn_type.as_str() {
        "802-11-wireless" => {
            if has_always_ask(
                connection
                    .get("802-11-wireless-security")
                    .and_then(|d| d.get("data"))
                    .and_then(|data| data.downcast_ref::<zbus::zvariant::Dict>().ok()),
            ) {
                return true;
            }
            if has_always_ask(
                connection
                    .get("802-1x")
                    .and_then(|d| d.get("data"))
                    .and_then(|data| data.downcast_ref::<zbus::zvariant::Dict>().ok()),
            ) {
                return true;
            }
        }

        "802-3-ethernet" => {
            if has_always_ask(
                connection
                    .get("pppoe")
                    .and_then(|d| d.get("data"))
                    .and_then(|data| data.downcast_ref::<zbus::zvariant::Dict>().ok()),
            ) {
                return true;
            }
            if has_always_ask(
                connection
                    .get("802-1x")
                    .and_then(|d| d.get("data"))
                    .and_then(|data| data.downcast_ref::<zbus::zvariant::Dict>().ok()),
            ) {
                return true;
            }
        }

        _ => {}
    }

    false
}

#[derive(Debug)]
pub struct SettingsSecretAgent {
    tx: futures::channel::mpsc::Sender<Event>,
}

#[zbus::interface(name = "org.freedesktop.NetworkManager.SecretAgent")]
impl SettingsSecretAgent {
    /// CancelGetSecrets method
    async fn cancel_get_secrets(
        &mut self,
        connection_path: zbus::zvariant::ObjectPath<'_>,
        setting_name: String,
    ) -> fdo::Result<()> {
        let conn = ConnectionSettingsProxy::builder(
            &zbus::Connection::system()
                .await
                .or_else(|_| Err(fdo::Error::Failed("failed to get uuid".to_string())))?,
        )
        .path(connection_path)?
        .build()
        .await
        .map_err(|e| fdo::Error::Failed(e.to_string()))?;

        let uuid = conn
            .get_settings()
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))?
            .get("connection")
            .and_then(|m| m.get("uuid"))
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| fdo::Error::Failed("failed to get uuid".to_string()))?
            .to_string();
        if let Err(e) = self
            .tx
            .clone()
            .send(Event::CancelGetSecrets {
                uuid,
                name: setting_name,
            })
            .await
            && e.is_disconnected()
        {
            return Err(fdo::Error::Failed(
                "failed to send cancel message".to_string(),
            ));
        }

        Ok(())
    }

    /// DeleteSecrets method
    async fn delete_secrets(
        &self,
        connection: HashMap<String, HashMap<String, zbus::zvariant::OwnedValue>>,
        connection_path: zbus::zvariant::ObjectPath<'_>,
    ) -> fdo::Result<()> {
        match self.delete_secrets_inner(connection, connection_path).await {
            Ok(_) => Ok(()),
            Err(err) => Err(fdo::Error::Failed(err.to_string())),
        }
    }

    /// GetSecrets method
    async fn get_secrets(
        &mut self,
        connection: HashMap<String, HashMap<String, zbus::zvariant::OwnedValue>>,
        connection_path: zbus::zvariant::ObjectPath<'_>,
        setting_name: String,
        hints: Vec<String>,
        flags: u32,
    ) -> HashMap<String, HashMap<String, zbus::zvariant::OwnedValue>> {
        match self
            .get_secrets_inner(connection, connection_path, setting_name, hints, flags)
            .await
        {
            Ok(result) => result,
            Err(_) => HashMap::new(),
        }
    }

    /// SaveSecrets method
    async fn save_secrets(
        &self,
        connection: HashMap<String, HashMap<String, zbus::zvariant::OwnedValue>>,
        _connection_path: zbus::zvariant::ObjectPath<'_>,
    ) -> fdo::Result<()> {
        match self.save_secrets_inner(connection).await {
            Ok(_) => Ok(()),
            Err(err) => Err(fdo::Error::Failed(err.to_string())),
        }
    }
}

impl SettingsSecretAgent {
    pub async fn get_secrets_inner(
        &mut self,
        connection: HashMap<String, HashMap<String, zbus::zvariant::OwnedValue>>,
        connection_path: zbus::zvariant::ObjectPath<'_>,
        setting_name: String,
        hints: Vec<String>,
        flags: u32,
    ) -> Result<HashMap<String, HashMap<String, zbus::zvariant::OwnedValue>>, Error> {
        let flags = GetSecretsFlags::from_bits_truncate(flags);

        let ss = secret_service::SecretService::connect(secret_service::EncryptionType::Dh)
            .await
            .map_err(|e| Arc::new(e))?;

        let collection = ss.get_default_collection().await.map_err(|e| Arc::new(e))?;

        let conn_uuid = connection
            .get("connection")
            .and_then(|m| m.get("uuid"))
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| Error::NoPasswordForIdentifier(setting_name.clone()))?
            .to_string();

        let conn =
            ConnectionSettingsProxy::builder(&zbus::Connection::system().await.or_else(|_| {
                Err(Error::Zbus(
                    fdo::Error::Failed("failed to get uuid".to_string()).into(),
                ))
            })?)
            .path(connection_path)?
            .build()
            .await
            .map_err(|e| Error::Zbus(fdo::Error::Failed(e.to_string()).into()))?;
        let settings = conn.get_settings().await?;
        let is_vpn = settings
            .get("connection")
            .and_then(|m| m.get("type"))
            .and_then(|v| v.downcast_ref::<String>().ok())
            .map_or(false, |t| t == "vpn");
        let is_always_ask = is_connection_always_ask(&settings);

        let mut setting_attributes = std::collections::HashMap::new();
        setting_attributes.insert("application", SECRET_ID);
        setting_attributes.insert("uuid", &conn_uuid);
        setting_attributes.insert("setting_name", &setting_name);

        let search_items = collection
            .search_items(setting_attributes.clone())
            .await
            .map_err(|e| Arc::new(e))?;
        let mut result = HashMap::new();
        let mut setting = HashMap::new();

        if hints.is_empty() {
            for item in &search_items {
                let name = item
                    .get_attributes()
                    .await
                    .map_err(|e| Arc::new(e))?
                    .get("name")
                    .cloned()
                    .unwrap_or_else(|| "unknown".to_string());
                let secret = item.get_secret().await.map_err(|e| Arc::new(e))?;
                let secret: String = String::from_utf8(secret)?.into();
                setting.insert(name, zbus::zvariant::OwnedValue::from(Str::from(secret)));
            }
            result.insert(setting_name, setting);
            return Ok(result);
        } else {
            let hints = parse_hints(hints);
            let mut requested = HashSet::new();

            for SecretHint { key, message } in &hints {
                if requested.contains(key) {
                    continue;
                }
                requested.insert(key);
                if flags.contains(GetSecretsFlags::REQUEST_NEW)
                    && flags.contains(GetSecretsFlags::ALLOW_INTERACTION)
                    || is_always_ask
                {
                    // request the secret via the message channel
                    let (resp_tx, resp_rx) = oneshot::channel();
                    // msg begins after ":"
                    let actual_hint = message.as_ref().map(|m| {
                        m.split_once(":")
                            .map(|(_, msg)| msg.trim().to_string())
                            .unwrap_or(m.clone())
                    });
                    if let Err(e) = self
                        .tx
                        .clone()
                        .send(Event::RequestSecret {
                            uuid: conn_uuid.clone(),
                            name: setting_name.clone(),
                            description: actual_hint.clone(),
                            previous: String::new().into(),
                            tx: Arc::new(tokio::sync::Mutex::new(Some(resp_tx))),
                        })
                        .await
                        && e.is_disconnected()
                    {
                        continue;
                    } else {
                        if let Ok(secret) = resp_rx.await {
                            let mut named_attribute = setting_attributes.clone();
                            named_attribute.insert("name", key);
                            let _item = collection
                                .create_item(
                                    "NetworkManager Secret",
                                    named_attribute,
                                    secret.unsecure().as_bytes(),
                                    true,
                                    "text/plain",
                                )
                                .await
                                .map_err(|e| Arc::new(e))?;

                            setting.insert(
                                key.clone(),
                                zbus::zvariant::OwnedValue::from(Str::from(secret.unsecure())),
                            );
                        }
                    }
                } else if !is_always_ask {
                    let mut pos = None;
                    let mut pos_with_message = None;
                    for item in &search_items {
                        let attributes = item.get_attributes().await.map_err(|e| Arc::new(e))?;
                        if let Some(value) = attributes.get("name") {
                            if value == key {
                                if let Some(saved_message) = attributes.get("message") {
                                    if message.as_ref().is_some_and(|msg| msg == saved_message) {
                                        pos_with_message = Some(item);
                                    }
                                    break;
                                } else {
                                    pos = Some(item);
                                }
                            }
                        }
                    }

                    if let Some(item) = pos_with_message.or(pos) {
                        let secret = item.get_secret().await.map_err(|e| Arc::new(e))?;
                        let secret: String = String::from_utf8(secret)?.into();
                        if is_vpn {
                            // ask anyway, but offer the previous one as a hint
                            let (resp_tx, resp_rx) = oneshot::channel();
                            let actual_hint = message.as_ref().map(|m| {
                                m.split_once(":")
                                    .map(|(_, msg)| msg.trim().to_string())
                                    .unwrap_or(m.clone())
                            });
                            if let Err(e) = self
                                .tx
                                .clone()
                                .send(Event::RequestSecret {
                                    uuid: conn_uuid.clone(),
                                    name: setting_name.clone(),
                                    description: actual_hint.clone(),
                                    previous: SecureString::from(secret.clone()),
                                    tx: Arc::new(tokio::sync::Mutex::new(Some(resp_tx))),
                                })
                                .await
                                && e.is_disconnected()
                            {
                                continue;
                            } else {
                                if let Ok(secret) = resp_rx.await {
                                    let mut named_attribute = setting_attributes.clone();
                                    named_attribute.insert("name", key);
                                    let _item = collection
                                        .create_item(
                                            "NetworkManager Secret",
                                            named_attribute,
                                            secret.unsecure().as_bytes(),
                                            true,
                                            "text/plain",
                                        )
                                        .await
                                        .map_err(|e| Arc::new(e))?;

                                    setting.insert(
                                        key.clone(),
                                        zbus::zvariant::OwnedValue::from(Str::from(
                                            secret.unsecure(),
                                        )),
                                    );
                                }
                            }
                        } else {
                            setting.insert(
                                key.clone(),
                                zbus::zvariant::OwnedValue::from(Str::from(secret)),
                            );
                        }
                    }
                } else {
                    // can't find the secret, and we can't request it, so we just skip it
                    continue;
                }
            }
            result.insert(setting_name, setting);
            return Ok(result);
        }
    }

    pub async fn delete_secrets_inner(
        &self,
        connection: HashMap<String, HashMap<String, zbus::zvariant::OwnedValue>>,
        _connection_path: zbus::zvariant::ObjectPath<'_>,
    ) -> Result<(), Error> {
        let ss = secret_service::SecretService::connect(secret_service::EncryptionType::Dh)
            .await
            .map_err(|e| Arc::new(e))?;
        let collection = ss.get_default_collection().await.map_err(|e| Arc::new(e))?;

        let conn_uuid = connection
            .get("connection")
            .and_then(|m| m.get("uuid"))
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| Error::NoPasswordForIdentifier("unknown".to_string()))?
            .to_string();

        let mut attributes = std::collections::HashMap::new();
        attributes.insert("application", SECRET_ID);
        attributes.insert("uuid", &conn_uuid);

        let search_items = collection
            .search_items(attributes)
            .await
            .map_err(|e| Arc::new(e))?;

        for item in &search_items {
            item.delete().await.map_err(|e| Arc::new(e))?;
        }
        Ok(())
    }

    pub async fn save_secrets_inner(
        &self,
        connection: HashMap<String, HashMap<String, zbus::zvariant::OwnedValue>>,
    ) -> Result<(), Error> {
        let ss = secret_service::SecretService::connect(secret_service::EncryptionType::Dh)
            .await
            .map_err(|e| Arc::new(e))?;
        let collection = ss.get_default_collection().await.map_err(|e| Arc::new(e))?;
        let conn_uuid = connection
            .get("connection")
            .and_then(|m| m.get("uuid"))
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| Error::NoPasswordForIdentifier("unknown".to_string()))?
            .to_string();

        let secret: Option<(String, String)> = connection
            .get("802-11-wireless-security")
            .and_then(|m| m.get("psk"))
            .and_then(|v| v.downcast_ref::<String>().ok())
            .map(|password| ("psk".to_string(), password.clone()))
            .or_else(|| {
                connection
                    .get("802-1x")
                    .and_then(|s| s.get("password"))
                    .and_then(|v| v.downcast_ref::<String>().ok())
                    .map(|password| ("802-1x-password".to_string(), password.clone()))
            });
        if let Some((name, secret)) = secret {
            let mut attributes = std::collections::HashMap::new();
            attributes.insert("application", SECRET_ID);
            attributes.insert("uuid", &conn_uuid);
            attributes.insert("setting_name", &name);
            let _item = collection
                .create_item(
                    "NetworkManager Secret",
                    attributes,
                    secret.as_bytes(),
                    true,
                    "text/plain",
                )
                .await
                .map_err(|e| Arc::new(e))?;
            Ok(())
        } else {
            Err(Error::NoPasswordForIdentifier("unknown".to_string()))
        }
    }
}
