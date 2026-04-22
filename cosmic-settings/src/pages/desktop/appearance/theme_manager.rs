use cosmic::cosmic_config::{Config, ConfigSet, CosmicConfigEntry};
use cosmic::cosmic_theme::palette::{Srgb, Srgba};
use cosmic::cosmic_theme::{
    BlurStrength, CornerRadii, DARK_THEME_BUILDER_ID, LIGHT_THEME_BUILDER_ID, Spacing, Theme,
    ThemeBuilder, ThemeMode,
};
use cosmic::iced::core::Color;

use cosmic::Task;
use cosmic::theme::ThemeType;
use std::sync::Arc;

use crate::app;

use super::ContextView;

#[derive(Debug)]
pub enum ThemeStaged {
    Current,
    Both,
}

#[derive(Debug)]
pub struct Manager {
    mode: (ThemeMode, Option<Config>),
    light: ThemeCustomizer,
    dark: ThemeCustomizer,

    custom_accent: Option<Srgb>,
}

#[derive(Debug)]
pub struct ThemeCustomizer {
    builder: (ThemeBuilder, Option<Config>),
    theme: (Theme, Option<Config>),
    accent_palette: Option<Vec<Srgba>>,
    custom_window_hint: Option<Srgb>,
}

impl From<(Option<Config>, Option<Config>, Option<Vec<Srgba>>)> for ThemeCustomizer {
    fn from(
        (theme_config, builder_config, palette): (
            Option<Config>,
            Option<Config>,
            Option<Vec<Srgba>>,
        ),
    ) -> Self {
        let theme = match Theme::get_entry(theme_config.as_ref().unwrap()) {
            Ok(theme) => theme,
            Err((errs, theme)) => {
                for err in errs {
                    tracing::warn!("Error while loading theme: {err:?}");
                }
                theme
            }
        };

        let mut theme_builder = match ThemeBuilder::get_entry(builder_config.as_ref().unwrap()) {
            Ok(t) => t,
            Err((errors, t)) => {
                for e in errors {
                    tracing::error!("{e}");
                }
                t
            }
        };

        theme_builder = theme_builder
            .accent(theme.accent.base.color)
            .bg_color(theme.bg_color())
            .corner_radii(theme.corner_radii)
            .destructive(theme.destructive.base.color)
            .spacing(theme.spacing)
            .success(theme.success.base.color)
            .warning(theme.warning.base.color)
            .neutral_tint(theme.palette.neutral_5.color)
            .text_tint(theme.background(false).on.color);

        theme_builder.gaps = theme.gaps;

        let mut customizer = Self {
            builder: (theme_builder, builder_config),
            theme: (theme, theme_config),
            accent_palette: palette,
            custom_window_hint: None,
        };

        if customizer.accent_palette.is_none() {
            let palette = customizer.builder.0.palette.as_ref();
            customizer.accent_palette = Some(vec![
                palette.accent_blue,
                palette.accent_indigo,
                palette.accent_purple,
                palette.accent_pink,
                palette.accent_red,
                palette.accent_orange,
                palette.accent_yellow,
                palette.accent_green,
                palette.accent_warm_grey,
            ]);
        }

        customizer
    }
}

impl Default for Manager {
    fn default() -> Self {
        let settings_config = crate::config::Config::new();

        let theme_mode_config = ThemeMode::config().ok();
        let theme_mode = theme_mode_config
            .as_ref()
            .map(|c| match ThemeMode::get_entry(c) {
                Ok(t) => t,
                Err((errors, t)) => {
                    for e in errors {
                        tracing::error!("{e}");
                    }
                    t
                }
            })
            .unwrap_or_default();

        let mut manager = Self {
            mode: (theme_mode, theme_mode_config),
            light: (
                Theme::light_config().ok(),
                ThemeBuilder::light_config().ok(),
                settings_config.accent_palette_light().ok(),
            )
                .into(),
            dark: (
                Theme::dark_config().ok(),
                ThemeBuilder::dark_config().ok(),
                settings_config.accent_palette_dark().ok(),
            )
                .into(),
            custom_accent: None,
        };

        let customizer = manager.selected_customizer();
        manager.custom_accent = customizer.builder.0.accent.filter(|c| {
            let c = Srgba::new(c.red, c.green, c.blue, 1.0);
            let theme = &customizer.theme.0;
            c != theme.palette.accent_blue
                && c != theme.palette.accent_green
                && c != theme.palette.accent_indigo
                && c != theme.palette.accent_orange
                && c != theme.palette.accent_pink
                && c != theme.palette.accent_purple
                && c != theme.palette.accent_red
                && c != theme.palette.accent_warm_grey
                && c != theme.palette.accent_yellow
        });

        manager
    }
}

