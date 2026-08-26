# Ethernet

Ariel OS supports Ethernet (IEEE 802.3) on specific microcontrollers and hardware configurations.

## MAC Address Used for the Ethernet Link

The MAC address used for the Ethernet link is derived from the device identity using [`if_index 0`][identity-interface-eui48-rustdoc].
A different index should therefore be used to generate other EUI-48 identifiers.

## Using Microcontrollers With Built-in Ethernet MAC

Some microcontrollers include an Ethernet MAC peripheral, which requires an external PHY chip for the board to support Ethernet.
The MAC communicates with the external PHY using the [media-independent interface][mii-wikipedia] (MII) or the [reduced media-independent interface][rmii-wikipedia] (RMII), which is almost functionally identical to the MII but requires half the signals between the MAC and the PHY.

Currently, among the set of MCUs supported by Ariel OS, only some STM32 microcontrollers feature a built-in Ethernet MAC.
On these MCUs, only the RMII is currently supported: its pinout is currently fixed, and is the same as the one used by the manufacturer's development kit demonstrating Ethernet functionality.
Using Ethernet as the network link is enabled by selecting the [`ethernet-stm32`][ethernet-stm32-networking-book] [laze-module][laze-modules-book].

> [!NOTE]
> The built-in Ethernet MAC can be used on other boards, as long as the same pinout is used.

[identity-interface-eui48-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/identity/fn.interface_eui48.html
[mii-wikipedia]: https://en.wikipedia.org/wiki/Media-independent_interface
[rmii-wikipedia]: https://en.wikipedia.org/wiki/Media-independent_interface#Reduced_media-independent_interface
[ethernet-stm32-networking-book]: ./networking.md#:~:text=ethernet%2Dstm32
[laze-modules-book]: ./build-system.md#laze-modules
