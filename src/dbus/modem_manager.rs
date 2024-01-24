//! # DBus interface proxy for: `org.freedesktop.ModemManager1`

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.ModemManager1", assume_defaults = true)]
trait ModemManager1 {
    /// InhibitDevice method
    fn inhibit_device(&self, uid: &str, inhibit: bool) -> zbus::Result<()>;

    /// ReportKernelEvent method
    fn report_kernel_event(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// ScanDevices method
    fn scan_devices(&self) -> zbus::Result<()>;

    /// SetLogging method
    fn set_logging(&self, level: &str) -> zbus::Result<()>;

    /// Version property
    #[dbus_proxy(property)]
    fn version(&self) -> zbus::Result<String>;
}