impl Manager {
    pub fn build_theme(&mut self, stage: ThemeStaged) -> Task<app::Message> {
        let map_data_fn = |customizer: &ThemeCustomizer| {
            (customizer.builder.0.clone(), customizer.theme.1.clone())
        };

        let current = map_data_fn(if self.mode.0.is_dark {
            &self.dark
        } else {
            &self.light
        });

        let other = if let ThemeStaged::Both = stage {
            Some(map_data_fn(if !self.mode.0.is_dark {
                &self.dark
            } else {
                &self.light
            }))
        } else {
            None
        };

        let mut data = std::iter::once(current).chain(other);

        cosmic::task::future(async move {
            for (builder, config) in data.by_ref() {
                if let Some(config) = config {
                    let cur = match Theme::get_entry(&config) {
                        Ok(theme) => theme,
                        Err((_errs, theme)) => theme,
                    };

                    let new = builder.build();

                    // do this without the macro
                    let tx = config.transaction();

                    if cur.accent != new.accent {
                        _ = tx.set("accent", new.accent.clone());
                    }
                    if cur.accent_button != new.accent_button {
                        _ = tx.set("accent_button", new.accent_button.clone());
                    }
                    if cur.background(false) != new.background(false) {
                        _ = tx.set("background", new.background(false).clone());
                    }
                    if cur.background(true) != new.background(true) {
                        _ = tx.set("transparent_background", new.background(true).clone());
                    }
                    if cur.button != new.button {
                        _ = tx.set("button", new.button.clone());
                    }
                    if cur.destructive != new.destructive {
                        _ = tx.set("destructive", new.destructive.clone());
                    }
                    if cur.destructive_button != new.destructive_button {
                        _ = tx.set("destructive_button", new.destructive_button.clone());
                    }
                    if cur.link_button != new.link_button {
                        _ = tx.set("link_button", new.link_button.clone());
                    }
                    if cur.icon_button != new.icon_button {
                        _ = tx.set("icon_button", new.icon_button.clone());
                    }
                    if cur.palette != new.palette {
                        _ = tx.set("palette", new.palette.clone());
                    }
                    if cur.primary(false) != new.primary(false) {
                        _ = tx.set("primary", new.primary(false).clone());
                    }
                    if cur.secondary(false) != new.secondary(false) {
                        _ = tx.set("secondary", new.secondary(false).clone());
                    }
                    if cur.primary(true) != new.primary(true) {
                        _ = tx.set("transparent_primary", new.primary(true).clone());
                    }
                    if cur.secondary(true) != new.secondary(true) {
                        _ = tx.set("transparent_secondary", new.secondary(true).clone());
                    }
                    if cur.shade != new.shade {
                        _ = tx.set("shade", new.shade.clone());
                    }
                    if cur.success != new.success {
                        _ = tx.set("success", new.success.clone());
                    }
                    if cur.text_button != new.text_button {
                        _ = tx.set("text_button", new.text_button.clone());
                    }
                    if cur.warning != new.warning {
                        _ = tx.set("warning", new.warning.clone());
                    }
                    if cur.warning_button != new.warning_button {
                        _ = tx.set("warning_button", new.warning_button.clone());
                    }
                    if cur.window_hint != new.window_hint {
                        _ = tx.set("window_hint", new.window_hint.clone());
                    }
                    if cur.accent_text != new.accent_text {
                        _ = tx.set("accent_text", new.accent_text.clone());
                    }
                    _ = tx.commit();
                }
            }

            app::Message::SetTheme(cosmic::theme::system_preference())
        })
    }

