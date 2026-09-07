// #![no_std]
#![no_main]

use ariel_os::thread::{CoreAffinity, CoreId};

// FAIL: the `autostart` parameter is mandatory
#[ariel_os::thread(autostart, affinity = CoreAffinity::one(CoreId::new(1)))]
fn main() {}
