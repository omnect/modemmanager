//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Firmware`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Firmware",
    assume_defaults = true
)]
trait Firmware {
    /// List method
    fn list(
        &self,
    ) -> zbus::Result<(
        String,
        Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>,
    )>;

    /// Select method
    fn select(&self, uniqueid: &str) -> zbus::Result<()>;

    /// UpdateSettings property
    #[zbus(property)]
    fn update_settings(
        &self,
    ) -> zbus::Result<(
        u32,
        std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    )>;
}
