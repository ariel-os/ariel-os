#![no_main]
#![no_std]

use ariel_os::{
    log::*,
    power::{ExitCode, exit},
};

#[ariel_os::task(autostart)]
async fn main() {
    info!("Hello World!");

    exit(ExitCode::SUCCESS);
}
