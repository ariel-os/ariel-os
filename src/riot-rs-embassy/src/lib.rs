//! This module provides an opinionated integration of `embassy`.

#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

pub mod define_peripherals;

#[cfg_attr(context = "nrf52", path = "arch/nrf52.rs")]
#[cfg_attr(context = "rp2040", path = "arch/rp2040.rs")]
#[cfg_attr(
    not(any(context = "nrf52", context = "rp2040")),
    path = "arch/dummy.rs"
)]
pub mod arch;

#[cfg(feature = "usb")]
mod usb;

#[cfg(feature = "net")]
pub mod network;

#[cfg(feature = "wifi_cyw43")]
mod wifi;

#[cfg(feature = "net")]
use core::cell::OnceCell;

// re-exports
pub use linkme::{self, distributed_slice};
pub use static_cell::make_static;

use crate::define_peripherals::DefinePeripheralsError;
use embassy_executor::Spawner;

#[cfg(feature = "usb")]
use usb::ethernet::NetworkDevice;

#[cfg(feature = "wifi_cyw43")]
use wifi::cyw43::NetworkDevice;

#[cfg(feature = "net")]
pub use network::NetworkStack;

#[cfg(feature = "threading")]
pub mod blocker;
pub mod sendcell;

pub type Task =
    fn(&mut arch::OptionalPeripherals) -> Result<&dyn Application, ApplicationInitError>;

#[derive(Copy, Clone)]
pub struct Drivers {
    #[cfg(feature = "net")]
    pub stack: &'static OnceCell<&'static NetworkStack>,
}

pub static EXECUTOR: arch::Executor = arch::Executor::new();

#[distributed_slice]
pub static EMBASSY_TASKS: [Task] = [..];

#[distributed_slice(riot_rs_rt::INIT_FUNCS)]
pub(crate) fn init() {
    riot_rs_rt::debug::println!("riot-rs-embassy::init()");
    let p = arch::OptionalPeripherals::from(arch::init(Default::default()));
    EXECUTOR.start(arch::SWI);
    EXECUTOR.spawner().spawn(init_task(p)).unwrap();

    riot_rs_rt::debug::println!("riot-rs-embassy::init() done");
}

