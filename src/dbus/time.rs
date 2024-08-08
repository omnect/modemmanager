//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Time`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Time",
    assume_defaults = true
)]
trait Time {
    /// GetNetworkTime method
    fn get_network_time(&self) -> zbus::Result<String>;

    /// NetworkTimeChanged signal
    #[zbus(signal)]
    fn network_time_changed(&self, time: &str) -> zbus::Result<()>;

    /// NetworkTimezone property
    #[zbus(property)]
    fn network_timezone(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;
}
