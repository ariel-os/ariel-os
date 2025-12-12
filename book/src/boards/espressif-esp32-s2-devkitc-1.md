# Espressif ESP32-S2-DevKitC-1

## laze Builders

For more information on laze builders, check out [this page](../build-system.md).
### `espressif-esp32-s2-devkitc-1`

- **Tier:** 3
- **Chip:** [ESP32-S2](../chips/esp32s2.md)
- **Chip Ariel OS Name:** `esp32s2`

To target this laze builder, run the following command in the root of your ArielOS app:

```bash
laze build -b espressif-esp32-s2-devkitc-1
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="needs testing">🚦</span>|
|SPI Main Mode|<span title="needs testing">🚦</span>|
|UART|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^usb-does-not-enumerate][^see-also-https-github-com-ariel-os-ariel-os-issues-903]|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
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

## References

- [Manufacturer link](https://web.archive.org/web/20251022182104/https://www.espressif.com/en/dev-board/esp32-s2-devkitc-1-en)


  
[^usb-does-not-enumerate]: USB does not enumerate.
[^see-also-https-github-com-ariel-os-ariel-os-issues-903]: See also: https://github.com/ariel-os/ariel-os/issues/903.
[^requires-partitioning-support]: Requires partitioning support.