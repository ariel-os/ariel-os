#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(used_with_arg)]

// FAIL: using hooks require the task to be autostart
#[ariel_os::task(usb_builder_hook)]
async fn main() {}
