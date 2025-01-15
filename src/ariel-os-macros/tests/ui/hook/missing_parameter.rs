#![no_main]

// FAIL: missing parameter
#[ariel_os::hook(usb_builder)]
fn usb_builder() {}
