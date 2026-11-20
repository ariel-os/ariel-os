pub use ariel_os_debug::ExitCode;

/// TODO: add desc
pub fn exit(code: ExitCode) {
    crate::hooks();
    ariel_os_debug::exit(code);
}
