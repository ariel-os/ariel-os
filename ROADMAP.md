<p align="center"> <img src="book/src/figures/ariel-hexacube-orange-rounded.svg" alt="logo" height="300"/> </p>

## Why do we do this?

We expect a revolution concerning distributed system software: much more robust building blocks will soon have to take over as the basis for the systems we increasingly depend upon.
This also applies to the tiniest hardware elements of distributed systems: energy-efficient microcontrollers.

## How do we approach this?

We believe open-source embedded Rust for safe software and open standards for secure low-power communication is the winning combination.
Concerning the latter, various standardization bodies including the IETF provide the basis for user empowerment end-to-end.
Concerning the former, the embedded Rust ecosystem is alive and kicking.
Its diversity is however a challenge. We can alleviate this diversity by combining real-time synchronous programming and asynchronous Rust (building on top of the Embassy framework), wrapped in efficient subsystem concepts providing the Rust embedded operating system everyone is waiting for in this field.

## What is Ariel OS?

Ariel OS is a (very) low memory footprint embedded software platform written in Rust from the ground up, providing built-in energy efficiency, async Rust and real-time capabilities (preemptive scheduling).
Ariel OS is designed for shortened development life-cycles, using modern tooling and leveraging the embedded Rust ecosystem.
Ariel OS is developed and maintained by a non-profit community of open-source developers organized around principles that are inspired by the Linux community and the IETF community among others.

## What's next for Ariel OS?

As for any relevant OS, Ariel is permanently worked on, by many people, and at various levels.
The current tip of the iceberg? See the day-to-day [delta](https://github.com/orgs/ariel-os/projects/1) and the current [feature planning](https://github.com/orgs/ariel-os/projects/3).
Beyond that, the following summarizes our roadmap at high level.

We are extending hardware support to not only upcoming 32-bit hardware, but also to a native target enabling development/debugging directly on your PC, leveraging low-cost, highly-portable abstractions.
We are working on improving the integration of our build system extensions with Cargo.
We are developing the building blocks for modern, secure DevOps and over-the-air software updates working out-of-the-box on most boards.
We are working on extending support for additional network link-layers (BLE, 802.15.4, LoRa) and for additional open standard network protocols (MQTT, DTLS...) to facilitate integration in various backends (E.g. Home Assistant).
We are working towards secure on-boarding mechanisms for device commissioning.
We are also working on extending our CI infrastructure with hardware-in-the-loop automation and formal verification checks for selected software modules.
