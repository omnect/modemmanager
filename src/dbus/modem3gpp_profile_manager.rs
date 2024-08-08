//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Modem3gpp.ProfileManager`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Modem3gpp.ProfileManager",
    assume_defaults = true
)]
trait ProfileManager {
    /// Delete method
    fn delete(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// List method
    fn list(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// Set method
    fn set(
        &self,
        requested_properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Updated signal
    #[zbus(signal)]
    fn updated(&self) -> zbus::Result<()>;

    /// IndexField property
    #[zbus(property)]
    fn index_field(&self) -> zbus::Result<String>;
}
