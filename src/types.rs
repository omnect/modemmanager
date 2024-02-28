/// Provide reexports of modem manager types.

#[cfg(feature = "ModemManager-1_20")]
pub use modemmanager_sys::{
    MMBearerAccessTypePreference as BearerAccessTypePreference,
    MMBearerProfileSource as BearerProfileSource,
    MMBearerRoamingAllowance as BearerRoamingAllowance,
};
pub use modemmanager_sys::{
    MMBearerAllowedAuth as BearerAllowedAuth, MMBearerApnType as BearerApnType,
    MMBearerIpFamily as BearerIpFamily, MMBearerIpMethod as BearerIpMethod,
    MMBearerMultiplexSupport as BearerMultiplexSupport, MMBearerType as BearerType,
};

pub use modemmanager_sys::{
    MMModemAccessTechnology as ModemAccessTechnology, MMModemBand as ModemBand,
    MMModemCapability as ModemCapability, MMModemCdmaRmProtocol as ModemCdmaRmProtocol,
    MMModemLock as ModemLock, MMModemMode as ModemMode, MMModemPortType as ModemPortType,
    MMModemPowerState as ModemPowerState, MMModemState as ModemState,
    MMModemStateChangeReason as ModemStateChangeReason,
    MMModemStateFailedReason as ModemStateFailedReason,
};
