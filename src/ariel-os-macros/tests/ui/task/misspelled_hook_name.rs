#![no_main]

// FAIL: misspelled hook name
#[ariel_os::task(autostart, usb_builder_hooook)]
async fn main() {}
