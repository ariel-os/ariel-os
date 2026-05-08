use log::{LevelFilter, Metadata, Record};

static LOGGER: DebugLogger = DebugLogger;

const MAX_LEVEL: LevelFilter = {
    let max_level = ariel_os_utils::str_from_env_or!("LOG_LEVEL", "info", "maximum level to log");

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

/// Initializes the `log` logger.
///
/// # Panics
///
/// When a logger has already been set.
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

    log::debug!("logging enabled at level {MAX_LEVEL}");
}

struct DebugLogger;

impl log::Log for DebugLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= MAX_LEVEL
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            crate::println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
