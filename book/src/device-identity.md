# Device Identity

Ariel OS provides a concept of device identity.
Building on the unique identifiers that can be extracted from a board, it provides an API to uniquely identify the device when needed, and to obtain unique but stable identifiers (e.g., MAC addresses).

## Availability

The API, exposed as [`ariel_os::identity`][ariel-os-identity-rustdoc], is always available, but its accessor functions may return errors when there is no implementation of it for the board.
This may happen for instance when the microcontroller does not feature a unique, factory-written ID, and the board does not allow either obtaining such unique identifiers (or there just is not an implementation to extract them yet).
It is not currently possible to provide custom implementations out-of-tree.
Instead, to make sure such an implementation exists in Ariel OS, the application may be made to depend on the `hw/device-identity` [laze module][laze-modules-book].
Additionally, the [support information][support-matrix-book] of each board mentions whether it is implemented.
To learn more about the properties of the unique identifiers provided, see the documentation of [`ariel_os::identity`][ariel-os-identity-rustdoc].

Examples of possible sources of unique identifiers include:

- A globally unique, factory-written ID of the microcontroller
- A universally administered EUI-48 (MAC address) of a networking chip present on the board
- A globally unique, factory-written ID of a flash chip present on the board

## MAC Address Generation

The [MAC address used for the Ethernet link][ethernet-mac-address-book] is derived from the device identity using [`if_index 0`][identity-interface-eui48-rustdoc].
A different index should therefore be used to generate other EUI-48 identifiers.

[ariel-os-identity-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/identity/index.html
[laze-modules-book]: ./build-system.md#laze-modules
[ethernet-mac-address-book]: ./ethernet.md#mac-address-used-for-the-ethernet-link
[support-matrix-book]: ./hardware-functionality-support.html
[identity-interface-eui48-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/identity/fn.interface_eui48.html
