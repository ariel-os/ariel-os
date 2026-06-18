# Universal Serial Bus (USB)

Ariel OS integrates support for USB peripherals built into many microcontrollers.

## Hardware Support

Many microcontrollers supported by Ariel OS include a USB peripheral that can be leveraged to build any USB device.
At the time of writing, most of them support USB 2.0, but their signaling rate is often limited to that of Full-Speed USB (FS), i.e., 12 Mbit/s (now also known as Basic-Speed USB), even though some also do support High-Speed USB, i.e., 480 Mbit/s (marketed as Hi-Speed USB).
Some also support USB On-The-Go (OTG), allowing the device to also behave as a USB Targeted Host on the same USB receptacle (by alternatively switching between the peripheral and host roles).
Currently only the USB peripheral role is supported by Ariel OS.

These USB microcontroller peripherals are "generic," in that they can be used to implement any USB device class.
Some microcontrollers also feature USB microcontroller peripherals that only support a fixed set of USB classes: e.g., multiple ESP32 MCUs comprise a USB CDC-ACM/JTAG peripheral, which can only be used for the standard [USB CDC-ACM][usb-cdc-acm-book-glossary] device class or a vendor-specific device class implementing JTAG access over USB.
Currently, only the "generic" USB peripherals are supported in Ariel OS applications.
The others may still be integrated by Ariel OS to implement specific functionality, like [logging][logging-transports-book].

> [!TIP]
> Development kits that support USB often feature two USB receptacles: one for the onboard [debug probe][debug-probes-book] (if there is one), and the other connected to the USB peripheral of the microcontroller.
> That second USB connection is usually called "user USB" to differentiate it from that of the debug probe and is the one that must be connected to the computer acting as USB host.

## Software Integration

Ariel OS provides support for the USB peripheral role through [`embassy-usb`][embassy-usb-docsrs], which provides a consistent API across the supported hardware.
It can be enabled with the `usb` Cargo feature.

An instance of [`embassy_usb::Builder`][ariel-os-usbbuilder-rustdoc] is created by Ariel OS, on which support for well-known USB device classes can be added using a dedicated [Ariel OS task hook][task-attr-docs]:

```rust
#[ariel_os::task(autostart, usb_builder_hook)]
async fn main() {
    let mut usb_class = USB_BUILDER_HOOK
        .with(|builder| {
            // USB class constructor that mutates the builder.
        })
        .await;
}
```

Support for well-known USB device classes is provided by `embassy-usb`, which is re-exported as [`ariel_os::reexports::embassy_usb`][ariel-os-reexports-embassy-usb-rustdoc].
Custom USB device classes can also be implemented.
Additionally, multiple USB device classes can be added on the builder, to create a composite USB device.

### USB Device Classes

The table below lists some of the well-known USB device classes and how to use them in Ariel OS :

| Device class                             | How to use                                                                                                                                    |
| ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| [USB CDC-ACM][usb-cdc-acm-book-glossary] | Apply the [`CdcAcmClass`][ariel-os-embassy-usb-cdcacmclass-rustdoc] constructor on the builder                                                |
| [USB CDC-NCM][usb-cdc-ncm-glossary-book] | Enable the [`usb-ethernet` laze module][usb-ethernet-laze-module-book]                                                                        |
| USB HID                                  | Enable the `usb-hid` Cargo feature and apply the [`HidReaderWriter`][ariel-os-embassy-usb-hidreaderwriter-rustdoc] constructor on the builder |

Other device classes are supported.

### Device Configuration

[Configuration for the USB device][ariel-os-embassy-usb-config-rustdoc] created can be provided using the [`#[ariel_os::config(usb)]`][config-attr-macro-rustdoc] attribute.
In particular, it allows setting the Vendor ID (VID) and Product ID (PID), and the manufacturer and product names.

Additionally, some environment variables are used by [`embassy-usb`][ariel-os-reexports-embassy-usb-rustdoc] for configuration.
See its documentation for more.

[usb-cdc-acm-book-glossary]: ./glossary.md#usb-cdc-acm
[logging-transports-book]: ./logging.md#logging-transports
[debug-probes-book]: ./flashing-debugging.md#debug-interfaces-protocols-and-probes
[embassy-usb-docsrs]: https://docs.rs/embassy-usb/latest/embassy_usb/
[ariel-os-usbbuilder-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/usb/type.UsbBuilder.html
[task-attr-docs]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/attr.task.html
[ariel-os-reexports-embassy-usb-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/reexports/embassy_usb/index.html
[ariel-os-embassy-usb-cdcacmclass-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/reexports/embassy_usb/class/cdc_acm/struct.CdcAcmClass.html
[ariel-os-embassy-usb-hidreaderwriter-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/reexports/embassy_usb/class/hid/struct.HidReaderWriter.html
[usb-ethernet-laze-module-book]: ./networking.md#network-link-selection
[usb-cdc-ncm-glossary-book]: ./glossary.md#usb-cdc-ncm
[ariel-os-embassy-usb-config-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/reexports/embassy_usb/struct.Config.html
[config-attr-macro-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/attr.config.html
