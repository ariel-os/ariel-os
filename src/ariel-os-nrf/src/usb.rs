use ariel_os_debug::log::debug;
use embassy_nrf::{
    Peri, pac, peripherals,
    usb::{Driver, vbus_detect::HardwareVbusDetect},
};

use crate::irqs::Irqs;

// TODO: as per docs, this does not work in combination with the softdevice
pub type UsbDriver = Driver<'static, HardwareVbusDetect>;

pub struct Peripherals {
    usbd: Peri<'static, peripherals::USBD>,
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
    UsbDriver::new(peripherals.usbd, Irqs, HardwareVbusDetect::new(Irqs))
}
