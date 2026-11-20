#![no_main]
#![no_std]

use ariel_os::log::*;
use ariel_os_boards as _;

#[ariel_os::thread(autostart)]
fn main() {
    match ariel_os::bench::benchmark(1000, || {
        // Insert the function to benchmark here.
        // Consider using `core::hint::black_box()` where necessary.
    }) {
        Ok(ticks) => {
            info!("took {} per iteration", ticks);
        }
        Err(err) => {
            info!("benchmark returned error: {}", err);
        }
    }
}
