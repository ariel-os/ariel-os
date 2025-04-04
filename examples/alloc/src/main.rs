#![no_main]
#![no_std]

extern crate alloc;

use ariel_os::debug::{ExitCode, exit, log::*};

#[ariel_os::task(autostart)]
async fn main() {
    info!("Hello from `alloc` example! ");

    use alloc::vec::Vec;

    let i = 1;

    info!("creating vector and pushing {}:", i);
    let mut some_vec = Vec::new();
    some_vec.push(i);

    info!("some_vec[0]={}", some_vec[0]);

    exit(ExitCode::SUCCESS);
}
