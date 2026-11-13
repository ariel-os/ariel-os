# lora-rx

## About

This application demonstrates raw LoRa reception with Ariel OS, driving the
Semtech SX1276 radio embedded on the ST B-L072Z-LRWAN1 board through the
[`lora-phy`](https://crates.io/crates/lora-phy) crate.

It listens continuously and logs the payload of every LoRa packet it receives,
along with its length, RSSI and SNR. It is the companion of the `lora-tx`
example: flash `lora-tx` on a second board tuned to the same frequency and
modulation parameters (spreading factor 7, 125 kHz bandwidth, coding rate 4/5)
to see packets arrive here.

The messages sent by `lora-tx` are decoded (see `src/message.rs`), and a gap in
their indexes is counted as lost messages, assuming a single transmitter:

    Received 16 bytes (RSSI -42 dBm, SNR 9 dB): [41, 52, 01, 8d, b9, 50, 0e, 0c, 00, 00, 00, b8, ea, 00, 00, 0e]
      message 12 from node 0xe50b98d, sent 60088 ms after boot at 14 dBm
      12 received, 1 lost (92% delivered)

Other LoRa packets are only logged.

Adjust `LORA_FREQUENCY_IN_HZ` in `src/main.rs` so it matches the transmitter and
is legal in your region.

## How to run

In this directory, run

    laze build -b st-b-l072z-lrwan1 run
