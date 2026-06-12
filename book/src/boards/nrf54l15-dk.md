# nRF54L15-DK

## References

- [Manufacturer link](https://www.nordicsemi.com/Products/Development-hardware/nRF54L15-DK)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `nrf54l15dk-app`

- **Tier:** 2
- **Chip:** [nRF54L15](../chips/nrf54l15.md)
- **Chip Ariel OS Name:** `nrf54l15-app`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b nrf54l15dk-app
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Logging|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|GPIO|<span title="needs testing">🚦</span>|
|I2C Controller Mode|<span title="needs testing">🚦</span>|
|SPI Main Mode|<span title="needs testing">🚦</span>|
|UART|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="needs testing">🚦</span>|
|Hardware Random Number Generator|<span title="needs testing">🚦</span>[^no-dedicated-rng-cracen]|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^rram-not-yet-supported]|

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


  
[^rram-not-yet-supported]: The nRF54L15 uses RRAM rather than flash; Ariel OS's storage backend does not yet support it.
  
[^no-dedicated-rng-cracen]: No dedicated RNG peripheral; randomness is provided by the CRACEN cryptographic accelerator.
