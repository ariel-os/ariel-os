//! This file defines the interrupt handlers used by different features.
//!
//! It is needed to regroup the interrupts here as some of them are used by multiple features, and these features could be enabled at the same time.

use embassy_nrf::bind_interrupts;

bind_interrupts!(pub(crate) struct Irqs {
    #[cfg(feature = "hwrng")]
    RNG => embassy_nrf::rng::InterruptHandler<embassy_nrf::peripherals::RNG>;

    #[cfg(feature = "usb")]
    USBD => embassy_nrf::usb::InterruptHandler<embassy_nrf::peripherals::USBD>;

    #[cfg(all(feature = "usb", context = "nrf5340"))]
    USBREGULATOR => embassy_nrf::usb::vbus_detect::InterruptHandler;

    CLOCK_POWER =>
    #[cfg(all(feature = "usb", context = "nrf52"))]
    embassy_nrf::usb::vbus_detect::InterruptHandler,
    #[cfg(feature = "ble")]
    nrf_sdc::mpsl::ClockInterruptHandler
    ;

    // SWI0 is used for the executor interrupt
    #[cfg(all(feature = "ble", context = "nrf52"))]
    EGU1_SWI1 => nrf_sdc::mpsl::LowPrioInterruptHandler;
    #[cfg(all(feature = "ble", context = "nrf5340"))]
    EGU1 => nrf_sdc::mpsl::LowPrioInterruptHandler;

    #[cfg(feature = "ble")]
    RADIO => nrf_sdc::mpsl::HighPrioInterruptHandler;

    #[cfg(feature = "ble")]
    TIMER0 => nrf_sdc::mpsl::HighPrioInterruptHandler;

    #[cfg(feature = "ble")]
    RTC0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
});
