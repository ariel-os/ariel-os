# Waveshare ESP32-S3-Matrix

## References

- [Manufacturer link](https://web.archive.org/web/20251121141909/https://www.waveshare.com/wiki/ESP32-S3-Matrix)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `waveshare-esp32-s3-matrix`

- **Tier:** 3
- **Chip:** [ESP32-S3Fx4R2](../chips/esp32s3fx4r2.md)
- **Chip Ariel OS Name:** `esp32s3fx4r2`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b waveshare-esp32-s3-matrix
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Output|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="needs testing">🚦</span>|
|SPI Main Mode|<span title="needs testing">🚦</span>|
|UART|<span title="needs testing">🚦</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="supported">✅</span>|
|Ethernet over USB|<span title="supported">✅</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^requires-partitioning-support]|

#### Additional Notes

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


  
[^requires-partitioning-support]: Requires partitioning support.