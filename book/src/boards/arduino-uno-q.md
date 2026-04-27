# Arduino UNO Q

## References

- [Manufacturer link](https://web.archive.org/web/20260315192100/https://docs.arduino.cc/hardware/uno-q/)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `arduino-uno-q`

- **Tier:** 2
- **Chip:** [STM32U585AI](../chips/stm32u585ai.md)
- **Chip Ariel OS Name:** `stm32u585ai`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b arduino-uno-q
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
|User USB|<span title="not available on this piece of hardware">–</span>[^despite-having-a-usb-port-on-the-board-it-is-only-usable-by-the-qualcomm-chip]|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="supported with some caveats">☑️</span>[^removing-items-not-supported]|

#### Additional Notes

The MCU can be flashed from the Qualcomm chip on the board which runs a Linux system with the [`openocd`](https://openocd.org/) binary ready for use. Install [adb](https://developer.android.com/tools/adb) to communicate with the Qualcomm chip.

Build your application (in this case the [blinky](https://github.com/ariel-os/ariel-os/tree/main/examples/blinky) example from the Ariel OS repository):
```bash
laze -C examples/blinky build -b arduino-uno-q
```

Then copy the binary onto the Qualcomm chip:
```bash
adb push build/bin/arduino-uno-q/cargo/thumbv8m.main-none-eabihf/release/blinky /tmp
```

Get a shell on the Qualcomm chip:
```bash
adb shell
```

Flash your application onto the STM32 MCU
```bash
/opt/openocd/bin/openocd -d3 -s /opt/openocd -f openocd_gpiod.cfg -c 'program /tmp/blinky verify reset exit'
```


This is a prose snippet that can be reused.

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


  
[^despite-having-a-usb-port-on-the-board-it-is-only-usable-by-the-qualcomm-chip]: Despite having a USB port on the board, it is only usable by the Qualcomm chip.
[^removing-items-not-supported]: Removing items not supported.