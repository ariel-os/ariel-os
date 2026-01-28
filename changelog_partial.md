- A `tcp-client` example is now available: it makes it easy to test Internet connectivity without requiring a HWRNG. ([#1690](https://github.com/ariel-os/ariel-os/pull/1690))
- (nRF5340) Both the application core and the network core of this MCU are now supported and their usage is documented. ([#1658](https://github.com/ariel-os/ariel-os/pull/1658))
- The concept of laze builder is now explained in the documentation. ([#1619](https://github.com/ariel-os/ariel-os/pull/1619))
- A thermometer example is now available: it demonstrates usage of the new sensor API and, on the STM32U083C-DK, displays the reading on the onboard LCD. ([#1530](https://github.com/ariel-os/ariel-os/pull/1530))
- The custom panic handler is now only provided on embedded architectures. This fixes potential issues when running host tests or generating documentation. ([#1614](https://github.com/ariel-os/ariel-os/pull/1614))
- Using `embassy-time` types (e.g., `Timer`) within threads is now supported. A generic timer queue is used, whose size can be configured using the `generic-timer-queue-*` Cargo features. ([#1555](https://github.com/ariel-os/ariel-os/pull/1555))
- (ESP32-S3) USB is now supported on this MCU. ([#1561](https://github.com/ariel-os/ariel-os/pull/1561))
- The `defmt` Cargo feature is now propagated to `embedded-hal`, `embedded-hal-async`, `embedded-io`, and `embedded-io-async`. ([#1535](https://github.com/ariel-os/ariel-os/pull/1535))
- (nRF5340, nRF9151, nRF9160) The `SERIAL3` peripheral is now dedicated to the UART drivers instead of the SPI drivers. ([#1507](https://github.com/ariel-os/ariel-os/pull/1507))
- The `Debug2Format` and `Display2Format` decorators are now provided for the `log` logging facade as well (on top of `defmt`'s), improving the portability of log statements. ([#1485](https://github.com/ariel-os/ariel-os/pull/1485))
- (ESP32-C3, ESP32-C6) Wi-Fi and multithreading can now be used at the same time. ([#1455](https://github.com/ariel-os/ariel-os/pull/1455))
- The `executor-interrupt` flavor is now using the lowest interrupt priority on STM32 and nRF MCUs. This allows using BLE in combination with that executor flavor on nRF. ([#1168](https://github.com/ariel-os/ariel-os/pull/1168))
- Support for the `getrandom` crate has been added. Applications can now use it directly or through transitive dependencies and the CSPRNG will automatically be seeded appropriately. See [the documentation](https://ariel-os.github.io/ariel-os/dev/docs/book/randomness.html) for details. ([#1416](https://github.com/ariel-os/ariel-os/pull/1416))
- DHCPv4 is now only enabled when the `network-config-ipv4-dhcp` laze module (formerly `network-config-dhcp`) is enabled, instead of always being enabled. This may reduce the size of applications not using DHCP. ([#1378](https://github.com/ariel-os/ariel-os/pull/1378))
- (STM32U083MC) Networking is now supported on this MCU. It was previously disabled because of lack of RAM. ([#1177](https://github.com/ariel-os/ariel-os/pull/1177))
- A `multicast` Cargo feature is now exposed, that enables multicast on the network stack. ([#1336](https://github.com/ariel-os/ariel-os/pull/1336))
- The `ariel_os::time` module now provides a `with_timeout()` function. ([#1329](https://github.com/ariel-os/ariel-os/pull/1329))
- (STM32F4) The flash cache is now enabled. This increases performance on supported STM32F4 MCUs. ([#1201](https://github.com/ariel-os/ariel-os/pull/1201))
- (ESP32-S3) Using GPIO26 to GPIO48 is now supported on this MCU. ([#1210](https://github.com/ariel-os/ariel-os/pull/1210))
- Log statements are now properly filtered based on their log level when using the `log` logger. ([#1152](https://github.com/ariel-os/ariel-os/pull/1152))
- The `#[thread]` attribute macro now allows pinning threads to specific cores using the `affinity` parameter. ([#1134](https://github.com/ariel-os/ariel-os/pull/1134))
- (Cortex-M) Hard floats are now supported on this architecture: applications are now compiled with the `eabihf` variant, and floating point registers are now saved and restored by the preemptive scheduler as necessary. ([#1097](https://github.com/ariel-os/ariel-os/pull/1097))
- A laze task has been added to configure Visual Studio Code and derivatives to work well in an Ariel OS project, you can read about it [in the book](https://ariel-os.github.io/ariel-os/dev/docs/book/vscode-configuration.html). ([#1049](https://github.com/ariel-os/ariel-os/pull/1049))
- (nRF5340) The HWRNG is now supported on the network core, allowing to use the `ariel_os::random` module. ([#1102](https://github.com/ariel-os/ariel-os/pull/1102))
- (RP235x) The `ariel_os::random` module is now seeding its RNGs from the TRNG (which is not available on the RP2040), instead of relying on the `RoscRng`. ([#1077](https://github.com/ariel-os/ariel-os/pull/1077))
- An `i2c-scanner` example is now available: it allows finding connected I2C devices. ([#1071](https://github.com/ariel-os/ariel-os/pull/1071))


// changelog:highlight
- Ariel OS's MSRV is now 1.91. `laze build install-toolchain` can be used to update the toolchains. ([#1720](https://github.com/ariel-os/ariel-os/pull/1720))
- The Embassy and `esp-hal`-related crates have been updated to newer versions. This comes with a few breaking changes:
  
  - The `executor-single-thread` async executor flavor has been removed.
  - TODO ([#1328](https://github.com/ariel-os/ariel-os/pull/1328))
- The hardware support documentation has been revamped: chips and boards now have dedicated pages, and board pages list laze builders that can be used. ([#1574](https://github.com/ariel-os/ariel-os/pull/1574))
- Support for IPv6 has been added, which can be used alongside IPv4 or in place of it. Only static configuration is currently supported. See [the networking documentation](https://ariel-os.github.io/ariel-os/dev/docs/book/networking.html) for details. ([#1377](https://github.com/ariel-os/ariel-os/pull/1377))
- A native target has been introduced: it allows running an application as a Linux process. This is especially useful for experimenting without a physical board, testing applications, and for simulation purposes. A subset of features is currently supported, and will be expanded in the future. ([#1617](https://github.com/ariel-os/ariel-os/pull/1617))
- The value of Cargo's `include` unstable configuration key has been updated to not use the now-unsupported string-only value. Existing applications need to update the value in their `.cargo/config.toml` configuration file to use the array or table types instead. ([#1572](https://github.com/ariel-os/ariel-os/pull/1572))
- Bluetooth Low Energy (BLE) is now supported on the nRF52 and the nRF53 chip families and on the Raspberry Pi Pico W and Pico 2 W boards using the onboard CYW43 chip. Two examples are available for testing: `ble-advertiser` and `ble-scanner`. See [the documentation](https://ariel-os.github.io/ariel-os/dev/docs/book/bluetooth.html) for details. ([#1560](https://github.com/ariel-os/ariel-os/pull/1560))
- A custom sensor abstraction has been introduced: sensor drivers can be written against it, and sensor driver instances can be registered in a sensor registry inside an application. The registry then allows to query sensor driver instances and fetch their readings asynchronously. The `sensors-debug` example is available for testing. See the documentation of `ariel_os::sensors` for details. ([#1313](https://github.com/ariel-os/ariel-os/pull/1313))
- A UART abstraction has been introduced, similar to the I2C and SPI abstractions. Drivers are provided for each currently supported HAL: ESP32, nRF, RP, and STM32. ([#1365](https://github.com/ariel-os/ariel-os/pull/1365))
- Board pin information is now read from declarative files and processed by [`sbd-gen`](https://github.com/ariel-os/sbd). This makes adding support for new boards easier, allows moving pin information out of applications, and these SBD files should be re-usable by other projects. ([#1397](https://github.com/ariel-os/ariel-os/pull/1397))
- The `network-config-dhcp` and `network-config-static` laze modules have been renamed to `network-config-ipv4-dhcp` and `network-config-ipv4-static` respectively. The old names are now deprecated. ([#1348](https://github.com/ariel-os/ariel-os/pull/1348))


// breaking
- The laze contexts and builders targeting the application core of the nRF5340 chip have been renamed. The chip laze context is now named `nrf5340-app` and the laze builder of the nRF5340-DK board targeting the application core is now `nrf5340dk-app`. This is a breaking change for applications relying on the laze context for feature-gating, and when targeting this specific development kit. ([#1699](https://github.com/ariel-os/ariel-os/pull/1699))
- The `ariel_os::asynch::blocker::block_on()` function has been moved into `ariel_os::thread` and is now `ariel_os::thread::block_on()`. ([#1567](https://github.com/ariel-os/ariel-os/pull/1567))
- (ST NUCLEO-WB55) The SWI has been switched from `LPUART1` to `USART1` to free up the interrupt for UART. ([#1457](https://github.com/ariel-os/ariel-os/pull/1457))
- (STM32U083C-DK) The SWI has been switched from `USART2_LPUART2` to `USART4_LPUART3` to free up the interrupt for UART. ([#1456](https://github.com/ariel-os/ariel-os/pull/1456))
- New laze contexts have been introduced for ESP32 chips with in-package flash. No existing `esp32*` chip laze modules have been deleted, but board laze builders have been adjusted to use the new ones when appropriate, which can be breaking if applications were relying on these for feature-gating. ([#1433](https://github.com/ariel-os/ariel-os/pull/1433))
- The documentation of the `ariel_os::time` module has been clarified: its items must only be used in combination with other items from that module, and not be passed as arguments to other crates. ([#1321](https://github.com/ariel-os/ariel-os/pull/1321))
- (ST NUCLEO-WBA55) The SWI has been switched from `LPUART1` to `USART2` to free up the interrupt for UART. ([#1203](https://github.com/ariel-os/ariel-os/pull/1203))
- The `network-config-static` Cargo feature has been removed from the documentation. It should not be used directly. ([#1090](https://github.com/ariel-os/ariel-os/pull/1090))


// changelog:new-hardware
- The Adafruit Feather nRF52840 Express and Sense boards are now supported. ([#1622](https://github.com/ariel-os/ariel-os/pull/1622))
- The Seeed Studio XIAO ESP32C6 board is now supported. ([#1479](https://github.com/ariel-os/ariel-os/pull/1479))
- The STM32H753ZI MCU and the ST NUCLEO-H753ZI board are now supported. ([#1494](https://github.com/ariel-os/ariel-os/pull/1494))
- The Espressif ESP32-C3-DevKit-RUST-1 board is now supported. ([#1466](https://github.com/ariel-os/ariel-os/pull/1466))
- The nRF9151-DK board is now supported. ([#1463](https://github.com/ariel-os/ariel-os/pull/1463))
- The ESP32-S2, ESP32-S2Fx2, ESP32-S2Fx4, ESP32-S2Fx4R2 MCUs, the ESP32-S2-SOLO-2 hardware module, and the Espressif ESP32-S2-DevKitC-1 board are now supported. ([#1465](https://github.com/ariel-os/ariel-os/pull/1465))
- The STM32U073KC MCU is now supported. ([#1183](https://github.com/ariel-os/ariel-os/pull/1183))
- The Seeed Studio LoRa-E5 mini board is now supported. ([#1125](https://github.com/ariel-os/ariel-os/pull/1125))
- The Heltec WiFi LoRa 32 V3 board is now supported. ([#1199](https://github.com/ariel-os/ariel-os/pull/1199))
- The STM32U585AI MCU and the ST STEVAL-MKBOXPRO board are now supported. ([#1117](https://github.com/ariel-os/ariel-os/pull/1117))
- The BBC micro:bit V1 board is now supported. ([#1068](https://github.com/ariel-os/ariel-os/pull/1068))
- The nRF52-DK board is now supported. ([#1066](https://github.com/ariel-os/ariel-os/pull/1066))
- The STM32WBA55CG MCU and the ST NUCLEO-WBA55CG board are now supported. ([#1064](https://github.com/ariel-os/ariel-os/pull/1064))
- The STM32F042K6 MCU and the ST NUCLEO-F042K6 board are now supported. ([#1050](https://github.com/ariel-os/ariel-os/pull/1050))


// changelog:sensor
- A sensor driver for the LPS22DF, compatible with the newly-introduced sensor API, is now available. ([#1418](https://github.com/ariel-os/ariel-os/pull/1418))
- A sensor driver for the LIS2DU12, compatible with the newly-introduced sensor API, is now available. ([#1431](https://github.com/ariel-os/ariel-os/pull/1431))
- A sensor driver for the STTS22H, compatible with the newly-introduced sensor API, is now available. ([#1363](https://github.com/ariel-os/ariel-os/pull/1363))


// changelog:skip

