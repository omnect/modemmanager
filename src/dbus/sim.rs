//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Sim`

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.ModemManager1.Sim",
    assume_defaults = true
)]
trait Sim {
    /// ChangePin method
    fn change_pin(&self, old_pin: &str, new_pin: &str) -> zbus::Result<()>;

    /// EnablePin method
    fn enable_pin(&self, pin: &str, enabled: bool) -> zbus::Result<()>;

    /// SendPin method
    fn send_pin(&self, pin: &str) -> zbus::Result<()>;

    /// SendPuk method
    fn send_puk(&self, puk: &str, pin: &str) -> zbus::Result<()>;

    /// SetPreferredNetworks method
    fn set_preferred_networks(&self, preferred_networks: &[(&str, u32)]) -> zbus::Result<()>;

    /// Active property
    #[dbus_proxy(property)]
    fn active(&self) -> zbus::Result<bool>;

    /// Eid property
    #[dbus_proxy(property)]
    fn eid(&self) -> zbus::Result<String>;

    /// EmergencyNumbers property
    #[dbus_proxy(property)]
    fn emergency_numbers(&self) -> zbus::Result<Vec<String>>;

    /// EsimStatus property
    #[dbus_proxy(property)]
    fn esim_status(&self) -> zbus::Result<u32>;

    /// Gid1 property
    #[dbus_proxy(property)]
    fn gid1(&self) -> zbus::Result<Vec<u8>>;

    /// Gid2 property
    #[dbus_proxy(property)]
    fn gid2(&self) -> zbus::Result<Vec<u8>>;

    /// Imsi property
    #[dbus_proxy(property)]
    fn imsi(&self) -> zbus::Result<String>;

    /// OperatorIdentifier property
    #[dbus_proxy(property)]
    fn operator_identifier(&self) -> zbus::Result<String>;

    /// OperatorName property
    #[dbus_proxy(property)]
    fn operator_name(&self) -> zbus::Result<String>;

    /// PreferredNetworks property
    #[dbus_proxy(property)]
    fn preferred_networks(&self) -> zbus::Result<Vec<(String, u32)>>;

    /// Removability property
    #[dbus_proxy(property)]
    fn removability(&self) -> zbus::Result<u32>;

    /// SimIdentifier property
    #[dbus_proxy(property)]
    fn sim_identifier(&self) -> zbus::Result<String>;

    /// SimType property
    #[dbus_proxy(property)]
    fn sim_type(&self) -> zbus::Result<u32>;
}
