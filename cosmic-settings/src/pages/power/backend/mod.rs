use chrono::{Duration, TimeDelta};
use futures::future::join_all;
use futures::FutureExt;
use upower_dbus::{BatteryType, DeviceProxy};
use zbus::Connection;

mod ppdaemon;
mod s76powerdaemon;

pub trait SetPowerProfile {
    async fn set_power_profile(&self, profile: PowerProfile);
}

pub trait GetCurrentPowerProfile {
    async fn get_current_power_profile(&self) -> PowerProfile;
}

pub enum PowerBackendEnum {
    S76(S76),
    PP(PP),
}

impl SetPowerProfile for PowerBackendEnum {
    async fn set_power_profile(&self, profile: PowerProfile) {
        match self {
            PowerBackendEnum::S76(backend) => backend.set_power_profile(profile).await,
            PowerBackendEnum::PP(backend) => backend.set_power_profile(profile).await,
        }
    }
}

impl GetCurrentPowerProfile for PowerBackendEnum {
    async fn get_current_power_profile(&self) -> PowerProfile {
        match self {
            PowerBackendEnum::S76(backend) => backend.get_current_power_profile().await,
            PowerBackendEnum::PP(backend) => backend.get_current_power_profile().await,
        }
    }
}

pub async fn get() -> Option<PowerBackendEnum> {
    match get_s76power_daemon_proxy().await {
        Ok(p) => match p.get_profile().await {
            Ok(_) => Some(PowerBackendEnum::S76(S76 {})),
            Err(_) => match get_power_profiles_proxy().await {
                Ok(pr) => match pr.active_profile().await {
                    Ok(_) => Some(PowerBackendEnum::PP(PP {})),
                    Err(_) => None,
                },
                Err(()) => None,
            },
        },
        Err(()) => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PowerProfile {
    Battery,
    Balanced,
    Performance,
}

impl PowerProfile {
    fn from_string(s: &str) -> PowerProfile {
        match s {
            "Battery" | "power-saver" => Self::Battery,
            "Performance" | "performance" => Self::Performance,
            _ => Self::Balanced,
        }
    }

    pub fn title(self) -> String {
        match self {
            Self::Battery => fl!("power-mode", "battery"),
            Self::Balanced => fl!("power-mode", "balanced"),
            Self::Performance => fl!("power-mode", "performance"),
        }
    }

    pub fn description(self) -> String {
        match self {
            Self::Battery => fl!("power-mode", "battery-desc"),
            Self::Balanced => fl!("power-mode", "balanced-desc"),
            Self::Performance => fl!("power-mode", "performance-desc"),
        }
    }
}

pub fn get_power_profiles() -> Vec<PowerProfile> {
    vec![
        PowerProfile::Battery,
        PowerProfile::Balanced,
        PowerProfile::Performance,
    ]
}

pub struct S76 {}

impl SetPowerProfile for S76 {
    async fn set_power_profile(&self, profile: PowerProfile) {
        let Ok(daemon) = get_s76power_daemon_proxy().await else {
            tracing::error!("Problem while setting power profile.");
            return;
        };

        match profile {
            PowerProfile::Battery => match daemon.battery().await {
                Ok(()) => tracing::info!("Battery mode activated."),
                Err(e) => tracing::error!("{e}"),
            },
            PowerProfile::Balanced => match daemon.balanced().await {
                Ok(()) => tracing::info!("Balanced mode activated."),
                Err(e) => tracing::error!("{e}"),
            },
            PowerProfile::Performance => match daemon.performance().await {
                Ok(()) => tracing::info!("Performance mode activated."),
                Err(e) => tracing::error!("{e}"),
            },
        }
    }
}

impl GetCurrentPowerProfile for S76 {
    async fn get_current_power_profile(&self) -> PowerProfile {
        let Ok(daemon) = get_s76power_daemon_proxy().await else {
            tracing::error!("Problem while getting power profile.");
            return PowerProfile::Balanced;
        };

        match daemon.get_profile().await {
            Ok(p) => PowerProfile::from_string(p.as_str()),
            //Default
            Err(_why) => {
                tracing::error!("Problem while getting power profile.");
                //Default
                PowerProfile::Balanced
            }
        }
    }
}

async fn get_s76power_daemon_proxy<'a>() -> Result<s76powerdaemon::PowerDaemonProxy<'a>, ()> {
    let connection = match Connection::system().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("zbus connection failed. {e}");
            return Err(());
        }
    };

    match s76powerdaemon::PowerDaemonProxy::new(&connection).await {
        Ok(d) => Ok(d),
        Err(e) => {
            tracing::error!("Power daemon proxy can't be created. Is it installed? {e}");
            Err(())
        }
    }
}

pub struct PP {}

