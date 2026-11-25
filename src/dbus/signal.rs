//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Signal`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Signal",
    assume_defaults = true
)]
pub trait Signal {
    /// Setup method
    fn setup(&self, rate: u32) -> zbus::Result<()>;

    /// SetupThresholds method
    fn setup_thresholds(
        &self,
        settings: std::collections::HashMap<&str, zbus::Variant<'_>>,
    ) -> zbus::Result<()>;

    /// Cdma property
    #[zbus(property)]
    fn cdma(&self) -> zbus::Result<std::collections::HashMap<String, zbus::OwnedVariant>>;

    /// ErrorRateThreshold property
    #[zbus(property)]
    fn error_rate_threshold(&self) -> zbus::Result<bool>;

    /// Evdo property
    #[zbus(property)]
    fn evdo(&self) -> zbus::Result<std::collections::HashMap<String, zbus::OwnedVariant>>;

    /// Gsm property
    #[zbus(property)]
    fn gsm(&self) -> zbus::Result<std::collections::HashMap<String, zbus::OwnedVariant>>;

    /// Lte property
    #[zbus(property)]
    fn lte(&self) -> zbus::Result<std::collections::HashMap<String, zbus::OwnedVariant>>;

    /// Nr5g property
    #[zbus(property)]
    fn nr5g(&self) -> zbus::Result<std::collections::HashMap<String, zbus::OwnedVariant>>;

    /// Rate property
    #[zbus(property)]
    fn rate(&self) -> zbus::Result<u32>;

    /// RssiThreshold property
    #[zbus(property)]
    fn rssi_threshold(&self) -> zbus::Result<u32>;

    /// Umts property
    #[zbus(property)]
    fn umts(&self) -> zbus::Result<std::collections::HashMap<String, zbus::OwnedVariant>>;
}
