#![no_main]
#![feature(impl_trait_in_assoc_type)]

// FAIL: the `peripherals` parameter is required in this case
#[ariel_os::task(autostart)]
async fn main(_peripherals: Peripherals) {}

struct Peripherals;
