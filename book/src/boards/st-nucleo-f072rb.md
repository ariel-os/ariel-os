# ST NUCLEO-F072RB

## References

- [Manufacturer link](https://www.st.com/en/evaluation-tools/nucleo-f072rb.html)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `st-nucleo-f072rb`

- **Tier:** 2
- **Chip:** [STM32F072RB](../chips/stm32f072rb.md)
- **Chip Ariel OS Name:** `stm32f072rb`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b st-nucleo-f072rb
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Output|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="not available on this piece of hardware">–</span>|
|Persistent Storage|<span title="supported">✅</span>[^removing-items-not-supported]|

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