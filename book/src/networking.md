# Networking

## Network Type Selection

Ariel OS currently supports two physical and link layers for networking: Ethernet-over-USB (aka CDC-NCM) and Wi-Fi.
Boards may support all of them, only some of them, or none of them.

Which link layer is used for networking must be selected at compile time,
through [laze modules](./build_system.md#laze-modules):

- `usb-ethernet`: Selects Ethernet-over-USB.
- `wifi-cyw43`: Selects Wi-Fi using the CYW43 chip along an RP2040 MCU (e.g., on the Raspberry Pi Pico W).
- `wifi-esp`: Selects Wi-Fi on an ESP32 MCU.

When available on the device, one of these module is always selected by default, currently preferring Wi-Fi networking.

Overriding this default selection is possible by explicitly selecting the desired module, as follows:

```sh
laze build --select usb-ethernet -b rpi-pico-w
```

## Network Credentials

For Wi-Fi, the network credentials have to be supplied via environment variables:

```sh
CONFIG_WIFI_NETWORK=<ssid> CONFIG_WIFI_PASSWORD=<pwd> laze build ...
```

## Using the Networking Link on the Device

### Configuring the Device

The [`#[ariel_os::config]` attribute macro][config-attr-macro-rustdoc] is currently used to provide network configuration for the device.

### Obtaining a Network Stack

A network stack can then be obtained using [`ariel_os::network::network_stack()`][network-stack-rustdoc].

See the [examples][examples-dir-repo] for details.

### Support for Network Protocols

Support for various network protocols can be enabled through [Cargo features listed in the documentation][rustdoc-homepage].
Most of these use `embassy_net`, which should be used through [`ariel_os::reexports::embassy_net`][embassy-net-reexport-rustdoc].

## Host Setup

### Static IPv4 Address Configuration

When using a device with a static IPv4 address,
the host computer can be configured as follows (where `host_address` is an IP address configured as gateway for the device):

```sh
ip address add <host_address>/24 dev <interface>
ip link set up dev <interface>
```

To double-check that the address has indeed be added, you can use:

```sh
ip address show dev <interface>
```

Replace `<interface>` with the name of the used network interface.
To find out the name of your interface you can use a command such as `ip addr`.

### Ethernet-over-USB

For Ethernet-over-USB, ensure that, in addition to the USB cable used for flashing
and debugging, the *user* USB port is also connected to the host computer with
a second cable.

[rustdoc-homepage]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/index.html
[config-attr-macro-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/attr.config.html
[network-stack-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/network/fn.network_stack.html
[embassy-net-reexport-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/reexports/embassy_net/index.html
[examples-dir-repo]: https://github.com/ariel-os/ariel-os/tree/main/examples
