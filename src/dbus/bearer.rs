//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Bearer`

use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use modemmanager_sys::{
    MMBearerAllowedAuth, MMBearerApnType, MMBearerIpFamily, MMBearerIpMethod,
    MMBearerMultiplexSupport, MMBearerType, MMModemCdmaRmProtocol,
};

#[cfg(feature = "ModemManager-1_20")]
use modemmanager_sys::{
    MMBearerAccessTypePreference, MMBearerProfileSource, MMBearerRoamingAllowance,
};

use zbus::dbus_proxy;
use zbus::zvariant::{Dict, OwnedValue};

use num::FromPrimitive;

pub struct Stats {
    pub rx_bytes: Option<u64>,
    pub tx_bytes: Option<u64>,
    pub duration: Option<u32>,
    pub attempts: Option<u32>,
    pub failed_attempts: Option<u32>,
    pub total_rx_bytes: Option<u64>,
    pub total_tx_bytes: Option<u64>,
    pub total_duration: Option<u32>,
}

// we can't use derive DeserializeDict with optional members:
// https://github.com/dbus2/zbus/issues/311, manually implement TryFrom
impl TryFrom<OwnedValue> for Stats {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let values_dict: Dict<'_, '_> = value.try_into()?;

        let rx_bytes = values_dict.get("rx-bytes")?.cloned();
        let tx_bytes = values_dict.get("tx-bytes")?.cloned();
        let duration = values_dict.get("duration")?.cloned();
        let attempts = values_dict.get("attempts")?.cloned();
        let failed_attempts = values_dict.get("failed-attempts")?.cloned();
        let total_rx_bytes = values_dict.get("total-rx-bytes")?.cloned();
        let total_tx_bytes = values_dict.get("total-tx-bytes")?.cloned();
        let total_duration = values_dict.get("total-duration")?.cloned();

        Ok(Stats {
            rx_bytes,
            tx_bytes,
            duration,
            attempts,
            failed_attempts,
            total_rx_bytes,
            total_tx_bytes,
            total_duration,
        })
    }
}

#[cfg(not(feature = "ModemManager-1_20"))]
pub struct Prop3Gpp {
    pub apn: String,
    pub ip_type: MMBearerIpFamily,
    pub apn_type: MMBearerApnType,
    pub allowed_auth: MMBearerAllowedAuth,
    pub user: String,
    pub password: String,
    pub profile_id: String,
    pub profile_name: String,
    pub profile_enabled: bool,
    pub allow_roaming: bool,
    pub multiplex: MMBearerMultiplexSupport,
}

#[cfg(feature = "ModemManager-1_20")]
pub struct Prop3Gpp {
    pub apn: String,
    pub ip_type: MMBearerIpFamily,
    pub apn_type: MMBearerApnType,
    pub allowed_auth: MMBearerAllowedAuth,
    pub user: String,
    pub password: String,
    pub access_type_preference: MMBearerAccessTypePreference,
    pub roaming_allowance: MMBearerRoamingAllowance,
    pub profile_id: String,
    pub profile_name: String,
    pub profile_enabled: bool,
    pub profile_source: MMBearerProfileSource,
    pub allow_roaming: bool,
    pub multiplex: MMBearerMultiplexSupport,
}

impl TryFrom<Dict<'_, '_>> for Prop3Gpp {
    type Error = zbus::Error;
    #[cfg(not(feature = "ModemManager-1_20"))]
    fn try_from(values: Dict) -> Result<Self, Self::Error> {
        let apn: &str = values.get("apn")?.ok_or(zbus::Error::InvalidField)?;
        let ip_type: u32 = values
            .get("ip-type")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let ip_type = MMBearerIpFamily::from_u32(ip_type).ok_or(zbus::Error::InvalidField)?;
        let apn_type: u32 = values
            .get("apn_type")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let apn_type = MMBearerApnType::from_u32(apn_type).ok_or(zbus::Error::InvalidField)?;
        let allowed_auth: u32 = values
            .get("allowed-auth")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let allowed_auth =
            MMBearerAllowedAuth::from_u32(allowed_auth).ok_or(zbus::Error::InvalidField)?;
        let user: &str = values.get("user")?.ok_or(zbus::Error::InvalidField)?;
        let password: &str = values.get("password")?.ok_or(zbus::Error::InvalidField)?;
        let profile_id: &str = values.get("profile-id")?.ok_or(zbus::Error::InvalidField)?;
        let profile_name: &str = values
            .get("profile-name")?
            .ok_or(zbus::Error::InvalidField)?;
        let profile_enabled: bool = *values
            .get("profile-enabled")?
            .ok_or(zbus::Error::InvalidField)?;
        let allow_roaming: bool = *values
            .get("allow-roaming")?
            .ok_or(zbus::Error::InvalidField)?;
        let multiplex: u32 = values
            .get("multiplex")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let multiplex =
            MMBearerMultiplexSupport::from_u32(multiplex).ok_or(zbus::Error::InvalidField)?;

        Ok(Prop3Gpp {
            apn: apn.to_string(),
            ip_type,
            apn_type,
            allowed_auth,
            user: user.to_string(),
            password: password.to_string(),
            profile_id: profile_id.to_string(),
            profile_name: profile_name.to_string(),
            profile_enabled,
            allow_roaming,
            multiplex,
        })
    }

