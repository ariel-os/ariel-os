# ESP32-S2Fx4R2

## Chip Info

- **Ariel OS Name:** `esp32s2fx4r2`

## Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Output|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="needs testing">🚦</span>|
|UART|<span title="supported">✅</span>|
|User USB|<span title="supported">✅</span>|
|Ethernet over USB|<span title="supported">✅</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="supported">✅</span>[^requires-a-partition-with-type-data-and-subtype-undefined-in-the-partition-table-https-docs-espressif-com-projects-esp-idf-en-stable-esp32-api-guides-partition-tables-html]|

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




[^requires-a-partition-with-type-data-and-subtype-undefined-in-the-partition-table-https-docs-espressif-com-projects-esp-idf-en-stable-esp32-api-guides-partition-tables-html]: Requires a partition with type `data` and subtype `undefined` in the [Partition Table](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-guides/partition-tables.html).