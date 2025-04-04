#![no_main]

// FAIL: the `autostart` parameter must be present when requesting peripherals
#[ariel_os::task(peripherals)]
async fn main(_foo: Bar) {}

struct Bar;
