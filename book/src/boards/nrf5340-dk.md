# nRF5340-DK

## laze Builders

For more information on laze builders, check out [this page](../build-system.md).
### `nrf5340dk`

- **Tier:** 1
- **Chip:** [nRF5340](../chips/nrf5340.md)
- **Chip Ariel OS Name:** `nrf5340`

To target this laze builder, run the following command in the root of your ArielOS app:

```bash
laze build -b nrf5340dk
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="supported">✅</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="supported">✅</span>|
|Hardware Random Number Generator|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^no-standalone-rng-in-the-application-core-only-in-the-cryptocell-which-is-not-currently-supported]|
|Persistent Storage|<span title="supported">✅</span>|
### `nrf5340dk-net`

- **Tier:** 1
- **Chip:** [nRF5340](../chips/nrf5340.md)
- **Chip Ariel OS Name:** `nrf5340`

To target this laze builder, run the following command in the root of your ArielOS app:

```bash
laze build -b nrf5340dk-net
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^no-standalone-rng-in-the-application-core-only-in-the-cryptocell-which-is-not-currently-supported]|
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

## References

- [Manufacturer link](https://web.archive.org/web/20250115224621/https://www.nordicsemi.com/Products/Development-hardware/nrf5340-dk)


  
[^no-standalone-rng-in-the-application-core-only-in-the-cryptocell-which-is-not-currently-supported]: No standalone RNG in the application core, only in the CryptoCell which is not currently supported.
  
[^no-standalone-rng-in-the-application-core-only-in-the-cryptocell-which-is-not-currently-supported]: No standalone RNG in the application core, only in the CryptoCell which is not currently supported.