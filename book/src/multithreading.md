# Multithreading

Ariel OS supports multithreading on the Cortex-M, RISC-V, and Xtensa architectures, and is compatible with async executors.

**Important:**
When an application requires multithreading, it must enable it by [selecting the `sw/threading` laze module][laze-modules-book], which enables the `threading` Cargo feature.

## Spawning Threads

The recommended way of starting threads is by using the [`#[ariel_os::thread]` attribute macro][thread-attr-macro-rustdoc], which creates and starts the thread during startup.
Threads can also be spawned dynamically at runtime. In this case, the thread stack must still be statically allocated at compile time.

The maximum number of threads is defined by the [`THREAD_COUNT`][max-thread-count-rustdoc] constant.

## Scheduling

### Multicore Support

Ariel OS currently supports symmetric multiprocessing (SMP) on the following MCUs:
  - ESP32-S3
  - RP2040
  - RP235XA

When the `sw/threading` [laze module][laze-modules-book] is selected and when available on the MCU, the `multi-core` laze module automatically gets selected, which enables SMP.
To disable multicore, disable the `multi-core` [laze module][laze-modules-book].

> [!NOTE]
> Porting single-core applications to support multicore requires no changes to them.

### Priority Scheduling

Ariel OS features a preemptive scheduler, which supports priority scheduling with up to [`SCHED_PRIO_LEVELS`][sched-prio-levels-rustdoc] priority levels.
The highest priority runnable thread (or threads in the multicore case) is always executed.
Threads having the same priority are scheduled cooperatively.
The scheduler itself is tickless, therefore time-slicing isn't supported.
Thread priorities are dynamic and can be changed at runtime using [`thread::set_priority()`][set-priority-rustdoc].

On multicore, a single global runqueue is shared across all cores.
The scheduler assigns the _C_ highest-priority, ready, and non-conflicting threads to the _C_ available cores.
The scheduler gets invoked individually on each core.
Whenever a higher priority thread becomes ready, the scheduler is triggered on the core with the lowest-priority running thread to perform a context switch.

### Idling

On single core, no idle threads are created.
Instead, if no threads are to be scheduled, the processor enters sleep mode until a thread is ready.

On multicore, one idle thread is created for each core.
When an idle thread is scheduled, it prompts the current core to enter sleep mode.

### Core Affinity

Core affinity, also known as core pinning, is optionally configurable for each thread using the [`#[ariel_os:thread]` attribute macro][thread-attr-macro-rustdoc].
It allows to restrict the execution of a thread to a specific core and prevent it from being scheduled on another one.
See the [`threading-multicore` example][threading-multicore-example-repo] for a usage example.

[Embassy]: https://embassy.dev/
[thread-attr-macro-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/attr.thread.html
[max-thread-count-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/thread/constant.THREAD_COUNT.html
[set-priority-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/thread/fn.set_priority.html
[sched-prio-levels-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/thread/constant.SCHED_PRIO_LEVELS.html
[laze-modules-book]: ./build-system.md#laze-modules
[threading-multicore-example-repo]: https://github.com/ariel-os/ariel-os/tree/main/examples/threading-multicore
