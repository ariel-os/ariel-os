# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.2.1] - 2025-06-24

### Fixed

- fix(deps): bump static_cell as it fixed a soundness issue ([#1107](https://github.com/ariel-os/ariel-os/pull/1107))
- fix(deps): disable static cell nightly feature ([#1106](https://github.com/ariel-os/ariel-os/pull/1106))

## [0.2.0] - 2025-05-07

This release allows Ariel OS to be built on stable Rust, and updates
all crates to edition 2024.
Apart from that, it adds support for a couple of new boards. And a lot of
internal polish that is not mentioned here.

### Added

- feat(build): default to `stable` build ([#987](https://github.com/ariel-os/ariel-os/pull/987))
- feat(boards): add support for the ST NUCLEO-F411RE ([#1002](https://github.com/ariel-os/ariel-os/pull/1002))
- feat: Add power management crate & implement reboot function ([#910](https://github.com/ariel-os/ariel-os/pull/910))
- feat(rt): more flexible stacksize configuration ([#786](https://github.com/ariel-os/ariel-os/pull/786))
- feat(stm32): allow the interrupt executor on STM32 ([#871](https://github.com/ariel-os/ariel-os/pull/871))
- feat(network): seed `embassy_net` from the device ID when no RNG ([#873](https://github.com/ariel-os/ariel-os/pull/873))
- feat(coap): support stored security configurations ([#814](https://github.com/ariel-os/ariel-os/pull/814))
- feat(network): Add ethernet from nucleo-144 board family ([#993](https://github.com/ariel-os/ariel-os/pull/993))
- feat(boards): add support for the SMT32U083C-DK ([#986](https://github.com/ariel-os/ariel-os/pull/986))
- feat(boards): add support for the FireBeetle 2 ESP32-C6 ([#983](https://github.com/ariel-os/ariel-os/pull/983))
- feat(boards): add initial support for Espressif ESP32-C3-LCDkit ([#477](https://github.com/ariel-os/ariel-os/pull/477))
- feat(boards): add support for the Nordic Thingy:91 X ([#974](https://github.com/ariel-os/ariel-os/pull/974))
- feat(boards): add support for the Raspberry Pi Pico 2 W ([#943](https://github.com/ariel-os/ariel-os/pull/943))
- feat(nrf): add basic support for nRF9160 ([#926](https://github.com/ariel-os/ariel-os/pull/926))
- feat(board): add support for the ST-NUCLEO-C031C6 board  ([#838](https://github.com/ariel-os/ariel-os/pull/838))

### Changed

- refactor(stm32)!: remove unneeded info from laze context names  ([#961](https://github.com/ariel-os/ariel-os/pull/961))
- chore(build): re-enable sccache ([#970](https://github.com/ariel-os/ariel-os/pull/970))
- fix(task-macro): avoid the need for importing `UsbBuilderHook` ([#918](https://github.com/ariel-os/ariel-os/pull/918))
- perf(storage): block on storage init to spare RAM ([#931](https://github.com/ariel-os/ariel-os/pull/931))
- build: enable Rust edition 2024 ([#584](https://github.com/ariel-os/ariel-os/pull/584))

### Fixed

- fix(log): add support for `log` on architectures without atomics ([#990](https://github.com/ariel-os/ariel-os/pull/990))

## [0.1.0] - 2025-02-25

<!-- next-url -->
[Unreleased]: https://github.com/ariel-os/ariel-os/compare/ariel-os-v0.2.1...HEAD
[0.2.1]: https://github.com/ariel-os/ariel-os/compare/ariel-os-v0.2.0...ariel-os-v0.2.1
[0.2.0]: https://github.com/ariel-os/ariel-os/compare/v0.1.0...ariel-os-v0.2.0
[0.1.0]: https://github.com/ariel-os/ariel-os/releases/tag/v0.1.0
