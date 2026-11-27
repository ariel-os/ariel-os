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

### Picking the Board's Name

The conventions to follow for the board name are based on [RIOT OS's RDM0003][riot-os-rdm0003].
In particular:

- When the manufacturer is clearly identified, the board name must be prefixed with the manufacturer name.
- The board name should generally follow the manufacturer's name for the board: spaces are replaced with dashes.
- Variant and version markers relevant to software should be included: in particular, differences in memory sizes should appear in the board name.
  Each variant and version therefore results in a different board name and [SBD file](#creating-an-sbd-file).

### Checklist

- Ensure that the HAL [is supported in `ariel-os-hal`](#adding-support-for-an-embassy-halmcu-family).
- Ensure that the chip [is supported](#adding-support-for-an-mcu-from-a-supported-mcu-family).
- [Create a new SBD file](#creating-an-sbd-file) `boards/<your-board-name>.yaml`.
- In `doc/support_matrix.yml`:
  - Add an entry under `builders`.
  - Add an entry under `boards`, that references that new builder.
    Include a link to a `web.archive.org` snapshot that describes the board.
  - Update the generated support pages from the book using `laze build update-book`.
- Some MCU families need extra steps, see [Extra steps for some MCU families](#extra-steps-for-some-mcu-families).

> [!TIP]
> To build every example and test for a board the following command can be used (as this is only for compilation the credentials do not need to be valid):
>
> ```sh
> CONFIG_WIFI_NETWORK='test' CONFIG_WIFI_PASSWORD='password' laze build --global -b <builder>
> ```

### Creating an SBD file

Ariel OS uses [SBD files][sbd-book] to describe boards.
Each file corresponds to a physical board.
Boards having multiple MCUs, or MCUs with multiple compilation targets, still require only a single file, but multiple `targets` entries are used to describe the different compilation targets.
When adding support for a new board, it is usually best to copy and adapt an existing one instead of starting from scratch.

When writing an SBD file to be added in tree, the following guidelines should be followed for each key:

- `chip`: The board's chip, needs to correspond to an existing laze context listed in `boards/ariel-chips.yaml`.
- `i2c`: I2C buses present on the board.
    To qualify, there should be at least either:

    - I2C nodes on the board,
    - A pair of pull-up resistors strong enough for I2C operation on the board (even if the lines are only exposed on connectors without other nodes on the board),
    - A connector whose pins are dedicated to I2C (e.g., Grove, STEMMA QT, Qwiic, Arduino Modulino, mikroBUS),
    - A well-known connector/header/pinout that conventionally includes I2C pins (e.g., Arduino headers, Seeed Studio XIAO pinout).

    Some aliases should use the following conventions:

    - `ArduinoI2c`: The I2C bus is exposed on Arduino headers.
    - `QwiicI2c`: The I2C bus is exposed on a Qwiic connector.
- `uarts`: UART RX/TX present on the board.
  Currently only host-facing UARTs should be added.
  Host-facing UARTs are UARTs exposed through a USB ⟷ UART adapter present on the board (or an interface MCU or equivalent acting as such adapter).

With the SBD file in place, regenerate the `ariel-os-boards` crate.
To do that, install [`sbd-gen`][sbd] with `cargo install sbd-gen`, then run the following command from the `ariel os` repository root:

```sh
sbd-gen generate-ariel boards -o src/ariel-os-boards --mode update
```

## Extra Steps for Some MCU Families

### `stm32`

- STM32 chips do not have a dedicated SWI, so you need to choose one. Select any unused interrupt, typically that of the WWDG, and set the `targets.<board_name>.ariel.swi` field in the SBD file.
- Each STM32 MCU needs an entry for configuring the clock config, in `src/ariel-os-stm32/src/rcc.rs`'s `default()`.

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
- In `boards/ariel-chips.yaml`:
  - Add the new laze context name to the list of `chips`.

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
[sbd-book]: ./structured-board-descriptions.md
[ci-builder-lists]: https://github.com/ariel-os/ariel-os/blob/5c23c04cd51c5cfebd561c9d27a47d4e26cdb649/.github/workflows/build.yml#L147-L168
[i2c-pull-up-resistors-book]: ./i2c.md#configuring-other-settings
[riot-os-rdm0003]: https://github.com/RIOT-OS/RIOT/blob/066ffa11188cdc9ced3095d4f5c8b8dfe481d926/doc/memos/rdm0003.md
