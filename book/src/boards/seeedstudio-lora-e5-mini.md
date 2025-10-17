# Seeed Studio LoRa-E5 mini

## Board Info

- **Tier:** 3
- **Ariel OS Name:** `seeedstudio-lora-e5-mini`
- **Chip:** STM32WLE5JC
- **Chip Ariel OS Name:** `stm32wle5jc`

### References

- [Manufacturer link](https://web.archive.org/web/20250802201959/https://wiki.seeedstudio.com/LoRa_E5_mini/)

## Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|SPI Main Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|UART|<span title="supported">✅</span>[^usart2-is-used-for-swi-on-this-board-and-cannot-be-used-as-uart-peripheral]|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
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

[^usart2-is-used-for-swi-on-this-board-and-cannot-be-used-as-uart-peripheral]: USART2 is used for SWI on this board and cannot be used as UART peripheral.