#![no_main]

// FAIL: specifies a non-default return type
#[ariel_os::hook(usb_builder)]
fn usb_builder(builder: ariel_os::usb::UsbBuilder) -> usize {}
