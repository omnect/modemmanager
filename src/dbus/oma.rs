//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Oma`

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Oma",
    assume_defaults = true
)]
trait Oma {
    /// AcceptNetworkInitiatedSession method
    fn accept_network_initiated_session(&self, session_id: u32, accept: bool) -> zbus::Result<()>;

    /// CancelSession method
    fn cancel_session(&self) -> zbus::Result<()>;

    /// Setup method
    fn setup(&self, features: u32) -> zbus::Result<()>;

    /// StartClientInitiatedSession method
    fn start_client_initiated_session(&self, session_type: u32) -> zbus::Result<()>;

    /// SessionStateChanged signal
    #[dbus_proxy(signal, name = "session_state_changed")]
    fn session_state_changed_func(
        &self,
        old_session_state: i32,
        new_session_state: i32,
        session_state_failed_reason: u32,
    ) -> zbus::Result<()>;

    /// Features property
    #[dbus_proxy(property)]
    fn features(&self) -> zbus::Result<u32>;

    /// PendingNetworkInitiatedSessions property
    #[dbus_proxy(property)]
    fn pending_network_initiated_sessions(&self) -> zbus::Result<Vec<(u32, u32)>>;

    /// SessionState property
    #[dbus_proxy(property)]
    fn session_state(&self) -> zbus::Result<i32>;

    /// SessionType property
    #[dbus_proxy(property)]
    fn session_type(&self) -> zbus::Result<u32>;
}
