# UART echo example

## About

This application hows how to use the UARTs that are defined in a board's SBD descriptions.

## How to run

2. In this directory, run:

    laze build -b nrf52840dk run

The example initializes the host-facing UART and echoes all incoming bytes.
