# Pins & General-Purpose Input/Outputs (GPIOs)

<!-- NOTE: nRF GPIOs are grouped into 32-GPIO ports. -->
<!-- NOTE: STM32 GPIOs are grouped into 16-GPIO ports. -->
<!-- NOTE: RP GPIOs are *not* grouped into ports. -->
GPIOs are the most basic peripheral microcontrollers can have: they allow reading the logic level of an input or setting the logic level of an output.
Most pins of microcontrollers can be used as GPIOs, and GPIOs are sometimes grouped into ports, which each contains 16 or 32 GPIOs depending on the microcontroller family.

## Pin Structure and Alternate Functions

Microcontroller pins, also referred to as pads, include the necessary circuitry to connect between the outside world and the internal peripherals.

> [!TIP]
> In this context, "pad" refers to a die's pad, not specifically to a package's pad.
> Additionally, "pin" and "pad" may refer only to the outward-facing electrical contact itself or may also include the internal circuitry between it and the internal registers.

On top of the GPIO logic detailed below, pins can be configured in the following other states:

- GPIOs can be connected to be controlled by other digital peripherals (often called "alternate functions"), e.g., I2C, SPI, UART,
- analog inputs (i.e., upstream of [the Schmitt trigger](#gpios-as-inputs)) can be directed to an analog-to-digital converter (ADC), and
- analog outputs can be connected to a digital-to-analog converter (DAC), if present on the microcontroller.

TODO: link to I2C, SPI, UART pages

The alternate functions available on a given pin—i.e., the peripherals that can be connected to it—is generally fixed and limited.
Some microcontroller families do however provide greater flexibility and allow connecting most peripherals to almost any pin.
These restrictions are enforced by Ariel OS (and by [Embassy-style HALs][embassy-style-hals]) at compile time, so it should not be possible to connect the wrong pin to a peripheral.
Refer to your microcontroller's datasheet to learn about the structure of its pins and about the peripherals that can be used with each of them.

## GPIOs as Inputs

Ariel OS provides [a portable driver][gpio-input-rustdoc] allowing to read the logic level of a GPIO:

```rust
let btn0 = Input::new(peripherals.buttons.button0, Pull::Up);
let level = btn0.is_low();
```

The driver implements [`embedded_hal::digital::InputPin`][embedded-hal-digital-inputpin], so it can be passed to higher-level drivers that require an input.

<!-- NOTE: The example above is intended to be usable directly with the examples from that page. -->
> [!TIP]
> The required peripheral ZST can be obtained using [the usual Ariel OS mechanisms][obtaining-peripheral-access-book].

Digital inputs are generally classified as "active low" or "active high": this is not a property of the input itself but of what each logic level represents, which depends on the board hardware.
For instance, an input that is *low* when a push-button connected to it is pressed is considered active low.

In addition, depending on the purpose of the input and on the board hardware, it may be necessary to enable the pin's internal [pull-up or pull-down resistor](#pull-uppull-down-resistors) so that the input is not left floating.

Each digital input comprises a [Schmitt trigger][schmitt-trigger-wikipedia] that implements a hysteresis on the (analog) input signal to prevent the digital input from "flickering" when the input switches levels.

### External Interrupts

To avoid polling the input as above, interrupts can be used instead, after enabling the `external-interrupts` Cargo feature:

```rust
let mut btn0 = Input::builder(peripherals.buttons.button0, Pull::Up)
    .build_with_interrupt()
    .unwrap();

loop {
    btn0.wait_for_low().await;

    led0.toggle();
    Timer::after_millis(100).await;
}
```

Interrupts have many benefits: in particular, they

- allow to respond with a lower latency than polling,
- may allow the processor and (some of the) peripherals to enter some low-power mode until the interrupt is triggered, greatly reducing power consumption, and
- allow to structure the application in an event-driven manner.

## GPIOs as Outputs

Digital outputs are tri-state: they use a pair of transistors to either drive the output low, high, or leave it floating, i.e., in a high impedance state (Hi-Z).
When the pair of transistors is used in a complementary way—i.e., driving the output either low or high—the output is used as a push–pull (PP).
If only the transistor able to drive the output low is used—i.e., the output is either driven low or is left in high impedance—the output is used as open-drain (OD).
Open-drain outputs are useful to "release" the output and allow another device, or a [pull-up resistor](#pull-uppull-down-resistors), to take control of it.
They are for instance used by certain bus protocols, such as I2C.

TODO: link to I2C page

Ariel OS provides [a portable driver][gpio-output-rustdoc] allowing to set the level of a GPIO:

```rust
let mut led0 = Output::new(peripherals.leds.led0, Level::Low);
led0.set_high();
```

The driver implements [`embedded_hal::digital::OutputPin`][embedded-hal-digital-outputpin] and [`embedded_hal::digital::StatefulOutputPin`][embedded-hal-digital-statefuloutputpin], so it can be passed to higher-level drivers that require an output, like SPI device drivers.

TODO: link to SPI page

> [!NOTE]
> Ariel OS does not currently provide a dedicated portable driver for open-drain outputs.

Similarly to inputs, outputs can generally be classified as "active low" or "active high" depending on how they are to be used.
For instance, an output that must be set *low* to turn an LED on is considered active low.

The current that the microcontroller can source (when the GPIO is high) or sink (when low) is limited for each pin.
In addition, the *total* current, across all pins at the same time, is also limited, to a value typically lower than the sum of the limit of each individual pin.
How much current is drawn depends on what is connected to the microcontroller pin.

> [!WARNING]
> Exceeding these limits may damage the microcontroller.
> Refer to your microcontroller's datasheet.

#### Output Configuration

Depending on the microcontroller, outputs may support configuring their drive strength and/or their speed/slew rate.
In Ariel OS, this is done through [`Output::builder()`][gpio-output-builder-rustdoc].

The drive strength of a pin determines how much current it can source/sink *without the voltage being affected*.
Increasing this setting is necessary when the load attached to the pin requires more current than the default, or when it is necessary to reduce the rise/fall time (i.e., to increase the slew rate).

> [!IMPORTANT]
> The drive strength does not *cap* the current through the pin.

The speed/slew rate of pins may also be configurable: faster transitions between levels may be required by certain protocols or components, but may increase electromagnetic interference as lower rise times make the unwanted electromagnetic emissions richer in higher frequencies.

> [!TIP]
> The speed of GPIOs should be left at their default value unless actually required by the board hardware.

## Pull-up/Pull-down Resistors

TODO: link *from* I2C page

Pull-up and pull-down resistors have two main use cases: forcing a known level on an otherwise-floating input, or in combination with an [open-drain output](#gpios-as-outputs) to force the output high when it is not driven low.

<!-- NOTE: The API does assume that both types are supported. -->
Because the logic level of a floating input is unpredictable, inputs should not be left floating and should either be driven by an external component or by a pull-up or pull-down resistor.
Such resistors can be added on the board, but all supported microcontrollers also feature internal pull-up and pull-down resistors which can be enabled in software.
In Ariel OS, [`Pull`][gpio-pull-rustdoc] can be passed to the [input constructor](#gpios-as-inputs) to configure its resistors.

> [!TIP]
> Enabling internal pull-up resistors even if external ones are also present is electrically safe, and only [reduces the resistance][parallel-resistance-wikipedia] as the resistors are in parallel, which only (slightly) increases power consumption.

Pull-up and pull-down resistors can be classified as strong or weak depending on their value: a low resistance allows more current through the resistor and makes it pull stronger, while a higher resistance makes the pull-up/pull-down resistor weaker.
Internal pull-up and pull-down resistors typically are quite weak, with a resistance of a few tens of kilohms[^nrf-stronger-pull].
Weaker pull-up/pull-down resistors result in a lower power consumption (as they let less current flow), but stronger ones are required in certain cases, e.g., to achieve higher frequencies with open-drain buses, like I2C.

TODO: link to I2C page

Internal resistors, manufactured in integrated circuits, are less accurate than external ones.
They are well-suited as generic pull-up or pull-down resistors, but must not typically be used when specific values are required.

[^nrf-stronger-pull]: With the exception of nRF MCUs, which have stronger pull-up and pull-down resistors.

[embassy-style-hals]: ./glossary.md#embassy-style-hals
[schmitt-trigger-wikipedia]: https://en.wikipedia.org/wiki/Schmitt_trigger
[gpio-input-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/gpio/struct.Input.html
[embedded-hal-digital-inputpin]: https://docs.rs/embedded-hal/1.0.0/embedded_hal/digital/trait.InputPin.html
[obtaining-peripheral-access-book]: ./application.md#obtaining-peripheral-access
[gpio-output-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/gpio/struct.Output.html
[gpio-output-builder-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/gpio/struct.Output.html#method.builder
[embedded-hal-digital-outputpin]: https://docs.rs/embedded-hal/1.0.0/embedded_hal/digital/trait.OutputPin.html
[embedded-hal-digital-statefuloutputpin]: https://docs.rs/embedded-hal/1.0.0/embedded_hal/digital/trait.StatefulOutputPin.html
[gpio-pull-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/gpio/enum.Pull.html
[parallel-resistance-wikipedia]: https://en.wikipedia.org/wiki/Series_and_parallel_circuits#Resistance_2
