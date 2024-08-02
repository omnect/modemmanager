//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Location`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Location",
    assume_defaults = true
)]
trait Location {
    /// GetLocation method
    fn get_location(
        &self,
    ) -> zbus::Result<std::collections::HashMap<u32, zbus::zvariant::OwnedValue>>;

    /// InjectAssistanceData method
    fn inject_assistance_data(&self, data: &[u8]) -> zbus::Result<()>;

    /// SetGpsRefreshRate method
    fn set_gps_refresh_rate(&self, rate: u32) -> zbus::Result<()>;

    /// SetSuplServer method
    fn set_supl_server(&self, supl: &str) -> zbus::Result<()>;

    /// Setup method
    fn setup(&self, sources: u32, signal_location: bool) -> zbus::Result<()>;

    /// AssistanceDataServers property
    #[zbus(property)]
    fn assistance_data_servers(&self) -> zbus::Result<Vec<String>>;

    /// Capabilities property
    #[zbus(property)]
    fn capabilities(&self) -> zbus::Result<u32>;

    /// Enabled property
    #[zbus(property)]
    fn enabled(&self) -> zbus::Result<u32>;

    /// GpsRefreshRate property
    #[zbus(property)]
    fn gps_refresh_rate(&self) -> zbus::Result<u32>;

    /// Location property
    #[zbus(property)]
    fn location(&self) -> zbus::Result<std::collections::HashMap<u32, zbus::zvariant::OwnedValue>>;

    /// SignalsLocation property
    #[zbus(property)]
    fn signals_location(&self) -> zbus::Result<bool>;

    /// SuplServer property
    #[zbus(property)]
    fn supl_server(&self) -> zbus::Result<String>;

    /// SupportedAssistanceData property
    #[zbus(property)]
    fn supported_assistance_data(&self) -> zbus::Result<u32>;
}
