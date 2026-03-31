# Async Executors

Ariel OS embraces async Rust and favors async interfaces over blocking ones.

## System Executor Flavors

Ariel OS provides a system executor which is automatically started at startup.
It comes in different flavors, identified by their laze module names:

| laze module              | Embassy executor | Description |
| ------------------------ | ---------------- | ----------- |
| `executor-interrupt`     | [InterruptExecutor][interrupt-executor-rustdoc] | Runs in handler mode. A software interrupt (SWI) handler is used when the MCU provides one, otherwise the [board configuration must specify](./adding-board-support.md#adding-support-for-a-board-1) which peripheral interrupt to dedicate to this executor. |
| `executor-thread`        | [Custom, based on `raw::Executor`][asynch-thread-executor-rustdoc] | Runs inside a dedicated thread automatically started at startup. |

A default flavor compatible with the MCU is automatically selected by default in the order of preference in which they are listed above.
Another flavor can be manually selected, replacing the default one, by [selecting its laze module][laze-modules-book].
Not all flavors are available on all MCUs however, and the laze configuration will only allow selecting one the compatible ones.

> [!NOTE]
> The `executor-interrupt` might offer a slight performance advantage.

<!-- TODO: When to use each of them? -->

## Using Multiple Executors

> [!NOTE]
> Using multiple executors is possible but currently undocumented.

Running multiple executors allows running them with different priorities.

<!-- TODO: reference asynch-thread-executor-rustdoc to start a thread mode executor inside multiple threads manually -->

<!-- ## Interaction with Multithreading -->

<!-- TODO: How do threading and async interact? -->

<!-- TODO: Power consumption optimization -->

## Memory Layout

This section is intended to give a relatively high-level overview of the memory layout used by Ariel OS across processor architectures and MCU families.
Minor variations across them are omitted for clarity.
Additionally, no stability guarantees are currently provided regarding the memory layout.

> [!TIP]
> If needed, the following command can be used to obtain the exact locations of sections:
>
> ```sh
> readelf --sections <path-to-elf>
> ```

- Ariel OS places the ISR stack (`.isr_stack`) before the `.data` section and at the very beginning of the RAM on architectures and MCUs that allow it.
  This provides a form of stack overflow protection, as write attempts would then collide with the boundary of the physical RAM and trigger a fault, instead of overwriting `static` data.
  See the [`flip-link` crate][flip-link-readme] for an explanation of the technique.
- Async tasks are allocated statically, as individual `static`s, anywhere in the `.bss` section.
- The thread stacks, if [multithreading][multithreading-book] is enabled, are currently allocated as individual `static`s, anywhere in the `.bss` section.
- If [multithreading][multithreading-book] is enabled, and if the MCU has two symmetrical cores and [`multi-core`][multicore-support-book] is enabled, the ISR stack of the second core is allocated as a `static`, anywhere in the `.bss` section.
- Depending on the architecture, the uninitialized section is either called `.uninit` or `.noinit`.
- The heap, if enabled, takes the remaining space.
  `heapsize_required` allows link-time checking that there is enough space for the heap.

<!--
The diagrams are based on the following resources:

- Location of the `.isr_stack` section (inserted by Ariel OS):
    - `build.rs`: https://github.com/ariel-os/ariel-os/blob/2002a036f49848e9c049b735ed0053bce23b6172/src/ariel-os-rt/build.rs#L17-L34
    - `isr_stack.ld.in`: https://github.com/ariel-os/ariel-os/blob/2002a036f49848e9c049b735ed0053bce23b6172/src/ariel-os-rt/isr_stack.ld.in#L1-L12
- Location of the ISR stack of the second SMP core:
    - https://github.com/ariel-os/ariel-os/blob/2002a036f49848e9c049b735ed0053bce23b6172/src/ariel-os-threads/src/lib.rs#L565-L567
- Location of the `.data` section
    - Cortex-M: https://github.com/rust-embedded/cortex-m/blob/84e5c011068f01b7716684d20f45841cdfe3f285/cortex-m-rt/link.x.in#L126
    - RISC-V ESP32: https://github.com/rust-embedded/riscv/blob/187453b7904f997c4aa679d5bcf135f16c8853f8/riscv-rt/link.x.in#L129
    - Xtensa ESP32: https://github.com/esp-rs/esp-hal/blob/d9815b205115b3403d798c28f1bfee4c9eea8fd1/xtensa-lx-rt/xtensa.in.x#L36
- Location of the `.bss` section
    - Cortex-M: https://github.com/rust-embedded/cortex-m/blob/84e5c011068f01b7716684d20f45841cdfe3f285/cortex-m-rt/link.x.in#L161
    - RISC-V ESP32: https://github.com/rust-embedded/riscv/blob/187453b7904f997c4aa679d5bcf135f16c8853f8/riscv-rt/link.x.in#L150
    - Xtensa ESP32: https://github.com/esp-rs/esp-hal/blob/d9815b205115b3403d798c28f1bfee4c9eea8fd1/xtensa-lx-rt/xtensa.in.x#L47
- Location and name of the `.uninit`/`.noinit` section
    - Cortex-M: https://github.com/rust-embedded/cortex-m/blob/84e5c011068f01b7716684d20f45841cdfe3f285/cortex-m-rt/link.x.in#L175
    - RISC-V ESP32: https://github.com/rust-embedded/riscv/blob/187453b7904f997c4aa679d5bcf135f16c8853f8/riscv-rt/link.x.in#L168
    - Xtensa ESP32: https://github.com/esp-rs/esp-hal/blob/d9815b205115b3403d798c28f1bfee4c9eea8fd1/xtensa-lx-rt/xtensa.in.x#L55
-->

<!-- These diagrams can be rendered with Svgbob https://github.com/ivanceras/svgbob -->

<figure>
<pre>
           .-------------. - beginning of RAM
           |      ⋮      |
           +-------------+ -
           |             |
         | |             | ^
Addresses| | .isr_stack  | | ≥ isr_stacksize_required + executor_stacksize_required
         v |             | v
           |             |
           +-------------+ -
           |             |
           |             |
           |             |    .- - .-------------.
           |             |    |    |      ⋮      |
           |    .data    |    |    +-------------+
           |             |    |    |    Async    |
           |             |    |    |    tasks    |
           |             |    |    |      ...    |
           |             |    |    +-------------+
           +-------------+ - -'    |      ⋮      |
           |             |         +-------------+
           |             |         |   [Thread   |
           |    .bss     |         |    stacks   |
           |             |         |      ...]   |
           |             |         +-------------+
           +-------------+ - -.    |      ⋮      |
           |             |    '- - '-------------'
           |   .uninit   |
           |     or      |
           |   .noinit   |
           |             |
           +-------------+ -
           |             |
           |             |
           '             ' ^
           |             | |
           '    [Heap]   ' | ≥ "heapsize_required"
           |             | |
           '             ' v
           |             |
           |             |
           '-------------' - end of RAM
</pre>
    <figcaption class="text-center">Memory layout when using <code>executor-interrupt</code></figcaption>
</figure>

<figure>
<pre>
           .-------------. - beginning of RAM
           |      ⋮      |
           +-------------+ -
           |             |
           |             |
         | |             | ^
Addresses| | .isr_stack  | | ≥ "isr_stacksize_required"
         v |             | v
           |             |
           +-------------+ -
           |             |
           |             |
           |             |    .- - .-------------.
           |             |    |    |      ⋮      |
           |             |    |    +-------------+
           |    .data    |    |    |    Async    |
           |             |    |    |    tasks    |
           |             |    |    |      ...    |
           |             |    |    +-------------+
           |             |    |    |      ⋮      |
           |             |    |    +-------------+ -
           +-------------+ - -'    |   Executor  | ^
           |             |         |    thread   | | ≥ "executor_stacksize_required"
           |             |         |    stack    | v
           |             |         +-------------+ -
           |    .bss     |         |      ⋮      |
           |             |         +-------------+
           |             |         |   [Thread   |
           |             |         |    stacks   |
           +-------------+ - -.    |      ...]   |
           |             |    |    +-------------+
           |             |    |    |      ⋮      |
           |   .uninit   |    '- - '-------------'
           |     or      |
           |   .noinit   |
           |             |
           |             |
           +-------------+ -
           |             |
           |             |
           '             ' ^
           |             | |
           '    [Heap]   ' | ≥ "heapsize_required"
           |             | |
           '             ' v
           |             |
           |             |
           '-------------' - end of RAM
</pre>
    <figcaption class="text-center">Memory layout when using <code>executor-thread</code></figcaption>
</figure>

<style>
.text-center {
    text-align: center;
}
</style>

[laze-modules-book]: ./build-system.md#laze-modules
[multithreading-book]: ./multithreading.md
[multicore-support-book]: ./multithreading.md#multicore-support
[interrupt-executor-rustdoc]: https://docs.embassy.dev/embassy-executor/git/cortex-m/struct.InterruptExecutor.html
[executor-rustdoc]: https://docs.embassy.dev/embassy-executor/git/cortex-m/struct.Executor.html
[asynch-thread-executor-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/asynch/thread_executor/index.html
[flip-link-readme]: https://github.com/knurling-rs/flip-link/blob/199347bebde115e690393dd1f5016f2703083df9/README.md
