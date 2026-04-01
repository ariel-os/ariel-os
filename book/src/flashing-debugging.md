# Flashing & Debugging

Ariel OS makes it easy to flash and debug applications, by unifying the different existing flashing mechanisms and debug protocols and selecting the most suitable one, based on the target board and development and production requirements.

> [!NOTE]
> The terms "debug", "debugging", and "debugger" tend to refer to a wide array of overlapping concepts.
> In the following, we define and use phrases that use narrower meanings, with the hope to make these clearer.

## Debug Protocols, Interfaces, and Probes

*Debug protocols* allow reading and writing from and to system memory and processor registers, setting breakpoints, and stepping through the program execution, among other things.
The two debug protocols supported by Ariel OS tooling are [JTAG][jtag-wikipedia] and [Serial Wire Debug (SWD)][swd-arm-spec]:
SWD is a variant of JTAG with a reduced pin count but, being an Arm technology, it is only found on Arm-based microcontrollers, while JTAG is vendor-agnostic.
Besides being a debug protocol, JTAG is actually more generic and, in particular, also enables boundary scans (automatically checking the traces of a PCB by taking direct control of the chip pins present on the board), which were originally its primary purpose.

Debug protocols interact with the microcontroller through a *debug interface*, which may be a physical interface or an interface internal to the microcontroller, connected to the processor and/or the microcontroller buses.

As host computers do not have support for these debug protocols, a debug probe is necessary.
*Debug probes* are USB devices that allow using these debug protocols, either with standard (e.g., [CMSIS-DAP][cmsis-dap]) or vendor-specific USB classes.
In some cases, debug probes can also be built into the microcontrollers themselves, behind a USB interface.

Ariel OS currently uses [probe-rs][probe-rs-tool-probe-rs-docs] to interact with debug probes.
probe-rs supports both SWD and JTAG, and allows to flash firmware, to reboot into it, and to fetch the debug output from the running application over the debug protocol.
When multiple host tools are available for a board, Ariel OS attempts to make the best choice, based on functionality and flashing performance.
However, to specifically choose probe-rs as the host tool, the `probe-rs` [laze module][laze-modules-book] can be selected.

<!-- NOTE: We refer to flashing a *board*, not just a microcontroller, as the flash memory may be outside the microcontroller. -->
## Flashing a Board

In general, there are two ways of writing the firmware to the flash memory (or memories) where the processor(s) execute(s) from---i.e., to flash the board: either by using a debug protocol, or by booting into a bootloader available on the board and then using one of its supported methods.

### Flashing Trough Debug Protocols

As debug protocols allow arbitrarily writing to system memory, they allow downloading the firmware into the flash memory, at the necessary location.
When the debug interface is available (e.g., during development), this is generally the preferred option when SWD is available.
After flashing has completed, debug protocols allow rebooting the microcontroller into the newly-flashed firmware.
Alternatively, debug probes also often feature a wire that allows asserting the reset signal of the microcontroller, thus triggering a hardware reset.

Ariel OS provides the [laze tasks][laze-tasks-book] listed in the following table:

| laze tasks        | Description                                                                                                    |
| ----------------- | -------------------------------------------------------------------------------------------------------------- |
| `run`             | Compiles, flashes, and runs an application. The [debug output](./debug-console.md) is printed in the terminal. |
| `flash`           | Compiles and flashes an application, before rebooting the target.                                              |
| `flash-erase-all` | Erases the entire flash memory, including user data. Unlocks it if locked.                                     |
| `reset`           | Reboots the target.                                                                                            |

> [!TIP]
> Debug protocols also allow writing the firmware to RAM (inside of flash) and rebooting from there, which could be useful during development as flash endurance is limited.
> However, as microcontrollers have much less RAM than flash, this is not often applicable, and not currently supported by Ariel OS.

> [!TIP]
> As debug protocols offer more functionality than bootloaders, and are usually easier to use in an automated fashion, they are generally preferred as the flashing method.
> However, sometimes they simply cannot be used, e.g., because the debug interface has been disabled in hardware or is not physically accessible.
> Additionally, in some cases, flashing through the bootloader may be faster than using the debug protocol, in which case it may be preferable to use the faster method.

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
| `run`             | ESP32 devices                      | Compiles, flashes, and runs an application. [Debug logs](#debug-logging-transports) (not the debug output) are printed in the terminal.                                                                                                          |
| `flash-dfuse`     | DfuSe devices, i.e., STM32 devices | Compiles and flashes an application via DfuSe, the non-standard ST protocol based on USB DFU, before rebooting the target. Requires bootloader support for DfuSe in the microcontroller, and [dfu-util][dfu-util-homepage] on the host.          |

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
