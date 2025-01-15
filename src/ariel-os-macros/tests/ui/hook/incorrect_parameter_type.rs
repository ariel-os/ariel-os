#![no_main]

// FAIL: incorrect parameter type
#[ariel_os::hook(usb_builder)]
fn usb_builder(builder: usize) {}
