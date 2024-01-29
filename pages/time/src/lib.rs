// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::time::SystemTime;

use icu_calendar::{types::IsoSecond, DateTime, Iso};
use icu_timezone::CustomTimeZone;
use timedate_zbus::TimeDateProxy;

pub struct Info {
    pub can_ntp: bool,
    pub timezone: CustomTimeZone,
    pub local_time: DateTime<Iso>,
}

impl Info {
    pub async fn load(proxy: &TimeDateProxy<'_>) -> Option<Info> {
        let can_ntp = proxy.can_ntp().await.unwrap_or_default();

        let Ok(timezone) = proxy
            .timezone()
            .await
            .unwrap_or_default()
            .parse::<CustomTimeZone>()
        else {
            return None;
        };

        let Ok(duration) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) else {
            return None;
        };

        let seconds = duration.as_secs();

        let Ok(iso_seconds) = IsoSecond::try_from((seconds % 60) as u8) else {
            return None;
        };

        #[allow(clippy::cast_possible_truncation)]
        let mut local_time = DateTime::from_minutes_since_local_unix_epoch((seconds / 60) as i32);

        local_time.time.second = iso_seconds;

        Some(Info {
            can_ntp,
            timezone,
            local_time,
        })
    }
}
