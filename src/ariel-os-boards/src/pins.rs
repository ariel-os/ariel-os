use ariel_os_hal::hal::peripherals;

#[cfg(context = "bbc-microbit-v1")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P0_13,
});
#[cfg(context = "bbc-microbit-v1")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P0_17,
button1: P0_26,
});
#[cfg(context = "bbc-microbit-v2")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P0_21,
});
#[cfg(context = "bbc-microbit-v2")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P0_14,
button1: P0_23,
});
#[cfg(context = "dfrobot-firebeetle2-esp32-c6")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: GPIO15,
});
#[cfg(context = "dfrobot-firebeetle2-esp32-c6")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: GPIO9,
});
#[cfg(context = "dwm1001")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P0_14,
led1: P0_30,
led2: P0_22,
led3: P0_31,
});
#[cfg(context = "dwm1001")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P0_02,
});
#[cfg(context = "heltec-wifi-lora-32-v3")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: GPIO35,
});
#[cfg(context = "heltec-wifi-lora-32-v3")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: GPIO0,
});
#[cfg(context = "nordic-thingy-91-x-nrf9151")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P0_29,
});
#[cfg(context = "nordic-thingy-91-x-nrf9151")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P0_26,
});
#[cfg(context = "nrf52840-mdk")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P0_23,
});
#[cfg(context = "nrf52840-mdk")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P1_00,
});
#[cfg(context = "nrf52840dk")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P0_13,
led1: P0_14,
led2: P0_15,
led3: P0_16,
});
#[cfg(context = "nrf52840dk")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P0_11,
button1: P0_12,
button2: P0_24,
button3: P0_25,
});
#[cfg(context = "nrf52dk")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P0_17,
led1: P0_18,
led2: P0_19,
led3: P0_20,
});
#[cfg(context = "nrf52dk")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P0_13,
button1: P0_14,
button2: P0_15,
button3: P0_16,
});
#[cfg(context = "nrf5340dk")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P0_28,
led1: P0_29,
led2: P0_30,
led3: P0_31,
});
#[cfg(context = "nrf5340dk")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P0_23,
button1: P0_24,
button2: P0_08,
button3: P0_09,
});
#[cfg(context = "nrf9160dk-nrf9160")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P0_02,
});
#[cfg(context = "nrf9160dk-nrf9160")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P0_06,
});
#[cfg(context = "particle-xenon")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P1_12,
});
#[cfg(context = "particle-xenon")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P0_11,
});
#[cfg(context = "rpi-pico")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PIN_25,
});
#[cfg(context = "st-b-l475e-iot01a")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PA5,
led1: PB14,
});
#[cfg(context = "st-b-l475e-iot01a")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC13,
});
#[cfg(context = "st-nucleo-c031c6")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PA5,
});
#[cfg(context = "st-nucleo-c031c6")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC13,
});
#[cfg(context = "st-nucleo-f042k6")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PA5,
});
#[cfg(context = "st-nucleo-f401re")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PB5,
});
#[cfg(context = "st-nucleo-f401re")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC13,
});
#[cfg(context = "st-nucleo-f411re")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PB5,
});
#[cfg(context = "st-nucleo-f411re")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC13,
});
#[cfg(context = "st-nucleo-f767zi")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PB0,
led1: PB7,
led2: PB14,
});
#[cfg(context = "st-nucleo-f767zi")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC13,
});
#[cfg(context = "st-nucleo-h755zi-q")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PB0,
led1: PE1,
led2: PE1,
});
#[cfg(context = "st-nucleo-h755zi-q")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC13,
});
#[cfg(context = "st-nucleo-wb55")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PB5,
led1: PB0,
led2: PB1,
});
#[cfg(context = "st-nucleo-wb55")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC4,
button1: PD0,
button2: PD1,
});
#[cfg(context = "st-nucleo-wba55")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PB4,
led1: PA9,
led2: PB8,
});
#[cfg(context = "st-nucleo-wba55")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC13,
button1: PB6,
button2: PB7,
});
#[cfg(context = "st-steval-mkboxpro")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PF6,
led1: PH11,
led2: PH12,
led3: PF9,
});
#[cfg(context = "st-steval-mkboxpro")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC13,
button1: PE0,
});
#[cfg(context = "stm32u083c-dk")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PA5,
});
#[cfg(context = "stm32u083c-dk")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC2,
});
