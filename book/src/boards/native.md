# native

## References

- [Manufacturer link](#)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `native`

- **Tier:** 1
- **Chip:** [native](../chips/native-chip.md)
- **Chip Ariel OS Name:** `native-chip`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b native
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Output|<span title="not available on this piece of hardware">–</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|I2C Controller Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|SPI Main Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|UART|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Wi-Fi|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Bluetooth Low Energy|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
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


  