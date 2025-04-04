#![no_main]

// FAIL: the `pool_size` parameter cannot be used on autostart task
#[ariel_os::task(autostart, pool_size = 4)]
async fn main() {}
