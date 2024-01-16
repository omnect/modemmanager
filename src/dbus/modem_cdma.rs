//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.ModemCdma`

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.ModemManager1.Modem.ModemCdma",
    assume_defaults = true
)]
trait ModemCdma {
    /// Activate method
    fn activate(&self, carrier_code: &str) -> zbus::Result<()>;

    /// ActivateManual method
    fn activate_manual(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// ActivationStateChanged signal
    #[dbus_proxy(signal, name = "activation_state_changed")]
    fn activation_state_changed_func(
        &self,
        activation_state: u32,
        activation_error: u32,
        status_changes: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// ActivationState property
    #[dbus_proxy(property)]
    fn activation_state(&self) -> zbus::Result<u32>;

    /// Cdma1xRegistrationState property
    #[dbus_proxy(property)]
    fn cdma1x_registration_state(&self) -> zbus::Result<u32>;

    /// Esn property
    #[dbus_proxy(property)]
    fn esn(&self) -> zbus::Result<String>;

    /// EvdoRegistrationState property
    #[dbus_proxy(property)]
    fn evdo_registration_state(&self) -> zbus::Result<u32>;

    /// Meid property
    #[dbus_proxy(property)]
    fn meid(&self) -> zbus::Result<String>;

    /// Nid property
    #[dbus_proxy(property)]
    fn nid(&self) -> zbus::Result<u32>;

    /// Sid property
    #[dbus_proxy(property)]
    fn sid(&self) -> zbus::Result<u32>;
}
