# Device Updates

Ariel OS provides facilities to update deployed devices.
It relies on the combination of two mechanisms: a bootloader with rollback support, which expects a update payload in a dedicated partition, and separate mechanisms to populate that partition with the update payload over different transports.

> [!NOTE]
> Ariel OS may later support updating only part of the deployed application, or its runtime configuration, in specific cases.

## Bootloader Behavior

> [!NOTE]
> This describes the behavior on non-ESP32 MCUs.
> Support for ESP32 MCUs is being worked on.

Ariel OS's bootloader relies on two partitions for updating, to allow recovering from an incomplete update process or rolling back a buggy firmware update.
The bootloader expects the following partitions in [non-volatile memory][non-volatile-memory-glossary-book]:

TODO: layout diagram (order not guaranteed), for non-ESP devices
TODO: layout: defined by Ariel OS (order of partitions no guaranteed); location of bootloader required by microcontroller
    - Bootloader
    - Bootloader state
    - Active
    - DFU (Not to be confused with USB DFU)

> [!NOTE]
> The name of the DFU partition is unrelated to the USB DFU protocol; other mechanisms may be used to populate it.

To update its firmware, the application for responsible carrying out the follow steps:

<!-- NOTE: The application marks the update as ready using <https://docs.rs/embassy-boot/0.7.0/embassy_boot/struct.FirmwareUpdater.html#method.mark_updated>. -->
1. Obtaining the update payload and populating the DFU partition in non-volatile memory with it.
1. Checking the integrity of the written update payload.
1. Marking the update as ready (with a flag in non-volatile memory) for the bootloader.
1. Triggering a microcontroller reset to enter the bootloader.
1. Marking the update as successful, after the application boots again, and after checking it operates properly.

<!-- NOTE: reference: <https://blog.drogue.io/firmware-updates-part-1/#power-fail-safety>. -->
Ariel OS's bootloader is always executed first by the microcontroller reset.
When an update has not been marked as ready by the application (which is the normal booting process), the bootloader then jumps to the application.
When an update *has* been marked as ready, the bootloader swaps the contents of the active partition and of the DFU partition in a way that is robust to power failure.
When swapping is complete, the bootloader clears the "update ready" flag, and then jumps to the application as usual.
If the update should be rolled backed (see below), the bootloader swaps back the active and DFU partitions.

<!-- TODO: consider providing some guarantees about what the bootloader does not modify.  -->

This process is robust to power failures:

- If swapping is interrupted, the bootloader detects this as the "update ready" flag is still set, and simply resums the swapping.
- Similarly, rollback resumes if interrupted.

If the bootloader runs while swapping has previously completed but the update has not been marked as successful by the application, it concludes that that the new firmware is not operating properly and initiates a rollback, swapping back the partitions.
On supported microcontrollers, the bootloader sets up a watchdog to automatically trigger reset after a fixed duration, to allow rebooting into the bootloader if the application is not responsive after an update.

TODO: document no rollback loop

TODO: `ariel-os-bootloader` is Ariel OS's bootloader (an Ariel OS "application")
TODO: how to compile and flash the bootloader

> [!IMPORTANT]
> Currently the behavior is not compatible with [persistent storage][storage-book]: attempting to use it will result in the storage contents functionally being deleted on each device update.

> [!TIP]
> `ariel-os-bootloader` currently uses [`embassy-boot`][embassy-boot-embassy-book] for its bootloader implementation on non-ESP32 microcontrollers.

> [!NOTE]
> Currently the bootloader itself does check the update payload for integrity or authenticity.
> The application is responsible for doing this, after populating the DFU partition.

## Populating the DFU Partition With the Update Payload

As mentioned above, the application is generally responsible for populating the DFU partition with the update payload, to be picked up by the bootloader.
The application can implement fetching the update payload from the network, or through other means.
Alternatively, the DFU partition may be populated by circumventing the application, for instance through the [debug interface][debug-interface-book] or over ROM bootloaders' protocols (e.g., [DfuSe][flashing-through-bootloaders-dfuse-book] on STM32 MCUs or PICOBOOT on RP MCUs), before marking the update as ready.

The DFU partition can be populated while the device is normally operating (e.g., an update payload may be fetched by the application in the background).

[embassy-boot-embassy-book]: https://embassy.dev/book/#_bootloader
[non-volatile-memory-glossary-book]: ./glossary.md#non-volatile-memory
[storage-book]: ./storage.md
[debug-interface-book]: ./flashing-debugging.md#debug-interfaces-protocols-and-probes
[flashing-through-bootloaders-dfuse-book]: ./flashing-debugging.md#:~:text=DfuSe
