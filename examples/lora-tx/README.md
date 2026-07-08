# lora-tx

## About

This application demonstrates raw LoRa transmission with Ariel OS, driving the
Semtech SX1276 radio embedded on the ST B-L072Z-LRWAN1 board through the
[`lora-phy`](https://crates.io/crates/lora-phy) crate.

It transmits a 16 bytes LoRa packet every five seconds and logs its payload:

    Sent message 12 (16 bytes): [41, 52, 01, 8d, b9, 50, 0e, 0c, 00, 00, 00, b8, ea, 00, 00, 0e]

The payload contains, after a `"AR"` magic and a format version byte, a node ID
derived from the device ID of the board, the index of the message (incremented
at each send), the uptime of the board when sending, in milliseconds, and the TX
power, in dBm. The layout is documented in `src/message.rs`.

Run the companion `lora-rx` example on a second board (tuned to the same
frequency and modulation parameters: spreading factor 7, 125 kHz bandwidth,
coding rate 4/5) to receive the packets.

With these parameters, a packet is about 47 ms on air: sending one every five
seconds stays below the 1% duty cycle allowed on the EU868 868.0-868.6 MHz
sub-band. Keep the duty cycle in mind when changing the payload, the
modulation parameters or the interval.

Adjust `LORA_FREQUENCY_IN_HZ` in `src/main.rs` to a frequency that is legal in
your region before transmitting.

## How to run

In this directory, run

    laze build -b st-b-l072z-lrwan1 run
