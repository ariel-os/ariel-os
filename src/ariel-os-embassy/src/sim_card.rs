use ariel_os_embassy_common::sim_card::{AuthenticationProtocol, Config, SimCredentials};

const fn auth_protocol_from_str(str: &str) -> Option<AuthenticationProtocol> {
    if const_str::equal!(str, "NONE") {
        Some(AuthenticationProtocol::None)
    } else if const_str::equal!(str, "PAP") {
        Some(AuthenticationProtocol::Pap)
    } else if const_str::equal!(str, "CHAP") {
        Some(AuthenticationProtocol::Chap)
    } else {
        None
    }
}

const CONFIG: Option<Config<'static>> = {
    let apn = option_env!("CONFIG_SIM_APN");
    let authentication_protocol = option_env!("CONFIG_SIM_AUTHENTICATION_PROTOCOL");
    let username = option_env!("CONFIG_SIM_USERNAME");
    let password = option_env!("CONFIG_SIM_PASSWORD");
    let pin = option_env!("CONFIG_SIM_PIN");

    let credentials = if let Some(username) = username {
        if let Some(password) = password {
            Some(SimCredentials { username, password })
        } else {
            panic!("If you set CONFIG_SIM_USERNAME you also need to set CONFIG_SIM_PASSWORD");
        }
    } else {
        None
    };

    if let Some(apn) = apn {
        if let Some(authentication_protocol) = authentication_protocol {
            let Some(authentication_protocol) = auth_protocol_from_str(authentication_protocol)
            else {
                panic!("Invalid value for CONFIG_SIM_AUTHENTICATION_PROTOCOL",);
            };

            Some(Config {
                apn,
                authentication_protocol,
                credentials,
                pin,
            })
        } else {
            panic!(
                "If you set CONFIG_SIM_APN you also need to set at least CONFIG_SIM_AUTHENTICATION_PROTOCOL",
            );
        }
    } else {
        None
    }
};

/// Returns the configuration to authenticate to the SIM card and provider
pub fn config() -> Option<Config<'static>> {
    CONFIG
}
