use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles",
    assume_defaults = true
)]
pub trait PowerProfiles {
    fn hold_profile(&self, profile: &str, reason: &str, application_id: &str) -> zbus::Result<u32>;

    fn release_profile(&self, cookie: u32) -> zbus::Result<()>;

    #[zbus(signal)]
    fn profile_released(&self, cookie: u32) -> zbus::Result<()>;

    #[zbus(property)]
    fn actions(&self) -> zbus::Result<Vec<String>>;

    #[zbus(property)]
    fn active_profile(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_active_profile(&self, value: &str) -> zbus::Result<()>;

    #[zbus(property)]
    fn active_profile_holds(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    #[zbus(property)]
    fn performance_degraded(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn performance_inhibited(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn profiles(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    #[zbus(property)]
    fn version(&self) -> zbus::Result<String>;
}
