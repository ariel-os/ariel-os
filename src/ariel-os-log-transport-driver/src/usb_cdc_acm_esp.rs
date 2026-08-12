use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use esp_hal::{Blocking, usb_serial_jtag::UsbSerialJtag};

use ariel_os_hal::hal::OptionalPeripherals;

static WRITER: Mutex<CriticalSectionRawMutex, Option<UsbSerialJtag<'_, Blocking>>> =
    Mutex::new(None);

pub(crate) fn write_bytes(bytes: &[u8]) {
    if let Ok(mut opt) = WRITER.try_lock()
        && let Some(writer) = opt.as_mut()
    {
        writer.write(bytes);
    }
}
pub(crate) fn flush() {
    if let Ok(mut opt) = WRITER.try_lock()
        && let Some(writer) = opt.as_mut()
    {
        writer.flush_tx();
    }
}

/// Initialize the USB CDC ACM transport using the special jtag/serial ESP peripheral.
pub fn init(peripherals: &mut OptionalPeripherals, _spawner: Spawner) {
    let w = UsbSerialJtag::new(peripherals.USB_DEVICE.take().unwrap());

    let mut writer = WRITER.try_lock().unwrap();
    writer.replace(w);

    ariel_os_log::custom_transport::register_transport_functions(write_bytes, flush);
}
