#![cfg_attr(not(any(test, context = "native")), no_std)]
#![cfg_attr(test, no_main)]

#[cfg(all(feature = "rtt-target", feature = "esp-println", feature = "std"))]
compile_error!(
    r#"features "rtt-target", "esp-println" or "std" cannot be enabled at the same time"#
);

#[cfg(all(
    feature = "debug-console",
    not(any(feature = "rtt-target", feature = "esp-println", feature = "std"))
))]
compile_error!(
    r#"feature "debug-console" enabled but no backend. Select feature "rtt-target", "esp-println" or "std"."#
);

/// Represents the exit code of a debug session.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExitCode {
    #[doc(hidden)]
    Success,
    #[doc(hidden)]
    Failure,
}

impl ExitCode {
    /// The [`ExitCode`] for success.
    pub const SUCCESS: Self = Self::Success;
    /// The [`ExitCode`] for failure.
    pub const FAILURE: Self = Self::Failure;

    #[allow(dead_code, reason = "not always used due to conditional compilation")]
    fn to_semihosting_code(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::Failure => 1,
        }
    }
}

pub fn exit(code: ExitCode) {
    #[cfg(feature = "semihosting")]
    semihosting::process::exit(code.to_semihosting_code());

    #[cfg(feature = "std")]
    std::process::exit(code.to_semihosting_code());

    #[allow(unreachable_code, reason = "stop nagging")]
    let _ = code;

    #[allow(unreachable_code, reason = "fallback loop")]
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(all(feature = "debug-console", feature = "rtt-target"))]
mod backend {
    pub use rtt_target::{rprint as print, rprintln as println};

    pub fn init() {
        #[cfg(not(feature = "defmt"))]
        {
            use rtt_target::ChannelMode::NoBlockTrim;

            rtt_target::rtt_init_print!(NoBlockTrim);

            #[cfg(feature = "log")]
            crate::log::logger::init();
        }

        #[cfg(feature = "defmt")]
        {
            use rtt_target::ChannelMode::{NoBlockSkip, NoBlockTrim};
            let channels = rtt_target::rtt_init! {
                up: {
                    0: {
                        size: 1024,
                        mode: NoBlockTrim,
                        name: "Terminal"
                    }
                    1: {
                        size: 1024,
                        mode: NoBlockSkip,
                        // probe-run autodetects whether defmt is in use based on this channel name
                        name: "defmt"
                    }
                }
            };

            rtt_target::set_print_channel(channels.up.0);
            rtt_target::set_defmt_channel(channels.up.1);
        }
    }
}

#[cfg(all(feature = "debug-console", feature = "esp-println"))]
mod backend {
    pub use esp_println::{print, println};
    pub fn init() {
        // TODO: unify logging config.
        // Until then, `ESP_LOGLEVEL` can be used.
        // See https://github.com/esp-rs/esp-println#logging.
        esp_println::logger::init_logger_from_env();
    }
}

#[cfg(all(feature = "debug-console", feature = "std"))]
mod backend {
    pub use std::{print, println};

    pub fn init() {
        #[cfg(feature = "log")]
        crate::log::logger::init();
    }
}

#[cfg(not(feature = "debug-console"))]
mod backend {
    pub fn init() {}

    #[macro_export]
    macro_rules! nop_println {
        ($($arg:tt)*) => {{
            let _ = ($($arg)*);
            // Do nothing
        }};
    }

    #[macro_export]
    macro_rules! nop_print {
        ($($arg:tt)*) => {{
            let _ = ($($arg)*);
            // Do nothing
        }};
    }

    pub use nop_print as print;
    pub use nop_println as println;
}

pub use backend::*;

#[cfg(feature = "defmt")]
pub mod log {
    pub use defmt;

    #[macro_export]
    macro_rules! __trace {
        ($($arg:tt)*) => {{
            use $crate::log::defmt;
            defmt::trace!($($arg)*);
        }};
    }

    #[macro_export]
    macro_rules! __debug {
        ($($arg:tt)*) => {{
            use $crate::log::defmt;
            defmt::debug!($($arg)*);
        }};
    }

    #[macro_export]
    macro_rules! __info {
        ($($arg:tt)*) => {{
            use $crate::log::defmt;
            defmt::info!($($arg)*);
        }};
    }

    #[macro_export]
    macro_rules! __warn {
        ($($arg:tt)*) => {{
            use $crate::log::defmt;
            defmt::warn!($($arg)*);
        }};
    }

    #[macro_export]
    macro_rules! __error {
        ($($arg:tt)*) => {{
            use $crate::log::defmt;
            defmt::error!($($arg)*);
        }};
    }

    pub use __debug as debug;
    pub use __error as error;
    pub use __info as info;
    pub use __trace as trace;
    pub use __warn as warn;
}

#[cfg(feature = "log")]
pub mod log {
    pub use log;

    #[macro_export]
    macro_rules! __stub {
        ($($arg:tt)*) => {{
            let _ = ($($arg)*); // Do nothing
        }};
    }

    #[macro_export]
    macro_rules! __info {
        ($($arg:tt)*) => {{
            use $crate::log::log;
            log::info!($($arg)*);
        }};
    }

    #[macro_export]
    macro_rules! __debug {
        ($($arg:tt)*) => {{
            use $crate::log::log;
            log::debug!($($arg)*);
        }};
    }

    #[macro_export]
    macro_rules! __error {
        ($($arg:tt)*) => {{
            use $crate::log::log;
            log::error!($($arg)*);
        }};
    }

    #[macro_export]
    macro_rules! __trace {
        ($($arg:tt)*) => {{
            use $crate::log::log;
            log::trace!($($arg)*);
        }};
    }

    #[macro_export]
    macro_rules! __warn {
        ($($arg:tt)*) => {{
            use $crate::log::log;
            log::warn!($($arg)*);
        }};
    }

    pub use __debug as debug;
    pub use __error as error;
    pub use __info as info;
    pub use __trace as trace;
    pub use __warn as warn;

    pub(crate) mod logger {
        use log::{Level, LevelFilter, Metadata, Record};

        static LOGGER: DebugLogger = DebugLogger;

        struct DebugLogger;

        pub fn init() {
            let max_level = LevelFilter::Info;
            log::set_logger(&LOGGER)
                .map(|()| log::set_max_level(max_level))
                .unwrap();
            log::trace!("logging enabled");
        }

        impl log::Log for DebugLogger {
            fn enabled(&self, metadata: &Metadata) -> bool {
                metadata.level() <= Level::Info
            }

            fn log(&self, record: &Record) {
                if self.enabled(record.metadata()) {
                    crate::println!("[{}] {}", record.level(), record.args());
                }
            }

            fn flush(&self) {}
        }
    }
}
