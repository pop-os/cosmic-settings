// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

crate::cache_dynamic_lazy! {
    pub static COLOR: String = fl!("color");
    pub static COLOR_DEPTH: String = fl!("color", "depth");
    pub static COLOR_PROFILE: String = fl!("color", "profile");
    pub static COLOR_PROFILES: String = fl!("color", "sidebar");
    pub static COLOR_TEMPERATURE: String = fl!("color", "temperature");

    pub static DISPLAY: String = fl!("display");
    pub static DISPLAY_ARRANGEMENT: String = fl!("display", "arrangement");
    pub static DISPLAY_ARRANGEMENT_DESC: String = fl!("display", "arrangement-desc");
    pub static DISPLAY_ENABLE: String = fl!("display", "enable");
    pub static DISPLAY_EXTERNAL: String = fl!("display", "external");
    pub static DISPLAY_LAPTOP: String = fl!("display", "laptop");
    pub static DISPLAY_OPTIONS: String = fl!("display", "options");
    pub static DISPLAY_REFRESH_RATE: String = fl!("display", "refresh-rate");
    pub static DISPLAY_RESOLUTION: String = fl!("display", "resolution");
    pub static DISPLAY_SCALE: String = fl!("display", "scale");

    pub static GRAPHICS_MODE: String = fl!("graphics-mode");
    pub static GRAPHICS_MODE_COMPUTE: String =
        fl!("graphics-mode", "mode", mode = super::graphics::COMPUTE);
    pub static GRAPHICS_MODE_COMPUTE_ENABLE: String =
        fl!("graphics-mode", "enable", mode = super::graphics::COMPUTE);
    pub static GRAPHICS_MODE_COMPUTE_DESC: String =
        fl!("graphics-mode", "desc", mode = super::graphics::COMPUTE);
    pub static GRAPHICS_MODE_HYBRID: String =
        fl!("graphics-mode", "mode", mode = super::graphics::HYBRID);
    pub static GRAPHICS_MODE_HYBRID_ENABLE: String =
        fl!("graphics-mode", "enable", mode = super::graphics::HYBRID);
    pub static GRAPHICS_MODE_HYBRID_DESC: String =
        fl!("graphics-mode", "desc", mode = super::graphics::HYBRID);
    pub static GRAPHICS_MODE_INTEGRATED: String =
        fl!("graphics-mode", "mode", mode = super::graphics::INTEGRATED);
    pub static GRAPHICS_MODE_INTEGRATED_ENABLE: String = fl!(
        "graphics-mode",
        "enable",
        mode = super::graphics::INTEGRATED
    );
    pub static GRAPHICS_MODE_INTEGRATED_DESC: String =
        fl!("graphics-mode", "desc", mode = super::graphics::INTEGRATED);
    pub static GRAPHICS_MODE_NVIDIA: String =
        fl!("graphics-mode", "mode", mode = super::graphics::NVIDIA);
    pub static GRAPHICS_MODE_NVIDIA_ENABLE: String =
        fl!("graphics-mode", "enable", mode = super::graphics::NVIDIA);
    pub static GRAPHICS_MODE_NVIDIA_DESC: String =
        fl!("graphics-mode", "desc", mode = super::graphics::NVIDIA);

    pub static MIRRORING: String = fl!("mirroring");

    pub static NIGHT_LIGHT: String = fl!("night-light");
    pub static NIGHT_LIGHT_AUTO: String = fl!("night-light", "auto");
    pub static NIGHT_LIGHT_DESCRIPTION: String = fl!("night-light", "desc");

    pub static ORIENTATION: String = fl!("orientation");
    pub static ORIENTATION_STANDARD: String = fl!("orientation", "standard");
    pub static ORIENTATION_ROTATE_90: String = fl!("orientation", "rotate-90");
    pub static ORIENTATION_ROTATE_180: String = fl!("orientation", "rotate-180");
    pub static ORIENTATION_ROTATE_270: String = fl!("orientation", "rotate-270");

    pub static SCHEDULING: String = fl!("scheduling");
    pub static SCHEDULING_MANUAL: String = fl!("scheduling", "manual");
}
