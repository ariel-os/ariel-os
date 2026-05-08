# Espressif ESP32-C6-DevKitC-1

## References

- [Manufacturer link](https://web.archive.org/web/20250122153727/https://www.espressif.com/en/dev-board/esp32-c6-devkitc-1-en)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `espressif-esp32-c6-devkitc-1`

- **Tier:** 1
- **Chip:** [ESP32-C6](../chips/esp32c6.md)
- **Chip Ariel OS Name:** `esp32c6`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b espressif-esp32-c6-devkitc-1
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Output|<span title="supported">✅</span>|
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