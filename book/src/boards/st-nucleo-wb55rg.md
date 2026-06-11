# ST NUCLEO-WB55RG

## References

- [Manufacturer link](https://web.archive.org/web/20240803070523/https://www.st.com/en/evaluation-tools/nucleo-wb55rg.html)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `st-nucleo-wb55`

- **Tier:** 1
- **Chip:** [STM32WB55RG](../chips/stm32wb55rg.md)
- **Chip Ariel OS Name:** `stm32wb55rg`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b st-nucleo-wb55
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="supported">✅</span>|
|Ethernet over USB|<span title="supported">✅</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
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


  
[^removing-items-not-supported]: Removing items not supported.