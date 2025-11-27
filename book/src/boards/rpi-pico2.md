# Raspberry Pi Pico 2

## Board Info

- **Tier:** 1
- **Ariel OS Name:** `rpi-pico2`
- **Chip:** [RP235xa](../chips/rp235xa.md)
- **Chip Ariel OS Name:** `rp235xa`



### References

- [Manufacturer link](https://web.archive.org/web/20250130144056/https://www.raspberrypi.com/products/raspberry-pi-pico-2/)

## Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^uart-loopback-test-only-works-once-after-a-power-cycle][^see-also-https-github-com-ariel-os-ariel-os-pull-1368-issuecomment-3406073140]|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="supported">✅</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="supported">✅</span>|
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

[^uart-loopback-test-only-works-once-after-a-power-cycle]: Uart-loopback test only works once after a power cycle.
[^see-also-https-github-com-ariel-os-ariel-os-pull-1368-issuecomment-3406073140]: See also: https://github.com/ariel-os/ariel-os/pull/1368#issuecomment-3406073140.