use ariel_os_debug::log::debug;
use embassy_nrf::{
    pac, peripherals,
    usb::{Driver, vbus_detect::HardwareVbusDetect},
};

use crate::irqs::Irqs;

pub type UsbDriver = Driver<'static, peripherals::USBD, HardwareVbusDetect>;

pub struct Peripherals {
    usbd: peripherals::USBD,
}

impl Peripherals {
    #[must_use]
    pub fn new(peripherals: &mut crate::OptionalPeripherals) -> Self {
        Self {
            usbd: peripherals.USBD.take().unwrap(),
        }
    }
}

pub fn init() {
    debug!("nrf: enabling ext hfosc...");
    pac::CLOCK.tasks_hfclkstart().write_value(1);
    while pac::CLOCK.events_hfclkstarted().read() != 1 {}
}

pub fn driver(peripherals: Peripherals) -> UsbDriver {
    Driver::new(peripherals.usbd, Irqs, HardwareVbusDetect::new(Irqs))
}
