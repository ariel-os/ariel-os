# Introduction

Ariel OS is an operating system for secure, memory-safe, low-power Internet of Things (IoT).
Ariel OS is based on Rust from the ground up.
Targeted hardware includes various hardware based on 32-bit microcontroller architectures (such as Cortex-M, RISC-V, Xtensa).
The targeted **memory footprint** (RAM and flash memory) is measured in hundreds of kilobytes.
The targeted **power consumption** levels enable applications lasting 1+ years on a small battery.

Ariel OS  builds on top of existing projects in the Embedded Rust ecosystem, including
[Embassy](https://github.com/embassy-rs/embassy), [esp-hal](https://github.com/esp-rs/esp-hal),
[defmt](https://github.com/knurling-rs/defmt), [probe-rs](https://github.com/probe-rs/probe-rs),
[sequential-storage](https://github.com/tweedegolf/sequential-storage), and
[embedded-test](https://github.com/probe-rs/embedded-test) among others.

Ariel OS follows an approach whereby it simultaneously integrates a curated ecosystem of libraries (available via crates.io),
and adds missing operating system functionalities as depicted below.
Such functionalities include for instance a preemptive multicore scheduler, portable peripheral APIs,
additional network security facilities, as well as a meta-build system to bind it all together.

As a result, a low-power IoT developer can focus on business logic
sitting on top of APIs which remain close to the hardware but
nevertheless stay the same across all supported hardware,
inspired by what [RIOT](https://github.com/RIOT-OS/RIOT/) tends to in that regard.
The intent is three-fold: decrease application development time,
increase code portability, and decrease core system vulnerabilities.

In a nutshell: Ariel OS contributes to the global effort aiming to (re)write IoT system software
foundations on more solid ground than what traditional building blocks written in C can provide.
And this is a joyful and welcoming open-source community, so: [join us](https://github.com/ariel-os/ariel-os)!

<p style="text-align: center; margin: 4em">
  <img src="figures/ariel-os-arch-diagram2.svg" alt="Architecture diagram">
</p>

### Further Reading

As Ariel OS builds on top of the embedded Rust ecosystem, readers of this book
could benefit from also reading the [Rust book](https://doc.rust-lang.org/book/),
the [Embedded Rust book](https://docs.rust-embedded.org/book/)
and the [Embassy book](https://embassy.dev/book/).
