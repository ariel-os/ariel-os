# device-metadata

## About

This application prints all device information that the application knows
about the device and the running program
at startup to the debug console.

On some boards where there is additional identifying hardware,
that hardware is queried too:
For example, on nRF91 boards, it queries the serial number of the on-chip modem.

## How to run

In this directory, run

    laze build -b nrf52840dk run

When running on nRF91 devices that are flashed with the DECT firmware,
make sure to pass `-s nrf-radiocore-firmware-dect` to laze --
otherwise, modem initialization will fail long before its serial number can be queried.

## Example output

When run, this example prints the board name and the device id, if available.
For example:

    INFO  Available information:
    INFO  Board type: nrf52840dk
    INFO  Device ID: [80, 6a, ec, 55, 8c, c2, 43, 8e]
    INFO  Device's first EUI-48 address: 02:9a:05:d7:38:e9
