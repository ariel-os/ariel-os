#![no_main]
#![no_std]

use ariel_os::{debug::log::info, thread};

#[ariel_os::thread(autostart)]
fn thread0() {
    info!("Hello from thread 0");
    let mut a: f32 = 1.111;
    let mut b = 2.222;
    let mut test = 0.0;
    for _ in 0..10 {
        a += 1.1234;
        b += 2.5678;

        test += a + b;
        thread::yield_same();
    }

    assert_eq!(test, 236.34601_f32);
    info!("Thread 0 success");
}

#[ariel_os::thread(autostart)]
fn thread1() {
    info!("Hello from thread 1");
    let mut a: f32 = 3.333;
    let mut b = 4.444;
    let mut test = 0.0;
    for _ in 0..10 {
        a += 3.4321;
        b += 4.8765;

        test += a * b;

        thread::yield_same();
    }

    assert_eq!(test, 8324.532_f32);
    info!("Thread 1 success");
}
