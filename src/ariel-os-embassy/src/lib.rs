//! This module provides an opinionated integration of `embassy`.

#![no_std]
#![cfg_attr(nightly, feature(doc_auto_cfg))]

pub mod gpio;

pub use ariel_os_hal as hal;

#[cfg(feature = "executor-thread")]
use ariel_os_embassy_common::executor_thread;

#[cfg(feature = "debug-uart")]
pub mod debug_uart;

#[cfg(feature = "i2c")]
pub mod i2c;

#[cfg(feature = "spi")]
pub mod spi;

#[cfg(feature = "usb")]
pub mod usb;

#[cfg(feature = "ble")]
pub mod ble;

#[cfg(feature = "net")]
pub mod net;

#[cfg(feature = "wifi")]
mod wifi;

#[cfg(feature = "eth")]
mod eth;

use ariel_os_debug::log::debug;

use linkme::distributed_slice;

// All items of this module are re-exported at the root of `ariel_os`.
pub mod api {
    pub use crate::{EMBASSY_TASKS, asynch, delegate, gpio, hal};

    pub mod cell {
        //! Shareable containers.

        pub use static_cell::{ConstStaticCell, StaticCell};
    }

    #[cfg(feature = "time")]
    pub mod time {
        //! Provides time-related facilities.
        // NOTE: we may want to re-export more items in the future, but not re-export the whole
        // crate.
        pub use embassy_time::{Delay, Duration, Instant, TICK_HZ, Timer};
    }

    #[cfg(feature = "ble")]
    pub use crate::ble;
    #[cfg(feature = "i2c")]
    pub use crate::i2c;
    #[cfg(feature = "net")]
    pub use crate::net;
    #[cfg(feature = "spi")]
    pub use crate::spi;
    #[cfg(feature = "usb")]
    pub use crate::usb;
}

// These are made available in `ariel_os::reexports`.
pub mod reexports {
    #[cfg(feature = "ble")]
    pub use ariel_os_embassy_common::ble;
    #[cfg(feature = "net")]
    pub use embassy_net;
    #[cfg(feature = "time")]
    pub use embassy_time;
    #[cfg(feature = "usb")]
    pub use embassy_usb;
    #[cfg(feature = "usb-hid")]
    pub use usbd_hid;
    // Used by a macro we provide
    pub use embassy_executor;
    // Used by macros for task autostarting.
    // (In most applications, it'd suffice to have this in ariel-os, but not when an internal crate
    // does an autostart)
    pub use linkme;
}

#[cfg(feature = "net")]
cfg_if::cfg_if! {
    if #[cfg(feature = "usb-ethernet")] {
        use usb::ethernet::NetworkDevice;
    } else if #[cfg(feature = "wifi")] {
        use wifi::NetworkDevice;
    } else if #[cfg(feature = "eth")] {
        use eth::NetworkDevice;
    } else if #[cfg(context = "ariel-os")] {
        compile_error!("no backend for net is active");
    } else {
        use net::DummyDriver as NetworkDevice;
    }
}

#[cfg(feature = "net")]
pub use net::NetworkStack;

pub mod asynch;
pub mod cell;
pub mod delegate;

#[cfg(feature = "executor-thread")]
pub mod thread_executor;

pub type Task = fn(asynch::Spawner, &mut hal::OptionalPeripherals);

#[doc(hidden)]
#[distributed_slice]
pub static EMBASSY_TASKS: [Task] = [..];

#[cfg(not(any(
    feature = "executor-interrupt",
    feature = "executor-none",
    feature = "executor-single-thread",
    feature = "executor-thread"
)))]
compile_error!(
    r#"must select one of "executor-interrupt", "executor-single-thread", "executor-thread", "executor-none"!"#
);

