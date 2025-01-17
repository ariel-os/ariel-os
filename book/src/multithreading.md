# Multithreading

Ariel OS supports multithreading on the Cortex-M, RISC-V, and Xtensa architectures, and is compatible with async executors.

**Important:**
When an application requires multithreading, it must enable it by [selecting the `sw/threading` laze module][laze-modules-book], which enables the `threading` Cargo feature.

## Spawning Threads

Threads can either be created using the [`#[ariel_os::thread]` attribute macro][thread-attr-macro-rustdoc], which creates and starts the thread during startup, or spawned dynamically at runtime. In the latter case, the thread stack must still be statically allocated at compile time.

The maximum number of concurrent threads is defined by the [`THREAD_COUNT`][max-thread-count-rustdoc] constant.
This value limits the number of *concurrently* running threads, but it is possible to create a higher number of threads during the application lifetime if earlier ones have finished their execution.

<!-- TODO: revisit when writing the async chapter -->
<!-- Multiple asynchronous tasks can be spawned within each thread with an executor from the integrated [Embassy] crate. This bridges the gap with async Rust, future-based concurrency, and asynchronous I/O. The executor executes all its tasks inside the thread context. When all tasks on the executor are pending, the owning thread is suspended. -->

## Scheduling

### Priority Scheduling

Ariel OS features a preemptive scheduler, which supports priority scheduling with up to 32 priority levels.
The highest priority runnable thread (or threads in the multicore case) is always executed.
Threads having the same priority are scheduled cooperatively.
The scheduler itself is tickless, therefore time-slicing isn't supported.
Thread priorities are dynamic and can be changed at runtime using [`thread::set_priority()`][set-priority-rustdoc].

### Idling

On single core, no idle threads are created.
Instead, if no threads are to be scheduled, the processor enters sleep mode until a thread is ready.
The context of the previously running thread is only saved once the next thread is ready and the context switch occurs.

On multicore, one idle thread is created for each core.
When the idle thread is scheduled, it prompts the current core to enter sleep mode.
<!-- TODO: reads as an implementation detail to me -->
<!-- This helps avoid conflicts that could occur on multicore if sleep mode (WFI) were entered directly from within the scheduler. -->

<!-- The runqueue is implemented with static memory allocation. All operations on the runqueue are performed in constant time, except for the deletion of a thread that is not the head of the runqueue. -->

## Multicore Support

Ariel OS currently supports symmetric multiprocessing (SMP) on the following MCUs:
  - ESP32-S3
  - RP2040

When the `sw/threading` laze module is selected and when available on the MCU, the `multi-core` laze module automatically gets selected, which enables SMP.
To disable multicore, [disable the `multi-core` laze module][laze-modules-book].

> Porting single-core applications to support multicore requires no changes to them.

### Priority Scheduling

A single global runqueue is shared across all cores.
The scheduler assigns the _C_ highest-priority, ready, and non-conflicting threads to the _C_ available cores.
The scheduler gets invoked individually on each core.
Whenever a higher priority thread becomes ready, the scheduler is triggered on the core with the lowest-priority running thread to perform a context switch.

### Core Affinity

Core affinity, also known as core pinning, is optionally configurable for each thread.
It allows to restrict the execution of a thread to a specific core and prevent it from being scheduled on another one.

<!-- ## Mutual Exclusion in the Kernel -->
<!---->
<!-- Ariel uses a single global critical section for all kernel operations. -->
<!-- - **On single-core** this critical section is implemented by masking all interrupts. -->
<!-- - **On multicore** an additional hardware spinlock is used in the case of the RP2040, and atomics are used on the ESP32-S3 to ensure that the critical section is enforced on all cores. -->

[Embassy]: https://embassy.dev/
[thread-attr-macro-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/attr.thread.html
[max-thread-count-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/thread/constant.THREAD_COUNT.html
[set-priority-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/thread/fn.set_priority.html
[laze-modules-book]: ./build_system.md#laze-modules
