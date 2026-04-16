# Flashing & Debugging

Ariel OS makes it easy to flash and debug applications, by unifying the different existing flashing mechanisms and debug interface protocols and selecting the most suitable one, based on the target board and development and production requirements.

> [!NOTE]
> The terms "debug", "debugging", and "debugger" tend to refer to a wide array of overlapping concepts.
> In the following, we define and use phrases that use narrower meanings, with the hope to make these clearer.

## Debug Interface Protocols, Debug Interfaces, and Probes

*Debug interface protocols* allow reading and writing from and to system memory and processor registers, setting breakpoints, and stepping through the program execution, among other things.
The two debug interface protocols supported by Ariel OS tooling are [JTAG][jtag-wikipedia] and [Serial Wire Debug (SWD)][swd-arm-spec]:
SWD is a variant of JTAG with a reduced pin count but, being an Arm technology, it is only found on Arm-based microcontrollers, while JTAG is vendor-agnostic.
Besides being a debug interface protocol, JTAG is actually more generic and, in particular, also enables boundary scans (automatically checking the traces of a PCB by taking direct control of the chip pins present on the board), which were originally its primary purpose.

Debug interface protocols interact with the microcontroller through a *debug interface*, which may be a physical interface or an interface internal to the microcontroller, connected to the processor and/or the microcontroller buses.

As host computers do not have support for these debug interface protocols, a debug probe is necessary.
*Debug probes* are USB devices that allow using these debug interface protocols, either with standard (e.g., [CMSIS-DAP][cmsis-dap]) or vendor-specific USB classes.
In some cases, debug probes can also be built into the microcontrollers themselves, behind a USB interface.

Ariel OS currently uses [probe-rs][probe-rs-tool-probe-rs-docs] to interact with debug probes.
probe-rs supports both SWD and JTAG, and allows to flash firmware, to reboot into it, and to fetch the debug output from the running application over the debug interface protocol.
When multiple host tools are available for a board, Ariel OS attempts to make the best choice, based on functionality and flashing performance.
However, to specifically choose probe-rs as the host tool, the `probe-rs` [laze module][laze-modules-book] can be selected.

