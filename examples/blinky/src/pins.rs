use ariel_os::hal::peripherals;

#[cfg(context = "bbc-microbit-v1")]
ariel_os::hal::define_peripherals!(LedPeripherals {
    led_col1: P0_04,
    led: P0_13,
});

#[cfg(context = "bbc-microbit-v2")]
ariel_os::hal::define_peripherals!(LedPeripherals {
    led_col1: P0_28,
    led: P0_21,
});

#[cfg(context = "dfrobot-firebeetle2-esp32-c6")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: GPIO15 });

#[cfg(context = "heltec-wifi-lora-32-v3")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: GPIO35 });

#[cfg(context = "nordic-thingy-91-x-nrf9151")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_29 });

#[cfg(context = "nrf52840dk")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_13 });

#[cfg(context = "nrf5340dk")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_28 });

#[cfg(context = "nrf9160dk-nrf9160")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_02 });

#[cfg(context = "nrf52dk")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_17 });

#[cfg(context = "particle-xenon")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P1_12 });

#[cfg(any(context = "rpi-pico", context = "rpi-pico2"))]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PIN_25 });

#[cfg(all(context = "rp", not(any(context = "rpi-pico", context = "rpi-pico2"))))]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PIN_1 });

#[cfg(all(
    context = "esp",
    not(any(
        context = "dfrobot-firebeetle2-esp32-c6",
        context = "heltec-wifi-lora-32-v3",
    ))
))]
ariel_os::hal::define_peripherals!(LedPeripherals { led: GPIO0 });

#[cfg(context = "st-b-l475e-iot01a")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PA5 });

#[cfg(context = "st-nucleo-c031c6")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PA5 });

#[cfg(context = "st-nucleo-f042k6")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PB3 });

#[cfg(any(context = "st-nucleo-f401re", context = "st-nucleo-f411re"))]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PA5 });

#[cfg(context = "st-nucleo-h755zi-q")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PB0 });

#[cfg(context = "st-nucleo-f767zi")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PB0 });

#[cfg(context = "st-nucleo-wb55")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PB5 });

#[cfg(context = "st-nucleo-wba55")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PB4 });

#[cfg(context = "st-steval-mkboxpro")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PF6 });

#[cfg(context = "stm32u083c-dk")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PA5 });

#[cfg(context = "seeedstudio-lora-e5-mini")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PB5 });
