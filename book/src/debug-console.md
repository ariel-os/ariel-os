# Debug Console

<!-- NOTE: "Currently" because it could be extended with other semihosting functionality to make it an actual console. -->
The debug console is currently conceptually composed of the debug output and of the ability for the target to close it (when supported).
The debug console is enabled by default and the corresponding [laze module][laze-modules-book] is `debug-console`.

## Printing on the Debug Console

When the `logging-over-debug-output` [laze module][laze-modules-book] is enabled, [logs][logging-book] are printed on the debug output.

## Closing the Debug Console from Firmware

When using semihosting, it is possible for the target to request the debug console to exit, and to return an exit code indicating success or failure.
In Ariel OS, the [`ariel_os::debug::exit()` function][debug-exit-fn-rustdoc] can be used for this.
When using a host tool that supports semihosting, this will cause the tool to exit, with the exit code given to [`exit()`][debug-exit-fn-rustdoc] on the target being passed to the host.

The laze configuration automatically enables semihosting on the target when the host tool used *for flashing* supports semihosting (e.g., probe-rs).
When the flashing tool does not, support for semihosting can still be enabled in the firmware by selecting the `semihosting` [laze module][laze-modules-book].
This is needed to later be able to attach a semihosting-enabled host tool to the target.

> [!NOTE]
> When semihosting is enabled on the target and no host tool supporting semihosting (or a debugger) is connected, calling [`ariel_os::debug::exit()`][debug-exit-fn-rustdoc] may result in a panic.
> For example on ESP using `espflash` you would get:
>
> ```
> [ERROR] panicked at 'Unhandled interrupt on ProCpu' (esp_hal src/interrupt/mod.rs:90)
> ```

[laze-modules-book]: ./build-system.md#laze-modules
[logging-book]: ./logging.md
[debug-exit-fn-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/debug/fn.exit.html
