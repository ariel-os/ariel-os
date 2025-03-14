//! Executor thread configuration.

/// Stack size used by the main executor thread.
pub const STACKSIZE: usize = ariel_os_utils::usize_from_env_or!(
    "CONFIG_EXECUTOR_STACKSIZE",
    16384,
    "executor thread stack size"
);

/// Priority used by the main executor thread.
pub const PRIORITY: u8 = ariel_os_utils::u8_from_env_or!(
    "CONFIG_EXECUTOR_THREAD_PRIORITY",
    8,
    "executor thread priority"
);
