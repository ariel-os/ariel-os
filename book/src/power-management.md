# Power Management

Microcontrollers may employ different techniques and mechanisms to manage power, i.e., to adjust the power draw to minimize the energy consumed over a period of time, while enabling proper operation.

The power draw of CMOS logic is the sum of two quantities: the static power and the dynamic power.
The static power comes from static losses and does not depend on the logic activity.
The dynamic power increases linearly with frequency, and quadratically with voltage.

This page goes over the various power management mechanisms and how they are supported in Ariel OS.

## Clock Gating

To minimize power consumption, the simplest technique is [clock gating][clock-gating-wikipedia]: by turning off the [clock][clock-signals-book] of peripherals and internal buses that are unused, either temporarily or for the entirety of the application, their dynamic power draw goes to zero.

In Ariel OS, the peripherals' clocks are automatically enabled when instantiating their drivers, and disabled when dropping them.
The [clock tree still needs to be configured][configuring-the-clock-tree-book] to provide the peripherals with appropriate clock signals.
Configuring the clock tree to only generate the required clock signals may help significantly reduce the power consumption.

On some microcontrollers, e.g., nRF MCUs, a hardware power management unit (PMU) automatically and dynamically clock-gates unused peripherals, reducing the power draw without software involvement.

The CPU(s) can be clock-gated with a dedicated instruction, and woken up on interrupts/events.

## Adjusting the Clocks of the CPU and Peripherals

As the dynamic power draw linearly increases with frequency, adjusting the frequency of the CPU(s), the peripherals and of their buses as required also helps reduce the power consumption.
However, as reducing their frequencies reduces their execution speed, it may also increase the time before which the microcontroller can go back to a [low-power mode](#low-power-modes), so this may not be beneficial overall.
For this reason, Ariel OS usually [by default configures the CPU(s)][configuring-the-clock-tree-book] to run at their maximum frequency.

<!-- NOTE: The RP2040 datasheet is very explicit about whether clock multiplexers are glitchless or not. -->
Currently, only a static clock configuration is supported.
It may however still be possible to switch the clock configuration dynamically, through [the third-party HAL directly][using-the-third-party-hals-directly-book].
Care must be taken when doing so that the clock hardware allows for glitchless switching.

> [!TIP]
> In applications where the peak current draw is limited, for instance because the board uses energy harvesting from an RF field or runs on a coin cell, it may be necessary to limit the CPU frequency.

## Dynamic Voltage Scaling

Most microcontrollers contain internal voltage regulators that power the CPU(s), the memories, and the peripherals.
Some of them support dynamic voltage scaling, which allows adjusting their output voltages to reduce the power consumption.
Reducing the voltage however limits the frequency the CPU(s) can be run at.

Ariel OS currently does not implement dynamic voltage scaling.
It is still possible to implement it manually if needed, on microcontrollers that support it.

## Low-Power Modes

Most microcontrollers comprise low-power modes, that may allow clock-gating the CPU, [clock-gating](#clock-gating) the peripherals, disabling many of the clocks, disabling some of the internal voltage regulators, and powering down the RAM.

> [!NOTE]
> Ariel OS does not yet leverage these low-power modes; only the CPU is automatically clock-gated when made possible by the [async executor][async-executors-book] in use.

[clock-gating-wikipedia]: https://en.wikipedia.org/wiki/Clock_gating
[clock-signals-book]: ./clocks.md#clock-signals
[configuring-the-clock-tree-book]: ./clocks.md#configuring-the-clock-tree
[using-the-third-party-hals-directly-book]: ./application.md#using-the-third-party-hals-directly
[async-executors-book]: ./async-support.md
