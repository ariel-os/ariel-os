#![no_main]
#![no_std]

use ariel_os::debug::log::info;
use core::cell::RefCell;
use embassy_futures::join::join;
use embassy_time::{Duration, Timer};
use heapless::Deque;
use trouble_host::prelude::*;

#[ariel_os::task(autostart)]
async fn run_scanner() {
    info!("starting ble stack");

    let Host {
        central,
        mut runner,
        ..
    } = ariel_os::ble::ble_stack().await.build();

    let printer = Printer {
        seen: RefCell::new(Deque::new()),
    };
    let mut scanner = Scanner::new(central);
    let _ = join(runner.run_with_handler(&printer), async {
        let mut config = ScanConfig::default();
        config.active = true;
        config.phys = PhySet::M1;
        config.interval = Duration::from_secs(1);
        config.window = Duration::from_secs(1);
        let mut _session = scanner.scan(&config).await.unwrap();
        // Scan forever
        loop {
            info!("scanning...");
            Timer::after(Duration::from_secs(1)).await;
        }
    })
    .await;
}

struct Printer {
    seen: RefCell<Deque<BdAddr, 128>>,
}

impl EventHandler for Printer {
    fn on_adv_reports(&self, mut it: LeAdvReportsIter<'_>) {
        let mut seen = self.seen.borrow_mut();
        while let Some(Ok(report)) = it.next() {
            if seen.iter().find(|b| b.raw() == report.addr.raw()).is_none() {
                info!("discovered: {:?}", report.addr);
                if seen.is_full() {
                    seen.pop_front();
                }
                seen.push_back(report.addr).unwrap();
            }
        }
    }
}
