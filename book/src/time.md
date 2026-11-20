# Time

## Elapsed Time Versus Wall Time

There is generally two different needs regarding time in embedded systems: being able to tell how much time has elapsed since a particular event occurred during execution, and being able to tell what time it is now (i.e., wall time).
The former only calls for a concept of elapsed time, while the later additionally requires an external time reference.

> [!NOTE]
> Ariel OS currently does not provide facilities for wall time.

## Timekeeping Hardware

To measure the passing of time, microcontrollers rely on hardware timers: hardware counters that run at a specified, known frequency, and that can generate interrupts when reaching a defined value or when overflowing.
The accuracy and stability of that timekeeping is therefore only as good as that of the [clock sources][clock-sources-book] used for these timers.
If high accuracy and stability are required, the clock source must be derived from a crystal (or an MEMS oscillator).

Microcontrollers generally feature multiple hardware timers, with varying resolutions, functionality, and power trade-offs.
The real-time clock (RTC) is also a hardware timer, that typically requires a 32,768 Hz [crystal resonator][crystal-resonator-book] and is kept running in all [low-power modes][low-power-modes-book]; instead of being a single integer, its output generally comprises separate integers for the year, month, day, hour, etc.
Hardware timers have many uses and are not limited to timekeeping.

## Software Timeouts

### The Time Driver and the Timer Queue

Ariel OS relies on [`embassy-time`][embassy-time-docsrs] for most of its timekeeping.
`embassy-time` introduces a timer queue: multiple software timeouts (i.e., *software* timers) that can run concurrently while being backed by a single [hardware timer](#timekeeping-hardware).
When using [multithreading][multithreading-book], it may be necessary to adjust the size of the timer queue used in that case using the `timer-generic-queue-*` Cargo features, to reduce usage of RAM or accommodate more concurrent software timeouts.
If the queue is not large enough, extra software timeouts will busy loop and prevent the CPU from going to sleep.
This is not necessary when multithreading is not enabled, and these Cargo features have no effect in that case.

<!-- NOTE: SysTick support: <https://github.com/embassy-rs/embassy/issues/786>. -->
To operate, `embassy-time` requires a matching *time driver*, that leverages the selected hardware timer to implement the timer queue functionality.
As this is hardware-specific, each [Ariel OS HAL][ariel-os-hals-book] provides its own time driver.
Underlying HALs that support multiple hardware timers for their time driver may allow selecting one using Cargo features.
These can be enabled in the application [by directly depending on the underlying HAL][using-third-party-hals-directly-book].
On Arm, using the SysTick timer is currently not supported.

A time driver is required as soon as any crate in the build relies on `embassy-time`'s items.
In Ariel OS, no time driver is enabled by default: the time driver can be enabled using the `time` Cargo feature, which additionally enables the time-related API items.

> [!NOTE]
> Software timeouts only guarantee that *at least* the configured duration has elapsed.

### The Tick Rate

<!-- NOTE: The RP2040 has a 64-bit timer. -->
The time driver relies on a fixed *tick rate*, which defines the duration value of a tick.
Each tick does not involve a CPU wakeup, the CPU only needs to wake up when an event from the hardware timer requires it: either when the next software timeout has expired, or when the hardware timer has overflowed.
Depending on the hardware timer, it may be possible to configure the tick rate, which can be selected in the application.
Selecting the tick rate involves a trade-off between time resolution and power consumption.
Indeed, selecting a higher tick rate does not *directly* cause the CPU to wake up more often, however since the hardware timer resolution (16 bit, 24 bit, 32 bit, or 64 bit) is limited, this increases how often the hardware timer overflows, which does result in a CPU wakeup (so that it is possible to use software timeout values greater than the one allowed by the hardware timer alone).
Additionally, a tick rate too high may result in overflow interrupts being missed in case of a long-running critical section.
On the other hand, reducing the tick rate reduces the time resolution, which may result in timeout durations slightly longer than intended.

<!-- NOTE: The default value is that of `embassy-time`. -->
The default tick rate is 1 MHz unless otherwise required by the time driver.
A different value can be selected in the application by depending on `embassy-time` and enabling one of its `tick-*` Cargo features.

TODO: warning about reducing the tick rate when using 16-bit timers, e.g., STM32G030

[clock-sources-book]: ./clocks.md#clock-sources-and-clock-generation
[embassy-time-docsrs]: https://docs.rs/embassy-time/latest/embassy_time/
[low-power-modes-book]: ./power-management.md#low-power-modes
[crystal-resonator-book]: ./clocks.md#piezoelectric-oscillators
[multithreading-book]: ./multithreading.md
[using-third-party-hals-directly-book]: ./application.md#using-the-third-party-hals-directly
[ariel-os-hals-book]: ./glossary.md#ariel-os-hals