impl SetPowerProfile for PP {
    async fn set_power_profile(&self, profile: PowerProfile) {
        let daemon = if let Ok(c) = get_power_profiles_proxy().await {
            c
        } else {
            tracing::error!("Problem while setting power profile.");
            return;
        };

        match profile {
            PowerProfile::Battery => match daemon.set_active_profile("power-saver").await {
                Ok(()) => tracing::info!("Battery mode activated."),
                Err(e) => tracing::error!("{e}"),
            },
            PowerProfile::Balanced => match daemon.set_active_profile("balanced").await {
                Ok(()) => tracing::info!("Balanced mode activated."),
                Err(e) => tracing::error!("{e}"),
            },
            PowerProfile::Performance => match daemon.set_active_profile("performance").await {
                Ok(()) => tracing::info!("Performance mode activated."),
                Err(e) => tracing::error!("{e}"),
            },
        }
    }
}

impl GetCurrentPowerProfile for PP {
    async fn get_current_power_profile(&self) -> PowerProfile {
        let Ok(daemon) = get_power_profiles_proxy().await else {
            tracing::error!("Problem while getting power profile.");
            return PowerProfile::Balanced;
        };

        let profile = match daemon.active_profile().await {
            Ok(p) => p,
            Err(_e) => {
                tracing::error!("Problem while getting power profile.");
                //Default
                return PowerProfile::Balanced;
            }
        };

        PowerProfile::from_string(&profile)
    }
}

async fn get_power_profiles_proxy<'a>() -> Result<ppdaemon::PowerProfilesProxy<'a>, ()> {
    let connection = match Connection::system().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("zbus connection failed. {e}");
            return Err(());
        }
    };

    match ppdaemon::PowerProfilesProxy::new(&connection).await {
        Ok(d) => Ok(d),
        Err(e) => {
            tracing::error!("Power daemon proxy can't be created. Is it installed? {e}");
            Err(())
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Battery {
    pub icon_name: String,
    pub is_present: bool,
    pub percent: f64,
    pub on_battery: bool,
    pub remaining_duration: Duration,
}

#[derive(Default, Debug, Clone)]
pub struct ConnectedDevice {
    pub model: String,
    pub device_icon: &'static str,
    pub battery: Battery,
}

async fn get_device_proxy<'a>() -> Result<upower_dbus::DeviceProxy<'a>, zbus::Error> {
    let connection = match Connection::system().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("zbus connection failed. {e}");
            return Err(e);
        }
    };

    match upower_dbus::UPowerProxy::new(&connection).await {
        Ok(p) => p.get_display_device().await,
        Err(e) => Err(e),
    }
}

async fn enumerate_devices<'a>() -> Result<Vec<upower_dbus::DeviceProxy<'a>>, zbus::Error> {
    let connection = match Connection::system().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("zbus connection failed. {e}");
            return Err(e);
        }
    };

    let devices = upower_dbus::UPowerProxy::new(&connection)
        .await?
        .enumerate_devices()
        .await?;

    let devices = futures::future::join_all(
        devices
            .into_iter()
            .map(|path| DeviceProxy::new(&connection, path)),
    )
    .await;

    let errors = devices.iter().filter(|device| device.is_err());
    if errors.count() > 0 {
        let mut errors: Vec<zbus::Error> = devices
            .into_iter()
            .filter_map(std::result::Result::err)
            .collect();
        if errors.len() > 1 {
            eprintln!("Multiple errors occurs when fetching connected device: {errors:?}. Only the last one will be returned.");
        }
        return Err(errors.pop().unwrap());
    }
    Ok(devices
        .into_iter()
        .filter_map(std::result::Result::ok)
        .collect())
}

async fn get_on_battery_status() -> Result<bool, zbus::Error> {
    let connection = match Connection::system().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("zbus connection failed. {e}");
            return Err(e);
        }
    };

    match upower_dbus::UPowerProxy::new(&connection).await {
        Ok(p) => p.on_battery().await,
        Err(e) => Err(e),
    }
}

impl Battery {
    pub async fn from_device(proxy: DeviceProxy<'_>) -> Self {
        let mut remaining_duration: Duration = Duration::default();

        let (is_present, percentage, on_battery) = futures::join!(
            proxy.is_present().map(Result::unwrap_or_default),
            proxy.percentage().map(Result::unwrap_or_default),
            get_on_battery_status().map(Result::unwrap_or_default)
        );

        let percent = percentage.clamp(0.0, 100.0);

        if on_battery {
            if let Ok(time) = proxy.time_to_empty().await {
                if let Ok(dur) = Duration::from_std(std::time::Duration::from_secs(time as u64)) {
                    remaining_duration = dur;
                }
            }
        } else if let Ok(time) = proxy.time_to_full().await {
            if let Ok(dur) = Duration::from_std(std::time::Duration::from_secs(time as u64)) {
                remaining_duration = dur;
            }
        }

        let battery_percent = if percent > 95.0 {
            100
        } else if percent > 80.0 {
            90
        } else if percent > 65.0 {
            80
        } else if percent > 35.0 {
            50
        } else if percent > 20.0 {
            35
        } else if percent > 14.0 {
            20
        } else if percent > 9.0 {
            10
        } else if percent > 5.0 {
            5
        } else {
            0
        };
        let charging = if on_battery { "" } else { "charging-" };

        let icon_name =
            format!("cosmic-applet-battery-level-{battery_percent}-{charging}symbolic",);

        Self {
            icon_name,
            is_present,
            percent,
            on_battery,
            remaining_duration,
        }
    }

