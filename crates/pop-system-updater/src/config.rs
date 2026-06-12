// Copyright 2021 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use crate::{Frequency, Interval, Schedule};
use serde::{Deserialize, Serialize};
use zvariant::Type;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// If it should automatically update when updates are available.
    pub auto_update: bool,

    /// When updates should be scheduled, if updates should be scheduled for.
    pub schedule: Option<Schedule>,
}

impl Config {
    pub const CONFIG_PATH: &str = "/etc/pop-system-updater/config.ron";

    #[must_use]
    pub const fn default_schedule() -> Schedule {
        Schedule {
            interval: Interval::Weekdays,
            hour: 22,
            minute: 0,
        }
    }

    pub async fn load() -> Self {
        Self::read_file()
            .await
            .and_then(|file| ron::from_str::<Self>(&file).ok())
            .unwrap_or_else(Self::default)
    }

    async fn read_file() -> Option<String> {
        tokio::fs::read_to_string(Self::CONFIG_PATH).await.ok()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_update: false,
            schedule: Some(Config::default_schedule()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub notification_frequency: Frequency,
}

impl NotificationConfig {
    pub const CONFIG_PATH: &str = ".config/pop-system-updater/config.ron";

    pub async fn load() -> Self {
        Self::read_file()
            .await
            .and_then(|file| ron::from_str::<Self>(&file).ok())
            .unwrap_or_else(Self::default)
    }

    async fn read_file() -> Option<String> {
        let home_dir = std::env::home_dir()?;
        tokio::fs::read_to_string(home_dir.join(Self::CONFIG_PATH))
            .await
            .ok()
    }
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            notification_frequency: Frequency::Weekly,
        }
    }
}
