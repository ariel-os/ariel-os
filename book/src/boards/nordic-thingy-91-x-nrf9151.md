# Nordic Thingy:91 X

## Board Info

- **Tier:** 2
- **Ariel OS Name:** `nordic-thingy-91-x-nrf9151`
- **Chip:** [nRF9151](../chips/nrf9151.md)
- **Chip Ariel OS Name:** `nrf9151`



### References

- [Manufacturer link](https://web.archive.org/web/20250329185651/https://www.nordicsemi.com/Products/Development-hardware/Nordic-Thingy-91-X)

## Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="needs testing">🚦</span>|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^requires-supporting-the-onboard-nrf7002-chip]|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
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

[^requires-supporting-the-onboard-nrf7002-chip]: Requires supporting the onboard nRF7002 chip.
[^only-available-through-the-cryptocell]: Only available through the CryptoCell.