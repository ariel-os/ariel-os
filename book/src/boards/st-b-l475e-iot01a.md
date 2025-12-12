# ST B-L475E-IOT01A

## laze Builders

For more information on laze builders, check out [this page](../build-system.md).
### `st-b-l475e-iot01a`

- **Tier:** 2
- **Chip:** [STM32L475VG](../chips/stm32l475vg.md)
- **Chip Ariel OS Name:** `stm32l475vg`

To target this laze builder, run the following command in the root of your ArielOS app:

```bash
laze build -b st-b-l475e-iot01a
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="needs testing">🚦</span>|
|SPI Main Mode|<span title="needs testing">🚦</span>|
|UART|<span title="needs testing">🚦</span>|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="supported">✅</span>|
|Wi-Fi|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^an-external-wi-fi-module-is-present-on-the-board]|
|Bluetooth Low Energy|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^usb-does-not-enumerate][^see-also-https-github-com-embassy-rs-embassy-issues-2376][^workaround-in-https-github-com-ariel-os-ariel-os-pull-1126]|
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

- [Manufacturer link](https://web.archive.org/web/20250402084429/https://www.st.com/en/evaluation-tools/b-l475e-iot01a.html)


  
[^an-external-wi-fi-module-is-present-on-the-board]: An external Wi-Fi module is present on the board.
[^usb-does-not-enumerate]: USB does not enumerate.
[^see-also-https-github-com-embassy-rs-embassy-issues-2376]: See also: https://github.com/embassy-rs/embassy/issues/2376.
[^workaround-in-https-github-com-ariel-os-ariel-os-pull-1126]: Workaround in: https://github.com/ariel-os/ariel-os/pull/1126.
[^removing-items-not-supported]: Removing items not supported.