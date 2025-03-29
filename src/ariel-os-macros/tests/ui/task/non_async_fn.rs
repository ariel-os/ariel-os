#![no_main]
#![feature(impl_trait_in_assoc_type)]

// FAIL: the function must be async
#[ariel_os::task]
fn main() {}
