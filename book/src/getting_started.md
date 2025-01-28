# Getting started with Ariel OS

## Installing the build prerequisites

1. Install the needed build dependencies.
   On Ubuntu, the following is sufficient:

```sh
apt install build-essential curl git python3 pkg-config \
  libssl-dev llvm-dev cmake libclang-dev gcc-arm-none-eabi \
  clang unzip lld ninja-build
```

1. Install the Rust installer [rustup](https://rustup.rs/) using the website's instructions or through your distribution package manager.

1. Install the build system [laze](https://github.com/kaspar030/laze):

```sh
cargo install laze
```

1. Install the debugging and flashing utility [probe-rs](https://github.com/probe-rs/probe-rs):

```sh
cargo install --locked probe-rs-tools
```

1. Clone the [Ariel OS repository][ariel-os-repo] and `cd` into it.

1. Install the Rust targets:

```sh
laze build install-toolchain
```

## Running the `hello-world` example

To check that everything is installed correctly, the `hello-word` example can compiled and run from the `ariel-os` directory.
The following assumes you have your target board connected to your host computer.

Find the Ariel OS name of your supported board in the [support matrix](./hardware_functionality_support.html).

> The following assumes the Nordic nRF52840-DK, whose Ariel OS name is `nrf52840dk`.
> Replace that name with your board's.

Then, **from the `ariel-os` directory**, compile and run the example, as follows:

```sh
laze -C examples/hello-world build -b nrf52840dk run
```

<details>
    <summary>(This might fail if the flash is locked, click here for unlocking instructions.)</summary>
This might fail due to a locked chip, e.g., on most nRF52840-DK boards that are fresh from the factory.
In that case, the above command throws an error that ends with something like this:

```sh
An operation could not be performed because it lacked the permission to do so: erase_all
```

The chip can be unlocked using this command:

```sh
laze -C examples/hello-world build -b nrf52840dk flash-erase-all
```
</details>

<!-- FIXME: path no accessible when deployed -->
![Terminal screencast of compiling and flashing the hello-world example](../../doc/hello-world_render.svg)

## Starting a project from a template repository

```sh
git clone https://github.com/ariel-os/ariel-os-hello
```

or

```sh
cargo generate ariel-os/ariel-os-template --name <some-name>
```

or

```sh
cargo generate ariel-os/ariel-os-template --name <some-name> --lib
```

FIXME: need to install `cargo-generate`

[ariel-os-repo]: https://github.com/ariel-os/ariel-os
