use embassy_stm32::{Peri, bind_interrupts, peripherals, usb, usb::Driver};

bind_interrupts!(struct Irqs {
    #[cfg(capability = "hw/stm32-usb-synopsis")]
    OTG_FS => usb::InterruptHandler<peripherals::USB_OTG_FS>;
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
    #[cfg(capability = "hw/stm32-usb-synopsis-hs")]
    USB_OTG_HS => usb::InterruptHandler<peripherals::USB_OTG_HS>;
    #[cfg(capability = "hw/stm32-usb-ucpd1-2")]
    USB_UCPD1_2 => usb::InterruptHandler<peripherals::USB>;
});

#[cfg(not(any(
    capability = "hw/stm32-usb-synopsis",
    capability = "hw/stm32-usb-synopsis-hs"
)))]
type UsbPeripheral = peripherals::USB;
#[cfg(capability = "hw/stm32-usb-synopsis")]
type UsbPeripheral = peripherals::USB_OTG_FS;
#[cfg(capability = "hw/stm32-usb-synopsis-hs")]
type UsbPeripheral = peripherals::USB_OTG_HS;

pub type UsbDriver = Driver<'static, UsbPeripheral>;

pub struct Peripherals {
    usb: Peri<'static, UsbPeripheral>,
    #[cfg(not(capability = "hw/stm32-usb-synopsis-hs"))]
    dp: Peri<'static, peripherals::PA12>,
    #[cfg(capability = "hw/stm32-usb-synopsis-hs")]
    dp: Peri<'static, peripherals::PD6>,
    #[cfg(not(capability = "hw/stm32-usb-synopsis-hs"))]
    dm: Peri<'static, peripherals::PA11>,
    #[cfg(capability = "hw/stm32-usb-synopsis-hs")]
    dm: Peri<'static, peripherals::PD7>,
}

impl Peripherals {
    #[must_use]
    pub fn new(peripherals: &mut crate::OptionalPeripherals) -> Self {
        Self {
            #[cfg(not(any(
                capability = "hw/stm32-usb-synopsis",
                capability = "hw/stm32-usb-synopsis-hs"
            )))]
            usb: peripherals.USB.take().unwrap(),
            #[cfg(capability = "hw/stm32-usb-synopsis")]
            usb: peripherals.USB_OTG_FS.take().unwrap(),
            #[cfg(capability = "hw/stm32-usb-synopsis-hs")]
            usb: peripherals.USB_OTG_HS.take().unwrap(),
            #[cfg(not(capability = "hw/stm32-usb-synopsis-hs"))]
            dp: peripherals.PA12.take().unwrap(),
            #[cfg(capability = "hw/stm32-usb-synopsis-hs")]
            dp: peripherals.PD6.take().unwrap(),
            #[cfg(not(capability = "hw/stm32-usb-synopsis-hs"))]
            dm: peripherals.PA11.take().unwrap(),
            #[cfg(capability = "hw/stm32-usb-synopsis-hs")]
            dm: peripherals.PD7.take().unwrap(),
        }
    }
}

#[cfg(not(any(
    capability = "hw/stm32-usb-synopsis",
    capability = "hw/stm32-usb-synopsis-hs"
)))]
pub fn driver(peripherals: Peripherals) -> UsbDriver {
    Driver::new(peripherals.usb, Irqs, peripherals.dp, peripherals.dm)
}

#[cfg(any(
    capability = "hw/stm32-usb-synopsis",
    capability = "hw/stm32-usb-synopsis-hs"
))]
pub fn driver(peripherals: Peripherals) -> UsbDriver {
    use static_cell::ConstStaticCell;

    // buffer size copied from upstream. There's this hint about its sizing:
    // "An internal buffer used to temporarily store received packets.
    // Must be large enough to fit all OUT endpoint max packet sizes.
    // Endpoint allocation will fail if it is too small."
    static EP_OUT_BUFFER: ConstStaticCell<[u8; 256]> = ConstStaticCell::new([0u8; 256]);
    let ep_out_buffer = EP_OUT_BUFFER.take();
    let mut config = embassy_stm32::usb::Config::default();

    // Enable vbus_detection
    // Note: some boards don't have this wired up and might not require it,
    // as they are powered through usb!
    // If you hang on boot, try setting this to "false"!
    // See https://embassy.dev/book/dev/faq.html#_the_usb_examples_are_not_working_on_my_board_is_there_anything_else_i_need_to_configure
    // for more information
    // NOTE(board-config)
    config.vbus_detection =
        ariel_os_utils::bool_from_env_or!("CONFIG_VBUS_DETECTION", true, "Enable vbus_detection");

    #[cfg(feature = "executor-interrupt")]
    {
        use embassy_stm32::interrupt::{InterruptExt, Priority};
        crate::SWI.set_priority(Priority::P1);

        #[cfg(capability = "hw/stm32-usb-synopsis")]
        embassy_stm32::interrupt::OTG_FS.set_priority(Priority::P0);
        #[cfg(capability = "hw/stm32-usb-synopsis-hs")]
        embassy_stm32::interrupt::USB_OTG_HS.set_priority(Priority::P0);
    }

    Driver::new_fs(
        peripherals.usb,
        Irqs,
        peripherals.dp,
        peripherals.dm,
        ep_out_buffer,
        config,
    )
}
