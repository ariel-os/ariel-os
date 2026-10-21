# ariel-os-bootloader

Bootloader using [embassy-boot](https://github.com/embassy-rs/embassy/tree/embassy-nrf-v0.9.0/embassy-boot).

## Flashing

```
laze -C src/ariel-os-bootloader/ build -b nrf52840dk -d logging run
```

## Bootloader state

Bootloader state is read from the first byte (offset 0) of the BOOTLOADER_STATE section.

Values are (from [embassy-boot](https://github.com/embassy-rs/embassy/blob/4c9a8998805b95d472b5e8137b16588369c2a8b6/embassy-boot/src/lib.rs#L33):

- `REVERT_MAGIC = 0xC0`: Firmware has been reverted because `BOOT_MAGIC` wasn't written by the app
  since last boot.
- `BOOT_MAGIC = 0xD0`: Written by the application to indicate that it successfully booted.
- `SWAP_MAGIC = 0xF0`: Written by the application to ask the bootloader to swap the active partition
  with the dfu partition (applies the update).
- `DFU_DETACH_MAGIC = 0xE0`: Written by the application to ask the bootloader to enter USB DFU mode
  This is currently not implemented in `ariel-os-bootloader`.