    #[inline]
    pub fn selected_customizer(&self) -> &ThemeCustomizer {
        if self.mode.0.is_dark {
            &self.dark
        } else {
            &self.light
        }
    }

    #[inline]
    pub fn selected_customizer_mut(&mut self) -> &mut ThemeCustomizer {
        if self.mode.0.is_dark {
            &mut self.dark
        } else {
            &mut self.light
        }
    }

    #[inline]
    pub fn theme(&self) -> &Theme {
        &self.selected_customizer().theme.0
    }

    #[inline]
    pub fn mode(&self) -> &ThemeMode {
        &self.mode.0
    }

    /// Update the locally cached `ThemeMode` from an external source (daemon / config watcher).
    ///
    /// This must not write back to config, since the daemon is the source of truth.
    pub fn sync_mode(&mut self, mode: ThemeMode) {
        self.mode.0 = mode;
    }

    #[inline]
    pub fn builder(&self) -> &ThemeBuilder {
        &self.selected_customizer().builder.0
    }

    #[inline]
    pub fn custom_accent(&self) -> &Option<Srgb> {
        &self.custom_accent
    }

    #[inline]
    pub fn accent_palette(&self) -> &Option<Vec<Srgba>> {
        &self.selected_customizer().accent_palette
    }

    #[inline]
    pub fn custom_window_hint(&self) -> &Option<Srgb> {
        self.selected_customizer().custom_window_hint()
    }

    #[inline]
    pub fn theme_mode_config(&self) -> &Option<Config> {
        &self.mode.1
    }

    pub fn dark_mode(&mut self, enabled: bool) -> Result<bool, cosmic_config::Error> {
        if let Some(config) = self.mode.1.as_ref() {
            return self.mode.0.set_is_dark(config, enabled);
        }

        self.mode.0.is_dark = enabled;

        let (theme_id, builder_fn): (&str, fn() -> ThemeBuilder) = if enabled {
            (DARK_THEME_BUILDER_ID, ThemeBuilder::dark)
        } else {
            (LIGHT_THEME_BUILDER_ID, ThemeBuilder::light)
        };

        let builder = cosmic::cosmic_config::Config::system(theme_id, ThemeBuilder::VERSION)
            .map_or_else(
                |_| builder_fn(),
                |config| match ThemeBuilder::get_entry(&config) {
                    Ok(t) => t,
                    Err((errs, t)) => {
                        for err in errs {
                            tracing::warn!(?err, "Error getting system theme builder");
                        }
                        t
                    }
                },
            );

        self.selected_customizer_mut().set_builder(builder);

        Ok(true)
    }

    pub fn auto_switch(&mut self, enabled: bool) {
        self.mode.0.auto_switch = enabled;

        if let Some(config) = self.mode.1.as_ref() {
            _ = config.set::<bool>("auto_switch", enabled);
        }
    }

    // TODO: Make it rollback if the first operation succeeds and the second
    // one fails?
    pub fn set_active_hint(&mut self, active_hint: u32) -> Option<ThemeStaged> {
        self.dark.set_active_hint(active_hint)?;
        self.light.set_active_hint(active_hint)?;
        Some(ThemeStaged::Both)
    }

    // TODO: Make it rollback if the first operation succeeds and the second
    // one fails?
    pub fn set_spacing(&mut self, spacing: Spacing) -> Option<ThemeStaged> {
        self.dark.set_spacing(spacing)?;
        self.light.set_spacing(spacing)?;
        Some(ThemeStaged::Both)
    }

    pub fn set_corner_radii(&mut self, radii: CornerRadii) -> Option<ThemeStaged> {
        self.dark.set_corner_radii(radii)?;
        self.light.set_corner_radii(radii)?;
        Some(ThemeStaged::Both)
    }

