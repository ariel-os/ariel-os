# Raspberry Pi Pico W

## References

- [Manufacturer link](https://web.archive.org/web/20250101022830/https://www.raspberrypi.com/products/raspberry-pi-pico/)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).
### `rpi-pico-w`

- **Tier:** 1
- **Chip:** [RP2040](../chips/rp2040.md)
- **Chip Ariel OS Name:** `rp2040`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b rpi-pico-w
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
|User USB|<span title="supported">✅</span>|
|Ethernet over USB|<span title="supported">✅</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="supported">✅</span>|
#### Additional notes

Test prose for the Raspberry Pi Pico W builder.

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


  