#[cfg(all(feature = "threading", feature = "executor-single-thread"))]
compile_error!(r#""executor-single-thread" and "threading" are mutually exclusive!"#);

#[cfg(feature = "executor-interrupt")]
#[distributed_slice(ariel_os_rt::INIT_FUNCS)]
pub(crate) fn init() {
    debug!("ariel-os-embassy::init(): using interrupt mode executor");
    let p = hal::init();

    #[cfg(any(context = "nrf", context = "rp", context = "stm32"))]
    {
        hal::EXECUTOR.start(hal::SWI);
        hal::EXECUTOR.spawner().must_spawn(init_task(p));
    }

    #[cfg(context = "esp")]
    EXECUTOR.run(|spawner| spawner.must_spawn(init_task(p)));
}

// SAFETY: the symbol name is unique enough to avoid accidental collisions and the function
// signature matches the one expected in `ariel-os-rt`.
#[cfg(feature = "executor-single-thread")]
#[unsafe(export_name = "__ariel_os_embassy_init")]
fn init() -> ! {
    use static_cell::StaticCell;

    debug!("ariel-os-embassy::init(): using single thread executor");
    let p = hal::init();

    static EXECUTOR: StaticCell<hal::Executor> = StaticCell::new();
    EXECUTOR
        .init_with(|| hal::Executor::new())
        .run(|spawner| spawner.must_spawn(init_task(p)))
}

#[cfg(feature = "executor-thread")]
#[ariel_os_macros::thread(autostart, no_wait, stacksize = executor_thread::STACKSIZE, priority = executor_thread::PRIORITY)]
fn init() {
    use static_cell::StaticCell;

    debug!(
        "ariel-os-embassy::init(): using thread executor with thread stack size {}",
        executor_thread::STACKSIZE
    );
    let p = hal::init();

    static EXECUTOR: StaticCell<thread_executor::Executor> = StaticCell::new();
    EXECUTOR
        .init_with(thread_executor::Executor::new)
        .run(|spawner| spawner.must_spawn(init_task(p)));
}

#[embassy_executor::task]
#[allow(clippy::too_many_lines)]
async fn init_task(mut peripherals: hal::OptionalPeripherals) {
    let spawner = asynch::Spawner::for_current_executor().await;
    asynch::set_spawner(spawner.make_send());

    #[cfg(feature = "debug-uart")]
    debug_uart::init(&mut peripherals);

    debug!("ariel-os-embassy::init_task()");

    #[cfg(all(context = "stm32", feature = "external-interrupts"))]
    hal::extint_registry::EXTINT_REGISTRY.init(&mut peripherals);

    #[cfg(feature = "i2c")]
    hal::i2c::init(&mut peripherals);

    #[cfg(feature = "spi")]
    hal::spi::init(&mut peripherals);

    #[cfg(feature = "hwrng")]
    hal::hwrng::construct_rng(&mut peripherals);
    // Clock startup and entropy collection may lend themselves to parallelization, provided that
    // doesn't impact runtime RAM or flash use.

    // Block on this Future to reduce the size of this startup task, which is statically
    // allocated.
    #[cfg(feature = "storage")]
    embassy_futures::block_on(ariel_os_storage::init(&mut peripherals));

    #[cfg(all(feature = "usb", context = "nrf"))]
    hal::usb::init();

    // Move out the peripherals required for drivers, so that tasks cannot mistakenly take them.

    #[cfg(all(feature = "ble", not(context = "rp")))]
    let ble_peripherals = hal::ble::Peripherals::new(&mut peripherals);

    #[cfg(feature = "usb")]
    let usb_peripherals = hal::usb::Peripherals::new(&mut peripherals);

    // Tasks have to be started before driver initializations so that the tasks are able to
    // configure the drivers using hooks.
    for task in EMBASSY_TASKS {
        task(spawner, &mut peripherals);
    }

    #[cfg(feature = "ble")]
    let ble_config = ble::config();
    #[cfg(all(feature = "ble", not(context = "rp")))]
    hal::ble::driver(ble_peripherals, spawner, ble_config);

    #[cfg(feature = "usb")]
    let mut usb_builder = {
        use static_cell::ConstStaticCell;

        static CONFIG_DESC: ConstStaticCell<[u8; 256]> = ConstStaticCell::new([0; 256]);
        static BOS_DESC: ConstStaticCell<[u8; 256]> = ConstStaticCell::new([0; 256]);
        static MSOS_DESC: ConstStaticCell<[u8; 128]> = ConstStaticCell::new([0; 128]);
        static CONTROL_BUF: ConstStaticCell<[u8; 128]> = ConstStaticCell::new([0; 128]);

        let usb_config = usb::config();

        let usb_driver = hal::usb::driver(usb_peripherals);

        // Create embassy-usb DeviceBuilder using the driver and config.
        usb::UsbBuilder::new(
            usb_driver,
            usb_config,
            CONFIG_DESC.take(),
            BOS_DESC.take(),
            MSOS_DESC.take(),
            CONTROL_BUF.take(),
        )
    };

    #[cfg(feature = "usb-ethernet")]
    let device = {
        use ariel_os_embassy_common::identity::DeviceId;
        use embassy_usb::class::cdc_ncm::{
            CdcNcmClass, State as CdcNcmState, embassy_net::State as NetState,
        };
        use static_cell::StaticCell;

        static CDC_ECM_STATE: StaticCell<CdcNcmState<'_>> = StaticCell::new();
        static NET_STATE: StaticCell<NetState<{ net::ETHERNET_MTU }, 4, 4>> = StaticCell::new();

        // Host's MAC addr. This is the MAC the host "thinks" its USB-to-ethernet adapter has.
        let host_mac_addr = crate::hal::identity::DeviceId::get()
            .map(|d| d.interface_eui48(1).0)
            .unwrap_or([0x8A, 0x88, 0x88, 0x88, 0x88, 0x88]);

        // Create classes on the builder.
        let usb_cdc_ecm = CdcNcmClass::new(
            &mut usb_builder,
            CDC_ECM_STATE.init_with(CdcNcmState::new),
            host_mac_addr,
            64,
        );

        let our_mac_addr = crate::hal::identity::DeviceId::get()
            .map(|d| d.interface_eui48(0).0)
            .unwrap_or([0xCA, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC]);

        let (runner, device) = usb_cdc_ecm.into_embassy_net_device::<{ net::ETHERNET_MTU }, 4, 4>(
            NET_STATE.init_with(NetState::new),
            our_mac_addr,
        );

        spawner.spawn(usb::ethernet::usb_ncm_task(runner)).unwrap();

        device
    };

    #[cfg(feature = "eth-stm32")]
    let device = hal::eth::device(&mut peripherals);

    #[cfg(feature = "usb")]
    {
        for hook in usb::USB_BUILDER_HOOKS {
            // SAFETY: `lend()` is only called once per hook instance, as required.
            unsafe {
                hook.lend(&mut usb_builder).await;
            }
        }
        let usb = usb_builder.build();
        spawner.spawn(usb::usb_task(usb)).unwrap();
    }

    #[cfg(all(feature = "ble-cyw43", not(feature = "wifi-cyw43")))]
    let _ = hal::cyw43::device(&mut peripherals, &spawner, ble_config).await;
    #[cfg(all(feature = "wifi-cyw43", not(feature = "ble-cyw43")))]
    let (device, control) = hal::cyw43::device(&mut peripherals, &spawner).await;
    #[cfg(all(feature = "ble-cyw43", feature = "wifi-cyw43"))]
    let (device, control) = hal::cyw43::device(&mut peripherals, &spawner, ble_config).await;

    #[cfg(feature = "wifi-esp")]
    let device = hal::wifi::esp_wifi::init(&mut peripherals, spawner);

    #[cfg(feature = "net")]
    {
        use embassy_net::StackResources;
        use static_cell::StaticCell;

        use crate::cell::SameExecutorCell;

        const MAX_CONCURRENT_SOCKETS: usize = ariel_os_utils::usize_from_env_or!(
            "CONFIG_NETWORK_MAX_CONCURRENT_SOCKETS",
            4,
            "maximum number of concurrent sockets allowed by the network stack"
        );

        static RESOURCES: StaticCell<StackResources<MAX_CONCURRENT_SOCKETS>> = StaticCell::new();

        #[cfg(not(any(
            feature = "usb-ethernet",
            feature = "wifi-cyw43",
            feature = "wifi-esp",
            feature = "eth"
        )))]
        // The creation of `device` is not organized in such a way that they could be put in a
        // cfg-if without larger refactoring; relying on unused variable lints to keep the
        // condition list up to date.
        let device: NetworkDevice = net::new_dummy();

        let config = net::config();

        let seed = net::unique_seed();
        debug!("Network stack seed: {:#x}", seed);

        // Init network stack
        let (stack, runner) = embassy_net::new(
            device,
            config,
            RESOURCES.init_with(StackResources::new),
            seed,
        );

        spawner.spawn(net::net_task(runner)).unwrap();

        if crate::net::STACK
            .init(SameExecutorCell::new(stack, spawner))
            .is_err()
        {
            unreachable!();
        }
    }

    #[cfg(feature = "wifi-cyw43")]
    {
        hal::cyw43::join(control).await;
    };

    // mark used
    let _ = peripherals;

    debug!("ariel-os-embassy::init_task() done");

    #[cfg(feature = "threading")]
    ariel_os_threads::events::THREAD_START_EVENT.set();
}
