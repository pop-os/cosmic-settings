use chrono::Duration;
use futures::FutureExt;
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
    S76(S76Backend),
    PP(PPBackend),
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

pub trait PowerBackend: SetPowerProfile + GetCurrentPowerProfile {}

pub async fn get_backend() -> Option<PowerBackendEnum> {
    match get_s76power_daemon_proxy().await {
        Ok(p) => match p.get_profile().await {
            Ok(_) => Some(PowerBackendEnum::S76(S76Backend {})),
            Err(_) => match get_power_profiles_proxy().await {
                Ok(pr) => match pr.active_profile().await {
                    Ok(_) => Some(PowerBackendEnum::PP(PPBackend {})),
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

    pub fn title(&self) -> String {
        match self {
            Self::Battery => fl!("power-mode", "battery"),
            Self::Balanced => fl!("power-mode", "balanced"),
            Self::Performance => fl!("power-mode", "performance"),
        }
    }

    pub fn description(&self) -> String {
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

pub struct S76Backend {}

impl PowerBackend for S76Backend {}

impl SetPowerProfile for S76Backend {
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

impl GetCurrentPowerProfile for S76Backend {
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

pub struct PPBackend {}

impl PowerBackend for PPBackend {}

impl SetPowerProfile for PPBackend {
    async fn set_power_profile(&self, profile: PowerProfile) {
        let daemon = match get_power_profiles_proxy().await {
            Ok(c) => c,
            Err(()) => {
                tracing::error!("Problem while setting power profile.");
                return;
            }
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

impl GetCurrentPowerProfile for PPBackend {
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
    pub remaining_time: String,
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
    pub async fn update_battery() -> Self {
        let proxy = get_device_proxy().await;

        if let Ok(proxy) = proxy {
            let mut remaining_duration: Duration = Duration::default();

            let (is_present, percentage, on_battery) = futures::join!(
                proxy.is_present().map(Result::unwrap_or_default),
                proxy.percentage().map(Result::unwrap_or_default),
                get_on_battery_status().map(Result::unwrap_or_default)
            );

            let percent = percentage.clamp(0.0, 100.0);

            if on_battery {
                if let Ok(time) = proxy.time_to_empty().await {
                    if let Ok(dur) = Duration::from_std(std::time::Duration::from_secs(time as u64))
                    {
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

            let remaining_time = |duration: Duration| {
                let total_seconds = duration.num_seconds();

                let hours = total_seconds / 3600;
                let minutes = (total_seconds % 3600) / 60;
                let seconds = total_seconds % 60;

                fl!(
                    "battery",
                    "remaining-time",
                    time = format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
                )
            };

            return Battery {
                icon_name,
                is_present,
                percent,
                on_battery,
                remaining_duration,
                remaining_time: remaining_time(remaining_duration),
            };
        }

        Battery::default()
    }
}
