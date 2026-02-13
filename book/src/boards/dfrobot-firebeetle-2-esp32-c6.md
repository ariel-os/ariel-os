# DFRobot FireBeetle 2 ESP32-C6

## References

- [Manufacturer link](https://web.archive.org/web/20250710082029/https://wiki.dfrobot.com/SKU_DFR1075_FireBeetle_2_Board_ESP32_C6)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).
### `dfrobot-firebeetle2-esp32-c6`

- **Tier:** 2
- **Chip:** [ESP32-C6Fx4](../chips/esp32c6fx4.md)
- **Chip Ariel OS Name:** `esp32c6fx4`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b dfrobot-firebeetle2-esp32-c6
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
|User USB|<span title="not available on this piece of hardware">–</span>[^no-generic-usb-peripheral]|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
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


  
[^no-generic-usb-peripheral]: No generic USB peripheral.
[^requires-a-partition-with-type-data-and-subtype-undefined-in-the-partition-table-https-docs-espressif-com-projects-esp-idf-en-stable-esp32-api-guides-partition-tables-html]: Requires a partition with type `data` and subtype `undefined` in the [Partition Table](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-guides/partition-tables.html).