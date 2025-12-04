# [Ferrocene]

> [!NOTE]
> Ferrocene is the open-source qualified Rust compiler toolchain for safety- and mission-critical. Qualified for automotive, industrial and medical development.

> [!NOTE]
> Ferrocene requires a (paid) license to use.

Ferrocene uses [`criticalup`][criticalup], its variant of `rustup`, to manage installing its toolchain and components. Once installed, wrapping regular Rust commands like `cargo` and `rustc` with `criticalup run` enables using the qualified tooling.

The Ariel OS build system seamlessly integrates this for targets supported by Ferrocene (currently all Cortex-M).

## Installing Ferrocene

Please refer to the official [Ferrocene documentation][ferrocene-docs] and the [`criticalup` User Guide][criticalup] for instructions.

## Using Ferrocene with Ariel OS

To select the Ferrocene toolchain, enable the `ferrocene` laze module.

Example:

    $ laze -Cexamples/hello-world build --builders nrf52830dk --select ferrocene

Alternatively, add `ferrocene` to the laze modules of your application.

[Ferrocene]: https://ferrocene.dev
[ferrocene-docs]: https://public-docs.ferrocene.dev/main/user-manual/index.html
[criticalup]: https://criticalup.ferrocene.dev
