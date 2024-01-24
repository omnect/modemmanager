//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Sar`

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Sar",
    assume_defaults = true
)]
trait Sar {
    /// Enable method
    fn enable(&self, enable: bool) -> zbus::Result<()>;

    /// SetPowerLevel method
    fn set_power_level(&self, level: u32) -> zbus::Result<()>;

    /// PowerLevel property
    #[dbus_proxy(property)]
    fn power_level(&self) -> zbus::Result<u32>;

    /// State property
    #[dbus_proxy(property)]
    fn state(&self) -> zbus::Result<bool>;
}
