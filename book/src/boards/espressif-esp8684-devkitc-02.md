# Espressif ESP8684-DevKitC-02

## References

- [Manufacturer link](https://web.archive.org/web/20260607024208/https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c2/esp8684-devkitc-02/user_guide.html)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `espressif-esp8684-devkitc-02-h2`

- **Tier:** 3
- **Chip:** [ESP8684H2](../chips/esp8684h2.md)
- **Chip Ariel OS Name:** `esp8684h2`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b espressif-esp8684-devkitc-02-h2
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="needs testing">🚦</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^requires-partitioning-support]|

### `espressif-esp8684-devkitc-02-h4`

- **Tier:** 3
- **Chip:** [ESP8684H4](../chips/esp8684h4.md)
- **Chip Ariel OS Name:** `esp8684h4`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b espressif-esp8684-devkitc-02-h4
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="needs testing">🚦</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
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


  
[^requires-partitioning-support]: Requires partitioning support.
  
[^requires-partitioning-support]: Requires partitioning support.