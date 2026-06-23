// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0

use enumflags2::bitflags;
use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::path::Path;
use zvariant::Type;

#[zbus::proxy(
    interface = "com.system76.PopUpgrade",
    default_service = "com.system76.PopUpgrade",
    default_path = "/com/system76/PopUpgrade"
)]
pub trait Client {
    fn cancel(&self) -> zbus::Result<()>;

    fn dismiss_notification(&self, event: DismissEvent) -> zbus::Result<bool>;

    fn fetch_updates(
        &self,
        additional_packages: &[String],
        download_only: bool,
    ) -> zbus::Result<(bool, u32, u32)>;

    fn fetch_updates_status(&self) -> zbus::Result<Status>;

    /// Begin updating system packages.
    fn package_upgrade(&self) -> zbus::Result<()>;

    /// Update the recovery partition with an ISO by path.
    fn recovery_upgrade_file(&self, path: &Path) -> zbus::Result<u8>;

    /// Update the recovery partition by downloading it.
    fn recovery_upgrade_release(
        &self,
        version: &str,
        arch: &str,
        flags: u8, // RecoveryReleaseFlags
    ) -> zbus::Result<()>;

    fn recovery_upgrade_release_status(&self) -> zbus::Result<Status>;

    fn recovery_version(&self) -> zbus::Result<(Box<str>, i16)>;

    fn refresh_os(&self, operation: RefreshOp) -> zbus::Result<bool>;

    fn release_check(&self, development: bool) -> zbus::Result<ReleaseCheck>;

    fn release_upgrade(&self, how: UpgradeMethod, from: &str, to: &str) -> zbus::Result<()>;

    fn release_upgrade_finalize(&self) -> zbus::Result<()>;

    fn release_upgrade_status(&self) -> zbus::Result<Status>;

    fn release_repair(&self) -> zbus::Result<()>;

    fn reset(&self) -> zbus::Result<()>;

    fn status(&self) -> zbus::Result<(u8, u8)>;

    fn update_and_restart(&self) -> zbus::Result<u8>;

    #[zbus(signal)]
    fn no_connection(&self) -> zbus::Result<()>;

    #[zbus(signal)]
    fn package_fetch_result(&self) -> zbus::Result<Status>;

    #[zbus(signal)]
    fn package_fetched(&self) -> zbus::Result<(Box<str>, u32, u32)>;

    #[zbus(signal)]
    fn package_fetching(&self) -> zbus::Result<Box<str>>;

    #[zbus(signal)]
    fn package_upgrade(&self) -> zbus::Result<HashMap<Box<str>, Box<str>>>;

    #[zbus(signal)]
    fn recovery_download_progress(&self) -> zbus::Result<(u64, u64)>;

    #[zbus(signal)]
    fn recovery_event(&self) -> zbus::Result<u8>;

    #[zbus(signal)]
    fn recovery_result(&self) -> zbus::Result<Status>;

    #[zbus(signal)]
    fn release_result(&self) -> zbus::Result<Status>;

    #[zbus(signal)]
    fn release_event(&self) -> zbus::Result<UpgradeEventStatus>;
}

/// current, next, build, urgent, is_lts
pub type ReleaseCheck = (Box<str>, Box<str>, i16, i16, bool);

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Type, Serialize_repr, Deserialize_repr)]
pub enum UpgradeMethod {
    Offline = 1,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Type, Serialize_repr, Deserialize_repr)]
pub enum RefreshOp {
    Status = 0,
    Enable = 1,
    Disable = 2,
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Type)]
pub enum RecoveryReleaseFlags {
    NEXT = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq, Type, Serialize_repr, Deserialize_repr)]
pub enum DismissEvent {
    ByTimestamp = 1,
    ByUser = 2,
    Unset = 3,
}

/// The status of an action, and a description of why.
pub type Status = (u8, Box<str>);

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq, Type, Serialize_repr, Deserialize_repr)]
pub enum RecoveryEventStatus {
    Fetching = 1,
    Verifying = 2,
    Syncing = 3,
    Complete = 4,
}

impl From<RecoveryEventStatus> for &'static str {
    fn from(event: RecoveryEventStatus) -> Self {
        match event {
            RecoveryEventStatus::Fetching => "fetching recovery files",
            RecoveryEventStatus::Syncing => "syncing recovery files with recovery partition",
            RecoveryEventStatus::Verifying => "verifying checksums of fetched files",
            RecoveryEventStatus::Complete => "recovery partition upgrade completed",
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq, Type, Serialize_repr, Deserialize_repr)]
pub enum UpgradeEventStatus {
    UpdatingPackageLists = 1,
    FetchingPackages = 2,
    UpgradingPackages = 3,
    InstallingPackages = 4,
    UpdatingSourceLists = 5,
    FetchingPackagesForNewRelease = 6,
    FetchingAdditionalPackagesForNewRelease = 7,
    AttemptingLiveUpgrade = 8,
    AttemptingSystemdUnit = 9,
    AttemptingRecovery = 10,
    Success = 11,
    SuccessLive = 12,
    Failure = 13,
    AptFilesLocked = 14,
    RemovingConflicts = 15,
    RemovingWacomConflicts = 16,
    Simulating = 17,
}

impl From<UpgradeEventStatus> for &'static str {
    fn from(action: UpgradeEventStatus) -> Self {
        match action {
            UpgradeEventStatus::AptFilesLocked => "waiting on a process holding the apt lock files",
            UpgradeEventStatus::AttemptingLiveUpgrade => {
                "attempting live upgrade to the new release"
            }
            UpgradeEventStatus::AttemptingSystemdUnit => {
                "setting up the system to perform an offline upgrade on the next boot"
            }
            UpgradeEventStatus::AttemptingRecovery => {
                "setting up the recovery partition to install the new release"
            }
            UpgradeEventStatus::Failure => "an error occurred while setting up the release upgrade",
            UpgradeEventStatus::FetchingPackages => {
                "fetching updated packages for the current release"
            }
            UpgradeEventStatus::FetchingPackagesForNewRelease => {
                "fetching updated packages for the new release"
            }
            UpgradeEventStatus::FetchingAdditionalPackagesForNewRelease => {
                "fetching additional packages for the new release"
            }
            UpgradeEventStatus::InstallingPackages => {
                "ensuring that system-critical packages are installed"
            }
            UpgradeEventStatus::RemovingConflicts => {
                "removing deprecated and/or conflicting packages"
            }
            UpgradeEventStatus::RemovingWacomConflicts => {
                "replacing Surface-tailored Wacom packages with standard ones"
            }
            UpgradeEventStatus::Success => "new release is ready to install",
            UpgradeEventStatus::SuccessLive => "new release was successfully installed",
            UpgradeEventStatus::UpdatingPackageLists => "updating package lists",
            UpgradeEventStatus::UpdatingSourceLists => "updating the source lists",
            UpgradeEventStatus::UpgradingPackages => "upgrading packages for the current release",
            UpgradeEventStatus::Simulating => "simulating upgrade",
        }
    }
}
