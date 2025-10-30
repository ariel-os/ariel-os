# Espressif ESP32-C3-DevKit-RUST-1

## Board Info

- **Tier:** 3
- **Ariel OS Name:** `espressif-esp32-c3-devkit-rust-1`
- **Chip:** ESP32-C3
- **Chip Ariel OS Name:** `esp32c3`

### References

- [Manufacturer link](http://web.archive.org/web/20250729095245/https://github.com/esp-rs/esp-rust-board/tree/v1.2)

## Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="not available on this piece of hardware">–</span>[^no-generic-usb-peripheral]|
|Wi-Fi|<span title="supported with some caveats">☑️</span>[^not-currently-compatible-with-threading]|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
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

[^no-generic-usb-peripheral]: No generic USB peripheral.
[^not-currently-compatible-with-threading]: Not currently compatible with threading.
[^requires-partitioning-support]: Requires partitioning support.