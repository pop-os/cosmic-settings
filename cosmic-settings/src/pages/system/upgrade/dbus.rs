use futures::{Stream, StreamExt};
use num_traits::FromPrimitive;
use pop_upgrade_client::{DaemonStatus, FetchStatus, RefreshOp, Status};
use std::{collections::HashMap, time::Duration};

use super::Progress;

pub const PACKAGE_FETCH_RESULT: &str = "PackageFetchResult";
pub const PACKAGE_FETCHING: &str = "PackageFetching";
pub const PACKAGE_FETCHED: &str = "PackageFetched";

pub const PACKAGE_UPGRADE: &str = "PackageUpgrade";

pub const RECOVERY_DOWNLOAD_PROGRESS: &str = "RecoveryDownloadProgress";
pub const RECOVERY_EVENT: &str = "RecoveryUpgradeEvent";
pub const RECOVERY_RESULT: &str = "RecoveryUpgradeResult";

pub const RELEASE_EVENT: &str = "ReleaseUpgradeEvent";
pub const RELEASE_RESULT: &str = "ReleaseUpgradeResult";

pub const REPO_COMPAT_ERROR: &str = "RepoCompatError";

pub const NO_CONNECTION: &str = "NoConnection";

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct RecoveryReleaseFlags: u8 {
        const NEXT = 1;
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DismissEvent {
    ByTimestamp = 1,
    ByUser = 2,
    Unset = 3,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unable to establish dbus connection")]
    Connection(#[source] zbus::Error),

    #[error("daemon status integer was outside the acceptable range of values")]
    DaemonStatusOutOfRange,

    #[error("{}", _0)]
    Status(Box<str>),
}

// #[derive(Debug)]
// pub enum SignalEvent {
//     FetchResult(Result<(), ReleaseError>),
//     Fetched(String, u32, u32),
//     Fetching(String),
//     NoConnection,
//     RecoveryDownloadProgress(u64, u64),
//     RecoveryUpgradeEvent(RecoveryEvent),
//     RecoveryUpgradeResult(Result<(), RecoveryError>),
//     ReleaseUpgradeEvent(UpgradeEvent),
//     Upgrade(AptUpgradeEvent),
// }

#[zbus::proxy(
    gen_blocking = false,
    default_service = "com.system76.PopUpgrade",
    default_path = "/com/system76/PopUpgrade"
)]
pub trait PopUpgrade {
    fn cancel(&self) -> zbus::Result<()>;

    /// Returns whether notifications were dismissed.
    fn dismiss_notification(&self, event: u8) -> zbus::Result<bool>;

    /// Returns `updates_available`, `completed`, and `total`.
    fn fetch_updates(
        &self,
        additional_packages: Vec<String>,
        download_only: bool,
    ) -> zbus::Result<(bool, u32, u32)>;

    fn fetch_updates_status(&self) -> zbus::Result<(u8, String)>;

    fn package_upgrade(&self) -> zbus::Result<()>;

    fn recovery_upgrade_release(
        &self,
        version: String,
        arch: String,
        flags: u8,
    ) -> zbus::Result<()>;

    fn recovery_upgrade_release_status(&self) -> zbus::Result<(u8, String)>;

    fn recovery_version(&self) -> zbus::Result<(String, i16)>;

    fn refresh_os(&self, operation: u8) -> zbus::Result<bool>;

    fn release_check(&self, development: bool) -> zbus::Result<(String, String, i16, i16, bool)>;

    fn release_repair(&self) -> zbus::Result<()>;

    fn release_upgrade(&self, how: u8, from: &str, to: &str) -> zbus::Result<()>;

    fn release_upgrade_finalize(&self) -> zbus::Result<()>;

    fn release_upgrade_status(&self) -> zbus::Result<(u8, String)>;

    fn reset(&self) -> zbus::Result<()>;

    fn status(&self) -> zbus::Result<(u8, u8)>;

    fn update_check(&self) -> zbus::Result<u8>;

    #[zbus(signal)]
    fn no_connection(&self) -> zbus::Result<()>;

    #[zbus(signal)]
    fn package_fetched(&self, package: String, completed: u32, total: u32) -> zbus::Result<()>;

    #[zbus(signal)]
    fn package_fetching(&self, package: String) -> zbus::Result<()>;

    #[zbus(signal)]
    fn package_upgrade(&self, packages: HashMap<String, String>) -> zbus::Result<()>;

    #[zbus(signal)]
    fn recovery_download_progress(&self, progress: u64, total: u64) -> zbus::Result<()>;

