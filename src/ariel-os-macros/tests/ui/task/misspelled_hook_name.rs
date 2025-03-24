#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(used_with_arg)]

// FAIL: misspelled hook name
#[ariel_os::task(autostart, usb_builder_hooook)]
async fn main() {}