    #[cfg(feature = "ModemManager-1_20")]
    fn try_from(values: Dict) -> Result<Self, Self::Error> {
        let apn: &str = values.get("apn")?.ok_or(zbus::Error::InvalidField)?;
        let ip_type: u32 = values
            .get("ip-type")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let ip_type = MMBearerIpFamily::from_u32(ip_type).ok_or(zbus::Error::InvalidField)?;
        let apn_type: u32 = values
            .get("apn_type")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let apn_type = MMBearerApnType::from_u32(apn_type).ok_or(zbus::Error::InvalidField)?;
        let allowed_auth: u32 = values
            .get("allowed-auth")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let allowed_auth =
            MMBearerAllowedAuth::from_u32(allowed_auth).ok_or(zbus::Error::InvalidField)?;
        let user: &str = values.get("user")?.ok_or(zbus::Error::InvalidField)?;
        let password: &str = values.get("password")?.ok_or(zbus::Error::InvalidField)?;
        let access_type_preference: u32 = values
            .get("access-type-preference")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let access_type_preference = MMBearerAccessTypePreference::from_u32(access_type_preference)
            .ok_or(zbus::Error::InvalidField)?;
        let roaming_allowance: u32 = values
            .get("roaming-allowance")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let roaming_allowance = MMBearerRoamingAllowance::from_u32(roaming_allowance)
            .ok_or(zbus::Error::InvalidField)?;
        let profile_id: &str = values.get("profile-id")?.ok_or(zbus::Error::InvalidField)?;
        let profile_name: &str = values
            .get("profile-name")?
            .ok_or(zbus::Error::InvalidField)?;
        let profile_enabled: bool = *values
            .get("profile-enabled")?
            .ok_or(zbus::Error::InvalidField)?;
        let profile_source: u32 = values
            .get("profile-source")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let profile_source =
            MMBearerProfileSource::from_u32(profile_source).ok_or(zbus::Error::InvalidField)?;
        let allow_roaming: bool = *values
            .get("allow-roaming")?
            .ok_or(zbus::Error::InvalidField)?;
        let multiplex: u32 = values
            .get("multiplex")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let multiplex =
            MMBearerMultiplexSupport::from_u32(multiplex).ok_or(zbus::Error::InvalidField)?;

        Ok(Prop3Gpp {
            apn: apn.to_string(),
            ip_type,
            apn_type,
            allowed_auth,
            user: user.to_string(),
            password: password.to_string(),
            access_type_preference,
            roaming_allowance,
            profile_id: profile_id.to_string(),
            profile_name: profile_name.to_string(),
            profile_enabled,
            profile_source,
            allow_roaming,
            multiplex,
        })
    }
}

pub struct Prop3Gpp2 {
    pub rm_protocol: MMModemCdmaRmProtocol,
    pub allow_roaming: bool,
    pub multiplex: MMBearerMultiplexSupport,
}

impl TryFrom<Dict<'_, '_>> for Prop3Gpp2 {
    type Error = zbus::Error;

    fn try_from(values: Dict) -> Result<Self, Self::Error> {
        let rm_protocol: u32 = values
            .get("rm-protocol")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let rm_protocol =
            MMModemCdmaRmProtocol::from_u32(rm_protocol).ok_or(zbus::Error::InvalidField)?;

        let allow_roaming: bool = values
            .get("allow-roaming")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;

        let multiplex: u32 = values
            .get("multiplex")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let multiplex =
            MMBearerMultiplexSupport::from_u32(multiplex).ok_or(zbus::Error::InvalidField)?;

        Ok(Prop3Gpp2 {
            rm_protocol,
            allow_roaming,
            multiplex,
        })
    }
}

