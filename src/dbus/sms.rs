//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Sms`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Sms",
    assume_defaults = true
)]
trait Sms {
    /// Send method
    fn send(&self) -> zbus::Result<()>;

    /// Store method
    fn store(&self, storage: u32) -> zbus::Result<()>;

    /// Class property
    #[zbus(property)]
    fn class(&self) -> zbus::Result<i32>;

    /// Data property
    #[zbus(property)]
    fn data(&self) -> zbus::Result<Vec<u8>>;

    /// DeliveryReportRequest property
    #[zbus(property)]
    fn delivery_report_request(&self) -> zbus::Result<bool>;

    /// DeliveryState property
    #[zbus(property)]
    fn delivery_state(&self) -> zbus::Result<u32>;

    /// DischargeTimestamp property
    #[zbus(property)]
    fn discharge_timestamp(&self) -> zbus::Result<String>;

    /// MessageReference property
    #[zbus(property)]
    fn message_reference(&self) -> zbus::Result<u32>;

    /// Number property
    #[zbus(property)]
    fn number(&self) -> zbus::Result<String>;

    /// PduType property
    #[zbus(property)]
    fn pdu_type(&self) -> zbus::Result<u32>;

    /// SMSC property
    #[zbus(property, name = "SMSC")]
    fn smsc(&self) -> zbus::Result<String>;

    /// ServiceCategory property
    #[zbus(property)]
    fn service_category(&self) -> zbus::Result<u32>;

    /// State property
    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;

    /// Storage property
    #[zbus(property)]
    fn storage(&self) -> zbus::Result<u32>;

    /// TeleserviceId property
    #[zbus(property)]
    fn teleservice_id(&self) -> zbus::Result<u32>;

    /// Text property
    #[zbus(property)]
    fn text(&self) -> zbus::Result<String>;

    /// Timestamp property
    #[zbus(property)]
    fn timestamp(&self) -> zbus::Result<String>;

    /// Validity property
    #[zbus(property)]
    fn validity(&self) -> zbus::Result<zbus::zvariant::OwnedValue>;
}
