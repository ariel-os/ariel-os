#![no_main]
#![feature(impl_trait_in_assoc_type)]

// FAIL: misspelled hook name
#[ariel_os::task(autostart, usb_builder_hooook)]
async fn main() {}
