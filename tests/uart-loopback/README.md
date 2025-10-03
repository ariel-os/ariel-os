# UART loopback test

## About

This application tests the UART peripheral via an external loopback wire.

## How to run

1. Ensure a wire is present between the TX and RX pins.
2. In this directory, run:

    laze build -b nrf52840dk run

The test attempts to do a transfer and compares if what was sent has been read back.

## Potential issues

- An existing serial connection on the board can interfere with the test.
  The TX pin of the other serial endpoint can conflict with the loopback operation.