> [!NOTE]
> probe-rs is currently focused on *debug interface protocols* only.
> It does not support other serial protocols used for [flashing through bootloaders](#flashing-through-bootloaders).

<!-- NOTE: We refer to flashing a *board*, not just a microcontroller, as the flash memory may be outside the microcontroller. -->
## Flashing a Board

In general, there are two ways of writing the firmware to the flash memory (or memories) where the processor(s) execute(s) from---i.e., to flash the board: either by using a debug interface protocol, or by booting into a bootloader available on the board and then using one of its supported methods.

### Flashing Trough Debug Interface Protocols

As debug interface protocols allow arbitrarily writing to system memory, they allow downloading the firmware into the flash memory, at the necessary location.
When the debug interface is available (e.g., during development), this is generally the preferred option when SWD is available.
After flashing has completed, debug interface protocols allow rebooting the microcontroller into the newly-flashed firmware.
Alternatively, debug probes also often feature a wire that allows asserting the reset signal of the microcontroller, thus triggering a hardware reset.

Ariel OS provides the [laze tasks][laze-tasks-book] listed in the following table:

| laze tasks        | Description                                                                                                    |
| ----------------- | -------------------------------------------------------------------------------------------------------------- |
| `run`             | Compiles, flashes, and runs an application. The [debug output](./debug-console.md) is printed in the terminal. |
| `flash`           | Compiles and flashes an application, before rebooting the target.                                              |
| `flash-erase-all` | Erases the entire flash memory, including user data. Unlocks it if locked.                                     |
| `reset`           | Reboots the target.                                                                                            |

> [!TIP]
> Debug interface protocols also allow writing the firmware to RAM (inside of flash) and rebooting from there, which could be useful during development as flash endurance is limited.
> However, as microcontrollers have much less RAM than flash, this is not often applicable, and not currently supported by Ariel OS.

> [!TIP]
> As debug interface protocols offer more functionality than bootloaders, and are usually easier to use in an automated fashion, they are generally preferred as the flashing method.
> However, sometimes they simply cannot be used, e.g., because the debug interface has been disabled in hardware or is not physically accessible.
> Additionally, in some cases, flashing through the bootloader may be faster than using the debug interface protocol, in which case it may be preferable to use the faster method.

### Flashing Through Bootloaders

Alternatively, boards can also be flashed through their bootloaders, if they have one.
Microcontrollers can have their own vendor-provided bootloaders, and/or be flashed with custom bootloaders.
Bootloaders thus allow to use various standard or vendor-specific serial protocols to flash firmware.

Depending on the microcontroller family and on the bootloader, the most common options are (in no particular order):

<!-- NOTE: Even if I2C is more rarely used, this is to illustrate that any serial protocol can usually be used by bootloaders; UART and SPI are not special. -->
- USB
    - [USB CDC-ACM][usb-cdc-acm-glossary-book]
    - [USB Device Firmware Upgrade (DFU)][usb-dfu-spec]
    - [DfuSe][dfuse-dfu-util] (non-standard ST protocol, based on USB DFU)
    - USB MSC (mass storage) with [UF2][uf2-repo], where the device appears as a mass storage device and expects a UF2 file to be copied to it
- UART
- SPI
- I2C

Other serial protocols may also be supported by bootloaders.

Depending on the serial protocol used and on the bootloader, the host tool may be able to trigger entering the bootloader automatically, or it may require a manual action on the board.
It also may or may not be possible to reboot the microcontroller automatically after flashing has completed.

Ariel OS provides the [laze tasks][laze-tasks-book] listed in the following table:

| laze tasks        | Availability                       | Description                                                                                                                                                                                                                                      |
| ----------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `run`             | ESP32 devices                      | Compiles, flashes, and runs an application. [Logs](#logging-transports) (not the debug output) are printed in the terminal. Currently uses [`espflash`][espflah-cratesio].                                                           |
| `flash-dfuse`     | DfuSe devices, i.e., STM32 devices | Compiles and flashes an application via DfuSe, the non-standard ST protocol based on USB DFU, before rebooting the target. Requires bootloader support for DfuSe in the microcontroller, and [dfu-util][dfu-util-homepage] on the host.          |

<!-- TODO: consider introducing an `espflash` laze module -->

## Debug Output Transports

<!-- TODO: mark "Debug Output" as not available on native -->
<!-- TODO: possibly link to the relevant section of Debug Console; but must be consistent with the definition/behavior of `ariel_os::debug::println!()` -->
Debug interface protocols as introduced above also allow providing an additional piece of functionality: a debug output.
Two main techniques exist to implement such debug output over debug interface protocols: [semihosting][arm-semihosting-docs], and [Real Time Transfer (RTT)][segger-rtt-wiki].
Even though originally vendor-specific technologies, they have been extended to other architectures and vendors (e.g., [semihosting on RISC-V][riscv-semihosting-spec]), and can be used on every microcontroller currently supported by Ariel OS.

The *debug output* provides an implementation for a `println!()` macro.

### Semihosting

[Semihosting][arm-semihosting-docs] provides various operations to interact with the host from the firmware running on the target.
A semihosting operation involves triggering a specific exception (e.g., with a breakpoint) after having set the arguments required for by operation in the appropriate processor registers.
This functionally behaves as a remote syscall interface: see for instance the [documentation of the `SYS_WRITE0` operation][arm-semihosting-sys-write0-docs], which allows sending a string to the host for the host to print it as debug output.

<!-- TODO: however the `semihosting` crate can still be imported and used normally; should we mention that? -->
> [!NOTE]
> Due to how semihosting works, it is extremely slow as a debug output, and semihosting is currently unsupported as a debug output in Ariel OS.

> [!TIP]
> probe-rs automatically prints the semihosting output when used in the firmware.

### Real Time Transfer (RTT)

[RTT][segger-rtt-wiki] output relies on in-memory buffers which are written to by the firmware on the target and read, in the background (when the microcontroller supports it), by the debug probe.
RTT supports having multiple such buffers, allowing to implement multiple channels.
In addition, RTT supports channels in both directions: from the target to the host ("up channels"), and from the host to the target ("down channels"), but the latter are not used for the debug output.
RTT also requires an in-memory RTT Control Block, which stores the locations of the in-memory channel buffers.
The RTT-enabled host tool either knows the location of the control block in memory, or scans the memory to find the magic bytes ("ID") the control block starts with.

> [!TIP]
> probe-rs automatically prints the RTT output when used in the firmware.

<!-- TODO: document the to-be-introduced laze module that enables RTT: `debug-output-rtt` -->

## Logging Transports

Orthogonally to the debug output, logging allows to print logs to the logging output.
In Ariel OS, macros from the [`ariel_os::log`][log-mod-rustdoc] module (from `trace!()` to `error!()`) are used for [logging][logging-book].
That module also provides a [`println!()` macro][log-println-macro-rustdoc], that also prints to the logging output.

Logging can use multiple transports; the table below presents those supported in Ariel OS and which hardware and host tool are required:

<!-- TODO: clarify *exactly* under which conditions each of these get enabled -->
| Logging transport | Description                                                   | Supported                    | How to enable                                                                    | Required hardware                                                                                                                                               | Required host tool             |
| ----------------- | ------------------------------------------------------------- | :--------------------------: | -------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| Debug output      | Prints logs on the [debug output](#debug-output-transports).  | ✅                           | Enabled with the [`debug-console`][debug-console-debug-console-book] laze module | [Debug probe](#debug-interface-protocols-debug-interfaces-and-probes) attached to the [debug interface](#debug-interface-protocols-debug-interfaces-and-probes) | Debug output-enabled host tool |
| USB CDC-ACM       | Prints logs through [USB CDC-ACM][usb-cdc-acm-glossary-book]. | Currently on ESP32 MCUs only | Enabled by the `esp-println` laze module, enabled by default on ESP32            | USB cable attached to the user USB port                                                                                                                         | Serial monitor                 |
| UART              | Prints logs over UART.                                        | Currently on ESP32 MCUs only | Enabled by the `esp-println` laze module, enabled by default on ESP32            | USB ⟷ UART adapter attached to the supported UART pins (may already be part of the board)                                                                       | Serial monitor                 |

<!-- TODO: document the to-be-introduced laze modules:
- `logging-over-debug-output`
- `logging-over-usb`
- `logging-over-uart`
-->

<!-- TODO: comment about the (non?) mutual exclusiveness of the logging transports: likely mutually exclusive at first, then simultaneous transports supported -->

<!-- TODO: document where panics get printed [#1061](https://github.com/ariel-os/ariel-os/issues/1061) -->

> [!IMPORTANT]
> When using [`defmt` as logging facade][defmt-facade-book], a `defmt`-enabled host tool must be used so that logs are rendered correctly, as `defmt` uses its own encoding on the wire.
> probe-rs and `espflash` both support `defmt`'s encoding transparently.
>
> When a separate serial monitor is needed, [`defmt-print`][defmt-print-cratesio] can be used as `defmt`-enabled serial monitor.
> If this is not possible, `defmt` should be disabled and [`log`][log-facade-book] used instead as logging facade.

> [!NOTE]
> Support for other logging transports will be added in the future, including support for UART and USB CDC-ACM on non-ESP32 devices.

<!-- TODO: verify this is true -->
> [!TIP]
> When a logging transport other than the debug output is enabled, logging can still be used when the debug output is disabled either in software (by disabling the [debug console][debug-console-debug-console-book]) or in hardware when the [debug interface](#debug-interface-protocols-debug-interfaces-and-probes) itself is disabled.
> This means that logging can still be used in production, even if the debug interface has been disabled.
>
> If this is unwanted, logging can be disabled altogether by disabling the [`logging-facade`][logging-book] laze module.

On ESP32 devices, Ariel OS uses [`espflash`][espflah-cratesio] by default to obtain and print logs.
Currently, the firmware automatically switches at runtime between using USB CDC-ACM or UART as logging transport.

> [!WARNING]
> This is likely to change in the future, and it may become necessary to select a specific laze module to choose which logging transport to enable and use.

## Additional Host-Related Functionality

On top of providing a debug output, [semihosting](#semihosting) also allows the implementation of other I/O and host-related functionality.
In particular, [`ariel_os::debug::exit()`][debug-console-exit-book] is currently implemented through semihosting on embedded platforms.

> [!TIP]
> Currently, Ariel OS uses the [`semihosting` crate][semihosting-cratesio], which provides support for semihosting on every architecture currently supported by Ariel OS.

[jtag-wikipedia]: https://en.wikipedia.org/wiki/JTAG
[swd-arm-spec]: https://developer.arm.com/documentation/ihi0031/latest/
[cmsis-dap]: https://arm-software.github.io/CMSIS-DAP/latest/index.html
[laze-modules-book]: ./build-system.md#laze-modules
[laze-tasks-book]: ./build-system.md#laze-tasks
[probe-rs-tool-probe-rs-docs]: https://probe.rs/docs/tools/probe-rs/
[usb-cdc-acm-glossary-book]: ./glossary.md
[usb-dfu-spec]: https://www.usb.org/sites/default/files/DFU_1.1.pdf
[dfu-util-homepage]: https://dfu-util.sourceforge.net/
[dfuse-dfu-util]: https://dfu-util.sourceforge.net/dfuse.html
[uf2-repo]: https://github.com/Microsoft/uf2
[arm-semihosting-docs]: https://developer.arm.com/documentation/dui0471/m/what-is-semihosting-/what-is-semihosting-
[arm-semihosting-sys-write0-docs]: https://developer.arm.com/documentation/dui0471/m/what-is-semihosting-/sys-write0--0x04-
[riscv-semihosting-spec]: https://docs.riscv.org/reference/platform-software/semihosting/_attachments/riscv-semihosting.pdf
[segger-rtt-wiki]: https://kb.segger.com/RTT
[log-mod-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/log/index.html
[logging-book]: ./debug-console.md#logging
[log-println-macro-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/log/macro.println.html
[debug-console-debug-console-book]: ./debug-console.md#debug-console
[espflah-cratesio]: https://crates.io/crates/espflash
[defmt-facade-book]: ./debug-console.md#defmt
[log-facade-book]: ./debug-console.md#log
[defmt-print-cratesio]: https://crates.io/crates/defmt-print
[semihosting-cratesio]: https://crates.io/crates/semihosting
[debug-console-exit-book]: ./debug-console.md#closing-the-debug-console-from-firmware
