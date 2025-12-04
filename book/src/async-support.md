# Async Executors

Ariel OS embraces async Rust and favors async interfaces over blocking ones.

## System Executor Flavors

Ariel OS provides a system executor which is automatically started at startup.
It comes in three different flavors, identified by their laze module names:

| laze module              | Embassy executor | Description |
| ------------------------ | ---------------- | ----------- |
| `executor-interrupt`     | [InterruptExecutor][interrupt-executor-rustdoc] | Runs in handler mode. A software interrupt (SWI) handler is used when the MCU provides one, otherwise the [board configuration must specify](./adding-board-support.md#adding-support-for-a-board-1) which peripheral interrupt to dedicate to this executor. |
| `executor-single-thread` | [Executor][executor-rustdoc] | Runs in thread mode in place of threads. Therefore not compatible with [multithreading][multithreading-book]. |
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

[laze-modules-book]: ./build-system.md#laze-modules
[multithreading-book]: ./multithreading.md
[interrupt-executor-rustdoc]: https://docs.embassy.dev/embassy-executor/git/cortex-m/struct.InterruptExecutor.html
[executor-rustdoc]: https://docs.embassy.dev/embassy-executor/git/cortex-m/struct.Executor.html
[asynch-thread-executor-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/asynch/thread_executor/index.html
