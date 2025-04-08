#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, exit, log::*};

use ariel_os::rt::stack::*;

#[ariel_os::thread(autostart)]
fn main() {
    let limits = Stack::get();
    let free_min = limits.free_min();
    let used_max = limits.used_max();

    info!(
        "size: {} used: {} free: {} free_min: {} used_max: {}",
        limits.size(),
        limits.used(),
        limits.free(),
        free_min,
        used_max,
    );

    limits.repaint();

    let free_min = limits.free_min();
    let used_max = limits.used_max();

    info!(
        "size: {} used: {} free: {} free_min: {} used_max: {}",
        limits.size(),
        limits.used(),
        limits.free(),
        free_min,
        used_max,
    );

    let free_min = limits.free_min();
    let used_max = limits.used_max();

    info!(
        "size: {} used: {} free: {} free_min: {} used_max: {}",
        limits.size(),
        limits.used(),
        limits.free(),
        free_min,
        used_max,
    );

    exit(ExitCode::SUCCESS);
}
