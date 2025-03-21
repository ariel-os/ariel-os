use embassy_stm32::{bind_interrupts, peripherals, usb, usb::Driver};

bind_interrupts!(struct Irqs {
    #[cfg(capability = "hw/stm32-usb")]
    USB => usb::InterruptHandler<peripherals::USB>;
    #[cfg(capability = "hw/stm32-usb-drd-fs")]
    USB_DRD_FS => usb::InterruptHandler<peripherals::USB>;
    #[cfg(capability = "hw/stm32-usb-fs")]
    USB_FS => usb::InterruptHandler<peripherals::USB>;
    #[cfg(capability = "hw/stm32-usb-lp")]
    USB_LP => usb::InterruptHandler<peripherals::USB>;
    #[cfg(capability = "hw/stm32-usb-lp-can1-rx0")]
    USB_LP_CAN1_RX0 => usb::InterruptHandler<peripherals::USB>;
    #[cfg(capability = "hw/stm32-usb-lp-can-rx0")]
    USB_LP_CAN_RX0 => usb::InterruptHandler<peripherals::USB>;
    #[cfg(capability = "hw/stm32-usb-ucpd1-2")]
    USB_UCPD1_2 => usb::InterruptHandler<peripherals::USB>;
});

pub type UsbDriver = Driver<'static, peripherals::USB>;

pub struct Peripherals {
    usb: peripherals::USB,
    dp: peripherals::PA12,
    dm: peripherals::PA11,
}

impl Peripherals {
    #[must_use]
    pub fn new(peripherals: &mut crate::OptionalPeripherals) -> Self {
        Self {
            usb: peripherals.USB.take().unwrap(),
            dp: peripherals.PA12.take().unwrap(),
            dm: peripherals.PA11.take().unwrap(),
        }
    }
}

pub fn driver(peripherals: Peripherals) -> UsbDriver {
    Driver::new(peripherals.usb, Irqs, peripherals.dp, peripherals.dm)
}
