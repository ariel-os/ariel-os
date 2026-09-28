// @generated

#![no_std]
#[rustfmt::skip]
cfg_select! {
    context = "adafruit-feather-nrf52840-express" => {
    include!("adafruit-feather-nrf52840-express.rs"); } context =
    "adafruit-feather-nrf52840-sense" => {
    include!("adafruit-feather-nrf52840-sense.rs"); } context = "arduino-uno-q" => {
    include!("arduino-uno-q.rs"); } context = "bbc-microbit-v1" => {
    include!("bbc-microbit-v1.rs"); } context = "bbc-microbit-v2" => {
    include!("bbc-microbit-v2.rs"); } context = "decawave-dwm1001-dev" => {
    include!("decawave-dwm1001-dev.rs"); } context = "dfrobot-firebeetle2-esp32-c6" => {
    include!("dfrobot-firebeetle2-esp32-c6.rs"); } context = "dwm3001cdk" => {
    include!("dwm3001cdk.rs"); } context = "espressif-esp32-c3-devkit-rust-1" => {
    include!("espressif-esp32-c3-devkit-rust-1.rs"); } context =
    "espressif-esp32-c3-lcdkit" => { include!("espressif-esp32-c3-lcdkit.rs"); } context
    = "espressif-esp32-c6-devkitc-1" => { include!("espressif-esp32-c6-devkitc-1.rs"); }
    context = "espressif-esp32-devkitc" => { include!("espressif-esp32-devkitc.rs"); }
    context = "espressif-esp32-h2-devkitm-1" => {
    include!("espressif-esp32-h2-devkitm-1.rs"); } context =
    "espressif-esp32-s2-devkitc-1" => { include!("espressif-esp32-s2-devkitc-1.rs"); }
    context = "espressif-esp32-s3-devkitc-1" => {
    include!("espressif-esp32-s3-devkitc-1.rs"); } context =
    "espressif-esp8684-devkitc-02" => { include!("espressif-esp8684-devkitc-02.rs"); }
    context = "heltec-wifi-lora-32-v3" => { include!("heltec-wifi-lora-32-v3.rs"); }
    context = "makerdiary-nrf52840-mdk-usb-dongle" => {
    include!("makerdiary-nrf52840-mdk-usb-dongle.rs"); } context =
    "makerdiary-nrf52840-mdk" => { include!("makerdiary-nrf52840-mdk.rs"); } context =
    "makerdiary-nrf9151-connect-kit" => { include!("makerdiary-nrf9151-connect-kit.rs");
    } context = "native" => { include!("native.rs"); } context =
    "nordic-thingy-91-x-nrf9151" => { include!("nordic-thingy-91-x-nrf9151.rs"); }
    context = "nordic-thingy-91-x-nrf5340-app" => {
    include!("nordic-thingy-91-x-nrf5340-app.rs"); } context =
    "nordic-thingy-91-x-nrf5340-net" => { include!("nordic-thingy-91-x-nrf5340-net.rs");
    } context = "nrf52840dk" => { include!("nrf52840dk.rs"); } context = "nrf52dk" => {
    include!("nrf52dk.rs"); } context = "nrf5340dk-app" => {
    include!("nrf5340dk-app.rs"); } context = "nrf5340dk-net" => {
    include!("nrf5340dk-net.rs"); } context = "nrf54l15dk-app" => {
    include!("nrf54l15dk-app.rs"); } context = "nrf9151-dk" => {
    include!("nrf9151-dk.rs"); } context = "nrf9160dk-nrf9160" => {
    include!("nrf9160dk-nrf9160.rs"); } context = "particle-xenon" => {
    include!("particle-xenon.rs"); } context = "rpi-pico-w" => {
    include!("rpi-pico-w.rs"); } context = "rpi-pico" => { include!("rpi-pico.rs"); }
    context = "rpi-pico2-w" => { include!("rpi-pico2-w.rs"); } context = "rpi-pico2" => {
    include!("rpi-pico2.rs"); } context = "seeedstudio-lora-e5-mini" => {
    include!("seeedstudio-lora-e5-mini.rs"); } context = "seeedstudio-xiao-esp32-s3" => {
    include!("seeedstudio-xiao-esp32-s3.rs"); } context = "seeedstudio-xiao-esp32c6" => {
    include!("seeedstudio-xiao-esp32c6.rs"); } context = "seeedstudio-xiao-nrf52840-plus"
    => { include!("seeedstudio-xiao-nrf52840-plus.rs"); } context = "st-b-l072z-lrwan1"
    => { include!("st-b-l072z-lrwan1.rs"); } context = "st-b-l475e-iot01a" => {
    include!("st-b-l475e-iot01a.rs"); } context = "st-nucleo-c031c6" => {
    include!("st-nucleo-c031c6.rs"); } context = "st-nucleo-f042k6" => {
    include!("st-nucleo-f042k6.rs"); } context = "st-nucleo-f303re" => {
    include!("st-nucleo-f303re.rs"); } context = "st-nucleo-f401re" => {
    include!("st-nucleo-f401re.rs"); } context = "st-nucleo-f411re" => {
    include!("st-nucleo-f411re.rs"); } context = "st-nucleo-f767zi" => {
    include!("st-nucleo-f767zi.rs"); } context = "st-nucleo-g431rb" => {
    include!("st-nucleo-g431rb.rs"); } context = "st-nucleo-h753zi" => {
    include!("st-nucleo-h753zi.rs"); } context = "st-nucleo-h755zi-q" => {
    include!("st-nucleo-h755zi-q.rs"); } context = "st-nucleo-wb55" => {
    include!("st-nucleo-wb55.rs"); } context = "st-nucleo-wba55" => {
    include!("st-nucleo-wba55.rs"); } context = "st-nucleo-wba65ri" => {
    include!("st-nucleo-wba65ri.rs"); } context = "st-steval-mkboxpro" => {
    include!("st-steval-mkboxpro.rs"); } context = "stm32u083c-dk" => {
    include!("stm32u083c-dk.rs"); } context = "ulanzi-tc001" => {
    include!("ulanzi-tc001.rs"); } context = "unihiker-k10" => {
    include!("unihiker-k10.rs"); } context = "waveshare-esp32-s3-eth" => {
    include!("waveshare-esp32-s3-eth.rs"); } context = "waveshare-esp32-s3-matrix" => {
    include!("waveshare-esp32-s3-matrix.rs"); } _ => {}
}
