# Getting started with Ariel OS

This guide is intended to get you started in about 5 minutes.
It explains how to compile and run the `hello-word` example to verify your setup, and how to bootstrap a new application.

<!-- If you modify these instructions, you should update .github/workflows/build-templates.yml to reflect that. -->

## Installing the build prerequisites

1. Install the needed build dependencies.

    **Ubuntu (GNU/Linux)**

   <!-- gcc and curl are only required for espup, but it doesn't hurt to install those here. -->

    ```sh
    apt install git ninja-build pkg-config libudev-dev clang gcc-arm-none-eabi gcc-riscv64-unknown-elf gcc curl make
    ```

    **Arch Linux**

   <!-- lib32-glibc is needed to build for Xtensa -->

    ```sh
    pacman -S git ninja pkgconf clang arm-none-eabi-gcc arm-none-eabi-newlib gcc curl make lib32-glibc
    ```

    For RISC-V chips, the compiler can be installed via the `riscv32-gnu-toolchain-elf-bin` AUR package.

    First, make sure to install the `fakeroot` and `debugedit` packages (part of the `base-devel` metapackage):

    ```sh
    pacman -S fakeroot debugedit
    ```

    `riscv32-gnu-toolchain-elf-bin` can be installed with an AUR helper, such as [yay](https://github.com/Jguer/yay):

    ```sh
    yay -S riscv32-gnu-toolchain-elf-bin
    ```

    Or manually:

    ```sh
    git clone https://aur.archlinux.org/riscv32-gnu-toolchain-elf-bin.git
    cd riscv32-gnu-toolchain-elf-bin
    makepkg -si
    ```

    **macOS (Apple Silicon)**

    ```sh
    brew install git ninja pkg-config llvm gcc curl
    ```

1. Install the Rust installer [rustup](https://rustup.rs/) using the website's instructions.

1. Only for using ESP devices, install [espup](https://github.com/esp-rs/espup) and related tools:

   ```sh
   cargo install espup --locked
   espup install
   cargo install espflash@3.3.0 --locked
   ```

   There is no need to actively source the `~/export-esp.sh` file which `espup` produces:
   Ariel OS's build system will consult that file when building for ESP.

1. Install the build system [laze](https://github.com/kaspar030/laze):

    ```sh
    cargo install laze
    ```

1. Install the debugging and flashing utility [probe-rs](https://github.com/probe-rs/probe-rs):

    ```sh
    cargo install --locked probe-rs-tools
    ```

   You might need to tweak permissions for non-admin access to connected
   devices. Please consult the [probe-rs documentation][probe-rs-udev] for specifics.

1. Clone the [Ariel OS repository][ariel-os-repo] and `cd` into it.

1. Install the Rust targets:

    ```sh
    laze build install-toolchain
    ```

    This invokes rustup to add all easy-to-install targets and components,
    and invokes espup if it is installed.

## Running the `hello-world` example

To check that everything is installed correctly, the `hello-word` example can be compiled and run from the `ariel-os` directory.
The following assumes you have your target board connected to your host computer.

Find the Ariel OS name of your supported board in the [support matrix](./hardware-functionality-support.html).

> [!NOTE]
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

![Terminal screencast of compiling and flashing the hello-world example](./hello-world_render.svg)

> [!TIP]
> If you do not plan on working on Ariel OS *itself*, this repository is not needed anymore and can be deleted.

## Starting an application project from a template repository

Applications are expected to be developed out-of-tree, outside of the `ariel-os` directory.
This is made possible by [laze's `imports`][laze-imports-book] feature.

To start a new application project, you can either clone the [`ariel-os-hello` repository][ariel-os-hello-repo] or, *alternatively*, use one of the [`cargo-generate`][cargo-generate-repo] templates.

### Cloning `ariel-os-hello`

```sh
git clone https://github.com/ariel-os/ariel-os-hello
```

### Using a `cargo-generate` project template

This requires installing [`cargo-generate`][cargo-generate-repo], then a new application project can be created as follows:

```sh
cargo generate --git https://github.com/ariel-os/ariel-os-template --name <new-project-name>
```

### Running the template example

To check your setup, the default application can be compiled and run as follows:

```sh
laze build -b nrf52840dk run
```

> [!IMPORTANT]
> The board name needs to be replaced with your board's.

See the [Build System page](./build-system.md) to learn more about laze and how to work with out-of-tree applications.

[ariel-os-repo]: https://github.com/ariel-os/ariel-os
[ariel-os-hello-repo]: https://github.com/ariel-os/ariel-os-hello
[laze-imports-book]: https://kaspar030.github.io/laze/dev/reference/imports.html
[cargo-generate-repo]: https://github.com/cargo-generate/cargo-generate
[probe-rs-udev]: https://probe.rs/docs/getting-started/probe-setup/#platform-specifics
