use ariel_os::hal::peripherals;

#[cfg(context = "nordic-thingy-91-x-nrf9151")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_29 });

#[cfg(context = "nrf9160dk-nrf9160")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_02 });
