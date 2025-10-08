# Espressif ESP32-C6-DevKitC-1

## Board Info

- **Tier:** 1
- **Ariel OS Name:** `espressif-esp32-c6-devkitc-1`
- **Chip:** ESP32-C6
- **Chip Ariel OS Name:** `esp32c6`

### References

- [Manufacturer link](https://web.archive.org/web/20250122153727/https://www.espressif.com/en/dev-board/esp32-c6-devkitc-1-en)

## Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
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