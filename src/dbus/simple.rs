//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Simple`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Simple",
    assume_defaults = true
)]
pub trait Simple {
    /// Connect method
    fn connect(
        &self,
        properties: std::collections::HashMap<&str, zbus::Variant<'_>>,
    ) -> zbus::Result<zbus::OwnedObjectPath>;

    /// Disconnect method
    fn disconnect(&self, bearer: &zbus::ObjectPath<'_>) -> zbus::Result<()>;

    /// GetStatus method
    fn get_status(&self) -> zbus::Result<std::collections::HashMap<String, zbus::OwnedVariant>>;
}
