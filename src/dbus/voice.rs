//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Voice`

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Voice",
    assume_defaults = true
)]
trait Voice {
    /// CallWaitingQuery method
    fn call_waiting_query(&self) -> zbus::Result<bool>;

    /// CallWaitingSetup method
    fn call_waiting_setup(&self, enable: bool) -> zbus::Result<()>;

    /// CreateCall method
    fn create_call(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// DeleteCall method
    fn delete_call(&self, path: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// HangupAll method
    fn hangup_all(&self) -> zbus::Result<()>;

    /// HangupAndAccept method
    fn hangup_and_accept(&self) -> zbus::Result<()>;

    /// HoldAndAccept method
    fn hold_and_accept(&self) -> zbus::Result<()>;

    /// ListCalls method
    fn list_calls(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Transfer method
    fn transfer(&self) -> zbus::Result<()>;

    /// CallAdded signal
    #[dbus_proxy(signal)]
    fn call_added(&self, path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// CallDeleted signal
    #[dbus_proxy(signal)]
    fn call_deleted(&self, path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// Calls property
    #[dbus_proxy(property)]
    fn calls(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// EmergencyOnly property
    #[dbus_proxy(property)]
    fn emergency_only(&self) -> zbus::Result<bool>;
}
