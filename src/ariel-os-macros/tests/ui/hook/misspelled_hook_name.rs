#![no_main]

// FAIL: misspelled hook name
#[ariel_os::hook(usb_buildeeer)]
fn usb_builder(builder: ariel_os::usb::UsbBuilder) {}
