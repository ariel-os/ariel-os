# IoT-LAB M3

## References

- [Manufacturer link](https://www.iot-lab.info/docs/boards/iot-lab-m3/)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `iotlab-m3`

- **Tier:** 3
- **Chip:** [STM32F103RE](../chips/stm32f103re.md)
- **Chip Ariel OS Name:** `stm32f103re`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b iotlab-m3
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="needs testing">🚦</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="needs testing">🚦</span>|
|SPI Main Mode|<span title="needs testing">🚦</span>|
|UART|<span title="needs testing">🚦</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="not available on this piece of hardware">–</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|

#### Additional Notes

This board is only available on the [FIT IoT-LAB](https://www.iot-lab.info/) testbed.
To get console output on the testbed serial infrastructure, the firmware must be built with the `logging-over-uart` laze module.

##### Usage on FIT IoT-LAB

Build your application (in this case the [log](https://github.com/ariel-os/ariel-os/tree/main/examples/log) example from the Ariel OS repository):
```bash
laze -C examples/log build -b iotlab-m3 -s logging-over-uart
```

Flash it on a node of your experiment with the IoT-LAB CLI tools, which upload the firmware through the IoT-LAB REST API:
```bash
iotlab-node --flash build/bin/iotlab-m3/cargo/thumbv7m-none-eabi/release/example-log -l <site>,m3,<id>
```

The console UART runs at 500000 baud and is exposed by the SSH frontend on TCP port 20000:
```bash
nc m3-<id> 20000
```

##### Flashing Without IoT-LAB Tools

The board embeds an FT2232H USB interface providing both JTAG and the console UART.
probe-rs supports FTDI adapters over JTAG, which the laze builder is configured to use;
OpenOCD (with `interface/ftdi/iotlab-usb.cfg`) can be used as a fallback.


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


  