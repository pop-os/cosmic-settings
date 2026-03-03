// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::cosmic_config::{Config, CosmicConfigEntry};
use cosmic_notifications_config::{Anchor, NotificationsConfig};
use tracing::{debug, instrument, trace, warn};

/// Macro to parse and set user timeouts.
#[macro_export]
macro_rules! set_max_timeout {
    ($self:expr, $name:literal, $field:ident, $setter:ident, $s:expr) => {
        if let Some(helper) = $self.config_helper.as_ref()
            && let Ok(max) = $s.parse::<u32>()
            && let Err(e) = $self.config.$setter(helper, max.into())
        {
            error!("Failed to set '{}': {}", $name, e);
        } else if let Some(helper) = $self.config_helper.as_ref()
            && $s.is_empty()
        {
            let max = NotificationsConfig::default().$field;
            if let Err(e) = $self.config.$setter(helper, max) {
                error!("Failed to reset '{}' to default: {}", $name, e);
            }
        }
    };
}

/// Load [`NotificationsConfig`] or return the default settings.
#[instrument(skip(helper), fields(id = %cosmic_notifications_config::ID))]
pub fn load_config(helper: Option<&Config>) -> NotificationsConfig {
    trace!("Attempting to load config");

    let Some(helper) = helper else {
        debug!("Missing config helper; using default settings.");
        return Default::default();
    };

    NotificationsConfig::get_entry(helper).unwrap_or_else(|(errors, config)| {
        warn!("Loading config failed with: ");
        for error in errors.iter().filter(|e| e.is_err()) {
            warn!("\t* {error}");
        }

        config
    })
}

#[inline]
pub const fn anchor_to_pos(anchor: Anchor) -> usize {
    match anchor {
        Anchor::Top => 0,
        Anchor::Bottom => 1,
        Anchor::Right => 2,
        Anchor::Left => 3,
        Anchor::TopLeft => 4,
        Anchor::TopRight => 5,
        Anchor::BottomLeft => 6,
        Anchor::BottomRight => 7,
    }
}
