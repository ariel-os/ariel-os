# Building an Application

This chapter covers fundamental concepts required to build an Ariel OS application.

## Obtaining Peripheral Access

[Embassy-style HALs][embassy-style-hals] define a type for each MCU peripheral, which needs to be provided to the driver of that peripheral.
These peripheral types, which we call *Embassy-style peripherals* or *peripheral ZSTs*, are [Zero Sized Types](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts) (ZSTs) that are used to statically enforce exclusive access to a peripheral.
These ZSTs indeed are by design neither [`Copy`](https://doc.rust-lang.org/stable/std/marker/trait.Copy.html) nor [`Clone`](https://doc.rust-lang.org/stable/std/clone/trait.Clone.html), making it impossible to duplicate them; they can only be *move*d around.

Drivers therefore require such ZSTs to be provided to make sure that the caller has (a) access to the peripheral and (b) is the only one having access, since only a single instance of the type can exist at any time.
Being ZSTs, they do not carry any data to the drivers, only their ownership is meaningful, which is enforced by taking them as parameters for drivers.

> [!TIP]
> If you are used to thinking about MCU peripherals as referenced by a base address (in the case of memory-mapped peripherals), you can think of these ZSTs as abstraction over these, with a zero-cost, statically-enforced lock ensuring exclusive access.

In Ariel OS, these peripheral ZSTs are provided by [Ariel OS HAL crates][ariel-os-hals] in the respective `peripherals` modules.
In applications, the only safe way to obtain an instance of such types is by using the [`define_peripherals!`][define_peripherals-docs] macro, combined with a [spawner or task][spawner-or-task].
The [`group_peripherals!`][group_peripherals-docs] macro can also be useful.

### Example

The [`define_peripherals!`][define_peripherals-docs] macro allows to define an *Ariel OS peripheral struct*, an instance of which can be obtained with [`spawner` or `task`][spawner-or-task]:

```rust,ignore
ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_13 });
```

Multiple Ariel OS peripheral structs can be grouped into another Ariel OS peripheral struct using the [`group_peripherals!`][group_peripherals-docs] macro:

<!-- TODO: this needs to be kept up to date -->
```rust,ignore
ariel_os::hal::group_peripherals!(Peripherals {
    leds: LedPeripherals,
    buttons: ButtonPeripherals,
});
```

Similarly to `LedPeripherals`, an instance of the `Peripherals` Ariel OS peripheral struct thus defined can be obtained with [`spawner` or `task`][spawner-or-task].

## The `spawner` and `task` Ariel OS macros

Unlike traditional Rust programs, Ariel OS applications do not have a single entrypoint.
Instead, multiple functions can be registered to be started during boot.
Functions can currently be registered as either `spawner`s or `task`s:

<!-- TODO: technically the Spawner links are for Cortex-M only -->
- [`spawner` functions][spawner-attr-docs] are non-`async` and should be used when no `async` functions need to be called.
  They are provided with a [`Spawner`](https://docs.embassy.dev/embassy-executor/git/cortex-m/struct.Spawner.html) instance and can therefore be used to [`spawn`](https://docs.embassy.dev/embassy-executor/git/cortex-m/struct.Spawner.html#method.spawn) other `async` tasks.
- [`task` functions][task-attr-docs] are `async` functions that are statically allocated at compile-time.
  They are especially useful for long-running, `async` tasks.

Both of these can be provided with an instance of an Ariel OS peripheral struct when needed, using the `peripherals` macro parameters (see the macros' documentation) and taking that Ariel OS peripheral struct as parameter.
Note that the application panics at runtime when attempting to take the same peripheral twice, either because it is taken twice in the application itself, or because it is already used by the OS.
This panic should happen early during startup, instead of later during execution.

> [!TIP]
> The peripheral ZSTs obtained this way are regular Embassy-style peripherals, which are compatible with both Ariel OS portable drivers and [Embassy-style HALs][embassy-style-hals] HAL-specific drivers.

### Examples

Here is an example of the `task` macro (the `pins` module internally uses `define_peripherals!`) from the [`blinky` example][blinky-example-src]:

```rust,ignore
#[ariel_os::task(autostart, peripherals)]
async fn blinky(peripherals: pins::LedPeripherals) {
    let mut led = Output::new(peripherals.led, Level::Low);

    loop {
        led.toggle();
        Timer::after(Duration::from_millis(500)).await;
    }
}
```

## Using the Third-Party HALs Directly

<!-- NOTE: This implies that underlying HALs are not implementation details of Ariel OS. -->
Ariel OS internally uses third-party HALs in [its HAL crates][ariel-os-hals], which currently all are [Embassy-style HALs][embassy-style-hals].
[Ariel OS HAL crates][ariel-os-hals] provide portable drivers and APIs to abstract over the most common types of APIs and peripherals, so that applications can generally run just the same on boards featuring microcontrollers from different manufacturers.
When Ariel OS does not (yet) provide a portable driver over a peripheral that *is* supported by the third-party HAL, it is still possible to directly use that driver.
This can be done by simply adding the third-party HAL as a dependency of the application and passing the required peripheral ZSTs obtained through [Ariel OS mechanisms](#obtaining-peripheral-access) to instantiate the third-party driver in the application.

> [!IMPORTANT]
> When adding the third-party HAL as a dependency, the same version as that of Ariel OS *must* be used.

This is made possible by the fact that Ariel OS takes peripherals and binds interrupts needed for a piece of functionality only when that functionality is enabled (through Cargo features or [laze modules][laze-modules-book]), unless it is a core functionality of the OS.
For instance, leaving the `i2c` Cargo feature disabled allows drivers other than Ariel OS's to bind the interrupts required for the I2C peripherals, allowing to use I2C drivers from third-party HALs.

[embassy-style-hals]: ./glossary.md#embassy-style-hals
[ariel-os-hals]: ./glossary.md#ariel-os-hals
[spawner-attr-docs]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/attr.spawner.html
[task-attr-docs]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/attr.task.html
[spawner-or-task]: #the-spawner-and-task-ariel-os-macros
[blinky-example-src]: https://github.com/ariel-os/ariel-os/tree/main/examples/blinky
[define_peripherals-docs]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/hal/macro.define_peripherals.html
[group_peripherals-docs]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/hal/macro.group_peripherals.html
[laze-modules-book]: ./build-system.md#laze-modules
