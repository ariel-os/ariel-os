# Persistent Storage

Ariel OS supports persistent storage on a number of boards.
It provides a key–value pair store based on [sequential-storage].

This storage module is responsible for setting up the flash storage,
and provides the functions to interact with the key–value pair store in the storage.

## Use Cases

Persistent storage can be used to store configuration and other variables
across reboots and power interruptions on the microcontroller.

Both primitives and compound types can be stored.

## Usage

### Enabling the Storage Module

The storage module can be enabled by [selecting the `sw/storage` laze module][laze-modules-book],
which enables the `storage` Cargo feature.

### Using the Storage Module

The [storage module] provides the necessary getters and setters
to interact with the storage backend.

Keys are always `&str` typed.
Values are required to implement the [`serde::Serialize`][serde-serialize]
and [`serde::Deserialize`][serde-deserialize] traits.
Under the hood, currently the values are serialized using [postcard].

Care must be taken to always read a key with the same value type
that it was written before.
While using a different value type for reading than for writing is never unsafe,
it might result in bogus data.

See the [example][storage-example-repo] for details on the usage.

### Durability and Corruption

The underlying [sequential-storage] crate guarantees that the storage can be repaired
after power failure during operations.
Corruption may lead to unrecoverable data that cannot be repaired.
The repair will make sure that the flash state is recovered,
so that any next operation should succeed.

## Flash Requirements

The storage module requires at least two flash pages.
The effective storage space available is `(N - 1) * PAGE_SIZE`,
where `N` is the number of flash pages allocated.

> Currently storage is only supported on flash whose pages have a uniform size.

These pages are allocated by Ariel OS after the `rodata` section in the flash
when the module is enabled.

> Updating the firmware can move and invalidate the storage pages
  when the firmware size differs from the previous version.

NOR flash has limited endurance.
When writing applications using the storage module,
care must be taken to limit the writes to a reasonable amount,
especially at startup when there is the danger of endless writing
due to a crash leading to a reboot.

[sequential-storage]: https://crates.io/crates/sequential-storage
[laze-modules-book]: ./build-system.md#laze-modules
[storage-example-repo]: https://github.com/ariel-os/ariel-os/tree/main/examples/storage
[storage module]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/storage/index.html
[serde-serialize]: https://docs.rs/serde/latest/serde/trait.Serialize.html
[serde-deserialize]: https://docs.rs/serde/latest/serde/trait.Deserialize.html
[postcard]: https://github.com/jamesmunns/postcard