    pub async fn update_battery() -> Self {
        let proxy = get_device_proxy().await;

        if let Ok(proxy) = proxy {
            return Self::from_device(proxy).await;
        }

        Battery::default()
    }
    pub fn remaining_time(&self) -> String {
        if self.remaining_duration <= TimeDelta::zero() {
            return String::new();
        }

        let total_seconds = self.remaining_duration.num_seconds();

        let days = total_seconds / 86400;
        let hours = total_seconds % 86400 / 3600;
        let minutes = (total_seconds % 3600) / 60;

        let mut time: Vec<String> = Vec::new();
        if days > 0 {
            time.push(fl!("battery", "day", value = days));
        }
        if hours > 0 {
            time.push(fl!("battery", "hour", value = hours));
        }
        if minutes > 0 {
            time.push(fl!("battery", "minute", value = minutes));
        }

        if time.len() == 3 {
            let last = time.pop().unwrap();
            time = vec![time.join(", "), last];
        }
        let time = if time.is_empty() {
            fl!("battery", "less-than-minute")
        } else {
            time.join(&format!(" {} ", fl!("battery", "and")))
        };

        fl!(
            "battery",
            "remaining-time",
            time = time,
            action = if self.on_battery { "empty" } else { "full" }
        )
    }
}

impl ConnectedDevice {
    async fn from_device_maybe(proxy: DeviceProxy<'_>) -> Option<Self> {
        let device_type = proxy.type_().await.unwrap_or(BatteryType::Unknown);
        if matches!(
            device_type,
            BatteryType::Unknown | BatteryType::LinePower | BatteryType::Battery
        ) {
            return None;
        }
        let model = proxy
            .model()
            .await
            .unwrap_or(fl!("connected-devices", "unknown"));
        let battery = Battery::from_device(proxy).await;
        let device_icon = match device_type {
            BatteryType::Ups => "uninterruptible-power-supply-symbolic",
            BatteryType::Monitor => "display-symbolic",
            BatteryType::Mouse => "input-mouse-symbolic",
            BatteryType::Keyboard => "input-keyboard-symbolic",
            BatteryType::Pda | BatteryType::Phone => "smartphone-symbolic",
            BatteryType::MediaPlayer => "multimedia-player-symbolic",
            BatteryType::Tablet => "tablet-symbolic",
            BatteryType::Computer => "laptop-symbolic",
            BatteryType::GamingInput => "input-gaming-symbolic",
            BatteryType::Pen => "input-tablet-symbolic",
            BatteryType::Touchpad => "input-touchpad-symbolic",
            BatteryType::Network => "network-wired-symbolic",
            BatteryType::Headset => "audio-headset-symbolic",
            BatteryType::Speakers => "speaker-symbolic",
            BatteryType::Headphones => "audio-headphones-symbolic",
            BatteryType::Video => "video-display-symbolic",
            BatteryType::OtherAudio => "audio-speakers-symbolic",
            BatteryType::Printer => "printer-network-symbolic",
            BatteryType::Scanner => "scanner-symbolic",
            BatteryType::Camera => "camera-photo-symbolic",
            _ => "bluetooth-symbolic",
        };

        Some(Self {
            model,
            device_icon,
            battery,
        })
    }

    pub async fn update_connected_devices() -> Vec<Self> {
        let proxy = enumerate_devices().await;

        if let Ok(devices) = proxy {
            return join_all(
                devices
                    .into_iter()
                    .map(|device| Self::from_device_maybe(device)),
            )
            .await
            .into_iter()
            .flatten()
            .collect();
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_format_battery_remaining() {
        let cases = [
            (59, "Less than a minute until empty"),
            (300, "5 minutes until empty"),
            (305, "5 minutes until empty"),
            (330, "5 minutes until empty"),
            (360, "6 minutes until empty"),
            (3660, "1 hour and 1 minute until empty"),
            (10800, "3 hours until empty"),
            (969400, "11 days, 5 hours and 16 minutes until empty"),
        ];
        for case in cases {
            let (actual, expected) = case;
            let battery = Battery {
                remaining_duration: Duration::new(actual, 0).unwrap(),
                on_battery: true,
                ..Default::default()
            };
            assert_eq!(battery.remaining_time(), expected);
        }
    }
}
