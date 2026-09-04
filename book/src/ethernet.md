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

## Using External Ethernet MAC + PHY Chips

External Ethernet MAC + PHY chips also exist, which can for instance be used with microcontrollers that do not comprise a built-in MAC peripheral.
These external chips most commonly interface with the microcontroller through either SPI or a parallel bus.
On top of including the MAC and PHY functional blocks, these chips may also feature hardware support for higher level protocols, such as IP or TCP.

Ariel OS currently only supports these chips over SPI (QSPI is also not supported).
The table below detail the supported chips, and how to enable support for them:

| Chip           | [laze module][laze-modules-book] to select                 |
| -------------- | ---------------------------------------------------------- |
| WIZnet W5100S  | [`ethernet-wiznet-5100s`][ethernet-wiznet-networking-book] |
| WIZnet W5500   | [`ethernet-wiznet-5500`][ethernet-wiznet-networking-book]  |
| WIZnet W6100   | [`ethernet-wiznet-6100`][ethernet-wiznet-networking-book]  |
| WIZnet W6300   | [`ethernet-wiznet-6300`][ethernet-wiznet-networking-book]  |

Only the MAC of these chips is currently used (i.e., WIZnet chips are used in MACRAW mode): the hardware acceleration of higher level protocols is not leveraged.

[identity-interface-eui48-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/identity/fn.interface_eui48.html
[mii-wikipedia]: https://en.wikipedia.org/wiki/Media-independent_interface
[rmii-wikipedia]: https://en.wikipedia.org/wiki/Media-independent_interface#Reduced_media-independent_interface
[ethernet-stm32-networking-book]: ./networking.md#:~:text=ethernet%2Dstm32
[ethernet-wiznet-networking-book]: ./networking.md#:~:text=ethernet%2Dwiznet%2D%2A
[laze-modules-book]: ./build-system.md#laze-modules
