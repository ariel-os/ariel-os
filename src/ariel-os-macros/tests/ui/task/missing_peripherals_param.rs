#![no_main]

// FAIL: the `peripherals` parameter is required in this case
#[ariel_os::task(autostart)]
async fn main(_peripherals: Peripherals) {}

struct Peripherals;
