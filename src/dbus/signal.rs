//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Signal`

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Signal",
    assume_defaults = true
)]
trait Signal {
    /// Setup method
    fn setup(&self, rate: u32) -> zbus::Result<()>;

    /// SetupThresholds method
    fn setup_thresholds(
        &self,
        settings: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Cdma property
    #[dbus_proxy(property)]
    fn cdma(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// ErrorRateThreshold property
    #[dbus_proxy(property)]
    fn error_rate_threshold(&self) -> zbus::Result<bool>;

    /// Evdo property
    #[dbus_proxy(property)]
    fn evdo(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Gsm property
    #[dbus_proxy(property)]
    fn gsm(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Lte property
    #[dbus_proxy(property)]
    fn lte(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Nr5g property
    #[dbus_proxy(property)]
    fn nr5g(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Rate property
    #[dbus_proxy(property)]
    fn rate(&self) -> zbus::Result<u32>;

    /// RssiThreshold property
    #[dbus_proxy(property)]
    fn rssi_threshold(&self) -> zbus::Result<u32>;

    /// Umts property
    #[dbus_proxy(property)]
    fn umts(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;
}
