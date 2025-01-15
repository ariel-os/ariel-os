#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(used_with_arg)]

mod pins;

use ariel_os::{
    cell::ConstStaticCell,
    debug::log::*,
    reexports::{
        embassy_usb::class::hid,
        usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor},
    },
    time::{Duration, Timer},
    usb::UsbDriver,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};

// Assuming a QWERTY US layout, see https://docs.qmk.fm/#/how_keyboards_work
// and https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf
const KC_A: u8 = 0x04;
const KC_C: u8 = 0x06;
const KC_G: u8 = 0x0a;
const KC_T: u8 = 0x17;

const KEY_RELEASED: u8 = 0x00;
// Maps physical buttons to keycodes/characters
const KEYCODE_MAPPING: [u8; buttons::KEY_COUNT] = [KC_A, KC_C, KC_G, KC_T];

const HID_READER_BUFFER_SIZE: usize = 1;
const HID_WRITER_BUFFER_SIZE: usize = 8;

#[ariel_os::config(usb)]
const USB_CONFIG: ariel_os::reexports::embassy_usb::Config = {
    let mut config = ariel_os::reexports::embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some(ariel_os::buildinfo::OS_NAME);
    config.product = Some("HID keyboard example");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Required for Windows support.
    config.composite_with_iads = true;
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config
};

static HID_WRITER: Mutex<
    CriticalSectionRawMutex,
    Option<hid::HidWriter<'static, UsbDriver, HID_WRITER_BUFFER_SIZE>>,
> = Mutex::new(None);
static HID_STATE: ConstStaticCell<hid::State> = ConstStaticCell::new(hid::State::new());

#[ariel_os::hook(usb_builder)]
fn usb_builder(builder: &mut ariel_os::usb::UsbBuilder) {
    let config = hid::Config {
        report_descriptor: <KeyboardReport as SerializedDescriptor>::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 64,
    };
    let hid_rw = hid::HidReaderWriter::<_, HID_READER_BUFFER_SIZE, HID_WRITER_BUFFER_SIZE>::new(
        builder,
        HID_STATE.take(),
        config,
    );
    let (_hid_reader, hid_writer) = hid_rw.split();
    *HID_WRITER.try_lock().unwrap() = Some(hid_writer);
}

#[ariel_os::task(autostart, peripherals)]
async fn usb_keyboard(button_peripherals: pins::Buttons) {
    let mut buttons = buttons::Buttons::new(button_peripherals);

    let mut hid_writer = HID_WRITER.lock().await.take().unwrap();

    loop {
        for (i, button) in buttons.iter_mut().enumerate() {
            if button.is_low() {
                info!("Button #{} pressed", i + 1);

                let report = get_keyboard_report(KEYCODE_MAPPING[i]);
                if let Err(e) = hid_writer.write_serialize(&report).await {
                    info!("Failed to send report: {:?}", e);
                }
                let report = get_keyboard_report(KEY_RELEASED);
                if let Err(e) = hid_writer.write_serialize(&report).await {
                    info!("Failed to send report: {:?}", e);
                }
            }
        }

        // Debounce events
        Timer::after(Duration::from_millis(50)).await;
    }
}

fn get_keyboard_report(keycode: u8) -> KeyboardReport {
    KeyboardReport {
        keycodes: [keycode, 0, 0, 0, 0, 0],
        leds: 0,
        modifier: 0,
        reserved: 0,
    }
}

mod buttons {
    use ariel_os::gpio::{Input, Pull};

    const PULL: Pull = Pull::Up;

    pub const KEY_COUNT: usize = 4;

    pub struct Buttons([Input; KEY_COUNT]);

    impl Buttons {
        pub fn new(button_peripherals: crate::pins::Buttons) -> Self {
            Self([
                Input::new(button_peripherals.btn1, PULL),
                Input::new(button_peripherals.btn2, PULL),
                Input::new(button_peripherals.btn3, PULL),
                Input::new(button_peripherals.btn4, PULL),
            ])
        }

        pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Input> {
            self.0.iter_mut()
        }
    }
}
