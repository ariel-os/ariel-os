# Black Pill STM32F401RC

## References

- [Manufacturer link](https://stm32world.com/wiki/Black_Pill)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `blackpill-f401rc`

- **Tier:** 3
- **Chip:** [STM32F401RC](../chips/stm32f401rc.md)
- **Chip Ariel OS Name:** `stm32f401rc`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b blackpill-f401rc
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
|User USB|<span title="supported">✅</span>|
|Ethernet over USB|<span title="supported">✅</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="not available on this piece of hardware">–</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^unsupported-heterogeneous-flash-organization]|

#### Additional Notes

The board features:
- 25 MHz HSE crystal
- 32.768 kHz LSE crystal
- USB-C connector connected directly to the MCU USB FS peripheral
- SWD programming interface
- Blue user LED on PC13
- User button on PA0
- soic-8 footprint connected to SPI1


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


  
[^unsupported-heterogeneous-flash-organization]: Unsupported heterogeneous flash organization.