# ST NUCLEO-H753ZI

## References

- [Manufacturer link](https://web.archive.org/web/20240625050043/https://www.st.com/en/evaluation-tools/nucleo-h753zi.html)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `st-nucleo-h753zi`

- **Tier:** 3
- **Chip:** [STM32H753ZI](../chips/stm32h753zi.md)
- **Chip Ariel OS Name:** `stm32h753zi`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b st-nucleo-h753zi
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|SPI Main Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|UART|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Ethernet|<span title="supported">✅</span>|
|User USB|<span title="supported">✅</span>|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^usb-does-not-enumerate-https-github-com-embassy-rs-embassy-issues-2376-workaround-https-github-com-ariel-os-ariel-os-pull-1126]|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
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


  
[^usb-does-not-enumerate-https-github-com-embassy-rs-embassy-issues-2376-workaround-https-github-com-ariel-os-ariel-os-pull-1126]: [USB does not enumerate](https://github.com/embassy-rs/embassy/issues/2376), [workaround](https://github.com/ariel-os/ariel-os/pull/1126).
[^removing-items-not-supported]: Removing items not supported.