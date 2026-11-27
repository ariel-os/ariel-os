# Adding Support for a Board

This document serves as a guide as to what is currently needed for adding support
for a board/device to Ariel OS.

Feel free to report anything that is unclear or missing!

> [!NOTE]
> This guide requires working on your own copy of Ariel OS.
> You may want to fork the repository to easily upstream your changes later.

## Checklist

- Ensure that the HAL [is supported in `ariel-os-hal`][adding-hal-support-book].
- Ensure that the chip [is supported][adding-mcu-support-book].
- [Create a new SBD file](#creating-an-sbd-file) `boards/<your-board-name>.yaml`.
- In `doc/support_matrix.yml`:
  - Add an entry under `builders`.
  - Add an entry under `boards`, that references that new builder.
    Include a link to a `web.archive.org` snapshot that describes the board.
  - Update the generated support pages from the book using `laze build update-book`.

> [!TIP]
> To build every example and test for a board the following command can be used (as this is only for compilation the credentials do not need to be valid):
>
> ```sh
> CONFIG_WIFI_NETWORK='test' CONFIG_WIFI_PASSWORD='password' laze build --global -b <builder>
> ```

## Creating an SBD file

Ariel OS uses [SBD files][sbd-book] to describe boards.
Each file corresponds to a physical board.
Boards having multiple MCUs, or MCUs with multiple compilation targets, still require only a single file, but multiple `targets` entries are used to describe the different compilation targets.
When adding support for a new board, it is usually best to copy and adapt an existing one instead of starting from scratch.

The SBD schema is defined and documented by the [`sbd-gen-schema` package][sbd-gen-schema-docsrs].
When writing an SBD file to be added in tree, the following guidelines should be followed for each key:

- `chip`: The board's chip, needs to correspond to an existing laze context listed in `boards/ariel-chips.yaml`.
- `leds`: The LEDs present on the board, that can be controlled by the MCU.
  Currently, each color channel of a multicolor LED is modeled as an individual LED.
  Addressable LEDs are not part of this key.
- `buttons`: Push-buttons present on the board.
  Push-buttons are typically monostable tactile switches.
- `switches`: Switches present on the board.
  Push-buttons qualify as switches, but it is generally preferable to add them as `buttons`.
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

### Extra Steps for Some MCU Families

#### `stm32`

- STM32 chips do not have a dedicated SWI, so you need to choose one. Select any unused interrupt, typically that of the WWDG, and set the `targets.<board_name>.ariel.swi` field in the SBD file.

[sbd]: https://github.com/ariel-os/sbd
[sbd-gen-schema-docsrs]: https://docs.rs/sbd-gen-schema/latest/sbd_gen_schema/
[sbd-book]: ./structured-board-descriptions.md
[ci-builder-lists]: https://github.com/ariel-os/ariel-os/blob/5c23c04cd51c5cfebd561c9d27a47d4e26cdb649/.github/workflows/build.yml#L147-L168
[i2c-pull-up-resistors-book]: ./i2c.md#configuring-other-settings
[adding-hal-support-book]: ./adding-hal-support.md#adding-support-for-an-embassy-halmcu-family
[adding-mcu-support-book]: ./adding-hal-support.md#adding-support-for-an-mcu-from-a-supported-mcu-family
