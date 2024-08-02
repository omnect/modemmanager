//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Simple`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Simple",
    assume_defaults = true
)]
trait Simple {
    /// Connect method
    fn connect(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Disconnect method
    fn disconnect(&self, bearer: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// GetStatus method
    fn get_status(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;
}
