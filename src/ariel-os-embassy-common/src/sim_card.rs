//! Common SIM card types to be used across different HALs.

/// Configuration to use the SIM to connect to the cellular network
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config<'a> {
    /// Access Point Name, usually a domain given by the provider
    pub apn: &'a str,
    /// Which protocol to use to authenticate with the provider.
    pub authentication_protocol: AuthenticationProtocol,
    /// Credentials, if necessary
    pub credentials: Option<SimCredentials<'a>>,
    /// A string containing the PIN, if set.
    pub pin: Option<&'a str>,
}

/// Authentication protocol to authenticate to the network provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AuthenticationProtocol {
    /// No authentication
    None,
    /// PAP
    Pap,
    /// CHAP
    Chap,
}

/// Credentials to authenticate to the network provider
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SimCredentials<'a> {
    /// Username
    pub username: &'a str,
    /// Password
    pub password: &'a str,
}
