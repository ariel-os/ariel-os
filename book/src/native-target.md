# Native Target

The native builder allows to run Ariel OS as an OS process.
This is especially useful for testing and for simulation purposes.

## Running on Native

The [`native`][native-builder-support] [laze builder][laze-builders-book] is used to [compile and run][laze-tasks-book] for native:

```sh
laze build -b native run
```

## Supported Platforms

Currently only GNU/Linux on x86-64 is supported.

> [!NOTE]
> Support for other platforms will be added later.

## Supported Functionalities

See [the support info of `native`][native-builder-support] for details.

[native-builder-support]: ./boards/native.html
[laze-builders-book]: ./build-system.md#laze-builders
[laze-tasks-book]: ./build-system.md#laze-tasks
