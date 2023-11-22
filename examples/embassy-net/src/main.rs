#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use riot_rs as _;

use riot_rs::rt::debug::println;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use riot_rs::embassy::{blocker, EXECUTOR};

static SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();

#[embassy_executor::task]
async fn async_task() {
    use embassy_time::{Duration, Timer, TICK_HZ};
    let mut counter = 0u32;
    loop {
        if counter % 2 == 0 {
            println!("async_task() signalling");
            SIGNAL.signal(counter);
        } else {
            println!("async_task()");
        }
        Timer::after(Duration::from_ticks(TICK_HZ / 10)).await;
        counter += 1;
    }
}

#[no_mangle]
fn riot_main() {
    println!(
        "Hello from riot_main()! Running on a {} board.",
        riot_rs::buildinfo::BOARD
    );

    let spawner = EXECUTOR.spawner();
    spawner.spawn(async_task()).unwrap();

    loop {
        let val = blocker::block_on(SIGNAL.wait());
        println!(
            "now={}ms threadtest() val={}",
            Instant::now().as_millis(),
            val
        );
    }
}
