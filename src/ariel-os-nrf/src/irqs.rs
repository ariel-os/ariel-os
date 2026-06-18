//! This file defines the interrupt handlers used by different features.
//!
//! It is needed to regroup the interrupts here as some of them are used by multiple features, and these features could be enabled at the same time.

use embassy_nrf::bind_interrupts;

bind_interrupts!(pub(crate) struct Irqs {
    #[cfg(all(feature = "hwrng", not(context = "nrf54l15-app")))]
    RNG => embassy_nrf::rng::InterruptHandler<embassy_nrf::peripherals::RNG>;

    #[cfg(feature = "usb")]
    USBD => embassy_nrf::usb::InterruptHandler<embassy_nrf::peripherals::USBD>;

    #[cfg(all(feature = "usb", context = "nrf5340-app"))]
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
    #[cfg(all(feature = "ble", context = "nrf53"))]
    SWI0 => nrf_sdc::mpsl::LowPrioInterruptHandler;
    // SWI00 is used by the executor on nRF54; use SWI01 for MPSL low-priority handler
    #[cfg(all(feature = "ble", context = "nrf54l15-app"))]
    SWI01 => nrf_sdc::mpsl::LowPrioInterruptHandler;

    #[cfg(all(feature = "ble", not(context = "nrf54l15-app")))]
    RADIO => nrf_sdc::mpsl::HighPrioInterruptHandler;
    #[cfg(all(feature = "ble", context = "nrf54l15-app"))]
    RADIO_0 => nrf_sdc::mpsl::HighPrioInterruptHandler;

    #[cfg(all(feature = "ble", not(context = "nrf54l15-app")))]
    TIMER0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
    #[cfg(all(feature = "ble", context = "nrf54l15-app"))]
    TIMER10 => nrf_sdc::mpsl::HighPrioInterruptHandler;

    #[cfg(all(feature = "ble", not(context = "nrf54l15-app")))]
    RTC0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
    #[cfg(all(feature = "ble", context = "nrf54l15-app"))]
    GRTC_3 => nrf_sdc::mpsl::HighPrioInterruptHandler;
});
