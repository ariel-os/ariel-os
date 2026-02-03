# Heltec WiFi LoRa 32 V3

## References

- [Manufacturer link](https://web.archive.org/web/20250807184214/https://heltec.org/project/wifi-lora-32-v3/)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).
### `heltec-wifi-lora-32-v3`

- **Tier:** 3
- **Chip:** [ESP32-S3](../chips/esp32s3.md)
- **Chip Ariel OS Name:** `esp32s3`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b heltec-wifi-lora-32-v3
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Output|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="needs testing">🚦</span>|
|UART|<span title="supported">✅</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
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


  
[^requires-partitioning-support]: Requires partitioning support.