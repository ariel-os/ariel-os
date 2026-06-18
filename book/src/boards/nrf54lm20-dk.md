# nRF54LM20-DK

## References

- [Manufacturer link](https://web.archive.org/web/2025/https://www.nordicsemi.com/Products/Development-hardware/nRF54LM20-DK)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `nrf54lm20dk`

- **Tier:** 3
- **Chip:** [nRF54LM20](../chips/nrf54lm20.md)
- **Chip Ariel OS Name:** `nrf54lm20`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b nrf54lm20dk
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Logging|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|GPIO|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|I2C Controller Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|SPI Main Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|UART|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Hardware Random Number Generator|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|

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


  