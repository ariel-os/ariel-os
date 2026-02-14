//! Common types used to configure the packet domain of a cellular connection.

/// Packet domain configuration.
///
/// Configures the networking between the modem and the cell tower.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PdConfig<'a> {
    /// Access Point Name, usually a domain given by the provider.
    pub apn: Option<&'a str>,
    /// Desired authentication parameters for the Packet Data Network.
    /// Setting this to `None` will keep the modem's default.
    pub pdn_auth: Option<PdnAuth<'a>>,
    /// Packet Domain Protocol type.
    pub pdp_type: PdpType,
}

/// Authentication parameters for the Packet Data Network (PDN).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PdnAuth<'a> {
    /// Which protocol to use to authenticate with the provider.
    pub authentication_protocol: AuthenticationProtocol,
    /// Credentials, if necessary.
    pub credentials: Option<PdnCredentials<'a>>,
}

/// Authentication protocol to authenticate to the network provider.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AuthenticationProtocol {
    /// No authentication.
    None,
    /// PAP.
    Pap,
    /// CHAP.
    Chap,
}

/// Which type of communication happens on this PDP.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdpType {
    /// IPv4.
    Ip,
    /// IPv6.
    Ipv6,
    /// Dual IP stack.
    Ipv4v6,
    /// Non-IP data.
    NonIp,
}

/// Credentials to authenticate to the network provider.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PdnCredentials<'a> {
    /// Username.
    pub username: &'a str,
    /// Password.
    pub password: &'a str,
}
