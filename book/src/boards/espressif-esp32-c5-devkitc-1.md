# Espressif ESP32-C5-DevKitC-1

## References

- [Manufacturer link](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c5/esp32-c5-devkitc-1/user_guide.html)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `espressif-esp32-c5-devkitc-1-n4`

- **Tier:** 2
- **Chip:** [ESP32-C5HF4](../chips/esp32c5hf4.md)
- **Chip Ariel OS Name:** `esp32c5hf4`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b espressif-esp32-c5-devkitc-1-n4
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="not available on this piece of hardware">–</span>[^no-generic-usb-peripheral]|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^requires-partitioning-support]|

#### Additional Notes

##### USB Ports

This board features two USB ports: one is labeled "USB" while the other is labeled "UART".

The "USB" port is connected to the USB CDC-ACM/JTAG USB peripheral of the MCU and can be used to flash the board either through the bootloader over USB CDC-ACM using `espflash`, or over JTAG using probe-rs.
It can also be used to obtain logs when the [`logging-over-usb`](../logging.md#logging-transports) laze module is enabled.

The "UART" port is connected to a USB ⟷ UART adapter, wired to UART0.
It can be used to flash the board through the bootloader using `espflash`.
It can also be used to obtain logs when the [`logging-over-uart`](../logging.md#logging-transports) laze module is enabled.

Note: On the ESP32-C3, enabling `logging-over-uart` will [not prevent the logs from also being printed using the USB CDC-ACM USB peripheral](https://github.com/esp-rs/esp-hal/issues/4510).

### `espressif-esp32-c5-devkitc-1-n4rx`

- **Tier:** 2
- **Chip:** [ESP32-C5HRx](../chips/esp32c5hrx.md)
- **Chip Ariel OS Name:** `esp32c5hrx`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b espressif-esp32-c5-devkitc-1-n4rx
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="not available on this piece of hardware">–</span>[^no-generic-usb-peripheral]|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^requires-partitioning-support]|

#### Additional Notes

##### USB Ports

This board features two USB ports: one is labeled "USB" while the other is labeled "UART".

The "USB" port is connected to the USB CDC-ACM/JTAG USB peripheral of the MCU and can be used to flash the board either through the bootloader over USB CDC-ACM using `espflash`, or over JTAG using probe-rs.
It can also be used to obtain logs when the [`logging-over-usb`](../logging.md#logging-transports) laze module is enabled.

The "UART" port is connected to a USB ⟷ UART adapter, wired to UART0.
It can be used to flash the board through the bootloader using `espflash`.
It can also be used to obtain logs when the [`logging-over-uart`](../logging.md#logging-transports) laze module is enabled.

Note: On the ESP32-C3, enabling `logging-over-uart` will [not prevent the logs from also being printed using the USB CDC-ACM USB peripheral](https://github.com/esp-rs/esp-hal/issues/4510).

### `espressif-esp32-c5-devkitc-1-n8rx`

- **Tier:** 2
- **Chip:** [ESP32-C5HRx](../chips/esp32c5hrx.md)
- **Chip Ariel OS Name:** `esp32c5hrx`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b espressif-esp32-c5-devkitc-1-n8rx
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="not available on this piece of hardware">–</span>[^no-generic-usb-peripheral]|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^requires-partitioning-support]|

#### Additional Notes

##### USB Ports

This board features two USB ports: one is labeled "USB" while the other is labeled "UART".

The "USB" port is connected to the USB CDC-ACM/JTAG USB peripheral of the MCU and can be used to flash the board either through the bootloader over USB CDC-ACM using `espflash`, or over JTAG using probe-rs.
It can also be used to obtain logs when the [`logging-over-usb`](../logging.md#logging-transports) laze module is enabled.

The "UART" port is connected to a USB ⟷ UART adapter, wired to UART0.
It can be used to flash the board through the bootloader using `espflash`.
It can also be used to obtain logs when the [`logging-over-uart`](../logging.md#logging-transports) laze module is enabled.

Note: On the ESP32-C3, enabling `logging-over-uart` will [not prevent the logs from also being printed using the USB CDC-ACM USB peripheral](https://github.com/esp-rs/esp-hal/issues/4510).

### `espressif-esp32-c5-devkitc-1-n16rx`

- **Tier:** 2
- **Chip:** [ESP32-C5HRx](../chips/esp32c5hrx.md)
- **Chip Ariel OS Name:** `esp32c5hrx`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b espressif-esp32-c5-devkitc-1-n16rx
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="not available on this piece of hardware">–</span>[^no-generic-usb-peripheral]|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^requires-partitioning-support]|

#### Additional Notes

##### USB Ports

This board features two USB ports: one is labeled "USB" while the other is labeled "UART".

The "USB" port is connected to the USB CDC-ACM/JTAG USB peripheral of the MCU and can be used to flash the board either through the bootloader over USB CDC-ACM using `espflash`, or over JTAG using probe-rs.
It can also be used to obtain logs when the [`logging-over-usb`](../logging.md#logging-transports) laze module is enabled.

The "UART" port is connected to a USB ⟷ UART adapter, wired to UART0.
It can be used to flash the board through the bootloader using `espflash`.
It can also be used to obtain logs when the [`logging-over-uart`](../logging.md#logging-transports) laze module is enabled.

Note: On the ESP32-C3, enabling `logging-over-uart` will [not prevent the logs from also being printed using the USB CDC-ACM USB peripheral](https://github.com/esp-rs/esp-hal/issues/4510).

### `espressif-esp32-c5-devkitc-1-n32rx`

- **Tier:** 2
- **Chip:** [ESP32-C5HRx](../chips/esp32c5hrx.md)
- **Chip Ariel OS Name:** `esp32c5hrx`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b espressif-esp32-c5-devkitc-1-n32rx
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="not available on this piece of hardware">–</span>[^no-generic-usb-peripheral]|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^requires-partitioning-support]|

#### Additional Notes

##### USB Ports

This board features two USB ports: one is labeled "USB" while the other is labeled "UART".

The "USB" port is connected to the USB CDC-ACM/JTAG USB peripheral of the MCU and can be used to flash the board either through the bootloader over USB CDC-ACM using `espflash`, or over JTAG using probe-rs.
It can also be used to obtain logs when the [`logging-over-usb`](../logging.md#logging-transports) laze module is enabled.

The "UART" port is connected to a USB ⟷ UART adapter, wired to UART0.
It can be used to flash the board through the bootloader using `espflash`.
It can also be used to obtain logs when the [`logging-over-uart`](../logging.md#logging-transports) laze module is enabled.

Note: On the ESP32-C3, enabling `logging-over-uart` will [not prevent the logs from also being printed using the USB CDC-ACM USB peripheral](https://github.com/esp-rs/esp-hal/issues/4510).

<p>Legend:</p>

<dl>
  <div>
    <dt>✅</dt><dd>supported</dd>
  </div>
  <div>
    <dt>☑️</dt><dd>supported with some caveats</dd>
  </div>
  <div>
    <dt>🚦</dt><dd>needs testing</dd>
  </div>
  <div>
    <dt>❌</dt><dd>available in hardware, but not currently supported by Ariel OS</dd>
  </div>
  <div>
    <dt>–</dt><dd>not available on this piece of hardware</dd>
  </div>
</dl>
<style>
dt, dd {
  display: inline;
}
</style>


  
[^no-generic-usb-peripheral]: No generic USB peripheral.
[^requires-partitioning-support]: Requires partitioning support.
  
[^no-generic-usb-peripheral]: No generic USB peripheral.
[^requires-partitioning-support]: Requires partitioning support.
  
[^no-generic-usb-peripheral]: No generic USB peripheral.
[^requires-partitioning-support]: Requires partitioning support.
  
[^no-generic-usb-peripheral]: No generic USB peripheral.
[^requires-partitioning-support]: Requires partitioning support.
  
[^no-generic-usb-peripheral]: No generic USB peripheral.
[^requires-partitioning-support]: Requires partitioning support.