    pub fn set_gap_size(&mut self, gap: u32) -> Option<ThemeStaged> {
        self.dark.set_gap_size(gap)?;
        self.light.set_gap_size(gap)?;
        Some(ThemeStaged::Both)
    }

    pub fn set_frosted(&mut self, frosted: BlurStrength) -> Option<ThemeStaged> {
        self.dark.set_frosted(frosted)?;
        self.light.set_frosted(frosted)?;
        Some(ThemeStaged::Both)
    }

    pub fn get_color(&self, context: &ContextView) -> Option<Color> {
        match *context {
            ContextView::CustomAccent => self.custom_accent().map(Color::from),
            ContextView::ApplicationBackground => self.builder().bg_color.map(Color::from),
            ContextView::ContainerBackground => {
                self.builder().primary_container_bg.map(Color::from)
            }
            ContextView::InterfaceText => self.builder().text_tint.map(Color::from),
            ContextView::ControlComponent => self.builder().neutral_tint.map(Color::from),
            ContextView::AccentWindowHint => self.builder().window_hint.map(Color::from),
            _ => None,
        }
    }

    pub fn set_color(
        &mut self,
        color: Option<Color>,
        context: &ContextView,
    ) -> Option<ThemeStaged> {
        let theme_customizer = self.selected_customizer_mut();
        match *context {
            ContextView::CustomAccent => theme_customizer.set_accent(color.map(Srgb::from)),
            ContextView::ApplicationBackground => {
                theme_customizer.set_bg_color(color.map(Srgba::from))
            }
            ContextView::ContainerBackground => {
                theme_customizer.set_primary_container_bg(color.map(Srgba::from))
            }
            ContextView::InterfaceText => theme_customizer.set_text_tint(color.map(Srgb::from)),
            ContextView::ControlComponent => {
                theme_customizer.set_neutral_tint(color.map(Srgb::from))
            }
            ContextView::AccentWindowHint => {
                theme_customizer.set_window_hint(color.map(Srgb::from))
            }
            _ => None,
        }
    }

    pub fn cosmic_theme(&self) -> cosmic::Theme {
        cosmic::Theme {
            theme_type: ThemeType::Custom(Arc::new(self.theme().clone())),
            ..cosmic::Theme::default()
        }
    }
}

impl ThemeCustomizer {
    /// Set theme builder without writing to cosmic-config.
    pub fn set_builder(&mut self, builder: ThemeBuilder) -> &mut Self {
        self.builder.0 = builder;
        self
    }

    /// Write theme builder to cosmic-config, notifying all subscribers.
    pub fn apply_builder(&mut self) -> &mut Self {
        if let Some(config) = self.builder.1.as_ref() {
            let _ = self.builder.0.write_entry(config);
        }

        self
    }

    /// Set theme without writing to cosmic-config.
    pub fn set_theme(&mut self, theme: Theme) -> &mut Self {
        self.theme.0 = theme;
        self
    }

    /// Write theme to cosmic-config, notifying all subscribers.
    pub fn apply_theme(&mut self) -> &mut Self {
        if let Some(config) = self.theme.1.as_ref() {
            let _ = self.theme.0.write_entry(config);
        }

        self
    }

    pub fn set_window_hint(&mut self, color: Option<Srgb>) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.custom_window_hint = color;
        self.builder.0.set_window_hint(config, color).ok()?;
        self.theme
            .0
            .set_window_hint(self.theme.1.as_ref()?, color)
            .ok()?;

