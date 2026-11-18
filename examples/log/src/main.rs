#![no_main]
#![no_std]

use ariel_os::log::*;
use ariel_os_boards as _;

#[ariel_os::task(autostart)]
async fn main() {
    println!("-- this is printed via `println!()`");
    trace!("-- trace log level enabled");
    debug!("-- debug log level enabled");
    info!("-- info log level enabled");
    warn!("-- warn log level enabled");
    error!("-- error log level enabled (just testing)");
}
