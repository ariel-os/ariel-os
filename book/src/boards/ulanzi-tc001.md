# Ulanzi TC001

## References

- [Manufacturer link](https://web.archive.org/web/20260206132721/https://www.ulanzi.com/products/ulanzi-pixel-smart-clock-2882)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `ulanzi-tc001`

- **Tier:** 2
- **Chip:** [ESP32-D0WD](../chips/esp32-d0wd.md)
- **Chip Ariel OS Name:** `esp32-d0wd`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b ulanzi-tc001
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Output|<span title="not available on this piece of hardware">–</span>[^jtag-pins-are-not-accessible-through-the-case]|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="needs testing">🚦</span>|
|UART|<span title="needs testing">🚦</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^requires-partitioning-support]|

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


  
[^jtag-pins-are-not-accessible-through-the-case]: JTAG pins are not accessible through the case.
[^requires-partitioning-support]: Requires partitioning support.