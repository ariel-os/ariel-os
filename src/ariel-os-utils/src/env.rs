pub use {const_panic, konst};

macro_rules! define_env_with_default_macro {
    ($macro_name:ident, $parse_fn_name:ident, $output_type_name:literal) => {
        #[macro_export]
        macro_rules! $macro_name {
            // $doc is currently unused
            // TODO: $$(,)? should be added to allow trailing commas if this gets re-exported
            // for users, but that requires the unstable `macro_metavar_expr` feature
            ($env_var:literal, $default:expr, $doc:literal) => {
                const {
                    if let Some(str_value) = option_env!($env_var) {
                        if let Ok(value) = $crate::env::konst::primitive::$parse_fn_name(str_value)
                        {
                            value
                        } else {
                            $crate::env::const_panic::concat_panic!(
                                "Could not parse environment variable `",
                                $env_var,
                                "=",
                                str_value,
                                "` as ",
                                $output_type_name,
                            );
                        }
                    } else {
                        $default
                    }
                }
            };
        }
    };
}

define_env_with_default_macro!(usize_from_env_or, parse_usize, "a usize");
define_env_with_default_macro!(u8_from_env_or, parse_u8, "a u8");
define_env_with_default_macro!(bool_from_env_or, parse_bool, "a bool");

/// Reads a value at compile time from the given environment variable, with a default.
///
/// - The `$default` parameter allows to provide a fallback value for when the environment variable
///   is not found.
/// - The `$doc` parameter allows to provide a documentation string for this tunable (see
///   [`str_from_env!`](str_from_env)).
///
/// # Errors
///
/// Produces a compile-time error when [`option_env!`](option_env) does.
#[macro_export]
macro_rules! str_from_env_or {
    // $doc is currently unused
    ($env_var:literal, $default:expr, $doc:literal $(,)?) => {
        const {
            if let Some(str_value) = option_env!($env_var) {
                str_value
            } else {
                $default
            }
        }
    };
}

/// Reads a value at compile time from the given environment variable.
///
/// The `$doc` parameter allows to provide a documentation string for this tunable.
/// It should complete the following sentence: "This environment variable provides the `$doc`".
///
/// # Errors
///
/// - Produces a compile-time error if the environment variable is not found.
/// - Produces a compile-time error when [`option_env!`](option_env) does.
#[macro_export]
macro_rules! str_from_env {
    ($env_var:literal, $doc:literal $(,)?) => {
        const {
            if let Some(str_value) = option_env!($env_var) {
                str_value
            } else {
                $crate::env::const_panic::concat_panic!(
                    "`",
                    $env_var,
                    "` environment variable was expected to provide the ",
                    $doc,
                );
            }
        }
    };
}
#[expect(unused_imports, reason = "used for docs of str_from_env_or")]
pub(crate) use str_from_env;

/// Reads an IPv4 address at compile time from the given environment variable, produces an
/// [`Ipv4Addr`](core::net::Ipv4Addr).
///
/// The `$doc` parameter allows to provide a documentation string for this tunable (see
///   [`str_from_env!`](str_from_env)).
///
/// # Errors
///
/// - Produces a compile-time error if the environment variable is not found.
/// - Produces a compile-time error when [`option_env!`](option_env) does.
/// - Produces a compile-time error when the environment variable cannot be parsed into an IPv4
///   address.
#[macro_export]
macro_rules! ipv4_addr_from_env {
    // $doc is currently unused
    ($env_var:literal, $doc:literal $(,)?) => {
        const {
            if let Some(str_value) = option_env!($env_var) {
                $crate::const_str::ip_addr!(v4, str_value)
            } else {
                $crate::env::const_panic::concat_panic!(
                    "`",
                    $env_var,
                    "` environment variable was expected to provide the ",
                    $doc,
                );
            }
        }
    };
}

/// Reads an IPv4 address at compile time from the given environment variable, produces an
/// [`Ipv4Addr`](core::net::Ipv4Addr).
///
/// - The `$default` parameter allows to provide a fallback value for when the environment variable
///   is not found.
/// - The `$doc` parameter allows to provide a documentation string for this tunable (see
///   [`str_from_env!`](str_from_env)).
///
/// # Errors
///
/// - Produces a compile-time error when [`option_env!`](option_env) does.
/// - Produces a compile-time error when the environment variable cannot be parsed into an IPv4
///   address.
#[macro_export]
macro_rules! ipv4_addr_from_env_or {
    // $doc is currently unused
    ($env_var:literal, $default:literal, $doc:literal $(,)?) => {{
        const {
            let str_addr = if let Some(str_value) = option_env!($env_var) {
                str_value
            } else {
                $default
            };
            $crate::const_str::ip_addr!(v4, str_addr)
        }
    }};
}

/// Reads an IPv6 address at compile time from the given environment variable, produces an
/// [`Ipv6Addr`](core::net::Ipv6Addr).
///
/// The `$doc` parameter allows to provide a documentation string for this tunable (see
///   [`str_from_env!`](str_from_env)).
///
/// # Errors
///
/// - Produces a compile-time error if the environment variable is not found.
/// - Produces a compile-time error when [`option_env!`](option_env) does.
/// - Produces a compile-time error when the environment variable cannot be parsed into an IPv6
///   address.
#[macro_export]
macro_rules! ipv6_addr_from_env {
    // $doc is currently unused
    ($env_var:literal, $doc:literal $(,)?) => {
        const {
            if let Some(str_value) = option_env!($env_var) {
                $crate::const_str::ip_addr!(v6, str_value)
            } else {
                $crate::env::const_panic::concat_panic!(
                    "`",
                    $env_var,
                    "` environment variable was expected to provide the ",
                    $doc,
                );
            }
        }
    };
}

/// Reads an IPv6 address at compile time from the given environment variable, produces an
/// [`Ipv6Addr`](core::net::Ipv6Addr).
///
/// - The `$default` parameter allows to provide a fallback value for when the environment variable
///   is not found.
/// - The `$doc` parameter allows to provide a documentation string for this tunable (see
///   [`str_from_env!`](str_from_env)).
///
/// # Errors
///
/// - Produces a compile-time error when [`option_env!`](option_env) does.
/// - Produces a compile-time error when the environment variable cannot be parsed into an IPv6
///   address.
#[macro_export]
macro_rules! ipv6_addr_from_env_or {
    // $doc is currently unused
    ($env_var:literal, $default:literal, $doc:literal $(,)?) => {{
        const {
            let str_addr = if let Some(str_value) = option_env!($env_var) {
                str_value
            } else {
                $default
            };
            $crate::const_str::ip_addr!(v6, str_addr)
        }
    }};
}
