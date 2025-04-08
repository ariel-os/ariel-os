#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, exit, log::*};

use ariel_os::rt::stack::*;

#[ariel_os::task(autostart)]
async fn main() {
    let stack = Stack::get();

    let free_min = stack.free_min();
    let used_max = stack.used_max();

    info!(
        "size: {} used: {} free: {} free_min: {} used_max: {}",
        stack.size(),
        stack.used(),
        stack.free(),
        free_min,
        used_max,
    );

    stack.repaint();

    let free_min = stack.free_min();
    let used_max = stack.used_max();

    info!(
        "size: {} used: {} free: {} free_min: {} used_max: {}",
        stack.size(),
        stack.used(),
        stack.free(),
        free_min,
        used_max,
    );

    let free_min = stack.free_min();
    let used_max = stack.used_max();

    info!(
        "size: {} used: {} free: {} free_min: {} used_max: {}",
        stack.size(),
        stack.used(),
        stack.free(),
        free_min,
        used_max,
    );

    exit(ExitCode::SUCCESS);
}
