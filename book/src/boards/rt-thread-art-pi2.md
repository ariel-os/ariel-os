# RT-Thread ART-Pi2

## References

- [Manufacturer link](https://web.archive.org/web/20261031114745/https://github.com/RT-Thread-Studio/sdk-bsp-stm32h7r-realthread-artpi2)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `rt-thread-art-pi2`

- **Tier:** 3
- **Chip:** [STM32H7R7L8](../chips/stm32h7r7l8.md)
- **Chip Ariel OS Name:** `stm32h7r7l8`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b rt-thread-art-pi2
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|SPI Main Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|UART|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Wi-Fi|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^an-external-wi-fi-module-is-present-on-the-board]|
|Bluetooth Low Energy|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^an-external-bluetooth-module-is-present-on-the-board]|
|Hardware Random Number Generator|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|

#### Additional Notes

Applications currently link into and run from the 64 KiB internal Flash at `0x08000000` (using 456 KiB of contiguous AXI SRAM at `0x24000000`), which overwrites any bootloader stored in internal Flash.
External HyperFlash and HyperRAM are not yet configured.

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


  
[^an-external-wi-fi-module-is-present-on-the-board]: An external Wi-Fi module is present on the board.
[^an-external-bluetooth-module-is-present-on-the-board]: An external Bluetooth module is present on the board.