# makerdiary nRF9151 Connect Kit

## References

- [Manufacturer link](https://web.archive.org/web/20260522170547/https://makerdiary.com/products/nrf9151-connectkit)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `makerdiary-nrf9151-connect-kit`

- **Tier:** 2
- **Chip:** [nRF9151](../chips/nrf9151.md)
- **Chip Ariel OS Name:** `nrf9151`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b makerdiary-nrf9151-connect-kit
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="needs testing">🚦</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^only-available-through-the-cryptocell]|
|Persistent Storage|<span title="supported">✅</span>|

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


  
[^only-available-through-the-cryptocell]: Only available through the CryptoCell.