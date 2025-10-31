## Bluetooth Low Energy

Ariel os provides an abstraction over [TrouBLE](https://github.com/embassy-rs/trouble). The hardware specific initialization is handled by Ariel OS for supported chips.

### Enabling the BLE stack

BLE is enabled by selecting one or both of the BLE [laze modules][laze-modules-book] depending on your use case:

- `ble-peripheral`: for advertising data, acting as a "sever".
- `ble-central`: for scanning and connecting to peripherals, reading their data.

If you want to learn more about BLE concepts, you can read the [TrouBLE documentation](https://embassy.dev/trouble/#_concepts).

### Optional configuration

There are some settings exposed via environment variables that you may want to change depending on your use case:

- `CONFIG_BLE_MTU`: Maximum Transmission Unit (unit: bytes), the default is 27 for better compatibility, newer versions of BLE allow for up to 251.
- `CONFIG_BLE_MAX_CONNS`: Maximum number of concurrent connections to be handled by the BLE stack, the default is 1 to reduce memory usage. If you need to connect to multiple devices or serve multiple devices, increase this value.
- `CONFIG_BLE_MAX_CHANNELS`: Maximum number of concurrent channels to be handled by the BLE stack, the default is 1 to reduce memory usage. Increase this value if you want to use multiple channels. GATT services use only one channel.

To set a static BLE address you need to select the `ble-config-static-mac` [laze module][laze-modules-book] and set your wanted address using the `CONFIG_BLE_STATIC_MAC` environment variable. The expected format is a hexadecimal representation of the 6 bytes address, colon separator is optional. Example: `CONFIG_BLE_STATIC_MAC=02:aa:aa:aa:aa:aa`.

### Using the BLE stack

To use BLE in your application, you need to get the pre-configured stack and build it:

```rust
let stack = ariel_os::ble::ble_stack().await;
let host = stack.build();
```

You then need to run the runner background task forever in your application.

```rust
host.runner.run().await;
```

> Note: the runner has other options like `run_with_handler(&lt;handler&gt;) that you may want to use (when scanning for example).

This function will return a result only when a critical error happened in the BLE stack. For most use cases you want to execute code alongside the runner, you can do that using `embassy_futures::join::join`:

```rust
embassy_futures::join::join(runner.run(), async {
  // execute BLE-related tasks here
}).await;
```

See [examples](https://github.com/ariel-os/ariel-os/tree/main/examples) for more details.

[laze-modules-book]: ./build-system.md#laze-modules
