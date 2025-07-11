// This is the entrypoint that libc executes.
//
// We specify this here (in `ariel-os-rt`) because Ariel OS applications don't
// specify a `fn main()`, and we cannot do that here.
//
// Unfortunately this bypasses some of Rust's setup, as that is private:
// - this does not use `std::rt::lang_start()`
// - at least on musl, this doesn't setup `args()`
//
// SAFETY: we are *the* main
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    crate::startup();
}

pub fn init() {}

/// Returns a `Stack` handle for the currently active thread.
pub(crate) fn stack() -> crate::stack::Stack {
    // Returning (0,0) is safe, but stack usage data will be empty.
    crate::stack::Stack::new(0, 0)
}

/// Returns the current `SP` register value
pub(crate) fn sp() -> usize {
    0
}
