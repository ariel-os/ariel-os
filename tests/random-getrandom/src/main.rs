#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, exit, log::*};

#[ariel_os::task(autostart)]
async fn main() {
    let mut buf = [0; 4];
    for _ in 0..10 {
        getrandom::fill(&mut buf).unwrap();
        info!(
            "The random value of this round is {:08x}.",
            u32::from_be_bytes(buf)
        );
    }
    exit(ExitCode::SUCCESS);
}
