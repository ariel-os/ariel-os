# Arduino Uno (ATmega328P)

## References

- [Manufacturer link](https://web.archive.org/web/20260315192100/https://docs.arduino.cc/hardware/uno-rev3/)
- [ATmega328P Datasheet](https://www.microchip.com/en-us/product/ATmega328P)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `arduino-uno`

- **Tier:** 2
- **Chip:** ATmega328P
- **Chip Ariel OS Name:** `atmega328p`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b arduino-uno
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="not available on this piece of hardware">–</span>|
|Logging|<span title="supported with some caveats">☑️</span>[^log-caveats]|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="not available on this piece of hardware">–</span>|
|Persistent Storage|<span title="not available on this piece of hardware">–</span>[^storage-caveats]|

#### Additional Notes

The Arduino Uno (ATmega328P) is an 8-bit AVR microcontroller with **32 KB Flash** and **2 KB SRAM**. 
Due to the severely constrained RAM, the standard Ariel OS threading runtime (`ariel-os-threads`) is not used. 
Instead, AVR uses Embassy's **thread-mode executor** directly without the full threading layer.

To build for Arduino Uno, you need:
- Nightly Rust toolchain (2026-05-24 or later)
- `avr-none` target (`rustup target add avr-none`)
- `avr-libc` and `gcc-avr` for linking
- Build with `-Zbuild-std=core,alloc` and `RUSTFLAGS="-C target-cpu=atmega328p --cfg context=\"avr\" --cfg context=\"atmega328p\""`

Example build command:
```bash
RUSTFLAGS='-C target-cpu=atmega328p --cfg context="avr" --cfg context="atmega328p"' \
laze -C examples/blinky build -b arduino-uno -DCARGO_ARGS+="-Zbuild-std=core,alloc"
```

The `avr-device` crate provides the entry point and startup code. The `embassy-avr` crate (in the Ariel OS Embassy fork) provides the time driver (Timer0 CTC @ 1kHz) and GPIO/UART/I2C/SPI bindings via `avr-hal`.

**Memory constraints:**
- Total RAM (.data + .bss) must stay under 2048 bytes (2 KB)
- Stack size: 512 bytes (configured via `ram-tiny` laze module)
- No standard allocator by default; `alloc` feature is optional and limited
- Code size target: < 32 KB Flash

Flashing can be done with `avrdude`:
```bash
avrdude -p atmega328p -c arduino -P /dev/ttyUSB0 -b 115200 -U flash:w:build/bin/arduino-uno/cargo/avr-none/release/blinky.hex:i
```

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

[^log-caveats]: Logging via UART is supported, but defmt/RTT is not available on AVR. Only `log` crate with UART backend works.
[^storage-caveats]: No persistent storage support currently (no EEPROM driver implemented).
