//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Modem3gpp`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Modem3gpp",
    assume_defaults = true
)]
trait Modem3gpp {
    /// DisableFacilityLock method
    fn disable_facility_lock(&self, properties: &(u32, &str)) -> zbus::Result<()>;

    /// Register method
    fn register(&self, operator_id: &str) -> zbus::Result<()>;

    /// Scan method
    fn scan(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// SetCarrierLock method
    fn set_carrier_lock(&self, data: &[u8]) -> zbus::Result<()>;

    /// SetEpsUeModeOperation method
    fn set_eps_ue_mode_operation(&self, mode: u32) -> zbus::Result<()>;

    /// SetInitialEpsBearerSettings method
    fn set_initial_eps_bearer_settings(
        &self,
        settings: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetNr5gRegistrationSettings method
    fn set_nr5g_registration_settings(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetPacketServiceState method
    fn set_packet_service_state(&self, state: u32) -> zbus::Result<()>;

    /// EnabledFacilityLocks property
    #[zbus(property)]
    fn enabled_facility_locks(&self) -> zbus::Result<u32>;

    /// EpsUeModeOperation property
    #[zbus(property)]
    fn eps_ue_mode_operation(&self) -> zbus::Result<u32>;

    /// Imei property
    #[zbus(property)]
    fn imei(&self) -> zbus::Result<String>;

    /// InitialEpsBearer property
    #[zbus(property)]
    fn initial_eps_bearer(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// InitialEpsBearerSettings property
    #[zbus(property)]
    fn initial_eps_bearer_settings(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Nr5gRegistrationSettings property
    #[zbus(property)]
    fn nr5g_registration_settings(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// OperatorCode property
    #[zbus(property)]
    fn operator_code(&self) -> zbus::Result<String>;

    /// OperatorName property
    #[zbus(property)]
    fn operator_name(&self) -> zbus::Result<String>;

    /// PacketServiceState property
    #[zbus(property)]
    fn packet_service_state(&self) -> zbus::Result<u32>;

    /// Pco property
    #[zbus(property)]
    fn pco(&self) -> zbus::Result<Vec<(u32, bool, Vec<u8>)>>;

    /// RegistrationState property
    #[zbus(property)]
    fn registration_state(&self) -> zbus::Result<u32>;

    /// SubscriptionState property
    #[zbus(property)]
    fn subscription_state(&self) -> zbus::Result<u32>;
}
