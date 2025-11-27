# STM32U083C-DK

## laze Builders

For more information on laze builders, check out [this page](../build-system.md).
### `stm32u083c-dk`

- **Tier:** 1
- **Chip:** [STM32U083MC](../chips/stm32u083mc.md)
- **Chip Ariel OS Name:** `stm32u083mc`

To target this laze builder, run the following command in the root of your ArielOS app:

```bash
laze build -b stm32u083c-dk
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
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="supported with some caveats">☑️</span>[^removing-items-not-supported]|

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

- [Manufacturer link](https://web.archive.org/web/20250119131656/https://www.st.com/en/evaluation-tools/stm32u083c-dk.html)


  
[^removing-items-not-supported]: Removing items not supported.