#[embassy_executor::task]
async fn init_task(mut peripherals: arch::OptionalPeripherals) {
    riot_rs_rt::debug::println!("riot-rs-embassy::init_task()");

    let drivers = Drivers {
        #[cfg(feature = "net")]
        stack: make_static!(OnceCell::new()),
    };

    #[cfg(all(context = "nrf52", feature = "usb"))]
    {
        // nrf52840
        let clock: embassy_nrf::pac::CLOCK = unsafe { core::mem::transmute(()) };

        riot_rs_rt::debug::println!("nrf: enabling ext hfosc...");
        clock.tasks_hfclkstart.write(|w| unsafe { w.bits(1) });
        while clock.events_hfclkstarted.read().bits() != 1 {}
    }

    #[cfg(feature = "usb")]
    let mut usb_builder = {
        let usb_config = usb::config();

        let usb_driver = arch::usb::driver(&mut peripherals);

        // Create embassy-usb DeviceBuilder using the driver and config.
        let builder = usb::UsbBuilder::new(
            usb_driver,
            usb_config,
            &mut make_static!([0; 256])[..],
            &mut make_static!([0; 256])[..],
            &mut make_static!([0; 256])[..],
            &mut make_static!([0; 128])[..],
            &mut make_static!([0; 128])[..],
        );

        builder
    };

    // Our MAC addr.
    #[cfg(feature = "usb_ethernet")]
    let our_mac_addr = [0xCA, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC];

    #[cfg(feature = "usb_ethernet")]
    let usb_cdc_ecm = {
        // Host's MAC addr. This is the MAC the host "thinks" its USB-to-ethernet adapter has.
        let host_mac_addr = [0x8A, 0x88, 0x88, 0x88, 0x88, 0x88];

        use embassy_usb::class::cdc_ncm::{CdcNcmClass, State};

        // Create classes on the builder.
        CdcNcmClass::new(
            &mut usb_builder,
            make_static!(State::new()),
            host_mac_addr,
            64,
        )
    };

    let spawner = Spawner::for_current_executor().await;

    #[cfg(feature = "usb")]
    {
        let usb = usb_builder.build();
        spawner.spawn(usb::usb_task(usb)).unwrap();
    }

    #[cfg(feature = "usb_ethernet")]
    let device = {
        use embassy_usb::class::cdc_ncm::embassy_net::State as NetState;
        let (runner, device) = usb_cdc_ecm
            .into_embassy_net_device::<{ network::ETHERNET_MTU }, 4, 4>(
                make_static!(NetState::new()),
                our_mac_addr,
            );

        spawner.spawn(usb::ethernet::usb_ncm_task(runner)).unwrap();

        device
    };

    #[cfg(feature = "wifi_cyw43")]
    let (device, control) = {
        let (net_device, control) = wifi::cyw43::device(&mut peripherals, &spawner).await;
        (net_device, control)
    };

    #[cfg(feature = "net")]
    {
        use crate::network::STACK;
        use crate::sendcell::SendCell;
        use embassy_net::{Stack, StackResources};

        const STACK_RESOURCES: usize =
            riot_rs_utils::usize_from_env_or!("CONFIG_STACK_RESOURCES", 4);

        let config = network::config();

        // Generate random seed
        // let mut rng = Rng::new(p.RNG, Irqs);
        // let mut seed = [0; 8];
        // rng.blocking_fill_bytes(&mut seed);
        // let seed = u64::from_le_bytes(seed);
        let seed = 1234u64;

        // Init network stack
        let stack = &*make_static!(Stack::new(
            device,
            config,
            make_static!(StackResources::<STACK_RESOURCES>::new()),
            seed
        ));

        spawner.spawn(network::net_task(stack)).unwrap();

        if STACK
            .lock(|c| c.set(SendCell::new(stack, &spawner)))
            .is_err()
        {
            unreachable!();
        }
    }

    #[cfg(feature = "wifi_cyw43")]
    {
        wifi::cyw43::join(control).await;
    };

    for task in EMBASSY_TASKS {
        // TODO: should all tasks be initialized before starting the first one?
        match task(&mut peripherals) {
            Ok(initialized_application) => initialized_application.start(spawner, drivers),
            Err(err) => panic!("Error while initializing an application: {err:?}"),
        }
    }

    // mark used
    let _ = peripherals;

    riot_rs_rt::debug::println!("riot-rs-embassy::init_task() done");
}

/// Defines an application.
///
/// Allows to separate its fallible initialization from its infallible running phase.
pub trait Application {
    /// Applications must implement this to obtain the peripherals they require.
    ///
    /// This function is only run once at startup and instantiates the application.
    /// No guarantee is provided regarding the order in which different applications are
    /// initialized.
    /// The [`assign_resources!`] macro can be leveraged to extract the required peripherals.
    fn initialize(
        peripherals: &mut arch::OptionalPeripherals,
    ) -> Result<&dyn Application, ApplicationInitError>
    where
        Self: Sized;

    /// After an application has been initialized, this method is called by the system to start the
    /// application.
    ///
    /// This function must not block but may spawn [Embassy tasks](embassy_executor::task) using
    /// the provided [`Spawner`](embassy_executor::Spawner).
    /// In addition, it is provided with the drivers initialized by the system.
    fn start(&self, spawner: embassy_executor::Spawner, drivers: Drivers);
}

/// Represents errors that can happen during application initialization.
#[derive(Debug)]
pub enum ApplicationInitError {
    /// The application could not obtain a peripheral, most likely because it was already used by
    /// another application or by the system itself.
    CannotTakePeripheral,
}

impl From<DefinePeripheralsError> for ApplicationInitError {
    fn from(err: DefinePeripheralsError) -> Self {
        match err {
            DefinePeripheralsError::TakingPeripheral => Self::CannotTakePeripheral,
        }
    }
}

/// Sets the [`Application::initialize()`] function implemented on the provided type to be run at
/// startup.
#[macro_export]
macro_rules! riot_initialize {
    ($prog_type:ident) => {
        #[$crate::distributed_slice($crate::EMBASSY_TASKS)]
        #[linkme(crate = $crate::linkme)]
        fn __init_application(
            peripherals: &mut $crate::arch::OptionalPeripherals,
        ) -> Result<&dyn $crate::Application, $crate::ApplicationInitError> {
            <$prog_type as Application>::initialize(peripherals)
        }
    };
}
