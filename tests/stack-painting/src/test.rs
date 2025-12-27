#![no_main]
#![no_std]

use core::hint::black_box;

use ariel_os::debug::{ExitCode, exit, log::info};

#[ariel_os::task(autostart)]
async fn main() {
    report_usage("1");
    do_something();

    report_usage("2");
    do_something();
    report_usage("3");

    let stack = ariel_os::rt::stack::Stack::get();
    assert!(stack.peak_usage() <= stack.total_size());
    // This requires that the reporting methods are inlined to work as expected.
    assert!(stack.current_usage() + stack.current_free_space() == stack.total_size());

    info!("Test passed!");
    exit(ExitCode::Success);
}

#[inline(always)]
fn report_usage(label: &str) {
    let stack = ariel_os::rt::stack::Stack::get();
    info!(
        "Stack usage ({}):
- {} B currently used (peak usage: {} B) out of {} B available
- {} B currently free",
        label,
        stack.current_usage(),
        stack.peak_usage(),
        stack.total_size(),
        stack.current_free_space(),
    );
}

#[inline(never)]
fn do_something() {
    let x = black_box(1) + black_box(1);
    let y = black_box(x);
    report_usage("do_something");
}
