# SPI

## Capabilities

Ariel OS provides portable SPI drivers, which implement [`embedded_hal_async::spi:SpiBus`][embedded-hal-async-spi-spibus-docsrs] and [`embedded_hal_async::spi::SpiDevice`][embedded-hal-async-spi-spidevice-docsrs], so they can be used by higher-level drivers (e.g., sensor drivers).
They can be enabled using the `spi` Cargo feature[^peripherals-free-cargo-feature].
Only the main role is currently supported.

Ariel OS currently favors async interfaces, and the SPI drivers do not implement [`embedded_hal::spi::SpiBus`][embedded-hal-spi-spibus-docsrs], although the underlying driver may still block.

## Usage

### Bus Sharing

Like all buses, SPI allows connecting multiple devices (SPI subs) on a single bus.
The driver for the SPI bus can be instantiated as follows (the configuration is [explained below](#bus-configuration)):

```rust
let spi_bus = SensorSpi::new(
    peripherals.spi_sck,
    peripherals.spi_miso,
    peripherals.spi_mosi,
    spi_config,
);
let spi_bus = Mutex::new(spi_bus);
```

The bus driver determines which SPI peripheral is used: they are available in `ariel_os::hal::spi::main`; their names match that of the [Embassy-style HAL][embassy-style-hals] (in the example above, `SensorSpi` is a type alias for the driver selected).
Pin peripherals can be [obtained through the usual mechanisms][obtaining-peripheral-access-book].
Some microcontrollers allow using (almost) any pair of pins, but most require specific pins for each peripheral: this is automatically enforced at compile time.

> [!NOTE]
> Currently, pins for both MISO and MOSI must always be provided.

Then, a driver instance for each SPI sub can be created using the following:

```rust
let cs_output = ariel_os::gpio::Output::new(peripherals.spi_cs, gpio::Level::High);
let mut spi_device = ariel_os::spi::main::SpiDevice::new(&spi_bus, cs_output);
```

The output must be set to high initially, as the chip select (CS) is an active-low signal.

<!-- NOTE: `embassy_embedded_hal::shared_bus::asynch::spi::SpiDeviceWithConfig` exists and support for it could be added later. -->
Methods provided by [`embedded_hal_async::spi::SpiDevice`][embedded-hal-async-spi-spidevice-docsrs] can then be used to communicate with the SPI sub.
Configuration is applied to the whole bus, and it is not currently possible to use different configurations for individual SPI subs.

### Bus Configuration

SPI configuration is HAL-specific, and a default configuration can be obtained this way:

```rust
let mut spi_config = hal::spi::main::Config::default();
```

#### Selecting the Bus Frequency

Ariel OS provides a [helper function `highest_freq_in()`][spi-main-highest-freq-in-rustdoc] that helps keep the application portable while allowing to use the bus at the highest frequency possible on the hardware:

```rust
spi_config.frequency = const { highest_freq_in(Kilohertz::kHz(1000)..=Kilohertz::kHz(2000)) };
```

Alternatively, if a specific frequency is required, enums are also available:

```rust
spi_config.frequency = ariel_os::spi::main::Frequency::_500k;
```

It it also possible to use a HAL-specific frequency setting:

```rust
spi_config.frequency = ariel_os::hal::spi::main::Frequency::F(Kilohertz::kHz(250)); // Selects 250 kHz.
// or, if the microcontroller or the HAL does not support arbitrary frequencies:
spi_config.frequency = ariel_os::hal::spi::main::Frequency::_500k; // Selects 500 kHz.
```

> [!TIP]
> HAL-specific frequency settings should only be used when required, to keep the application as portable as possible.

The maximum available frequency depends on the microcontroller, and may be further limited by the [Embassy-style HAL][embassy-style-hals] and by Ariel OS.
In particular, it is currently limited to that of the SPI peripheral supporting the lowest maximum frequency.

#### Selecting the SPI Mode

The SPI mode can be configured using [`Mode`][spi-mode-rustdoc]:

```rust
spi_config.mode = ariel_os::spi::Mode::Mode3;
```

[^peripherals-free-cargo-feature]: When the Cargo feature is not enabled (including indirectly as a dependency of another Cargo feature), the peripherals and their interrupts are not managed by Ariel OS and can be used separately, including through their [Embassy-style HAL].

[embedded-hal-async-spi-spibus-docsrs]: https://docs.rs/embedded-hal-async/latest/embedded_hal_async/spi/trait.SpiBus.html
[embedded-hal-async-spi-spidevice-docsrs]: https://docs.rs/embedded-hal-async/latest/embedded_hal_async/spi/trait.SpiDevice.html
[embedded-hal-spi-spibus-docsrs]: https://docs.rs/embedded-hal/latest/embedded_hal/spi/trait.SpiBus.html
[spi-main-highest-freq-in-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/spi/main/fn.highest_freq_in.html
[spi-mode-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/spi/enum.Mode.html
[obtaining-peripheral-access-book]: ./application.md#obtaining-peripheral-access
[embassy-style-hals]: ./glossary.md#embassy-style-hals
