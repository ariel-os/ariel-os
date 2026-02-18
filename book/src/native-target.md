# Native Target

The native target allows to run Ariel OS as an OS process.
This is especially useful for experimenting without a physical board, testing applications, and for simulation purposes.

## Running on Native

The [`native`][native-builder-support] [laze builder][laze-builders-book] is used to [compile and run][laze-tasks-book] for native:

```sh
laze build -b native run
```

## Supported Host Platforms

Currently only GNU/Linux on x86-64 is supported.

> [!NOTE]
> Support for other host platforms will be added later.

## Supported Functionalities

See [the support info of `native`][native-builder-support] for details.

## Multithreading Behavior

Native itself enables [multithreading][multithreading-book], and creates one "virtual core" per Ariel OS thread using host threads.
This means that threads all run in *parallel* from the point of view of Ariel OS and of the application.

## Networking

Applications that set the `network` [laze module]
will automatically select the `tuntap` module, which opens the `tap0` tap device
(or any other name given in the `ARIEL_NATIVE_TUNTAP` environment variable)
and exchanges traffic through there.

If the device has not yet been created, is in use or otherwise inaccessible,
you get this error:

```
thread '<unnamed>' (...) panicked at src/ariel-os-native/src/lib.rs:
Error opening interface tap0: Operation not permitted (os error 1)
```

Setting up a suitable interface depends on your platform and preferred configuration:

* To instruct NetworkManager to create a connection that gets enabled automatically
  and forward traffic from any uplink interface, run:

  ```console
  $ sudo nmcli connection add type tun mode tap owner $(id -u) ifname tap0 con-name tap0 ipv6.method shared ipv4.method shared
  ```

* To create a manually managed device that only persists until the next reboot, run:

  ```console
  $ sudo ip tuntap add dev tap0 user $(id -u) mode tap
  $ sudo ip link set dev tap0 up
  ```

  Note that for provisioning addresses, you may need to run a DHCP server on that interface,
  and depending on your application, you may need to set up routing manually.

* For device-to-device communication between multiple native instances,
  you can create a bridge and attach one tap device per instance to the bridge;
  the setup for that is currently beyond the scope of this documentation.

At the time of writing, the tap implementation is limited to Linux.

## Storage

When the `storage` [laze module] is selected,
the native target opens a file at `flash.bin`
that represents the flash content byte for byte.
That file name can be changed by setting it as the `ARIEL_NATIVE_FLASH_FILE` environment variable.
If the file is not present,
it is created with a default size of 16KiB.
To simulate a different flash size,
provide a file (e.g. with 0x00 bytes) of a more suitable size.

Write and erase sizes are currently fixed (to 1 and 256 byte, respectively).
Mechanisms to influence those may be provided in the future as use cases arise.

[native-builder-support]: ./boards/native.html
[laze-builders-book]: ./build-system.md#laze-builders
[laze-tasks-book]: ./build-system.md#laze-tasks
[multithreading-book]: ./multithreading.md
[laze module]: ./build-system.md#laze-modules
