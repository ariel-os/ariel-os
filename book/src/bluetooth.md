# Bluetooth Low Energy (BLE)

Ariel OS supports BLE through [TrouBLE][trouble-github].
The hardware-specific initialization is handled by Ariel OS.

## Enabling the BLE Stack

BLE is enabled by selecting one or both of the BLE [laze modules][laze-modules-book] depending on your use case:

- `ble-peripheral`: peripheral BLE role, allows for advertising the device's presence.
- `ble-central`: central BLE role, allows for scanning and creating connections to devices.

If you want to learn more about BLE concepts, you can read the [TrouBLE documentation][trouble-doc].

## Configuring the BLE Stack

The ability to configure which Bluetooth address is used and other capacity parameters like the MTU is planned in future updates.

> [!IMPORTANT]
> For compatibility reasons the MTU is fixed at 27 bytes.
>
> The device address is randomly generated at boot and may be periodically rotated.
>
> Current implementation: the address is a static device address and is not rotated during execution.
> This allows to use the BLE feature of Ariel OS on multiple devices in the same location.
> We later plan to switch to private device addresses by default, which *are* rotated during execution.

## Using the BLE Stack

To use BLE in your application, you need to get the pre-configured stack and build the host instance from it:

```rust
let stack = ariel_os::ble::ble_stack().await;
let host = stack.build();
```

Here `stack` is of type [`trouble_host::Stack`][trouble-host-stack] and `host` is of type [`trouble_host::Host`][trouble-host-host].

You then need to run the runner background task in your application.

```rust
host.runner.run().await
```

> [!NOTE]
> The runner has other options like `run_with_handler(<handler>)` that you may want to use (when scanning for example).

This function will return a result only when a critical error happened in the BLE stack. For most use cases you will need to execute code alongside the runner in the same task (that accesses elements of the `stack` or `host`), you can do that using `embassy_futures::join::join`:

```rust
embassy_futures::join::join(host.runner.run(), async {
  // Execute BLE-related code here that uses `host` or `stack`
}).await;
```

See [the Ariel OS examples](https://github.com/ariel-os/ariel-os/tree/main/examples) for more details.

[laze-modules-book]: ./build-system.md#laze-modules
[trouble-github]: https://github.com/embassy-rs/trouble
[trouble-doc]: https://embassy.dev/trouble/#_concepts
[trouble-host-stack]: https://docs.embassy.dev/trouble-host/0.1.0/default/struct.Stack.html
[trouble-host-host]: https://docs.embassy.dev/trouble-host/0.1.0/default/struct.Host.html
