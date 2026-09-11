# UART

## Capabilities

Ariel OS provides portable [UART][uart-glossary-book] drivers, which implement [`embedded_io_async::Read`][embedded-io-async-read-docsrs] and [`embedded_io_async::Write`][embedded-io-async-write-docsrs].
They can be enabled using the `uart` Cargo feature[^peripherals-free-cargo-feature].

Ariel OS currently favors async interfaces, and the UART drivers do not implement [`embedded_io::Read`][embedded-io-read-docsrs] or [`embedded_io::Write`][embedded-io-write-docsrs].

TODO: a couple of sentences on the UART drivers being buffered, the benefits, the trade-offs, other implications (if any), and the usual "non-buffered UART drivers are not currently supported/provided" if that makes sense

## Usage

### Instantiating the Driver

The driver for a buffered UART can be instantiated as follows (the configuration is [explained below](#configuration)):

```rust
let mut rx_buf = [0u8; 32];
let mut tx_buf = [0u8; 32];

let mut uart = TestUart::new(
    peripherals.uart_rx,
    peripherals.uart_tx,
    &mut rx_buf,
    &mut tx_buf,
    uart_config,
)
.expect("Invalid UART configuration");
```

The UART driver determines which UART peripheral is used: they are available in `ariel_os::hal::uart`; their names match that of the [Embassy-style HAL][embassy-style-hals] (in the example above, `TestUart` is a type alias for the driver selected).
Pin peripherals can be [obtained through the usual mechanisms][obtaining-peripheral-access-book].
Some microcontrollers allow using (almost) any pair of pins, but most require specific pins for each peripheral: this is automatically enforced at compile time.

> [!NOTE]
> Currently, pins for both RX and TX must always be provided.

TODO: how to determine the sizing of the buffers? The RX and TX buffers do not need to have the same size.

### Configuration

UART configuration is HAL-specific, and a default configuration can be obtained this way:

```rust
let mut uart_config = ariel_os::hal::uart::Config::default();
```

The default configuration is the following:

- 8 data bits
- 1 stop bit
- No parity bit

#### Selecting the Baud Rate

Ariel OS provides a set of common baud rates with [`ariel_os::uart::Baudrate`][uart-baudrate-rustdoc], up to 115,200 baud:

```rust
uart_config.baudrate = ariel_os::uart::Baudrate::_115200;
```

To use a baud rate outside of that set, especially a higher baud rate, HAL-specific types must be used instead:

```rust
uart_config.baudrate = ariel_os::uart::Baudrate::Hal(250_000.into())
// or, if the microcontroller or the HAL does not support arbitrary baud rates:
uart_config.baudrate = ariel_os::hal::uart::Baudrate::_1000000,
```

When providing an arbitrary baud rate, it is checked at runtime when the [driver is instantiated](#instantiating-the-driver).

> [!IMPORTANT]
> On the RP2040 and the RP235x MCUs, providing a baud rate too high instead uses the highest supported baud rate.

> [!TIP]
> HAL-specific baud rates types should only be used when required, to keep the application as portable as possible.

#### Configuring Other Settings

The number of data and stop bits and whether a parity bit is used can also be configured, when supported by the microcontroller and the HAL.

[^peripherals-free-cargo-feature]: When the Cargo feature is not enabled (including indirectly as a dependency of another Cargo feature), the peripherals and their interrupts are not managed by Ariel OS and can be used separately, including through their [Embassy-style HAL].

[uart-glossary-book]: ./glossary.md#uart
[embedded-io-async-read-docsrs]: https://docs.rs/embedded-io-async/latest/embedded_io_async/trait.Read.html
[embedded-io-async-write-docsrs]: https://docs.rs/embedded-io-async/latest/embedded_io_async/trait.Write.html
[embedded-io-read-docsrs]: https://docs.rs/embedded-io/latest/embedded_io/trait.Read.html
[embedded-io-write-docsrs]: https://docs.rs/embedded-io/latest/embedded_io/trait.Write.html
[obtaining-peripheral-access-book]: ./application.md#obtaining-peripheral-access
[uart-baudrate-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/uart/enum.Baudrate.html
[embassy-style-hals]: ./glossary.md#embassy-style-hals
