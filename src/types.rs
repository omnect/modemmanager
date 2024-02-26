/// Provide reexports of modem manager types.

#[cfg(feature = "ModemManager-1_22")]
pub use modemmanager_sys::MMBearerAccessTypePreference as BearerAccessTypePreference;
pub use modemmanager_sys::MMBearerAllowedAuth as BearerAllowedAuth;
pub use modemmanager_sys::MMBearerApnType as BearerApnType;
pub use modemmanager_sys::MMBearerIpFamily as BearerIpFamily;
pub use modemmanager_sys::MMBearerIpMethod as BearerIpMethod;
pub use modemmanager_sys::MMBearerMultiplexSupport as BearerMultiplexSupport;
#[cfg(feature = "ModemManager-1_22")]
pub use modemmanager_sys::MMBearerProfileSource as BearerProfileSource;
#[cfg(feature = "ModemManager-1_22")]
pub use modemmanager_sys::MMBearerRoamingAllowance as BearerRoamingAllowance;
pub use modemmanager_sys::MMBearerType as BearerType;

pub use modemmanager_sys::MMModemAccessTechnology as ModemAccessTechnology;
pub use modemmanager_sys::MMModemBand as ModemBand;
pub use modemmanager_sys::MMModemCapability as ModemCapability;
pub use modemmanager_sys::MMModemCdmaRmProtocol as ModemCdmaRmProtocol;
pub use modemmanager_sys::MMModemLock as ModemLock;
pub use modemmanager_sys::MMModemMode as ModemMode;
pub use modemmanager_sys::MMModemPortType as ModemPortType;
pub use modemmanager_sys::MMModemPowerState as ModemPowerState;
pub use modemmanager_sys::MMModemState as ModemState;
pub use modemmanager_sys::MMModemStateChangeReason as ModemStateChangeReason;
pub use modemmanager_sys::MMModemStateFailedReason as ModemStateFailedReason;
