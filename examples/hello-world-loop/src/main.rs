#![no_main]
#![no_std]

extern crate alloc;

use ariel_os::debug::println;
use ariel_os::time::Timer;
use ariel_os::storage;

#[ariel_os::task(autostart)]
async fn main() {
    println!("=== ESP32 START ===");
    println!("Getting counter...");
    
    let mut counter: u32 = storage::get("counter").await.unwrap().unwrap_or(999);
    println!("Counter: {}", counter);
    
    loop {
        println!("Loop #{}", counter);
        counter += 1;
        storage::insert("counter", counter).await.unwrap();
        Timer::after_secs(2).await;
    }
}