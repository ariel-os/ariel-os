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
The [Ariel OS sleep mode](#ariel-os-low-power-modes) allows clock-gating the CPU(s).

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

### Ariel OS Low-Power Modes

As microcontrollers feature a great diversity of low-power modes (and sometimes quite many low-power modes on a given microcontroller), Ariel OS introduces its own low-power modes, defined as follows:

| Ariel OS low-power mode | Description                                                                                                                                                                                                    |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ariel OS sleep mode     | Clock-gates the CPU, and may adjust clocks and regulators to further reduce the consumption.                                                                                                                   |
| Ariel OS stop mode      | Disables almost all the clocks, but retains the RAM contents and the GPIOs' state, including their pull-up/pull-down setting.                                                                                  |
| Ariel OS standby mode   | Disables almost all the clocks and may power down the RAM and the peripherals' memories, losing their contents. The GPIOs' state is also generally lost. The RTC can still operate if allowed by the hardware. |

Ariel OS implements these modes by using the microcontroller's low-power modes and by individually adjusting hardware settings.

> [!IMPORTANT]
> Ariel OS implements these low-power modes on a best-effort basis: some microcontrollers may not support the required low-power settings, they may not be supported yet, or they may be lacking testing.
> Do measure the power consumption of your hardware when relevant for your application.

#### Ariel OS Sleep Mode

All peripherals generally keep operating in Ariel OS sleep mode, including DMA when the hardware allows it, unless manually disabled in software by the application.

#### Ariel OS Stop Mode

<!-- NOTE: Timers from the `time` module are currently provided by `embass-time`, but this is mostly an implementation detail. -->
Peripherals other than GPIOs and the RTC generally cannot operate in Ariel OS stop mode, as their clocks are disabled.
In particular, this means that hardware timers cannot be assumed to be running and that (software) timers—from the [`time` module][ariel-os-time-rustdoc] and from [`embassy-time`][ariel-os-reexports-embassy-time-rustdoc]—may underestimate elapsed time.
It may be possible to use the RTC as the backing hardware timer for software timers, which still keep track of elapsed time in this low-power mode.

#### Ariel OS Standby Mode

As in [Ariel OS stop mode](#ariel-os-stop-mode), most peripherals cannot operate in Ariel OS standby mode as their clocks are disabled.

### Entering a Low-Power Mode

Entering an Ariel OS low-power mode simply involves calling [`power::sleep_mode::enter()`], [`power::stop_mode::enter()`], or [`power::standby_mode::enter()`].

TODO: async executors may also execute a wfi/wfe (or similar) instruction

The Ariel OS stop mode and Ariel OS standby mode need to be provided with [wake-up triggers](#wake-up-triggers) that determine the exit conditions.

Entering these low-power modes may be delayed by a few cycles, in particular because of outstanding memory writes.

> [!IMPORTANT]
> There is currently little verification in software prior to entering the Ariel OS stop mode and Ariel OS standby mode.
> Peripheral operations in progress, in particular I2C or UART transactions, may be interrupted.
> It is currently the application's responsibility to ensure that the relevant peripherals are inactive before entering these modes.

> [!NOTE]
> In the future, Ariel OS should become aware of which peripherals are currently in use and prevent entering low-power modes that would interrupt their operations.

### Exiting from Low-Power Modes

Exiting low-power modes requires an interrupt to wake up the CPU(s) and/or the microcontroller.

#### Wake-Up Triggers

As most clocks are off in the Ariel OS stop mode and Ariel OS standby mode, the triggers must come from either an external interrupt (implemented in hardware using asynchronous logic) or an RTC event, or hardware that allows to keep the RTC running.

The triggers that must trigger an exit from these low-power modes are given to the [function call used to enter the low-power mode].

TODO: define trigger events

TODO: RTC event not yet supported

TODO: guarantees about the sleep mode exit

##### Ariel OS Stop Mode

Ariel OS will only exit Ariel OS stop mode when the configured trigger was triggered.
Depending on the hardware, there may still be spurious wake-ups.
After waking up, the application will resume execution right after the call to [`power::stop_mode::enter()`] that triggered the entry (ISRs may still execute before that).

As most clocks are stopped in this mode, the wake-up time is higher than that of sleep mode, as the [clock generators][clock-sources-clock-generation-book] need to restart and the PLLs (if used) to lock again.
Using a higher driving strength for the [piezoelectric resonator][piezoelectric-oscillators-book] (when supported by the hardware) may help reduce the start-up time, but also increases the power consumption when running.

TODO: clock configuration when exiting: default restored?

##### Ariel OS Standby Mode

TODO: guarantees about the standby mode exit

The hardware may only offer a limited set of pins (and of trigger events) usable for waking up from Ariel OS standby mode.
As the RAM contents may be lost in this mode, the microcontroller is reset and the application reboots (this is also needed to reconfigure the peripherals, as their own memories are also lost).
As a reset is required, exiting the standing mode requires much more time than for the Ariel OS stop mode.

> [!NOTE]
> In the future, Ariel OS will provide an API to distinguish resets triggered by a wake-up from power-on resets and other reset reasons.

[clock-gating-wikipedia]: https://en.wikipedia.org/wiki/Clock_gating
[clock-signals-book]: ./clocks.md#clock-signals
[configuring-the-clock-tree-book]: ./clocks.md#configuring-the-clock-tree
[using-the-third-party-hals-directly-book]: ./application.md#using-the-third-party-hals-directly
[async-executors-book]: ./async-support.md
[clock-sources-clock-generation-book]: ./clocks.md#clock-sources-and-clock-generation
[piezoelectric-oscillators-book]: ./clocks.md#piezoelectric-oscillators
[ariel-os-time-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/time/index.html
[ariel-os-reexports-embassy-time-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/reexports/embassy_time/index.html
