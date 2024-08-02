//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Call`

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.ModemManager1.Call",
    assume_defaults = true
)]
trait Call {
    /// Accept method
    fn accept(&self) -> zbus::Result<()>;

    /// Deflect method
    fn deflect(&self, number: &str) -> zbus::Result<()>;

    /// Hangup method
    fn hangup(&self) -> zbus::Result<()>;

    /// JoinMultiparty method
    fn join_multiparty(&self) -> zbus::Result<()>;

    /// LeaveMultiparty method
    fn leave_multiparty(&self) -> zbus::Result<()>;

    /// SendDtmf method
    fn send_dtmf(&self, dtmf: &str) -> zbus::Result<()>;

    /// Start method
    fn start(&self) -> zbus::Result<()>;

    /// DtmfReceived signal
    #[zbus(signal)]
    fn dtmf_received(&self, dtmf: &str) -> zbus::Result<()>;

    /// StateChanged signal
    #[zbus(signal, name = "state_changed")]
    fn state_changed_func(&self, old: i32, new: i32, reason: u32) -> zbus::Result<()>;

    /// AudioFormat property
    #[zbus(property)]
    fn audio_format(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// AudioPort property
    #[zbus(property)]
    fn audio_port(&self) -> zbus::Result<String>;

    /// Direction property
    #[zbus(property)]
    fn direction(&self) -> zbus::Result<i32>;

    /// Multiparty property
    #[zbus(property)]
    fn multiparty(&self) -> zbus::Result<bool>;

    /// Number property
    #[zbus(property)]
    fn number(&self) -> zbus::Result<String>;

    /// State property
    #[zbus(property)]
    fn state(&self) -> zbus::Result<i32>;

    /// StateReason property
    #[zbus(property)]
    fn state_reason(&self) -> zbus::Result<i32>;
}
