#![no_main]

// FAIL: missing hook
#[ariel_os::hook]
fn usb_builder() {}
