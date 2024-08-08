//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Modem3gpp.Ussd`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Modem3gpp.Ussd",
    assume_defaults = true
)]
trait Ussd {
    /// Cancel method
    fn cancel(&self) -> zbus::Result<()>;

    /// Initiate method
    fn initiate(&self, command: &str) -> zbus::Result<String>;

    /// Respond method
    fn respond(&self, response: &str) -> zbus::Result<String>;

    /// NetworkNotification property
    #[zbus(property)]
    fn network_notification(&self) -> zbus::Result<String>;

    /// NetworkRequest property
    #[zbus(property)]
    fn network_request(&self) -> zbus::Result<String>;

    /// State property
    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;
}
