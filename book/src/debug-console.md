# Debug Console

## Printing on the Debug Console

The debug console is enabled by default and the corresponding [laze module][laze-modules-book] is `debug-console`.
The [`ariel_os::debug::println!()`][println-macro-rustdoc] macro is used to print on the debug console.

When the debug console is enabled, panic messages are automatically printed to it.
If this is unwanted, the `panic-printing` [laze module][laze-modules-book] can be disabled.

## Debug Output Backends

The debug console backend determines where `ariel_os::debug::println!()`
output, panic messages, and `log` output are sent.
This is a separate choice from selecting the logging facade (`defmt` or `log`).

In typical Ariel OS builds, the backend is usually selected by laze as part of
the target and runner setup, and can be overridden through laze modules when
needed.
Only one backend can be selected at a time.

| Backend                           | What it does                                                                                                                                     | How to select it                                                                                                                                                 |
|-----------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| RTT (`defmt-rtt` / `rtt-target`)  | Sends output over RTT. Use `defmt-rtt` with the `defmt` facade, and `rtt-target` with the `log` facade for human-readable debug console output.  | `defmt-rtt` is selected automatically when using `defmt` with `probe-rs`. `rtt-target` can be selected explicitly when you want RTT output for the `log` facade. |
| `debug-uart`                      | Sends human-readable output over the board's configured debug UART. This is a good choice when you want logs on a serial console.                | Select the `debug-uart` laze module (only for the `log` facade).                                                                                                 |
| `esp-println`                     | Uses `esp-println` on ESP targets. This integrates well with `espflash monitor`; with `defmt`, it also supports `defmt-espflash`.                | Selected automatically for ESP targets.                                                                                                                          |
| `custom-log-handler`              | Calls an application-provided function for every `println!()` call and every `log` record. Until a handler is installed, it is a no-op.          | Select the `custom-log-handler` laze module.                                                                                                                     |
| `std`                             | Sends output to the host standard output stream using `std::println!()`. This is the backend used by native builds.                              | Selected automatically for native builds.                                                                                                                        |

### Using `custom-log-handler`

`custom-log-handler` is intended for `println!()` output and for the `log`
facade.
Because the handler receives `core::fmt::Arguments<'_>`, it should format or
forward the arguments immediately rather than storing them for later.

Select the `debug-console` and `custom-log-handler` laze modules, and
optionally `log` if you want `log` records to use the same backend.
Then install the handler early during startup.
The handler can only be installed once and cannot be removed.

```rust
use ariel_os::debug::{self, log::info};
use core::fmt::Arguments;
use std::fs::OpenOptions;
use std::io::Write;

fn my_log_handler(args: Arguments<'_>) {
    // Do whatever you want with the line, such as writing it to a file or sending it to your server
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.log")
    {
        let _ = file.write_fmt(format_args!("{args}"));
    }
}

#[ariel_os::task(autostart)]
async fn main() {
    let _ = debug::install_log_handler(my_log_handler);

    debug::println!("Hello from println!");
    info!("Hello from log!");
}
```

Before `install_log_handler()` is called, this backend drops all output.

## Debug Logging

Ariel OS supports debug logging on all platforms and it is enabled by default with the `debug-logging-facade` [laze module][laze-modules-book].
Debug logging offers a set of macros that print on the debug console with helpful logging formatting.

### Logging

Within Rust code, import `ariel_os::debug::log` items, then use Ariel OS logging macros:

```rust
use ariel_os::debug::log::info;

#[ariel_os::task(autostart)]
async fn main() {
    info!("Hello!");
}
```

### Filtering Logs

In Ariel OS, the log level defaults to `info`. It can be configured using the
laze variable `LOG`.
Depending on the logger, it may be possible to configure different levels per crate or per module.

Example:

```shell
$ laze build -C examples/log --builders nrf52840dk -DLOG=info run
```

### Logging Facades and Loggers

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
There are some wrapper structs available in the [`ariel_os::debug::log`] module
that help represent some types in a portable way;
in particular, this includes [`Debug2Format`] and [`Display2Format`],
which (while defeating some of `defmt`'s optimizations) come in handy when debugging third party types.

[`ariel_os::debug::log`]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/debug/log/
[`Debug2Format`]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/debug/log/struct.Debug2Format.html
[`Display2Format`]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/debug/log/struct.Display2Format.html

#### [defmt]

See the [defmt documentation] for general info on the defmt's facade and logger.

The defmt logger supports configuring the log level per crate and per module, as follows:

```shell
$ laze build -C examples/log --builders nrf52840dk -DLOG=info,ariel_os_rt=trace run
```

Note: On Cortex-M devices, the order of `ariel_os::debug::println!()` output and
      `defmt` log output is not deterministic.

#### [log]

Ariel OS's logger for `log` supports configuring the log level globally, but does not currently support per-crate filtering.

[defmt]: https://github.com/knurling-rs/defmt
[defmt documentation]: https://defmt.ferrous-systems.com/
[log]: https://github.com/rust-lang/log
[laze-modules-book]: ./build-system.md#laze-modules
[println-macro-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/debug/macro.println.html
