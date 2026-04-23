# Logging

Ariel OS supports logging on all platforms and it is enabled by default with the `logging-facade` [laze module][laze-modules-book].
Logging offers a set of macros that print on the debug console with helpful logging formatting.

## Logging

Within Rust code, import `ariel_os::log` items, then use Ariel OS logging macros:

```rust
use ariel_os::log::info;

#[ariel_os::task(autostart)]
async fn main() {
    info!("Hello!");
}
```

## Filtering Logs

In Ariel OS, the log level defaults to `info`. It can be configured using the
laze variable `LOG`.
Depending on the logger, it may be possible to configure different levels per crate or per module.

Example:

```shell
$ laze build -C examples/log --builders nrf52840dk -DLOG=info run
```

## Logging Facades and Loggers

Ariel OS supports multiple logging facades and loggers.
Only one of them may be enabled at a time;
if none of them are enabled, logging statements become no-operations.
Enabling either the `defmt` or `log` [laze modules][laze-modules-book] allows selecting which logging facade and logger is used.
defmt should be preferred when possible as it results in smaller binaries.

> [!TIP]
> The `defmt` laze module is favored and enabled by default when possible for
> the target.
> Applications that specifically depend on it still need to explicitly
> [select][laze-modules-book] it to make the dependency explicit and increase
> robustness to potential future changes.

The precise set of formatting operations and traits required on formatted data
depends on the selected backend.
There are some wrapper structs available in the [`ariel_os::log`] module
that help represent some types in a portable way;
in particular, this includes [`Debug2Format`] and [`Display2Format`],
which (while defeating some of `defmt`'s optimizations) come in handy when debugging third party types.

[`ariel_os::log`]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/log/
[`Debug2Format`]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/log/struct.Debug2Format.html
[`Display2Format`]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/log/struct.Display2Format.html

### [defmt]

See the [defmt documentation] for general info on the defmt's facade and logger.

The defmt logger supports configuring the log level per crate and per module, as follows:

```shell
$ laze build -C examples/log --builders nrf52840dk -DLOG=info,ariel_os_rt=trace run
```

Note: On Cortex-M devices, the order of `ariel_os::log::println!()` output and
      `defmt` log output is not deterministic.

### [log]

Ariel OS's logger for `log` supports configuring the log level globally, but does not currently support per-crate filtering.

## Logging Transports

Logging can use multiple transports; the table below presents those supported in Ariel OS and which hardware and host tool are required:

| Logging transport                        | Description                                                   | Supported                    | How to enable                                            | Required hardware                                                                                                               | Required host tool             |
| ---------------------------------------- | ------------------------------------------------------------- | :--------------------------: | -------------------------------------------------------- | ----------------------------------------------------------------------------------------- | ------------------------------ |
| Debug output                             | Prints logs on the debug output.                              | ✅                           | Enabled with the `logging-over-debug-output` laze module | Debug probe attached to the debug interface                                               | Debug output-enabled host tool |
| [USB CDC-ACM][usb-cdc-acm-glossary-book] | Prints logs through USB CDC-ACM.                              | Currently on ESP32 MCUs only | Currently enabled by default on ESP32                    | USB cable attached to the user USB port                                                   | Serial monitor                 |
| [UART][uart-glossary-book]               | Prints logs over UART.                                        | Currently on ESP32 MCUs only | Currently enabled by default on ESP32                    | USB ⟷ UART adapter attached to the supported UART pins (may already be part of the board) | Serial monitor                 |

> [!NOTE]
> Logging transports are currently mutually exclusive: only one can be selected at a time.
> Support for using multiple transports at the same time may be added in the future.

> [!IMPORTANT]
> When using [`defmt` as logging facade](#defmt), a `defmt`-enabled host tool must be used so that logs are rendered correctly, as `defmt` uses its own encoding on the wire.
> probe-rs and `espflash` both support `defmt`'s encoding transparently.
>
> When a separate serial monitor is needed, [`defmt-print`][defmt-print-cratesio] can be used as `defmt`-enabled serial monitor.
> If this is not possible, `defmt` should be disabled and [`log`](#log) used instead as logging facade.

> [!NOTE]
> Support for other logging transports will be added in the future, including support for UART and USB CDC-ACM on non-ESP32 devices.

<!-- TODO: link to "debug interface" when the page exists -->
> [!TIP]
> When a logging transport other than the debug output is enabled, logging can still be used when the debug output is disabled either in software (by disabling the `logging-over-debug-output` laze module) or in hardware when the debug interface itself is disabled.
> This means that logging can still be used in production, even if the debug interface has been disabled.
>
> If this is unwanted, logging can be disabled altogether by disabling the [`logging`](#logging) laze module.

On ESP32 devices, Ariel OS uses [`espflash`][espflah-cratesio] by default to obtain and print logs.
Currently, the firmware automatically switches at runtime between using USB CDC-ACM or UART as logging transport.

> [!WARNING]
> This is likely to change in the future, and it may become necessary to select a specific laze module to choose which logging transport to enable and use.

[defmt]: https://github.com/knurling-rs/defmt
[defmt documentation]: https://defmt.ferrous-systems.com/
[log]: https://github.com/rust-lang/log
[laze-modules-book]: ./build-system.md#laze-modules
[usb-cdc-acm-glossary-book]: ./glossary.md#usb-cdc-acm
[uart-glossary-book]: ./glossary.md#uart
[log-mod-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/log/index.html
[defmt-print-cratesio]: https://crates.io/crates/defmt-print
[debug-console-debug-console-book]: ./debug-console.md#debug-console
[espflah-cratesio]: https://crates.io/crates/espflash
