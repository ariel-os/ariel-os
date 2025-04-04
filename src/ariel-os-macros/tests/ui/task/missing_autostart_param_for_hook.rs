#![no_main]

// FAIL: using hooks require the task to be autostart
#[ariel_os::task(usb_builder_hook)]
async fn main() {}
