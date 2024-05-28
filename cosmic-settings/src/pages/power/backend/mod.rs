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

#[derive(Clone, Debug, PartialEq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    Battery,
}

impl PowerProfile {
    fn from_string(s: &str) -> PowerProfile {
        match s {
            "Performance" | "performance" => Self::Performance,
            "Battery" | "power-saver" => Self::Battery,
            _ => Self::Balanced,
        }
    }

    pub fn title(&self) -> String {
        match self {
            Self::Performance => fl!("power-profiles", "performance"),
            Self::Balanced => fl!("power-profiles", "balanced"),
            Self::Battery => fl!("power-profiles", "battery"),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::Performance => fl!("power-profiles", "performance-desc"),
            Self::Balanced => fl!("power-profiles", "balanced-desc"),
            Self::Battery => fl!("power-profiles", "battery-desc"),
        }
    }
}

pub fn get_power_profiles() -> Vec<PowerProfile> {
    vec![
        PowerProfile::Performance,
        PowerProfile::Balanced,
        PowerProfile::Battery,
    ]
}

pub struct S76Backend {}

impl PowerBackend for S76Backend {}

impl SetPowerProfile for S76Backend {
    async fn set_power_profile(&self, profile: PowerProfile) {
        let daemon = match get_s76power_daemon_proxy().await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("[Cosmic Settings] Problem while setting power profile.");
                return;
            }
        };

        match profile {
            PowerProfile::Performance => match daemon.performance().await {
                Ok(x) => tracing::info!("[Cosmic Settings] Performance mode activated."),
                Err(e) => tracing::error!("{e}"),
            },
            PowerProfile::Balanced => match daemon.balanced().await {
                Ok(x) => tracing::info!("[Cosmic Settings] Balanced mode activated."),
                Err(e) => tracing::error!("{e}"),
            },
            PowerProfile::Battery => match daemon.battery().await {
                Ok(x) => tracing::info!("[Cosmic Settings] Battery mode activated."),
                Err(e) => tracing::error!("{e}"),
            },
        }
    }
}

impl GetCurrentPowerProfile for S76Backend {
    async fn get_current_power_profile(&self) -> PowerProfile {
        let daemon = match get_s76power_daemon_proxy().await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("[Cosmic Settings] Problem while getting power profile.");
                //Default
                return PowerProfile::Balanced;
            }
        };

        match daemon.get_profile().await {
            Ok(p) => PowerProfile::from_string(p.as_str()),
            //Default
            Err(_) => {
                tracing::error!("[Cosmic Settings] Problem while getting power profile.");
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
            tracing::error!("[Cosmic Settings] zbus connection failed. {e}");
            return Err(());
        }
    };

    match s76powerdaemon::PowerDaemonProxy::new(&connection).await {
        Ok(d) => Ok(d),
        Err(e) => {
            tracing::error!(
                "[S76PowerDaemon] Power daemon proxy can't created. Is it installed? {e}"
            );
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
            Err(e) => {
                tracing::error!("[Cosmic Settings] Problem while setting power profile.");
                return;
            }
        };

        match profile {
            PowerProfile::Performance => match daemon.set_active_profile("performance").await {
                Ok(x) => tracing::info!("[Cosmic Settings] Performance mode activated."),
                Err(e) => tracing::error!("{e}"),
            },
            PowerProfile::Balanced => match daemon.set_active_profile("balanced").await {
                Ok(x) => tracing::info!("[Cosmic Settings] Balanced mode activated."),
                Err(e) => tracing::error!("{e}"),
            },
            PowerProfile::Battery => match daemon.set_active_profile("power-saver").await {
                Ok(x) => tracing::info!("[Cosmic Settings] Battery mode activated."),
                Err(e) => tracing::error!("{e}"),
            },
        }
    }
}

impl GetCurrentPowerProfile for PPBackend {
    async fn get_current_power_profile(&self) -> PowerProfile {
        let daemon = match get_power_profiles_proxy().await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("[Cosmic Settings] Problem while getting power profile.");
                //Default
                return PowerProfile::Balanced;
            }
        };

        let profile = match daemon.active_profile().await {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("[Cosmic Settings] Problem while getting power profile.");
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
            tracing::error!("[Cosmic Settings] zbus connection failed. {e}");
            return Err(());
        }
    };

    match ppdaemon::PowerProfilesProxy::new(&connection).await {
        Ok(d) => Ok(d),
        Err(e) => {
            tracing::error!(
                "[S76PowerDaemon] Power daemon proxy can't created. Is it installed? {e}"
            );
            Err(())
        }
    }
}
