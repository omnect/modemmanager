//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.ModemCdma`

use zbus::proxy;

#[proxy(
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
    #[zbus(signal, name = "activation_state_changed")]
    fn activation_state_changed_func(
        &self,
        activation_state: u32,
        activation_error: u32,
        status_changes: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// ActivationState property
    #[zbus(property)]
    fn activation_state(&self) -> zbus::Result<u32>;

    /// Cdma1xRegistrationState property
    #[zbus(property)]
    fn cdma1x_registration_state(&self) -> zbus::Result<u32>;

    /// Esn property
    #[zbus(property)]
    fn esn(&self) -> zbus::Result<String>;

    /// EvdoRegistrationState property
    #[zbus(property)]
    fn evdo_registration_state(&self) -> zbus::Result<u32>;

    /// Meid property
    #[zbus(property)]
    fn meid(&self) -> zbus::Result<String>;

    /// Nid property
    #[zbus(property)]
    fn nid(&self) -> zbus::Result<u32>;

    /// Sid property
    #[zbus(property)]
    fn sid(&self) -> zbus::Result<u32>;
}
