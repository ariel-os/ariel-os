# nRF5340-DK

## Board Info

- **Tier:** 1
- **Ariel OS Name:** `nrf5340dk`
- **Chip:** [nRF5340](../chips/nrf5340.md)
- **Chip Ariel OS Name:** `nrf5340`

### References

- [Manufacturer link](https://web.archive.org/web/20250115224621/https://www.nordicsemi.com/Products/Development-hardware/nrf5340-dk)

## Support Matrix

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
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Ethernet over USB|<span title="supported">✅</span>|
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

[^no-standalone-rng-in-the-application-core-only-in-the-cryptocell-which-is-not-currently-supported]: No standalone RNG in the application core, only in the CryptoCell which is not currently supported.