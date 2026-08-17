# ATmega328P

## Overview

The ATmega328P is an 8-bit AVR RISC microcontroller from Microchip Technology (formerly Atmel). It is the MCU used on the Arduino Uno board.

## Specifications

| Specification | Value |
|---|---|
| Architecture | 8-bit AVR RISC |
| Flash Memory | 32 KB |
| SRAM | 2 KB |
| EEPROM | 1 KB |
| Clock Speed | 16 MHz (typical, Arduino Uno) |
| Package | TQFP-32 / QFN-32 |
| Core | AVR |

## Ariel OS Support

### Architecture

The ATmega328P uses the `avr-none` Rust target. It requires a nightly Rust toolchain and `-Zbuild-std=core,alloc`.

### Runtime

Due to the 2 KB SRAM constraint, the standard Ariel OS threading runtime (`ariel-os-threads`) is **not used**. Instead, AVR uses Embassy's **thread-mode executor** directly via `executor-thread` feature (which has been decoupled from `threading`).

### Time Driver

The `embassy-avr` crate provides a time driver using Timer0 in CTC mode at 1 kHz tick rate.

### Key Crate Dependencies

- `avr-device`: Provides register access and interrupt vector definitions
- `avr-hal` (via `embassy-avr`): Provides GPIO, UART, I2C, SPI HAL abstractions
- `embassy-executor` with `arch-avr` feature: Thread-mode executor for AVR

### Memory Constraints

| Resource | Limit |
|---|---|
| Total RAM (.data + .bss) | < 2048 bytes (2 KB) |
| Stack size | 512 bytes (via `ram-tiny` laze module) |
| Flash | < 32 KB |
| Timer queue | 8 entries (via `timer-generic-queue-8` laze module) |

### Known Limitations

- No `linkme::distributed_slice` support (exceeds 2 KB SRAM)
- No dynamic threading or `ariel-os-threads` support
- No defmt/RTT support (logging via UART only)
- No hardware USB (not available on this chip)
- No hardware random number generator
- No persistent storage driver (EEPROM not yet implemented)
- `alloc` requires careful use due to 2 KB SRAM limit

### Build Flags

```bash
RUSTFLAGS='-C target-cpu=atmega328p --cfg context="avr" --cfg context="atmega328p"'
CARGO_ARGS='-Zbuild-std=core,alloc'
```
