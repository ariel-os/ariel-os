# Bluetooth Low Energy (BLE)

Ariel OS supports BLE through [TrouBLE][trouble-github].
The hardware-specific initialization is handled by Ariel OS.

## Enabling the BLE Stack

BLE is enabled by selecting one or both of the BLE [laze modules][laze-modules-book] depending on your use case:

- `ble-peripheral`: peripheral BLE role, allows for advertising the device's presence.
- `ble-central`: central BLE role, allows for scanning and creating connections to devices.

If you want to learn more about BLE concepts, you can read the [TrouBLE documentation][trouble-doc].

## Configuring the BLE Stack

The ability to use a private address and other capacity parameters like the MTU is planned in future updates.

> [!IMPORTANT]
> For compatibility reasons the MTU is fixed at 27 bytes.
>
> By default the device address is randomly generated at boot and may be periodically rotated.
>
> Current implementation: the address is a static device address and is not rotated during execution.
> This allows to use the BLE feature of Ariel OS on multiple devices in the same location.
> We later plan to switch to private device addresses by default, which *are* rotated during execution.

### Using a Static Address

> [!IMPORTANT]
> It is not recommended to have personal devices advertise a static (and constant) address as this can easily be tracked and compromise the privacy of the user.
>
> The use case of advertising a static address is for static devices like BLE beacons.

To set an address that will persist across reboots, use the `ble-config-static-address` [laze module][laze-modules-book] and set the address using the `CONFIG_BLE_STATIC_ADDRESS` environment variable, the expected format is `XX:XX:XX:XX:XX:XX` where X is a hex digit.

> [!NOTE]
> The address will be advertised as a [random static address](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-7edea27a-a47f-8436-4bd7-aedc1945c366_figure-idm4497995733171233616486354268) and the two most significant bits of the address will therefore be set to 1.

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

To obtain the address at runtime, use [`ariel_os::ble::current_address()`](https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/ble/fn.current_address.html).

See [the Ariel OS examples](https://github.com/ariel-os/ariel-os/tree/main/examples) for more details.

[laze-modules-book]: ./build-system.md#laze-modules
[trouble-github]: https://github.com/embassy-rs/trouble
[trouble-doc]: https://embassy.dev/trouble/#_concepts
[trouble-host-stack]: https://docs.embassy.dev/trouble-host/0.1.0/default/struct.Stack.html
[trouble-host-host]: https://docs.embassy.dev/trouble-host/0.1.0/default/struct.Host.html
