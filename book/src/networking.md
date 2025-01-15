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

## Static IPv4 Address Configuration

When using a device with a static IPv4 address,
the host computer can be configured as follows:

```sh
ip address add <address>/24 dev <interface>
ip link set up dev <interface>
```

To double-check that the address has indeed be added, you can use:

```sh
ip address show dev <interface>
```

Replace `<interface>` with the name of the used network interface.
To find out the name of your interface you can use a command such as `ip addr`.

## Network Credentials

For Wi-Fi, the network credentials have to be supplied via environment variables:

```sh
CONFIG_WIFI_NETWORK=<ssid> CONFIG_WIFI_PASSWORD=<pwd> laze build ...
```

## Ethernet-over-USB

For Ethernet-over-USB, ensure that, in addition to the USB cable used for flashing
and debugging, the *user* USB port is also connected to the host computer with
a second cable.
