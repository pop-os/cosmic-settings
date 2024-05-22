use zbus::Connection;
mod s76powerdaemon;

// Get backend, if exist s76powerdaemon preferred. If not exist power profiles deamon preferred.
async fn get_power_daemon<'a>() -> Result<s76powerdaemon::PowerDaemonProxy<'a>, ()> {
    let connection = match Connection::system().await {
        Ok(c) => c,
        Err(e) => {
            println!("[Cosmic Settings] zbus connection failed. {e}");
            return Err(());
        }
    };

    match s76powerdaemon::PowerDaemonProxy::new(&connection).await {
        Ok(d) => Ok(d),
        Err(e) => {
            println!("[S76PowerDaemon] Power daemon proxy can't created. Is it installed? {e}");
            Err(())
        }
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
            "Performance" => Self::Performance,
            "Battery" => Self::Battery,
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

pub async fn set_power_profile(profile: PowerProfile) {
    let daemon = match get_power_daemon().await {
        Ok(c) => c,
        Err(e) => {
            println!("[Cosmic Settings] Problem while setting power profile.");
            return;
        }
    };

    match profile {
        PowerProfile::Performance => match daemon.performance().await {
            Ok(x) => println!("[Cosmic Settings] Performance mode activated."),
            Err(e) => println!("{e}"),
        },
        PowerProfile::Balanced => match daemon.balanced().await {
            Ok(x) => println!("[Cosmic Settings] Balanced mode activated."),
            Err(e) => println!("{e}"),
        },
        PowerProfile::Battery => match daemon.battery().await {
            Ok(x) => println!("[Cosmic Settings] Battery mode activated."),
            Err(e) => println!("{e}"),
        },
    }
}

pub fn get_power_profiles() -> Vec<PowerProfile> {
    vec![
        PowerProfile::Performance,
        PowerProfile::Balanced,
        PowerProfile::Battery,
    ]
}

pub async fn get_current_power_profile() -> PowerProfile {
    let daemon = match get_power_daemon().await {
        Ok(c) => c,
        Err(e) => {
            println!("[Cosmic Settings] Problem while setting power profile.");
            //Default
            return PowerProfile::Balanced;
        }
    };

    match daemon.get_profile().await {
        Ok(p) => PowerProfile::from_string(p.as_str()),
        //Default
        Err(_) => {
            println!("[Cosmic Settings] Problem while setting power profile.");
            //Default
            PowerProfile::Balanced
        }
    }
}
