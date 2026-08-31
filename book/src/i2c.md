# I2C

## Capabilities

Ariel OS provides portable I2C drivers, which implement [`embedded_hal_async::i2c:I2c`][embedded-hal-async-i2c-i2c-docsrs], so they can be used by higher-level drivers (e.g., sensor drivers).
They can be enabled using the `i2c` Cargo feature[^peripherals-free-cargo-feature].
Only the controller role is currently supported.

<!-- NOTE: Some ESP32 MCUs support hardware timeouts. -->
Ariel OS currently favors async interfaces, and the I2C drivers do not implement [`embedded_hal::i2c::I2c`][embedded-hal-i2c-i2c-docsrs], although the underlying driver may still block.
This allows Ariel OS to implement a software timeout on I2C transactions, so that unresponsive I2C targets do not lead to an unresponsive application even on hardware that does not implement hardware timeouts.

Even though the driver is called I2C (instead of TWI), [clock stretching][i2c-clock-stretching-wikipedia] is only supported if the microcontroller peripheral supports it.

## Usage

### Bus Sharing

Like all buses, I2C allows connecting multiple devices (I2C targets) on a single bus.
The driver for the I2C bus can be instantiated as follows (the configuration is [explained below](#bus-configuration)):

```rust
let i2c_bus = SensorI2c::new(peripherals.i2c_sda, peripherals.i2c_scl, i2c_config);
let i2c_bus = Mutex::new(i2c_bus);
```

The bus driver determines which I2C peripheral is used: they are available in `ariel_os::hal::i2c::controller`; their names match that of the [Embassy-style HAL][embassy-style-hals] (in the example above, `SensorI2c` is a type alias for the driver selected).
Pin peripherals can be [obtained through the usual mechanisms][obtaining-peripheral-access-book].
Some microcontrollers allow using (almost) any pair of pins, but most require specific pins for each peripheral: this is automatically enforced at compile time.

Then, a driver instance for each I2C target can be created using the following:

```rust
let mut i2c_device = ariel_os::i2c::controller::I2cDevice::new(&i2c_bus);
```

<!-- NOTE: `embassy_embedded_hal::shared_bus::asynch::i2c::I2cDeviceWithConfig` exists and support for it could be added later. -->
Methods provided by [`embedded_hal_async::i2c::I2c`][embedded-hal-async-i2c-i2c-docsrs] can then be used to communicate with the I2C target.
Configuration is applied to the whole bus, and it is not currently possible to use different configurations for individual I2C targets.

> [!TIP]
> If only a single I2C target is used on the bus, using the device driver is unnecessary as the bus driver also implements [`embedded_hal_async::i2c::I2c`][embedded-hal-async-i2c-i2c-docsrs].

### Bus Configuration

I2C configuration is HAL-specific, and a default configuration can be obtained this way:

```rust
let mut i2c_config = ariel_os::hal::i2c::controller::Config::default();
```

#### Selecting the Bus Frequency

Ariel OS provides a [helper function `highest_freq_in()`][i2c-controller-highest-freq-in-rustdoc] that helps keep the application portable while allowing to use the bus at the highest frequency possible on the hardware:

```rust
i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(100)..=Kilohertz::kHz(400)) };
```

Alternatively, if a specific frequency is required, enums are also available:

```rust
i2c_config.frequency = ariel_os::i2c::controller::Frequency::_400k;
```

It it also possible to use a HAL-specific frequency setting:

```rust
i2c_config.frequency = ariel_os::hal::i2c::controller::Frequency::UpTo400k(200), // Selects 200 kHz.
// or, if the microcontroller or the HAL does not support arbitrary frequencies:
i2c_config.frequency = ariel_os::hal::i2c::controller::Frequency::_400k; // Selects 400 kHz.
```

> [!TIP]
> HAL-specific frequency settings should only be used when required, to keep the application as portable as possible.

The maximum available frequency depends on the microcontroller, and may be further limited by the [Embassy-style HAL][embassy-style-hals] and by Ariel OS.
In particular, Fast Mode Plus (i.e., 1 MHz) is generally not currently supported.

#### Configuring Other Settings

Other settings may be available depending on the HAL and the microcontroller.

Some HALs allow enabling internal pull-up resistors on the I2C lines: these typically use the regular GPIO internal pull-ups, which have a higher resistance than usually recommended for I2C and will therefore limit the maximum achievable bus frequency.

Some HALs also allow increasing the drive strength of the pins used for the I2C lines: this increases the current draw but helps achieve higher frequencies as well as accommodate a higher number of I2C targets on the bus (i.e., a higher bus capacitance).

[^peripherals-free-cargo-feature]: When the Cargo feature is not enabled (including indirectly as a dependency of another Cargo feature), the peripherals and their interrupts are not managed by Ariel OS and can be used separately, including [through their Embassy-style HAL][using-third-party-hals-directly-book].

[embedded-hal-async-i2c-i2c-docsrs]: https://docs.rs/embedded-hal-async/latest/embedded_hal_async/i2c/trait.I2c.html
[embedded-hal-i2c-i2c-docsrs]: https://docs.rs/embedded-hal/latest/embedded_hal/i2c/trait.I2c.html
[i2c-controller-highest-freq-in-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/i2c/controller/fn.highest_freq_in.html
[i2c-clock-stretching-wikipedia]: https://en.wikipedia.org/wiki/I2C#Clock_stretching_using_SCL
[obtaining-peripheral-access-book]: ./application.md#obtaining-peripheral-access
[embassy-style-hals]: ./glossary.md#embassy-style-hals
[using-third-party-hals-directly-book]: ./application.md#using-the-third-party-hals-directly
