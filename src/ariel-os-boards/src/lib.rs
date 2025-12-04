// @generated

#![no_std]
cfg_if::cfg_if! {
    if #[cfg(context = "ai-c3")] { include!("ai-c3.rs"); } else if #[cfg(context =
    "bbc-microbit-v1")] { include!("bbc-microbit-v1.rs"); } else if #[cfg(context =
    "bbc-microbit-v2")] { include!("bbc-microbit-v2.rs"); } else if #[cfg(context =
    "dfrobot-firebeetle2-esp32-c6")] { include!("dfrobot-firebeetle2-esp32-c6.rs"); }
    else if #[cfg(context = "dwm1001")] { include!("dwm1001.rs"); } else if #[cfg(context
    = "espressif-esp32-c3-devkit-rust-1")] {
    include!("espressif-esp32-c3-devkit-rust-1.rs"); } else if #[cfg(context =
    "espressif-esp32-c3-lcdkit")] { include!("espressif-esp32-c3-lcdkit.rs"); } else if
    #[cfg(context = "espressif-esp32-c6-devkitc-1")] {
    include!("espressif-esp32-c6-devkitc-1.rs"); } else if #[cfg(context =
    "espressif-esp32-devkitc")] { include!("espressif-esp32-devkitc.rs"); } else if
    #[cfg(context = "espressif-esp32-s2-devkitc-1")] {
    include!("espressif-esp32-s2-devkitc-1.rs"); } else if #[cfg(context =
    "espressif-esp32-s3-devkitc-1")] { include!("espressif-esp32-s3-devkitc-1.rs"); }
    else if #[cfg(context = "heltec-wifi-lora-32-v3")] {
    include!("heltec-wifi-lora-32-v3.rs"); } else if #[cfg(context = "native")] {
    include!("native.rs"); } else if #[cfg(context = "nordic-thingy-91-x-nrf9151")] {
    include!("nordic-thingy-91-x-nrf9151.rs"); } else if #[cfg(context = "nrf52840-mdk")]
    { include!("nrf52840-mdk.rs"); } else if #[cfg(context = "nrf52840dk")] {
    include!("nrf52840dk.rs"); } else if #[cfg(context = "nrf52dk")] {
    include!("nrf52dk.rs"); } else if #[cfg(context = "nrf5340dk-net")] {
    include!("nrf5340dk-net.rs"); } else if #[cfg(context = "nrf5340dk")] {
    include!("nrf5340dk.rs"); } else if #[cfg(context = "nrf9151-dk")] {
    include!("nrf9151-dk.rs"); } else if #[cfg(context = "nrf9160dk-nrf9160")] {
    include!("nrf9160dk-nrf9160.rs"); } else if #[cfg(context = "particle-xenon")] {
    include!("particle-xenon.rs"); } else if #[cfg(context = "rpi-pico-w")] {
    include!("rpi-pico-w.rs"); } else if #[cfg(context = "rpi-pico")] {
    include!("rpi-pico.rs"); } else if #[cfg(context = "rpi-pico2-w")] {
    include!("rpi-pico2-w.rs"); } else if #[cfg(context = "rpi-pico2")] {
    include!("rpi-pico2.rs"); } else if #[cfg(context = "seeedstudio-lora-e5-mini")] {
    include!("seeedstudio-lora-e5-mini.rs"); } else if #[cfg(context =
    "seeedstudio-xiao-esp32c6")] { include!("seeedstudio-xiao-esp32c6.rs"); } else if
    #[cfg(context = "st-b-l475e-iot01a")] { include!("st-b-l475e-iot01a.rs"); } else if
    #[cfg(context = "st-nucleo-c031c6")] { include!("st-nucleo-c031c6.rs"); } else if
    #[cfg(context = "st-nucleo-f042k6")] { include!("st-nucleo-f042k6.rs"); } else if
    #[cfg(context = "st-nucleo-f401re")] { include!("st-nucleo-f401re.rs"); } else if
    #[cfg(context = "st-nucleo-f411re")] { include!("st-nucleo-f411re.rs"); } else if
    #[cfg(context = "st-nucleo-f767zi")] { include!("st-nucleo-f767zi.rs"); } else if
    #[cfg(context = "st-nucleo-h753zi")] { include!("st-nucleo-h753zi.rs"); } else if
    #[cfg(context = "st-nucleo-h755zi-q")] { include!("st-nucleo-h755zi-q.rs"); } else if
    #[cfg(context = "st-nucleo-wb55")] { include!("st-nucleo-wb55.rs"); } else if
    #[cfg(context = "st-nucleo-wba55")] { include!("st-nucleo-wba55.rs"); } else if
    #[cfg(context = "st-steval-mkboxpro")] { include!("st-steval-mkboxpro.rs"); } else if
    #[cfg(context = "stm32u083c-dk")] { include!("stm32u083c-dk.rs"); } else {}
}
