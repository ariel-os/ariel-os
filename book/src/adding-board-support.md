# Adding Support for a Board

This document serves as a guide as to what is currently needed for adding support
for a board/device to Ariel OS.

Feel free to report anything that is unclear or missing!

> [!NOTE]
> This guide requires working on your own copy of Ariel OS.
> You may want to fork the repository to easily upstream your changes later.

> [!IMPORTANT]
> Unless documented in the User Guide, please expect the module and context names that are defined in the `laze-project.yml` file to change.
> We're still figuring out a proper naming scheme.
> You've been warned.

## Adding Support for a Board

Ariel OS uses [sbd][sbd] (Structured Board Description) files to describe boards.

- Ensure that the HAL [is supported in `ariel-os-hal`](#adding-support-for-an-embassy-halmcu-family).
- Ensure that the chip [is supported](#adding-support-for-an-mcu-from-a-supported-mcu-family).
- Create a new board description file `boards/<your-board-name>.yaml`.
  - It is usually best to copy and adapt an existing one.
  - `chip`: The board's chip, needs to correspond to an existing laze context in `laze-project.yml`.
- In `doc/support_matrix.yml`:
  - Add an entry under `boards`. Include a link to a `web.archive.org` snapshot that describes the board.
  - Update the generated support pages from the book using `laze build update-book`.
- Some MCU families need extra steps, see [Extra steps for some MCU families](#extra-steps-for-some-mcu-families).

```yaml
# boards/<your-board-name>.yaml
boards:
  st-nucleo-f401re:
    chip: stm32f401re
    # Generally the board description is supposed to be OS agnostic.
    # In order to be useful, we allow OS specific configuration in subtrees.
    # Ariel OS specific configuration is in the `ariel` subtree.
    # It contains e.g., the choice of SWI interrupt used for the embassy interrupt executor,
    # which is needed to be set on e.g., stm32 MCUs.
    ariel:
      swi: USART2
    leds:
      led0:
        pin: PB5
        color: green
        active: high
    buttons:
      button0:
        pin: PC13
        active: high
```

With the board description file in place, regenerate the `ariel-os-boards` crate.
To do that, install [`sbd-gen`][sbd] with `cargo install sbd-gen`, then run the following command from the `ariel os` repository root:

```sh
sbd-gen generate-ariel boards -o src/ariel-os-boards --mode update
```

> [!TIP]
> To build every example and test for a board the following command can be used (as this is only for compilation the credentials do not need to be valid):
>
> ```sh
> CONFIG_WIFI_NETWORK='test' CONFIG_WIFI_PASSWORD='password' laze build --global -b <builder>
> ```

## Extra Steps for Some MCU Families

### `stm32`

- STM32 chips do not have a dedicated SWI, so you need to choose one. Select any unused interrupt, like one of the UARTs, and set the `boards.<board_name>.ariel.swi` field in the board description.
- Each STM32 MCU needs an entry for configuring the clock config, in `src/ariel-os-stm32/src/lib.rs` `rcc_config()`.

### `esp32`

- Some ancillary esp-hal crates require a chip-specific feature to be enabled. You will need to add a device-specific dependency section to `ariel-os-debug`, and `ariel-os-esp`, similar to the existing ones.

## Adding Support for an MCU from a Supported MCU family

- In `laze-project.yml`:
  - Add a context for the MCU (if it does not already exist).
    - `parent`: The closest Embassy HAL's context.
    - `selects`: A [rustc-target](#adding-support-for-a-processor-architecture) module or one of the `cortex-m*` modules if applicable.
- In `doc/support_matrix.yml`:
  - Add an entry under `chips`, with the laze context and supported features.
  - Update the generated support pages from the book using `laze build update-book`.

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
[ci-builder-lists]: https://github.com/ariel-os/ariel-os/blob/5c23c04cd51c5cfebd561c9d27a47d4e26cdb649/.github/workflows/build.yml#L147-L168
