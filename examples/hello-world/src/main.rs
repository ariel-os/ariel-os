#![no_main]
#![no_std]

use ariel_os::{
    debug::{ExitCode, exit},
    log::*,
};
use ariel_os_boards as _;

#[ariel_os::task(autostart)]
async fn main() {
    info!("Hello World!");

    exit(ExitCode::SUCCESS);
}
