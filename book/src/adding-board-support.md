# Adding Support for a Board

This document serves as a guide as to what is currently needed for adding support
for a board/device to Ariel OS.

Feel free to report anything that is unclear or missing!

> This guide requires working on your own copy of Ariel OS.
> You may want to fork the repository to easily upstream your changes later.

> Unless documented in the User Guide, please expect the module and context names that are defined in the `laze-project.yml` file to change.
> We're still figuring out a proper naming scheme.
> You've been warned.

## Adding Support for a Board

The more similar a board is to one that is already supported, the easier.
It is usually best to copy and adapt an existing one.

- Ensure that the HAL [is supported in `ariel-os-hal`](#adding-support-for-an-embassy-halmcu-family).
- In `laze-project.yml`:
  - `parent`: The MCU's laze context.
  - If the MCU does not have a dedicated software interrupt (SWI), choose one
    now and set the `CONFIG_SWI` environment variable.
  - Ensure there is a way to flash the board:
    - If the MCU is supported by probe-rs, specify `PROBE_RS_CHIP`
      and `PROBE_RS_PROTOCOL`.
      `PROBE_RS_PROTOCOL` can be omitted to inherit the default value from the `ariel-os` laze context.
    - If the board is based on `esp`, it should inherit the espflash support.
    - If neither of these are supported, please open an issue.
  - Add a builder for the actual board that uses the context from above as `parent`.

Whether to add an intermediate context or just a builder depends on whether the
MCU-specific code can be re-used.

Example for the `st-nucleo-f401re` board:

```yaml
builders:
  # ...
  - name: st-nucleo-f401re
    parent: stm32f401re
    provides:
      - has_swi
    env:
      CARGO_ENV:
        - CONFIG_SWI=USART2
```

> To build every example and test for a board the following command can be used (as this is only for compilation the credentials do not need to be valid):
>
> ```sh
> CONFIG_WIFI_NETWORK='test' CONFIG_WIFI_PASSWORD='password' laze build --global -b <builder>
> ```

## Adding Support for an MCU from a Supported MCU family

- In `laze-project.yml`:
  - Add a context for the MCU (if it does not already exist).
    - `parent`: The closest Embassy HAL's context.
    - `selects`: A [rustc-target](#adding-support-for-a-processor-architecture) module or one of the `cortex-m*` modules if applicable.

MCU-specific items inside Ariel OS crates are gated behind
`#[cfg(context = $CONTEXT)]` attributes, where `$CONTEXT` is the [MCU's `laze
context` name](./build-system.md#laze-contexts).
These need to be expanded for adding support for the new MCU.

At least the following crates may need to be updated:

- The Ariel OS HAL crate for the MCU family.
- `ariel-os-storage`
- `ariel-os-embassy`

Example for the `stm32f401re` MCU:

```yaml
contexts:
  # ...
  - name: stm32f401re
    parent: stm32
    selects:
      - cortex-m4f
    env:
      PROBE_RS_CHIP: STM32F401RE
```

## Adding Support for an Embassy HAL/MCU family

As of this writing, Ariel OS supports most HALs that Embassy supports,
including `esp-hal`, `nrf`, `rp`, and `stm32`, but excluding `std` and `wasm`.

The steps to add support for another Embassy supported HAL are:

- `src/ariel-os-hal`:
  - `Cargo.toml`: Add a dependency on the Embassy HAL crate.
  - `src/lib.rs`: Add the Ariel OS HAL to the dispatch logic.
- Create a new Ariel OS HAL crate (similar to `ariel-os-nrf`).

## Adding Support for a Processor Architecture

Each rustc target needs its own module in `laze-project.yml`.
If the processor architecture that is being added is not listed yet, you will
need to take care of that.

Example:

```yaml
modules:
  # ...
  - name: thumbv6m-none-eabi
    env:
      global:
        RUSTC_TARGET: thumbv6m-none-eabi
        CARGO_TARGET_PREFIX: CARGO_TARGET_THUMBV6M_NONE_EABI
        RUSTFLAGS:
          - --cfg armv6m
```

The variables `RUSTC_TARGET` and `CARGO_TARGET_PREFIX` need to be adjusted.
Add `--cfg $HAL` as needed.

Chances are that if you need to add this, you will also have to add support for
the processor architecture to `ariel-os-bench`, `ariel-os-rt`, `ariel-os-threads`.
