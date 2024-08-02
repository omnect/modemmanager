//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Sar`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Sar",
    assume_defaults = true
)]
trait Sar {
    /// Enable method
    fn enable(&self, enable: bool) -> zbus::Result<()>;

    /// SetPowerLevel method
    fn set_power_level(&self, level: u32) -> zbus::Result<()>;

    /// PowerLevel property
    #[zbus(property)]
    fn power_level(&self) -> zbus::Result<u32>;

    /// State property
    #[zbus(property)]
    fn state(&self) -> zbus::Result<bool>;
}