pub enum Properties {
    Prop3Gpp(Prop3Gpp),
    Prop3Gpp2(Prop3Gpp2),
}

impl TryFrom<OwnedValue> for Properties {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let values_dict: Dict = value.try_into()?;

        let apn: Result<Option<&str>, _> = values_dict.get("apn");

        if let Ok(Some(_)) = apn {
            return Ok(Properties::Prop3Gpp(values_dict.try_into()?));
        }

        Ok(Properties::Prop3Gpp2(values_dict.try_into()?))
    }
}

pub struct IpConfig<IpAddrType> {
    pub method: MMBearerIpMethod,
    pub address: Option<IpAddrType>,
    pub prefix: Option<u32>,
    pub dns1: Option<IpAddrType>,
    pub dns2: Option<IpAddrType>,
    pub dns3: Option<IpAddrType>,
    pub gateway: Option<IpAddrType>,
    pub mtu: Option<u32>,
}

// manually implement TryFrom
impl<AddrType: FromStr> TryFrom<OwnedValue> for IpConfig<AddrType> {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let values_dict: Dict<'_, '_> = value.try_into()?;

        let method: u32 = values_dict
            .get("method")?
            .cloned()
            .ok_or(zbus::Error::InvalidField)?;
        let method = MMBearerIpMethod::from_u32(method).ok_or(zbus::Error::InvalidField)?;
        let address = values_dict
            .get("address")?
            .map(|addr| AddrType::from_str(addr).or(Err(zbus::Error::InvalidField)))
            .transpose()?;
        let prefix = values_dict.get("prefix")?.cloned();
        let dns1 = values_dict
            .get("dns1")?
            .map(|addr| AddrType::from_str(addr).or(Err(zbus::Error::InvalidField)))
            .transpose()?;
        let dns2 = values_dict
            .get("dns2")?
            .map(|addr| AddrType::from_str(addr).or(Err(zbus::Error::InvalidField)))
            .transpose()?;
        let dns3 = values_dict
            .get("dns3")?
            .map(|addr| AddrType::from_str(addr).or(Err(zbus::Error::InvalidField)))
            .transpose()?;
        let gateway = values_dict
            .get("gateway")?
            .map(|addr| AddrType::from_str(addr).or(Err(zbus::Error::InvalidField)))
            .transpose()?;
        let mtu = values_dict.get("mtu")?.cloned();

        Ok(IpConfig::<AddrType> {
            method,
            address,
            prefix,
            dns1,
            dns2,
            dns3,
            gateway,
            mtu,
        })
    }
}

#[dbus_proxy(
    interface = "org.freedesktop.ModemManager1.Bearer",
    assume_defaults = true
)]
trait Bearer {
    /// Connect method
    fn connect(&self) -> zbus::Result<()>;

    /// Disconnect method
    fn disconnect(&self) -> zbus::Result<()>;

    /// BearerType property
    #[dbus_proxy(property)]
    fn bearer_type(&self) -> zbus::Result<MMBearerType>;

    /// Connected property
    #[dbus_proxy(property)]
    fn connected(&self) -> zbus::Result<bool>;

    /// ConnectionError property
    #[dbus_proxy(property)]
    fn connection_error(&self) -> zbus::Result<(String, String)>;

    /// Interface property
    #[dbus_proxy(property)]
    fn interface(&self) -> zbus::Result<String>;

    /// Ip4Config property
    #[dbus_proxy(property)]
    fn ip4_config(&self) -> zbus::Result<IpConfig<Ipv4Addr>>;

    /// Ip6Config property
    #[dbus_proxy(property)]
    fn ip6_config(&self) -> zbus::Result<IpConfig<Ipv6Addr>>;

    /// IpTimeout property
    #[dbus_proxy(property)]
    fn ip_timeout(&self) -> zbus::Result<u32>;

    /// Multiplexed property
    #[dbus_proxy(property)]
    fn multiplexed(&self) -> zbus::Result<bool>;

    /// ProfileId property
    #[dbus_proxy(property)]
    fn profile_id(&self) -> zbus::Result<i32>;

    /// Properties property
    #[dbus_proxy(property)]
    fn properties(&self) -> zbus::Result<Properties>;

    /// ReloadStatsSupported property
    #[dbus_proxy(property)]
    fn reload_stats_supported(&self) -> zbus::Result<bool>;

    /// Stats property
    #[dbus_proxy(property)]
    fn stats(&self) -> zbus::Result<Stats>;

    /// Suspended property
    #[dbus_proxy(property)]
    fn suspended(&self) -> zbus::Result<bool>;
}
