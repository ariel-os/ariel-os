#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::debug::println;

#[riot_rs::thread(autostart)]
fn main() {
    match riot_rs::rt::benchmark(10000, || {
        //
    }) {
        Ok(ticks) => {
            println!("took {} per iteration", ticks);
        }
        Err(_) => {
            println!("benchmark returned error");
        }
    }
}
