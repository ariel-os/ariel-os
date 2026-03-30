use ariel_os_embassy_common::cellular_networking::{
    PdConfig, PdnAuthentication, PdnCredentials, PdpType,
};

#[cfg(all(feature = "ipv4", feature = "ipv6"))]
const PDP_TYPE: PdpType = PdpType::Ipv4v6;
#[cfg(all(feature = "ipv4", not(feature = "ipv6")))]
const PDP_TYPE: PdpType = PdpType::Ip;
#[cfg(all(not(feature = "ipv4"), feature = "ipv6"))]
const PDP_TYPE: PdpType = PdpType::Ipv6;
#[cfg(not(any(feature = "ipv4", feature = "ipv6")))]
const PDP_TYPE: PdpType = PdpType::NonIp;

/// Internal function to parse the `CONFIG_CELLULAR_PDN_AUTHENTICATION_PROTOCOL` string and match it with the credentials.
///
/// # Panics
///
/// To indicate an invalid configuration. This is meant to panic at build time.
const fn auth_protocol_from_str<'a>(
    str: &str,
    credentials: Option<PdnCredentials<'a>>,
) -> Option<PdnAuthentication<'a>> {
    if const_str::equal!(str, "NONE") {
        Some(PdnAuthentication::None)
    } else if const_str::equal!(str, "PAP") {
        if let Some(credentials) = credentials {
            Some(PdnAuthentication::Pap(credentials))
        } else {
            panic!(
                "PAP authentication needs CONFIG_CELLULAR_PDN_USERNAME and CONFIG_CELLULAR_PDN_PASSWORD to be set"
            )
        }
    } else if const_str::equal!(str, "CHAP") {
        if let Some(credentials) = credentials {
            Some(PdnAuthentication::Chap(credentials))
        } else {
            panic!(
                "CHAP authentication needs CONFIG_CELLULAR_PDN_USERNAME and CONFIG_CELLULAR_PDN_PASSWORD to be set"
            )
        }
    } else {
        None
    }
}

const PIN: Option<&'static str> = {
    let pin = option_env!("CONFIG_SIM_PIN");
    if let Some(pin) = pin {
        assert!(
            const_str::is_ascii!(pin),
            "CONFIG_SIM_PIN must only contain ASCII characters"
        );
    }
    pin
};

const CONFIG: PdConfig<'static> = {
    let apn = option_env!("CONFIG_CELLULAR_PDN_APN");
    let authentication_protocol = option_env!("CONFIG_CELLULAR_PDN_AUTHENTICATION_PROTOCOL");
    let username = option_env!("CONFIG_CELLULAR_PDN_USERNAME");
    let password = option_env!("CONFIG_CELLULAR_PDN_PASSWORD");

    if let Some(apn) = apn {
        assert!(
            const_str::is_ascii!(apn),
            "CONFIG_CELLULAR_PDN_APN must only contain ASCII characters"
        );
    }

    let credentials = if let Some(username) = username {
        if let Some(password) = password {
            assert!(
                const_str::is_ascii!(password),
                "CONFIG_CELLULAR_PDN_PASSWORD must only contain ASCII characters"
            );

            assert!(
                const_str::is_ascii!(username),
                "CONFIG_CELLULAR_PDN_USERNAME must only contain ASCII characters"
            );

            Some(PdnCredentials { username, password })
        } else {
            panic!(
                "If you set CONFIG_CELLULAR_PDN_USERNAME you also need to set CONFIG_CELLULAR_PDN_PASSWORD"
            );
        }
    } else {
        None
    };

    let pdn_auth = if let Some(authentication_protocol) = authentication_protocol {
        Some(
            auth_protocol_from_str(authentication_protocol, credentials)
                .expect("Invalid value for CONFIG_CELLULAR_PDN_AUTHENTICATION_PROTOCOL"),
        )
    } else {
        None
    };

    PdConfig {
        pdn_auth,
        apn,
        pdp_type: PDP_TYPE,
    }
};

/// Returns the configuration to authenticate to the cell network.
#[allow(dead_code, reason = "false positive during builds outside of laze")]
pub(crate) fn config() -> PdConfig<'static> {
    CONFIG
}

/// Returns the pin, if set, for the SIM.
#[allow(dead_code, reason = "false positive during builds outside of laze")]
pub(crate) fn pin() -> Option<&'static str> {
    PIN
}
