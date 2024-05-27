use zbus::{proxy, Connection};
#[proxy(
    interface = "com.system76.PowerDaemon",
    default_path = "/com/system76/PowerDaemon",
    assume_defaults = true
)]
pub trait PowerDaemon {
    fn balanced(&self) -> zbus::Result<()>;

    fn battery(&self) -> zbus::Result<()>;

    fn get_charge_profiles(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    fn get_charge_thresholds(&self) -> zbus::Result<(u8, u8)>;

    fn get_default_graphics(&self) -> zbus::Result<String>;

    fn get_external_displays_require_dgpu(&self) -> zbus::Result<bool>;

    fn get_graphics(&self) -> zbus::Result<String>;

    fn get_graphics_power(&self) -> zbus::Result<bool>;

    fn get_profile(&self) -> zbus::Result<String>;

    fn get_switchable(&self) -> zbus::Result<bool>;

    fn performance(&self) -> zbus::Result<()>;

    fn set_charge_thresholds(&self, thresholds: &(u8, u8)) -> zbus::Result<()>;

    fn set_graphics(&self, vendor: &str) -> zbus::Result<()>;

    fn set_graphics_power(&self, power: bool) -> zbus::Result<()>;

    #[zbus(signal)]
    fn hot_plug_detect(&self, port: u64) -> zbus::Result<()>;
}