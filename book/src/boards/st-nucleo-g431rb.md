# ST NUCLEO-G431RB

## References

- [Manufacturer link](https://web.archive.org/web/20260318122407/https://www.st.com/en/evaluation-tools/nucleo-g431rb.html)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `st-nucleo-g431rb`

- **Tier:** 3
- **Chip:** [STM32G431RB](../chips/stm32g431rb.md)
- **Chip Ariel OS Name:** `stm32g431rb`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b st-nucleo-g431rb
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="needs testing">🚦</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
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


  