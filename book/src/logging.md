# Logging

Ariel OS supports logging on all platforms and it is enabled by default with the `logging` [laze module][laze-modules-book].
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

[defmt]: https://github.com/knurling-rs/defmt
[defmt documentation]: https://defmt.ferrous-systems.com/
[log]: https://github.com/rust-lang/log
[laze-modules-book]: ./build-system.md#laze-modules
