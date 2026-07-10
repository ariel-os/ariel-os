# Clocks

This page is intended to provide some general background on clocks, and is MCU-agnostic.
It should help with the necessary configuration of the MCU's clocks, and may help discover useful mechanisms that MCUs may have, but is not meant to replace their datasheets.

## Clock Signals

Microcontrollers being mostly [synchronous logic][synchronous-logic-wikipedia], they require various clock signals to run.
*Clock signals* are digital signals that dictate the frequencies at which the various subcomponents operate.
In particular, the CPU(s), the buses, and the microcontroller peripherals all need clock signals, which may be different.

Performance (throughput and latency) increases with the frequency, but the power draw also increases linearly with it.

> [!NOTE]
> Clock signals are usually simply called "clocks."

## Clock Sources and Clock Generation

To generate the required clock signals, microcontrollers provide different mechanisms: some are completely internal, while others require external components.

### Internal Oscillators

Microcontrollers contain one or multiple internal oscillators, which are implemented as *RC oscillators*.
They do not require any external component, and their frequency may either be fixed or configurable within a range.

As they do not require extra parts, they help keep the BOM cost low but are less stable over time and across voltage and temperature changes.
They are also less accurate than oscillators with external components, which makes them not suitable as a clock source for keeping track of time, e.g., with a real-time clock (RTC).

<!-- NOTE: STM32 and RP MCUs start with an internal oscillator. -->
<!-- NOTE: The ESP32-C6 always requires a crystal resonator. -->
Many microcontrollers use one of the internal oscillators as the initial clock source after reset, before [clocks are configured in software](#configuring-the-clock-tree), to avoid the hard requirement on an external resonator.

### Clock Sources Involving External Components

Microcontrollers also feature clock sources that are more stable and accurate than RC oscillators, but require external components.

#### Piezoelectric Oscillators

[Piezoelectric oscillators][crystal-oscillator-wikipedia] are oscillators that require an external piezoelectric resonator.
A piezoelectric resonator is made of a piezoelectric material that physically vibrates as part of the oscillator circuit, so that the oscillator precisely maintains its expected frequency.
Crystal resonators use quartz crystals as the piezoelectric material, while ceramic resonators use ceramic materials.
The frequency of a piezoelectric resonator is fixed, but microcontrollers' piezoelectric oscillators may either require a specific frequency, or instead allow a range to accommodate different values.
RTCs also typically require a dedicated 32,768 Hz crystal resonator, at least so that the RTC can operate in low-power modes.

> [!TIP]
> Crystal resonators are sometimes incorrectly referred to as crystal *oscillators*, but in this case the oscillator is really the assembly of the internal "oscillator block," of the external crystal resonator, and of the two load capacitors.
> Actual [crystal oscillators as dedicated integrated circuits](#external-clock-signals) do exist however.

#### External Clock Signals

Microcontrollers may also allow directly using an *external clock signal*, without using an internal "oscillator block" at all.
This makes it possible to synchronize the microcontroller with other clock signals provided by the board, e.g., another microcontroller.
More importantly, this allows using external *crystal oscillators* (e.g., TCXOs): dedicated integrated circuits comprising a crystal resonator.
Alternatively, external *MEMS oscillators* can also be used, which are physically smaller and less expensive.
Using external oscillators may allow reusing the second pin (that would otherwise be needed for the resonator) as a regular GPIO.

In addition, it is sometimes also possible to output some of the clock signals, either for debugging purposes or to feed the clock signal to another component on the same board (e.g., another microcontroller).

## Configuring the Clock Tree

Clock signals are distributed from the clock source(s) to the CPU(s), buses, and peripherals through a clock tree.
In addition to the clock sources, it comprises multiplexers, dividers, multipliers, and PLLs, which allow configuring the frequencies required for the various subcomponents.
Refer to your MCU's datasheet to learn about its clock tree.

The typical configuration is to use an oscillator with a crystal resonator, and to derive the required frequencies using the PLL and the dividers available for the peripheral clocks.
Unless the peak current the MCU is allowed to draw is limited (e.g., because it uses energy harvesting of an RF field, or runs on a coin cell), the CPU is usually run at its maximum frequency, and the peripherals at lower frequencies to limit power consumption.
When configuring the PLL, care must be taken to minimize the settling time and the maximum intermediate frequency: to this end, multipliers and dividers should usually be set to the smallest values that satisfy the desired outputs.
Additionally, if the RTC is needed, an appropriate clock source must also be configured, especially if it needs to run in low-power modes.

At reset, the CPU clock is set by hardware to the default clock source (often one of the [internal oscillators](#internal-oscillators)).
Early in Ariel OS's startup, clocks are configured with a default configuration that depends on the MCU and the board.
The [`#[ariel_os::config]` macro][config-attr-macro-rustdoc] allows providing a custom configuration for clocks, which will be used by Ariel OS instead of the default one; currently this is only supported on STM32 MCUs.

Peripheral drivers automatically enable the peripheral's clock when instantiated, and disable it (["gate it"][clock-gating-wikipedia]) when dropped, but the clock tree still needs to be configured manually to distribute the required clock signal to the peripheral.

[synchronous-logic-wikipedia]: https://en.wikipedia.org/wiki/Synchronous_circuit
[crystal-oscillator-wikipedia]: https://en.wikipedia.org/wiki/Crystal_oscillator
[config-attr-macro-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/attr.config.html
[clock-gating-wikipedia]: https://en.wikipedia.org/wiki/Clock_gating