    #[zbus(signal)]
    fn recovery_event(&self, event: u8) -> zbus::Result<()>;

    #[zbus(signal)]
    fn recovery_result(&self, status: u8, why: String) -> zbus::Result<()>;

    #[zbus(signal)]
    fn release_event(&self, event: u8) -> zbus::Result<()>;

    #[zbus(signal)]
    fn release_result(&self, status: u8, why: String) -> zbus::Result<()>;
}

/// A signal received by the daemon.
#[derive(Debug)]
pub enum Signal {
    NoConnection,
    PackageFetchResult(Status),
    PackageFetched(FetchStatus),
    PackageFetching(Box<str>),
    PackageUpgrade(HashMap<Box<str>, Box<str>>),
    RecoveryDownloadProgress(Progress),
    RecoveryEvent(pop_upgrade_client::RecoveryEvent),
    RecoveryResult(Status),
    ReleaseResult(Status),
    ReleaseEvent(pop_upgrade_client::UpgradeEvent),
}

impl<'a> PopUpgradeProxy<'a> {
    pub async fn daemon_status_is(&self, expected: DaemonStatus) -> zbus::Result<bool> {
        self.status()
            .await
            .map(|(status, _)| status == expected as u8)
    }

    pub async fn is_inactive(&self) -> zbus::Result<bool> {
        self.status().await.map(|(status, _)| status == 0)
    }

    pub async fn inactivity_timeout(&self) {
        let mut inactivity_count = 0;

        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;

            if self.is_inactive().await.unwrap_or(true) {
                if inactivity_count < 6 {
                    inactivity_count += 1;
                    continue;
                }

                break;
            }

            inactivity_count = 0;
        }
    }

    pub fn watch_signals<'b>(&'b self) -> impl Stream<Item = Signal> + 'b {
        async_stream::stream! {
            let Ok(mut stream) = self.inner().receive_all_signals().await else {
                return;
            };

            while let Some(message) = stream.next().await {
                let header = message.header();
                let Some(member) = header.member() else {
                    continue;
                };

                yield match member.as_str() {
                    NO_CONNECTION => Signal::NoConnection,

                    PACKAGE_FETCH_RESULT => match message.body().deserialize::<(u8, String)>() {
                        Ok((status, why)) => Signal::PackageFetchResult(pop_upgrade_client::Status { status, why: why.into() }),
                        Err(_) => continue
                    },

                    PACKAGE_FETCHED => match message.body().deserialize::<(String, u32, u32)>() {
                        Ok((package, completed, total)) => {
                            Signal::PackageFetched(FetchStatus {
                                package: package.into(),
                                completed,
                                total,
                            })
                        },

                        Err(_) => continue,
                    }

                    PACKAGE_FETCHING => match message.body().deserialize::<String>() {
                        Ok(package) => Signal::PackageFetching(Box::from(package)),
                        Err(_) => continue,
                    }

                    PACKAGE_UPGRADE => match message.body().deserialize::<HashMap<String, String>>() {
                        Ok(upgrade) => {
                            Signal::PackageUpgrade(upgrade
                                .into_iter()
                                .map(|(key, value)| (Box::from(key), Box::from(value)))
                                .collect::<HashMap<Box<str>, Box<str>>>())
                        },
                        Err(_) => continue,
                    }

                    RECOVERY_DOWNLOAD_PROGRESS => match message.body().deserialize::<(u64, u64)>() {
                        Ok((progress, total)) => Signal::RecoveryDownloadProgress(Progress { progress, total }),
                        Err(_) => continue,
                    }

                    RECOVERY_EVENT => match message.body().deserialize::<u8>() {
                        Ok(event) => Signal::RecoveryEvent(pop_upgrade_client::RecoveryEvent::from_u8(event).expect("unexpected recovery event value")),
                        Err(_) => continue,
                    }

                    RECOVERY_RESULT => match message.body().deserialize::<(u8, String)>() {
                        Ok((status, why)) => Signal::RecoveryResult(Status { status, why: why.into() }),
                        Err(_) => continue,
                    }

                    RELEASE_EVENT => match message.body().deserialize::<u8>() {
                        Ok(event) => Signal::ReleaseEvent(pop_upgrade_client::UpgradeEvent::from_u8(event).expect("unexpected release event value")),
                        Err(_) => continue,
                    }

                    RELEASE_RESULT => match message.body().deserialize::<(u8, String)>() {
                        Ok((status, why)) => Signal::ReleaseResult(Status { status, why: why.into() }),
                        Err(_) => continue,
                    }

                    _ => continue
                };
            }
        }
    }
}
