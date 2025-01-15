#![no_main]

// FAIL: the function must not be async
#[ariel_os::hook(usb_builder)]
async fn main() {}
