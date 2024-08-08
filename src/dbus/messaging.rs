//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Messaging`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Messaging",
    assume_defaults = true
)]
trait Messaging {
    /// Create method
    fn create(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Delete method
    fn delete(&self, path: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// List method
    fn list(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Added signal
    #[zbus(signal)]
    fn added(&self, path: zbus::zvariant::ObjectPath<'_>, received: bool) -> zbus::Result<()>;

    /// Deleted signal
    #[zbus(signal)]
    fn deleted(&self, path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// DefaultStorage property
    #[zbus(property)]
    fn default_storage(&self) -> zbus::Result<u32>;

    /// Messages property
    #[zbus(property)]
    fn messages(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// SupportedStorages property
    #[zbus(property)]
    fn supported_storages(&self) -> zbus::Result<Vec<u32>>;
}
