// #![no_std]
#![no_main]

use ariel_os::thread::{CoreAffinity, CoreId};

// FAIL: using the affinity argument requires enabling the `core-affinity` feature.
#[ariel_os::thread(autostart, affinity = CoreAffinity::one(CoreId::new(1)))]
fn main() {}
