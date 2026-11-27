# Adding Support for Microcontrollers

> [!IMPORTANT]
> Unless documented in the User Guide, please expect the module and context names that are defined in the `laze-project.yml` file to change.
> We're still figuring out a proper naming scheme.

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

### Extra Steps for Some MCU Families

### `stm32`

- Each STM32 MCU needs an entry for configuring the clock config, in `src/ariel-os-stm32/src/rcc.rs`'s `default()`.

### `esp32`

- Some ancillary esp-hal crates require a chip-specific feature to be enabled. You will need to add a device-specific dependency section to `ariel-os-debug`, and `ariel-os-esp`, similar to the existing ones.

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
