# ST NUCLEO-H755ZI-Q

## Board Info

- **Tier:** 1
- **Ariel OS Name:** `st-nucleo-h755zi-q`
- **Chip:** [STM32H755ZI](../chips/stm32h755zi.md)
- **Chip Ariel OS Name:** `stm32h755zi`

### References

- [Manufacturer link](https://web.archive.org/web/20240524105149/https://www.st.com/en/evaluation-tools/nucleo-h755zi-q.html)

## Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="supported">✅</span>|
|Wi-Fi|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^usb-does-not-enumerate][^see-also-https-github-com-embassy-rs-embassy-issues-2376][^workaround-in-https-github-com-ariel-os-ariel-os-pull-1126]|
|Hardware Random Number Generator|<span title="supported">✅</span>|
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

[^usb-does-not-enumerate]: USB does not enumerate.
[^see-also-https-github-com-embassy-rs-embassy-issues-2376]: See also: https://github.com/embassy-rs/embassy/issues/2376.
[^workaround-in-https-github-com-ariel-os-ariel-os-pull-1126]: Workaround in: https://github.com/ariel-os/ariel-os/pull/1126.
[^removing-items-not-supported]: Removing items not supported.