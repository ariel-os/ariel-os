#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::debug::log::*;

#[riot_rs::task(autostart)]
async fn main() {
    info!("Hello World!");
}
