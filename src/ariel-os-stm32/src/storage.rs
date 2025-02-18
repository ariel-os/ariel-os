// TODO: we may later want to introduce a Cargo feature for async flash drivers, possibly enabled
// by laze.

use embassy_stm32::flash;

#[cfg(context = "stm32f401retx")]
embassy_stm32::bind_interrupts!(struct Irqs {
    FLASH => flash::InterruptHandler;
});

#[cfg(context = "stm32f401retx")]
pub type Flash = flash::Flash<'static>;
#[cfg(any(
    context = "stm32c031c6tx",
    context = "stm32h755zitx",
    context = "stm32wb55rgvx",
))]
pub type Flash =
    embassy_embedded_hal::adapter::BlockingAsync<flash::Flash<'static, flash::Blocking>>;
pub type FlashError = flash::Error;

pub fn init(peripherals: &mut crate::OptionalPeripherals) -> Flash {
    #[cfg(context = "stm32f401retx")]
    let flash = flash::Flash::new(peripherals.FLASH.take().unwrap(), Irqs);
    #[cfg(any(
        context = "stm32c031c6tx",
        context = "stm32h755zitx",
        context = "stm32wb55rgvx",
    ))]
    let flash = embassy_embedded_hal::adapter::BlockingAsync::new(flash::Flash::new_blocking(
        peripherals.FLASH.take().unwrap(),
    ));
    flash
}
