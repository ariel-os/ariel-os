#![no_main]

// FAIL: the function must be async
#[ariel_os::task]
fn main() {}
