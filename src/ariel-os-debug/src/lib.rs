//! Provides debug interface facilities.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, no_main)]
#![deny(missing_docs)]

#[cfg(all(feature = "rtt-target", feature = "esp-println"))]
compile_error!(
    r#"feature "rtt-target" and feature "esp-println" cannot be enabled at the same time"#
);

#[cfg(all(
    feature = "debug-console",
    not(any(feature = "rtt-target", feature = "esp-println"))
))]
compile_error!(
    r#"feature "debug-console" enabled but no backend. Select feature "rtt-target" or feature "esp-println"."#
);

#[doc(inline)]
pub use ariel_os_debug_log as log;

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

/// Terminates the debug output session.
///
/// # Note
///
/// This may or may not stop the MCU.
pub fn exit(code: ExitCode) {
    #[cfg(feature = "semihosting")]
    semihosting::process::exit(code.to_semihosting_code());

    #[allow(unreachable_code, reason = "stop nagging")]
    let _ = code;

    #[allow(unreachable_code, reason = "fallback loop")]
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(all(feature = "debug-console", feature = "rtt-target"))]
mod backend {
    #[cfg(not(feature = "defmt"))]
    pub use rtt_target::rprintln as println;

    #[cfg(feature = "defmt")]
    pub use ariel_os_debug_log::println;

    #[doc(hidden)]
    pub fn init() {
        #[cfg(not(feature = "defmt"))]
        {
            use rtt_target::ChannelMode::NoBlockTrim;

            rtt_target::rtt_init_print!(NoBlockTrim);
        }

        #[cfg(feature = "log")]
        crate::logger::init();

        #[cfg(feature = "defmt")]
        {
            use rtt_target::ChannelMode::NoBlockSkip;
            const DEFMT_BUFFER_SIZE: usize = 1024;
            let channels = rtt_target::rtt_init! {
                up: {
                    0: {
                        size: DEFMT_BUFFER_SIZE,
                        mode: NoBlockSkip,
                        // probe-run autodetects whether defmt is in use based on this channel name
                        name: "defmt"
                    }
                }
            };

            rtt_target::set_defmt_channel(channels.up.0);
        }
    }
}

#[cfg(all(feature = "debug-console", feature = "esp-println"))]
mod backend {
    pub use esp_println::println;

    #[doc(hidden)]
    pub fn init() {
        #[cfg(feature = "log")]
        crate::logger::init();
    }
}

#[cfg(not(feature = "debug-console"))]
mod backend {
    #[doc(hidden)]
    pub fn init() {}

    /// Prints to the debug output, with a newline.
    #[macro_export]
    macro_rules! println {
        ($($arg:tt)*) => {{
            let _ = ($($arg)*);
            // Do nothing
        }};
    }
}

pub use backend::*;

#[doc(hidden)]
#[cfg(feature = "log")]
mod logger {
    use log::{Level, LevelFilter, Metadata, Record};

    static LOGGER: DebugLogger = DebugLogger;

    const MAX_LEVEL: LevelFilter = {
        let max_level =
            ariel_os_utils::str_from_env_or!("DEBUG_LOG_LEVEL", "info", "maximum level to log");

        // NOTE: these magic strings could likely be replaced with calls to
        // `LevelFilter::*::as_str()` if that method was const.
        if const_str::compare!(==, max_level, "trace") {
            LevelFilter::Trace
        } else if const_str::compare!(==, max_level, "debug") {
            LevelFilter::Debug
        } else if const_str::compare!(==, max_level, "info") {
            LevelFilter::Info
        } else if const_str::compare!(==, max_level, "warn") {
            LevelFilter::Warn
        } else if const_str::compare!(==, max_level, "error") {
            LevelFilter::Error
        } else if const_str::compare!(==, max_level, "off") {
            LevelFilter::Off
        } else if const_str::compare!(==, max_level, "") {
            // Default level
            LevelFilter::Info
        } else {
            panic!("invalid log level");
        }
    };

    pub fn init() {
        #[cfg(target_has_atomic = "ptr")]
        {
            log::set_logger(&LOGGER).unwrap();
            log::set_max_level(MAX_LEVEL);
        }

        // The non-racy functions are not available on architectures with no pointer-wide atomics.
        #[cfg(not(target_has_atomic = "ptr"))]
        {
            critical_section::with(|_| {
                // NOTE: these calls do not need to be made atomically but this still uses a single
                // critical section.

                // SAFETY: the critical section prevents concurrent calls of `set_logger_racy()` or
                // `logger()`.
                unsafe {
                    log::set_logger_racy(&LOGGER).unwrap();
                }
                // SAFETY: the critical section prevents concurrent calls of `set_max_level_racy()`
                // or `max_level()`.
                unsafe {
                    log::set_max_level_racy(MAX_LEVEL);
                }
            });
        }

        log::trace!("debug logging enabled");
    }

    struct DebugLogger;

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
