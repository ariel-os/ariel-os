//! Items specific to the "native" implementation.

#![cfg_attr(nightly, feature(doc_cfg))]

pub mod gpio;

#[cfg(feature = "hwrng")]
pub mod hwrng;

pub mod identity;
pub mod peripherals {
    use std::sync::LazyLock;
    pub static GPIO_OUT_STREAM: LazyLock<&'static str> = LazyLock::new(|| "GPIO0_OUT");

    use ariel_os_dummy::gpio::output::OutputPin;
    pub struct GPIO0;

    impl OutputPin for GPIO0 {
    }
}

#[allow(non_snake_case)]
pub struct OptionalPeripherals {
    pub GPIO0: Option<crate::peripheral::Peri<'static, peripherals::GPIO0>>,
}

#[must_use]
pub fn init() -> OptionalPeripherals {
    OptionalPeripherals {
        GPIO0: Some(crate::peripheral::Peri::empty()),
    }
}

pub use ariel_os_dummy::{peripheral, IntoPeripheral};

pub struct SWI {}

pub mod tuntap {
    pub type NetworkDevice = embassy_net_tuntap::TunTapDevice;

    /// Creates a TUN/TAP network device as configured in the environment.
    ///
    /// Unlike the hardware device initializers that are inline in `ariel_os_embassy::init_task`,
    /// this doesn't work automatically from init'ing with peripherals and a spawner, but goes
    /// through extra steps of finding which device to actually use.
    ///
    /// # Panics
    ///
    /// This function panics if the network interface can not be opened (eg. for lack of
    /// permission, when it does not exist, or when it is in use).
    #[must_use]
    pub fn create() -> NetworkDevice {
        let ifname = std::env::var("ARIEL_NATIVE_TUNTAP").unwrap_or_else(|_| "tap0".to_owned());
        match NetworkDevice::new(&ifname) {
            Ok(d) => d,
            Err(e) => panic!("Error opening interface {ifname}: {e}"),
        }
    }
}
