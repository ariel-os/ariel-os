#![no_main]

// FAIL: the function is expected to take a type having a `take_peripherals()` method as first
// parameter
#[ariel_os::task(autostart, peripherals)]
async fn main(_foo: Bar) {}

struct Bar;
