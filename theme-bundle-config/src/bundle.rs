//! System theme bundle compatibility API.

pub use crate::{
    BACKGROUND_VERSION, Bundle, Mode, NAME, Part, background_id, bundle_id, config_dir,
};

use crate::{Error, Store};

/// List the theme bundles installed system-wide.
pub fn system_bundles() -> Result<Vec<Bundle>, Error> {
    Store::new()?.system_bundles()
}

/// Apply a bundle's background to the live background config for a mode.
pub fn apply_background(slug: &str, mode: Mode) -> Result<bool, Error> {
    Store::new()?.apply_background(slug, mode)
}
