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

Ariel OS uses [sbd][sbd] (Structured Board Description) files to describe boards.

- Ensure that the HAL [is supported in `ariel-os-hal`](#adding-support-for-an-embassy-halmcu-family).
- Ensure that the chip [is supported](#adding-support-for-an-mcu-from-a-supported-mcu-family) is supported.
- Create a new board description file `boards/<your-board-name>.yaml`.
  - It is usually best to copy and adapt an existing one.
  - `chip`: The board's chip, needs to correspond to an existing laze context in `laze-project.yml`.
- Some MCU families need extra steps, see [Extra steps for some MCU families](#extra-steps-for-some-mcu-families).

```yaml
# boards/<your-board-name>.yaml
boards:
  st-nucleo-f401re:
    chip: stm32f401re
    ariel:
      swi: USART2
    leds:
      led0:
        pin: PB5
        # color: green
        # active: high
    buttons:
      button0:
        pin: PC13
        # active: high
```

With the board description file in place, regenerate `ariel-os-boards`.
Install [`sbd-gen`][sbd] with `cargo install sbd-gen`, then run the following command from the ariel repository root:

```sh
sbd-gen generate-ariel boards -o src/ariel-os-boards --mode update
```

> To build every example and test for a board the following command can be used (as this is only for compilation the credentials do not need to be valid):
>
> ```sh
> CONFIG_WIFI_NETWORK='test' CONFIG_WIFI_PASSWORD='password' laze build --global -b <builder>
> ```

## Extra steps for some MCU families

### `nrf`

- Add an entry with flash total and page sizes in `src/ariel-os-rt/build.rs`.

### `stm32`

- STM32 chips don't have a dedicated SWI, so you need to choose one. Select any unused ISR, like one of the UARTs, and set the `boards.<board_name>.ariel.swi` field in the board description.
- Each STM32 MCU needs an entry for configuring the clock config, in `src/ariel-os-stm32/src/lib.rs` `rcc_config()`.

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

[sbd]: https://github.com/ariel-os/sbd
