# lora-tx

## About

This application demonstrates raw LoRa transmission with Ariel OS, driving the
Semtech SX1276 radio embedded on the ST B-L072Z-LRWAN1 board through the
[`lora-phy`](https://crates.io/crates/lora-phy) crate.

It transmits a short LoRa packet every five seconds. Run the companion `lora-rx`
example on a second board (tuned to the same frequency and modulation
parameters: spreading factor 10, 125 kHz bandwidth, coding rate 4/8) to receive
the packets.

Adjust `LORA_FREQUENCY_IN_HZ` in `src/main.rs` to a frequency that is legal in
your region before transmitting.

## How to run

In this directory, run

    laze build -b st-b-l072z-lrwan1 run