        Some(ThemeStaged::Current)
    }

    pub fn custom_window_hint(&self) -> &Option<Srgb> {
        &self.custom_window_hint
    }

    pub fn set_bg_color(&mut self, color: Option<Srgba>) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.builder.0.set_bg_color(config, color).ok()?;
        Some(ThemeStaged::Current)
    }

    pub fn set_primary_container_bg(&mut self, color: Option<Srgba>) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.builder
            .0
            .set_primary_container_bg(config, color)
            .ok()?;

        Some(ThemeStaged::Current)
    }

    pub fn set_accent(&mut self, color: Option<Srgb>) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.builder.0.set_accent(config, color).ok()?;
        Some(ThemeStaged::Current)
    }

    pub fn set_text_tint(&mut self, color: Option<Srgb>) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.builder.0.set_text_tint(config, color).ok()?;
        Some(ThemeStaged::Current)
    }

    pub fn set_neutral_tint(&mut self, color: Option<Srgb>) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.builder.0.set_neutral_tint(config, color).ok()?;
        Some(ThemeStaged::Current)
    }

    pub fn set_spacing(&mut self, spacing: Spacing) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.builder.0.set_spacing(config, spacing).ok()?;
        self.theme
            .0
            .set_spacing(self.theme.1.as_ref()?, spacing)
            .ok()?;

        Some(ThemeStaged::Current)
    }

    pub fn set_corner_radii(&mut self, corner_radii: CornerRadii) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.builder.0.set_corner_radii(config, corner_radii).ok()?;

        self.theme
            .0
            .set_corner_radii(self.theme.1.as_ref()?, corner_radii)
            .ok()?;

        Some(ThemeStaged::Current)
    }

    pub fn set_gap_size(&mut self, gap: u32) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;
        let builder = &mut self.builder.0;
        let mut gaps = builder.gaps;

        // Ensure that the gap is never less than what the active hint size is.
        gaps.1 = if gap < builder.active_hint {
            builder.active_hint
        } else {
            gap
        };

        if let Err(err) = builder.set_gaps(config, gaps) {
            tracing::error!(?err, "Error setting the gap");
            return None;
        }

        self.theme.0.set_gaps(self.theme.1.as_ref()?, gaps).ok()?;
        Some(ThemeStaged::Current)
    }

    pub fn set_frosted(&mut self, frosted: BlurStrength) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.builder.0.set_frosted(config, frosted).ok()?;
        self.theme
            .0
            .set_frosted(self.theme.1.as_ref()?, frosted)
            .ok()?;

        Some(ThemeStaged::Current)
    }

    pub fn set_frosted_windows(&mut self, enabled: bool) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.builder.0.set_frosted_windows(config, enabled).ok()?;
        self.theme
            .0
            .set_frosted_windows(self.theme.1.as_ref()?, enabled)
            .ok()?;

        Some(ThemeStaged::Current)
    }

    pub fn set_frosted_system_interface(&mut self, enabled: bool) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.builder
            .0
            .set_frosted_system_interface(config, enabled)
            .ok()?;
        self.theme
            .0
            .set_frosted_system_interface(self.theme.1.as_ref()?, enabled)
            .ok()?;

        Some(ThemeStaged::Current)
    }

    pub fn set_frosted_applets(&mut self, enabled: bool) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.builder.0.set_frosted_applets(config, enabled).ok()?;
        self.theme
            .0
            .set_frosted_applets(self.theme.1.as_ref()?, enabled)
            .ok()?;

        Some(ThemeStaged::Current)
    }

    pub fn set_frosted_panel(&mut self, enabled: bool) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.builder.0.set_frosted_panel(config, enabled).ok()?;
        self.theme
            .0
            .set_frosted_panel(self.theme.1.as_ref()?, enabled)
            .ok()?;

        Some(ThemeStaged::Current)
    }

    // set active hints is set on all themes to be consistent between dark & light themes.
    pub fn set_active_hint(&mut self, active_hint: u32) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;
        let builder = &mut self.builder.0;

        if let Err(err) = builder.set_active_hint(config, active_hint) {
            tracing::error!(?err, "Error setting the active hint");
            return None;
        }

        // Update the gap if it's less than the active hint
        if active_hint > builder.gaps.1 {
            let mut gaps = builder.gaps;
            gaps.1 = active_hint;
            if builder.set_gaps(config, gaps).unwrap_or_default() {
                let _ = self.theme.0.set_gaps(self.theme.1.as_ref()?, gaps);
            }
        }

        // Update the active_hint in the config
        self.theme
            .0
            .set_active_hint(self.theme.1.as_ref()?, active_hint)
            .ok()?;

        Some(ThemeStaged::Current)
    }
}
