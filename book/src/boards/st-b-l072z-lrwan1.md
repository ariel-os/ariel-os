# ST B-L072Z-LRWAN1

## References

- [Manufacturer link](https://www.st.com/en/evaluation-tools/b-l072z-lrwan1.html)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `st-b-l072z-lrwan1`

- **Tier:** 3
- **Chip:** [STM32L072CZ](../chips/stm32l072cz.md)
- **Chip Ariel OS Name:** `stm32l072cz`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b st-b-l072z-lrwan1
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="needs testing">🚦</span>|
|SPI Main Mode|<span title="needs testing">🚦</span>|
|UART|<span title="needs testing">🚦</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="needs testing">🚦</span>|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^not-enough-ram-for-the-network-stack]|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="needs testing">🚦</span>|
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


  
[^not-enough-ram-for-the-network-stack]: Not enough RAM for the network stack.