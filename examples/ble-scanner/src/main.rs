//! Adapted from the example in `trouble_host`
#![no_main]
#![no_std]

use core::cell::RefCell;

use embassy_futures::join::join;
use heapless::Deque;
use trouble_host::{
    Host,
    connection::{PhySet, ScanConfig},
    prelude::{BdAddr, EventHandler, LeAdvReportsIter},
    scan::Scanner,
};

use ariel_os::{
    debug::log::info,
    time::{Duration, Timer},
};

#[ariel_os::task(autostart)]
async fn run_scanner() {
    info!("starting ble stack");
    let stack = ariel_os::ble::ble_stack().await;

    let Host {
        central,
        mut runner,
        ..
    } = stack.build();

    let printer = Printer {
        seen: RefCell::new(Deque::new()),
    };
    let mut scanner = Scanner::new(central);
    let _ = join(runner.run_with_handler(&printer), async {
        let config = ScanConfig::<'_> {
            active: true,
            phys: PhySet::M1,
            interval: Duration::from_secs(1),
            window: Duration::from_secs(1),
            ..Default::default()
        };
        let mut _session = scanner.scan(&config).await.unwrap();
        // Scan forever
        loop {
            info!("scanning...");
            Timer::after_secs(1).await;
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
            if !seen.iter().any(|b| b.raw() == report.addr.raw()) {
                info!("discovered: {:?}", report.addr);
                if seen.is_full() {
                    seen.pop_front();
                }
                seen.push_back(report.addr).unwrap();
            }
        }
    }
}
