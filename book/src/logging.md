# Logging

Ariel OS supports logging on all platforms and it is enabled by default thanks to the `logging` [laze module][laze-modules-book].
Logging offers a set of macros that print on the debug console with helpful logging formatting.

## Printing Panics

Panics are automatically printed on the logging output.
If this is unwanted, the `panic-printing` [laze module][laze-modules-book] can be disabled.

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

In Ariel OS, the log level defaults to `info`. It can be configured using the
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

### [log]

To build your project with plain logging support rather than defmt, use the `--select=log` option on the command line. For example:

```
$ laze build -C examples/log --builders nrf52840dk --select=log
```

Ariel OS's logger for `log` supports configuring the log level globally, but does not currently support per-crate filtering.

## Logging Transports

Logging can use various transports, but currently only one can be used at a time.
The table below presents those supported in Ariel OS and which hardware and host tool are required:

| Logging transport                        | Supported               | laze module                  | Required hardware                                                                         | Required host tool              |
| ---------------------------------------- | :---------------------: | ---------------------------- | ----------------------------------------------------------------------------------------- | ------------------------------- |
| [Debug channel][debug-channel-book]      | Available on all chips  | `logging-over-debug-channel` | Debug probe attached to the debug interface                                               | Debug channel-enabled host tool |
| [USB CDC-ACM][usb-cdc-acm-glossary-book] | On ESP32 MCUs only      | `logging-over-usb`           | USB cable attached to the user USB port                                                   | Serial monitor                  |
| [UART][uart-glossary-book]               | On ESP32 MCUs only      | `logging-over-uart`          | USB ⟷ UART adapter attached to the supported UART pins (may already be part of the board) | Serial monitor                  |

On ESP32 devices, Ariel OS uses [`espflash`][espflah-cratesio] by default to obtain and print logs, whose usage is determined by the `espflash` [laze module][laze-modules-book].
When `espflash` is selected at the time of compilation, `logging-over-debug-channel` is not enabled and one of the other available logging transports is used instead.

> [!IMPORTANT]
> When using [`defmt` as logging facade](#defmt), a `defmt`-enabled host tool must be used so that logs are rendered correctly, as `defmt` uses its own encoding on the wire.
> probe-rs and `espflash` both support `defmt`'s encoding transparently.
>
> When a separate serial monitor is needed, [`defmt-print`][defmt-print-cratesio] can be used as `defmt`-enabled serial monitor.
> If this is not possible, `defmt` should be disabled and [`log`](#log) used instead as logging facade.
>
> This is what you may see instead of your messages if you use `defmt` logging but use a serial monitor that is not `defmt`-enabled:
> ```
> ~� z� j� z� j� z� j� z� j� z� j
> ```

> [!TIP]
> When a logging transport other than the [debug channel][debug-channel-book] is enabled, logging can still be used when the debug channel is disabled either in software (by disabling the `logging-over-debug-channel` laze module) or in hardware when the debug interface itself is disabled.
> This means that logging can still be used in production, even if the debug interface has been disabled.
>
> If this is unwanted, logging can be disabled altogether by disabling the [`logging`](#logging) laze module.

> [!NOTE]
> Future plans for logging facilities:
>
> - Other logging transports will later be supported, including UART and USB CDC-ACM on non-ESP32 devices.
> - Using multiple transports at the same time may be supported in the future.

## Fetching and Displaying the Logging Output on the Host

When the [flashing][flashing-board-book] transport allows it, the `run` laze task not only flashes the board but also fetches and displays the logging output on the host.
In addition, the `attach` laze task is available to fetch and display the logging output from an already-flashed, running target.
Both the `run` and `attach` tasks may be [terminated by the target][closing-debug-console-book] when using [semihosting][semihosting-book].

The available laze tasks are summarized in the following table:

| [laze task][laze-tasks-book] | Supported          | Host tool              | [Logging transport](#logging-transports) |
| ---------------------------- | ------------------ | ---------------------- | ---------------------------------------- |
| `run`                        | On all chips       | probe-rs               | [Debug channel][debug-channel-book]      |
| `run`                        | On ESP32 MCUs only | `espflash`             | USB CDC-ACM or UART                      |
| `attach`                     | On all chips       | probe-rs               | [Debug channel][debug-channel-book]      |
| `attach`                     | On ESP32 MCUs only | `espflash`             | USB CDC-ACM or UART                      |

When multiple tasks of the same name exist, which variant is used depends on which host tool is enabled through its associated [laze module][laze-modules-book] (e.g., `probe-rs` or `espflash`).

> [!NOTE]
> Depending on the logging transport and on the [debug channel transport][debug-channel-book] used (if used as logging transport), the `attach` task may display some logs that have been issued before the task was started: this may happen when the transport uses internal buffers that are only emptied when fetched by a host tool (e.g., [RTT][rtt-book]).

> [!TIP]
> It is possible to [`flash` the board][flashing-board-book] using one host tool, before switching to another one to fetch and display the logging output (e.g., in production).

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
[debug-channel-book]: ./flashing-debugging.md#debug-channel-transports
[flashing-board-book]: ./flashing-debugging.md#flashing-a-board
[closing-debug-console-book]: ./debug-console.md#closing-the-debug-console-from-firmware
[semihosting-book]: ./flashing-debugging.md#semihosting
[laze-tasks-book]: ./build-system.md#laze-tasks
[rtt-book]: ./flashing-debugging.md#real-time-transfer-rtt
