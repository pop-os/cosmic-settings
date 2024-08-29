use zbus::proxy;
use zbus::zvariant::OwnedValue;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, OwnedValue)]
#[repr(u32)]
pub enum BatteryState {
    Unknown = 0,
    Charging = 1,
    Discharging = 2,
    Empty = 3,
    FullyCharged = 4,
    PendingCharge = 5,
    PendingDischarge = 6,
}

#[proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower",
    assume_defaults = false
)]
trait Device {
    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn state(&self) -> zbus::Result<BatteryState>;

    #[zbus(property)]
    fn time_to_empty(&self) -> zbus::Result<i64>;

    #[zbus(property)]
    fn time_to_full(&self) -> zbus::Result<i64>;
}

#[proxy(interface = "org.freedesktop.UPower", assume_defaults = true)]
trait UPower {
    /// GetDisplayDevice method
    #[zbus(object = "Device")]
    fn get_display_device(&self);
}
