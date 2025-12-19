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

[native-builder-support]: ./boards/native.html
[laze-builders-book]: ./build-system.md#laze-builders
[laze-tasks-book]: ./build-system.md#laze-tasks
[multithreading-book]: ./multithreading.md
